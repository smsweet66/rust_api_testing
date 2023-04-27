use warp::Filter;
use warp::filters::BoxedFilter;

pub fn home() -> BoxedFilter<()> {
	warp::path::end()
		.and(warp::get())
		.boxed()
}