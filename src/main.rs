mod mesh;
mod cv;
mod plotting;
mod sim;  
mod window;
mod examples;

//use bevy::prelude::*;

use bevy::{
    asset::weak_handle,
    color::palettes::{basic::YELLOW, css::{BLACK, GRAY, WHITE}},
    core_pipeline::core_2d::{Transparent2d, CORE_2D_DEPTH_FORMAT},
    math::{ops, FloatOrd},
    prelude::*,
    render::{
        mesh::{Indices, MeshVertexAttribute, RenderMesh, VertexAttributeValues},
        render_asset::{RenderAssetUsages, RenderAssets},
        render_phase::{
            AddRenderCommand, DrawFunctions, PhaseItemExtraIndex, SetItemPipeline,
            ViewSortedRenderPhases,
        },
        render_resource::{
            BlendState, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState,
            DepthStencilState, Face, FragmentState, FrontFace, MultisampleState, PipelineCache,
            PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipelineDescriptor,
            SpecializedRenderPipeline, SpecializedRenderPipelines, StencilFaceState, StencilState,
            TextureFormat, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
        },
        sync_component::SyncComponentPlugin,
        sync_world::{MainEntityHashMap, RenderEntity},
        view::{ExtractedView, RenderVisibleEntities, ViewTarget},
        Extract, Render, RenderApp, RenderSet,
    },
    sprite::{
        extract_mesh2d, DrawMesh2d, Material2dBindGroupId, Mesh2dPipeline, Mesh2dPipelineKey,
        Mesh2dTransforms, MeshFlags, RenderMesh2dInstance, SetMesh2dBindGroup,
        SetMesh2dViewBindGroup,
    },
    window::WindowResolution,
};
use sim::cauchy_fvm::CauchyFVM;
use std::f32::consts::PI;

fn main() -> () {

    let tmesh = mesh::TriangleMesh::new_beam(6.0, 2.0, (12, 4));
    
    App::new()
        .add_plugins((DefaultPlugins
                .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(640.,480.).with_scale_factor_override(1.0),
                    ..default() 
                }),
                ..default()
            }),
            ColoredMesh2dPlugin))
        .insert_resource(TriangleMeshResource(tmesh))
        .insert_resource(Time::<Fixed>::from_hz(1200.0))
        //.insert_resource(SimulationTimer(Timer::from_seconds(0.001, TimerMode::Repeating)))
        .add_systems(Startup, beam)
        .add_systems(Startup, create_simulator)
        .add_systems(FixedUpdate, update_simulator)
        .add_systems(Update, set_new_vertices_with_simulator)
        .run(); 
}

fn beam(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    beam_mesh: Res<TriangleMeshResource>,
) {
    // create Bevy Mesh
    let mut beam = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::all(), // RENDER_WORLD // maybe needs all() ?
    );

    // add vertex positions from TriangleMesh struct
    let mut v_pos: Vec<[f32; 3]> = Vec::new();
    for vertex in beam_mesh.0.vertices.rows() {
        let x = vertex[[0]] as f32;
        let y = vertex[[1]] as f32;
        let z = 0.0;
        v_pos.push([x,y,z]);
    }
    beam.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos.clone());
    
    // set beam mesh color
    //beam.insert_attribute(Mesh::ATTRIBUTE_COLOR, LinearRgba::from(WHITE).as_u32());
    
    let mut v_color: Vec<u32> = Vec::new();
    for _v in 0..v_pos.len() {
        v_color.extend_from_slice(&[LinearRgba::from(GRAY).as_u32()]);
    }

    beam.insert_attribute(
        MeshVertexAttribute::new("Vertex_Color", 1, VertexFormat::Uint32),
        v_color
    );

    // triangle list
    let mut indices: Vec<u32> = Vec::new();
    for triangle in beam_mesh.0.triangles.rows() {
        let i = triangle[[0]] as u32;
        let j = triangle[[1]] as u32;
        let k = triangle[[2]] as u32;
        indices.extend_from_slice(&[i,j,k]);
    }
    beam.insert_indices(Indices::U32(indices));
    
    // spawn entities for star and camera
    commands.spawn((
            ColoredMesh2d,
            Mesh2d(meshes.add(beam))
    ));
    //commands.spawn(Camera2d);
    commands.spawn((
        Camera2d::default(),
        Projection::from(OrthographicProjection {
            scale: 0.02,
            ..OrthographicProjection::default_2d()
            },
        )
    ));
}


#[derive(Resource)]
struct SimulationTimer(Timer);

#[derive(Resource)]
struct TriangleMeshResource(mesh::TriangleMesh);

#[derive(Component)]
pub struct MeshSimulator {
    // wraps the CauchyFVM simulator
    sim: CauchyFVM,
}

fn create_simulator(mut commands: Commands, tmesh: Res<TriangleMeshResource>) {
    let simulator = MeshSimulator { 
        sim: CauchyFVM::new(&tmesh.0, "rubber", 1e-3) 
    };
    commands.spawn(simulator);
}

fn update_simulator(
    //time: Res<Time>,
    //mut timer: ResMut<SimulationTimer>,
    mut query: Query<&mut MeshSimulator>) {
    //if timer.0.tick(time.delta()).just_finished() {
        for mut simulator in &mut query {
            simulator.sim.update();
        }
    //} 
}

fn set_new_vertices_with_simulator(mut query: Query<&MeshSimulator>, shape: Single<&Mesh2d>, mut meshes: ResMut<Assets<Mesh>>) {
    let Some(mesh) = meshes.get_mut(*shape) else { return; };
    if let Some(VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        let simulator = query.single();
        let sim_vertices = &simulator.unwrap().sim.sim_mesh.vertices;

        for (idx, position) in positions.iter_mut().enumerate() {
            let v = sim_vertices.row(idx);
            position[0] = v[0] as f32;
            position[1] = v[1] as f32;
        }
    }
}

/// A marker component for colored 2d meshes
#[derive(Component, Default)]
pub struct ColoredMesh2d;

/// Custom pipeline for 2d meshes with vertex colors
#[derive(Resource)]
pub struct ColoredMesh2dPipeline {
    // this struct wraps standard Mesh2dPipeline
    mesh2d_pipeline: Mesh2dPipeline,
}

impl FromWorld for ColoredMesh2dPipeline {
    fn from_world(world: &mut World) -> Self {
        Self {
            mesh2d_pipeline: Mesh2dPipeline::from_world(world),
        }
    }
}

// Implement `SpecializedPipeline` to customize the default rendering from `Mesh2dPipeline`
impl SpecializedRenderPipeline for ColoredMesh2dPipeline {
    type Key = Mesh2dPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        // Customize how to store the meshes' vertex attributes in the vertex buffer
        // Our meshes only have position and color
        let formats = vec![
            VertexFormat::Float32x3, // position
            VertexFormat::Uint32, // color
        ];

       let vertex_layout = VertexBufferLayout::from_vertex_formats(VertexStepMode::Vertex, formats);

       let format = match key.contains(Mesh2dPipelineKey::HDR) {
            true => ViewTarget::TEXTURE_FORMAT_HDR,
            false => TextureFormat::bevy_default(),
       };
    
       RenderPipelineDescriptor {
           vertex: VertexState {
           // use custom shader
           shader: COLORED_MESH2D_SHADER_HANDLE,
           entry_point: "vertex".into(),
           shader_defs: vec![],
           // use our custom vertex buffer
           buffers: vec![vertex_layout],
           },
           fragment: Some(FragmentState {
                // use custom shader
                shader: COLORED_MESH2D_SHADER_HANDLE,
                shader_defs: vec![],
                entry_point: "fragment".into(),
                targets: vec![Some(ColorTargetState {
                    format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
           }),
           
           // use 2 standard uniforms for 2d meshes
           layout: vec![
                // bind group 0 is the view uniform
                self.mesh2d_pipeline.view_layout.clone(),
                // bind group 1 is the mesh uniform
                self.mesh2d_pipeline.mesh_layout.clone(),
           ],
           push_constant_ranges: vec![],
           
           primitive: PrimitiveState {
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Line,
                conservative: false,
                topology: key.primitive_topology(),
                strip_index_format: None,
           },

           depth_stencil: Some(DepthStencilState {
                format: CORE_2D_DEPTH_FORMAT,
                depth_write_enabled: false,
                depth_compare: CompareFunction::GreaterEqual,
                stencil: StencilState {
                    front: StencilFaceState::IGNORE,
                    back: StencilFaceState::IGNORE,
                    read_mask: 0,
                    write_mask: 0,
                },
                bias: DepthBiasState {
                    constant: 0,
                    slope_scale: 0.0,
                    clamp: 0.0,
                },
           }),

           multisample: MultisampleState {
                count: key.msaa_samples(),
                mask: !0,
                alpha_to_coverage_enabled: false,
           },

           label: Some("colored_mesh2d_pipeline".into()),

           zero_initialize_workgroup_memory: false,
       }
    }
}

/// Our custom pipeline needs its own instance storage
#[derive(Resource, Deref, DerefMut, Default)]
pub struct RenderColoredMesh2dInstances(MainEntityHashMap<RenderMesh2dInstance>);

// This specifies how to render a colored 2d mesh
type DrawColoredMesh2d = (
    SetItemPipeline,
    SetMesh2dViewBindGroup<0>,
    SetMesh2dBindGroup<1>,
    DrawMesh2d,
);

/// Plugin that renders [`ColoredMesh2d`]s
pub struct ColoredMesh2dPlugin;

impl Plugin for ColoredMesh2dPlugin {
    fn build(&self, app: &mut App) {
        // load our custom shader
        let mut shaders = app.world_mut().resource_mut::<Assets<Shader>>();
        shaders.insert(
            &COLORED_MESH2D_SHADER_HANDLE,
            Shader::from_wgsl(COLORED_MESH2D_SHADER, file!()),
        );
        app.add_plugins(SyncComponentPlugin::<ColoredMesh2d>::default());

        // Register our custom draw functions and add our render systems
        app.get_sub_app_mut(RenderApp)
            .unwrap()
            .add_render_command::<Transparent2d, DrawColoredMesh2d>()
            .init_resource::<SpecializedRenderPipelines<ColoredMesh2dPipeline>>()
            .init_resource::<RenderColoredMesh2dInstances>()
            .add_systems(
                ExtractSchedule,
                extract_colored_mesh2d.after(extract_mesh2d),
            )
            .add_systems(
                Render,
                queue_colored_mesh2d.in_set(RenderSet::QueueMeshes),
            );
    }

    fn finish(&self, app: &mut App) {
        // register our custom pipeline
        app.get_sub_app_mut(RenderApp)
            .unwrap()
            .init_resource::<ColoredMesh2dPipeline>();
    }
}

/// Extract the [`ColoredMesh2d`] marker component into the render app
pub fn extract_colored_mesh2d(
    mut commands: Commands,
    mut previous_len: Local<usize>,
    // When extracting, you must use `Extract` to mark the `SystemParam`s
    // which should be taken from the main world.
    query: Extract<
        Query<
            (
                Entity,
                RenderEntity,
                &ViewVisibility,
                &GlobalTransform,
                &Mesh2d,
            ),
            With<ColoredMesh2d>
        >,
    >,
    mut render_mesh_instances: ResMut<RenderColoredMesh2dInstances>,
    ) {
    let mut values = Vec::with_capacity(*previous_len);
    for (entity, render_entity, view_visibility, transform, handle) in &query {
        if !view_visibility.get() {
            continue;
        }

        let transforms = Mesh2dTransforms {
            world_from_local: (&transform.affine()).into(),
            flags: MeshFlags::empty().bits(),
        };

        values.push((render_entity, ColoredMesh2d));
        render_mesh_instances.insert(
            entity.into(),
            RenderMesh2dInstance {
                mesh_asset_id: handle.0.id(),
                transforms,
                material_bind_group_id: Material2dBindGroupId::default(),
                automatic_batching: false,
                tag: 0,
            },
        );
    }
    *previous_len = values.len();
    commands.try_insert_batch(values);
}

/// Queue the 2d meshes marked with [`ColoredMesh2d`] using our custom pipeline and draw function
pub fn queue_colored_mesh2d(
    transparent_draw_functions: Res<DrawFunctions<Transparent2d>>,
    colored_mesh2d_pipeline: Res<ColoredMesh2dPipeline>,
    mut pipelines: ResMut<SpecializedRenderPipelines<ColoredMesh2dPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    render_meshes: Res<RenderAssets<RenderMesh>>,
    render_mesh_instances: Res<RenderColoredMesh2dInstances>,
    mut transparent_render_phases: ResMut<ViewSortedRenderPhases<Transparent2d>>,
    views: Query<(&RenderVisibleEntities, &ExtractedView, &Msaa)>,
) {
    if render_mesh_instances.is_empty() {
        return;
    }
    // Iterate each view (a camera is a view)
    for (visible_entities, view, msaa) in &views {
        let Some(transparent_phase) = transparent_render_phases.get_mut(&view.retained_view_entity)
        else {
            continue;
        };

        let draw_colored_mesh2d = transparent_draw_functions.read().id::<DrawColoredMesh2d>();

        let mesh_key = Mesh2dPipelineKey::from_msaa_samples(msaa.samples())
            | Mesh2dPipelineKey::from_hdr(view.hdr);

        // Queue all entities visible to that view
        for (render_entity, visible_entity) in visible_entities.iter::<Mesh2d>() {
            if let Some(mesh_instance) = render_mesh_instances.get(visible_entity) {
                let mesh2d_handle = mesh_instance.mesh_asset_id;
                let mesh2d_transforms = &mesh_instance.transforms;
                // Get our specialized pipeline
                let mut mesh2d_key = mesh_key;
                let Some(mesh) = render_meshes.get(mesh2d_handle) else {
                    continue;
                };
                mesh2d_key |= Mesh2dPipelineKey::from_primitive_topology(mesh.primitive_topology());

                let pipeline_id =
                    pipelines.specialize(&pipeline_cache, &colored_mesh2d_pipeline, mesh2d_key);

                let mesh_z = mesh2d_transforms.world_from_local.translation.z;
                transparent_phase.add(Transparent2d {
                    entity: (*render_entity, *visible_entity),
                    draw_function: draw_colored_mesh2d,
                    pipeline: pipeline_id,
                    // The 2d render items are sorted according to their z value before rendering,
                    // in order to get correct transparency
                    sort_key: FloatOrd(mesh_z),
                    // This material is not batched
                    batch_range: 0..1,
                    extra_index: PhaseItemExtraIndex::None,
                    extracted_index: usize::MAX,
                    indexed: mesh.indexed(),
                });
            }
        }
    }
}

// The custom shader can be inline like here, included from another file at build time
// using `include_str!()`, or loaded like any other asset with `asset_server.load()`.
const COLORED_MESH2D_SHADER: &str = r"
// Import the standard 2d mesh uniforms and set their bind groups
#import bevy_sprite::mesh2d_functions

// The structure of the vertex buffer is as specified in `specialize()`
struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) color: u32,
};

struct VertexOutput {
    // The vertex shader must set the on-screen position of the vertex
    @builtin(position) clip_position: vec4<f32>,
    // We pass the vertex color to the fragment shader in location 0
    @location(0) color: vec4<f32>,
};

/// Entry point for the vertex shader
@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    // Project the world position of the mesh into screen position
    let model = mesh2d_functions::get_world_from_local(vertex.instance_index);
    out.clip_position = mesh2d_functions::mesh2d_position_local_to_clip(model, vec4<f32>(vertex.position, 1.0));
    // Unpack the `u32` from the vertex buffer into the `vec4<f32>` used by the fragment shader
    out.color = vec4<f32>((vec4<u32>(vertex.color) >> vec4<u32>(0u, 8u, 16u, 24u)) & vec4<u32>(255u)) / 255.0;
    return out;
}

// The input of the fragment shader must correspond to the output of the vertex shader for all `location`s
struct FragmentInput {
    // The color is interpolated between vertices by default
    @location(0) color: vec4<f32>,
};

/// Entry point for the fragment shader
@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    return in.color;
}
";

/// Handle to the custom shader with a unique random ID
pub const COLORED_MESH2D_SHADER_HANDLE: Handle<Shader> =
    weak_handle!("f48b148f-7373-4638-9900-392b3b3ccc66");

fn star(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mut star = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD
    );

    let mut v_pos = vec![[0.0, 0.0, 0.0]];
    for i in 0..10 {
        let a = i as f32 * PI / 5.0;
        let r = (1 - i % 2) as f32 * 100.0 + 100.0;
        v_pos.push([r * ops::sin(a), r * ops::cos(a), 0.0])
    }

    // Set the position attribute
    star.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);

    // And a RGB color attribute as well. A built-in `Mesh::ATTRIBUTE_COLOR` exists, but we
    // use a custom vertex attribute here for demonstration purposes.
    let mut v_color: Vec<u32> = vec![LinearRgba::BLACK.as_u32()];
    v_color.extend_from_slice(&[LinearRgba::from(YELLOW).as_u32(); 10]);

    star.insert_attribute(
        MeshVertexAttribute::new("Vertex_Color", 1, VertexFormat::Uint32),
        v_color
    );
    
    // triangle list
    let mut indices = vec![0, 1, 10];
    for i in 2..=10 {
        indices.extend_from_slice(&[0, i, i-1]);
    }
    star.insert_indices(Indices::U32(indices));

    // spawn entities for star and camera
    commands.spawn((
            ColoredMesh2d,
            Mesh2d(meshes.add(star))
    ));
    
    commands.spawn(Camera2d);
}

