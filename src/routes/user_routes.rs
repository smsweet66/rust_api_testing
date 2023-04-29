use actix_web::{HttpResponse, Responder, web, get, post, put, delete, HttpRequest};
use diesel::{r2d2::{Pool, ConnectionManager}, SqliteConnection};
use crate::{handlers::user_handlers::*, models::user_model::{UserNew, UserLogin}};

type DB = web::Data<Pool<ConnectionManager<SqliteConnection>>>;

#[get("/users/{id}")]
pub async fn get_user(pool: DB, id: web::Path<i32>, req: HttpRequest) -> impl Responder {
	let token = req.headers().get("Authorization").unwrap().to_str();
	if token.is_err() {
		return HttpResponse::Unauthorized().body("Invalid token");
	}

	let res = get_user_handler(pool, id.into_inner(), token.unwrap()).await;

	match res {
		Ok(user) => HttpResponse::Ok().json(user),
		Err(e) => HttpResponse::InternalServerError().body(e)
	}
}

#[delete("/users/{id}")]
pub async fn delete_user(pool: DB, id: web::Path<i32>) -> impl Responder {
	let res = delete_user_handler(pool, id.into_inner()).await;

	match res {
		Ok(_) => HttpResponse::Ok().body("User deleted"),
		Err(_) => HttpResponse::InternalServerError().body("Error trying to delete user")
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
		Err(_) => HttpResponse::InternalServerError().body("Error trying to create user")
	}
}

#[post("/login")]
pub async fn login(pool: DB, item: web::Json<UserLogin>) -> impl Responder {
	let res = login_handler(pool, item).await;

	match res {
		Ok(token) => HttpResponse::Ok().json(token),
		Err(_) => HttpResponse::InternalServerError().body("Error trying to login")
	}
}