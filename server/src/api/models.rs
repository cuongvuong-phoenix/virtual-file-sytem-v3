use super::{AppError, VfsError};
use axum::Json;
use chrono::{DateTime, Utc};
use futures::future;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{Acquire, Pool, Postgres};

// ----------------------------------------------------------------
// Serializable
// ----------------------------------------------------------------
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    id: i32,
    path: Vec<String>,
    is_folder: bool,
    created_at: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeLsItem {
    id: i32,
    path: Vec<String>,
    is_folder: bool,
    created_at: DateTime<Utc>,
    size: i64,
}

// ----------------------------------------------------------------
// Deserializable
// ----------------------------------------------------------------
#[derive(Deserialize)]
pub struct NodeCr {
    path: Vec<String>,
    data: Option<String>,
    parents_opt: bool,
}

impl NodeCr {
    pub async fn cr(&self, db_pool: &Pool<Postgres>) -> Result<Node, AppError> {
        do_manipulate_root_path(&self.path)?;

        let mut transaction = db_pool.begin().await?;

        transaction.begin().await?;

        let is_parent_folder = sqlx::query!(
            r#"
            SELECT is_folder
            FROM node
            WHERE "path" = ($1::text[])[:(array_upper($1, 1) - 1)]
            "#,
            &self.path
        )
        .fetch_optional(&mut transaction)
        .await?
        .map(|rec| rec.is_folder as bool);

        let result = match is_parent_folder {
            Some(parent_is_folder) => {
                if parent_is_folder {
                    sqlx::query_as!(
                        Node,
                        r#"
                        INSERT INTO node ("path", is_folder, "data")
                        VALUES ($1, $2::text IS NULL, $2)
                        ON CONFLICT ("path") DO NOTHING
                        RETURNING id, "path", is_folder, created_at;
                        "#,
                        &self.path,
                        self.data
                    )
                    .fetch_optional(&mut transaction)
                    .await?
                    .ok_or_else(|| AppError::Vfs(VfsError::PathExist))
                } else {
                    Err(AppError::Vfs(VfsError::ParentPathIsAFile))
                }
            }
            None => {
                if !self.parents_opt {
                    Err(AppError::Vfs(VfsError::ParentPathNotExist))
                } else {
                    sqlx::query_as!(
                        Node,
                        r#"
                        WITH RECURSIVE rec AS (
                            SELECT
                                $1::text[] AS "path",
                                $2::text IS NULL AS is_folder,
                                $2 AS "data"
                            UNION ALL
                            SELECT "path"[:(array_upper("path", 1) - 1)], TRUE AS is_folder, NULL
                            FROM rec
                            WHERE array_upper("path", 1) > 1
                        )
                        INSERT INTO node("path", is_folder, "data")
                        SELECT "path", is_folder, "data"
                        FROM rec
                        ON CONFLICT ("path") DO NOTHING
                        RETURNING id, "path", is_folder, created_at;
                        "#,
                        &self.path,
                        self.data
                    )
                    .fetch_one(&mut transaction)
                    .await
                    .map_err(AppError::Database)
                }
            }
        };

        transaction.commit().await?;

        result
    }
}

#[derive(Deserialize)]
pub struct NodePath {
    path: Vec<String>,
}

impl NodePath {
    pub async fn cd(&self, db_pool: &Pool<Postgres>) -> Result<Node, AppError> {
        let node = sqlx::query_as!(
            Node,
            r#"
            SELECT id, "path", is_folder, created_at
            FROM node
            WHERE "path" = $1
            "#,
            &self.path
        )
        .fetch_optional(db_pool)
        .await?
        .ok_or_else(|| AppError::Vfs(VfsError::PathNotExist))?;

        if node.is_folder {
            Ok(node)
        } else {
            Err(AppError::Vfs(VfsError::PathNotAFolder))
        }
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
        .await?
        .ok_or_else(|| AppError::Vfs(VfsError::PathNotExist))
        .map(|rec| rec.data as Option<String>)?
        .ok_or_else(|| AppError::Vfs(VfsError::PathNotAFile))
    }

    pub async fn ls(&self, db_pool: &Pool<Postgres>) -> Result<Vec<NodeLsItem>, AppError> {
        let mut transaction = db_pool.begin().await?;

        transaction.begin().await?;

        let is_folder = sqlx::query!(
            r#"
            SELECT is_folder
            FROM node
            WHERE "path" = $1
            "#,
            &self.path
        )
        .fetch_optional(&mut transaction)
        .await?
        .ok_or_else(|| AppError::Vfs(VfsError::PathNotExist))?
        .is_folder as bool;

        if !is_folder {
            return Err(AppError::Vfs(VfsError::PathNotAFolder));
        }

        let nodes = sqlx::query_as!(
            NodeLsItem,
            r#"
            SELECT
                id AS "id!",
                "path" AS "path!",
                is_folder AS "is_folder!",
                created_at AS "created_at!",
                char_length("data") AS "size!"
            FROM node
            WHERE
                NOT is_folder
                AND "path" @> $1
                AND array_upper("path", 1) = coalesce(array_upper($1, 1), 0) + 1
            UNION ALL
            SELECT p.id, p."path", p.is_folder, p.created_at, c_calc."size"
            FROM
                node p
                JOIN LATERAL (
                    SELECT
                        p."path",
                        coalesce(sum(char_length(c."data")), 0) AS "size"
                    FROM node c
                    WHERE
                        NOT c.is_folder
                        AND c."path" @> p."path"
                        AND array_upper(c."path", 1) = array_upper(p."path", 1) + 1
                ) c_calc ON p."path" = c_calc."path"
            WHERE
                is_folder
                AND p."path" @> $1
                AND array_upper(p."path", 1) = coalesce(array_upper($1, 1), 0) + 1;
            "#,
            &self.path
        )
        .fetch_all(db_pool)
        .await?;

        transaction.commit().await?;

        Ok(nodes)
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
            SELECT id, "path", is_folder, created_at
            FROM node
            WHERE
                "path" @> $1
                AND array_upper("path", 1) > coalesce(array_upper($1, 1), 0)
                AND "path"[array_upper("path", 1)] LIKE '%' || $2 || '%';
            "#,
            &self.path,
            self.name
        )
        .fetch_all(db_pool)
        .await
        .map_err(AppError::Database)
    }
}

#[derive(Deserialize)]
pub struct NodePathNameData {
    path: Vec<String>,
    name: String,
    data: Option<String>,
}

impl NodePathNameData {
    pub async fn up(&self, db_pool: &Pool<Postgres>) -> Result<Node, AppError> {
        do_manipulate_root_path(&self.path)?;

        match &self.data {
            Some(data) => sqlx::query_as!(
                Node,
                r#"
                UPDATE node
                SET
                    "path" = "path"[:(array_upper("path", 1) - 1)] || ARRAY[$2],
                    "data" = $3
                WHERE
                    NOT is_folder
                    AND "path" = $1
                RETURNING id, "path", is_folder, created_at;
                "#,
                &self.path,
                self.name,
                data
            )
            .fetch_optional(db_pool)
            .await?
            .ok_or_else(|| AppError::Vfs(VfsError::PathNotAFile)),

            None => sqlx::query_as!(
                Node,
                r#"
                UPDATE node
                SET
                    "path" = "path"[:(coalesce(array_upper($1, 1), 0) - 1)]
                        || ARRAY[$2]
                        || "path"[(coalesce(array_upper($1, 1), 0) + 1):]
                WHERE "path" @> $1
                RETURNING id, "path", is_folder, created_at;
                "#,
                &self.path,
                self.name
            )
            .fetch_optional(db_pool)
            .await?
            .ok_or_else(|| AppError::Vfs(VfsError::PathNotExist)),
        }
    }
}

#[derive(Deserialize)]
pub struct NodePathFolderPath {
    path: Vec<String>,
    folder_path: Vec<String>,
}

impl NodePathFolderPath {
    pub async fn mv(&self, db_pool: &Pool<Postgres>) -> Result<Node, AppError> {
        do_manipulate_root_path(&self.path)?;

        let mut transaction = db_pool.begin().await?;

        transaction.begin().await?;

        let is_folder_path = sqlx::query!(
            r#"
            SELECT is_folder
            FROM node
            WHERE "path" = $1
            "#,
            &self.folder_path
        )
        .fetch_optional(&mut transaction)
        .await?
        .ok_or_else(|| AppError::Vfs(VfsError::FolderPathNotExist))?
        .is_folder as bool;

        if !is_folder_path {
            return Err(AppError::Vfs(VfsError::FolderPathIsAFile));
        }

        let node = sqlx::query_as!(
            Node,
            r#"
            UPDATE node
            SET
                "path" = $2
                    || "path"[(coalesce(array_upper($1, 1), 0)):]
            WHERE "path" @> $1
            RETURNING id, "path", is_folder, created_at;
            "#,
            &self.path,
            &self.folder_path
        )
        .fetch_optional(db_pool)
        .await?
        .ok_or_else(|| AppError::Vfs(VfsError::PathNotExist))?;

        transaction.commit().await?;

        Ok(node)
    }
}

#[derive(Deserialize)]
pub struct NodePaths {
    paths: Vec<Vec<String>>,
}

impl NodePaths {
    pub async fn rm(&self, db_pool: &Pool<Postgres>) -> Json<Value> {
        for path in &self.paths {
            if path.len() == 0 {
                return Json(json!({
                    "error": {
                        "code": "400",
                        "message": "cannot delete root path"
                    }
                }));
            }
        }

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
                .map_err(AppError::Database)
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

fn do_manipulate_root_path(path: &[String]) -> Result<(), AppError> {
    if path.len() == 0 {
        return Err(AppError::Vfs(VfsError::ManipulateRootPath));
    }

    Ok(())
}
