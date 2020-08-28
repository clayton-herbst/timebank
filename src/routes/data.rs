use std::fmt::Debug;
use std::string::String;

use crate::models::User;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUser {
    username: String,
    first_name: String,
    last_name: String,
    email: Option<String>,
    dob: Option<String>,
}

impl NewUser {
    pub fn create_user(self) -> User {
        let mut hasher = Sha256::new();
        hasher.input_str(&self.username);

        User {
            id: hasher.result_str(),
            first_name: self.first_name,
            last_name: self.last_name,
            email: self.email,
            dob: self.dob,
        }
    }
}
