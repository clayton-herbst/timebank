use rocket::http;
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
use crate::models::response::{BooleanJson, ErrorJson};

// Helpers
use crate::helpers::http_response::{HttpResponse, HttpResponseBuilder};

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
pub fn signup<'r>(_conn: DbConn, user: Json<SignUpUser>) -> HttpResponse<ErrorJson> {
    let _user_entry: User = user.into_inner().into();

    //let db_query_err: Option<Error> = User::add_user(conn, &user_entry).err();

    HttpResponse::InternalServerError("Sign up controller not implemented".to_string())
}

#[get("/protect")]
pub fn protect<'r>(token: ProtectedRequest) -> HttpResponse<BooleanJson> {
    println!("{}", token.id);

    HttpResponse::Ok(BooleanJson::default())
}

#[get("/status/all")]
pub fn statuses<'r>(conn: DbConn) -> response::Result<'r> {
    match Status::all(conn) {
        Ok(results) => HttpResponseBuilder::build(http::Status::Ok, results),
        Err(_) => HttpResponseBuilder::build(
            http::Status::InternalServerError,
            ErrorJson::new("Could not fetch all status options".to_string()),
        ),
    }
}

#[get("/category/all")]
pub fn categories<'r>(conn: DbConn) -> response::Result<'r> {
    match Category::all(conn) {
        Ok(results) => HttpResponseBuilder::build(http::Status::Ok, results),
        Err(_) => HttpResponseBuilder::build(
            http::Status::InternalServerError,
            ErrorJson::new("Could not fetch all status options".to_string()),
        ),
    }
}

#[post("/activity", data = "<activity>")]
pub fn add_activity<'r>(_conn: DbConn, activity: Json<NewActivity>) -> HttpResponse<ErrorJson> {
    println!("{:?}", activity.into_inner());

    HttpResponse::InternalServerError("Sign up controller not implemented".to_string())
}

#[get("/activity/all")]
pub fn activities<'r>(token: ProtectedRequest, conn: DbConn) -> response::Result<'r> {
    match Activity::user_all(conn, &token.id) {
        Ok(results) => HttpResponseBuilder::build(http::Status::Ok, results),
        Err(_) => HttpResponseBuilder::build(
            http::Status::InternalServerError,
            ErrorJson::new("Could not fetch all status options".to_string()),
        ),
    }
}
