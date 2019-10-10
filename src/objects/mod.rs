#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::objects;

pub mod handler;
pub mod repository;

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
pub struct Object {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub scale_x: Option<f32>,
    pub scale_y: Option<f32>,
    pub scale_z: Option<f32>,
    pub height: Option<f32>,
    pub radius: Option<f32>
}

#[derive(Insertable)]
// #[changeset_options(treat_none_as_null="true")]// https://github.com/diesel-rs/diesel/blob/master/guide_drafts/trait_derives.md#aschangeset
#[table_name = "objects"]
pub struct InsertableObject {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub scale_x: Option<f32>,
    pub scale_y: Option<f32>,
    pub scale_z: Option<f32>,
    pub height: Option<f32>,
    pub radius: Option<f32>
}

impl InsertableObject {

    fn from_object(objects: Object) -> InsertableObject {
        InsertableObject {
            x: objects.x,
            y: objects.y,
            z: objects.z,
            scale_x: objects.scale_x,
            scale_y: objects.scale_y,
            scale_z: objects.scale_z,
            height: objects.height,
            radius: objects.radius
        }
    }
}