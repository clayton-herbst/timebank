pub mod auth;
pub mod headers;
pub mod helpers;
pub mod request;
pub mod response;

use std::result::Result;
use diesel::result::Error;
use rocket::response::NamedFile;
use rocket::response::status::NotFound;
use rocket_contrib::json::Json;
use std::path::{Path, PathBuf};

// Local
use crate::models::{User, Status, Category};
use crate::DbConn;
use auth::{AuthTokenBuilder, UserClaims};
use request::{AuthReq, NewUser};
use response::{LoginResponse, Response};

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

#[get("/login?<username>")]
pub fn login(conn: DbConn, username: String) -> LoginResponse {
    let id = helpers::generate_hash(username.as_str());
    let user: Option<User> = User::get_user(conn, id.as_str()).ok();

    if user.is_none() {
        return LoginResponse::error(Some(String::from("Could not find user")));
    }

    let auth_builder = AuthTokenBuilder::default();

    let auth_token = auth_builder.encode(UserClaims::from(user.unwrap()));

    match auth_token {
        Ok(token) => LoginResponse::success(token),
        Err(err) => LoginResponse::error(Some(err.to_string())),
    }
}

#[post("/signup", data = "<user>")]
pub fn signup(conn: DbConn, user: Json<NewUser>) -> Response {
    let user_entry: User = user.into_inner().into();

    let db_query_err: Option<Error> = User::add_user(conn, &user_entry).err();

    match db_query_err {
        Some(err) => {
            println!("{}", err);

            Response::error(Some(err.to_string()))
        }
        None => Response::default(),
    }
}

#[get("/protect")]
pub fn protect(authorized: AuthReq) -> Response {
    match authorized {
        AuthReq::Valid(id) => {
            println!("{}", id);
            Response::success()
        }
        AuthReq::InValid(err_str) => Response::error(Some(err_str)),
        AuthReq::NoToken => Response::error(Some(String::from("Bad Request"))),
    }
}

#[get("/statuses")]
pub fn statuses(conn: DbConn) -> Result<Json<Vec<Status>>,  NotFound<String>> {
    match Status::all(conn) {
        Ok(results) => Ok(Json(results)),
        Err(err) => Err(NotFound(err.to_string()))
    }
}

#[get("/categories")]
pub fn categories(conn: DbConn) -> Result<Json<Vec<Category>>, NotFound<String>> {
    match Category::all(conn) {
        Ok(results) => Ok(Json(results)),
        Err(err) => Err(NotFound(err.to_string()))
    }
}