use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Internal Server Error")]
    Internal,
    #[error("Database Error")]
    Database,
    #[error(transparent)]
    Vfs(#[from] VfsError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::Database => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::Vfs(e) => match e {
                VfsError::PathNotExist => (StatusCode::NOT_FOUND, e.to_string()),
                VfsError::PathIsAFolder => (StatusCode::BAD_REQUEST, e.to_string()),
            },
        };

        let body = Json(json!({
            "error": {
                "code": status.as_str(),
                "message": err_msg
            },
        }));

        (status, body).into_response()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum VfsError {
    #[error("path does not exists")]
    PathNotExist,
    #[error("path is a folder")]
    PathIsAFolder,
}
