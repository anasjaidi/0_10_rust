use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessHttpResponse<T>
where
    T: Serialize,
{
    pub message: String,
    pub code: u16,
    pub data: Option<T>,
}

impl<T> SuccessHttpResponse<T>
where
    T: Serialize,
{
    pub fn new(code: impl Into<u16>, message: impl Into<String>, data: Option<T>) -> Self {
        Self {
            message: message.into(),
            code: code.into(),
            data,
        }
    }

    pub fn change_data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    pub fn change_code<V>(mut self, code: V) -> Self
    where
        V: Into<u16>,
    {
        self.code = code.into();
        self
    }
}

impl<T> Default for SuccessHttpResponse<T>
where
    T: Serialize,
{
    fn default() -> Self {
        Self {
            message: "success".to_owned(),
            code: 200,
            data: None,
        }
    }
}

impl<T: Serialize> IntoResponse for SuccessHttpResponse<T> {
    fn into_response(self) -> Response {
        (StatusCode::from_u16(self.code).unwrap(), Json(self)).into_response()
    }
}
