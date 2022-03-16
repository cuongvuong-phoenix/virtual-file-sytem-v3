use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;

pub struct AppResponse<T: Serialize> {
    data: T,
    code: StatusCode,
}

impl<T> AppResponse<T>
where
    T: Serialize,
{
    pub fn new(data: T, code: StatusCode) -> Self {
        Self { data, code }
    }
}

impl<T> IntoResponse for AppResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (
            self.code,
            Json(json!({
                "data": self.data
            })),
        )
            .into_response()
    }
}
