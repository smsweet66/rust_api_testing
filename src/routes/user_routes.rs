use actix_web::{HttpResponse, Responder, web, get, post, put, delete, HttpRequest};
use diesel::{r2d2::{Pool, ConnectionManager}, SqliteConnection};
use crate::{handlers::user_handlers::*, models::user_model::{UserNew, UserLogin}};

type DB = web::Data<Pool<ConnectionManager<SqliteConnection>>>;

#[get("/users")]
pub async fn get_user(pool: DB, req: HttpRequest) -> impl Responder {
	let token = req.headers().get("Authorization");
	if token.is_none() {
		return HttpResponse::Unauthorized().body("Missing token");
	}

	let token = token.unwrap().to_str();

	let res = get_user_handler(pool, token.unwrap()).await;

	match res {
		Ok(user) => HttpResponse::Ok().json(user),
		Err(e) => match e {
			UserError::NotFound => HttpResponse::NotFound().body(e.message()),
			UserError::InvalidToken => HttpResponse::Unauthorized().body(e.message()),
			_ => HttpResponse::InternalServerError().body(e.message())
		}
	}
}

#[delete("/users")]
pub async fn delete_user(pool: DB, req: HttpRequest) -> impl Responder {
	let token = req.headers().get("Authorization");
	if token.is_none() {
		return HttpResponse::Unauthorized().body("Missing token");
	}

	let token = token.unwrap().to_str();

	let res = delete_user_handler(pool, token.unwrap()).await;

	match res {
		Ok(_) => HttpResponse::Ok().body("User deleted"),
		Err(e) => match e {
			UserError::NotFound => HttpResponse::NotFound().body(e.message()),
			UserError::InvalidToken => HttpResponse::Unauthorized().body(e.message()),
			_ => HttpResponse::InternalServerError().body(e.message())
		}
	}
}

#[put("/users/{id}")]
pub async fn update_user(pool: DB, id: web::Path<i32>, item: web::Json<UserNew>) -> impl Responder {
	let res = update_user_handler(pool, id.into_inner(), item).await;

	match res {
		Ok(user) => HttpResponse::Ok().json(user),
		Err(_) => HttpResponse::InternalServerError().body("Error trying to update user")
	}
}

#[post("/register")]
pub async fn register(pool: DB, item: web::Json<UserNew>) -> impl Responder {
	let res = register_handler(pool, item).await;

	match res {
		Ok(token) => HttpResponse::Ok().json(token),
		Err(e) => match e {
			UserError::AlreadyExists => HttpResponse::Conflict().body(e.message()),
			_ => HttpResponse::InternalServerError().body(e.message())
		}
	}
}

#[post("/login")]
pub async fn login(pool: DB, item: web::Json<UserLogin>) -> impl Responder {
	let res = login_handler(pool, item).await;

	match res {
		Ok(token) => HttpResponse::Ok().json(token),
		Err(e) => match e {
			UserError::NotFound => HttpResponse::NotFound().body(e.message()),
			UserError::InvalidPassword => HttpResponse::Unauthorized().body(e.message()),
			_ => HttpResponse::InternalServerError().body(e.message())
		}
	}
}