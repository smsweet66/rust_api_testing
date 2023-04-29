use actix_web::{HttpResponse, Responder, web, get, post, put, delete};
use diesel::{r2d2::{Pool, ConnectionManager}, SqliteConnection};
use crate::{data_access::*, models::user_model::UserNew};

type DB = web::Data<Pool<ConnectionManager<SqliteConnection>>>;

#[get("/users")]
pub async fn get_users(pool: DB) -> impl Responder {
	let res = get_users_handler(pool).await;

	match res {
		Ok(users) => HttpResponse::Ok().json(users),
		Err(_) => HttpResponse::InternalServerError().body("Error trying to get users")
	}
}

#[get("/users/{id}")]
pub async fn get_user(pool: DB, id: web::Path<i32>) -> impl Responder {
	let res = get_user_handler(pool, id.into_inner()).await;

	match res {
		Ok(user) => HttpResponse::Ok().json(user),
		Err(_) => HttpResponse::InternalServerError().body("Error trying to get user")
	}
}

#[delete("/users/{id}/delete")]
pub async fn delete_user(pool: DB, id: web::Path<i32>) -> impl Responder {
	let res = delete_user_handler(pool, id.into_inner()).await;

	match res {
		Ok(_) => HttpResponse::Ok().body("User deleted"),
		Err(_) => HttpResponse::InternalServerError().body("Error trying to delete user")
	}
}

#[put("/users/{id}/update")]
pub async fn update_user(pool: DB, id: web::Path<i32>, item: web::Json<UserNew>) -> impl Responder {
	let res = update_user_handler(pool, id.into_inner(), item).await;

	match res {
		Ok(user) => HttpResponse::Ok().json(user),
		Err(_) => HttpResponse::InternalServerError().body("Error trying to update user")
	}
}

#[post("/users/create")]
pub async fn create_user(pool: DB, item: web::Json<UserNew>) -> impl Responder {
	let res = create_user_handler(pool, item).await;

	match res {
		Ok(user) => HttpResponse::Ok().json(user),
		Err(_) => HttpResponse::InternalServerError().body("Error trying to create user")
	}
}