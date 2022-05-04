use axum::{Router, routing::get};



pub fn router() -> Router {
    Router::new().route("/ping", get(|| async { "pong" }))
}