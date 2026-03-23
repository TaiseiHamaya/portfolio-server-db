use std::sync::{Arc, Mutex};

use crate::{
    generated::proto_server::{
        PayloadLoginRequest, PayloadLoginResponse, PayloadLogoutRequest, PayloadLogoutResponse,
        PayloadSignupResponse, user_db_service_server::UserDbService,
    },
    imdb::user_db::UserImdb,
};

#[derive(Debug, Default)]
pub struct UserDbServiceImpl {
    user_imdb: Arc<Mutex<UserImdb>>,
}

#[tonic::async_trait]
impl UserDbService for UserDbServiceImpl {
    /// Login: authenticate player and create session.
    async fn login(
        &self,
        request: tonic::Request<PayloadLoginRequest>,
    ) -> std::result::Result<tonic::Response<PayloadLoginResponse>, tonic::Status> {
        // データの受け取り
        let Some(session_id) = request.into_inner().session_id else {
            return Err(tonic::Status::invalid_argument("Missing session_id"));
        };

        let Ok(mut users) = self.user_imdb.lock() else {
            return Err(tonic::Status::internal(
                "Failed to acquire lock on user database.",
            ));
        };

        // ログインチェック
        match users.auth_user(&session_id) {
            Some(session_id) => {
                return Ok(tonic::Response::new(PayloadLoginResponse {
                    is_succeeded: true,
                    session_id: Some(session_id),
                }));
            }
            None => {
                return Err(tonic::Status::unauthenticated("Invalid session_id"));
            }
        }
    }
    /// Logout: clear session and perform cleanup.
    async fn logout(
        &self,
        request: tonic::Request<PayloadLogoutRequest>,
    ) -> std::result::Result<tonic::Response<PayloadLogoutResponse>, tonic::Status> {
        let Ok(mut users) = self.user_imdb.lock() else {
            return Err(tonic::Status::internal(
                "Failed to acquire lock on user database.",
            ));
        };

        match users.logout_user(request.into_inner().user_id) {
            Some(()) => {
                return Ok(tonic::Response::new(PayloadLogoutResponse {
                    is_succeeded: true,
                }));
            }
            None => {
                return Err(tonic::Status::not_found("User not found"));
            }
        }
    }
    /// Signup: create new player.
    async fn signup(
        &self,
        _: tonic::Request<()>,
    ) -> std::result::Result<tonic::Response<PayloadSignupResponse>, tonic::Status> {
        let Ok(mut users) = self.user_imdb.lock() else {
            return Err(tonic::Status::internal(
                "Failed to acquire lock on user database.",
            ));
        };

        match users.create_user() {
            Ok(session_id) => {
                return Ok(tonic::Response::new(PayloadSignupResponse {
                    is_succeeded: true,
                    session_id: Some(session_id),
                }));
            }
            Err(_) => {
                return Err(tonic::Status::internal("Failed to create user."));
            }
        }
    }
}
