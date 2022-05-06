use std::collections::LinkedList;

use crate::domain::selectionStatus::SelectionStatus;
use crate::domain::selectionStatus::SelectionProcessStatus;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Candidate {
    pub(crate) name: String,
    pub(crate) age: u32,
    pub(crate) processStatus: SelectionProcessStatus,
    pub(crate) selectionStatus: SelectionStatus,
}

#[derive(Debug, Clone, Serialize)]
pub struct Candidates {
    pub(crate) value: LinkedList<Candidate>
}