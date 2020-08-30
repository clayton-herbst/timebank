pub mod data;
pub mod request;

use rocket::http::Status;
use rocket::response::NamedFile;
use diesel::prelude::*;
use rocket_contrib::json::{Json};
use std::path::{Path, PathBuf};

// Local
use self::data::{NewUser};
use self::request::{UserId, UserIdError};
use crate::schema::users;
use crate::models::User;
use crate::DbConn;


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

#[get("/login")]
pub fn get_user_info(id: Result<UserId, UserIdError>) -> Option<Status> { 

    match id {
        Ok(id) => {
            println!("{:?}", id);
            Some(Status::Ok)
        },
        Err(e) => {
            println!("{:?}", e);
            None
        }
    }
}

#[post("/signup", data = "<user>")]
pub fn signup(conn: DbConn, user: Json<NewUser>) -> Option<Status> {
    let user_entry: User = user.into_inner().create_user().unwrap();

    let response = diesel::insert_into(users::table).values(user_entry).execute(&*conn);

    match response {
        Ok(v) => {
            println!("{}", v);
            Some(Status::Ok)
        },
        Err(e) => {
            println!("{}", e);

            Some(Status::InternalServerError)
        }
    }
}
