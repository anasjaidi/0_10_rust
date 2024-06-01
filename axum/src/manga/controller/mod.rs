use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};

use super::dto::{Create, Update};

pub async fn create(
    Extension(header): Extension<String>,
    Json(dto): Json<Create>,
) -> impl IntoResponse {
    println!("{}", header);
    Json(dto)
}

pub async fn get_all() -> impl IntoResponse {
    // Placeholder: Return an empty list for now
}

pub async fn get_one(Path(uid): Path<String>) -> impl IntoResponse {
    Json(uid)
}

pub async fn update(Path(uid): Path<String>, Json(dto): Json<Update>) -> impl IntoResponse {
    // Placeholder: Update logic not implemented yet
    Json(dto)
}

pub async fn delete_one(Path(uid): Path<String>) -> impl IntoResponse {
    (StatusCode::NO_CONTENT, Json(uid))
}

pub async fn delete_all() -> impl IntoResponse {
    // Placeholder: Delete all logic not implemented yet
    StatusCode::NO_CONTENT
}
