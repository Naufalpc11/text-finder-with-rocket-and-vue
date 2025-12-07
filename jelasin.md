# Penjelasan Lengkap Text-Search-API Backend

## Daftar Isi
1. [Gambaran Umum](#gambaran-umum)
2. [Struktur Folder & Arsitektur](#struktur-folder--arsitektur)
3. [Penjelasan Per File/Modul](#penjelasan-per-filemodul)
4. [Konsep Penting](#konsep-penting)
5. [Alur Kerja Program](#alur-kerja-program)

---

## Gambaran Umum

**Text-Search-API** adalah backend yang ditulis dengan bahasa Rust menggunakan framework Rocket. Program ini berfungsi untuk:
- Membaca file PDF otomatis dari folder dataset saat aplikasi pertama kali dijalankan
- Menyimpan isi PDF di memori (RAM) agar pencarian lebih cepat
- Mencari kata-kata dalam dokumen secara paralel (menggunakan banyak CPU core sekaligus)
- Membandingkan kecepatan pencarian paralel vs sequential
- Memberikan data ke frontend melalui API (HTTP endpoints)

**Kenapa menggunakan Rust?**
- Performa sangat cepat (setara dengan C/C++)
- Aman dari memory leak dan data race
- Mendukung pemrograman fungsional
- Library Rayon memudahkan pemrosesan paralel

**Kenapa menggunakan Rocket?**
- Framework web yang mudah digunakan di Rust
- Routing yang simple dan type-safe
- Built-in JSON serialization/deserialization
- Cocok untuk membuat REST API

---

## Struktur Folder & Arsitektur

### Mengapa Kode Dipecah Menjadi Modular?

**Sebelum Revisi:** Semua kode ada dalam 1 file `main.rs` (369 baris) - sulit dibaca, sulit di-maintain.

**Setelah Revisi:** Dipecah menjadi beberapa modul berdasarkan tanggung jawab (Separation of Concerns):

```
text-search-api/
├── Cargo.toml                 # File konfigurasi project & dependencies
└── src/
    ├── main.rs                # Entry point aplikasi (setup & launch)
    │
    ├── models/                # Data structures (struct-struct)
    │   ├── mod.rs
    │   ├── document.rs        # Model untuk Document
    │   ├── request.rs         # Model untuk HTTP request
    │   └── response.rs        # Model untuk HTTP response
    │
    ├── routes/                # HTTP handlers (terima request, kirim response)
    │   ├── mod.rs
    │   ├── document_routes.rs # Endpoints: list, stats
    │   └── search_routes.rs   # Endpoint: search
    │
    ├── services/              # Business logic (TIDAK depend ke Rocket)
    │   ├── mod.rs
    │   ├── document_service.rs # Logic pemrosesan dokumen
    │   └── search_service.rs   # Logic pencarian kata
    │
    └── utils/                 # Helper functions
        ├── mod.rs
        ├── text_processor.rs  # Normalisasi & tokenisasi teks
        └── pdf_handler.rs     # Ekstraksi teks dari PDF
```

**Keuntungan Struktur Modular:**
1. **Mudah dipahami** - Setiap file punya tanggung jawab yang jelas
2. **Mudah di-maintain** - Kalau ada bug, tinggal cari di modul yang bersangkutan
3. **Mudah di-test** - Bisa test setiap modul secara terpisah
4. **Framework Independent** - Business logic di `services/` tidak terikat dengan Rocket, jadi kalau mau ganti framework tinggal ganti layer `routes/` saja

---

## Penjelasan Per File/Modul

### 1. `main.rs` - Entry Point Aplikasi

**Fungsi:** Titik awal program, setup aplikasi, dan launch server.

**Isi File:**
```rust
#[macro_use]
extern crate rocket;

mod models;
mod routes;
mod services;
mod utils;
```
- `extern crate rocket` - Import library Rocket
- `mod models, routes, services, utils` - Deklarasi modul-modul yang akan digunakan

```rust
pub struct AppState {
    pub docs: RwLock<Vec<Document>>,
    pub next_id: AtomicUsize,
}
```
**AppState** adalah "database sementara" di memori yang menyimpan:
- `docs` - Daftar semua dokumen (dibungkus `RwLock` agar thread-safe)
- `next_id` - Generator ID otomatis untuk dokumen baru

**❓ Kenapa pakai RwLock?**
- Karena aplikasi ini multi-threaded (banyak request bisa datang bersamaan)
- `RwLock` memastikan tidak ada 2 thread yang menulis data bersamaan (mencegah data corruption)
- `RwLock` membolehkan banyak thread membaca data secara bersamaan (read lock)

**❓ Kenapa pakai AtomicUsize?**
- Untuk increment ID secara aman di multi-threaded environment
- Tanpa `Atomic`, bisa terjadi 2 thread dapat ID yang sama

```rust
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
```
**CORS Configuration** - Mengizinkan frontend (Vue.js yang jalan di port 5173) untuk akses API ini.

**❓ Apa itu CORS?**
- Cross-Origin Resource Sharing
- Browser block request dari domain berbeda secara default
- CORS configuration memberikan izin ke domain tertentu

```rust
    let dataset_path = r"d:\Semester 5\PF\Tubes\Text-Finder\dataset";
    let documents = load_pdfs_from_dataset(dataset_path);
    let doc_count = documents.len();
```
**Auto-load PDFs** - Saat aplikasi start, langsung load semua PDF dari folder dataset.

**❓ Kenapa load di awal, bukan saat user upload?**
- Lebih cepat untuk demo/testing
- User tidak perlu upload manual
- File PDF sudah siap untuk di-search

```rust
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
```
**Setup Rocket:**
- `.manage()` - Simpan AppState sebagai global state
- `.mount()` - Daftarkan semua endpoints
- `.attach()` - Pasang CORS middleware

```rust
#[launch]
fn rocket() -> _ {
    build_rocket()
}
```
**Launch** - Macro `#[launch]` membuat function ini jadi entry point Rocket.

---

### 2. `models/` - Data Structures

#### `models/document.rs`

**Fungsi:** Mendefinisikan struktur data untuk Document.

```rust
pub type DocId = usize;
```
**Type Alias** - Membuat nama yang lebih jelas untuk ID dokumen (daripada pakai `usize` langsung).

```rust
#[derive(Debug, Clone)]
pub struct Document {
    pub id: DocId,
    pub name: String,
    pub content: String,
    pub word_counts: HashMap<String, usize>,
}
```
**Document** adalah struktur data utama yang menyimpan:
- `id` - Identifier unik dokumen
- `name` - Nama file (contoh: "Buku.pdf")
- `content` - Isi teks lengkap dari PDF
- `word_counts` - HashMap yang menyimpan jumlah kemunculan setiap kata
  - Key: kata (contoh: "komputer")
  - Value: jumlah kemunculan (contoh: 15)

**❓ Kenapa simpan word_counts?**
- Agar tidak perlu hitung ulang setiap kali search
- Pre-computed untuk performa lebih cepat
- Trade-off: pakai lebih banyak memori, tapi search lebih cepat

**❓ Apa itu HashMap?**
- Struktur data key-value pair
- Lookup sangat cepat (O(1))
- Contoh: `{"komputer": 15, "data": 8, "informasi": 12}`

```rust
#[derive(Debug, Clone, Serialize)]
pub struct DocumentInfo {
    pub id: DocId,
    pub name: String,
}
```
**DocumentInfo** - Versi ringan dari Document, hanya ID dan nama (untuk endpoint `/docs`).

**❓ Kenapa tidak kirim Document lengkap?**
- Hemat bandwidth
- Frontend hanya butuh list nama file, tidak butuh isi lengkap

---

#### `models/request.rs`

**Fungsi:** Struktur data untuk HTTP request yang diterima dari frontend.

```rust
#[derive(Debug, Clone, Deserialize)]
pub struct SearchRequest {
    pub query: String,
}
```
**SearchRequest** - Request untuk endpoint `/search`.

**Sebelum Revisi:**
```rust
pub struct SearchRequest {
    pub words: Vec<String>,  // ["kata1", "kata2"]
}
```

**Setelah Revisi:**
```rust
pub struct SearchRequest {
    pub query: String,  // "kata1 kata2 kata3"
}
```

**❓ Kenapa diubah?**
- Lebih user-friendly - user tinggal ketik semua kata dalam 1 field
- Tidak ada limit jumlah kata
- Frontend lebih simple (tidak perlu array)

**❓ Apa itu Deserialize?**
- Proses mengubah JSON menjadi struct Rust
- Contoh: `{"query": "komputer data"}` → `SearchRequest { query: "komputer data" }`
- Library Serde yang handle otomatis

---

#### `models/response.rs`

**Fungsi:** Struktur data untuk HTTP response yang dikirim ke frontend.

```rust
derive_response!(pub struct PerDocCount {
    pub doc_id: DocId,
    pub doc_name: String,
    pub count: usize,
    pub snippets: Vec<String>,
});
```
**PerDocCount** - Info kemunculan kata di 1 dokumen:
- `doc_id` - ID dokumen
- `doc_name` - Nama file
- `count` - Jumlah kemunculan kata
- `snippets` - Cuplikan kalimat yang mengandung kata (maks 3 cuplikan)

**❓ Apa itu derive_response macro?**
```rust
macro_rules! derive_response {
    ($item:item) => {
        #[derive(Debug, Clone, Serialize)]
        $item
    };
}
```
- Macro untuk mengurangi repetisi kode
- Otomatis menambahkan `#[derive(Debug, Clone, Serialize)]` ke setiap struct
- Lebih DRY (Don't Repeat Yourself)

```rust
derive_response!(pub struct WordResult {
    pub word: String,
    pub total_count: usize,
    pub per_doc: Vec<PerDocCount>,
});
```
**WordResult** - Hasil pencarian untuk 1 kata:
- `word` - Kata yang dicari (sudah di-normalize)
- `total_count` - Total kemunculan di semua dokumen
- `per_doc` - Detail per dokumen

```rust
derive_response!(pub struct BenchmarkTiming {
    pub parallel_ms: f64,
    pub sequential_ms: f64,
    pub speedup: f64,
});
```
**BenchmarkTiming** - Hasil perbandingan performa (revisi baru):
- `parallel_ms` - Waktu search dengan multiprocessing (milliseconds)
- `sequential_ms` - Waktu search tanpa multiprocessing (milliseconds)
- `speedup` - Berapa kali lebih cepat (sequential/parallel)

**❓ Kenapa ada benchmark?**
- Untuk menunjukkan keunggulan pemrosesan paralel
- Membuktikan bahwa Rayon benar-benar mempercepat pencarian
- Requirement dari revisi dosen

```rust
derive_response!(pub struct DocumentMatch {
    pub doc_id: DocId,
    pub doc_name: String,
    pub matched_words: usize,
});
```
**DocumentMatch** - Dokumen yang mengandung semua kata yang dicari.

```rust
derive_response!(pub struct SearchResponse {
    pub results: Vec<WordResult>,
    pub benchmark: BenchmarkTiming,
    pub docs_with_all_words: Vec<DocumentMatch>,
});
```
**SearchResponse** - Response utama dari endpoint `/search`:
- `results` - Hasil untuk setiap kata
- `benchmark` - Timing information
- `docs_with_all_words` - Dokumen yang mengandung semua kata

---

### 3. `utils/` - Helper Functions

#### `utils/text_processor.rs`

**Fungsi:** Fungsi-fungsi untuk memproses teks (normalisasi, tokenisasi, counting).

```rust
pub fn normalize_token(token: &str) -> String {
    token
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase()
}
```
**normalize_token** - Membersihkan dan standarisasi kata:
- Hapus simbol/tanda baca (`.`, `,`, `!`, `?`, dll)
- Lowercase semua huruf
- Contoh: `"Komputer!"` → `"komputer"`

**❓ Kenapa perlu normalisasi?**
- Agar "Komputer", "komputer", "KOMPUTER", "komputer!" dianggap sama
- Pencarian jadi case-insensitive
- Hasil lebih akurat

**❓ Bagaimana cara kerjanya?**
1. `.chars()` - Ubah string jadi iterator karakter
2. `.filter(|c| c.is_alphanumeric())` - Ambil hanya huruf dan angka
3. `.collect::<String>()` - Gabungkan karakter jadi string lagi
4. `.to_lowercase()` - Lowercase semua

**Ini adalah contoh pemrograman fungsional:**
- Tidak ada mutable variable
- Menggunakan method chaining
- Menggunakan higher-order function (filter)

```rust
pub fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(normalize_token)
        .filter(|w| !w.is_empty())
        .collect()
}
```
**tokenize** - Memecah teks menjadi array kata:
- `.split_whitespace()` - Pisah berdasarkan spasi, tab, newline
- `.map(normalize_token)` - Normalize setiap kata
- `.filter(|w| !w.is_empty())` - Buang kata kosong
- Contoh: `"Hello World!"` → `["hello", "world"]`

**Pemrograman fungsional:**
- `map` adalah higher-order function
- Tidak ada loop `for`
- Lebih deklaratif (menjelaskan "apa" bukan "bagaimana")

```rust
pub fn build_word_counts(text: &str) -> HashMap<String, usize> {
    tokenize(text)
        .into_iter()
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        })
}
```
**build_word_counts** - Menghitung frekuensi setiap kata:
- `tokenize(text)` - Pecah teks jadi array kata
- `.fold()` - Akumulasi hasil (seperti reduce di JavaScript)
- `HashMap::new()` - Nilai awal (HashMap kosong)
- `acc.entry(word).or_insert(0)` - Ambil value, kalau belum ada isi 0
- `+= 1` - Increment counter

**❓ Apa itu fold?**
- Higher-order function untuk akumulasi
- Mengubah collection menjadi 1 nilai
- Seperti "lipat" berkali-kali

**Contoh:**
```
Input: "hello world hello"
Output: {"hello": 2, "world": 1}
```

---

#### `utils/pdf_handler.rs`

**Fungsi:** Ekstraksi teks dari file PDF.

```rust
use base64::{Engine as _, engine::general_purpose};

pub fn extract_text_from_pdf(base64_content: &str) -> Result<String, String> {
    let pdf_bytes = general_purpose::STANDARD
        .decode(base64_content)
        .map_err(|e| format!("Failed to decode base64: {}", e))?;
    
    pdf_extract::extract_text_from_mem(&pdf_bytes)
        .map_err(|e| format!("Failed to extract PDF text: {}", e))
}
```

**extract_text_from_pdf** - Ubah PDF menjadi plain text:
1. Decode base64 string → binary data (Vec<u8>)
2. Extract text dari binary data menggunakan library `pdf-extract`

**❓ Kenapa pakai base64?**
- Frontend tidak bisa kirim binary data langsung dalam JSON
- Base64 adalah cara encode binary jadi string
- Backend decode kembali ke binary

**❓ Apa itu Result<String, String>?**
- Type untuk error handling di Rust
- `Ok(String)` jika berhasil
- `Err(String)` jika gagal
- Lebih aman daripada exception (tidak bisa lupa handle error)

**❓ Apa itu `?` operator?**
- Shortcut untuk error propagation
- Jika error, langsung return Err
- Jika Ok, lanjut ke baris berikutnya

**❓ Kenapa pakai pdf-extract library?**
- Parsing PDF itu kompleks (format binary yang rumit)
- Library sudah handle semua kompleksitas
- Kita tinggal panggil function

---

### 4. `services/` - Business Logic (Framework-Independent)

**Konsep Penting:** Layer ini TIDAK depend ke Rocket. Semua function bisa dipanggil tanpa HTTP request. Ini memudahkan testing dan memungkinkan ganti framework tanpa ubah business logic.

#### `services/document_service.rs`

**Fungsi:** Logic untuk memproses dan manage dokumen.

```rust
pub fn load_pdfs_from_dataset(dataset_path: &str) -> Vec<Document> {
    let path = Path::new(dataset_path);
    
    if !path.exists() || !path.is_dir() {
        eprintln!("Warning: Dataset folder not found at {}", dataset_path);
        return Vec::new();
    }
```
**load_pdfs_from_dataset** - Load semua PDF dari folder:
1. Cek apakah folder exist
2. Jika tidak, return empty Vec (tidak crash)

```rust
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
```
**Filter PDF files:**
- `fs::read_dir()` - Baca semua entry di folder
- `.filter_map(|entry| entry.ok())` - Ambil hanya yang berhasil (skip error)
- `.filter(...)` - Ambil hanya file dengan extension `.pdf` (case-insensitive)

**Pemrograman fungsional:**
- Menggunakan iterator chains
- `filter_map`, `filter` adalah higher-order functions
- Tidak ada mutable variable

```rust
    let processed: Vec<(String, String, HashMap<String, usize>)> = pdf_files
        .iter()
        .filter_map(|entry| process_pdf_file(&entry.path()))
        .collect();
```
**Process setiap file:**
- `.filter_map()` - Process file, skip jika gagal
- Return tuple: (nama_file, content, word_counts)

```rust
fn process_pdf_file(path: &Path) -> Option<(String, String, HashMap<String, usize>)> {
    let filename = path.file_name()?.to_str()?.to_string();
    
    let file_bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error reading file {}: {}", filename, e);
            return None;
        }
    };
    
    let base64_content = base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD, 
        &file_bytes
    );
```
**Proses 1 file PDF:**
1. Baca file menjadi bytes (Vec<u8>)
2. Encode bytes ke base64 string

```rust
    let content = match panic::catch_unwind(|| extract_text_from_pdf(&base64_content)) {
        Ok(Ok(text)) => text,
        Ok(Err(e)) => {
            eprintln!("Warning: Failed to extract text from {}: {}", filename, e);
            return None;
        }
        Err(_) => {
            eprintln!("Warning: PDF extraction panicked for {}", filename);
            return None;
        }
    };
```
**Extract text dengan safety:**
- `panic::catch_unwind()` - Tangkap panic dari library pdf-extract
- Jika panic, skip file (tidak crash seluruh aplikasi)
- Jika error, print warning dan skip

**❓ Kenapa perlu catch panic?**
- Library `pdf-extract` kadang panic untuk PDF yang corrupt/kompleks
- Tanpa `catch_unwind`, 1 PDF rusak bisa crash seluruh aplikasi
- Dengan ini, hanya skip file bermasalah

```rust
    let word_counts = build_word_counts(&content);
    println!("Successfully loaded {} ({} words)", filename, word_counts.values().sum::<usize>());
    
    Some((filename, content, word_counts))
}
```
**Hitung word counts dan return:**
- `build_word_counts()` - Hitung frekuensi kata
- Return `Some(...)` jika berhasil
- Return `None` jika gagal (sudah di-handle di atas)

**❓ Apa itu Option<T>?**
- Type untuk nilai yang mungkin ada atau tidak ada
- `Some(value)` jika ada
- `None` jika tidak ada
- Lebih aman daripada null pointer

```rust
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
```
**create_document** - Constructor untuk Document struct (pure function).

```rust
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
```
**calculate_doc_stats** - Hitung statistik dokumen:
- Total jumlah dokumen
- Total jumlah kata
- Total ukuran bytes
- Rata-rata kata per dokumen

**Pemrograman fungsional:**
- Menggunakan `.iter()`, `.map()`, `.sum()`
- Tidak ada mutable variable
- Pure function (tidak ada side effect)

---

#### `services/search_service.rs`

**Fungsi:** Logic untuk mencari kata dalam dokumen.

```rust
pub fn split_query_into_words(query: &str) -> Vec<String> {
    query
        .split_whitespace()
        .map(|w| w.trim().to_string())
        .filter(|w| !w.is_empty())
        .collect()
}
```
**split_query_into_words** - Pisah query string jadi array kata:
- Input: `"komputer data sistem"`
- Output: `["komputer", "data", "sistem"]`

**❓ Kenapa perlu function ini?**
- Setelah revisi, frontend kirim 1 string (bukan array)
- Backend harus split string jadi kata-kata
- Simple tapi penting

```rust
pub fn search_words_parallel(docs: &[Document], words: &[String]) -> Vec<WordResult> {
    words
        .par_iter()
        .map(|w| search_single_word(docs, w))
        .collect()
}
```
**search_words_parallel** - Cari banyak kata secara PARALEL:
- `.par_iter()` - Parallel iterator dari Rayon
- `.map(|w| search_single_word(...))` - Cari setiap kata
- Setiap kata diproses di thread berbeda

**❓ Apa itu par_iter?**
- Dari library Rayon
- Otomatis split kerja ke banyak thread
- Memanfaatkan semua CPU core
- API sama dengan `.iter()` biasa

```rust
pub fn search_words_sequential(docs: &[Document], words: &[String]) -> Vec<WordResult> {
    words
        .iter()
        .map(|w| search_single_word(docs, w))
        .collect()
}
```
**search_words_sequential** - Cari banyak kata secara SEQUENTIAL:
- `.iter()` - Iterator biasa (single-threaded)
- Kata dicari satu per satu

**❓ Kenapa ada 2 versi?**
- Untuk benchmark (requirement revisi)
- Membandingkan performa parallel vs sequential
- Membuktikan bahwa parallel lebih cepat

```rust
pub fn search_single_word(docs: &[Document], raw_word: &str) -> WordResult {
    let word = normalize_token(raw_word);
    
    let per_doc: Vec<PerDocCount> = docs
        .iter()
        .filter_map(|doc| {
            doc.word_counts.get(&word).copied().and_then(|count| {
                if count > 0 {
                    let snippets = extract_snippets(&doc.content, raw_word, 3);
                    Some(PerDocCount {
                        doc_id: doc.id,
                        doc_name: doc.name.clone(),
                        count,
                        snippets,
                    })
                } else {
                    None
                }
            })
        })
        .collect();
```
**search_single_word** - Cari 1 kata di semua dokumen:
1. Normalize kata yang dicari
2. Loop semua dokumen
3. Ambil count dari `word_counts` HashMap (O(1) - sangat cepat!)
4. Jika count > 0, extract snippets
5. Return PerDocCount

**❓ Kenapa pakai filter_map?**
- Combine filter + map dalam 1 step
- Return `Some(...)` untuk dokumen yang mengandung kata
- Return `None` untuk dokumen yang tidak mengandung (otomatis di-skip)

```rust
    let total_count = calculate_total_count(&per_doc);
    
    #[cfg(debug_assertions)]
    {
        let recursive_total = count_word_recursive(docs, &word, 0, 0);
        debug_assert_eq!(
            total_count, recursive_total,
            "Mismatch: iterative={} vs recursive={}",
            total_count, recursive_total
        );
    }
    
    WordResult {
        word,
        total_count,
        per_doc,
    }
}
```
**Return hasil:**
- `total_count` - Total kemunculan di semua dokumen
- `debug_assert_eq!` - Validasi hasil (hanya jalan saat development)

**❓ Apa itu #[cfg(debug_assertions)]?**
- Conditional compilation
- Code ini hanya jalan saat mode debug
- Saat release (production), code ini tidak ada
- Tidak mempengaruhi performa production

```rust
fn calculate_total_count(per_doc: &[PerDocCount]) -> usize {
    per_doc.iter().map(|pd| pd.count).sum()
}
```
**calculate_total_count** - Jumlahkan semua count (pemrograman fungsional).

```rust
fn extract_snippets(content: &str, search_word: &str, max_snippets: usize) -> Vec<String> {
    let normalized_search = normalize_token(search_word);
    
    content
        .split(|c| c == '.' || c == '!' || c == '?')
        .filter(|s| !s.trim().is_empty())
        .filter(|sentence| {
            let words: Vec<String> = sentence
                .split_whitespace()
                .map(|w| normalize_token(w))
                .collect();
            words.contains(&normalized_search)
        })
        .take(max_snippets)
        .map(|sentence| {
            let trimmed = sentence.trim();
            if trimmed.len() > 150 {
                format!("{}...", &trimmed[..150])
            } else {
                trimmed.to_string()
            }
        })
        .collect()
}
```
**extract_snippets** - Ambil cuplikan kalimat yang mengandung kata (PURELY FUNCTIONAL):
1. `.split()` - Pisah content jadi kalimat (berdasarkan `.`, `!`, `?`)
2. `.filter()` - Buang kalimat kosong
3. `.filter()` - Ambil hanya kalimat yang mengandung kata yang dicari
4. `.take(max_snippets)` - Ambil maksimal 3 cuplikan pertama
5. `.map()` - Format setiap kalimat (trim & potong jika > 150 karakter)
6. `.collect()` - Kumpulkan jadi Vec<String>

**❓ Kenapa ini lebih baik dari versi sebelumnya?**
- **TIDAK ADA `mut`** - Purely immutable
- **Declarative** - Lebih jelas "apa" yang dilakukan, bukan "bagaimana"
- **Iterator chains** - Konsep pemrograman fungsional
- **Lebih ringkas** - Dari ~35 baris jadi ~20 baris
- **No manual loop** - Menggunakan higher-order functions

**❓ Kenapa ada snippets?**
- Agar user bisa lihat konteks kata
- Tidak perlu baca seluruh dokumen
- User langsung tahu kata digunakan dalam konteks apa

**❓ Apa itu `.take()`?**
- Higher-order function untuk limit jumlah item
- Lebih functional daripada `if snippets.len() >= max`
- Otomatis stop setelah ambil N item

```rust
pub fn find_docs_with_all_words(docs: &[Document], words: &[String]) -> Vec<(usize, String, usize)> {
    if words.is_empty() {
        return Vec::new();
    }
    
    docs.iter()
        .filter_map(|doc| {
            let normalized_words: Vec<String> = words.iter()
                .map(|w| normalize_token(w))
                .collect();
            
            let matched_count = normalized_words.iter()
                .filter(|word| doc.word_counts.get(*word).copied().unwrap_or(0) > 0)
                .count();
            
            if matched_count == words.len() {
                Some((doc.id, doc.name.clone(), matched_count))
            } else {
                None
            }
        })
        .collect()
}
```
**find_docs_with_all_words** - Cari dokumen yang mengandung SEMUA kata:
1. Normalize semua kata
2. Cek setiap dokumen
3. Hitung berapa kata yang match
4. Jika semua kata match, return dokumen tersebut

**❓ Kenapa perlu function ini?**
- User mungkin cari dokumen yang membahas topik tertentu
- Contoh: cari "machine learning python" → mau dokumen yang bahas ketiganya
- Dokumen yang hanya ada 1-2 kata tidak relevan

```rust
fn count_word_recursive(docs: &[Document], word: &str, index: usize, acc: usize) -> usize {
    if index >= docs.len() {
        return acc;
    }
    
    let count = docs[index]
        .word_counts
        .get(word)
        .copied()
        .unwrap_or(0);
    
    count_word_recursive(docs, word, index + 1, acc + count)
}
```
**count_word_recursive** - Hitung total dengan recursion (untuk validasi):
- Base case: jika sudah sampai akhir, return accumulator
- Recursive case: ambil count dari dokumen ke-i, panggil function lagi dengan i+1

**❓ Kenapa ada function ini?**
- Untuk demonstrasi recursion (konsep pemrograman fungsional)
- Untuk validasi hasil iterative (pastikan hasil sama)
- Hanya jalan saat debug mode

**❓ Kenapa tidak dipakai untuk production?**
- Recursion lebih lambat dari iteration di Rust
- Bisa stack overflow kalau dokumen banyak
- Tapi bagus untuk demonstrasi konsep

---

### 5. `routes/` - HTTP Handlers (Rocket-Dependent)

**Konsep Penting:** Layer ini depend ke Rocket. Tugasnya hanya handle HTTP (terima request, panggil service, format response). Business logic ada di services.

#### `routes/document_routes.rs`

**Fungsi:** Endpoints untuk manage dokumen.

```rust
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
```
**GET /docs** - Ambil list semua dokumen:
1. `.read()` - Acquire read lock (banyak thread bisa read bersamaan)
2. `.iter().map()` - Transform Document → DocumentInfo
3. `Json(...)` - Convert ke JSON response

**❓ Apa itu State<AppState>?**
- Rocket inject global state ke function
- Otomatis, tidak perlu passing manual
- Type-safe

**❓ Kenapa tidak kirim Document lengkap?**
- Hemat bandwidth
- Frontend hanya butuh nama file untuk ditampilkan

```rust
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
```
**GET /stats** - Ambil statistik dokumen:
- Panggil `calculate_doc_stats()` dari services
- Format jadi JSON object
- Rocket otomatis set Content-Type: application/json

**❓ Apa itu serde_json::json!?**
- Macro untuk buat JSON object dengan syntax yang clean
- Otomatis convert Rust types ke JSON types

---

#### `routes/search_routes.rs`

**Fungsi:** Endpoint untuk search.

```rust
#[post("/search", format = "json", data = "<req>")]
pub fn search(state: &State<AppState>, req: Json<SearchRequest>) -> Json<SearchResponse> {
    let words = split_query_into_words(&req.query);

    let docs_guard = state.docs.read().expect("RwLock poisoned");
```
**POST /search** - Endpoint utama untuk pencarian:
- `format = "json"` - Accept JSON request
- `data = "<req>"` - Parse request body jadi SearchRequest
- `split_query_into_words()` - Pisah query string jadi array kata

```rust
    let start_parallel = Instant::now();
    let results_parallel = if words.len() <= 1 {
        search_words_sequential(&docs_guard, &words)
    } else {
        search_words_parallel(&docs_guard, &words)
    };
    let parallel_duration = start_parallel.elapsed();
```
**Jalankan parallel search dengan timing:**
- `Instant::now()` - Catat waktu mulai
- Jika cuma 1 kata, tidak perlu parallel (overhead tidak worth it)
- Jika >1 kata, pakai parallel
- `.elapsed()` - Hitung durasi

```rust
    let start_sequential = Instant::now();
    let _results_sequential = search_words_sequential(&docs_guard, &words);
    let sequential_duration = start_sequential.elapsed();
```
**Jalankan sequential search untuk benchmark:**
- Hasil tidak dipakai (makanya pakai `_` prefix)
- Hanya untuk ukur waktu
- Requirement dari revisi

```rust
    let parallel_ms = parallel_duration.as_secs_f64() * 1000.0;
    let sequential_ms = sequential_duration.as_secs_f64() * 1000.0;
    let speedup = if parallel_ms > 0.0 {
        sequential_ms / parallel_ms
    } else {
        1.0
    };
```
**Hitung metrics:**
- Convert durasi ke milliseconds
- Hitung speedup (berapa kali lebih cepat)

```rust
    let docs_with_all = find_docs_with_all_words(&docs_guard, &words);
    let docs_with_all_words: Vec<DocumentMatch> = docs_with_all.into_iter()
        .map(|(id, name, matched)| DocumentMatch {
            doc_id: id,
            doc_name: name,
            matched_words: matched,
        })
        .collect();

    Json(SearchResponse {
        results: results_parallel,
        benchmark: BenchmarkTiming {
            parallel_ms,
            sequential_ms,
            speedup,
        },
        docs_with_all_words,
    })
}
```
**Return response:**
- Hasil pencarian
- Benchmark timing
- Dokumen yang mengandung semua kata
- Rocket otomatis serialize ke JSON

---

## Konsep Penting

### 1. Pemrograman Fungsional di Rust

**Karakteristik yang diterapkan:**

1. **Immutability** - Minimize penggunaan `mut`:
```rust
// ❌ Sebelum revisi (mutable)
let mut docs = state.docs.write().unwrap();
docs.retain(|d| d.id != id);

// ✅ Setelah revisi (immutable)
let filtered_docs = filter_documents_by_id(current_docs, id);
*state.docs.write().unwrap() = filtered_docs;
```

2. **Higher-Order Functions** - Functions yang terima/return functions:
```rust
.map(|x| x * 2)         // map
.filter(|x| x > 0)      // filter
.fold(0, |acc, x| acc + x)  // fold/reduce
```

3. **Iterator Chains** - Combine banyak operasi:
```rust
text.split_whitespace()
    .map(normalize_token)
    .filter(|w| !w.is_empty())
    .collect()
```

4. **Pure Functions** - Tidak ada side effects:
```rust
// Pure: selalu return hasil yang sama untuk input yang sama
pub fn normalize_token(token: &str) -> String {
    token.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase()
}
```

5. **Pattern Matching & Option/Result** - Error handling yang eksplisit:
```rust
match fs::read(path) {
    Ok(bytes) => bytes,
    Err(e) => {
        eprintln!("Error: {}", e);
        return None;
    }
}
```

---

### 2. Parallel Processing dengan Rayon

**Kenapa perlu parallel?**
- Komputer modern punya banyak CPU core (4, 8, 16, dst)
- Tanpa parallel, hanya 1 core yang kerja
- Dengan parallel, semua core dimanfaatkan
- Kecepatan bisa naik 2-8x tergantung jumlah core

**Cara kerja Rayon:**
```rust
// Sequential - 1 thread
words.iter().map(|w| search_single_word(docs, w))

// Parallel - banyak thread
words.par_iter().map(|w| search_single_word(docs, w))
```

**Rayon otomatis:**
- Split array jadi chunks
- Assign setiap chunk ke thread berbeda
- Merge hasil di akhir
- Handle thread pool & scheduling

**Kapan parallel lebih cepat?**
- Ketika ada banyak pekerjaan (>2 kata)
- Ketika setiap pekerjaan independent (tidak depend satu sama lain)
- Ketika overhead thread < benefit parallelism

**Kapan parallel TIDAK lebih cepat?**
- Kalau cuma 1 kata (overhead lebih besar daripada benefit)
- Kalau pekerjaan terlalu cepat (microseconds)

---

### 3. Thread Safety dengan RwLock

**Kenapa perlu RwLock?**
- Aplikasi web punya banyak request bersamaan
- Setiap request jalan di thread berbeda
- Tanpa synchronization, bisa terjadi data race

**Data Race:**
```
Thread 1: Baca data → dapat nilai 5
Thread 2: Baca data → dapat nilai 5
Thread 1: Tulis data + 1 → jadi 6
Thread 2: Tulis data + 1 → jadi 6  ❌ Harusnya 7!
```

**RwLock solusinya:**
- Read lock: Banyak thread bisa read bersamaan
- Write lock: Hanya 1 thread bisa write, block semua read/write lain
- Otomatis unlock setelah keluar scope

```rust
// Read (banyak thread bisa akses)
let docs = state.docs.read().unwrap();

// Write (hanya 1 thread)
let mut docs = state.docs.write().unwrap();
```

---

### 4. Framework Independence

**Kenapa penting?**
- Rocket mungkin tidak selalu jadi pilihan terbaik
- Suatu saat mungkin mau ganti ke Actix-web, Axum, dll
- Business logic tidak boleh terikat dengan framework

**Arsitektur layered:**
```
┌─────────────────────────────┐
│  Routes (Rocket-dependent)  │ ← HTTP layer
├─────────────────────────────┤
│  Services (Pure Rust)       │ ← Business logic
├─────────────────────────────┤
│  Utils (Pure Rust)          │ ← Helper functions
└─────────────────────────────┘
```

**Benefit:**
- Ganti framework: cuma ganti layer Routes
- Test business logic tanpa HTTP server
- Bisa reuse logic di CLI/desktop app

---

## Alur Kerja Program

### 1. Startup Flow

```
main() 
  → build_rocket()
    → load_pdfs_from_dataset()
      → read_dir(dataset_path)
      → filter PDF files
      → process_pdf_file() untuk setiap file
        → fs::read() → bytes
        → encode base64
        → extract_text_from_pdf()
        → build_word_counts()
      → create_document()
    → manage(AppState with docs)
    → mount routes
  → Launch server
```

### 2. Search Request Flow

```
Frontend kirim POST /search dengan body: {"query": "komputer data"}
  ↓
Rocket route handler: search()
  ↓
split_query_into_words("komputer data") → ["komputer", "data"]
  ↓
PARALLEL:
  Thread 1: search_single_word(docs, "komputer")
            → normalize "komputer"
            → lookup di word_counts HashMap
            → extract_snippets()
            → return WordResult
            
  Thread 2: search_single_word(docs, "data")
            → normalize "data"
            → lookup di word_counts HashMap
            → extract_snippets()
            → return WordResult
  ↓
Merge results: [WordResult1, WordResult2]
  ↓
SEQUENTIAL (for benchmark):
  search_single_word(docs, "komputer") → WordResult1
  search_single_word(docs, "data") → WordResult2
  ↓
Calculate timing & speedup
  ↓
find_docs_with_all_words(["komputer", "data"])
  ↓
Return SearchResponse ke frontend
```

### 3. Data Flow

```
PDF File (binary)
  → base64 encode
  → extract_text_from_pdf()
  → Plain text string
  → tokenize()
  → Array of words
  → build_word_counts()
  → HashMap<String, usize>
  → Store in Document
  → Save in AppState (RwLock<Vec<Document>>)
```

---

## FAQ - Pertanyaan Umum Dosen

### Q1: Kenapa tidak pakai database (MySQL/PostgreSQL)?

**Jawaban:**
- Untuk demo/prototype, in-memory storage lebih cepat
- Tidak perlu setup database server
- Akses data O(1) dengan HashMap
- Untuk production, bisa ditambahkan persistence layer

### Q2: Apa yang terjadi kalau server restart?

**Jawaban:**
- Data hilang (karena in-memory)
- Tapi langsung auto-load dari folder dataset saat startup
- Jadi data PDF tetap ada

### Q3: Kenapa parallel search lebih cepat?

**Jawaban:**
- Memanfaatkan multi-core CPU
- Setiap kata dicari di thread berbeda
- 4 kata di 4 core = 4x lebih cepat (idealnya)
- Rayon handle thread pool & scheduling otomatis

### Q4: Kenapa harus normalize kata?

**Jawaban:**
- Agar "Komputer", "komputer", "KOMPUTER" dianggap sama
- Hapus tanda baca agar "komputer." = "komputer"
- Hasil search lebih akurat

### Q5: Apakah business logic depend ke Rocket?

**Jawaban:**
- TIDAK. Semua logic ada di `services/` dan `utils/`
- Services tidak import Rocket sama sekali
- Bisa dipakai di CLI app, desktop app, atau framework lain
- Routes yang depend ke Rocket, tapi itu cuma thin layer

### Q6: Apa bedanya RwLock dengan Mutex?

**Jawaban:**
- Mutex: hanya 1 thread bisa akses (read/write)
- RwLock: banyak thread bisa read, tapi hanya 1 bisa write
- RwLock lebih efisien untuk read-heavy workload
- Search adalah read-heavy (banyak read, jarang write)

### Q7: Kenapa pakai HashMap untuk word_counts?

**Jawaban:**
- Lookup O(1) - sangat cepat
- Alternatif: loop array O(n) - lambat
- Trade-off: pakai lebih banyak memori, tapi search instant

### Q8: Kenapa ada benchmark di setiap search?

**Jawaban:**
- Untuk menunjukkan keunggulan parallel processing
- Membuktikan Rayon benar-benar mempercepat
- Requirement dari revisi dosen
- User bisa lihat speedup real-time

### Q9: Bagaimana cara handle PDF yang corrupt?

**Jawaban:**
- Pakai `panic::catch_unwind()` untuk tangkap panic
- Jika PDF gagal di-extract, skip file tersebut
- Print warning, tapi tidak crash aplikasi
- File lain tetap bisa diproses

### Q10: Apakah recursive function lebih baik daripada iterative?

**Jawaban:**
- Di Rust, iterative lebih baik (lebih cepat, no stack overflow)
- Recursive hanya untuk demonstrasi konsep FP
- Hanya jalan saat debug mode
- Production pakai iterative

---

## Kesimpulan

Text-Search-API adalah backend yang memanfaatkan:

1. **Rust** - Performa tinggi, memory safety
2. **Rocket** - Framework web yang mudah
3. **Rayon** - Parallel processing otomatis
4. **Pemrograman Fungsional** - Immutability, higher-order functions, iterator chains
5. **Arsitektur Modular** - Separation of concerns, framework independence

**Hasil Revisi:**
- ✅ PDF support dengan `pdf-extract`
- ✅ Single text field untuk multi-word search
- ✅ Benchmark parallel vs sequential
- ✅ Struktur modular (9 files)
- ✅ Eliminasi mutable variables
- ✅ Framework-independent business logic

Program ini bukan hanya berfungsi dengan baik, tapi juga:
- **Maintainable** - Code terorganisir dengan baik
- **Testable** - Setiap modul bisa ditest terpisah
- **Scalable** - Mudah ditambah fitur baru
- **Educational** - Demonstrasi konsep FP & parallel programming

Semua requirement telah terpenuhi dan siap untuk demo/presentasi! 🚀
