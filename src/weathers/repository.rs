#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use schema::weathers;
use weathers::Weather;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Weather>> {
    weathers::table.load::<Weather>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Weather> {
    weathers::table.find(id).get_result::<Weather>(connection)
}

pub fn insert(weathers: Weather, connection: &PgConnection) -> QueryResult<Weather> {
    diesel::insert_into(weathers::table)
        .values(&InsertableWeather::from_weather(weathers))
        .get_result(connection)
}

pub fn update(id: i32, weathers: Weather, connection: &PgConnection) -> QueryResult<Weather> {
    diesel::update(weathers::table.find(id))
        .set(&weathers)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(weathers::table.find(id))
        .execute(connection)
}

#[derive(Insertable)]
#[table_name = "weathers"]
struct InsertableWeather {
    pub x: i32,
    pub y: i32,
    pub sun: i32,
}

impl InsertableWeather {

    fn from_weather(weathers: Weather) -> InsertableWeather {
        InsertableWeather {
            x: weathers.x,
            y: weathers.y,
            sun: weathers.sun
        }
    }
}