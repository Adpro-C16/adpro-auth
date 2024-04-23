use rocket::{serde::json::Json, State};
use sqlx::{Pool, Postgres};

use crate::{
    guard::JWT,
    model::user::{Role, User},
    typing::NetworkResponse,
};

#[get("/user")]
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

#[get("/")]
pub fn index() -> &'static str {
    "Heymart C14 - Auth Service"
}
