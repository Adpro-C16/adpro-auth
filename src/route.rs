use argon2::Config;
use rocket::{serde::json::Json, State};
use sqlx::{Pool, Postgres};
use std::env;
use uuid::Uuid;

use crate::{
    guard::{create_jwt, JWT},
    model::{Role, User},
    typing::{
        LoginRequest, NetworkResponse, RegisterRequest, RegisterResponse, Response, ResponseBody,
    },
};

#[post("/login", data = "<user>", format = "application/json")]
pub async fn login(
    pool: &State<Pool<Postgres>>,
    user: Json<LoginRequest<'_>>,
) -> Result<String, NetworkResponse> {
    let result = match sqlx::query!(
        "SELECT id, role, password FROM users WHERE username = $1",
        user.username
    )
    .fetch_optional(&pool.inner().clone())
    .await
    {
        Ok(result) => {
            if result.is_none() {
                return Err(NetworkResponse::BadRequest(
                    "Username / Email does not exists.".to_string(),
                ));
            }
            result.unwrap()
        }
        Err(_) => return Err(NetworkResponse::BadRequest("Database error.".to_string())),
    };
    let matching = argon2::verify_encoded(&result.password, user.password.as_bytes()).unwrap();
    if !matching {
        return Err(NetworkResponse::Unauthorized(
            "Wrong username / password.".to_string(),
        ));
    }
    let token = create_jwt(result.id.to_string(), result.role.to_string()).unwrap();
    let response = Response {
        body: ResponseBody::AuthToken(token),
    };
    return Ok(serde_json::to_string(&response).unwrap());
}

#[post("/register", data = "<data>", format = "application/json")]
pub async fn register(
    pool: &State<Pool<Postgres>>,
    data: Json<RegisterRequest<'_>>,
) -> Result<String, NetworkResponse> {
    let result = match sqlx::query!(
        "SELECT email FROM users WHERE email = $1 OR username = $2",
        data.email,
        data.username
    )
    .fetch_optional(&pool.inner().clone())
    .await
    {
        Ok(result) => result,
        Err(_) => return Err(NetworkResponse::BadRequest("Database error.".to_string())),
    };
    if result.is_some() {
        return Err(NetworkResponse::Conflict(
            "Username / Email already exists.".to_string(),
        ));
    }
    let config = Config::default();
    let salt = env::var("SALT").unwrap();
    let hash = argon2::hash_encoded(data.password.as_bytes(), salt.as_bytes(), &config).unwrap();
    let _ = match sqlx::query!(
        "INSERT INTO users (id, username, email, password, role) VALUES ($1, $2, $3, $4, $5)",
        Uuid::new_v4(),
        data.username,
        data.email,
        hash,
        Role::User.to_string()
    )
    .execute(&pool.inner().clone())
    .await
    {
        Ok(_) => {
            let response = RegisterResponse {
                body: "Registration Successfull".to_string(),
            };
            return Ok(serde_json::to_string(&response).unwrap());
        }
        Err(_) => return Err(NetworkResponse::BadRequest("Database error.".to_string())),
    };
}

#[post("/verify")]
pub async fn authorize(
    pool: &State<Pool<Postgres>>,
    key: Result<JWT, NetworkResponse>,
) -> Result<String, NetworkResponse> {
    let key = key?;
    match sqlx::query!(
        "SELECT id FROM users WHERE id = $1",
        Uuid::parse_str(&key.claims.id).unwrap()
    )
    .fetch_one(&pool.inner().clone())
    .await
    {
        Ok(_) => return Ok("Authorized".to_string()),
        Err(_) => return Err(NetworkResponse::BadRequest("Database error.".to_string())),
    };
}

#[get("/user")]
pub async fn get_user(
    pool: &State<Pool<Postgres>>,
    key: Result<JWT, NetworkResponse>,
) -> Result<String, NetworkResponse> {
    let key = key?;
    let result = match sqlx::query!(
        "SELECT id, username, email, role FROM users WHERE id = $1",
        Uuid::parse_str(&key.claims.id).unwrap()
    )
    .fetch_one(&pool.inner().clone())
    .await
    {
        Ok(result) => result,
        Err(_) => return Err(NetworkResponse::BadRequest("Database error.".to_string())),
    };
    let user = User {
        id: result.id.to_string(),
        username: result.username,
        email: result.email,
        role: Role::User,
    };
    let response = serde_json::to_string(&user).unwrap();
    return Ok(serde_json::to_string(&response).unwrap());
}
