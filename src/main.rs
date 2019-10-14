#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate ws;

use std::sync::{Arc, Mutex};

use dotenv::dotenv;
use server::Server;
use std::thread;
use ws::listen;

mod connection;
mod objects;
mod schema;
mod server;

fn main() {
    dotenv().ok();
    let mutex = Arc::new(Mutex::new(Server { out: None }));
    let c_mutex = mutex.clone();

    thread::spawn(move || {
        listen(format!("{}:{}", String::from("127.0.0.1"), 3012), |out| {
            let mut lock = c_mutex.try_lock();
            if let Ok(ref mut mutex) = lock {
                **mutex = Server { out: Some(out) };
            } else {
                println!("try_lock failed");
            }
            |_| Ok(())
        })
        .unwrap()
    });

    let c_mutex = mutex.clone();
    //thread::spawn(move || {
    rocket::ignite()
        .manage(connection::init_pool())
        .manage(c_mutex)
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
        .launch();
    //});
}
