use actix_web::{HttpRequest, get, post, put, delete, Responder, HttpResponse, web};
use diesel::{r2d2::{Pool, ConnectionManager}, PgConnection};
use crate::{handlers::profile_handlers::*, models::profile_model::ProfileUpdate};

type DB = web::Data<Pool<ConnectionManager<PgConnection>>>;

#[get("")]
async fn get_profiles(pool: DB, req: HttpRequest) -> impl Responder {
    let token = req.headers().get("Authorization");
    if token.is_none() {
        return HttpResponse::Unauthorized().body("Missing token");
    }

    let token = token.unwrap().to_str();
    let profiles = get_profiles_handler(pool, token.unwrap()).await;

    match profiles {
        Ok(profiles) => HttpResponse::Ok().json(profiles),
        Err(ProfileError::InvalidToken) => HttpResponse::Unauthorized().body(ProfileError::InvalidToken.message()),
        Err(e) => HttpResponse::InternalServerError().body(e.message())
    }
}

#[post("")]
async fn create_profile(pool: DB, item: web::Json<ProfileUpdate>, req: HttpRequest) -> impl Responder {
    let token = req.headers().get("Authorization");
    if token.is_none() {
        return HttpResponse::Unauthorized().body("Missing token");
    }

    let token = token.unwrap().to_str();
    let result = create_profile_handler(pool, item, token.unwrap()).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Profile created successfully"),
        Err(ProfileError::InvalidToken) => HttpResponse::Unauthorized().body(ProfileError::InvalidToken.message()),
        Err(ProfileError::AlreadyExists) => HttpResponse::Conflict().body(ProfileError::AlreadyExists.message()),
        Err(e) => HttpResponse::InternalServerError().body(e.message())
    }
}

#[put("/{id}")]
async fn update_profile(pool: DB, path: web::Path<i32>, item: web::Json<ProfileUpdate>, req: HttpRequest) -> impl Responder {
    let token = req.headers().get("Authorization");
    if token.is_none() {
        return HttpResponse::Unauthorized().body("Missing token");
    }

    let token = token.unwrap().to_str();
    let result = update_profile_handler(pool, item, path.into_inner(), token.unwrap()).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Profile updated successfully"),
        Err(ProfileError::InvalidToken) => HttpResponse::Unauthorized().body(ProfileError::InvalidToken.message()),
        Err(ProfileError::NotFound) => HttpResponse::NotFound().body(ProfileError::NotFound.message()),
        Err(e) => HttpResponse::InternalServerError().body(e.message())
    }
}

#[delete("/{id}")]
async fn delete_profile(pool: DB, path: web::Path<i32>, req: HttpRequest) -> impl Responder {
    let token = req.headers().get("Authorization");
    if token.is_none() {
        return HttpResponse::Unauthorized().body("Missing token");
    }

    let token = token.unwrap().to_str();
    let profile = delete_profile_handler(pool, path.into_inner(), token.unwrap()).await;

    match profile {
        Ok(_) => HttpResponse::Ok().body("Profile deleted"),
        Err(ProfileError::InvalidToken) => HttpResponse::Unauthorized().body(ProfileError::InvalidToken.message()),
        Err(ProfileError::NotFound) => HttpResponse::NotFound().body(ProfileError::NotFound.message()),
        Err(e) => HttpResponse::InternalServerError().body(e.message())
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_profiles);
    cfg.service(create_profile);
    cfg.service(update_profile);
    cfg.service(delete_profile);
}