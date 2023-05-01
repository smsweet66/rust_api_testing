use actix_web::{HttpRequest, get, post, Responder, HttpResponse, web};
use diesel::{r2d2::{Pool, ConnectionManager}, SqliteConnection};
use crate::{handlers::profile_handlers::*, models::profile_model::ProfileUpdate};

type DB = web::Data<Pool<ConnectionManager<SqliteConnection>>>;

#[get("/profiles")]
pub async fn get_profiles(pool: DB, req: HttpRequest) -> impl Responder {
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

#[post("/profiles")]
pub async fn create_profile(pool: DB, item: web::Json<ProfileUpdate>, req: HttpRequest) -> impl Responder {
    let token = req.headers().get("Authorization");
    if token.is_none() {
        return HttpResponse::Unauthorized().body("Missing token");
    }

    let token = token.unwrap().to_str();
    let profile = create_profile_handler(pool, item, token.unwrap()).await;

    match profile {
        Ok(profile) => HttpResponse::Ok().json(profile),
        Err(ProfileError::InvalidToken) => HttpResponse::Unauthorized().body(ProfileError::InvalidToken.message()),
        Err(ProfileError::DatabaseError) => HttpResponse::InternalServerError().body(ProfileError::DatabaseError.message()),
        Err(ProfileError::AlreadyExists) => HttpResponse::Conflict().body(ProfileError::AlreadyExists.message())
    }
}