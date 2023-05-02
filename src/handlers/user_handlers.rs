use actix_web::web;
use diesel::{prelude::*, r2d2::{Pool, ConnectionManager}};
use argon2::{self, Config};
use rand::Rng;

use crate::models::user_model::*;
use crate::schema::users;
use crate::authentication;

type DB = web::Data<Pool<ConnectionManager<PgConnection>>>;

pub enum UserError {
	NotFound,
	AlreadyExists,
	InvalidPassword,
	InvalidToken,
	DatabaseError
}

impl UserError
{
	pub fn message(&self) -> String {
		match self {
			UserError::NotFound => "User not found".to_owned(),
			UserError::AlreadyExists => "User already exists".to_owned(),
			UserError::InvalidPassword => "Invalid password".to_owned(),
			UserError::InvalidToken => "Invalid token".to_owned(),
			UserError::DatabaseError => "Database error".to_owned()
		}
	}
}

fn hash(password: &[u8]) -> String {
	let salt = rand::thread_rng().gen::<[u8; 32]>();
	let config = Config::default();
	argon2::hash_encoded(password, &salt, &config).unwrap()
}

fn verify(hash: &str, password: &[u8]) -> bool {
	argon2::verify_encoded(hash, password).unwrap()
}

pub async fn register_handler(pool: DB, item: web::Json<UserNew>) -> Result<String, UserError> {
	let mut conn = pool.get().unwrap();
	let new_user = UserNew {
		name: item.name.clone(),
		email: item.email.clone(),
		password: hash(&item.password.as_bytes()),
	};

	let result = diesel::insert_into(users::table)
		.values(&new_user)
		.get_result::<User>(&mut conn);

	match result {
		Ok(user) => {
			let token = authentication::create_token(user.id);
			Ok(token)
		},
		Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => Err(UserError::AlreadyExists),
		Err(_) => Err(UserError::DatabaseError)
	}
}

pub async fn login_handler(pool: DB, item: web::Json<UserLogin>) -> Result<String, UserError> {
	let mut conn = pool.get().unwrap();
	let user = users::table
		.filter(users::email.eq(&item.email))
		.first::<User>(&mut conn);

	if user.is_err() {
		return Err(UserError::NotFound);
	}

	let user = user.unwrap();

	if verify(&user.password, &item.password.as_bytes()) {
		let token = authentication::create_token(user.id);
		Ok(token)
	} else {
		Err(UserError::InvalidPassword)
	}
}

pub async fn get_user_handler(pool: DB, token: &str) -> Result<User, UserError> {
	let id = match authentication::validate_token(token) {
		Ok(id) => id,
		Err(_) => return Err(UserError::InvalidToken),
	};

	let mut conn = pool.get().unwrap();
	let res = users::table.find(id).first::<User>(&mut conn);

	match res {
		Ok(user) => Ok(user),
		Err(diesel::result::Error::NotFound) => Err(UserError::NotFound),
		Err(_) => Err(UserError::DatabaseError),
	}
}

pub async fn update_user_handler(pool: DB, item: web::Json<UserNew>, token: &str) -> Result<User, UserError> {
	let id = match authentication::validate_token(token) {
		Ok(id) => id,
		Err(_) => return Err(UserError::InvalidToken),
	};

	let mut conn = pool.get().unwrap();
	let res = diesel::update(users::table.find(id))
		.set((
			users::name.eq(&item.name),
			users::email.eq(&item.email),
			users::password.eq(hash(&item.password.as_bytes()))),
		)
		.get_result::<User>(&mut conn);

	match res {
		Ok(user) => Ok(user),
		Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => return Err(UserError::AlreadyExists),
		Err(_) => return Err(UserError::DatabaseError)
	}
}

pub async fn delete_user_handler(pool: DB, token: &str) -> Result<(), UserError> {
	let id = match authentication::validate_token(token) {
		Ok(id) => id,
		Err(_) => return Err(UserError::InvalidToken),
	};

	let mut conn = pool.get().unwrap();
	let res = diesel::delete(users::table.find(id))
		.execute(&mut conn);

	match res {
		Ok(_) => Ok(()),
		Err(diesel::result::Error::NotFound) => Err(UserError::NotFound),
		Err(_) => Err(UserError::DatabaseError),
	}
}