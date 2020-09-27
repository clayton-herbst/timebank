use rocket::response::{self, NamedFile};
use rocket_contrib::json::Json;
use std::path::{Path, PathBuf};

// Controllers
use crate::controllers::auth::ProtectedRequest;
use crate::controllers::login::LoginController;

// Models
use crate::models::activity::NewActivity;
use crate::models::auth::SignUpUser;
use crate::models::database::{Activity, Category, DbConn, Status, User};

// Helpers
use crate::helpers::http_response;

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
pub fn signup<'r>(_conn: DbConn, user: Json<SignUpUser>) -> response::Result<'r> {
    let _user_entry: User = user.into_inner().into();

    //let db_query_err: Option<Error> = User::add_user(conn, &user_entry).err();

    http_response::internal_server_error()
}

#[get("/protect")]
pub fn protect<'r>(token: ProtectedRequest) -> response::Result<'r> {
    println!("{}", token.id);

    http_response::internal_server_error()
}

#[get("/status/all")]
pub fn statuses<'r>(conn: DbConn) -> response::Result<'r> {
    match Status::all(conn) {
        Ok(results) => http_response::success(results),
        Err(_) => http_response::internal_server_error(),
    }
}

#[get("/category/all")]
pub fn categories<'r>(conn: DbConn) -> response::Result<'r> {
    match Category::all(conn) {
        Ok(results) => http_response::success(results),
        Err(_) => http_response::internal_server_error(),
    }
}

#[post("/activity", data = "<activity>")]
pub fn add_activity<'r>(_conn: DbConn, activity: Json<NewActivity>) -> response::Result<'r> {
    println!("{:?}", activity.into_inner());

    http_response::internal_server_error()
}

#[get("/activity/all")]
pub fn activities<'r>(token: ProtectedRequest, conn: DbConn) -> response::Result<'r> {
    match Activity::user_all(conn, &token.id) {
        Ok(results) => http_response::success(results),
        Err(_) => http_response::internal_server_error(),
    }
}
