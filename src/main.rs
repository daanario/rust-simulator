mod mesh;
mod cv;
mod plotting;
mod sim;  
mod window;

use crate::mesh::TriangleMesh;
use crate::sim::cauchy_fvm::CauchyFVM;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use ndarray::array;

fn main() -> () { 
    let tmesh = TriangleMesh::new(6.0, 2.0, (12, 4));
    let sim = Arc::new(Mutex::new(CauchyFVM::new(&tmesh, "rubber", 6e-4)));
    // thread loop
    let sim_thread = sim.clone();
    thread::spawn(move || {
        let mut secs_5 = false;
        let mut secs_10 = false;
        let mut secs_15 = false;
        let mut secs_20 = false;

        loop {
            {
                let mut sim = sim_thread.lock().unwrap();
                sim.update();
                if sim.t > 5.0 && !secs_5 {
                    sim.set_immovable_boundary("leftright");
                    sim.set_traction_boundary("down");
                    sim.set_traction_force(array![0.0, -1e6]);
                    secs_5 = true;
                }
                if sim.t > 10.0 && !secs_10 {
                    sim.set_immovable_boundary("left");
                    sim.set_traction_boundary("right");
                    sim.set_traction_force(array![0.0, 1e5]);
                    secs_10 = true;
                }
                if sim.t > 15.0 && !secs_15 {
                    sim.set_immovable_boundary("leftright");
                    sim.set_traction_boundary("up");
                    sim.set_traction_force(array![0.0, 1e6]);
                    secs_15 = true;
                }
                if sim.t > 20.0 && !secs_20 {
                    sim.set_immovable_boundary("left");
                    sim.set_traction_boundary("right");
                    sim.set_traction_force(array![0.0, -1e5]);
                    secs_20 = true;
                }


            }
            std::thread::sleep(Duration::from_nanos(1)); 
        }
    });
    
    //window::create_sim_window();
    window::create_sim_window_threaded(sim);
    ()
    
    // Benchmarking code
    // Create triangle mesh
    //let tmesh = TriangleMesh::new(6.0, 2.0, (12, 4));
        
    // Create FVM simulator
    //let mut sim = CauchyFVM::new(&tmesh); 
    //sim.benchmark(5000); 
 
    // plot mesh if we want
    //plotting::plot_triangle_mesh(&sim.sim_mesh, "mesh.png"); 
}

