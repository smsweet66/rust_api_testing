use serde::{Serialize, Deserialize};
use crate::schema::users;
use diesel::prelude::*;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct User {
	pub uid: i32,
	pub name: String,
	pub email: String,
	pub password: String,
	pub created_at: chrono::NaiveDateTime,
	pub updated_at: chrono::NaiveDateTime
}

#[derive(Debug, Clone, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct UserNew {
	pub name: String,
	pub email: String,
	pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLogin {
	pub email: String,
	pub password: String,
}