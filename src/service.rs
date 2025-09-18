use crate::geyser;
use futures_core::Stream;
use futures_util::StreamExt;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct MyGeyserService {
    pub rx: Arc<Mutex<Option<mpsc::UnboundedReceiver<geyser::UpdateMessage>>>>,
}

#[tonic::async_trait]
impl geyser::geyser_service_server::GeyserService for MyGeyserService {
    type StreamUpdatesStream = Pin<
        Box<dyn Stream<Item = std::result::Result<geyser::UpdateMessage, Status>> + Send + 'static>,
    >;

    async fn stream_updates(
        &self,
        _req: Request<geyser::StreamRequest>,
    ) -> std::result::Result<Response<Self::StreamUpdatesStream>, Status> {
        let rx_lock = self.rx.clone();
        let base_stream = UnboundedReceiverStream::new(rx_lock.lock().await.take().unwrap());
        let stream = base_stream.map(|x| Ok(x));
        Ok(Response::new(Box::pin(stream) as Self::StreamUpdatesStream))
    }
}
