use warp::{self, Filter};

mod routes;
mod handlers;
mod api;

use self::routes::*;
use self::handlers::*;

#[tokio::main]
async fn main() {
    let routes = home!().with(warp::log("home_api"));
    //create a server listening on localhost:3030 using a separate task
    tokio::spawn(warp::serve(routes).run(([127, 0, 0, 1], 3030)));

    //add shutdown for ctrl+c
    match tokio::signal::ctrl_c().await {
        Ok(_) => println!("Shutting down"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
