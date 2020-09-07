use rocket::request::{self, FromRequest};
use rocket::Outcome;
use rocket::Request;
use serde::{Deserialize, Serialize};
use std::convert::Into;
use std::fmt::Debug;
use std::string::String;

// Local
use super::auth::{AuthTokenBuilder, UserClaims};
use super::helpers::generate_hash;
use crate::models::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    username: String,
    first_name: String,
    last_name: String,
    email: Option<String>,
    dob: Option<String>,
}

impl Into<User> for NewUser {
    fn into(self) -> User {
        let id: String = generate_hash(self.username.as_str());

        User {
            id,
            first_name: self.first_name,
            last_name: self.last_name,
            email: self.email,
            dob: self.dob,
        }
    }
}

#[derive(Debug)]
pub enum AuthReq {
    Valid(String),
    InValid(String),
    NoToken,
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthReq {
    type Error = AuthReq;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("token");

        if token.is_none() {
            return Outcome::Success(AuthReq::NoToken);
        }

        let token_builder = AuthTokenBuilder::default();
        let decoded_token = token_builder.decode::<UserClaims>(token.unwrap().to_owned());

        match decoded_token {
            Ok(data) => Outcome::Success(AuthReq::Valid(data.claims.id)),
            Err(err) => Outcome::Success(AuthReq::InValid(err.to_string())),
        }
    }
}
