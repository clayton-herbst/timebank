pub mod request;
pub mod helpers;
pub mod response;
pub mod headers;

use rocket::response::NamedFile;
use rocket_contrib::json::{Json};
use std::path::{Path, PathBuf};
use diesel::result::{Error};
use jsonwebtoken::{encode as jwt_encode, Header as jwt_Header, EncodingKey};
use dotenv;

// Local
use request::NewUser;
use response::{LoginResponse, Response};
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
pub fn login<'r>(conn: DbConn, username: String) -> LoginResponse {
    let id = helpers::generate_hash(username.as_str()); 
    let user: Option<User> = User::get_user(conn, id.as_str()).ok();

    if user.is_none() {
        return LoginResponse::error(Some(String::from("Could not find user")));
    }

    let secret = dotenv::var("SECRET_KEY").expect("Unable to read SECRET_KEY from .env");

    let auth_token = jwt_encode(&jwt_Header::default(), &user, &EncodingKey::from_secret(secret.as_ref()));

    match auth_token {
        Ok(token) => {
            LoginResponse::success(token)
        },
        Err(err) => {
            println!("{:?}", err);

            LoginResponse::error(Some(err.to_string()))
        }
    }
}

#[post("/signup", data = "<user>")]
pub fn signup<'r>(conn: DbConn, user: Json<NewUser>) -> Response {
    let user_entry: User = user.into_inner().into();

    let db_query_err: Option<Error> = User::add_user(conn, &user_entry).err();

    match db_query_err {
        Some(err) => {
            println!("{}", err);
            
            Response::error(Some(err.to_string()))
        },
        None => {
            Response::default()
        }
    }
}
