use connection::DbConn;
use diesel::result::Error;
use objects;
use objects::Object;
use rocket::{State, http::Status, response::status};
use rocket_contrib::json::Json;
use server::Server;
use std::{env, sync::{Arc, Mutex}};
use serde_json::json;


#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Object>>, Status> {
    objects::repository::all(&connection)
        .map(|objects| Json(objects))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<Object>, Status> {
    objects::repository::get(id, &connection)
        .map(|objects| Json(objects))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<objects>")]
pub fn post(objects: Json<Object>, connection: DbConn, server: State<Arc<Mutex<Server>>>) -> Result<status::Created<Json<Object>>, Status> {
    objects::repository::insert(objects.into_inner(), &connection)
        .map(|objects| {
            let res = object_created(objects.clone());
            if !server.inner().lock().unwrap().out.is_none() {
                println!("Broadcast POST");
                let msg = json!({
                    "protocol": "POST".to_owned(),
                    "data": &objects
                });
                // inner() get the thing inside State, then lock mutex, unwrap ...
                // We send the serialized data
                server.inner().lock().unwrap().out.as_ref().unwrap().broadcast(serde_json::to_string(&msg).unwrap());
            } else {
                println!("No clients connected");
            }

            res
        })
        .map_err(|error| error_status(error))
}

fn object_created(objects: Object) -> status::Created<Json<Object>> {
    status::Created(
        format!("{host}:{port}/objects/{id}", host = host(), port = port(), id = objects.id).to_string(),
        Some(Json(objects)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

#[put("/<id>", format = "application/json", data = "<objects>")]
pub fn put(id: i32, objects: Json<Object>, connection: DbConn, server: State<Arc<Mutex<Server>>>) -> Result<Json<Object>, Status> {
    objects::repository::update(id, objects.into_inner(), &connection)
        .map(|objects| {
            if !server.inner().lock().unwrap().out.is_none() {
                println!("Broadcast PUT");
                let msg = json!({
                    "protocol": "PUT".to_owned(),
                    "data": &objects
                });
                // Get the Json<Weather>> inside Created with .0 and converts it to string to send
                // Broadcast the new item to all clients
                // inner() get the thing inside State, then lock mutex, unwrap ...
                // We send the serialized data
                server.inner().lock().unwrap().out.as_ref().unwrap().broadcast(serde_json::to_string(&msg).unwrap());
            } else {
                println!("No clients connected");
            }
            Json(objects)
        })
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn, server: State<Arc<Mutex<Server>>>) -> Result<Status, Status> {
    match objects::repository::get(id, &connection) {
        Ok(_) => objects::repository::delete(id, &connection)
            .map(|_| {
                if !server.inner().lock().unwrap().out.is_none() {
                    println!("Broadcast DELETE");
                    let msg = json!({
                        "protocol": "DELETE".to_owned(),
                        "data": {
                            "id": id 
                        }
                    });
                    // Get the Json<Weather>> inside Created with .0 and converts it to string to send
                    // Broadcast the new item to all clients
                    // inner() get the thing inside State, then lock mutex, unwrap ...
                    // We send the serialized data
                    server.inner().lock().unwrap().out.as_ref().unwrap().broadcast(serde_json::to_string(&msg).unwrap());
                } else {
                    println!("No clients connected");
                }
                Status::NoContent
            })
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}
