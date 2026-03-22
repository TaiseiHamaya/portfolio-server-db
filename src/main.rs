use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;
use tonic::{Request, Response, Status, service::LayerExt as _, transport::Server};

mod generated;
mod imdb;
mod proto_service;

use crate::generated::proto_server;
use crate::imdb::*;

#[tokio::main]
async fn main() {
    // DBの初期化
    let session_imdb = session_db::SessionImdb::new();
    let record_imdb = record_db::RecordImdb::new();

    // Tonicサーバの起動
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 50051);
}
