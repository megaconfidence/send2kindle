#[macro_use]
extern crate dotenv_codegen;

mod routes;
mod services;

use axum::{
    Router,
    routing::{get, post},
};
use crate::routes::send;
use std::net::SocketAddr;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(|| async { "server online" }))
        .route("/send", post(send::send_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], dotenv!("PORT").parse().unwrap()));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
