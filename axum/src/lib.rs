use axum::{http::Method, Router};
use std::{error::Error, net::SocketAddr};
use tower_http::cors::{Any, CorsLayer};
mod auth;
mod manga;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let router = Router::new()
        .nest("/manga", manga::router())
        .nest("/auth", auth::router());

    let address = "0.0.0.0:3000".parse::<SocketAddr>()?;

    println!("Listening on {}", address);

    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
