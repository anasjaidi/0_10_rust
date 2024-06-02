use std::{error::Error, net::SocketAddr};

use axum::{
    extract::Query,
    response::{Html, IntoResponse, Response},
    routing, Router, Server,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Args {
    name: Option<String>,
}

async fn route_handler(Query(data): Query<Args>) -> Response {
    println!("{:?}", data);
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
