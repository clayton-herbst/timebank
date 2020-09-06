use std::fmt::Debug;
use std::string::String;

use diesel::prelude::*;
use diesel::{Insertable, Queryable, QueryResult};
use serde::{Deserialize, Serialize};

use crate::schema::users;
use crate::DbConn;

#[derive(Insertable, Queryable, Serialize, Deserialize, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub dob: Option<String>,
}

impl User {
    pub fn get_user(conn: DbConn, id: &str) -> QueryResult<User> {
        users::table.find(id).first::<User>(&*conn)
    }

    pub fn add_user(conn: DbConn, user: &User) -> QueryResult<usize> {
        diesel::insert_into(users::table).values(user).execute(&*conn)
    }
}