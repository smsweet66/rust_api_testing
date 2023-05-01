use actix_web::{HttpResponse, Responder, web, get, post, put, delete, HttpRequest};
use diesel::{r2d2::{Pool, ConnectionManager}, SqliteConnection};
use crate::{handlers::user_handlers::*, models::user_model::{UserNew, UserLogin}};

type DB = web::Data<Pool<ConnectionManager<SqliteConnection>>>;

#[get("")]
async fn get_user(pool: DB, req: HttpRequest) -> impl Responder {
	let token = req.headers().get("Authorization");
	if token.is_none() {
		return HttpResponse::Unauthorized().body("Missing token");
	}

	let token = token.unwrap().to_str();

	let res = get_user_handler(pool, token.unwrap()).await;

	match res {
		Ok(user) => HttpResponse::Ok().json(user),
		Err(UserError::NotFound) => HttpResponse::NotFound().body(UserError::NotFound.message()),
		Err(UserError::InvalidToken) => HttpResponse::Unauthorized().body(UserError::InvalidToken.message()),
		Err(e) => HttpResponse::InternalServerError().body(e.message())
	}
}

#[delete("")]
async fn delete_user(pool: DB, req: HttpRequest) -> impl Responder {
	let token = req.headers().get("Authorization");
	if token.is_none() {
		return HttpResponse::Unauthorized().body("Missing token");
	}

	let token = token.unwrap().to_str();

	let res = delete_user_handler(pool, token.unwrap()).await;

	match res {
		Ok(_) => HttpResponse::Ok().body("User deleted"),
		Err(UserError::NotFound) => HttpResponse::NotFound().body(UserError::NotFound.message()),
		Err(UserError::InvalidToken) => HttpResponse::Unauthorized().body(UserError::InvalidToken.message()),
		Err(e) => HttpResponse::InternalServerError().body(e.message())
	}
}

#[put("")]
async fn update_user(pool: DB, item: web::Json<UserNew>, req: HttpRequest) -> impl Responder {
	let token = req.headers().get("Authorization");
	if token.is_none() {
		return HttpResponse::Unauthorized().body("Missing token");
	}

	let token = token.unwrap().to_str();

	let res = update_user_handler(pool, item, token.unwrap()).await;

	match res {
		Ok(user) => HttpResponse::Ok().json(user),
		Err(UserError::NotFound) => HttpResponse::NotFound().body(UserError::NotFound.message()),
		Err(UserError::InvalidToken) => HttpResponse::Unauthorized().body(UserError::InvalidToken.message()),
		Err(UserError::AlreadyExists) => HttpResponse::Conflict().body(UserError::AlreadyExists.message()),
		Err(e) => HttpResponse::InternalServerError().body(e.message())
	}
}

#[post("/register")]
async fn register(pool: DB, item: web::Json<UserNew>) -> impl Responder {
	let res = register_handler(pool, item).await;

	match res {
		Ok(token) => HttpResponse::Ok().json(token),
		Err(UserError::AlreadyExists) => HttpResponse::Conflict().body(UserError::AlreadyExists.message()),
		Err(e) => HttpResponse::InternalServerError().body(e.message())
	}
}

#[post("/login")]
async fn login(pool: DB, item: web::Json<UserLogin>) -> impl Responder {
	let res = login_handler(pool, item).await;

	match res {
		Ok(token) => HttpResponse::Ok().json(token),
		Err(UserError::NotFound) => HttpResponse::NotFound().body(UserError::NotFound.message()),
		Err(UserError::InvalidPassword) => HttpResponse::Unauthorized().body(UserError::InvalidPassword.message()),
		Err(e) => HttpResponse::InternalServerError().body(e.message())
	}
}

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(get_user);
	cfg.service(delete_user);
	cfg.service(update_user);
	cfg.service(register);
	cfg.service(login);
}