use serde::{Deserialize, Serialize};

use crate::schema::users;
use crate::schema::categories;


#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
#[table_name="categories"]
pub struct Category {
    pub id: i32,
    pub category_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="categories"]
pub struct NewCategory {
    pub category_name: String,
}











#[derive(Debug, Clone, Serialize, Queryable, Insertable)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
}
