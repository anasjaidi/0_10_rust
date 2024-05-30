use std::net::SocketAddr;

use axum::Router;

#[tokio::main]
async fn main() {
    let router = Router::new().route(
        "/",
        axum::routing::get(|| async { axum::response::Html("anas jaidi") }),
    );

    let address = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("{}", address);
    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
