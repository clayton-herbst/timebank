use crate::schema::users;
use std::string::String;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use rocket::FromForm;

#[derive(Insertable, Serialize, Deserialize, FromForm, Debug)]
#[table_name="users"]
pub struct NewUser {
    pub id: String,
		pub first_name: String,
		pub family_name: String,
		pub email: String,
		pub dob: String,
}