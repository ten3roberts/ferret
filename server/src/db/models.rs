use std::borrow::Cow;

use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Identifiable, Clone, PartialEq, Serialize, Deserialize, Queryable, Insertable)]
#[primary_key(user_id)]
#[table_name = "users"]
pub struct User<'a> {
    pub user_id: String,
    pub username: Cow<'a, str>,
}

#[derive(
    Identifiable, Associations, Debug, Clone, PartialEq, Serialize, Deserialize, Queryable,
)]
#[belongs_to(parent = "User<'_>")]
pub struct Post {
    pub id: i32,
    pub user_id: String,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub user_id: &'a str,
    pub title: &'a str,
    pub body: &'a str,
}
