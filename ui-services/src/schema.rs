table! {
    categories (id) {
        id -> Int4,
        category_name -> Varchar,
    }
}

table! {
    channels (id) {
        id -> Int4,
        channel_name -> Varchar,
    }
}

table! {
    corrs (id) {
        id -> Int4,
        corr_name -> Varchar,
    }
}

table! {
    languages (id) {
        id -> Int4,
        language_name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    categories,
    channels,
    corrs,
    languages,
    users,
);
