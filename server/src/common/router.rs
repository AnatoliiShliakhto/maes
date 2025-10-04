use ::axum::{
    Router,
    routing::{get, post},
};
use ::std::path::PathBuf;
use ::tower_http::services::{ServeDir, ServeFile};
use crate::handler::*;

pub fn init_router(path: PathBuf) -> Router {
    Router::new()
        .fallback_service(
            ServeDir::new(path.clone()).fallback(ServeFile::new(path.join("index.html"))),
        )
        .route("/health", get(health::liveness))
}