use std::net::SocketAddr;

use axum::{
    response::{Html, IntoResponse},
    Router,
};

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", axum::routing::get(handle_hello));

    let address = "0.0.0.0:3000".parse::<SocketAddr>().unwrap();

    println!("Listning on {}", address);
    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn handle_hello() -> impl IntoResponse {
    Html("Hello, World!")
}
