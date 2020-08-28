#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
extern crate crypto;

#[database("main_db")]
pub struct DbConn(diesel::SqliteConnection);

pub mod models;
pub mod routes;
pub mod schema;
