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
        opt_out -> Int4,
        retention_period -> Int4,
    }
}

table! {
    channel_configs (channel_config_id) {
        channel_config_id -> Int4,
        category_mappings_id -> Int4,
        channel_id -> Int4,
        permitted -> Int4,
    }
}

table! {
    channels (channel_id) {
        channel_id -> Int4,
        channel_name -> Varchar,
    }
}

table! {
    corrs (correspondence_id) {
        correspondence_id -> Int4,
        correspondence_name -> Varchar,
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
joinable!(templates -> languages (language_id));

allow_tables_to_appear_in_same_query!(
    categories,
    category_mappings,
    channel_configs,
    channels,
    corrs,
    languages,
    templates,
);
