#[macro_export]
macro_rules! math {
	() => {
		math_routes::math()
			.and_then(math_handlers::math)
	}
}

#[macro_export]
macro_rules! plus {
	() => {
		math_routes::plus()
			.and_then(math_handlers::plus)
	}
}

#[macro_export]
macro_rules! times {
	() => {
		math_routes::times()
			.and_then(math_handlers::times)
	}
}