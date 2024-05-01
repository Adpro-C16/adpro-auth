use chrono::Utc;
use dotenvy::dotenv;
use jsonwebtoken::{
    decode, encode,
    errors::{Error, ErrorKind},
    DecodingKey, EncodingKey, Header, Validation,
};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};
use std::env;

pub mod typing;

use crate::typing::{NetworkResponse, Response, ResponseBody};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub id: i32,
    pub role: String,
    exp: usize,
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims,
}

fn is_valid(key: &str) -> Result<Claims, Error> {
    Ok(decode_jwt(String::from(key))?)
}

pub fn verify_token(token: &str) -> Result<Claims, String> {
    dotenv().ok();
    match is_valid(token) {
        Ok(claims) => Ok(claims),
        Err(err) => match &err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                return Err("Error validating JWT token - Expired Token".to_string());
            }
            jsonwebtoken::errors::ErrorKind::InvalidToken => {
                return Err("Error validating JWT token - Invalid Token".to_string());
            }
            _ => {
                return Err(format!("Error validating JWT token - {}", err));
            }
        },
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = NetworkResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, NetworkResponse> {
        match req.headers().get_one("authorization") {
            None => {
                let response = Response {
                    body: ResponseBody::Message(String::from(
                        "Error validating JWT token - No token provided",
                    )),
                };
                Outcome::Error((
                    Status::Unauthorized,
                    NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap()),
                ))
            }
            Some(key) => match verify_token(key) {
                Ok(claims) => Outcome::Success(JWT { claims }),
                Err(err) => {
                    let response = Response {
                        body: ResponseBody::Message(err),
                    };
                    Outcome::Error((
                        Status::Unauthorized,
                        NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap()),
                    ))
                }
            },
        }
    }
}

pub fn create_jwt(id: i32, role: String) -> Result<String, Error> {
    dotenv().ok();
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(1))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        id,
        role,
        exp: expiration as usize,
    };

    let header = Header::new(jsonwebtoken::Algorithm::HS512);
    let secret = env::var("SECRET_KEY").expect("JWT_SECRET must be set.");
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
    dotenv().ok();
    let token = token.trim_start_matches("Bearer").trim();
    let secret = env::var("SECRET_KEY").expect("JWT_SECRET must be set.");

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(jsonwebtoken::Algorithm::HS512),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned()),
    }
}
