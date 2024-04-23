use rocket::State;
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
) -> Result<String, NetworkResponse> {
    let key = key?;
    let result = match sqlx::query!(
        "SELECT id, username, email, role FROM users WHERE id = $1",
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
    };
    let response = serde_json::to_string(&user).unwrap();
    return Ok(serde_json::to_string(&response).unwrap());
}

#[get("/")]
pub fn index() -> &'static str {
    "Heymart C14 - Auth Service"
}
