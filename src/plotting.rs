use std::error::Error;
use crate::mesh::TriangleMesh;
use crate::cv::MedianCentroidControlVolume;
use plotters::prelude::*;
use plotters::prelude::full_palette::PINK; 

use plotters_gtk4::Paintable;
use plotters_gtk4::PaintableBackend;

pub fn plot_triangle_mesh(mesh: &TriangleMesh, filename: &str) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(filename, (600,400)).into_drawing_area();
    root.fill(&WHITE)?;
    
    let mut chart = ChartBuilder::on(&root)
        .caption("Mesh", ("sans-serif", 12))
        .build_cartesian_2d(-6.0..6.0, -4.0..4.0)?;
    
    chart.configure_mesh().draw()?;
    
    for tri in mesh.triangles.outer_iter() {
        let triangle = vec![
            (mesh.vertices[[tri[0], 0]], mesh.vertices[[tri[0], 1]]),
            (mesh.vertices[[tri[1], 0]], mesh.vertices[[tri[1], 1]]),
            (mesh.vertices[[tri[2], 0]], mesh.vertices[[tri[2], 1]]),
            (mesh.vertices[[tri[0], 0]], mesh.vertices[[tri[0], 1]]), // Close the loop
        ];

        chart.draw_series(std::iter::once(
                PathElement::new(triangle, &BLACK)))?;
        }

    root.present()?;

    Ok(())

}

pub fn plot_triangle_mesh_with_highlight(
    mesh: &TriangleMesh,
    highlight_vertex: usize,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_range = -6.0..6.0;
    let y_range = -4.0..4.0;

    let mut chart = ChartBuilder::on(&root)
        .caption("Mesh with Highlight", ("sans-serif", 20))
        .margin(10)
        .build_cartesian_2d(x_range, y_range)?;

    chart.configure_mesh().draw()?;

    // First: draw all triangles as thin outlines
    for tri in mesh.triangles.outer_iter() {
        let i = tri[0];
        let j = tri[1];
        let k = tri[2];

        let triangle = vec![
            (mesh.vertices[[i, 0]], mesh.vertices[[i, 1]]),
            (mesh.vertices[[j, 0]], mesh.vertices[[j, 1]]),
            (mesh.vertices[[k, 0]], mesh.vertices[[k, 1]]),
            (mesh.vertices[[i, 0]], mesh.vertices[[i, 1]]),
        ];

        chart.draw_series(std::iter::once(
            PathElement::new(triangle, &BLACK.mix(0.2)),
        ))?;
    }

    // Then: highlight the triangles adjacent to `highlight_vertex`
    if let Some(tris) = mesh.vertex_neighbor_tris.get(highlight_vertex) {
        for &tid in tris {
            let tri = mesh.triangles.row(tid);
            let i = tri[0];
            let j = tri[1];
            let k = tri[2];

            let triangle = vec![
                (mesh.vertices[[i, 0]], mesh.vertices[[i, 1]]),
                (mesh.vertices[[j, 0]], mesh.vertices[[j, 1]]),
                (mesh.vertices[[k, 0]], mesh.vertices[[k, 1]]),
                (mesh.vertices[[i, 0]], mesh.vertices[[i, 1]]), // close loop
            ];

            chart.draw_series(std::iter::once(
                PathElement::new(triangle, &BLUE.mix(0.6)),
            ))?;
        }
    }

    // Finally: draw the highlighted vertex as a red dot
    let x = mesh.vertices[[highlight_vertex, 0]];
    let y = mesh.vertices[[highlight_vertex, 1]];

    chart.draw_series(std::iter::once(
        Circle::new((x, y), 5, RED.filled()),
    ))?;

    root.present()?;
    Ok(())
}

pub fn plot_triangle_mesh_with_cv(
    mesh: &TriangleMesh,
    control_volume: &MedianCentroidControlVolume,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_range = -6.0..6.0;
    let y_range = -4.0..4.0;

    let mut chart = ChartBuilder::on(&root)
        .caption("Mesh with Control Volume", ("sans-serif", 20))
        .margin(10)
        .build_cartesian_2d(x_range, y_range)?;

    chart.configure_mesh().draw()?;

    // First: draw all triangles as thin outlines
    for tri in mesh.triangles.outer_iter() {
        let i = tri[0];
        let j = tri[1];
        let k = tri[2];

        let triangle = vec![
            (mesh.vertices[[i, 0]], mesh.vertices[[i, 1]]),
            (mesh.vertices[[j, 0]], mesh.vertices[[j, 1]]),
            (mesh.vertices[[k, 0]], mesh.vertices[[k, 1]]),
            (mesh.vertices[[i, 0]], mesh.vertices[[i, 1]]),
        ];

        chart.draw_series(std::iter::once(
            PathElement::new(triangle, &BLACK.mix(0.2)),
        ))?;
    }
    
    let center_vertex = control_volume.vertex;

    // Then: highlight the triangles adjacent to `center_vertex` i.e. all neighbor triangles of the
    // control volume center node
    for triangle in control_volume.neighbor_triangles.outer_iter() {
        let i = triangle[[0]];
        let j = triangle[[1]];
        let k = triangle[[2]];

        let polygon = vec![
            (mesh.vertices[[i, 0]], mesh.vertices[[i, 1]]),
            (mesh.vertices[[j, 0]], mesh.vertices[[j, 1]]),
            (mesh.vertices[[k, 0]], mesh.vertices[[k, 1]]),
            (mesh.vertices[[i, 0]], mesh.vertices[[i, 1]]), // close loop
        ];

        chart.draw_series(std::iter::once(
                PathElement::new(polygon, &RED.mix(0.6)),
        ))?;
    }
    

    // Finally: draw the highlighted vertex as a red dot
    let x = mesh.vertices[[center_vertex, 0]];
    let y = mesh.vertices[[center_vertex, 1]];

    chart.draw_series(std::iter::once(
        Circle::new((x, y), 5, RED.filled()),
    ))?;

    root.present()?;
    Ok(())
}

pub fn draw_triangle_mesh_on_area(mesh: &TriangleMesh, paintable: &Paintable) -> () {
    let backend = PaintableBackend::new(paintable);
    let root = backend.into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Beam mesh", ("sans-serif", 12))
        .build_cartesian_2d(-3.1..6.0, -4.0..4.0)
        .unwrap();
    
    // draw a background grid
    //chart.configure_mesh().draw().unwrap();

    for tri in mesh.triangles.outer_iter() {
        let triangle = vec![
            (mesh.vertices[[tri[0], 0]], mesh.vertices[[tri[0], 1]]),
            (mesh.vertices[[tri[1], 0]], mesh.vertices[[tri[1], 1]]),
            (mesh.vertices[[tri[2], 0]], mesh.vertices[[tri[2], 1]]),
            (mesh.vertices[[tri[0], 0]], mesh.vertices[[tri[0], 1]]), // close loop
        ];

        chart
            .draw_series(std::iter::once(PathElement::new(triangle, &PINK)))
            .unwrap();
    }

    root.present().unwrap();
}
