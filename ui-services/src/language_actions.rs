use diesel::prelude::*;

//mod models;
use crate::models;


/// Find all languages
pub fn find_languages (
    conn: &PgConnection,
) -> Result<Vec<models::Language>, diesel::result::Error> {
    use crate::schema::languages::dsl::*;

    let results = languages
        .order(language_name.asc())
        .load::<models::Language>(conn)
        .expect("Error loading languages");

    Ok(results)
}


/// Run query using Diesel to insert a new database row and return the result.
pub fn upsert_new_languages(
    upsert_list: &Vec<models::Language>,
    conn: &PgConnection,
) -> Result<Vec<models::Language>, diesel::result::Error> {
    use crate::schema::languages::dsl::*;

    let mut inserts = Vec::new();

    for item in upsert_list {
        if item.language_id == -1 {
            inserts.push(models::NewLanguage{language_name: item.language_name.clone()});
        } else {
            // Update the existing languages
            info!("Updating language with values: {:?}", item);
            match diesel::update(languages.filter(language_id.eq(item.language_id)))
                .set(language_name.eq(item.language_name.clone()))
                .execute(conn)
            {
                Ok(results) => debug!("Successful update into languages. Result: {:?}", results),
                Err(e) => error!("Error updating languages, error: {:?}", e),
            }
        }
    }

    // Insert the new languages
    match diesel::insert_into(languages)
        .values(inserts)
        .execute(conn)
    {
        Ok(results) => debug!("Successful insert into languages. Result: {:?}", results),
        Err(e) => error!("Error inserting languages, error: {:?}", e),
    }

    // Send back a complete list of the items in the table
    let results = languages
        .limit(1000)
        .load::<models::Language>(conn)
        .expect("Error obtaining list of languages");

    Ok(results)
}


/// Run query using Diesel to delete languages given their id's
pub fn delete_existing_languages(
    delete_list: &Vec<models::Language>,
    conn: &PgConnection,
) -> Result<Vec<models::Language>, diesel::result::Error> {
    use crate::schema::languages::dsl::*;

    for item in delete_list {
        // Delete the existing languages
        info!("Deleteing language with values: {:?}", item);
        match diesel::delete(languages.filter(language_id.eq(item.language_id)))
            .execute(conn)
        {
            Ok(results) => debug!("Successful delete from languages. Result: {:?}", results),
            Err(e) => error!("Error deleting languages, error: {:?}", e),
        }
    }

    // Send back a complete list of the items left in the table after the delete
    let results = languages
        .limit(1000)
        .load::<models::Language>(conn)
        .expect("Error obtaining list of languages");

    Ok(results)
}


