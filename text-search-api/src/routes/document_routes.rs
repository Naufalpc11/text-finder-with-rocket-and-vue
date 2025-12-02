use rocket::{State, serde::json::Json};
use crate::models::document::DocumentInfo;
use crate::services::calculate_doc_stats;
use crate::AppState;

#[get("/docs")]
pub fn list_docs(state: &State<AppState>) -> Json<Vec<DocumentInfo>> {
    let docs_guard = state.docs.read().expect("RwLock poisoned");
    let list = docs_guard
        .iter()
        .map(|d| DocumentInfo {
            id: d.id,
            name: d.name.clone(),
        })
        .collect();
   
    Json(list)
}

#[get("/stats")]
pub fn get_stats(state: &State<AppState>) -> Json<serde_json::Value> {
    let docs_guard = state.docs.read().expect("RwLock poisoned");
    let (total_docs, total_words, total_bytes, avg_words) = calculate_doc_stats(&docs_guard);
   
    Json(serde_json::json!({
        "total_documents": total_docs,
        "total_words": total_words,
        "total_bytes": total_bytes,
        "average_words_per_doc": avg_words,
    }))
}
