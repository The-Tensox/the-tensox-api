#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::weather;

pub mod handler;
pub mod repository;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "weather"]
pub struct Weather {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub sun: i32,
}
