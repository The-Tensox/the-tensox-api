#![allow(proc_macro_derive_resolution_fallback)]

pub mod handler;
pub mod repository;
use mongodb::bson;
/*
trait Shape {

}

pub struct CapsuleShape {

}

impl Shape for CapsuleShape {
    
}

pub struct BoxShape {

}

impl Shape for BoxShape {
    
}

pub struct ConvexShape {

}

impl Shape for ConvexShape {
    
}
*/

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Object {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: Option<bson::oid::ObjectId>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
