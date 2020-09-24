use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::sync::mpsc;
use tonic::{Request, Response, Status};
use tonic_grpc_web::GrpcWeb;

use crate::pb::product_service_server::{ProductService, ProductServiceServer};
use crate::pb::{
    CreateProductRequest, DeleteProductRequest, Empty, ListProductsRequest, ListProductsResponse,
    Product,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum ProductEvent {
    Created(Product),
    Deleted(Uuid),
}

pub struct Products {
    db: Arc<Mutex<HashMap<Uuid, Product>>>,
    tx: mpsc::Sender<ProductEvent>,
}

#[tonic::async_trait]
impl ProductService for Products {
    async fn list_products(
        &self,
        _req: Request<ListProductsRequest>,
    ) -> Result<Response<ListProductsResponse>, Status> {
        let db = self.db.lock().unwrap();
        let mut products = db.values().cloned().collect::<Vec<_>>();
        products.sort_by_key(|p| p.name.clone());

        Ok(Response::new(ListProductsResponse { products }))
    }

    async fn create_product(
        &self,
        req: Request<CreateProductRequest>,
    ) -> Result<Response<Product>, Status> {
        let product = {
            let mut db = self.db.lock().unwrap();
            let id = Uuid::new_v4();

            let product = Product {
                id: id.to_string(),
                name: req.into_inner().name,
            };

            db.insert(id, product.clone());

            product
        };

        let mut tx = self.tx.clone();
        let event = ProductEvent::Created(product.clone());
        tx.send(event).await.unwrap();

        Ok(Response::new(product))
    }

    async fn delete_product(
        &self,
        req: Request<DeleteProductRequest>,
    ) -> Result<Response<Empty>, Status> {
        let id = Uuid::parse_str(&req.get_ref().id).unwrap();

        {
            let mut db = self.db.lock().unwrap();
            db.remove(&id);
        }

        let event = ProductEvent::Deleted(id);
        let mut tx = self.tx.clone();
        tx.send(event).await.unwrap();

        Ok(Response::new(Empty {}))
    }
}

pub fn service(tx: mpsc::Sender<ProductEvent>) -> GrpcWeb<ProductServiceServer<Products>> {
    let mut db = HashMap::new();

    let id = Uuid::new_v4();
    db.insert(
        id,
        Product {
            id: id.to_string(),
            name: "Widgets".into(),
        },
    );

    let id = Uuid::new_v4();
    db.insert(
        id,
        Product {
            id: id.to_string(),
            name: "Gadgets".into(),
        },
    );

    GrpcWeb::new(ProductServiceServer::new(Products {
        db: Arc::new(Mutex::new(db)),
        tx,
    }))
}
