use actix_web::web;
use diesel::{prelude::*, r2d2::{Pool, ConnectionManager}};
use argon2::{self, Config};
use jsonwebtoken::{Header, Algorithm, EncodingKey, encode};
use rand::Rng;

use crate::models::user_model::*;
use crate::schema::users;

type DB = web::Data<Pool<ConnectionManager<SqliteConnection>>>;

fn hash(password: &[u8]) -> String {
	let salt = rand::thread_rng().gen::<[u8; 32]>();
	let config = Config::default();
	argon2::hash_encoded(password, &salt, &config).unwrap()
}

fn verify(hash: &str, password: &[u8]) -> bool {
	argon2::verify_encoded(hash, password).unwrap()
}

pub async fn register_handler(pool: DB, item: web::Json<UserNew>) -> Result<User, diesel::result::Error> {
	let mut conn = pool.get().unwrap();
	let new_user = UserNew {
		name: item.name.clone(),
		email: item.email.clone(),
		password: hash(&item.password.as_bytes()),
	};

	diesel::insert_into(users::table)
		.values(&new_user)
		.execute(&mut conn)?;

	return Ok(users::table.order(users::id.desc()).first(&mut conn)?);
}

pub async fn get_user_handler(pool: DB, id: i32) -> Result<User, diesel::result::Error> {
	let mut conn = pool.get().unwrap();
	let res = users::table.find(id).first::<User>(&mut conn)?;

	Ok(res)
}

pub async fn update_user_handler(pool: DB, id: i32, item: web::Json<UserNew>) -> Result<User, diesel::result::Error> {
	let mut conn = pool.get().unwrap();
	diesel::update(users::table.find(id))
		.set((
			users::name.eq(&item.name),
			users::email.eq(&item.email),
			users::password.eq(hash(&item.password.as_bytes()))),
		)
		.execute(&mut conn)?;


	Ok(users::table.find(id).first::<User>(&mut conn)?)
}

pub async fn delete_user_handler(pool: DB, id: i32) -> Result<usize, diesel::result::Error> {
	let mut conn = pool.get().unwrap();
	let res = diesel::delete(users::table.find(id))
		.execute(&mut conn)?;

	Ok(res)
}