use connection::DbConn;
use diesel::result::Error;
use std::env;
use objects;
use objects::Object;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

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
pub fn post(objects: Json<Object>, connection: DbConn) -> Result<status::Created<Json<Object>>, Status> {
    objects::repository::insert(objects.into_inner(), &connection)
        .map(|objects| object_created(objects))
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
pub fn put(id: i32, objects: Json<Object>, connection: DbConn) -> Result<Json<Object>, Status> {
    objects::repository::update(id, objects.into_inner(), &connection)
        .map(|objects| Json(objects))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match objects::repository::get(id, &connection) {
        Ok(_) => objects::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}
