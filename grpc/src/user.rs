use autometrics::autometrics;
use shared::verify_token;
use tonic::{Request, Response, Status};

use crate::services::{
    user_service_server::UserService, UpdateBalanceRequest, UpdateBalanceResponse,
};

pub struct MyUserService {
    pub pool: sqlx::PgPool,
}

#[tonic::async_trait]
#[autometrics]
impl UserService for MyUserService {
    async fn update_balance(
        &self,
        req: Request<UpdateBalanceRequest>,
    ) -> Result<Response<UpdateBalanceResponse>, Status> {
        let body = req.into_inner();
        let claims = match verify_token(&body.token) {
            Ok(result) => result,
            Err(_) => return Err(Status::unauthenticated("Invalid Token".to_string())),
        };

        if body.amount < 0 {
            return Err(Status::invalid_argument("Invalid amount.".to_string()));
        }

        match body.transaction_type {
            0 => {
                let user = sqlx::query!("SELECT balance FROM users WHERE id = $1", claims.id)
                    .fetch_one(&self.pool.clone())
                    .await
                    .unwrap();
                if user.balance.unwrap_or(0) < body.amount {
                    return Err(Status::invalid_argument(
                        "Insufficient balance.".to_string(),
                    ));
                }
                let result = match sqlx::query!(
                    "UPDATE users SET balance = balance - $1 WHERE id = $2 RETURNING balance",
                    body.amount,
                    &claims.id
                )
                .fetch_one(&self.pool.clone())
                .await
                {
                    Ok(result) => result,
                    Err(_) => return Err(Status::internal("Database error.".to_string())),
                };
                return Ok(Response::new(UpdateBalanceResponse {
                    success: true,
                    new_balance: result.balance.unwrap_or_else(|| 0),
                }));
            }
            1 => {
                // Add balance
                let result = match sqlx::query!(
                    "UPDATE users SET balance = balance + $1 WHERE id = $2 RETURNING balance",
                    body.amount,
                    &claims.id
                )
                .fetch_one(&self.pool.clone())
                .await
                {
                    Ok(result) => result,
                    Err(_) => return Err(Status::internal("Database error.".to_string())),
                };
                return Ok(Response::new(UpdateBalanceResponse {
                    success: true,
                    new_balance: result.balance.unwrap_or_else(|| 0),
                }));
            }
            _ => {
                return Err(Status::invalid_argument("Invalid type.".to_string()));
            }
        }
    }
}
