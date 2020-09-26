use serde::{Deserialize, Serialize};
use std::convert::Into;
use std::fmt::Debug;
use std::string::String;
use chrono::offset::Utc;
use chrono::DateTime;

// Models
use crate::models::database::User;

// Helpers
use crate::helpers::generate_hash;

#[derive(Serialize, Deserialize, Debug)]
pub struct SignUpUser {
    username: String,
    first_name: String,
    last_name: String,
    email: Option<String>,
    dob: Option<String>,
}

impl Into<User> for SignUpUser {
    fn into(self) -> User {
        let id: String = generate_hash(self.username.as_str());

        User {
            id,
            first_name: self.first_name,
            last_name: self.last_name,
            email: self.email,
            dob: self.dob,
        }
    }
}

/// JWT Token Contents
#[derive(Serialize, Deserialize, Debug)]
pub struct TokenClaims {
    exp: i64,
    pub id: String,
}

impl TokenClaims {
    pub fn new(id: String) -> TokenClaims {
        let now: DateTime<Utc> = Utc::now();
        let exp_duration: i64 = 60 * 5; // 5 minute duration

        TokenClaims {
            exp: now.timestamp() + exp_duration,
            id,
        }
    }
}

impl From<User> for TokenClaims {
    fn from(user: User) -> TokenClaims {
        let now: DateTime<Utc> = Utc::now();
        let exp_duration: i64 = 60 * 5; // 5 minute token expiration

        TokenClaims {
            exp: now.timestamp() + exp_duration,
            id: user.id,
        }
    }
}