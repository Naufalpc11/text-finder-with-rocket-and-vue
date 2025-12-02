# Migration Guide: Monolithic → Modular Structure

## Overview

Program ini di-refactor dari **1 file monolithic** (`mainss.rs` - 369 baris) menjadi **struktur modular** (9 files terpisah).

**Tujuan Refactoring:**
- ✅ Separation of concerns (pemisahan tanggung jawab)
- ✅ Easier maintenance dan testing
- ✅ Framework-independent business logic
- ✅ Better code organization

---

## 📊 Complete Migration Table

### **1. Data Structures (Models)**

| Kode di `mainss.rs` | Baris Lama | Lokasi Baru | Alasan Pemindahan |
|---------------------|------------|-------------|-------------------|
| `type DocId = usize` | 16 | `models/document.rs` | Type alias untuk Document ID |
| `struct Document` | 18-25 | `models/document.rs` | Core data model |
| `struct DocumentInfo` | 27-32 | `models/document.rs` | Response model untuk list docs |
| `struct UploadedFile` | 34-39 | `models/document.rs` | Request model untuk upload |
| `struct AppState` | 40-43 | `main.rs` | Global state (tetap di main) |
| `struct UploadResponse` | 45-50 | `models/response.rs` | Response model untuk upload |
| `struct SearchRequest` | 52-57 | `models/request.rs` | Request model untuk search |
| `struct PerDocCount` | 59-65 | `models/response.rs` | Nested response model |
| `struct WordResult` | 67-73 | `models/response.rs` | Response model untuk word result |
| `struct SearchResponse` | 75-79 | `models/response.rs` | Response model untuk search |
| `struct DeleteResponse` | 81-86 | `models/response.rs` | Response model untuk delete |
| `struct DeleteAllResponse` | 88-93 | `models/response.rs` | Response model untuk delete all |

**Perubahan:**
- ✅ `SearchRequest` diubah dari `words: Vec<String>` menjadi `query: String`
- ✅ `SearchResponse` ditambah field `benchmark: BenchmarkTiming`
- ✅ Ditambah struct baru: `BenchmarkTiming` untuk performance metrics

---

### **2. Utility Functions (Helper Functions)**

| Kode di `mainss.rs` | Baris Lama | Lokasi Baru | Alasan Pemindahan |
|---------------------|------------|-------------|-------------------|
| `fn normalize_token()` | 95-101 | `utils/text_processor.rs` | Text processing utility |
| `fn tokenize()` | 103-108 | `utils/text_processor.rs` | Text processing utility |
| `fn build_word_counts()` | 110-117 | `utils/text_processor.rs` | Text processing utility |

**Perubahan:**
- ✅ Semua function dibuat `pub` agar bisa diakses dari module lain
- ✅ Ditambah file baru: `utils/pdf_handler.rs` dengan function `extract_text_from_pdf()`

---

### **3. Business Logic Functions (Services)**

| Kode di `mainss.rs` | Baris Lama | Lokasi Baru | Alasan Pemindahan |
|---------------------|------------|-------------|-------------------|
| `fn count_total_occurrences()` | 119-121 | **DIHAPUS** | Diganti dengan `.sum()` langsung di search logic |
| `fn filter_docs_with_word()` | 123-127 | **DIHAPUS** | Logic dipindah ke `search_single_word()` |
| `fn count_word()` (recursive) | 129-141 | **DIHAPUS** | Tidak dipakai lagi (hanya untuk debug) |
| `fn calculate_doc_stats()` | 143-158 | `services/document_service.rs` | Document-related business logic |
| `fn search_single_word()` | 160-195 | `services/search_service.rs` | Search business logic |
| Logic di `upload_files()` | 202-218 | `services/document_service.rs` → `process_uploaded_files()` | Extract business logic dari route handler |

**Perubahan:**
- ✅ `search_single_word()` disederhanakan, menghapus recursive function
- ✅ Ditambah function baru: `search_words_parallel()` dan `search_words_sequential()`
- ✅ Ditambah function baru: `split_query_into_words()` untuk memisahkan query string
- ✅ Ditambah function baru: `filter_documents_by_id()` untuk delete operation
- ✅ Ditambah function baru: `process_single_file()` untuk PDF processing

---

### **4. Route Handlers (HTTP Endpoints)**

| Kode di `mainss.rs` | Baris Lama | Lokasi Baru | Alasan Pemindahan |
|---------------------|------------|-------------|-------------------|
| `#[post("/upload")]` | 197-241 | `routes/document_routes.rs` | Document-related endpoint |
| `#[get("/docs")]` | 243-255 | `routes/document_routes.rs` | Document-related endpoint |
| `#[get("/stats")]` | 257-268 | `routes/document_routes.rs` | Document-related endpoint |
| `#[post("/search")]` | 270-294 | `routes/search_routes.rs` | Search-related endpoint |
| `#[delete("/docs/<id>")]` | 297-319 | `routes/document_routes.rs` | Document-related endpoint |
| `#[delete("/docs")]` | 321-330 | `routes/document_routes.rs` | Document-related endpoint |

**Perubahan:**
- ✅ Route handlers dipecah berdasarkan domain (document vs search)
- ✅ Business logic diekstrak ke services layer
- ✅ Route handlers hanya handle HTTP layer (request/response)
- ✅ `upload_files()` sekarang immutable, tidak ada `mut`
- ✅ `delete_doc()` menggunakan `filter_documents_by_id()` service
- ✅ `search()` sekarang menjalankan parallel DAN sequential untuk benchmark

---

### **5. Application Setup**

| Kode di `mainss.rs` | Baris Lama | Lokasi Baru | Alasan Pemindahan |
|---------------------|------------|-------------|-------------------|
| `fn build_rocket()` | 332-364 | `main.rs` | Application setup (tetap di main) |
| `#[launch] fn rocket()` | 366-369 | `main.rs` | Entry point (tetap di main) |

**Perubahan:**
- ✅ Ditambah konfigurasi `Limits` untuk menaikkan limit JSON/file (50MB)
- ✅ CORS configuration tetap sama

---

## 📁 New File Structure

```
mainss.rs (369 lines)  →  Dipecah menjadi:

src/
├── main.rs                    (69 lines)  ← Setup & launch only
│
├── models/
│   ├── mod.rs                 (5 lines)   ← Module exports
│   ├── document.rs            (25 lines)  ← Document, DocumentInfo, UploadedFile
│   ├── request.rs             (6 lines)   ← SearchRequest
│   └── response.rs            (48 lines)  ← All response structs
│
├── routes/
│   ├── mod.rs                 (8 lines)   ← Module exports
│   ├── document_routes.rs     (101 lines) ← Upload, list, stats, delete endpoints
│   └── search_routes.rs       (48 lines)  ← Search endpoint with benchmark
│
├── services/
│   ├── mod.rs                 (8 lines)   ← Module exports
│   ├── document_service.rs    (78 lines)  ← Document processing logic
│   └── search_service.rs      (90 lines)  ← Search algorithms
│
└── utils/
    ├── mod.rs                 (5 lines)   ← Module exports
    ├── text_processor.rs      (31 lines)  ← Tokenize, normalize
    └── pdf_handler.rs         (18 lines)  ← PDF extraction (NEW)
```

**Total:** 9 files modular vs 1 file monolithic

---

## 🔄 Key Changes Summary

### **Removed (Dihapus):**
- ❌ `fn count_total_occurrences()` → Replaced with direct `.sum()`
- ❌ `fn filter_docs_with_word()` → Merged into search logic
- ❌ `fn count_word()` recursive → Removed (debug only)
- ❌ Mutable variables (`mut docs`) → Replaced with immutable approach

### **Added (Ditambahkan):**
- ✅ `utils/pdf_handler.rs` → PDF text extraction
- ✅ `services/search_service.rs` → `split_query_into_words()`, `search_words_parallel()`, `search_words_sequential()`
- ✅ `services/document_service.rs` → `process_uploaded_files()`, `filter_documents_by_id()`, `process_single_file()`
- ✅ `models/response.rs` → `BenchmarkTiming` struct
- ✅ Benchmark timing untuk parallel vs sequential comparison

### **Modified (Dimodifikasi):**
- 🔄 `SearchRequest`: `words: Vec<String>` → `query: String`
- 🔄 `SearchResponse`: Added `benchmark: BenchmarkTiming`
- 🔄 `upload_files()`: Extracted business logic, made immutable
- 🔄 `delete_doc()`: Using functional approach with `filter_documents_by_id()`
- 🔄 `search()`: Now runs both parallel and sequential with timing

---

## 💡 Benefits of Modular Structure

| Aspect | Monolithic (`mainss.rs`) | Modular (Current) |
|--------|-------------------------|-------------------|
| **Lines per file** | 369 lines | 18-101 lines (average ~40) |
| **Testability** | ❌ Hard to test | ✅ Easy to test each module |
| **Maintainability** | ❌ Hard to find code | ✅ Clear organization |
| **Reusability** | ❌ Coupled to Rocket | ✅ Services are framework-independent |
| **Collaboration** | ❌ Merge conflicts | ✅ Multiple people can work on different modules |
| **Understanding** | ❌ Need to read everything | ✅ Can focus on one module at a time |

---

## 🚀 Architecture Layers

### **Before (Monolithic)**
```
┌─────────────────────────────┐
│       mainss.rs             │
│  • Data structures          │
│  • Business logic           │
│  • HTTP handlers            │
│  • Utilities                │
│  All mixed together ❌      │
└─────────────────────────────┘
```

### **After (Clean Architecture)**
```
┌───────────────────────────────────────┐
│  Routes (HTTP Layer)                  │  ← Rocket-dependent
│  • HTTP request/response handling     │
└───────────────┬───────────────────────┘
                │
┌───────────────▼───────────────────────┐
│  Services (Business Logic)            │  ← Framework-independent ✅
│  • Pure functions                     │
│  • Search algorithms                  │
│  • Document processing                │
└───────────────┬───────────────────────┘
                │
┌───────────────▼───────────────────────┐
│  Models (Data Structures)             │
│  • Request/Response types             │
└───────────────────────────────────────┘
                │
┌───────────────▼───────────────────────┐
│  Utils (Helper Functions)             │
│  • Text processing                    │
│  • PDF extraction                     │
└───────────────────────────────────────┘
```

---

## 📝 Migration Checklist

- [x] Struct definitions → `models/`
- [x] HTTP routes → `routes/`
- [x] Business logic → `services/`
- [x] Utility functions → `utils/`
- [x] Eliminate mutable variables
- [x] Framework-independent services
- [x] Add PDF support
- [x] Add benchmark comparison
- [x] Single text field for search
- [x] Documentation (README, QUICKSTART, etc.)

**Status: ✅ COMPLETE**

---

## 🔍 Example: Search Flow Comparison

### **Before (mainss.rs)**
```rust
#[post("/search", format = "json", data = "<req>")]
fn search(state: &State<AppState>, req: Json<SearchRequest>) -> Json<SearchResponse> {
    let words = req.words.clone();  // Multiple words array
    let docs_guard = state.docs.read().expect("RwLock poisoned");
    
    // Direct logic in route handler
    let results: Vec<WordResult> = if words.len() <= 1 {
        words.iter().map(|w| search_single_word(&docs_guard, w)).collect()
    } else {
        words.par_iter().map(|w| search_single_word(&docs_guard, w)).collect()
    };
    
    Json(SearchResponse { results })  // No benchmark
}
```

### **After (Modular)**
```rust
// routes/search_routes.rs (HTTP Layer)
#[post("/search", format = "json", data = "<req>")]
pub async fn search(state: &State<AppState>, req: Json<SearchRequest>) -> Json<SearchResponse> {
    let query = &req.query;  // Single query string
    let words = split_query_into_words(query);
    let docs_guard = state.docs.read().expect("RwLock poisoned");
    
    // Parallel with timing
    let start_parallel = Instant::now();
    let results_parallel = search_words_parallel(&docs_guard, &words);
    let parallel_ms = start_parallel.elapsed().as_secs_f64() * 1000.0;
    
    // Sequential with timing
    let start_sequential = Instant::now();
    let _results_sequential = search_words_sequential(&docs_guard, &words);
    let sequential_ms = start_sequential.elapsed().as_secs_f64() * 1000.0;
    
    // Calculate speedup
    let speedup = sequential_ms / parallel_ms;
    
    Json(SearchResponse {
        results: results_parallel,
        benchmark: BenchmarkTiming { parallel_ms, sequential_ms, speedup },
    })
}

// services/search_service.rs (Business Logic - Framework Independent)
pub fn search_words_parallel(docs: &[Document], words: &[String]) -> Vec<WordResult> {
    words.par_iter().map(|w| search_single_word(docs, w)).collect()
}

pub fn search_words_sequential(docs: &[Document], words: &[String]) -> Vec<WordResult> {
    words.iter().map(|w| search_single_word(docs, w)).collect()
}
```

**Benefits:**
- ✅ Route handler only handles HTTP concerns
- ✅ Business logic in separate functions (testable!)
- ✅ Benchmark comparison built-in
- ✅ Single query string (more user-friendly)

---

## 🎓 Lessons Learned

1. **Separation of Concerns**: Each module has one responsibility
2. **Framework Independence**: Services don't depend on Rocket
3. **Immutability**: Functional approach reduces bugs
4. **Modularity**: Easy to add new features (e.g., PDF support)
5. **Testability**: Each function can be tested independently

**Conclusion**: Modular structure makes the codebase more maintainable, testable, and scalable! 🚀
