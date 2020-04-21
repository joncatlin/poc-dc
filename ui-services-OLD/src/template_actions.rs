use diesel::prelude::*;

//mod models;
use crate::models;



/// Get a list of templates
// pub fn find_templates (
//     conn: &PgConnection,
// ) -> Result<Vec<models::TemplateWithLanguage>, diesel::result::Error> {
//     // use crate::schema::templates::dsl::*;
//     // use crate::schema::languages::dsl::*;
//     use diesel::sql_query;

//     let results = sql_query("
//         SELECT templates.template_id, 
//             templates.template_name,
//             languages.language_id,
//             languages.language_name
//         FROM templates
//         INNER JOIN languages ON templates.language_id = languages.language_id
//     ")
//         .load::<models::TemplateWithLanguage>(conn)
//         .expect("Error loading templates");

//     // let results = templates
//     //     .inner_join(languages)
//     //     .load::<(models::Template, models::Language)>(conn)
//     //     .expect("Error loading templates");

//     println!("Results={:?}", results);
//     Ok(results)
// }


/// Find a single template
// pub fn find_template (
//     obj_id: i32,
//     conn: &PgConnection,
// ) -> Result<Vec<models::TemplateWithLanguage>, diesel::result::Error> {
//     use diesel::sql_query;
//     use diesel::sql_types::Integer;

//     let results = sql_query("
//         SELECT templates.template_id, 
//             templates.template_name,
//             languages.language_id,
//             languages.language_name
//         FROM templates
//         INNER JOIN languages ON templates.language_id = languages.language_id
//         WHERE templates.template_id = $1
//     ")
//         .bind::<Integer, _>(obj_id)
//         .get_results(conn)
//         .expect("Error loading template");

//     Ok(results)
// }


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


