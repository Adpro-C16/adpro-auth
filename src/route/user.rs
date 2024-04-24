use rocket::{serde::json::Json, State};
use sqlx::{Pool, Postgres};

use crate::{
    guard::JWT,
    model::user::{Role, User},
    typing::NetworkResponse,
};

#[derive(serde::Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct BalanceRequest {
    amount: i32,
}

#[get("/")]
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
        role: Role::User,
        balance: result.balance.unwrap_or(0),
    };
    return Ok(Json(user));
}

#[post("/balance", data = "<balance>")]
pub async fn update_balance(
    pool: &State<Pool<Postgres>>,
    key: Result<JWT, NetworkResponse>,
    balance: Json<BalanceRequest>,
) -> Result<Json<User>, NetworkResponse> {
    let key = key?;
    let amount = match balance.amount {
        amount if amount < 0 => {
            return Err(NetworkResponse::BadRequest("Invalid amount.".to_string()))
        }
        _ => balance.amount,
    };
    let user = sqlx::query!("SELECT balance FROM users WHERE id = $1", &key.claims.id)
        .fetch_one(&pool.inner().clone())
        .await
        .unwrap();
    if user.balance.unwrap_or(0) < amount {
        return Err(NetworkResponse::BadRequest(
            "Insufficient balance.".to_string(),
        ));
    }
    let result = match sqlx::query!(
        "UPDATE users SET balance = balance - $1 WHERE id = $2 RETURNING id, username, email, role, balance",
        amount,
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

#[post("/topup", data = "<balance>")]
pub async fn topup_balance(
    pool: &State<Pool<Postgres>>,
    key: Result<JWT, NetworkResponse>,
    balance: Json<BalanceRequest>,
) -> Result<Json<User>, NetworkResponse> {
    let key = key?;
    let amount = match balance.amount {
        amount if amount < 0 => {
            return Err(NetworkResponse::BadRequest("Invalid amount.".to_string()))
        }
        _ => balance.amount,
    };
    let result = match sqlx::query!(
        "UPDATE users SET balance = balance + $1 WHERE id = $2 RETURNING id, username, email, role, balance",
        amount,
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
