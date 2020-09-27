use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

// Models
use crate::models::auth::TokenClaims;

// Helpers
use crate::helpers::decode;

pub type ProtectedRequest = TokenClaims;

impl<'a, 'r> FromRequest<'a, 'r> for ProtectedRequest {
	type Error = ();

	fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
		let req_header_token = request.headers().get_one("token");

		if req_header_token.is_none() {
			return Outcome::Failure((Status::BadRequest, ()));
		}

		match decode::<TokenClaims>(
			req_header_token.expect("Error unwrapping auth token in request header."),
		) {
			Ok(jwt_token) => Outcome::Success(jwt_token.claims),
			Err(_err) => Outcome::Failure((Status::Unauthorized, ())),
		}
	}
}
