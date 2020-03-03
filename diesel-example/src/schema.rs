table! {
    event_trans (id) {
        id -> Int4,
        event_id -> Varchar,
        event_type -> Varchar,
        event_status -> Varchar,
        event_datetime -> Timestamp,
        event_specific_data -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    event_trans,
    users,
);
