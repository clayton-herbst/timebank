#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::helmet::SpaceHelmet;
use time_bank::routes;

fn main() {
    rocket::ignite()
        .attach(SpaceHelmet::default())
        .mount("/", routes![routes::welcome, routes::static_files])
        .mount("/api/", routes![routes::get_user_info])
        .launch();
}
