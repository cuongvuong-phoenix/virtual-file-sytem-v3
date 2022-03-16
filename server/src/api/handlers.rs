use super::{models::NodePath, AppError, AppResponse, Node, NodeLsItem, NodePathName};
use crate::State;
use axum::{extract::Extension, http::StatusCode, Json};
use std::sync::Arc;

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
) -> Result<AppResponse<Vec<NodeLsItem>>, AppError> {
    let nodes = node.ls(&state.db_pool).await?;

    Ok(AppResponse::new(nodes, StatusCode::OK))
}

pub async fn find(
    Extension(state): Extension<Arc<State>>,
    Json(node): Json<NodePathName>,
) -> Result<AppResponse<Vec<Node>>, AppError> {
    let nodes = node.find(&state.db_pool).await?;

    Ok(AppResponse::new(nodes, StatusCode::OK))
}
