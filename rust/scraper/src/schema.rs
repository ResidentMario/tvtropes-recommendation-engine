table! {
    media (id) {
        id -> Nullable<Integer>,
        name -> Text,
        media_type_id -> Nullable<Integer>,
    }
}

table! {
    media_types (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

table! {
    trope_entries (id) {
        id -> Nullable<Integer>,
        media_id -> Nullable<Integer>,
        trope_id -> Nullable<Integer>,
    }
}

table! {
    tropes (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

joinable!(media -> media_types (media_type_id));
joinable!(trope_entries -> media (media_id));

allow_tables_to_appear_in_same_query!(
    media,
    media_types,
    trope_entries,
    tropes,
);
