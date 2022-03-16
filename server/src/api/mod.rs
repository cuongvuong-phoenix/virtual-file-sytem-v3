use axum::{routing::post, Router};

mod errors;
mod handlers;
mod models;
mod response;

pub use errors::*;
pub use handlers::*;
pub use models::*;
pub use response::*;

pub fn init_routes() -> Router {
    Router::new()
        .route("/cd", post(cd))
        .route("/cat", post(cat))
        .route("/ls", post(ls))
        .route("/find", post(find))
        .route("/up", post(up))
}
