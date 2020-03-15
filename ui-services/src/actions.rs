use diesel::prelude::*;
use uuid::Uuid;

//mod models;
use crate::models;
//use crate::models:: {Category, User};


/// Find all categories
pub fn find_categories (
    conn: &PgConnection,
//) -> Result<Option<models::Category>, diesel::result::Error> {
) -> Result<Vec<models::Category>, diesel::result::Error> {
    use crate::schema::categories::dsl::*;

    let results = categories
        .limit(100)
        .load::<models::Category>(conn)
        .expect("Error loading posts");

    Ok(results)
}


/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_categories(
    // prevent collision with `name` column imported inside the function
    cats: &Vec<models::NewCategory>,
    conn: &PgConnection,
) -> Result<Vec<models::Category>, diesel::result::Error> {
    use crate::schema::categories::dsl::*;
//    use schema::categories;

    // let mut new_categories = Vec::<models::NewCategory>::new();
    // for cat_name in cat_names {
    //     new_categories.push(models::NewCategory {category_name: cat_name.to_string()});
    // }
    

    let results = diesel::insert_into(categories)
        .values(cats)
        .get_results(conn)
        .expect("Error saving new post");

    
    // models::User {
    //     id: Uuid::new_v4().to_string(),
    //     name: nm.to_owned(),
    // };

    // diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(results)
}




















/// Run query using Diesel to insert a new database row and return the result.
pub fn find_user_by_uid(
    uid: Uuid,
    conn: &PgConnection,
) -> Result<Option<models::User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(uid.to_string()))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_user(
    // prevent collision with `name` column imported inside the function
    nm: &str,
    conn: &PgConnection,
) -> Result<models::User, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::users::dsl::*;

    let new_user = models::User {
        id: Uuid::new_v4().to_string(),
        name: nm.to_owned(),
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}
