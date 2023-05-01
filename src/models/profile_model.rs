use diesel::{Queryable, Insertable, Associations};
use serde::{Deserialize, Serialize};
use crate::models::user_model::User;
use crate::schema::profiles;


#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
	pub id: i32,
	pub user_id: i32,
	pub name: String,
	pub body_sizes: String,
	pub created_at: chrono::NaiveDateTime,
	pub updated_at: chrono::NaiveDateTime
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize, Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = profiles)]
pub struct ProfileNew {
	pub user_id: i32,
	pub name: String,
	pub body_sizes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileUpdate {
	pub name: String,
	pub body_sizes: String,
}