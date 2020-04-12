table! {
    account (message_id, channel) {
        message_id -> Varchar,
        channel -> Varchar,
        account_id -> Varchar,
    }
}

table! {
    categories (category_id) {
        category_id -> Int4,
        category_name -> Varchar,
    }
}

table! {
    category_mappings (category_mappings_id) {
        category_mappings_id -> Int4,
        category_id -> Int4,
        correspondence_id -> Int4,
        opt_out -> Varchar,
        retention_period -> Int4,
    }
}

table! {
    channel_configs (channel_config_id) {
        channel_config_id -> Int4,
        category_mappings_id -> Int4,
        channel_id -> Int4,
        permitted -> Varchar,
    }
}

table! {
    channels (channel_id) {
        channel_id -> Int4,
        channel_name -> Varchar,
    }
}

table! {
    client_pref_channel_configs (client_pref_channel_config_id) {
        client_pref_channel_config_id -> Int4,
        client_preferences_id -> Int4,
        channel_id -> Int4,
        permitted -> Varchar,
    }
}

table! {
    client_preferences (client_preferences_id) {
        client_preferences_id -> Int4,
        category_id -> Int4,
        correspondence_id -> Int4,
        opt_out -> Varchar,
        retention_period -> Int4,
        developer -> Nullable<Varchar>,
        project -> Nullable<Varchar>,
        lender -> Nullable<Varchar>,
    }
}

table! {
    corrs (correspondence_id) {
        correspondence_id -> Int4,
        correspondence_name -> Varchar,
    }
}

table! {
    dpl (dpl_id) {
        dpl_id -> Int4,
        d -> Nullable<Varchar>,
        p -> Nullable<Varchar>,
        l -> Nullable<Varchar>,
        msg -> Text,
    }
}

table! {
    event (message_id, channel, event_status) {
        message_id -> Varchar,
        channel -> Varchar,
        event_status -> Varchar,
        event_timestamp -> Varchar,
        event_specific_data -> Nullable<Text>,
    }
}

table! {
    languages (language_id) {
        language_id -> Int4,
        language_name -> Varchar,
    }
}

table! {
    templates (template_id) {
        template_id -> Int4,
        template_name -> Varchar,
        language_id -> Int4,
    }
}

joinable!(category_mappings -> categories (category_id));
joinable!(category_mappings -> corrs (correspondence_id));
joinable!(channel_configs -> category_mappings (category_mappings_id));
joinable!(channel_configs -> channels (channel_id));
joinable!(client_pref_channel_configs -> channels (channel_id));
joinable!(client_pref_channel_configs -> client_preferences (client_preferences_id));
joinable!(client_preferences -> categories (category_id));
joinable!(client_preferences -> corrs (correspondence_id));
joinable!(templates -> languages (language_id));

allow_tables_to_appear_in_same_query!(
    account,
    categories,
    category_mappings,
    channel_configs,
    channels,
    client_pref_channel_configs,
    client_preferences,
    corrs,
    dpl,
    event,
    languages,
    templates,
);
