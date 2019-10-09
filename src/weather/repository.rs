#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use schema::weather;
use weather::Weather;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Weather>> {
    weather::table.load::<Weather>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Weather> {
    weather::table.find(id).get_result::<Weather>(connection)
}

pub fn insert(weather: Weather, connection: &PgConnection) -> QueryResult<Weather> {
    diesel::insert_into(weather::table)
        .values(&InsertableWeather::from_weather(weather))
        .get_result(connection)
}

pub fn update(id: i32, weather: Weather, connection: &PgConnection) -> QueryResult<Weather> {
    diesel::update(weather::table.find(id))
        .set(&weather)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(weather::table.find(id))
        .execute(connection)
}

#[derive(Insertable)]
#[table_name = "weather"]
struct InsertableWeather {
    pub x: i32,
    pub y: i32,
    pub sun: i32,
}

impl InsertableWeather {

    fn from_weather(weather: Weather) -> InsertableWeather {
        InsertableWeather {
            x: weather.x,
            y: weather.y,
            sun: weather.sun
        }
    }
}