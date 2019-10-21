#![allow(proc_macro_derive_resolution_fallback)]
use mongodb::{Bson, bson, doc};
use crate::objects::Object;
use crate::mongo_connection::Conn;
use crate::r2d2_mongodb::mongodb::db::ThreadedDatabase;

pub fn all(connection: &Conn) -> Vec<Object> {
    // objects::table.load::<Object>(&*connection)
        let o = Object { 
                    id: 0, 
                    position_x: Some(0.0),
                    position_y: Some(0.0),
                    position_z: Some(0.0),
                    rotation_x: Some(0.0),
                    rotation_y: Some(0.0),
                    rotation_z: Some(0.0),
                    scale_x: Some(0.0),
                    scale_y: Some(0.0),
                    scale_z: Some(0.0),
                    mass: Some(0.0),
                    velocity_x: Some(0.0),
                    velocity_y: Some(0.0),
                    velocity_z: Some(0.0),
                    collision_x: Some(0.0),
                    collision_y: Some(0.0),
                    collision_z: Some(0.0),
                    height: Some(0.0),
                    radius: Some(0.0),
                    kind: Some(String::from("")),
                };
    Vec::new()
}

pub fn get(id: i32, connection: &Conn) -> Result<Option<Object>, bool> {
    match connection.collection("objects").find_one( Some(doc!{"id": id}), None) {
        Ok(db_result) => {
            match db_result {
                Some(result_doc) => {
                    match bson::from_bson(bson::Bson::Document(result_doc)) {
                        Ok(result_model) => {
                            return Ok(Some(result_model))
                        },
                        Err(err) => {
                            println!("failed to get model from bson");
                            return Err(false)
                        }
                    }
                },
                None => {
                    println!("No model found");
                    return Ok(None)
                }
            }
        },
        Err(err) => {
            println!("Failed to delete doc from database:\n{}",err);
            return Err(false)
        }
    }
}

pub fn insert(objects: Object, connection: &Conn) -> Result<Object, bool> {
    match bson::to_bson(&objects) {
        Ok(model_bson) => {
            match model_bson{
                bson::Bson::Document(model_doc) => {
                    match connection.collection("objects").insert_one( model_doc, None) {
                        Ok(db_result) => {
                            return Ok(objects)
                        },
                        Err(err) => {
                            println!("Failed to insert new model doc into database:\n{}",err);
                            return Err(false)
                        }
                    }
                },
                _ => {
                    println!("Failed to create document from new model bson");
                    return Err(false)
                }
            }
        },
        Err(err) => {
            println!("Failed to create bson from new model:\n{}",err);
            return Err(false)
        }
    }
}

pub fn update(id: i32, objects: Object, connection: &Conn) -> Object {
    /*
    diesel::update(objects::table.find(id))
        .set(&objects)
        .get_result(connection)
    */
    let o = Object { 
                id: 0, 
                position_x: Some(0.0),
                position_y: Some(0.0),
                position_z: Some(0.0),
                rotation_x: Some(0.0),
                rotation_y: Some(0.0),
                rotation_z: Some(0.0),
                scale_x: Some(0.0),
                scale_y: Some(0.0),
                scale_z: Some(0.0),
                mass: Some(0.0),
                velocity_x: Some(0.0),
                velocity_y: Some(0.0),
                velocity_z: Some(0.0),
                collision_x: Some(0.0),
                collision_y: Some(0.0),
                collision_z: Some(0.0),
                height: Some(0.0),
                radius: Some(0.0),
                kind: Some(String::from("")),
            };
    o
}

pub fn delete(id: i32, connection: &Conn) -> i64 {
    0
    // diesel::delete(objects::table.find(id)).execute(connection)
}
