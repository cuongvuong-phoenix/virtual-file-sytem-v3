use std::sync::Arc;

use super::{models::NodePath, AppError, AppResponse, NodeListItem};
use crate::State;
use axum::{extract::Extension, http::StatusCode, Json};

pub async fn cd(
    Extension(state): Extension<Arc<State>>,
    Json(node): Json<NodePath>,
) -> Result<AppResponse<bool>, AppError> {
    let is_folder = node.cd(&state.db_pool).await?;

    Ok(AppResponse::new(is_folder, StatusCode::OK))
}

pub async fn cat(
    Extension(state): Extension<Arc<State>>,
    Json(node): Json<NodePath>,
) -> Result<AppResponse<String>, AppError> {
    let data = node.cat(&state.db_pool).await?;

    Ok(AppResponse::new(data, StatusCode::OK))
}

pub async fn ls(
    Extension(state): Extension<Arc<State>>,
    Json(node): Json<NodePath>,
) -> Result<AppResponse<Vec<NodeListItem>>, AppError> {
    let nodes = node.ls(&state.db_pool).await?;

    Ok(AppResponse::new(nodes, StatusCode::OK))
}
