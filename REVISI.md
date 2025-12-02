# Text Search Tool - Revisi

## Perubahan Yang Dilakukan

### 1. ✅ Support PDF Files
- **Sebelumnya**: Hanya bisa membaca file `.txt`
- **Sekarang**: Fokus hanya pada file `.pdf` (TXT support dihapus)
- **Implementasi**: 
  - Menambahkan dependency `pdf-extract` dan `base64` di `Cargo.toml`
  - PDF di-upload sebagai base64 dari frontend
  - Backend selalu mengekstrak teks dari PDF menggunakan `pdf-extract`
  - File handler di `utils/pdf_handler.rs`
  - Frontend hanya accept `.pdf` files

### 2. ✅ Single Text Field untuk Search
- **Sebelumnya**: Limited 2 kata dalam 2 input field terpisah
- **Sekarang**: 1 text field yang bisa menerima banyak kata
- **Contoh**: Input "Kami Tidur Makan Nasi" akan mencari kata: Kami, Tidur, Makan, Nasi
- **Implementasi**:
  - `SearchRequest` sekarang menerima `query: String`
  - Backend memisahkan query berdasarkan spasi
  - Function `split_query_into_words()` di `services/search_service.rs`

### 3. ✅ Benchmark Parallel vs Sequential
- **Feature**: Setiap search menjalankan 2 mode: parallel dan sequential
- **Output**: 
  - `parallel_ms`: waktu dengan multiprocessing (Rayon)
  - `sequential_ms`: waktu tanpa multiprocessing
  - `speedup`: perbandingan kecepatan (sequential/parallel)
- **Ditampilkan**: Di frontend dengan visual yang menarik
- **Implementasi**: 
  - `search_words_parallel()` dan `search_words_sequential()` di `services/search_service.rs`
  - Response mengembalikan `BenchmarkTiming` struct

### 4. ✅ Struktur Modular
**Sebelumnya**: Semua kode dalam 1 file `main.rs` (~350 lines)

**Sekarang**: Terstruktur dalam beberapa modul:
```
src/
├── main.rs                 (hanya setup & launch Rocket)
├── models/
│   ├── mod.rs
│   ├── document.rs        (Document, DocumentInfo, UploadedFile)
│   ├── request.rs         (SearchRequest)
│   └── response.rs        (semua response structs)
├── routes/
│   ├── mod.rs
│   ├── document_routes.rs (upload, list, delete endpoints)
│   └── search_routes.rs   (search endpoint)
├── services/
│   ├── mod.rs
│   ├── document_service.rs (business logic untuk dokumen)
│   └── search_service.rs   (business logic untuk search)
└── utils/
    ├── mod.rs
    ├── text_processor.rs   (tokenize, normalize)
    └── pdf_handler.rs      (extract PDF text)
```

**Keuntungan**:
- ✅ Mudah di-maintain
- ✅ Separation of concerns
- ✅ Testable
- ✅ Framework-independent business logic

### 5. ✅ Eliminasi Mutable Variables
**Sebelumnya**: Menggunakan `mut` di beberapa tempat seperti:
```rust
let mut docs = state.docs.write().expect("RwLock poisoned");
docs.retain(|d| d.id != id);
```

**Sekarang**: Menggunakan functional approach:
```rust
let filtered_docs = filter_documents_by_id(current_docs, id);
*state.docs.write().expect("RwLock poisoned") = filtered_docs;
```

**Implementasi**:
- Function `filter_documents_by_id()` mengembalikan Vec baru
- Tidak memodifikasi data in-place
- Lebih functional programming style

### 6. ✅ Framework-Independent Business Logic
**Sebelumnya**: Business logic tercampur dengan Rocket code

**Sekarang**: 
- **Services layer**: Pure functions, tidak depend pada Rocket
- **Routes layer**: Hanya handle HTTP (parsing, response formatting)

**Contoh**:
```rust
// services/search_service.rs - Pure function
pub fn search_single_word(docs: &[Document], raw_word: &str) -> WordResult {
    // ... pure logic
}

// routes/search_routes.rs - HTTP handler
#[post("/search", format = "json", data = "<req>")]
pub fn search(state: &State<AppState>, req: Json<SearchRequest>) -> Json<SearchResponse> {
    // ... call service functions
}
```

**Keuntungan**:
- ✅ Mudah ganti framework (Actix, Axum, etc)
- ✅ Testable tanpa HTTP context
- ✅ Reusable logic

### 7. ✅ Clean Code - No Duplication
- Repeated patterns direfactor ke functions
- Upload processing di `process_uploaded_files()`
- Doc stats calculation di `calculate_doc_stats()`
- Search logic terpusat di `search_service.rs`

### 8. ✅ Frontend Updates
- **Single text field** untuk multiple words
- **PDF upload support** (PDF icon berbeda)
- **Benchmark display**: Menampilkan perbandingan waktu parallel vs sequential
- **Responsive design** dengan Tailwind CSS
- **Visual feedback**: Loading states, error messages, animations

## Cara Menjalankan

### Backend (Rust + Rocket)
```powershell
cd text-search-api
cargo build --release
cargo run
```
Server akan jalan di `http://localhost:8000`

### Frontend (Vue.js)
```powershell
cd text-search-ui
npm install
npm run dev
```
UI akan jalan di `http://localhost:5173`

## Teknologi
- **Backend**: Rust, Rocket 0.5, Rayon (parallel processing), pdf-extract
- **Frontend**: Vue.js 3, Vite, Tailwind CSS
- **Architecture**: Clean architecture, modular, framework-independent

## Performa
Benchmark secara real-time menunjukkan:
- **Untuk multiple words (>1)**: Parallel processing biasanya 1.5x - 3x lebih cepat
- **Untuk single word**: Sequential sama cepat (overhead parallelization)
- **Ditampilkan langsung** di UI setiap kali search

## File Structure Changes
```
text-search-api/
├── Cargo.toml              (+ pdf-extract, base64)
└── src/
    ├── main.rs             (minimal, hanya setup)
    ├── models/             (NEW: data structures)
    ├── routes/             (NEW: HTTP handlers)
    ├── services/           (NEW: business logic)
    └── utils/              (NEW: helpers)

text-search-ui/
└── src/
    ├── api.js              (modified: single query param)
    └── views/
        └── HomePage.vue    (rewritten: PDF support, benchmark display)
```

## Best Practices Implemented
1. ✅ **Separation of Concerns**: Models, Routes, Services, Utils terpisah
2. ✅ **Pure Functions**: Business logic tidak depend pada framework
3. ✅ **Immutability**: Menghindari mutable state
4. ✅ **DRY Principle**: No code duplication
5. ✅ **Error Handling**: Proper Result types dan error messages
6. ✅ **Performance**: Parallel processing dengan benchmark
7. ✅ **User Experience**: Responsive UI dengan feedback
