#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::objects;

pub mod handler;
pub mod repository;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug, Clone)]
pub struct Object {
    pub id: i32,
    pub position_x: Option<f32>,
    pub position_y: Option<f32>,
    pub position_z: Option<f32>,
    pub rotation_x: Option<f32>,
    pub rotation_y: Option<f32>,
    pub rotation_z: Option<f32>,
    pub scale_x: Option<f32>,
    pub scale_y: Option<f32>,
    pub scale_z: Option<f32>,
    pub mass: Option<f32>,
    pub velocity_x: Option<f32>,
    pub velocity_y: Option<f32>,
    pub velocity_z: Option<f32>,
    pub collision_x: Option<f32>,
    pub collision_y: Option<f32>,
    pub collision_z: Option<f32>,
    pub height: Option<f32>,
    pub radius: Option<f32>,
    pub kind: Option<String>,
}

#[derive(Insertable, AsChangeset)]
#[changeset_options(treat_none_as_null = "true")]
#[table_name = "objects"]
pub struct InsertableObject {
    pub position_x: Option<f32>,
    pub position_y: Option<f32>,
    pub position_z: Option<f32>,
    pub rotation_x: Option<f32>,
    pub rotation_y: Option<f32>,
    pub rotation_z: Option<f32>,
    pub scale_x: Option<f32>,
    pub scale_y: Option<f32>,
    pub scale_z: Option<f32>,
    pub mass: Option<f32>,
    pub velocity_x: Option<f32>,
    pub velocity_y: Option<f32>,
    pub velocity_z: Option<f32>,
    pub collision_x: Option<f32>,
    pub collision_y: Option<f32>,
    pub collision_z: Option<f32>,
    pub height: Option<f32>,
    pub radius: Option<f32>,
    pub kind: Option<String>,
}

impl InsertableObject {
    fn from_object(objects: Object) -> InsertableObject {
        InsertableObject {
            position_x: objects.position_x,
            position_y: objects.position_y,
            position_z: objects.position_z,
            rotation_x: objects.rotation_x,
            rotation_y: objects.rotation_y,
            rotation_z: objects.rotation_z,
            scale_x: objects.scale_x,
            scale_y: objects.scale_y,
            scale_z: objects.scale_z,
            mass: objects.mass,
            velocity_x: objects.velocity_x,
            velocity_y: objects.velocity_y,
            velocity_z: objects.velocity_z,
            collision_x: objects.collision_x,
            collision_y: objects.collision_y,
            collision_z: objects.collision_z,
            height: objects.height,
            radius: objects.radius,
            kind: objects.kind,
        }
    }
}
