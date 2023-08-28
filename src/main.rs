extern crate dotenv;
use dotenv::dotenv;

mod routes;
mod services;

use crate::routes::send;
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let port = std::env::var("PORT")
        .expect("provide $PORT")
        .parse()
        .unwrap();

    let app = Router::new()
        .route("/", get(|| async { "server online" }))
        .route("/send", post(send::send_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
