use actix_web::App;
use actix_web::HttpServer;
use actix_web::web::Data;
use actix_web::web::JsonConfig;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::SqliteConnection;

mod models;
mod routes;
mod schema;
mod handlers;
mod authentication;

use self::routes::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let database_url = "userbase.db";
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
            .service(user_routes::create_user)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
