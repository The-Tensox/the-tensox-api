#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use(bson, doc)]
extern crate mongodb;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate r2d2_mongodb;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate ws;

use std::sync::{Arc, Mutex};

use rocket::{Rocket, Request};
use dotenv::dotenv;
use server::Server;
use std::thread;
use ws::listen;

mod mongo_connection;
mod connection;
mod objects;
mod schema;
mod server;


#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(400)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

pub fn rocket() -> Rocket {
    dotenv().ok();
    let server = Arc::new(Mutex::new(Server { out: None }));
    let clone_server = server.clone();

    // Starting WebSocket server responsible of the notification of updates(POST/PUT/DELETE) to clients
    thread::spawn(move || {
        listen(format!("{}:{}", String::from("127.0.0.1"), 3012), |out| {
            let mut lock = clone_server.try_lock();
            if let Ok(ref mut server) = lock {
                **server = Server { out: Some(out.clone()) };
            } else {
                println!("try_lock failed");
            }
            |_|  Ok(())
        })
        .unwrap()
    });
    
    /*
    thread::spawn(move || {
        loop {
            handle_input_events();
            mechanical_world.step(
                        &mut geometrical_world,
                        &mut body_set,
                        &mut collider_set,
                        &mut constraint_set,
                        &mut force_generator_set,
                    );
            handle_physics_events();
            render_scene();
        }
    });
    */

    let clone_server = server.clone();
    rocket::ignite()
        .register(catchers![internal_error, not_found])
        .manage(mongo_connection::init_pool())
        .manage(clone_server)
        .mount(
            "/objects",
            routes![
                objects::handler::all,
                objects::handler::get,
                objects::handler::post,
                objects::handler::put,
                objects::handler::delete
            ],
        )
}
