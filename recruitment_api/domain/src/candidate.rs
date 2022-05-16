use std::collections::LinkedList;

use serde::Serialize;

use crate::selection_status::{SelectionProcessStatus, SelectionStatus};

#[derive(Debug, Clone, Serialize)]
pub struct Candidate {
    pub name: String,
    pub age: u32,
    pub process_status: SelectionProcessStatus,
    pub selection_status: SelectionStatus,
}

#[derive(Debug, Clone, Serialize)]
pub struct Candidates {
    pub value: LinkedList<Candidate>
}