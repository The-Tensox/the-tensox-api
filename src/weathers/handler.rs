use connection::DbConn;
use diesel::result::Error;
use std::env;
use weathers;
use weathers::Weather;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Weather>>, Status> {
    weathers::repository::all(&connection)
        .map(|weathers| Json(weathers))
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}

#[get("/<id>")]
pub fn get(id: i32, connection: DbConn) -> Result<Json<Weather>, Status> {
    weathers::repository::get(id, &connection)
        .map(|weathers| Json(weathers))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<weathers>")]
pub fn post(weathers: Json<Weather>, connection: DbConn) -> Result<status::Created<Json<Weather>>, Status> {
    weathers::repository::insert(weathers.into_inner(), &connection)
        .map(|weathers| weather_created(weathers))
        .map_err(|error| error_status(error))
}

fn weather_created(weathers: Weather) -> status::Created<Json<Weather>> {
    status::Created(
        format!("{host}:{port}/weathers/{id}", host = host(), port = port(), id = weathers.id).to_string(),
        Some(Json(weathers)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

#[put("/<id>", format = "application/json", data = "<weathers>")]
pub fn put(id: i32, weathers: Json<Weather>, connection: DbConn) -> Result<Json<Weather>, Status> {
    weathers::repository::update(id, weathers.into_inner(), &connection)
        .map(|weathers| Json(weathers))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match weathers::repository::get(id, &connection) {
        Ok(_) => weathers::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}
