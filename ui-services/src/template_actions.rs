use diesel::prelude::*;

//mod models;
use crate::models;



/// Get a list of templates
pub fn find_templates (
    conn: &PgConnection,
) -> Result<Vec<models::TemplateWithLanguage>, diesel::result::Error> {
    // use crate::schema::templates::dsl::*;
    // use crate::schema::languages::dsl::*;
    use diesel::sql_query;

    let results = sql_query("
        SELECT templates.template_id, 
            templates.template_name,
            languages.language_id,
            languages.language_name
        FROM templates
        INNER JOIN languages ON templates.language_id = languages.language_id
    ")
        .load::<models::TemplateWithLanguage>(conn)
        .expect("Error loading templates");

    // let results = templates
    //     .inner_join(languages)
    //     .load::<(models::Template, models::Language)>(conn)
    //     .expect("Error loading templates");

    println!("Results={:?}", results);
    Ok(results)
}


/// Find a single template
pub fn find_template (
    obj_id: i32,
    conn: &PgConnection,
) -> Result<Vec<models::TemplateWithLanguage>, diesel::result::Error> {
//    use crate::schema::templates::dsl::*;
    use diesel::sql_query;
    use diesel::sql_types::Integer;

//     let users = sql_query("SELECT * FROM users WHERE id > ? AND name <> ?")
//     .bind::<Integer, _>(1)
//     .bind::<Text, _>("Tess")
//     .get_results(&connection);
// let expected_users = vec![
//     User { id: 3, name: "Jim".into() },
// ];
// assert_eq!(Ok(expected_users), users)

    let results = sql_query("
        SELECT templates.template_id, 
            templates.template_name,
            languages.language_id,
            languages.language_name
        FROM templates
        INNER JOIN languages ON templates.language_id = languages.language_id
        WHERE templates.template_id = $1
    ")
        .bind::<Integer, _>(obj_id)
//        .filter(templates.template_id.eq(obj_id))
//        .load::<models::TemplateWithLanguage>(conn)
        .get_results(conn)
        .expect("Error loading template");

    // let results = templates
    //     .filter(template_id.eq(obj_id))
    //     .load::<models::Template>(conn)
    //     .expect("Error loading templates");
    Ok(results)
}


/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_templates(
    cats: &Vec<models::NewTemplate>,
    conn: &PgConnection,
) -> Result<Vec<models::Template>, diesel::result::Error> {
    use crate::schema::templates::dsl::*;

    let results = diesel::insert_into(templates)
        .values(cats)
        .get_results(conn)
        .expect("Error saving new templates");

    Ok(results)
}


