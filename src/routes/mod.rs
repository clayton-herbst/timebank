pub mod data;
pub mod request;

use rocket::response::{NamedFile, status};
use rocket_contrib::json::{Json};
use std::path::{Path, PathBuf};
use diesel::result::{Error};

// Local
use self::data::{NewUser};
use self::request::{generate_hash, LoginResponse, JsonResponse};
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

#[get("/login?<username>")]
pub fn login(conn: DbConn, username: String) -> LoginResponse {
    let id = generate_hash(username.as_str()); 
    let user: Option<User> = User::get_user(conn, id.as_str()).ok();

    LoginResponse::new(user)
}

#[post("/signup", data = "<user>")]
pub fn signup<'r>(conn: DbConn, user: Json<NewUser>) -> Result<status::Accepted::<Json::<JsonResponse>>, status::Conflict::<String>> {
    let user_entry: User = user.into_inner().create_user();

    let query_err: Option<Error> = User::create_user(conn, &user_entry).err();

    match query_err {
        Some(err) => {
            println!("{}", err);
            
            Err(status::Conflict(Some(err.to_string())))
        },
        None => {
            Ok(status::Accepted(Some(Json(JsonResponse::ok()))))
        }
    }
}
