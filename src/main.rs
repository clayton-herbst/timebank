#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::helmet::SpaceHelmet;

// Models
use timebank::models::database::DbConn;

// Routes
use timebank::routes;

fn main() {
    rocket::ignite()
        .attach(SpaceHelmet::default())
        .attach(DbConn::fairing())
        .mount("/", routes![routes::welcome, routes::static_files])
        .mount(
            "/api/",
            routes![routes::health, routes::login, routes::signup, routes::protect, routes::statuses, routes::categories, routes::activities, routes::add_activity]
        )
        .launch();
}
