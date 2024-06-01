use axum::{response::Json, routing, Router};
use serde::{Deserialize, Serialize};
pub mod dto;
pub mod enums;
pub mod model;

pub mod controller;

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            routing::post(controller::create)
                .get(controller::get_all)
                .delete(controller::delete_all),
        )
        .route(
            "/:id",
            routing::delete(controller::delete_one)
                .patch(controller::update)
                .get(controller::get_one),
        )
}
