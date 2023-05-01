use actix_web::{HttpResponse, Responder, get, web};

#[get("")]
async fn math() -> impl Responder {
	HttpResponse::Ok().body("Math route")
}

#[get("/add/{a}/{b}")]
async fn add(path: web::Path<(i32, i32)>) -> impl Responder {
	let (a, b) = path.into_inner();
	HttpResponse::Ok().body(format!("{} + {} = {}", a, b, a + b))
}

#[get("/multiply/{a}/{b}")]
async fn multiply(path: web::Path<(i32, i32)>) -> impl Responder {
	let (a, b) = path.into_inner();
	HttpResponse::Ok().body(format!("{} * {} = {}", a, b, a * b))
}

pub fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(math)
		.service(add)
		.service(multiply);
}