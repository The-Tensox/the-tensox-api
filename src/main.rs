#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate ws;

use std::sync::{Arc, Mutex};

use dotenv::dotenv;
use server::Server;
use ws::{listen};
use std::{thread, time, cell::Cell, rc::Rc};
use weathers::Weather;

mod weathers;
mod objects;
mod schema;
mod connection;
mod server;

fn main() {
    dotenv().ok();
    let mutex = Arc::new(Mutex::new(Server{ out: None }));
    let c_mutex = mutex.clone();

    thread::spawn(move || {
        let count = Rc::new(Cell::new(0));
        listen(format!("{}:{}", String::from("127.0.0.1"), 3012), |out| {
            let mut lock = c_mutex.try_lock();
            if let Ok(ref mut mutex) = lock {
                **mutex = Server{ out: Some(out) };
            } else {
                println!("try_lock failed");
            }
            |_| {
                Ok(())
            }
        }).unwrap()
    });
    let ten_millis = time::Duration::from_millis(1000);

    /*
    loop {
        thread::sleep(ten_millis);
        //println!("{:?}", *mutex.lock().unwrap())
        
        if !mutex.lock().unwrap().out.is_none() {
            println!("Sending a message");
            mutex.lock().unwrap().out.as_ref().unwrap().send("yay");
        }
    }*/

    let c_mutex = mutex.clone();
    //thread::spawn(move || {
        rocket::ignite()
            .manage(connection::init_pool())
            .manage(c_mutex)
            .mount("/weathers",
                routes![weathers::handler::all,
                        weathers::handler::get,
                        weathers::handler::post,
                        weathers::handler::put,
                        weathers::handler::delete],
            )
            .mount("/objects",
                routes![objects::handler::all,
                        objects::handler::get,
                        objects::handler::post,
                        objects::handler::put,
                        objects::handler::delete],
            ).launch();
    //});
}