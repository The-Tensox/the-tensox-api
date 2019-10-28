#![allow(proc_macro_derive_resolution_fallback)]

pub mod handler;
pub mod repository;
use mongodb::bson;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Mesh {
    normals: [f32; 3],
    uvs: [f32; 2],
    vertices: [f32; 3]
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Shape {
    Box {
        x: f32,
        y: f32,
        z: f32
    },
    Capsule {
        height: f32,
        radius: f32
    },
    Array {
        meshes: Vec<Mesh>
    }
}

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
    pub kind: Option<String>,
    pub mesh: Option<Shape>,
    pub collision: Option<Shape>,
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
    pub kind: Option<String>,
    pub mesh: Option<Shape>,
    pub collision: Option<Shape>,
}

impl InsertableObject {
    fn from_object(objects: Object) -> InsertableObject {
        InsertableObject {
            position_x: Some(objects.position_x.unwrap_or(0.0)),
            position_y: Some(objects.position_y.unwrap_or(0.0)),
            position_z: Some(objects.position_z.unwrap_or(0.0)),
            rotation_x: Some(objects.rotation_x.unwrap_or(0.0)),
            rotation_y: Some(objects.rotation_y.unwrap_or(0.0)),
            rotation_z: Some(objects.rotation_z.unwrap_or(0.0)),
            scale_x: Some(objects.scale_x.unwrap_or(1.0)),
            scale_y: Some(objects.scale_y.unwrap_or(1.0)),
            scale_z: Some(objects.scale_z.unwrap_or(1.0)),
            mass: Some(objects.mass.unwrap_or(0.0)),
            velocity_x: Some(objects.velocity_x.unwrap_or(0.0)),
            velocity_y: Some(objects.velocity_y.unwrap_or(0.0)),
            velocity_z: Some(objects.velocity_z.unwrap_or(0.0)),
            kind: Some(objects.kind.unwrap_or(String::from("ground"))),
            mesh: Some(objects.mesh.unwrap_or(Shape::Box{ x: 1.0, y: 1.0, z: 1.0 })),
            collision: Some(objects.collision.unwrap_or(Shape::Box{ x: 1.0, y: 1.0, z: 1.0 })),
        }
    }
    
    fn assign_id(object: InsertableObject, new_id: bson::oid::ObjectId) -> Object {
        Object {
            id: Some(new_id),
            position_x: object.position_x,
            position_y: object.position_y,
            position_z: object.position_z,
            rotation_x: object.rotation_x,
            rotation_y: object.rotation_y,
            rotation_z: object.rotation_z,
            scale_x: object.scale_x,
            scale_y: object.scale_y,
            scale_z: object.scale_z,
            mass: object.mass,
            velocity_x: object.velocity_x,
            velocity_y: object.velocity_y,
            velocity_z: object.velocity_z,
            kind: object.kind,
            mesh: object.mesh,
            collision: object.collision,
        }
    }
}
