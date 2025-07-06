mod mesh;
mod cv;
mod plotting;
mod sim;  
mod window;
mod examples;

use crate::mesh::TriangleMesh;
use crate::sim::cauchy_fvm::CauchyFVM;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use ndarray::array;

fn main() -> () {
    examples::beam_example1();
    
    // Benchmarking code
    // Create triangle mesh
    //let tmesh = TriangleMesh::new(6.0, 2.0, (12, 4));
        
    // Create FVM simulator
    //let mut sim = CauchyFVM::new(&tmesh); 
    //sim.benchmark(5000); 
 
    // plot mesh if we want
    //plotting::plot_triangle_mesh(&sim.sim_mesh, "mesh.png"); 
}

