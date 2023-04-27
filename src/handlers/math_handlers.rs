use warp;

pub async fn math() -> Result<impl warp::Reply, warp::Rejection> {
	Ok(warp::reply::html("<h1>Math</h1>
		<ul>
			<li><a href=\"/math/plus/1/2\">1 + 2</a></li>
			<li><a href=\"/math/times/1/2\">1 * 2</a></li>
		</ul>"))
}

pub async fn plus(a: i32, b: i32) -> Result<impl warp::Reply, warp::Rejection> {
	Ok(warp::reply::html(format!("{} + {} = {}", a, b, a + b)))
}

pub async fn times(a: i32, b: i32) -> Result<impl warp::Reply, warp::Rejection> {
	Ok(warp::reply::html(format!("{} * {} = {}", a, b, a * b)))
}