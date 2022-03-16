use super::{AppError, VfsError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Serialize)]
pub struct NodeListItem {
    path: Vec<String>,
    is_folder: bool,
    created_at: DateTime<Utc>,
    size: i64,
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
        .ok_or_else(|| AppError::Vfs(VfsError::PathNotExist))
    }

    pub async fn cat(&self, db_pool: &Pool<Postgres>) -> Result<String, AppError> {
        sqlx::query!(
            r#"
            SELECT "data"
            FROM node
            WHERE "path" = $1
            "#,
            &self.path
        )
        .fetch_optional(db_pool)
        .await
        .map_err(|_| AppError::Database)?
        .map(|rec| rec.data)
        .ok_or_else(|| AppError::Vfs(VfsError::PathNotExist))?
        .ok_or_else(|| AppError::Vfs(VfsError::PathIsAFolder))
    }

    pub async fn ls(&self, db_pool: &Pool<Postgres>) -> Result<Vec<NodeListItem>, AppError> {
        sqlx::query_as!(
            NodeListItem,
            r#"
            SELECT
                "path" AS "path!",
                is_folder AS "is_folder!",
                created_at AS "created_at!",
                char_length("data") AS "size!"
            FROM node
            WHERE
                NOT is_folder
                AND "path" @> $1
                AND array_length("path", 1) = array_length($1, 1) + 1
            UNION
                SELECT p."path", p.is_folder, p.created_at, c_calc."size"
                FROM
                    node p
                    JOIN LATERAL (
                        SELECT p."path", coalesce(sum(char_length(c."data")), 0) AS "size"
                        FROM node c
                        WHERE
                            NOT c.is_folder
                            AND c."path" @> p."path"
                            AND array_length(c."path", 1) = array_length(p."path", 1) + 1
                    ) c_calc ON p."path" = c_calc."path"
                WHERE
                    is_folder
                    AND p."path" @> $1
                    AND array_length(p."path", 1) = array_length($1, 1) + 1;
            "#,
            &self.path
        )
        .fetch_all(db_pool)
        .await
        .map_err(|_| AppError::Database)
    }
}
