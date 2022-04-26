use std::primitive;

use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Identifiable, Clone, PartialEq, Serialize, Deserialize, Queryable, Insertable)]
#[primary_key(user_id)]
#[table_name = "users"]
pub struct User {
    pub user_id: String,
    pub picture: Option<String>,
    pub name: String,
}

#[derive(
    Identifiable, Associations, Debug, Clone, PartialEq, Deserialize, Queryable, Serialize,
)]
#[belongs_to(User)]
#[primary_key(post_id)]
pub struct Post {
    pub post_id: i32,
    pub user_id: String,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, PartialEq)]
pub struct UserPost {
    user: User,
    post: Post,
}

impl UserPost {
    pub fn new(user: User, post: Post) -> Self {
        Self { user, post }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(
    Identifiable, Associations, Debug, Clone, PartialEq, Deserialize, Serialize, Queryable,
)]
#[primary_key(comment_id)]
pub struct Comment {
    pub comment_id: i32,
    pub post_id: i32,
    pub user_id: String,
    pub body: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Insertable)]
#[table_name = "comments"]
pub struct NewComment<'a> {
    pub post_id: i32,
    pub body: &'a str,
}
