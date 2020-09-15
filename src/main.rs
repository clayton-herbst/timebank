#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::helmet::SpaceHelmet;
use timebank::{routes, DbConn};

fn main() {
    rocket::ignite()
        .attach(SpaceHelmet::default())
        .attach(DbConn::fairing())
        .mount("/", routes![routes::welcome, routes::static_files])
        .mount(
            "/api/",
            routes![routes::login, routes::signup, routes::protect, routes::statuses, routes::categories],
        )
        .launch();
}
