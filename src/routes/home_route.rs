use actix_web::{HttpResponse, get, Responder};

#[get("/")]
pub async fn root() -> impl Responder {
	HttpResponse::Ok().body("Hello world!")
}