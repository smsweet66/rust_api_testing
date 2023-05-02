use actix_web::{App, HttpServer, web::{self, Data, JsonConfig}};
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use dotenv::dotenv;

mod models;
mod routes;
mod schema;
mod handlers;
mod authentication;

use self::routes::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Pool::builder()
        .build(ConnectionManager::<PgConnection>::new(database_url))
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(JsonConfig::default())
            .service(home_route::root)
            .service(web::scope("/math").configure(math_routes::config))
            .service(web::scope("/users").configure(user_routes::config))
            .service(web::scope("/profiles").configure(profile_routes::config))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
