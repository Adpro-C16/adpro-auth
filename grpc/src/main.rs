use crate::{auth::MyAuthService, services::auth_service_server::AuthServiceServer};
use dotenvy::dotenv;
use tonic::transport::Server;

pub mod services {
    tonic::include_proto!("services");
}
pub mod auth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let addr = "0.0.0.0:50051".parse().unwrap();
    let auth_service = MyAuthService::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(AuthServiceServer::new(auth_service))
        .serve(addr)
        .await?;

    Ok(())
}
