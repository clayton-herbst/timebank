use rocket_contrib::json::{JsonValue, Json};
use rocket::response::{ NamedFile};
use std::path::{PathBuf, Path};
use super::models::NewUser;
use rocket::http::Status;

#[get("/")]
pub fn welcome() -> Option<NamedFile> {
	NamedFile::open(Path::new("public/index.html")).ok()
}

#[get("/static/<file..>")]
pub fn static_files(file: PathBuf) -> Option<NamedFile> {
	let mut path_buf = file;
	if path_buf.file_name() == None {
		path_buf.set_file_name("index");
		path_buf.set_extension("html");
	}

	NamedFile::open(Path::new("public/static/").join(path_buf)).ok()
}

#[get("/user")]
pub fn get_user_info() -> Option<JsonValue> {
	Some(json!({
			"id": 83,
			"values": [1, 2, 3, 4]
	}))
}

#[post("/signup", data = "<user>")]
pub fn signup<'r>(user: Json<NewUser>) -> Status {
	println!("{:?}", user.into_inner());

	Status::Ok
}