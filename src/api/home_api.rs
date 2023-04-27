#[macro_export]
macro_rules! home {
	() => {
		home_route::home()
			.and_then(home_handler::home)
	}
}