use ndarray::prelude::*;
use ndarray::{Array1, Array2};

#[derive(Clone)]
pub struct TriangleMesh {
    pub vertices: Array2<f64>,      // (N, 2)
    pub triangles: Array2<usize>,   // (M, 3)

    // Mesh info properties
    pub areas: Array1<f64>,         // (M)
    pub vertex_neighbor_tris: Vec<Vec<usize>>, // (N) 
}

impl TriangleMesh {
    pub fn new(width : f64, height : f64, shape : (usize, usize)) -> TriangleMesh {
        let (vertices, triangles) = Self::make_beam_mesh(width, height, shape);
       
        let areas = Self::compute_triangle_areas(&vertices, &triangles);
        
        // verify that all areas are positive
        if areas.iter().any(|&a| a <= 0.0) {
            panic!("Could not create mesh! Triangle with negative area found");
        }
        
        let vertex_neighbor_tris = Self::compute_vertex_triangle_adjacency(&vertices, &triangles);

        TriangleMesh {vertices, triangles, areas, vertex_neighbor_tris}
    }
    
    fn make_beam_mesh(width : f64, height : f64, shape : (usize, usize)) -> (Array2<f64>, Array2<usize>) {
        if width < 0.0 || height < 0.0 { panic!("Could not create mesh! Width/height cannot be negative") }
        let x0 = -width/2.0;
        let y0 = -height/2.0;
        let i_max = shape.0;
        let j_max = shape.1;
        let dx = width/(i_max as f64);
        let dy = height/(j_max as f64);
        let vert_count = (i_max+1) * (j_max+1);

        let mut v = Array2::<f64>::zeros((vert_count, 2));
        let mut t = Array2::<usize>::zeros((2*i_max*j_max, 3));
        
        // build vertices
        for j in 0..(j_max+1) {
            for i in 0..(i_max+1) {
                let k: usize = i + j*(i_max + 1);
                v[[k,0]] = x0 + (i as f64)*dx;
                v[[k,1]] = y0 + (j as f64)*dy;   
            }
        }
        // build triangles
        for j in 0..j_max {
            for i in 0..i_max {
                let k00: usize = i + j*(i_max+1); 
                let k01: usize = (i+1) + j*(i_max+1);
                let k10: usize = i + (j+1)*(i_max+1);
                let k11: usize = (i+1) + (j+1)*(i_max+1);
                let e: usize = 2 * (i+j*i_max);
                if (i + j + 1)%2 != 0 {
                    t.slice_mut(s![e, ..]).assign(&array![k00, k01, k11]);
                    t.slice_mut(s![e+1, ..]).assign(&array![k00, k11, k10]);
                } else {
                    t.slice_mut(s![e, ..]).assign(&array![k10, k00, k01]);
                    t.slice_mut(s![e+1, ..]).assign(&array![k10, k01, k11]);
                }
            }
        }
        
        (v, t)
    }
    
    fn compute_triangle_areas(vertices: &Array2<f64>, triangles: &Array2<usize>) -> Array1<f64> {
        let mut areas = Array1::<f64>::zeros(triangles.nrows());

        for (e, tri) in triangles.outer_iter().enumerate() {
            let xi = vertices[[tri[0], 0]];
            let xj = vertices[[tri[1], 0]];
            let xk = vertices[[tri[2], 0]];
            let yi = vertices[[tri[0], 1]];
            let yj = vertices[[tri[1], 1]];
            let yk = vertices[[tri[2], 1]];
   
            let xkj = &xk - &xj;
            let ykj = &yk - &yj;
            let xij = &xi - &xj;
            let yij = &yi - &yj;

            let area = (xkj * yij - xij * ykj) / 2.0;

            areas[[e]] = area;
        }
        areas
    }
    fn compute_vertex_triangle_adjacency(vertices: &Array2<f64>, triangles: &Array2<usize>) -> Vec<Vec<usize>> {
        let vertex_count: usize = vertices.shape()[0];
        let mut vertex_neighbor_tris: Vec<Vec<usize>> = vec![Vec::new(); vertex_count];

        for (triangle_idx, tri) in triangles.outer_iter().enumerate() {
            for &v in tri.iter() { 
                vertex_neighbor_tris[v].push(triangle_idx);
            }
        }
        vertex_neighbor_tris
    }

}
