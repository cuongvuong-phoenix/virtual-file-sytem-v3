use std::sync::Arc;

use super::{models::NodePath, AppError, AppResponse};
use crate::State;
use axum::{extract::Extension, http::StatusCode, Json};

pub async fn cd(
    Extension(state): Extension<Arc<State>>,
    Json(node): Json<NodePath>,
) -> Result<AppResponse<bool>, AppError> {
    let is_folder = node.cd(&state.db_pool).await?;

    Ok(AppResponse::new(is_folder, StatusCode::OK))
}
