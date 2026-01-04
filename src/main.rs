mod domain;
mod http;
mod repository;
mod service;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::serve;
use http::router;
use repository::InMemoryFlagRepository;
use service::FlagService;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Set up our dependencies - we keep everything explicit so we can manage ownership cleanly
    let repository = InMemoryFlagRepository::new();
    let service = Arc::new(FlagService::new(repository));

    let app = router(service);

    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();
    println!("feature flag service listening on http://{}", addr);

    let listener = TcpListener::bind(&addr)
        .await
        .expect("failed to bind");
    
    serve(listener, app)
        .await
        .expect("server failed");
}
