use crate::pb::{Action, Event};
use crate::products::ProductEvent;
use tokio::sync::{broadcast, mpsc};
use tonic::transport::Server;
use uuid::Uuid;

mod pb;

mod audit_log;
mod products;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (btx, _) = broadcast::channel(2);

    let (tx, mut rx) = mpsc::channel::<ProductEvent>(2);

    let btx2 = btx.clone();

    tokio::spawn(async move {
        while let Some(evt) = rx.recv().await {
            let mut event = Event {
                id: Uuid::new_v4().to_string(),
                ..Default::default()
            };

            match evt {
                ProductEvent::Created(product) => {
                    event.action = Action::Created as i32;
                    event.product = Some(product);
                }

                ProductEvent::Deleted(id) => {
                    event.action = Action::Deleted as i32;
                    event.product_id = id.to_string();
                }
            }

            if let Err(e) = btx2.send(event) {
                println!("error broadcasting: {:?}", e)
            }
        }
    });

    Server::builder()
        .add_service(audit_log::service(btx))
        .add_service(products::service(tx))
        .serve(([127, 0, 0, 1], 9999).into())
        .await
        .map_err(Into::into)
}