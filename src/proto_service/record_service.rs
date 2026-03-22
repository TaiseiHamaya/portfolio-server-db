use crate::generated::proto_server::{
    PayloadPlayerCreateRequest, PayloadPlayerCreateResponse, PayloadPlayerLoadRequest,
    PayloadPlayerLoadResponse, PayloadPlayerSaveRequest, PayloadPlayerSaveResponse,
    record_player_db_service_server::RecordPlayerDbService,
};

#[derive(Debug, Default)]
pub struct RecordPlayerDbServiceImpl {}

#[tonic::async_trait]
impl RecordPlayerDbService for RecordPlayerDbServiceImpl {
    /// World -> DB: create player and get player id.
    async fn create_player(
        &self,
        request: tonic::Request<PayloadPlayerCreateRequest>,
    ) -> std::result::Result<tonic::Response<PayloadPlayerCreateResponse>, tonic::Status> {
        Ok(tonic::Response::new(PayloadPlayerCreateResponse::default()))
    }
    /// World → Db: load player data.
    async fn load_player(
        &self,
        request: tonic::Request<PayloadPlayerLoadRequest>,
    ) -> std::result::Result<tonic::Response<PayloadPlayerLoadResponse>, tonic::Status> {
        Ok(tonic::Response::new(PayloadPlayerLoadResponse::default()))
    }
    /// World → Db: save player data.
    async fn save_player(
        &self,
        request: tonic::Request<PayloadPlayerSaveRequest>,
    ) -> std::result::Result<tonic::Response<PayloadPlayerSaveResponse>, tonic::Status> {
        Ok(tonic::Response::new(PayloadPlayerSaveResponse::default()))
    }
}
