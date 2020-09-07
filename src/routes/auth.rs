use chrono::offset::Utc;
use chrono::DateTime;
use jsonwebtoken::errors;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::default::Default;

// Local
use crate::models::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserClaims {
    exp: i64,
    pub id: String,
}

impl UserClaims {
    pub fn new(id: String) -> UserClaims {
        let now: DateTime<Utc> = Utc::now();
        let exp_duration: i64 = 60 * 5; // 5 minute duration

        UserClaims {
            exp: now.timestamp() + exp_duration,
            id,
        }
    }
}

impl From<User> for UserClaims {
    fn from(user: User) -> UserClaims {
        let now: DateTime<Utc> = Utc::now();
        let exp_duration: i64 = 60 * 5; // 5 minute token expiration

        UserClaims {
            exp: now.timestamp() + exp_duration,
            id: user.id,
        }
    }
}

pub struct AuthTokenBuilder {
    secret: String,
    algorithm: Algorithm,
}

impl AuthTokenBuilder {
    pub fn decode<T>(self, token: String) -> errors::Result<TokenData<T>>
    where
        T: DeserializeOwned,
    {
        decode::<T>(
            &token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::new(self.algorithm),
        )
    }

    pub fn encode<T: Serialize>(self, claims: T) -> errors::Result<String> {
        encode(
            &Header::new(self.algorithm),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
    }
}

impl Default for AuthTokenBuilder {
    fn default() -> AuthTokenBuilder {
        let secret = dotenv::var("SECRET_KEY").expect("Unable to read SECRET_KEY from .env");

        AuthTokenBuilder {
            secret,
            algorithm: Algorithm::HS256,
        }
    }
}
