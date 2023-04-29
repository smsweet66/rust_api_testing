use paseto::tokens;

///Get the secret key from the file
fn get_secret_key() -> String {
	let key = std::fs::read_to_string("secret.key").expect("Error reading secret key");
	key
}

///Generate a token for a user
pub fn generate_token(user_id: i32) -> String {
	let key = get_secret_key();
	let token = tokens::PasetoBuilder::new()
		.set_key(key.as_bytes())
		.set_issued_at(None)
		.set_expiration_time(chrono::Duration::hours(1))
		.set_claim("user_id", user_id)
		.build()
		.expect("Error generating token");
	token
}

///Validate a token and return the user_id
pub fn validate_token(token: &str) -> Result<i32, String> {
	let key = get_secret_key();
	tokens::validate_local_token(token, None, key.as_bytes(), &tokens::TimeBackend::Chrono)
		.map(|token| token.get_claim("user_id").unwrap())
		.map_err(|_| "Error validating token".to_string())
}