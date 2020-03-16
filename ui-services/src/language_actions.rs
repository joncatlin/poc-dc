use diesel::prelude::*;

//mod models;
use crate::models;


/// Find all languages
pub fn find_languages (
    conn: &PgConnection,
) -> Result<Vec<models::Language>, diesel::result::Error> {
    use crate::schema::languages::dsl::*;

    let results = languages
        .limit(100)
        .load::<models::Language>(conn)
        .expect("Error loading posts");

    Ok(results)
}


/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_languages(
    cats: &Vec<models::NewLanguage>,
    conn: &PgConnection,
) -> Result<Vec<models::Language>, diesel::result::Error> {
    use crate::schema::languages::dsl::*;

    let results = diesel::insert_into(languages)
        .values(cats)
        .get_results(conn)
        .expect("Error saving new post");

    Ok(results)
}


