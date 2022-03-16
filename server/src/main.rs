use axum::{
    extract::Extension,
    http::{header, Method},
    routing::get,
    Router, Server,
};
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{
    cors::{CorsLayer, Origin},
    trace::TraceLayer,
};

mod api;
mod config;

pub struct State {
    db_pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    config::load_env();
    tracing_subscriber::fmt::init();

    // DB.
    let db_pool = Pool::<Postgres>::connect(&config::DATABASE_URL)
        .await
        .unwrap();

    sqlx::migrate!().run(&db_pool).await.unwrap();

    // State.
    let state = Arc::new(State { db_pool });

    // Middlewares.
    let middlewares = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
                .allow_origin(Origin::list(
                    config::ALLOWED_ORIGIN
                        .iter()
                        .map(|origin| origin.parse().unwrap()),
                )),
        )
        .layer(Extension(state));

    // App.
    let app = Router::new()
        .route("/health", get(|| async { "Healthy!" }))
        .nest("/api", api::init_routes())
        .layer(middlewares);

    tracing::info!("Server listening on: {}", *config::ADDRESS);

    Server::bind(&config::ADDRESS.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}
