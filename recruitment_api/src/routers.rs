use axum::response::IntoResponse;
use axum::Json;
use axum::{Router, routing::{get, Route}};
use hyper::StatusCode;

use crate::domain;



pub fn router() -> Router {
     Router::new().route("/ping", get(|| async {"ping"}))
                    .route("/candidate", get( get_candidate))
}

async fn get_candidate() -> impl IntoResponse {
    let candidate  = domain::candidate::Candidate { name: "bob".to_string(), age: 20 };
    (StatusCode::CREATED, Json(candidate))
}
