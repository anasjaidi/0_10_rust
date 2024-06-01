use core::fmt;
use std::{error::Error, fmt::Display};

// Using Axum
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Default, Serialize)]
pub struct ErrorHttpResponse {
    pub message: String,
    pub code: u16,
    pub description: Option<String>,
}

impl ErrorHttpResponse {
    fn new(
        code: impl Into<u16>,
        message: impl Into<String>,
        description: Option<impl Into<String>>,
    ) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            description: description.map(|d| d.into()),
        }
    }

    fn add_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    fn add_code(mut self, code: impl Into<u16>) -> Self {
        self.code = code.into();
        self
    }
}

impl Display for ErrorHttpResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self {
                message,
                description: Some(description),
                code,
            } => write!(
                f,
                "Error: {}\n{}\nError code: {}",
                message, description, code
            ),

            _ => write!(f, "Error: {}\nError code: {}", self.message, self.code),
        }
    }
}

impl Error for ErrorHttpResponse {}
impl IntoResponse for ErrorHttpResponse {
    /*         (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response() */
    fn into_response(self) -> Response {
        (
            StatusCode::from_u16(self.code).unwrap(),
            Json(json!({ "error": self.message })),
        )
            .into_response()
    }
}
