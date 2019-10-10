#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use schema::objects;
use objects::Object;
use objects::InsertableObject;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Object>> {
    objects::table.load::<Object>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Object> {
    objects::table.find(id).get_result::<Object>(connection)
}

pub fn insert(objects: Object, connection: &PgConnection) -> QueryResult<Object> {
    diesel::insert_into(objects::table)
        .values(&InsertableObject::from_object(objects))
        .get_result(connection)
}

pub fn update(id: i32, objects: Object, connection: &PgConnection) -> QueryResult<Object> {
    diesel::update(objects::table.find(id))
        .set(&objects)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(objects::table.find(id))
        .execute(connection)
}