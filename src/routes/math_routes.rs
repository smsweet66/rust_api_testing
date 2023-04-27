use warp::Filter;
use warp::filters::BoxedFilter;

pub fn math() -> BoxedFilter<()> {
	warp::path("math")
		.and(warp::get())
		.and(warp::path::end())
		.boxed()
}

pub fn plus() -> BoxedFilter<(i32, i32)> {
	warp::path!("math" / "plus" / i32 / i32)
		.and(warp::get())
		.and(warp::path::end())
		.boxed()
}

pub fn times() -> BoxedFilter<(i32, i32)> {
	warp::path!("math" / "times" / i32 / i32)
		.and(warp::get())
		.and(warp::path::end())
		.boxed()
}