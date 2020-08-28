use std::fmt::Debug;
use std::string::String;

use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Insertable, Queryable, Serialize, Deserialize, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub dob: Option<String>,
}
