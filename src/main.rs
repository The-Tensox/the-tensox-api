#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;

mod weathers;
mod schema;
mod connection;

fn main() {
    dotenv().ok();
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/weathers",
               routes![weathers::handler::all,
                    weathers::handler::get,
                    weathers::handler::post,
                    weathers::handler::put,
                    weathers::handler::delete],
        ).launch();
}
