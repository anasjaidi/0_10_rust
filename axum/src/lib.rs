use axum::{http::Method, Extension, Router};
use std::{error::Error, net::SocketAddr};
use tower_http::cors::{AllowMethods, Any, CorsLayer};
mod auth;
mod manga;

#[derive(Default, Clone)]
pub struct Shared {
    pub database: (),
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    // FIXME: add cors
    let _cors = CorsLayer::new().allow_origin(Any);

    let shared_state = Shared::default();

    let router = Router::new()
        .nest("/manga", manga::router())
        .nest("/auth", auth::router())
        .layer(Extension(shared_state));

    let address = "0.0.0.0:3000".parse::<SocketAddr>()?;

    println!("Listening on {}", address);

    axum::Server::bind(&address)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
