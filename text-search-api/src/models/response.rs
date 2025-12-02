use serde::Serialize;
use super::document::DocId;

#[derive(Debug, Clone, Serialize)]
pub struct PerDocCount {
    pub doc_id: DocId,
    pub doc_name: String,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct WordResult {
    pub word: String,
    pub total_count: usize,
    pub per_doc: Vec<PerDocCount>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BenchmarkTiming {
    pub parallel_ms: f64,
    pub sequential_ms: f64,
    pub speedup: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchResponse {
    pub results: Vec<WordResult>,
    pub benchmark: BenchmarkTiming,
}
