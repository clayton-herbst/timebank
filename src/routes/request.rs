use std::string::String;
use std::fmt::Debug;
use std::convert::{From, Into};
use crypto::sha1::Sha1;
use crypto::digest::Digest;
use rocket::http::{Status, RawStr};
use rocket::{Outcome};
use rocket::request::{self, Request, FromRequest, FromFormValue};
use diesel::prelude::*;

// Local
use crate::schema::users;
use crate::models::User;
use crate::DbConn;

#[derive(Debug)]
pub struct UserId {
	id: String
}

#[derive(Debug)]
pub enum UserIdError {
	Missing,
	NotExist
}

impl UserId {
	pub fn new(id: String) -> UserId {
		let mut hasher = Sha1::new();
		hasher.input_str(&id);

		UserId {
			id: hasher.result_str()
		}
	}
}

impl<'a, 'r> FromRequest<'a, 'r> for UserId {
	type Error = UserIdError;

	fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
		let username_query: Option<UserId> = request.get_query_value("id")
			.and_then(|q| q.ok());

		let conn = request.guard::<DbConn>().unwrap();

		let query_result = username_query.map(|id| users::table.find::<String>(id.into()).first::<User>(&*conn).unwrap());

		match query_result {
			Some(user) => {
				println!("{:?}", user);
				Outcome::Success(UserId::new(user.id))
			},
			None => Outcome::Failure((Status::NoContent, UserIdError::NotExist))
		}
	}
}

impl<'v> FromFormValue<'v> for UserId {
    type Error = UserIdError;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        match form_value.parse::<String>() {
            Ok(id) => Ok(UserId::new(id)),
            _ => Err(UserIdError::Missing),
        }
	}
}

impl From<String> for UserId {
	fn from(id: String) -> Self {
		UserId {
			id: id
		}
	}
}

impl Into<String> for UserId {
	fn into(self) -> String {
		self.id
	}
}

impl From<&'_ str> for UserId {
	fn from(id: &'_ str) -> Self {
		UserId {
			id: String::from(id)
		}
	}
}