use axum::{Router, middleware as axum_middleware};
use std::net::SocketAddr;

mod controllers;
mod db;
mod middleware;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let pool = db::create_pool().await;
    println!("🚀 Database connection established");

    let app = Router::new()
        .nest("/", routes::api::api_routes())
        .layer(axum_middleware::from_fn(middleware::logger::log_requests))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
