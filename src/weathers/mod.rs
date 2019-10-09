#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::weathers;

pub mod handler;
pub mod repository;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
pub struct Weather {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub sun: i32,
}
