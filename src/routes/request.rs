use std::string::String;
use crypto::sha1::Sha1;
use crypto::digest::Digest;
use rocket::{Responder};
use rocket::http::{ContentType, Header};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode as jwt_encode, Header as jwt_Header, EncodingKey};

// Local
use crate::models::User;

#[derive(Deserialize, Serialize)]
pub struct JsonResponse {
	ok: bool,
	message: Option<String>
}

struct TokenHeader {
	token: Option<String>
}

impl TokenHeader {
	pub fn new(token: Option<String>) -> Self {
		TokenHeader {
			token: token
		}
	}
}

impl<'r> Into<Header<'r>> for TokenHeader {
	fn into(self) -> Header<'r> {
		match self.token {
			Some(t) => Header::new("token", t),
			None => Header::new("token", "")
		}
	}
}

impl JsonResponse {
	pub fn ok() -> Self {
		JsonResponse {
			ok: true,
			message: None
		}
	}
	
	pub fn err(message: &str) -> Self {
		JsonResponse {
			ok: false,
			message: Some(message.to_owned())
		}
	}
}


#[derive(Responder)]
pub struct LoginResponse {
	// Responder object
	inner: Json<JsonResponse>,
	// Headers
	content_type: ContentType,
	token: TokenHeader
}

impl LoginResponse {
	pub fn new(user: Option<User>) -> Self {
		if user.is_none() {
			return LoginResponse {
				inner: Json(JsonResponse::err("Could not find user")),
				content_type: ContentType::JSON,
				token: TokenHeader::new(None)
			}
		}

		let encoded_token = jwt_encode(&jwt_Header::default(), &user, &EncodingKey::from_secret("yolo".as_ref()));

		match encoded_token {
			Ok(token) => {
				LoginResponse {
					inner: Json(JsonResponse::ok()),
					content_type: ContentType::JSON,
					token: TokenHeader::new(Some(token)),
				}
			},
			Err(err) => {
				println!("{:?}", err);

				LoginResponse {
					inner: Json(JsonResponse::err("Error generating authorisation token")),
					content_type: ContentType::JSON,
					token: TokenHeader::new(None)
				}
			}
		}
	}
}

pub fn generate_hash(id: &str) -> String {
	let mut hasher = Sha1::new();
	hasher.input_str(&id);
	hasher.result_str()
}