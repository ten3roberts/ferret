use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub username: String,
    pub title: String,
    pub body: String,
}

use crate::schema::posts;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub username: &'a str,
    pub title: &'a str,
    pub body: &'a str,
}
