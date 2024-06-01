use axum::{routing, Router};

pub fn router() -> Router {
    Router::new()
        .route("/login", routing::post(|| async { /* login */ }))
        .route("/signup", routing::post(|| async { /* sign up */ }))
        .route(
            "/forgot-password",
            routing::post(|| async { /* forgot password */ }),
        )
        .route(
            "/reset-password",
            routing::post(|| async { /* reset password */ }),
        )
        .route(
            "/change-password",
            routing::post(|| async { /* change password */ }),
        )
}
