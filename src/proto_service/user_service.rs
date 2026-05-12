use crate::{
    db_helper::user_db::UserDBHelper,
    generated::proto_server::{
        PayloadLoginRequest, PayloadLoginResponse, PayloadLogoutRequest, PayloadLogoutResponse,
        PayloadSignupRequest, PayloadSignupResponse, SessionId,
        user_db_service_server::UserDbService,
    },
};

#[derive(Debug)]
pub struct UserDbServiceImpl {
    user_imdb: UserDBHelper,
}

impl UserDbServiceImpl {
    pub fn new(user_imdb: UserDBHelper) -> Self {
        Self { user_imdb }
    }
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

        // ログインチェック
        log::info!(
            "Authenticating user with session_id: {:016x}{:016x}",
            session_id.high,
            session_id.low
        );
        let session_id_bytes = match session_id
            .high
            .to_be_bytes()
            .iter()
            .chain(session_id.low.to_be_bytes().iter())
            .cloned()
            .collect::<Vec<u8>>()
            .try_into()
        {
            Ok(bytes) => bytes,
            Err(_) => {
                log::error!("Failed to convert session_id to bytes.");
                return Err(tonic::Status::invalid_argument("Invalid session_id format"));
            }
        };
        match self.user_imdb.auth_user(session_id_bytes).await {
            Some((user_id, session_id)) => {
                log::info!("User logged in successfully: {}", user_id);
                return Ok(tonic::Response::new(PayloadLoginResponse {
                    is_succeeded: true,
                    user_id,
                    session_id: Some(SessionId {
                        high: u64::from_be_bytes(session_id[0..8].try_into().unwrap_or_default()),
                        low: u64::from_be_bytes(session_id[8..16].try_into().unwrap_or_default()),
                    }),
                }));
            }
            None => {
                log::error!(
                    "Invalid session_id provided. {:016x}{:#016x}",
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

        let user_id = request.into_inner().user_id;
        match self.user_imdb.logout_user(user_id).await {
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
        match self.user_imdb.create_user().await {
            Some((user_id, session_id_byte)) => {
                let session_id = SessionId {
                    high: u64::from_be_bytes(session_id_byte[0..8].try_into().unwrap_or_default()),
                    low: u64::from_be_bytes(session_id_byte[8..16].try_into().unwrap_or_default()),
                };
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
            None => {
                log::error!("Failed to create user.");
                return Err(tonic::Status::internal("Failed to create user."));
            }
        }
    }
}
