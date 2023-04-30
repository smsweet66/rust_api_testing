use actix_web::{App, HttpServer, web::Data, web::JsonConfig};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
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
        .build(ConnectionManager::<SqliteConnection>::new(database_url))
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(JsonConfig::default())
            .service(home_route::root)
            .service(math_routes::math)
            .service(math_routes::add)
            .service(math_routes::multiply)
            .service(user_routes::get_user)
            .service(user_routes::delete_user)
            .service(user_routes::update_user)
            .service(user_routes::register)
            .service(user_routes::login)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
