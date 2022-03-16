use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database Error")]
    OldDatabase,
    #[error(transparent)]
    Database(#[from] sqlx::Error),
    #[error(transparent)]
    Vfs(#[from] VfsError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            AppError::OldDatabase => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Vfs(e) => (StatusCode::BAD_REQUEST, e.to_string()),
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
    #[error("parent path is a file")]
    ParentPathIsAFile,
    #[error("parent path does not exist")]
    ParentPathNotExist,
    #[error("path has already existed")]
    PathExist,
    #[error("path does not exist")]
    PathNotExist,
    #[error("path is not a file")]
    PathNotAFile,
    #[error("path is not a folder")]
    PathNotAFolder,
}
