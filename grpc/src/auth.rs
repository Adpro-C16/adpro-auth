use crate::services::{auth_service_server::AuthService, VerifyRequest, VerifyResponse};
use shared::verify_token;

#[derive(Default)]
pub struct MyAuthService;

#[tonic::async_trait]
impl AuthService for MyAuthService {
    async fn verify(
        &self,
        req: tonic::Request<VerifyRequest>,
    ) -> Result<tonic::Response<VerifyResponse>, tonic::Status> {
        match verify_token(req.into_inner().token.as_str()) {
            Ok(_) => Ok(tonic::Response::new(VerifyResponse { is_valid: true })),
            Err(_) => Err(tonic::Status::unauthenticated("Invalid Token".to_string())),
        }
    }
}
