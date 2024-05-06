use crate::services::{
    auth_service_server::AuthService, ClaimsRequest, ClaimsResponse, RbacRequest, RbacResponse,
    VerifyRequest, VerifyResponse,
};
use shared::{decode_jwt, verify_token};

#[derive(Default)]
pub struct MyAuthService;

#[tonic::async_trait]
impl AuthService for MyAuthService {
    async fn verify_auth(
        &self,
        req: tonic::Request<VerifyRequest>,
    ) -> Result<tonic::Response<VerifyResponse>, tonic::Status> {
        match verify_token(req.into_inner().token.as_str()) {
            Ok(_) => Ok(tonic::Response::new(VerifyResponse { is_valid: true })),
            Err(_) => Err(tonic::Status::unauthenticated("Invalid Token".to_string())),
        }
    }

    async fn verify_access(
        &self,
        req: tonic::Request<RbacRequest>,
    ) -> Result<tonic::Response<RbacResponse>, tonic::Status> {
        let payload = req.into_inner();
        let token = payload.token;
        let role = payload.role;
        match decode_jwt(token.as_str()) {
            Ok(c) => {
                if c.role == role {
                    Ok(tonic::Response::new(RbacResponse { is_valid: true }))
                } else {
                    Err(tonic::Status::permission_denied(
                        "Permission Denied".to_string(),
                    ))
                }
            }
            Err(_) => Err(tonic::Status::unauthenticated("Invalid Token".to_string())),
        }
    }

    async fn get_claims(
        &self,
        req: tonic::Request<ClaimsRequest>,
    ) -> Result<tonic::Response<ClaimsResponse>, tonic::Status> {
        match decode_jwt(req.into_inner().token.as_str()) {
            Ok(c) => Ok(tonic::Response::new(ClaimsResponse {
                role: c.role,
                user_id: c.id,
            })),
            Err(_) => Err(tonic::Status::unauthenticated("Invalid Token".to_string())),
        }
    }
}
