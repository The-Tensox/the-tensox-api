#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::weathers;
pub mod handler;
pub mod repository;

#[derive(Queryable, AsChangeset, Deserialize, Serialize, Debug, Clone)]
pub struct Weather {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub sun: i32,
}

/*
impl Serialize for Weather {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 4 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Weather", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        state.serialize_field("sun", &self.sun)?;
        state.end()
    }
}*/