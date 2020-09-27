use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde::Serialize;
use serde_json::to_vec_pretty;
use std::default::Default;
use std::io::Cursor;

// Models
use crate::models::response::{BooleanJson, ErrorJson};

#[derive(Serialize)]
pub enum HttpResponse<T>
where
	T: Serialize,
{
	Ok(T),
	BadRequest(String),
	InternalServerError(String),
	Unauthorized(String),
}

impl Default for HttpResponse<BooleanJson> {
	fn default() -> HttpResponse<BooleanJson> {
		HttpResponse::Ok(BooleanJson::default())
	}
}

impl<'r, T> Responder<'r> for HttpResponse<T>
where
	T: Serialize,
{
	fn respond_to(self, _request: &Request) -> response::Result<'r> {
		match self {
			HttpResponse::Ok(contents) => HttpResponseBuilder::build(Status::Ok, contents),
			HttpResponse::BadRequest(message) => {
				HttpResponseBuilder::build(Status::BadRequest, ErrorJson::new(message))
			}
			HttpResponse::InternalServerError(message) => {
				HttpResponseBuilder::build(Status::InternalServerError, ErrorJson::new(message))
			}
			HttpResponse::Unauthorized(message) => {
				HttpResponseBuilder::build(Status::Unauthorized, ErrorJson::new(message))
			}
		}
	}
}

pub struct HttpResponseBuilder {}

impl HttpResponseBuilder {
	pub fn build<'r, T>(status: Status, contents: T) -> response::Result<'r>
	where
		T: Serialize,
	{
		let serialized_body = to_vec_pretty(&contents);

		match serialized_body {
			Ok(body) => {
				let body_stream = Cursor::new(body);
				Response::build()
					.status(status)
					.header(ContentType::JSON)
					.sized_body(body_stream)
					.ok()
			}
			Err(_) => Err(Status::InternalServerError),
		}
	}

	pub fn build_empty<'r>(status: Status) -> response::Result<'r> {
		Response::build().status(status).ok()
	}
}
