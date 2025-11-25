#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket, State};
use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket_cors::{AllowedOrigins, CorsOptions};
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::RwLock;

type DocId = usize;

// =================== MODEL & STATE ===================

#[derive(Debug, Clone)]
struct Document {
    id: DocId,
    name: String,
    content: String,
    // index kata -> jumlah kemunculan di dokumen ini
    word_counts: HashMap<String, usize>,
}

// dipakai untuk list dokumen ke frontend
#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
struct DocumentInfo {
    id: DocId,
    name: String,
}

// payload upload dari frontend
#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
struct UploadedFile {
    name: String,
    content: String,
}

struct AppState {
    docs: RwLock<Vec<Document>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
struct UploadResponse {
    total_files: usize,
    doc_ids: Vec<DocId>,
}

// payload search dari frontend
#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
struct SearchRequest {
    // misal ["kami", "mahasiswa"]
    words: Vec<String>,
}

// hasil pencarian per dokumen
#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
struct PerDocCount {
    doc_id: DocId,
    doc_name: String,
    count: usize,
}

// hasil pencarian per kata
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

// =================== FUNGSI FP: TOKEN & INDEX ===================

// normalisasi satu token: buang tanda baca, jadi lowercase
fn normalize_token(token: &str) -> String {
    token
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase()
}

// split text jadi list kata yang sudah dinormalisasi
fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(normalize_token)
        .filter(|w| !w.is_empty())
        .collect()
}

// bangun index kata -> count untuk satu dokumen
fn build_word_counts(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for w in tokenize(text) {
        *counts.entry(w).or_insert(0) += 1;
    }
    counts
}

// fungsi pure: cari satu kata di semua dokumen
fn search_single_word(docs: &[Document], raw_word: &str) -> WordResult {
    let word = normalize_token(raw_word);
    let mut per_doc = Vec::new();
    let mut total = 0;

    for doc in docs {
        if let Some(&count) = doc.word_counts.get(&word) {
            if count > 0 {
                total += count;
                per_doc.push(PerDocCount {
                    doc_id: doc.id,
                    doc_name: doc.name.clone(),
                    count,
                });
            }
        }
    }

    WordResult {
        word,
        total_count: total,
        per_doc,
    }
}

// =================== ROUTES ===================

#[post("/upload", format = "json", data = "<files>")]
async fn upload_files(
    state: &State<AppState>,
    files: Json<Vec<UploadedFile>>,
) -> Json<UploadResponse> {
    let mut docs_guard = state.docs.write().expect("RwLock poisoned");
    let mut new_ids = Vec::new();

    for f in files.iter() {
        let id = docs_guard.len();
        let word_counts = build_word_counts(&f.content);

        let doc = Document {
            id,
            name: f.name.clone(),
            content: f.content.clone(),
            word_counts,
        };

        docs_guard.push(doc);
        new_ids.push(id);
    }

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

// ========== INI BAGIAN SEARCH + MULTI-PROCESSING ==========

#[post("/search", format = "json", data = "<req>")]
fn search(state: &State<AppState>, req: Json<SearchRequest>) -> Json<SearchResponse> {
    // buang spasi kosong dsb
    let words: Vec<String> = req
        .words
        .iter()
        .map(|w| w.trim().to_string())
        .filter(|w| !w.is_empty())
        .collect();

    let docs_guard = state.docs.read().expect("RwLock poisoned");

    let results: Vec<WordResult> = if words.len() <= 1 {
        // -------- 1 kata -> TIDAK parallel (single thread) --------
        words
            .iter()
            .map(|w| search_single_word(&docs_guard, w))
            .collect()
    } else {
        // -------- >= 2 kata -> parallel, pakai Rayon (multi-thread) --------
        words
            .par_iter() // <- di sini multi-thread jalan
            .map(|w| search_single_word(&docs_guard, w))
            .collect()
    };

    Json(SearchResponse { results })
}

// =================== ROCKET + CORS ===================

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
        })
        .mount("/api", routes![upload_files, list_docs, search])
        .attach(cors)
}

#[launch]
fn rocket() -> _ {
    build_rocket()
}
