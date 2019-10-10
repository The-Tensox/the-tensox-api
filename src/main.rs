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

use dotenv::dotenv;

mod weathers;
mod objects;
mod schema;
mod connection;
mod server;

fn main() {
    dotenv().ok();
    server::init_server(String::from("127.0.0.1"), 3012);
    rocket::ignite()
        .manage(connection::init_pool())
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
}