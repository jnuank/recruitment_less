use std::collections::LinkedList;

use axum::response::IntoResponse;
use axum::Json;
use axum::{Router, routing::{get}};
use domain::candidate::Candidate;
use domain::selection_status::{SelectionProcessStatus, SelectionStatus};
use hyper::StatusCode;

pub fn router() -> Router {
    Router::new()
        .route("/ping", get(|| async {"ping"}))
        .route("/candidate", get( get_candidate))
}

async fn get_candidate() -> impl IntoResponse {
    let mut candidates = LinkedList::new();
    candidates.push_back(
    Candidate {
            name: "bob".to_string(),
            age: 20,
            process_status: SelectionProcessStatus::エントリー,
            selection_status: SelectionStatus::選考中, 
    });
    candidates.push_back(
    Candidate {
            name: "alice".to_string(),
            age: 25,
            process_status: SelectionProcessStatus::一次面接,
            selection_status: SelectionStatus::辞退, 
    });
    candidates.push_back(
    Candidate {
            name: "cathy".to_string(),
            age: 23,
            process_status: SelectionProcessStatus::最終面接,
            selection_status: SelectionStatus::選考中, 
    });

    let candidate_fileter: Vec<Candidate> = candidates
        .iter()
        .cloned()
        .filter(|x| x.selection_status == SelectionStatus::選考中 ).collect();    

    (StatusCode::CREATED, Json(candidate_fileter))
}