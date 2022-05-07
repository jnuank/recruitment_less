use std::collections::LinkedList;

use crate::domain::candidate::Candidate;
use axum::response::IntoResponse;
use axum::Json;
use axum::{Router, routing::{get}};
use hyper::StatusCode;

use crate::domain::selectionStatus::{SelectionProcessStatus, SelectionStatus};



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
            processStatus: SelectionProcessStatus::エントリー,
            selectionStatus: SelectionStatus::選考中, 
    });
    candidates.push_back(
    Candidate {
            name: "alice".to_string(),
            age: 25,
            processStatus: SelectionProcessStatus::一次面接,
            selectionStatus: SelectionStatus::辞退, 
    });
    candidates.push_back(
    Candidate {
            name: "cathy".to_string(),
            age: 23,
            processStatus: SelectionProcessStatus::最終面接,
            selectionStatus: SelectionStatus::選考中, 
    });

    let candidate_fileter: Vec<Candidate> = candidates
        .iter()
        .cloned()
        .filter(|x| x.selectionStatus == SelectionStatus::選考中 ).collect();    

    (StatusCode::CREATED, Json(candidate_fileter))
}