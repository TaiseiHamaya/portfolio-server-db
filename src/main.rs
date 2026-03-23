use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tonic::transport::Server;

mod generated;
mod imdb;
mod proto_service;

use crate::generated::proto_server::{
    record_player_db_service_server::RecordPlayerDbServiceServer,
    user_db_service_server::UserDbServiceServer,
};
use crate::proto_service::{
    record_service::RecordPlayerDbServiceImpl, user_service::UserDbServiceImpl,
};

#[tokio::main]
async fn main() {
    // Tonicサーバの起動
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 50051);

    // サービスの実装
    let session_service = UserDbServiceImpl::default();
    let record_service = RecordPlayerDbServiceImpl::default();

    // サーバの構築と起動
    Server::builder()
        .add_service(UserDbServiceServer::new(session_service))
        .add_service(RecordPlayerDbServiceServer::new(record_service))
        .serve(addr)
        .await
        .expect("Failed to start gRPC server");
}
