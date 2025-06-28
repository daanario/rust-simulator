use std::error::Error;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use plotters_gtk4::Paintable;

mod plotting;
mod mesh;
mod cv;
mod simulator;

use glib::source::timeout_add_local;
use std::cell::RefCell;
use std::rc::Rc;
//use glib::Continue;
//use crate::glib::ControlFlow::Continue;

fn main() -> glib::ExitCode {   
    let app = Application::builder().
        application_id("org.example.HelloWorld").
        build();

    app.connect_activate(|app| {
        // create main window
        let window = ApplicationWindow::builder().
            application(app).
            default_width(640).
            default_height(480).
            title("Hello World!").
            build();
        let paintable = Paintable::new((640,480)); 
        let image = gtk::Picture::for_paintable(&paintable);
        window.set_child(Some(&image));
        
        // Create triangle mesh
        let tmesh = mesh::TriangleMesh::new(6.0, 2.0, (12, 4));
        
        // Create refcell simulator
        let sim = Rc::new(RefCell::new(simulator::CauchyFVM::new(&tmesh)));
        
        {
            let paintable = paintable.clone();
            let sim = sim.clone();
            
            glib::idle_add_local(move || {
                sim.borrow_mut().update();
                plotting::draw_triangle_mesh_on_area(&sim.borrow().sim_mesh, &paintable);
                glib::ControlFlow::Continue
            });

            // timed intervals
            /*
            glib::timeout_add_local(std::time::Duration::from_millis(25), move || {
                sim.borrow_mut().update();
                plotting::draw_triangle_mesh_on_area(&sim.borrow().sim_mesh, &paintable);
                glib::ControlFlow::Continue
            });
            */
        }

        // show window
        window.present();
    }); 
    
    app.run()
    
    // create control volume for node 30
    //let control_volume = cv::MedianCentroidControlVolume::new(30, &tmesh);

    // plot and control volume
    //plotting::plot_triangle_mesh_with_cv(&tmesh, &control_volume, "mesh.png") 
    
}

