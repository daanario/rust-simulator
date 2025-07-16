use crate::mesh::TriangleMesh;
use crate::sim::cauchy_fvm::CauchyFVM;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use ndarray::array;
use crate::window;

pub fn beam_example1() -> () {
    let tmesh = TriangleMesh::new_beam(6.0, 2.0, (12, 4));
    let sim = Arc::new(Mutex::new(CauchyFVM::new(&tmesh, "rubber", 6e-4)));
    // thread loop
    let sim_thread = sim.clone();
    thread::spawn(move || {
        let mut secs_5 = false;
        let mut secs_10 = false;
        let mut secs_15 = false;
        let mut secs_20 = false;
        let mut secs_25 = false;
        let mut secs_30 = false;
        let mut secs_35 = false;
        let mut secs_40 = false;

        loop {
            {
                let mut sim = sim_thread.lock().unwrap();
                sim.update();
                if sim.t > 5.0 && !secs_5 {
                    sim.set_immovable_boundary("leftright");
                    sim.set_traction_boundary("down");
                    sim.set_traction_force(array![0.0, -5e5]);
                    secs_5 = true;
                }
                if sim.t > 10.0 && !secs_10 {
                    sim.set_immovable_boundary("leftright");
                    sim.set_traction_boundary("down");
                    sim.set_traction_force(array![0.0, 0.0]);
                    secs_10 = true;
                }
                if sim.t > 15.0 && !secs_15 {
                    sim.set_immovable_boundary("left");
                    sim.set_traction_boundary("right");
                    sim.set_traction_force(array![0.0, 1e4]);
                    secs_15 = true;
                }
                if sim.t > 20.0 && !secs_20 {
                    sim.set_immovable_boundary("leftright");
                    sim.set_traction_boundary("down");
                    sim.set_traction_force(array![0.0, 0.0]);
                    secs_20 = true;
                }
                if sim.t > 25.0 && !secs_25 {
                    sim.set_immovable_boundary("left");
                    sim.set_traction_boundary("right");
                    sim.set_traction_force(array![0.0, 1e5]);
                    secs_25 = true;
                }
                if sim.t > 30.0 && !secs_30 {
                    sim.set_immovable_boundary("leftright");
                    sim.set_traction_boundary("up");
                    sim.set_traction_force(array![0.0, 0.0]);
                    secs_30 = true;
                } 
                if sim.t > 35.0 && !secs_35 {
                    sim.set_immovable_boundary("left");
                    sim.set_traction_boundary("right");
                    sim.set_traction_force(array![1e5, 0.0]);
                    secs_35 = true;
                }
                if sim.t > 40.0 && !secs_40 {
                    sim.set_immovable_boundary("left");
                    sim.set_traction_boundary("right");
                    sim.set_traction_force(array![0.0, 0.0]);
                    secs_40 = true;
                }
            }
            std::thread::sleep(Duration::from_nanos(1)); 
        }
    });
 
    window::create_sim_window_threaded(sim);
    ()
}

pub fn ball_example1() -> () {
    let tmesh = TriangleMesh::new_ball(5);
    let sim = Arc::new(Mutex::new(CauchyFVM::new(&tmesh, "rubber", 7e-4)));
    // thread loop
    let sim_thread = sim.clone();
    thread::spawn(move || {
        let mut secs_5 = false;
        let mut secs_5_5 = false;

        loop {
            {
                let mut sim = sim_thread.lock().unwrap();
                sim.update();
                /*
                if sim.t > 2.0 && !secs_5 {
                    sim.set_immovable_boundary("leftright");
                    sim.set_traction_boundary("down");
                    sim.set_traction_force(array![0.0, 0.0]);
                    secs_5 = true;
                }
                if sim.t > 2.1 && !secs_5_5 {
                    sim.set_immovable_boundary("leftright");
                    sim.set_traction_boundary("down");
                    sim.set_traction_force(array![0.0, 0.0]);
                    secs_5_5 = true;
                }
                */
            }
            std::thread::sleep(Duration::from_nanos(1)); 
        }
    });
 
    window::create_sim_window_threaded(sim);
}

pub fn beam_example2() -> () {
    // LARGE beam
    let tmesh = TriangleMesh::new_beam(60.0, 20.0, (12, 4));
    let sim = Arc::new(Mutex::new(CauchyFVM::new(&tmesh, "rubber", 1e-5))); 
    // thread loop
    let sim_thread = sim.clone();
    thread::spawn(move || {
        let mut secs_5 = false;
        let mut secs_5_5 = false;

        loop {
            {
                let mut sim = sim_thread.lock().unwrap();
                sim.update();
                /*
                if sim.t > 2.0 && !secs_5 {
                    sim.set_immovable_boundary("leftright");
                    sim.set_traction_boundary("down");
                    sim.set_traction_force(array![0.0, 0.0]);
                    secs_5 = true;
                }
                if sim.t > 2.1 && !secs_5_5 {
                    sim.set_immovable_boundary("leftright");
                    sim.set_traction_boundary("down");
                    sim.set_traction_force(array![0.0, 0.0]);
                    secs_5_5 = true;
                }
                */
            }
            std::thread::sleep(Duration::from_nanos(1)); 
        }
    });
 
    window::create_sim_window_threaded(sim);
}
