extern crate dotenv;
use dotenv::dotenv;

mod routes;
mod services;

use crate::routes::send;
use axum::{routing::post, Router};
use axum_client_ip::SecureClientIpSource;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::{services::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .compact()
        .init();

    let port = std::env::var("PORT")
        .expect("provide $PORT")
        .parse()
        .unwrap();

    let app = Router::new()
        .nest_service("/", ServeDir::new("public"))
        .route("/send", post(send::send_handler))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .layer(SecureClientIpSource::ConnectInfo.into_extension());

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
