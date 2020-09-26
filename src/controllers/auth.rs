use std::string::String;
use std::default::Default;
use rocket::request::{self, FromRequest, Request};
use rocket::response::Responder;
use rocket::Outcome;
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::Serialize;

// Models
use crate::models::auth::TokenClaims;
use crate::models::database::{DbConn, User};

// Helpers
use crate::helpers::{decode, generate_hash};

#[derive(Serialize)]
struct Success {
	ok: bool
}

impl Default for Success {
	fn default() -> Self {
		Success {
			ok: true
		}
	}
}

#[derive(Responder)]
pub struct LoginController(Json<User>);

impl <'a, 'r> FromRequest<'a, 'r> for LoginController {
	type Error = ();

	fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
		let username_query = request.get_query_value::<String>("username");
		if username_query.is_none() {
			return Outcome::Failure((Status::BadRequest, ()));
		}

		let user_id_hash = username_query.unwrap().map(|username| {
			generate_hash(&username)
		});
		if user_id_hash.is_err() {
			return Outcome::Failure((Status::InternalServerError, ()));
		}

		// database connection from pool
		let conn = request.guard::<DbConn>().succeeded();
		if conn.is_none() {
			return Outcome::Failure((Status::InternalServerError, ()));
		}

		// fetch user from database
		let db_user = User::get_user(conn.unwrap(), &user_id_hash.unwrap());
		
		match db_user {
			Ok(user) => {
				println!("{:?}", user);
				Outcome::Success(
					LoginController(
						Json(user)
					)
				)
			},
			Err(err) => {
				println!("{}", err);
				Outcome::Failure((Status::NotFound, ()))
			}
		}
	}
}

pub type ProtectedRequest = TokenClaims;

impl<'a, 'r> FromRequest<'a, 'r> for ProtectedRequest {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let req_header_token = request.headers().get_one("token");

        if req_header_token.is_none() {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        match decode::<TokenClaims>(req_header_token.expect("Error unwrapping auth token in request header.")) {
            Ok(jwt_token) => Outcome::Success(jwt_token.claims),
            Err(_err) => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}