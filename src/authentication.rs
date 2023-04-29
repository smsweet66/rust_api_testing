use chrono::Utc;
use paseto::tokens;
use serde_json::json;

///Get the secret key from the file
fn get_secret_key() -> String {
	let key = std::fs::read_to_string("secret.key").expect("Error reading secret key");
	key
}

///Create a token for a user
pub fn create_token(user_id: i32) -> String {
	let key = get_secret_key();
	let dt = Utc::now() + chrono::Duration::hours(1);

	let token = tokens::PasetoBuilder::new()
		.set_encryption_key(key.as_bytes())
		.set_issued_at(None)
		.set_expiration(&dt)
		.set_claim("user_id", json!(user_id))
		.build()
		.expect("Error creating token");

	token
}

///Verify a token
pub fn validate_token(token: &str) -> Result<i32, String> {
	let key = get_secret_key();
	let verified_token = tokens::validate_local_token(
		token,
		None,
		key.as_bytes(),
		&tokens::TimeBackend::Chrono);

	match verified_token {
		Ok(token) => {
			let user_id = token["user_id"].as_i64();
			match user_id {
				Some(id) => Ok(id as i32),
				None => Err(String::from("Error validating token"))
			}
		},
		Err(_) => Err(String::from("Error validating token"))
	}
}