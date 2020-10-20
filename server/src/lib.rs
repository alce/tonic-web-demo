use std::time::SystemTime;

use prost_types::Timestamp;
use tokio::sync::{broadcast, mpsc};
use tonic::transport::Server;
use tracing::{error, info};
use uuid::Uuid;

use crate::pb::{Action, Event};
use crate::products::ProductEvent;

mod audit_log;
mod pb;
mod products;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (btx, _) = broadcast::channel(2);

    let (tx, mut rx) = mpsc::channel::<ProductEvent>(2);

    let btx2 = btx.clone();

    tokio::spawn(async move {
        while let Some(evt) = rx.recv().await {
            let mut event = Event {
                id: Uuid::new_v4().to_string(),
                create_time: Some(Timestamp::from(SystemTime::now())),
                ..Default::default()
            };

            match evt {
                ProductEvent::Created(product, user) => {
                    event.action = Action::Created as i32;
                    event.product = Some(product);
                    event.user = user;
                }

                ProductEvent::Deleted(id, user) => {
                    event.action = Action::Deleted as i32;
                    event.product_id = id.to_string();
                    event.user = user;
                }
            }

            if let Err(e) = btx2.send(event) {
                error!("failed to broadcast event: {:?}", e)
            }
        }
    });

    let addr = ([127, 0, 0, 1], 9999).into();

    let config = tonic_web::config().allow_origins(vec!["http://localhost:3000"]);
    let audit_log = config.enable(audit_log::service(btx));
    let products = config.enable(products::service(tx));

    info!("listening on {}", addr);

    Server::builder()
        .accept_http1(true)
        .add_service(audit_log)
        .add_service(products)
        .serve(addr)
        .await
        .map_err(Into::into)
}
