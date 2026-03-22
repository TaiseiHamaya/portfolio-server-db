use crate::generated::proto_server::{
    PayloadLoginRequest, PayloadLoginResponse, PayloadLogoutRequest, PayloadLogoutResponse,
    PayloadSignupRequest, PayloadSignupResponse,
    record_session_service_server::RecordSessionService,
};

#[derive(Debug, Default)]
pub struct RecordSessionServiceImpl {}

#[tonic::async_trait]
impl RecordSessionService for RecordSessionServiceImpl {
    /// Login: authenticate player and create session.
    async fn login(
        &self,
        request: tonic::Request<PayloadLoginRequest>,
    ) -> std::result::Result<tonic::Response<PayloadLoginResponse>, tonic::Status> {
        Ok(tonic::Response::new(PayloadLoginResponse::default()))
    }
    /// Logout: clear session and perform cleanup.
    async fn logout(
        &self,
        request: tonic::Request<PayloadLogoutRequest>,
    ) -> std::result::Result<tonic::Response<PayloadLogoutResponse>, tonic::Status> {
        Ok(tonic::Response::new(PayloadLogoutResponse::default()))
    }
    /// Signup: create new player.
    async fn signup(
        &self,
        request: tonic::Request<PayloadSignupRequest>,
    ) -> std::result::Result<tonic::Response<PayloadSignupResponse>, tonic::Status> {
        Ok(tonic::Response::new(PayloadSignupResponse::default()))
    }
}
