use crate::mongo_connection::Conn;
use diesel::result::Error;
use crate::objects;
use objects::Object;
use rocket::{http::Status, response::status, State};
use rocket_contrib::json::Json;
use serde_json::json;
use crate::server::Server;
use std::{
    env,
    sync::{Arc, Mutex},
};

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[get("/")]
pub fn all(connection: Conn) -> Json<Vec<Object>> {
    Json(objects::repository::all(&connection))
}

#[get("/<id>")]
pub fn get(id: i32, connection: Conn) -> Result<Json<Object>, bool> {
    match objects::repository::get(id, &connection) {
        Ok(res) => Ok(Json(res.unwrap())),
        Err(err) => Err(false)//error_status(err)
    }
}

#[post("/", format = "application/json", data = "<objects>")]
pub fn post(
    objects: Json<Object>,
    connection: Conn,
    server: State<Arc<Mutex<Server>>>,
) -> Result<Json<Object>, bool> {
    match objects::repository::insert(objects.into_inner(), &connection) {
        Ok(res) => {
            if !server.inner().lock().unwrap().out.is_none() {
                println!("Broadcast POST");
                let msg = json!({
                    "protocol": "POST".to_owned(),
                    "data": &res
                });
                // inner() get the thing inside State, then lock mutex, unwrap ...
                // We send the serialized data
                server
                    .inner()
                    .lock()
                    .unwrap()
                    .out
                    .as_ref()
                    .unwrap()
                    .broadcast(serde_json::to_string(&msg).unwrap())
                    .expect("Failed to broadcast");
            } else {
                println!("No clients connected");
            }
            Ok(Json(res))
        },
        Err(err) => Err(false)
    }
}

#[put("/<id>", format = "application/json", data = "<objects>")]
pub fn put(
    id: i32,
    objects: Json<Object>,
    connection: Conn,
    server: State<Arc<Mutex<Server>>>,
) -> Json<Object> {
    let object = objects::repository::update(id, objects.into_inner(), &connection);
    if !server.inner().lock().unwrap().out.is_none() {
        println!("Broadcast PUT");
        let msg = json!({
            "protocol": "PUT".to_owned(),
            "data": &object
        });
        // Get the Json<Object>> inside Created with .0 and converts it to string to send
        // Broadcast the new item to all clients
        // inner() get the thing inside State, then lock mutex, unwrap ...
        // We send the serialized data
        server
            .inner()
            .lock()
            .unwrap()
            .out
            .as_ref()
            .unwrap()
            .broadcast(serde_json::to_string(&msg).unwrap())
            .expect("Failed to broadcast");
    } else {
        println!("No clients connected");
    }
    Json(object)
}

#[delete("/<id>")]
pub fn delete(
    id: i32,
    connection: Conn,
    server: State<Arc<Mutex<Server>>>,
) -> Json<i64> {
    let id = objects::repository::delete(id, &connection);
    if !server.inner().lock().unwrap().out.is_none() {
        println!("Broadcast DELETE");
        let msg = json!({
            "protocol": "DELETE".to_owned(),
            "data": {
                "id": &id
            }
        });
        // Get the Json<Object>> inside Created with .0 and converts it to string to send
        // Broadcast the new item to all clients
        // inner() get the thing inside State, then lock mutex, unwrap ...
        // We send the serialized data
        server
            .inner()
            .lock()
            .unwrap()
            .out
            .as_ref()
            .unwrap()
            .broadcast(serde_json::to_string(&msg).unwrap())
            .expect("Failed to broadcast");
    } else {
        println!("No clients connected");
    }
    Json(id)
}
