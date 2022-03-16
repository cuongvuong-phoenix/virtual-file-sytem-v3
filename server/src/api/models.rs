use super::{AppError, VfsError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

// ----------------------------------------------------------------
// Serializable
// ----------------------------------------------------------------
#[derive(Serialize)]
pub struct NodeLsItem {
    path: Vec<String>,
    is_folder: bool,
    created_at: DateTime<Utc>,
    size: i64,
}

#[derive(Serialize)]
pub struct Node {
    path: Vec<String>,
    is_folder: bool,
    created_at: DateTime<Utc>,
}

// ----------------------------------------------------------------
// Deserializable
// ----------------------------------------------------------------
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
        .ok_or_else(|| AppError::Vfs(VfsError::PathNotExist))
        .map(|rec| rec.data)?
        .ok_or_else(|| AppError::Vfs(VfsError::PathNotAFile))
    }

    pub async fn ls(&self, db_pool: &Pool<Postgres>) -> Result<Vec<NodeLsItem>, AppError> {
        sqlx::query_as!(
            NodeLsItem,
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

#[derive(Deserialize)]
pub struct NodePathName {
    path: Vec<String>,
    name: String,
}

impl NodePathName {
    pub async fn find(&self, db_pool: &Pool<Postgres>) -> Result<Vec<Node>, AppError> {
        sqlx::query_as!(
            Node,
            r#"
            SELECT "path", is_folder, created_at
            FROM node
            WHERE
                "path" @> $1
                AND array_length("path", 1) > array_length($1, 1)
                AND "path"[array_length("path", 1)] LIKE '%' || $2 || '%';
            "#,
            &self.path,
            self.name
        )
        .fetch_all(db_pool)
        .await
        .map_err(|_| AppError::Database)
    }
}

#[derive(Deserialize)]
pub struct NodePathNameData {
    path: Vec<String>,
    name: String,
    data: Option<String>,
}

impl NodePathNameData {
    pub async fn up(&self, db_pool: &Pool<Postgres>) -> Result<Vec<String>, AppError> {
        match &self.data {
            Some(data) => sqlx::query!(
                r#"
                UPDATE node
                SET
                    "path" = "path"[:(array_length("path", 1) - 1)] || ARRAY[$2],
                    "data" = $3
                WHERE
                    NOT is_folder
                    AND "path" = $1
                RETURNING "path";
                "#,
                &self.path,
                self.name,
                data
            )
            .fetch_optional(db_pool)
            .await
            .map_err(|_| AppError::Database)?
            .ok_or_else(|| AppError::Vfs(VfsError::PathNotAFile))
            .map(|rec| rec.path),

            None => sqlx::query!(
                r#"
                UPDATE node
                SET
                    "path" = "path"[:(array_length($1, 1) - 1)]
                        || ARRAY[$2]
                        || "path"[(array_length($1, 1) + 1):]
                WHERE "path" @> $1
                RETURNING "path";
                "#,
                &self.path,
                self.name
            )
            .fetch_optional(db_pool)
            .await
            .map_err(|_| AppError::Database)?
            .ok_or_else(|| AppError::Vfs(VfsError::PathNotExist))
            .map(|rec| rec.path),
        }
    }
}

#[derive(Deserialize)]
pub struct NodePathFolderPath {
    path: Vec<String>,
    folder_path: Vec<String>,
}

impl NodePathFolderPath {
    pub async fn mv(&self, db_pool: &Pool<Postgres>) -> Result<Vec<String>, AppError> {
        sqlx::query!(
            r#"
            UPDATE node
            SET
                "path" = $2
                    || "path"[(array_length($1, 1)):]
            WHERE "path" @> $1
            RETURNING "path", is_folder, "data", created_at;
            "#,
            &self.path,
            &self.folder_path
        )
        .fetch_optional(db_pool)
        .await
        .map_err(|_| AppError::Database)?
        .ok_or_else(|| AppError::Vfs(VfsError::PathNotExist))
        .map(|rec| rec.path)
    }
}