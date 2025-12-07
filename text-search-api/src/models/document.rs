use serde::Serialize;
use std::collections::HashMap;

pub type DocId = usize;

#[derive(Debug, Clone)]
pub struct Document {
    pub id: DocId,
    pub name: String,
    pub content: String,
    pub word_counts: HashMap<String, usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DocumentInfo {
    pub id: DocId,
    pub name: String,
}