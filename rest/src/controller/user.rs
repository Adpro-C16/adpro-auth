use autometrics::autometrics;
use rocket::{serde::json::Json, State};
use sqlx::{Pool, Postgres};

use crate::model::user::{Role, User};
use shared::typing::NetworkResponse;
use shared::JWT;

#[get("/")]
#[autometrics]
pub async fn get_user(
    pool: &State<Pool<Postgres>>,
    key: Result<JWT, NetworkResponse>,
) -> Result<Json<User>, NetworkResponse> {
    let key = key?;
    let result = match sqlx::query!(
        "SELECT id, username, email, role, balance FROM users WHERE id = $1",
        &key.claims.id
    )
    .fetch_one(&pool.inner().clone())
    .await
    {
        Ok(result) => result,
        Err(_) => return Err(NetworkResponse::BadRequest("Database error.".to_string())),
    };
    let user = User {
        id: result.id,
        username: result.username,
        email: result.email,
        role: Role::from(result.role.as_str()),
        balance: result.balance.unwrap_or(0),
    };
    return Ok(Json(user));
}

#[derive(serde::Deserialize)]
struct TopupDTO {
    amount: i32,
}

#[post("/topup", data = "<body>")]
#[autometrics]
pub async fn topup(
    pool: &State<Pool<Postgres>>,
    key: Result<JWT, NetworkResponse>,
    body: Json<TopupDTO>,
) -> Result<Json<User>, NetworkResponse> {
    let key = key?;
    if body.amount <= 0 {
        return Err(NetworkResponse::BadRequest("Invalid amount.".to_string()));
    }
    let result = match sqlx::query!(
        "UPDATE users SET balance = balance + $1 WHERE id = $2 RETURNING id, username, email, role, balance",
        body.amount,
        &key.claims.id
    )
    .fetch_one(&pool.inner().clone())
    .await
    {
        Ok(result) => result,
        Err(_) => return Err(NetworkResponse::BadRequest("Database error.".to_string())),
    };
    let user = User {
        id: result.id,
        username: result.username,
        email: result.email,
        role: Role::User,
        balance: result.balance.unwrap_or(0),
    };
    return Ok(Json(user));
}
