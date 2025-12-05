use serde::Serialize;
use super::document::DocId;

macro_rules! derive_response {
    ($item:item) => {
        #[derive(Debug, Clone, Serialize)]
        $item
    };
}

derive_response!(pub struct PerDocCount {
    pub doc_id: DocId,
    pub doc_name: String,
    pub count: usize,
    pub snippets: Vec<String>,
});

derive_response!(pub struct WordResult {
    pub word: String,
    pub total_count: usize,
    pub per_doc: Vec<PerDocCount>,
});

derive_response!(pub struct BenchmarkTiming {
    pub parallel_ms: f64,
    pub sequential_ms: f64,
    pub speedup: f64,
});

derive_response!(pub struct DocumentMatch {
    pub doc_id: DocId,
    pub doc_name: String,
    pub matched_words: usize,
});

derive_response!(pub struct SearchResponse {
    pub results: Vec<WordResult>,
    pub benchmark: BenchmarkTiming,
    pub docs_with_all_words: Vec<DocumentMatch>,
});
