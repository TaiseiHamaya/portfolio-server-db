use crate::generated::proto_server::{
    PayloadPlayerCreateRequest, PayloadPlayerCreateResponse, PayloadPlayerLoadRequest,
    PayloadPlayerLoadResponse, PayloadPlayerRecord, PayloadPlayerSaveRequest,
    PayloadPlayerSaveResponse, record_player_db_service_server::RecordPlayerDbService,
};

use crate::db_helper::player_db::{PlayerDBHelper, PlayerRecord};

#[derive(Debug)]
pub struct RecordPlayerDbServiceImpl {
    record_imdb: PlayerDBHelper,
}

impl RecordPlayerDbServiceImpl {
    pub fn new(record_imdb: PlayerDBHelper) -> Self {
        Self { record_imdb }
    }
}

#[tonic::async_trait]
impl RecordPlayerDbService for RecordPlayerDbServiceImpl {
    /// World -> DB: create player and get player id.
    async fn create_player(
        &self,
        request: tonic::Request<PayloadPlayerCreateRequest>,
    ) -> std::result::Result<tonic::Response<PayloadPlayerCreateResponse>, tonic::Status> {
        log::info!(
            "Received create player request for user_id: {}",
            request.get_ref().user_id
        );

        let user_id = request.get_ref().user_id;
        match self
            .record_imdb
            .create_player_record(user_id, request.into_inner().username)
            .await
        {
            Some(()) => {
                log::info!(
                    "Player record created successfully for user_id: {}",
                    user_id
                );
                return Ok(tonic::Response::new(PayloadPlayerCreateResponse {
                    is_succeeded: true,
                }));
            }
            None => {
                // プレイヤーデータの作成に失敗（すでに同じユーザーIDのデータが存在する）
                log::error!("Player record already exists for user_id: {}", user_id);
                return Err(tonic::Status::already_exists(
                    "Player record already exists",
                ));
            }
        }
    }

    /// World → Db: load player data.
    async fn load_player(
        &self,
        request: tonic::Request<PayloadPlayerLoadRequest>,
    ) -> std::result::Result<tonic::Response<PayloadPlayerLoadResponse>, tonic::Status> {
        let user_id = request.get_ref().user_id;
        log::info!("Received load player request for user_id: {}", user_id);

        let Some(player) = self.record_imdb.load_player_record(user_id).await else {
            // ユーザーIDに対応するプレイヤーデータが見つからない
            return Err(tonic::Status::not_found("Player not found"));
        };

        log::info!("Player record loaded successfully for user_id: {}", user_id);
        Ok(tonic::Response::new(PayloadPlayerLoadResponse {
            is_succeeded: true,
            record: Some(PayloadPlayerRecord {
                user_id: user_id,
                username: player.user_name,
                zone_id: player.last_zone_id,
                position: player.last_position,
            }),
        }))
    }

    /// World → Db: save player data.
    async fn save_player(
        &self,
        request: tonic::Request<PayloadPlayerSaveRequest>,
    ) -> std::result::Result<tonic::Response<PayloadPlayerSaveResponse>, tonic::Status> {
        log::info!(
            "Received save player request: {:?}",
            request.get_ref().record
        );
        let request = request.into_inner();
        let Some(record) = request.record else {
            // 受け取ったリクエストにプレイヤーデータが添付されていない
            return Err(tonic::Status::invalid_argument("Missing player record"));
        };

        // リクエストからプレイヤーデータを構築
        let player_record = PlayerRecord::new(
            record.user_id,
            record.username,
            record.zone_id,
            record.position,
        );

        // プレイヤーデータを保存
        let user_id = match player_record.user_id.parse::<u64>() {
            Ok(id) => id,
            Err(_) => {
                log::error!(
                    "DB | Player | save_player | Invalid user_id format: {}",
                    player_record.user_id
                );
                return Err(tonic::Status::invalid_argument("Invalid user_id"));
            }
        };
        match self.record_imdb.save_player_record(player_record).await {
            Some(()) => {
                log::info!("Player record saved successfully for user_id: {}", user_id);
                Ok(tonic::Response::new(PayloadPlayerSaveResponse {
                    is_succeeded: true,
                }))
            }
            None => {
                log::error!("Failed to save player record for user_id: {}", user_id);
                return Err(tonic::Status::internal("Failed to save player record"));
            }
        }
    }
}
