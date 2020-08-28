mod data;

use rocket::http::Status;
use rocket::response::NamedFile;
use rocket_contrib::json::{Json, JsonValue};
use std::path::{Path, PathBuf};

// Data
use self::data::NewUser;

// Database
use crate::schema::users::dsl::*;
use crate::DbConn;
use diesel::prelude::*;

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
pub fn signup(conn: DbConn, user: Json<NewUser>) -> Option<Status> {
    let data = user.into_inner().create_user();

    println!("{:?}", data);

    diesel::insert_into(users)
        .values(data)
        .execute(&*conn)
        .unwrap();

    Some(Status::Ok)
}
