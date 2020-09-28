use rocket::http::{Header, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};

// Models
use crate::models::auth::{SignUpUser, TokenClaims};
use crate::models::database::{DbConn, User};

// Helpers
use crate::helpers::encode;
use crate::helpers::http_response::HttpResponseBuilder;

pub struct SignUpController {
	user: User,
}

impl SignUpController {
	pub fn new(new_user: SignUpUser) -> SignUpController {
		let user: User = new_user.into();

		SignUpController { user }
	}
}

impl<'r> Responder<'r> for SignUpController {
	fn respond_to(self, request: &Request) -> response::Result<'r> {
		// database connection from pool
		let conn = request.guard::<DbConn>().succeeded();
		if conn.is_none() {
			return HttpResponseBuilder::build_empty(Status::InternalServerError);
		}
		match User::add_user(&conn.unwrap(), &self.user) {
			Ok(num_entries) => {
				if num_entries == 0 {
					return HttpResponseBuilder::build_empty(Status::InternalServerError);
				}
			}
			Err(err) => {
				println!("{}", err);
				return HttpResponseBuilder::build_empty(Status::InternalServerError);
			}
		}

		let token = encode::<TokenClaims>(self.user.to_owned().into());
		if token.is_err() {
			return HttpResponseBuilder::build_empty(Status::InternalServerError);
		}

		HttpResponseBuilder::build(Status::Ok, self.user).and_then(|base| {
			Response::build_from(base)
				.header(Header::new("token", token.unwrap()))
				.ok()
		})
	}
}
