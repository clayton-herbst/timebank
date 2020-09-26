use rocket::http::ContentType;
use rocket::Responder;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fmt::Debug;
use std::string::String;
use rocket::http::Header;

#[derive(Responder)]
#[response(status = 202)]
pub struct TokenResponse<'h> {
    inner: Json<JsonValue>,
    content_type: ContentType,
    token: Header<'h>,
}

impl<'h> TokenResponse<'h> {
    fn new(token: String) -> Self {
        TokenResponse {
            inner: Json(json!({
                "ok": true
            })),
            content_type: ContentType::JSON,
            token: Header::new("token", token),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorJson {
    ok: bool,
    message: String,
}

impl ErrorJson {
    pub fn new(message: String) -> Self {
        ErrorJson { ok: false, message }
    }
}

impl Default for ErrorJson {
    fn default() -> ErrorJson {
        ErrorJson {
            ok: false,
            message: String::from("Error has occured!"),
        }
    }
}

#[derive(Responder)]
pub struct ErrorResponse {
    inner: Json<ErrorJson>,
    content_type: ContentType,
}

impl ErrorResponse {
    pub fn new(message: String) -> ErrorResponse {
        ErrorResponse {
            inner: Json(ErrorJson::new(message)),
            content_type: ContentType::JSON,
        }
    }
}

impl Default for ErrorResponse {
    fn default() -> ErrorResponse {
        ErrorResponse {
            inner: Json(ErrorJson::default()),
            content_type: ContentType::JSON,
        }
    }
}

#[derive(Responder)]
pub enum LoginResponse<'h> {
    Success(TokenResponse<'h>),
    Error(ErrorResponse),
}

impl<'h> LoginResponse<'h> {
    pub fn success(token: String) -> LoginResponse<'h> {
        LoginResponse::Success(TokenResponse::new(token))
    }

    pub fn error(message: Option<String>) -> LoginResponse<'h> {
        match message {
            Some(m) => LoginResponse::Error(ErrorResponse::new(m)),
            None => LoginResponse::Error(ErrorResponse::default()),
        }
    }
}

#[derive(Responder)]
pub enum Response {
    #[response(status = 200)]
    Success(Json<JsonValue>),
    #[response(status = 500)]
    Error(ErrorResponse),
}

impl Default for Response {
    fn default() -> Response {
        Response::Success(Json(json!({
            "ok": true
        })))
    }
}

impl Response {
    pub fn success<T>(body: T) -> Response
        where T: Serialize
    {
        Response::Success(Json(json!(body)))
    }

    pub fn error(message: Option<String>) -> Response {
        match message {
            Some(m) => Response::Error(ErrorResponse::new(m)),
            None => Response::Error(ErrorResponse::default()),
        }
    }
}
