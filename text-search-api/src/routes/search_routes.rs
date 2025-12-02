use rocket::{State, serde::json::Json};
use crate::models::request::SearchRequest;
use crate::models::response::{SearchResponse, BenchmarkTiming};
use crate::services::{search_words_parallel, search_words_sequential};
use crate::services::search_service::split_query_into_words;
use crate::AppState;
use std::time::Instant;

#[post("/search", format = "json", data = "<req>")]
pub fn search(state: &State<AppState>, req: Json<SearchRequest>) -> Json<SearchResponse> {
    let words = split_query_into_words(&req.query);

    let docs_guard = state.docs.read().expect("RwLock poisoned");
    
    let start_parallel = Instant::now();
    let results_parallel = if words.len() <= 1 {
        search_words_sequential(&docs_guard, &words)
    } else {
        search_words_parallel(&docs_guard, &words)
    };
    let parallel_duration = start_parallel.elapsed();
    
    let start_sequential = Instant::now();
    let _results_sequential = search_words_sequential(&docs_guard, &words);
    let sequential_duration = start_sequential.elapsed();
    
    let parallel_ms = parallel_duration.as_secs_f64() * 1000.0;
    let sequential_ms = sequential_duration.as_secs_f64() * 1000.0;
    let speedup = if parallel_ms > 0.0 {
        sequential_ms / parallel_ms
    } else {
        1.0
    };

    Json(SearchResponse {
        results: results_parallel,
        benchmark: BenchmarkTiming {
            parallel_ms,
            sequential_ms,
            speedup,
        },
    })
}
