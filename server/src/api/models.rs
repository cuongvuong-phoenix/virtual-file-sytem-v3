use super::{AppError, VfsError};
use axum::Json;
use chrono::{DateTime, Utc};
use futures::future;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
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
        .map(|rec| rec.is_folder as bool)
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
        .map(|rec| rec.data as Option<String>)?
        .ok_or_else(|| AppError::Vfs(VfsError::PathNotAFile))
    }

    pub async fn ls(&self, db_pool: &Pool<Postgres>) -> Result<Vec<NodeLsItem>, AppError> {
        let paths = sqlx::query_as!(
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
        .map_err(|_| AppError::Database)?;

        if paths.len() > 0 {
            Ok(paths)
        } else {
            Err(AppError::Vfs(VfsError::PathNotAFolder))
        }
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
            .map(|rec| rec.path as Vec<String>),

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
            .map(|rec| rec.path as Vec<String>),
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
        .map(|rec| rec.path as Vec<String>)
    }
}

#[derive(Deserialize)]
pub struct NodePaths {
    paths: Vec<Vec<String>>,
}

impl NodePaths {
    pub async fn rm(&self, db_pool: &Pool<Postgres>) -> Json<Value> {
        let a: Vec<Result<(&Vec<String>, bool), AppError>> =
            future::join_all(self.paths.iter().map(|path| async move {
                sqlx::query!(
                    r#"
                    DELETE FROM node
                    WHERE "path" @> $1
                    RETURNING "path"
                    "#,
                    path
                )
                .fetch_optional(db_pool)
                .await
                .map_err(|_| AppError::Database)
                .map(|rec_opt| match rec_opt {
                    Some(_) => (path, true),
                    None => (path, false),
                })
            }))
            .await;

        let paths: Vec<_> = a.into_iter().filter_map(|result| result.ok()).collect();

        let (removed_paths, non_existed_paths): (Vec<_>, Vec<_>) =
            paths.into_iter().partition(|tuple| tuple.1);

        let removed_paths: Vec<_> = removed_paths.into_iter().map(|tuple| tuple.0).collect();
        let non_existed_paths: Vec<_> =
            non_existed_paths.into_iter().map(|tuple| tuple.0).collect();

        Json(json!({
            "data": {
                "removedPaths": removed_paths,
                "nonExistedPaths": non_existed_paths
            }
        }))
    }
}
