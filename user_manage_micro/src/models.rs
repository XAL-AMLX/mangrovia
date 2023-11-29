// models.rs

use diesel::prelude::*;

#[derive(Queryable, Insertable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub full_name: String,
    pub email: String,
    pub phone_number: i64,
    pub address: String,
}