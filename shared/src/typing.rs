use rocket::serde::{Deserialize, Serialize};
use rocket::Responder;

#[derive(Responder, Debug)]
pub enum NetworkResponse {
    #[response(status = 201)]
    Created(String),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 401)]
    Unauthorized(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 409)]
    Conflict(String),
    #[response(status = 500)]
    InternalServerError(String),
}

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    AuthToken(String),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RegisterResponse {
    pub body: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest<'r> {
    pub username: &'r str,
    pub password: &'r str,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegisterRequest<'r> {
    pub username: &'r str,
    pub email: &'r str,
    pub password: &'r str,
}
