table! {
    categories (id) {
        id -> Int4,
        category_name -> Varchar,
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
    users,
);
