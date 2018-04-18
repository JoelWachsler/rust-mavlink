extern crate mavlink;
use std::sync::Arc;
use std::thread;
use std::env;
use std::time::Duration;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: mavlink-dump (tcp|udpin|udpout):ip:port");
        return;
    }

    let vehicle = Arc::new(mavlink::connect(&args[1]).unwrap());
    
    vehicle.send(&mavlink::request_parameters()).unwrap();
    vehicle.send(&mavlink::request_stream()).unwrap();

    thread::spawn({
        let vehicle = vehicle.clone();
        move || {
            loop {
                let controls: Vec<f32> = vec![ 0.0,
                                               0.0,
                                               0.0,
                                               0.0,
                                               -1.0,
                                               -1.0,
                                               -1.0,
                                               -1.0,
                                               -1.0,
                                               -1.0,
                                               -1.0,
                                               -1.0,
                                               -1.0,
                                               -1.0,
                                               -1.0,
                                               -1.0, ];

                let msg = mavlink::common::MavMessage::HIL_ACTUATOR_CONTROLS(mavlink::common::HIL_ACTUATOR_CONTROLS_DATA {
                    time_usec: 0,
                    flags: 0,
                    controls,
                    mode: 0,
                });

                vehicle.send(&msg).ok();
                // vehicle.send(&mavlink::heartbeat_message()).ok();
                thread::sleep(Duration::from_secs(1));
            }
        }
    });

    loop {
        if let Ok(msg) = vehicle.recv() {
            println!("{:?}", msg);
        } else {
            break;
        }
    }
}