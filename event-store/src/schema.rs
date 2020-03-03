table! {
    account (message_id, channel) {
        message_id -> Varchar,
        channel -> Varchar,
        account_id -> Varchar,
    }
}

table! {
    event (message_id, channel) {
        message_id -> Varchar,
        channel -> Varchar,
        event_status -> Varchar,
        event_timestamp -> Varchar,
        event_specific_data -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    account,
    event,
);
