pub mod http_response;

use crypto::digest::Digest;
use crypto::sha1::Sha1;
use dotenv;
use jsonwebtoken::errors;
use jsonwebtoken::{self, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub fn generate_hash<'a>(id: &'a str) -> String {
	let mut hasher = Sha1::new();
	hasher.input_str(&id);
	hasher.result_str()
}

pub fn decode<'a, T>(token: &'a str) -> errors::Result<TokenData<T>>
where
	T: DeserializeOwned,
{
	let secret: String = dotenv::var("SECRET_KEY").expect("Unable to read SECRET_KEY from .env");

	jsonwebtoken::decode::<T>(
		&token,
		&DecodingKey::from_secret(secret.as_ref()),
		&Validation::new(Algorithm::HS256),
	)
}

pub fn encode<T: Serialize>(claims: T) -> errors::Result<String> {
	let secret: String = dotenv::var("SECRET_KEY").expect("Unable to read SECRET_KEY from .env");

	jsonwebtoken::encode(
		&Header::new(Algorithm::HS256),
		&claims,
		&EncodingKey::from_secret(secret.as_ref()),
	)
}
