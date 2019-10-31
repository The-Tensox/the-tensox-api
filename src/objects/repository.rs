#![allow(proc_macro_derive_resolution_fallback)]
use crate::mongo_connection::Conn;
use crate::objects::{InsertableObject, Object};
use crate::r2d2_mongodb::mongodb::db::ThreadedDatabase;
use mongodb::{bson, coll::results::DeleteResult, doc, error::Error, oid::ObjectId, coll::options::FindOptions};

const COLLECTION: &str = "objects";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ObjectsItemsLeft {
    pub objects: Vec<Object>,
    pub items_left: i64
}

/// Returns objects and items left
/// items left are computed as follow: total items - skip option - batch size (found)
/// 
/// # Arguments
///
/// * `options` - Options for the query
///
pub fn all(options: Option<FindOptions>, connection: &Conn) -> Result<ObjectsItemsLeft, Error> {
    let cursor = connection.collection(COLLECTION).find(None, options.clone()).unwrap();
    let mut items_left = count(connection).expect("");
    let objects = cursor
        .map(|result| match result {
            Ok(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
                Ok(result_model) => Ok(result_model),
                Err(_) => Err(Error::DefaultError(String::from(""))),
            },
            Err(err) => Err(err),
        })
        .collect::<Result<Vec<Object>, Error>>();
    match objects {
        Ok(obj) => {
                match options {
                    Some(o) => items_left = items_left - o.skip.unwrap_or(0) - obj.len() as i64,
                    None => items_left = 0 // No options = we want to get it all
                }
                Ok(ObjectsItemsLeft{objects: obj.clone(), items_left: items_left})
            },
        Err(err) => Err(err)
    }
}

pub fn get(id: ObjectId, connection: &Conn) -> Result<Option<Object>, Error> {
    match connection
        .collection(COLLECTION)
        .find_one(Some(doc! {"_id": id}), None)
    {
        Ok(db_result) => match db_result {
            Some(result_doc) => match bson::from_bson(bson::Bson::Document(result_doc)) {
                Ok(result_model) => Ok(Some(result_model)),
                Err(_) => Err(Error::DefaultError(String::from(
                    "Failed to create reverse BSON",
                ))),
            },
            None => Ok(None),
        },
        Err(err) => Err(err),
    }
}

pub fn insert(objects: Object, connection: &Conn) -> Result<Object, Error> {
    let insertable = InsertableObject::from_object(objects.clone());
    match bson::to_bson(&insertable) {
        Ok(model_bson) => match model_bson {
            bson::Bson::Document(model_doc) => {
                match connection
                    .collection(COLLECTION)
                    .insert_one(model_doc, None)
                {
                    Ok(res) => match res.inserted_id {
                        Some(res) => match bson::from_bson(res) {
                            Ok(res) => Ok(InsertableObject::assign_id(insertable, res)),
                            Err(_) => Err(Error::DefaultError(String::from("Failed to read BSON"))),
                        },
                        None => Err(Error::DefaultError(String::from("None"))),
                    },
                    Err(err) => Err(err),
                }
            }
            _ => Err(Error::DefaultError(String::from(
                "Failed to create Document",
            ))),
        },
        Err(_) => Err(Error::DefaultError(String::from("Failed to create BSON"))),
    }
}

pub fn update(id: ObjectId, objects: Object, connection: &Conn) -> Result<Object, Error> {
    let mut new_object = objects.clone();
    new_object.id = Some(id.clone());
    match bson::to_bson(&new_object) {
        Ok(model_bson) => match model_bson {
            bson::Bson::Document(model_doc) => {
                match connection.collection(COLLECTION).replace_one(
                    doc! {"_id": id},
                    model_doc,
                    None,
                ) {
                    Ok(_) => Ok(new_object),
                    Err(err) => Err(err),
                }
            }
            _ => Err(Error::DefaultError(String::from(
                "Failed to create Document",
            ))),
        },
        Err(_) => Err(Error::DefaultError(String::from("Failed to create BSON"))),
    }
}

pub fn delete(id: ObjectId, connection: &Conn) -> Result<DeleteResult, Error> {
    connection
        .collection(COLLECTION)
        .delete_one(doc! {"_id": id}, None)
}

pub fn delete_all(connection: &Conn) -> Result<(), Error> {
    connection.collection(COLLECTION).drop()
}

pub fn count(connection: &Conn) -> Result<i64, Error> {
    connection.collection(COLLECTION).count(None, None)
}