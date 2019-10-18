table! {
    objects (id) {
        id -> Int4,
        position_x -> Nullable<Float4>,
        position_y -> Nullable<Float4>,
        position_z -> Nullable<Float4>,
        rotation_x -> Nullable<Float4>,
        rotation_y -> Nullable<Float4>,
        rotation_z -> Nullable<Float4>,
        scale_x -> Nullable<Float4>,
        scale_y -> Nullable<Float4>,
        scale_z -> Nullable<Float4>,
        mass -> Nullable<Float4>,
        velocity_x -> Nullable<Float4>,
        velocity_y -> Nullable<Float4>,
        velocity_z -> Nullable<Float4>,
        collision_x -> Nullable<Float4>,
        collision_y -> Nullable<Float4>,
        collision_z -> Nullable<Float4>,
        height -> Nullable<Float4>,
        radius -> Nullable<Float4>,
        kind -> Nullable<Text>,
    }
}

table! {
    weathers (id) {
        id -> Int4,
        x -> Int4,
        y -> Int4,
        sun -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    objects,
    weathers,
);
