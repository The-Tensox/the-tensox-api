use connection::DbConn;
use diesel::result::Error;
use std::env;
use weather;
use weather::Weather;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Weather>>, Status> {
    weather::repository::all(&connection)
        .map(|weather| Json(weather))
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
    weather::repository::get(id, &connection)
        .map(|weather| Json(weather))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<weather>")]
pub fn post(weather: Json<Weather>, connection: DbConn) -> Result<status::Created<Json<Weather>>, Status> {
    weather::repository::insert(weather.into_inner(), &connection)
        .map(|weather| weather_created(weather))
        .map_err(|error| error_status(error))
}

fn weather_created(weather: Weather) -> status::Created<Json<Weather>> {
    status::Created(
        format!("{host}:{port}/weather/{id}", host = host(), port = port(), id = weather.id).to_string(),
        Some(Json(weather)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

#[put("/<id>", format = "application/json", data = "<weather>")]
pub fn put(id: i32, weather: Json<Weather>, connection: DbConn) -> Result<Json<Weather>, Status> {
    weather::repository::update(id, weather.into_inner(), &connection)
        .map(|weather| Json(weather))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    match weather::repository::get(id, &connection) {
        Ok(_) => weather::repository::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error))
    }
}
