use rocket_contrib::json::{JsonValue};

#[get("/")]
pub fn welcome() -> Option<&'static str> {
  Some("Welcome to the time bank!")
}

#[get("/user")]
pub fn get_user_info() -> Option<JsonValue> {
	Some(json!({
			"id": 83,
			"values": [1, 2, 3, 4]
	}))
}