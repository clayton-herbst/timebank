pub mod auth;
pub mod headers;
pub mod helpers;
pub mod request;
pub mod response;

use diesel::result::Error;
use rocket::response::NamedFile;
use rocket_contrib::json::Json;
use std::path::{Path, PathBuf};

// Local
use crate::models::{User, Status, Category, Activity};
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
            Response::default()
        }
        AuthReq::InValid(err_str) => Response::error(Some(err_str)),
        AuthReq::NoToken => Response::error(Some(String::from("Bad Request"))),
    }
}

#[get("/statuses")]
pub fn statuses(conn: DbConn) -> Response {
    match Status::all(conn) {
        Ok(results) => Response::success(results),
        Err(err) => Response::error(Some(err.to_string()))
    }
}

#[get("/categories")]
pub fn categories(conn: DbConn) -> Response {
    match Category::all(conn) {
        Ok(results) => Response::success(results),
        Err(err) => Response::error(Some(err.to_string()))
    }
}

#[get("/activities")]
pub fn activities(auth: AuthReq, conn: DbConn) -> Response {
    let auth_req: Result<String, String> = match auth {
        AuthReq::Valid(id) => {
            println!("{}", id);
            Ok(id)
        },
        AuthReq::InValid(err_str) => Err(err_str),
        AuthReq::NoToken => Err(String::from("Bad request")),
    };

    if auth_req.is_err() {
        return Response::error(auth_req.err());
    }
    
    let user_id: String = auth_req.ok().expect("Expected user_id in request jwt token");
    
    match Activity::user_all(conn, &user_id) {
        Ok(results) => Response::success(results),
        Err(err) => Response::error(Some(err.to_string()))
    }
}
