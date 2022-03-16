use super::{AppError, VfsError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Serialize)]
pub struct Node {
    path: Vec<String>,
    is_folder: bool,
    data: Option<String>,
    created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct NodePath {
    path: Vec<String>,
}

impl NodePath {
    pub async fn cd(&self, db_pool: &Pool<Postgres>) -> Result<bool, AppError> {
        sqlx::query!(
            r#"
            SELECT is_folder
            FROM node
            WHERE
                "path" = $1
            "#,
            &self.path
        )
        .fetch_optional(db_pool)
        .await
        .map_err(|_| AppError::Database)?
        .map(|rec| rec.is_folder)
        .ok_or_else(|| AppError::Vfs(VfsError::NotExist))
    }
}
