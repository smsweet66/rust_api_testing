use actix_web::web;
use diesel::{prelude::*, r2d2::{Pool, ConnectionManager}};
use argon2::{self, Config};
use rand::Rng;

use crate::models::user_model::*;
use crate::schema::users;
use crate::authentication;

type DB = web::Data<Pool<ConnectionManager<SqliteConnection>>>;

fn hash(password: &[u8]) -> String {
	let salt = rand::thread_rng().gen::<[u8; 32]>();
	let config = Config::default();
	argon2::hash_encoded(password, &salt, &config).unwrap()
}

fn verify(hash: &str, password: &[u8]) -> bool {
	argon2::verify_encoded(hash, password).unwrap()
}

pub async fn register_handler(pool: DB, item: web::Json<UserNew>) -> Result<String, diesel::result::Error> {
	let mut conn = pool.get().unwrap();
	let new_user = UserNew {
		name: item.name.clone(),
		email: item.email.clone(),
		password: hash(&item.password.as_bytes()),
	};

	diesel::insert_into(users::table)
		.values(&new_user)
		.execute(&mut conn)?;

	let user = users::table
		.filter(users::email.eq(&item.email))
		.first::<User>(&mut conn)?;

	let token = authentication::create_token(user.id);

	Ok(token)
}

pub async fn login_handler(pool: DB, item: web::Json<UserLogin>) -> Result<String, diesel::result::Error> {
	let mut conn = pool.get().unwrap();
	let user = users::table
		.filter(users::email.eq(&item.email))
		.first::<User>(&mut conn)?;

	if verify(&user.password, &item.password.as_bytes()) {
		let token = authentication::create_token(user.id);
		Ok(token)
	} else {
		Err(diesel::result::Error::NotFound)
	}
}

pub async fn get_user_handler(pool: DB, id: i32, token: &str) -> Result<User, String> {
	let user_id = match authentication::validate_token(token) {
		Ok(id) => id,
		Err(_) => return Err("Invalid token".to_string()),
	};

	if user_id != id { return Err("Forbidden".to_string()) }

	let mut conn = pool.get().unwrap();
	let res = users::table.find(id).first::<User>(&mut conn);
	match res {
		Ok(user) => Ok(user),
		Err(_) => Err("User not found".to_string()),
	}
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