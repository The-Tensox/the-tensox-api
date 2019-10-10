table! {
    objects (id) {
        id -> Int4,
        x -> Int4,
        y -> Int4,
        z -> Int4,
        scale_x -> Nullable<Float4>,
        scale_y -> Nullable<Float4>,
        scale_z -> Nullable<Float4>,
        height -> Nullable<Float4>,
        radius -> Nullable<Float4>,
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
