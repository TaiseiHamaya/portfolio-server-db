use std::sync::{Arc, Mutex};

use crate::generated::proto_server::{
    PayloadPlayerCreateRequest, PayloadPlayerCreateResponse, PayloadPlayerLoadRequest,
    PayloadPlayerLoadResponse, PayloadPlayerRecord, PayloadPlayerSaveRequest,
    PayloadPlayerSaveResponse, record_player_db_service_server::RecordPlayerDbService,
};

use crate::imdb::record_db::{PlayerRecord, RecordImdb};

#[derive(Debug, Default)]
pub struct RecordPlayerDbServiceImpl {
    record_imdb: Arc<Mutex<RecordImdb>>,
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
        let Ok(mut records) = self.record_imdb.lock() else {
            // RecordImdbのロックが取得できない
            return Err(tonic::Status::internal(
                "Failed to acquire lock on RecordImdb",
            ));
        };

        let user_id = request.get_ref().user_id;
        match records.create_player_record(user_id, request.into_inner().username) {
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
        log::info!(
            "Received load player request for user_id: {}",
            request.get_ref().user_id
        );

        let Ok(records) = self.record_imdb.lock() else {
            // RecordImdbのロックが取得できない
            return Err(tonic::Status::internal(
                "Failed to acquire lock on RecordImdb",
            ));
        };

        let Some(player) = records.load_player_record(request.into_inner().user_id) else {
            // ユーザーIDに対応するプレイヤーデータが見つからない
            return Err(tonic::Status::not_found("Player not found"));
        };

        log::info!(
            "Player record loaded successfully for user_id: {}",
            player.user_id
        );
        Ok(tonic::Response::new(PayloadPlayerLoadResponse {
            is_succeeded: true,
            record: Some(PayloadPlayerRecord {
                user_id: player.user_id,
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
        log::info!("Received save player request: {:?}", request);
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

        // RecordImdbのロックを取得してプレイヤーデータを保存
        let Ok(mut records) = self.record_imdb.lock() else {
            // RecordImdbのロックが取得できない
            return Err(tonic::Status::internal(
                "Failed to acquire lock on RecordImdb",
            ));
        };

        // プレイヤーデータを保存
        let user_id = player_record.user_id;
        match records.save_player_record(player_record) {
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
