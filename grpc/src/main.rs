use crate::{
    auth::MyAuthService, services::auth_service_server::AuthServiceServer,
    services::user_service_server::UserServiceServer, user::MyUserService,
};
use autometrics::prometheus_exporter;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tonic::transport::Server;

pub mod services {
    tonic::include_proto!("services");
}
pub mod auth;
pub mod user;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    prometheus_exporter::init();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(&url)
        .await
        .unwrap();

    let addr = "0.0.0.0:8080".parse().unwrap();
    let auth_service = MyAuthService::default();
    let user_service = MyUserService { pool };

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(AuthServiceServer::new(auth_service))
        .add_service(UserServiceServer::new(user_service))
        .serve(addr)
        .await
        .expect("Failed to start server.");

    Ok(())
}
