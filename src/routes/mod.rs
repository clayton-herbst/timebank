pub mod response;

use diesel::result::Error;
use rocket::response::NamedFile;
use rocket_contrib::json::Json;
use std::path::{Path, PathBuf};

// Controllers
use crate::controllers::auth::{ProtectedRequest, LoginController};

// Models
use crate::models::database::{User, Status, Category, Activity, DbConn};
use crate::models::auth::SignUpUser;
use crate::models::activity::{NewActivity};

// Local
use response::{Response};

#[get("/")]
pub fn welcome() -> Option<NamedFile> {
    NamedFile::open(Path::new("public/index.html")).ok()
}

#[get("/health")]
pub fn health() -> &'static str {
    "RustBank server is healthy"
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
pub fn login(controller: LoginController) -> LoginController {
    controller
}

#[post("/signup", data = "<user>")]
pub fn signup(conn: DbConn, user: Json<SignUpUser>) -> Response {
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
pub fn protect(token: ProtectedRequest) -> Response {
    println!("{}", token.id);

    Response::default()
}

#[get("/status/all")]
pub fn statuses(conn: DbConn) -> Response {
    match Status::all(conn) {
        Ok(results) => Response::success(results),
        Err(err) => Response::error(Some(err.to_string()))
    }
}

#[get("/category/all")]
pub fn categories(conn: DbConn) -> Response {
    match Category::all(conn) {
        Ok(results) => Response::success(results),
        Err(err) => Response::error(Some(err.to_string()))
    }
}

#[post("/activity", data = "<activity>")]
pub fn add_activity(_conn: DbConn, activity: Json<NewActivity>) -> Response {
    println!("{:?}", activity.into_inner());

    Response::default()
}

#[get("/activity/all")]
pub fn activities(token: ProtectedRequest, conn: DbConn) -> Response {
    match Activity::user_all(conn, &token.id) {
        Ok(results) => Response::success(results),
        Err(err) => Response::error(Some(err.to_string()))
    }
}
