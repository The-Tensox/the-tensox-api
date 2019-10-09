table! {
    weather (id) {
        id -> Int4,
        x -> Int4,
        y -> Int4,
        sun -> Int4,
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
    weather,
    weathers,
);
