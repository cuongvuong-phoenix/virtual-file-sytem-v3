use super::{
    models::NodePath, AppError, AppResponse, Node, NodeLsItem, NodePathFolderPath, NodePathName,
    NodePathNameData, NodePaths,
};
use crate::State;
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
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

pub async fn up(
    Extension(state): Extension<Arc<State>>,
    Json(node): Json<NodePathNameData>,
) -> Result<AppResponse<Vec<String>>, AppError> {
    let path = node.up(&state.db_pool).await?;

    Ok(AppResponse::new(path, StatusCode::OK))
}

pub async fn mv(
    Extension(state): Extension<Arc<State>>,
    Json(node): Json<NodePathFolderPath>,
) -> Result<AppResponse<Vec<String>>, AppError> {
    let path = node.mv(&state.db_pool).await?;

    Ok(AppResponse::new(path, StatusCode::OK))
}

pub async fn rm(
    Extension(state): Extension<Arc<State>>,
    Json(node): Json<NodePaths>,
) -> impl IntoResponse {
    let body = node.rm(&state.db_pool).await;

    (StatusCode::OK, body).into_response()
}
