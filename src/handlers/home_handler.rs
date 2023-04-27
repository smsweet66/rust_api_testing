use warp;

pub async fn home() -> Result<impl warp::Reply, warp::Rejection> {
	Ok(warp::reply::html("<h1>Hello, world!</h1>"))
}