use std::{error::Error, net::SocketAddr};

use axum::{
    response::{Html, IntoResponse, Response},
    routing, Router, Server,
};

async fn route_handler() -> Response {
    Html("Hello, World!").into_response()
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let router = Router::new().route("/", routing::get(route_handler));

    let addr = "0.0.0.0:3000".parse::<SocketAddr>()?;

    let address = "0.0.0.0:3000".parse::<SocketAddr>()?;

    println!("server running on: {}", addr);

    Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
