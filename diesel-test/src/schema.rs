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
