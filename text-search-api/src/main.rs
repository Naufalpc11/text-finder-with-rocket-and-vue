#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket, State};
use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::http::Status;
use rocket::response::status;
use rocket_cors::{AllowedOrigins, CorsOptions};
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{
    RwLock,
    atomic::{AtomicUsize, Ordering},
};

type DocId = usize;

#[derive(Debug, Clone)]
struct Document {
    id: DocId,
    name: String,
    content: String,
   
    word_counts: HashMap<String, usize>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
struct DocumentInfo {
    id: DocId,
    name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
struct UploadedFile {
    name: String,
    content: String,
}
struct AppState {
    docs: RwLock<Vec<Document>>,
    next_id: AtomicUsize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
struct UploadResponse {
    total_files: usize,
    doc_ids: Vec<DocId>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
struct SearchRequest {
   
    words: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
struct PerDocCount {
    doc_id: DocId,
    doc_name: String,
    count: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
struct WordResult {
    word: String,
    total_count: usize,
    per_doc: Vec<PerDocCount>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
struct SearchResponse {
    results: Vec<WordResult>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
struct DeleteResponse {
    success: bool,
    remaining: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
struct DeleteAllResponse {
    success: bool,
    remaining: usize,
}

fn normalize_token(token: &str) -> String {
    token
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase()
}

fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(normalize_token)
        .filter(|w| !w.is_empty())
        .collect()
}

fn build_word_counts(text: &str) -> HashMap<String, usize> {
    tokenize(text)
        .into_iter()
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        })
}

fn count_total_occurrences(per_doc: &[PerDocCount]) -> usize {
    per_doc.iter().map(|pd| pd.count).sum()
}

fn filter_docs_with_word<'a>(docs: &'a [Document], word: &str) -> Vec<&'a Document> {
    docs.iter()
        .filter(|doc| doc.word_counts.contains_key(word))
        .collect()
}

fn count_word(docs: &[Document], word: &str, index: usize, acc: usize) -> usize {
    if index >= docs.len() {
        return acc;
    }
   
    let count = docs[index]
        .word_counts
        .get(word)
        .copied()
        .unwrap_or(0);
   
    count_word(docs, word, index + 1, acc + count)
}

fn calculate_doc_stats(docs: &[Document]) -> (usize, usize, usize, f64) {
    let total_docs = docs.len();
    let total_words: usize = docs
        .iter()
        .map(|doc| doc.word_counts.values().sum::<usize>())
        .sum();
    
    let total_bytes: usize = docs.iter().map(|d| d.content.len()).sum();
   
    let avg_words = if total_docs > 0 {
        total_words as f64 / total_docs as f64
    } else {
        0.0
    };
    (total_docs, total_words, total_bytes, avg_words)
}

fn search_single_word(docs: &[Document], raw_word: &str) -> WordResult {
    let word = normalize_token(raw_word);
    
    let relevant_docs = filter_docs_with_word(docs, &word);
    
    let per_doc: Vec<PerDocCount> = relevant_docs
        .into_iter()
        .filter_map(|doc| {
            let count = doc.word_counts.get(&word).copied().unwrap_or(0);
            if count > 0 {
                Some(PerDocCount {
                    doc_id: doc.id,
                    doc_name: doc.name.clone(),
                    count,
                })
            } else {
                None
            }
        })
        .collect();

    let total_count = count_total_occurrences(&per_doc);

    #[cfg(debug_assertions)]
    {
        let recursive_total = count_word(docs, &word, 0, 0);
        debug_assert_eq!(total_count, recursive_total, 
            "Mismatch: iterative={} vs recursive={}", total_count, recursive_total);
    }

    WordResult {
        word,
        total_count,
        per_doc,
    }
}

#[post("/upload", format = "json", data = "<files>")]
async fn upload_files(
    state: &State<AppState>,
    files: Json<Vec<UploadedFile>>,
) -> Json<UploadResponse> {
    let processed_docs: Vec<(String, String, HashMap<String, usize>)> = if files.len() >= 2 {
        files
            .par_iter()
            .map(|f| {
                let word_counts = build_word_counts(&f.content);
                (f.name.clone(), f.content.clone(), word_counts)
            })
            .collect()
    } else {
        files
            .iter()
            .map(|f| {
                let word_counts = build_word_counts(&f.content);
                (f.name.clone(), f.content.clone(), word_counts)
            })
            .collect()
    };

    let mut docs_guard = state.docs.write().expect("RwLock poisoned");
    let new_ids: Vec<DocId> = processed_docs
        .into_iter()
        .map(|(name, content, word_counts)| {
            let id = state.next_id.fetch_add(1, Ordering::Relaxed);
            let doc = Document {
                id,
                name,
                content,
                word_counts,
            };
            docs_guard.push(doc);
            id
        })
        .collect();


    Json(UploadResponse {
        total_files: docs_guard.len(),
        doc_ids: new_ids,
    })
}

#[get("/docs")]
fn list_docs(state: &State<AppState>) -> Json<Vec<DocumentInfo>> {
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
fn get_stats(state: &State<AppState>) -> Json<serde_json::Value> {
    let docs_guard = state.docs.read().expect("RwLock poisoned");
    let (total_docs, total_words, total_bytes, avg_words) = calculate_doc_stats(&docs_guard);
   
    Json(serde_json::json!({
        "total_documents": total_docs,
        "total_words": total_words,
        "total_bytes": total_bytes,
        "average_words_per_doc": avg_words,
    }))
}

#[post("/search", format = "json", data = "<req>")]
fn search(state: &State<AppState>, req: Json<SearchRequest>) -> Json<SearchResponse> {
    let words: Vec<String> = req
        .words
        .iter()
        .map(|w| w.trim().to_string())
        .filter(|w| !w.is_empty())
        .collect();

    let docs_guard = state.docs.read().expect("RwLock poisoned");
    let results: Vec<WordResult> = if words.len() <= 1 {
       
        words
            .iter()
            .map(|w| search_single_word(&docs_guard, w))
            .collect()
    } else {
        words
            .par_iter()
            .map(|w| search_single_word(&docs_guard, w))
            .collect()
    };

    Json(SearchResponse { results })
}


#[delete("/docs/<id>")]
fn delete_doc(
    state: &State<AppState>,
    id: DocId,
) -> Result<Json<DeleteResponse>, status::Custom<String>> {
    let mut docs = state.docs.write().expect("RwLock poisoned");
    let before = docs.len();

    docs.retain(|d| d.id != id);

    if docs.len() == before {
       
        Err(status::Custom(
            Status::NotFound,
            format!("Document with id {} not found", id),
        ))
    } else {
        Ok(Json(DeleteResponse {
            success: true,
            remaining: docs.len(),
        }))
    }
}

#[delete("/docs")]
fn delete_all_docs(state: &State<AppState>) -> Json<DeleteAllResponse> {
    let mut docs = state.docs.write().expect("RwLock poisoned");
    docs.clear();
    state.next_id.store(0, Ordering::Relaxed);
    Json(DeleteAllResponse {
        success: true,
        remaining: 0,
    })
}

fn build_rocket() -> Rocket<Build> {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:5173",
        "http://127.0.0.1:5173",
    ]);

    let cors = CorsOptions {
        allowed_origins,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error building CORS");


    rocket::build()
        .manage(AppState {
            docs: RwLock::new(Vec::new()),
            next_id: AtomicUsize::new(0),
        })
        .mount(
            "/",
            routes![
                upload_files,
                list_docs,
                get_stats,
                search,
                delete_doc,
                delete_all_docs
            ],
        )
        .attach(cors)
}

#[launch]
fn rocket() -> _ {
    build_rocket()
}
