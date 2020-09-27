use rocket::http::{ContentType, Status};
use rocket::response::{self, Response};
use serde::Serialize;
use serde_json::to_vec_pretty;
use std::default::Default;
use std::io::Cursor;

#[derive(Serialize)]
struct JsonResponse {
	ok: bool,
}

impl Default for JsonResponse {
	fn default() -> Self {
		JsonResponse { ok: true }
	}
}

pub fn success<'r, T>(contents: T) -> response::Result<'r>
where
	T: Serialize,
{
	let serialized_body = to_vec_pretty(&contents);

	match serialized_body {
		Ok(body) => {
			let body_stream = Cursor::new(body);
			Response::build()
				.status(Status::Ok)
				.header(ContentType::JSON)
				.sized_body(body_stream)
				.ok()
		}
		Err(_) => Err(Status::InternalServerError),
	}
}

pub fn bad_request<'r>(_message: &'static str) -> response::Result<'r> {
	let serialized_body = to_vec_pretty(&JsonResponse { ok: false });

	match serialized_body {
		Ok(body) => {
			let body_stream = Cursor::new(body);
			Response::build()
				.status(Status::BadRequest)
				.header(ContentType::JSON)
				.sized_body(body_stream)
				.ok()
		}
		Err(_) => Err(Status::InternalServerError),
	}
}

pub fn internal_server_error<'r>() -> response::Result<'r> {
	let serialized_body = to_vec_pretty(&JsonResponse { ok: false });

	match serialized_body {
		Ok(body) => {
			let body_stream = Cursor::new(body);
			Response::build()
				.status(Status::InternalServerError)
				.header(ContentType::JSON)
				.sized_body(body_stream)
				.ok()
		}
		Err(_) => Err(Status::InternalServerError),
	}
}
