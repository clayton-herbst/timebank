use rocket::http::{Header, Status};
use rocket::request::{self, FromRequest, Request};
use rocket::response::{self, Responder, Response};
use rocket::Outcome;
use std::string::String;

// Models
use crate::models::database::{DbConn, User};

// Helpers
use crate::helpers::http_response;
use crate::helpers::{encode, generate_hash};

pub struct LoginController(User);

impl<'a, 'r> FromRequest<'a, 'r> for LoginController {
	type Error = ();

	fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
		let username_query = request.get_query_value::<String>("username");
		if username_query.is_none() {
			return Outcome::Failure((Status::BadRequest, ()));
		}

		let user_id_hash = username_query
			.unwrap()
			.map(|username| generate_hash(&username));
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
				Outcome::Success(LoginController(user))
			}
			Err(err) => {
				println!("{}", err);
				Outcome::Failure((Status::NotFound, ()))
			}
		}
	}
}

impl<'r> Responder<'r> for LoginController {
	fn respond_to(self, _req: &Request) -> response::Result<'r> {
		let user = self.0.to_owned();
		let token = encode::<User>(self.0.into());
		if token.is_err() {
			return http_response::internal_server_error();
		}

		http_response::success(Some(&user)).and_then(|base| {
			Response::build_from(base)
				.header(Header::new("token", token.unwrap()))
				.ok()
		})
	}
}
