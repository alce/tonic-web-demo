use std::pin::Pin;

use futures_util::TryStreamExt;
use tokio_stream::Stream;
use tokio::sync::broadcast::{self, RecvError};
use tonic::{Request, Response, Status};

use crate::pb::audit_log_service_server::{AuditLogService, AuditLogServiceServer};
use crate::pb::{Event, SubscribeRequest};

pub struct AuditLog {
    sender: broadcast::Sender<Event>,
}

#[tonic::async_trait]
impl AuditLogService for AuditLog {
    type SubscribeStream = Pin<Box<dyn Stream<Item = Result<Event, Status>> + Sync + Send>>;

    async fn subscribe(
        &self,
        _req: Request<SubscribeRequest>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let stream = self
            .sender
            .subscribe()
            .into_stream()
            .map_err(internal_error);

        Ok(Response::new(Box::pin(stream)))
    }
}

fn internal_error(e: RecvError) -> Status {
    Status::internal(format!("{:?}", e))
}

pub fn service(sender: broadcast::Sender<Event>) -> AuditLogServiceServer<AuditLog> {
    AuditLogServiceServer::new(AuditLog { sender })
}
