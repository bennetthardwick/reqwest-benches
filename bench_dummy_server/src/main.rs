use std::convert::Infallible;

use axum::{body::Body, response::IntoResponse, routing::get, Router};
use bytes::Bytes;
use futures::{stream::repeat, StreamExt};
use tokio::net::TcpListener;

const ONES: &[u8] = &[1; 8 * 1024];

async fn handler() -> impl IntoResponse {
    Body::from_stream(
        repeat(Bytes::from_static(ONES))
            // buffer is 8K * 1024 * 1024 * 2 = 16G
            .take(2 * 1024 * 1024)
            .map(Ok::<_, Infallible>),
    )
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind");

    eprintln!("Server running on 3000");

    axum::serve(listener, Router::new().route("/", get(handler)))
        .await
        .expect("failed to serve");
}
