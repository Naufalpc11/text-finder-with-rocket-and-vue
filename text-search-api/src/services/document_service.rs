use crate::models::document::{Document, DocId};
use crate::utils::{build_word_counts, extract_text_from_pdf};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::panic;

pub fn load_pdfs_from_dataset(dataset_path: &str) -> Vec<Document> {
    let path = Path::new(dataset_path);
    
    if !path.exists() || !path.is_dir() {
        eprintln!("Warning: Dataset folder not found at {}", dataset_path);
        return Vec::new();
    }

    let pdf_files: Vec<_> = fs::read_dir(path)
        .expect("Failed to read dataset directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("pdf"))
                .unwrap_or(false)
        })
        .collect();

    println!("Found {} PDF files in dataset folder", pdf_files.len());

    // Process files sequentially to avoid panic in parallel threads
    let processed: Vec<(String, String, HashMap<String, usize>)> = pdf_files
        .iter()
        .filter_map(|entry| process_pdf_file(&entry.path()))
        .collect();

    processed
        .into_iter()
        .enumerate()
        .map(|(idx, (name, content, word_counts))| {
            create_document(idx, name, content, word_counts)
        })
        .collect()
}

fn process_pdf_file(path: &Path) -> Option<(String, String, HashMap<String, usize>)> {
    let filename = path.file_name()?.to_str()?.to_string();
    
    println!("Loading PDF: {}", filename);
    //membaca seluruh isi file ke dalam vec<u8> sebelumnya
    let file_bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error reading file {}: {}", filename, e);
            return None;
        }
    };
    //encode u8 jadi string dari base64
    let base64_content = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &file_bytes);
    
    // Catch panic dari library pdf-extract
    let content = match panic::catch_unwind(|| extract_text_from_pdf(&base64_content)) {
        Ok(Ok(text)) => text,//simpan dalam konten
        Ok(Err(e)) => {
            eprintln!("Warning: Failed to extract text from {}: {}", filename, e);
            eprintln!("Skipping this file...");
            return None;
        }
        Err(_) => {
            eprintln!("Warning: PDF extraction panicked for {}", filename);
            eprintln!("Skipping this file...");
            return None;
        }
    };
    
    if content.trim().is_empty() {
        eprintln!("Warning: No text content in {}, skipping...", filename);
        return None;
    }
    
    //mulai menghitung kata yg sudh di normalisasi
    let word_counts = build_word_counts(&content);
    println!("Successfully loaded {} ({} words)", filename, word_counts.values().sum::<usize>());
    
    //return
    Some((filename, content, word_counts))
}

pub fn create_document(
    id: DocId,
    name: String,
    content: String,
    word_counts: HashMap<String, usize>,
) -> Document {
    Document {
        id,
        name,
        content,
        word_counts,
    }
}

//menghitung statistik dokumen (kek jumlah dokumen odf, total jumlah kata, ukuran teks dll)
pub fn calculate_doc_stats(docs: &[Document]) -> (usize, usize, usize, f64) {
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
