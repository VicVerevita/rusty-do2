use chrono::NaiveDateTime;
use diesel::{Associations, Identifiable, Insertable, Queryable};

use super::schema::{tasks, users};

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(User, foreign_key = "user_id")]
#[table_name = "tasks"]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub finished: bool,
    pub created_at: NaiveDateTime,
    pub user_id: uuid::Uuid,
}

#[derive(Queryable, Identifiable)]
#[primary_key("user_id")]
#[table_name = "users"]
pub struct User {
    pub user_id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTask<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub finished: bool,
    pub user_id: uuid::Uuid,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
}
