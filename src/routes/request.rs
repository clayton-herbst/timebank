use std::fmt::Debug;
use std::string::String;
use std::convert::Into;
use crate::models::User;
use super::helpers::generate_hash;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    username: String,
    first_name: String,
    last_name: String,
    email: Option<String>,
    dob: Option<String>,
}

impl Into<User> for NewUser {
    fn into(self) -> User {
        let id: String = generate_hash(self.username.as_str());
        
        User {
            id: id,
            first_name: self.first_name,
            last_name: self.last_name,
            email: self.email,
            dob: self.dob,
        }
    }
}