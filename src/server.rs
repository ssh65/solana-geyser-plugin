use crate::geyser;
use crate::service::MyGeyserService;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tonic::transport::Server;

pub async fn start_grpc_server(rx: mpsc::UnboundedReceiver<geyser::UpdateMessage>) {
    let addr = "127.0.0.1:9090".parse().unwrap();
    let service = MyGeyserService {
        rx: Arc::new(Mutex::new(Some(rx))),
    };

    Server::builder()
        .add_service(geyser::geyser_service_server::GeyserServiceServer::new(
            service,
        ))
        .serve(addr)
        .await
        .unwrap();
}
