use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Candidate {
    pub(crate) name: String,
    pub(crate) age: u32,
}