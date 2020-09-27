use serde::Serialize;
use std::default::Default;

#[derive(Serialize)]
pub struct BooleanJson {
	ok: bool,
}

impl Default for BooleanJson {
	fn default() -> BooleanJson {
		BooleanJson { ok: true }
	}
}

impl BooleanJson {
	pub fn new(success: bool) -> BooleanJson {
		BooleanJson { ok: success }
	}
}

#[derive(Serialize)]
pub struct ErrorJson {
	message: String,
}

impl ErrorJson {
	pub fn new(message: String) -> ErrorJson {
		ErrorJson { message }
	}
}
