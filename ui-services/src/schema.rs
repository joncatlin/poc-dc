table! {
    categories (category_id) {
        category_id -> Int4,
        category_name -> Varchar,
    }
}

table! {
    category_mappings (category_id, correspondence_id) {
        category_mappings_id -> Int4,
        category_id -> Int4,
        correspondence_id -> Int4,
        opt_out -> Bool,
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
joinable!(templates -> languages (language_id));

allow_tables_to_appear_in_same_query!(
    categories,
    category_mappings,
    channels,
    corrs,
    languages,
    templates,
);
