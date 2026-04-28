use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tonic::transport::Server;

mod ec2_helper;
mod etcd_client_helper;
mod generated;
mod imdb;
mod logger;
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
    logger::init().expect("Failed to initialize logger.");

    if dotenvy::dotenv().is_ok() {
        log::info!(".env file loaded successfully.");
    }

    let server_address = ec2_helper::get_local_ip().await;
    let port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "50050".into())
        .parse()
        .unwrap_or(50050);
    let server_endpoint = SocketAddr::new(IpAddr::V4(server_address), port);

    let etcd_client = etcd_client_helper::create_etcd_client().await;
    etcd_client_helper::register_service_endpoint(
        etcd_client.clone(),
        server_endpoint,
        "db".to_string(),
    )
    .await;

    // Tonicサーバの起動
    let service_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port);
    log::info!("Starting gRPC server on {}", service_address);
    // サービスの実装
    let session_service = UserDbServiceImpl::default();
    let record_service = RecordPlayerDbServiceImpl::default();
    tokio::spawn(async move {
        // サーバの構築と起動
        Server::builder()
            .add_service(UserDbServiceServer::new(session_service))
            .add_service(RecordPlayerDbServiceServer::new(record_service))
            .serve(service_address)
            .await
            .expect("Failed to start gRPC server");
    });

    log::info!(
        "DB server was initialized and is listening on {}",
        server_endpoint
    );

    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for shutdown signal");
    log::info!("Shutting down gracefuly...");
}
