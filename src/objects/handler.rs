use crate::mongo_connection::Conn;
use crate::objects;
use crate::server::Server;
use mongodb::{doc, error::Error, oid::ObjectId};
use objects::Object;
use rocket::{http::Status, State};
use rocket_contrib::json::Json;
use serde_json::json;
use std::{
    sync::{Arc, Mutex},
};

fn error_status(error: Error) -> Status {
    match error {
        Error::CursorNotFoundError => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[get("/")]
pub fn all(connection: Conn) -> Result<Json<Vec<Object>>, Status> {
    match objects::repository::all(&connection) {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(error_status(err)),
    }
}

#[get("/<id>")]
pub fn get(id: String, connection: Conn) -> Result<Json<Object>, Status> {
    match ObjectId::with_string(&String::from(&id)) {
        Ok(res) => match objects::repository::get(res, &connection) {
            Ok(res) => Ok(Json(res.unwrap())),
            Err(err) => Err(error_status(err)),
        },
        Err(_) => Err(error_status(Error::DefaultError(String::from(
            "Couldn't parse ObjectId",
        )))),
    }
}

#[post("/", format = "application/json", data = "<objects>")]
pub fn post(
    objects: Json<Object>,
    connection: Conn,
    server: State<Arc<Mutex<Server>>>,
) -> Result<Json<Object>, Status> {
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
        }
        Err(err) => Err(error_status(err)),
    }
}

#[put("/<id>", format = "application/json", data = "<objects>")]
pub fn put(
    id: String,
    objects: Json<Object>,
    connection: Conn,
    server: State<Arc<Mutex<Server>>>,
) -> Result<Json<Object>, Status> {
    match ObjectId::with_string(&String::from(&id)) {
        Ok(res) => match objects::repository::update(res, objects.into_inner(), &connection) {
            Ok(res) => {
                if !server.inner().lock().unwrap().out.is_none() {
                    println!("Broadcast PUT");
                    let msg = json!({
                        "protocol": "PUT".to_owned(),
                        "data": &res
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
                Ok(Json(res))
            }
            Err(err) => Err(error_status(err)),
        },
        Err(_) => Err(error_status(Error::DefaultError(String::from(
            "Couldn't parse ObjectId",
        )))),
    }
}

#[delete("/<id>")]
pub fn delete(
    id: String,
    connection: Conn,
    server: State<Arc<Mutex<Server>>>,
) -> Result<Json<String>, Status> {
    match ObjectId::with_string(&String::from(&id)) {
        Ok(res) => match objects::repository::delete(res, &connection) {
            Ok(_) => {
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
                Ok(Json(id))
            }
            Err(err) => Err(error_status(err)),
        },
        Err(_) => Err(error_status(Error::DefaultError(String::from(
            "Couldn't parse ObjectId",
        )))),
    }
}

#[delete("/")]
pub fn delete_all(
    connection: Conn,
    server: State<Arc<Mutex<Server>>>,
) -> Result<Json<bool>, Status> {
    match objects::repository::delete_all(&connection) {
        Ok(_) => {
            if !server.inner().lock().unwrap().out.is_none() {
                println!("Broadcast DELETE ALL");
                let msg = json!({
                    "protocol": "DELETE_ALL".to_owned(),
                    "data": {} // TODO: think & improve the websocket protocol
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
            Ok(Json(true))
        }
        Err(err) => Err(error_status(err)),
    }
}
