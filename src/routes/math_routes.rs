use actix_web::{HttpResponse, Responder, get, web};

#[get("/math")]
pub async fn math() -> impl Responder {
	HttpResponse::Ok().body("Math route")
}

#[get("/math/add/{a}/{b}")]
pub async fn add(path: web::Path<(i32, i32)>) -> impl Responder {
	let (a, b) = path.into_inner();
	HttpResponse::Ok().body(format!("{} + {} = {}", a, b, a + b))
}

#[get("/math/multiply/{a}/{b}")]
pub async fn multiply(path: web::Path<(i32, i32)>) -> impl Responder {
	let (a, b) = path.into_inner();
	HttpResponse::Ok().body(format!("{} * {} = {}", a, b, a * b))
}