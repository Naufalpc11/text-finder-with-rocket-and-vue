#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket, State};
use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::sync::RwLock;

// ==== Model dokumen yang disimpan di server ====

#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
struct Document {
    id: usize,
    name: String,
    // untuk sekarang kita simpan full content (nanti bisa dipakai buat indexing)
    content: String,
}

// info ringan untuk dikirim ke frontend
#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
struct DocumentInfo {
    id: usize,
    name: String,
}

// payload yang dikirim dari frontend saat upload
#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
struct UploadedFile {
    name: String,
    content: String,
}

// state global aplikasi
struct AppState {
    docs: RwLock<Vec<Document>>,
}

// respon setelah upload
#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")]
struct UploadResponse {
    total_files: usize,
    doc_ids: Vec<usize>,
}

// ==== ROUTES ====

#[post("/upload", format = "json", data = "<files>")]
async fn upload_files(
    state: &State<AppState>,
    files: Json<Vec<UploadedFile>>,
) -> Json<UploadResponse> {
    let mut docs = state.docs.write().expect("RwLock poisoned");
    let mut new_ids = Vec::new();

    for f in files.iter() {
        let id = docs.len(); // id sederhana berdasarkan index
        let doc = Document {
            id,
            name: f.name.clone(),
            content: f.content.clone(),
        };
        docs.push(doc);
        new_ids.push(id);
    }

    Json(UploadResponse {
        total_files: docs.len(),
        doc_ids: new_ids,
    })
}

// route kecil buat cek apa dokumen sudah tersimpan
#[get("/docs")]
fn list_docs(state: &State<AppState>) -> Json<Vec<DocumentInfo>> {
    let docs = state.docs.read().expect("RwLock poisoned");
    let list = docs
        .iter()
        .map(|d| DocumentInfo {
            id: d.id,
            name: d.name.clone(),
        })
        .collect();
    Json(list)
}

// ==== SETUP ROCKET + CORS ====

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
        .mount("/api", routes![upload_files, list_docs])
        .attach(cors)
}

#[launch]
fn rocket() -> _ {
    build_rocket()
}
