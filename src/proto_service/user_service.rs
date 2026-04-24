use std::sync::{Arc, Mutex};

use crate::{
    generated::proto_server::{
        PayloadLoginRequest, PayloadLoginResponse, PayloadLogoutRequest, PayloadLogoutResponse,
        PayloadSignupRequest, PayloadSignupResponse, user_db_service_server::UserDbService,
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
        log::info!("Received login request.");

        // データの受け取り
        let Some(session_id) = request.into_inner().session_id else {
            log::error!("Login request missing session_id.");
            return Err(tonic::Status::invalid_argument("Missing session_id"));
        };

        let Ok(mut users) = self.user_imdb.lock() else {
            log::error!("Failed to acquire lock on user database.");
            return Err(tonic::Status::internal(
                "Failed to acquire lock on user database.",
            ));
        };

        // ログインチェック
        log::info!(
            "Authenticating user with session_id: {:016x}{:016x}",
            session_id.high,
            session_id.low
        );
        match users.auth_user(&session_id) {
            Some((user_id, session_id)) => {
                log::info!("User logged in successfully: {}", user_id);
                return Ok(tonic::Response::new(PayloadLoginResponse {
                    is_succeeded: true,
                    user_id,
                    session_id: Some(session_id),
                }));
            }
            None => {
                log::error!(
                    "Invalid session_id provided. Session ID: {:016x}{:016x}",
                    session_id.high,
                    session_id.low
                );
                return Err(tonic::Status::unauthenticated("Invalid session_id"));
            }
        }
    }
    /// Logout: clear session and perform cleanup.
    async fn logout(
        &self,
        request: tonic::Request<PayloadLogoutRequest>,
    ) -> std::result::Result<tonic::Response<PayloadLogoutResponse>, tonic::Status> {
        log::info!("Received logout request: {:?}", request);

        let Ok(mut users) = self.user_imdb.lock() else {
            log::error!("Failed to acquire lock on user database.");
            return Err(tonic::Status::internal(
                "Failed to acquire lock on user database.",
            ));
        };

        let user_id = request.into_inner().user_id;
        match users.logout_user(user_id) {
            Some(()) => {
                log::info!("User logged out successfully: {}", user_id);
                return Ok(tonic::Response::new(PayloadLogoutResponse {
                    is_succeeded: true,
                }));
            }
            None => {
                log::error!("User not found: {}", user_id);
                return Err(tonic::Status::not_found("User not found"));
            }
        }
    }
    /// Signup: create new player.
    async fn signup(
        &self,
        _request: tonic::Request<PayloadSignupRequest>,
    ) -> std::result::Result<tonic::Response<PayloadSignupResponse>, tonic::Status> {
        let Ok(mut users) = self.user_imdb.lock() else {
            log::error!("Failed to acquire lock on user database.");
            return Err(tonic::Status::internal(
                "Failed to acquire lock on user database.",
            ));
        };

        match users.create_user() {
            Ok((user_id, session_id)) => {
                log::info!(
                    "User created successfully: User ID: {}, Session ID: {:016x}{:016x}",
                    user_id,
                    session_id.high,
                    session_id.low
                );
                return Ok(tonic::Response::new(PayloadSignupResponse {
                    is_succeeded: true,
                    user_id,
                    session_id: Some(session_id),
                }));
            }
            Err(_) => {
                log::error!("Failed to create user.");
                return Err(tonic::Status::internal("Failed to create user."));
            }
        }
    }
}
