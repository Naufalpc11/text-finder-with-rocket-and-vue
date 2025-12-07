#[macro_use]
extern crate rocket;

mod models;
mod routes;
mod services;
mod utils;

use models::Document;
use rocket::{Build, Rocket};
use rocket_cors::{AllowedOrigins, CorsOptions};
use services::load_pdfs_from_dataset;
use std::sync::{
    RwLock,
    atomic::AtomicUsize,
};

pub struct AppState {
    pub docs: RwLock<Vec<Document>>,
    pub next_id: AtomicUsize,
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

    // Load PDF dari dataset folder pas startup
    println!("Loading PDFs from dataset folder...");
    let dataset_path = r"C:\FP\text-finder-with-rocket-and-vue\dataset";
    let documents = load_pdfs_from_dataset(dataset_path);
    let doc_count = documents.len();
    
    println!("Successfully loaded {} PDF files", doc_count);

    rocket::build()
        .manage(AppState {
            docs: RwLock::new(documents),
            next_id: AtomicUsize::new(doc_count),
        })
        .mount(
            "/",
            routes![
                routes::list_docs,
                routes::get_stats,
                routes::search,
            ],
        )
        .attach(cors)
}

#[launch]
fn rocket() -> _ {
    build_rocket()
}
