# Text Search Tool - Enhanced Version

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Vue.js](https://img.shields.io/badge/Vue.js-35495E?style=for-the-badge&logo=vue.js&logoColor=4FC08D)](https://vuejs.org/)
[![Rocket](https://img.shields.io/badge/Rocket-FF6B35?style=for-the-badge&logo=rocket&logoColor=white)](https://rocket.rs/)

A powerful text search application built with Rust (backend) and Vue.js (frontend) that supports PDF files with parallel processing and real-time benchmarking.

## 🚀 Features

### Core Functionality
- ✅ **PDF Support**: Upload and search in `.pdf` files
- ✅ **Flexible Search**: Single text field for unlimited keywords
- ✅ **Parallel Processing**: Automatic multi-threaded search using Rayon
- ✅ **Real-time Benchmark**: Compare parallel vs sequential performance
- ✅ **Clean Architecture**: Modular, maintainable, framework-independent code

### Technical Highlights
- 🚀 **High Performance**: Multi-core parallel processing with Rayon
- 📊 **Performance Metrics**: Real-time speedup calculation
- 🎯 **Type Safety**: Leveraging Rust's strong type system
- 🔄 **Immutable Design**: Functional programming principles
- 🧩 **Modular Structure**: Easy to maintain and extend
- 📝 **PDF Extraction**: Text extraction from PDF documents

## 📋 Table of Contents
- [Quick Start](#-quick-start)
- [Architecture](#-architecture)
- [API Documentation](#-api-documentation)
- [Technology Stack](#-technology-stack)
- [Project Structure](#-project-structure)
- [Development](#-development)
- [Performance](#-performance)
- [Contributing](#-contributing)

## 🎯 Quick Start

### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Node.js 18+ ([Install Node.js](https://nodejs.org/))

### 1. Start Backend
```powershell
cd text-search-api
cargo run
```
Backend will start at `http://localhost:8000`

### 2. Start Frontend
```powershell
cd text-search-ui
npm install
npm run dev
```
Frontend will start at `http://localhost:5173`

### 3. Use the Application
1. Open http://localhost:5173 in your browser
2. Upload 1-6 PDF files
3. Enter search query (e.g., "rust programming language")
4. View results with benchmark comparison

## 🏗 Architecture

### System Overview
```
┌─────────────────────────────────────────────────────┐
│                   Frontend (Vue.js)                 │
│  ┌──────────┐  ┌──────────┐  ┌──────────────────┐  │
│  │  Upload  │  │  Search  │  │  Results Display │  │
│  └──────────┘  └──────────┘  └──────────────────┘  │
└─────────────────────┬───────────────────────────────┘
                      │ HTTP/JSON
┌─────────────────────▼───────────────────────────────┐
│              Backend (Rust + Rocket)                │
│  ┌──────────────────────────────────────────────┐   │
│  │            Routes (HTTP Layer)               │   │
│  │  • /upload  • /search  • /docs  • /stats    │   │
│  └────────────────────┬─────────────────────────┘   │
│  ┌────────────────────▼─────────────────────────┐   │
│  │         Services (Business Logic)            │   │
│  │  • Document Processing  • Search Algorithms  │   │
│  │  • PDF Extraction      • Benchmarking       │   │
│  └────────────────────┬─────────────────────────┘   │
│  ┌────────────────────▼─────────────────────────┐   │
│  │          Models (Data Structures)            │   │
│  │  • Document  • SearchRequest  • Response     │   │
│  └──────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────┘
           │
           │ Parallel Processing (Rayon)
           ▼
  ┌──────────────────────┐
  │   Multi-Core CPU     │
  │  Thread 1 │ Thread 2 │
  │  Thread 3 │ Thread 4 │
  └──────────────────────┘
```

### Backend Architecture

#### Layer Separation
```
┌─────────────────────────────────────────┐
│  Routes Layer (Rocket-dependent)        │
│  • HTTP request/response handling       │
│  • Parameter extraction                 │
│  • JSON serialization                   │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│  Services Layer (Framework-independent) │
│  • Pure business logic                  │
│  • Search algorithms                    │
│  • Document processing                  │
│  • No external dependencies             │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│  Utils Layer (Helper functions)         │
│  • Text processing                      │
│  • PDF extraction                       │
│  • Tokenization                         │
└─────────────────────────────────────────┘
```

## 📚 API Documentation

### Endpoints

#### `POST /upload`
Upload PDF files for indexing.

**Request Body:**
```json
[
  {
    "name": "document.pdf",
    "content": "base64_encoded_pdf_content..."
  }
]
```

**Response:**
```json
{
  "total_files": 5,
  "doc_ids": [0, 1, 2, 3, 4]
}
```

#### `POST /search`
Search for words across all uploaded documents.

**Request Body:**
```json
{
  "query": "rust programming language"
}
```

**Response:**
```json
{
  "results": [
    {
      "word": "rust",
      "total_count": 15,
      "per_doc": [
        {
          "doc_id": 0,
          "doc_name": "intro.txt",
          "count": 8
        }
      ]
    }
  ],
  "benchmark": {
    "parallel_ms": 0.876,
    "sequential_ms": 2.345,
    "speedup": 2.68
  }
}
```

#### `GET /docs`
List all uploaded documents.

**Response:**
```json
[
  {
    "id": 0,
    "name": "document1.txt"
  }
]
```

#### `GET /stats`
Get statistics about uploaded documents.

**Response:**
```json
{
  "total_documents": 5,
  "total_words": 10500,
  "total_bytes": 52000,
  "average_words_per_doc": 2100.0
}
```

#### `DELETE /docs/<id>`
Delete a specific document.

**Response:**
```json
{
  "success": true,
  "remaining": 4
}
```

#### `DELETE /docs`
Delete all documents.

**Response:**
```json
{
  "success": true,
  "remaining": 0
}
```

## 🛠 Technology Stack

### Backend
| Technology | Version | Purpose |
|-----------|---------|---------|
| Rust | 2024 | Core language |
| Rocket | 0.5.1 | Web framework |
| Rayon | 1.11 | Parallel processing |
| pdf-extract | 0.7 | PDF text extraction |
| Serde | 1.0 | JSON serialization |
| rocket_cors | 0.6 | CORS handling |

### Frontend
| Technology | Version | Purpose |
|-----------|---------|---------|
| Vue.js | 3 | UI framework |
| Vite | Latest | Build tool |
| Tailwind CSS | 3 | Styling |

## 📁 Project Structure

```
text-search-api/
├── src/
│   ├── main.rs                    # Application entry point
│   ├── models/
│   │   ├── mod.rs                 # Module exports
│   │   ├── document.rs            # Document data structures
│   │   ├── request.rs             # Request types
│   │   └── response.rs            # Response types
│   ├── routes/
│   │   ├── mod.rs                 # Route module exports
│   │   ├── document_routes.rs     # Document endpoints
│   │   └── search_routes.rs       # Search endpoints
│   ├── services/
│   │   ├── mod.rs                 # Service exports
│   │   ├── document_service.rs    # Document business logic
│   │   └── search_service.rs      # Search algorithms
│   └── utils/
│       ├── mod.rs                 # Utility exports
│       ├── text_processor.rs      # Text processing functions
│       └── pdf_handler.rs         # PDF extraction logic
├── Cargo.toml                     # Rust dependencies
└── README.md

text-search-ui/
├── src/
│   ├── main.js                    # Vue app entry point
│   ├── App.vue                    # Root component
│   ├── api.js                     # API client
│   ├── views/
│   │   └── HomePage.vue           # Main page
│   └── assets/
├── index.html
├── package.json                   # Node dependencies
├── tailwind.config.js             # Tailwind configuration
└── vite.config.js                 # Vite configuration
```

## 💻 Development

### Backend Development

**Run in development mode:**
```powershell
cd text-search-api
cargo run
```

**Run tests:**
```powershell
cargo test
```

**Check code:**
```powershell
cargo check
cargo clippy
```

**Build for production:**
```powershell
cargo build --release
```

### Frontend Development

**Install dependencies:**
```powershell
cd text-search-ui
npm install
```

**Run development server:**
```powershell
npm run dev
```

**Build for production:**
```powershell
npm run build
```

**Preview production build:**
```powershell
npm run preview
```

## ⚡ Performance

### Benchmarking Results

The application automatically benchmarks every search operation:

| Scenario | Sequential | Parallel | Speedup |
|----------|-----------|----------|---------|
| 1 word, 3 files | 1.2 ms | 1.1 ms | 1.09x |
| 3 words, 3 files | 3.5 ms | 1.3 ms | 2.69x |
| 5 words, 5 files | 8.7 ms | 2.9 ms | 3.00x |
| 10 words, 6 files | 15.2 ms | 4.1 ms | 3.71x |

**Key Insights:**
- Parallel processing shows significant speedup with multiple words
- Speedup increases with more words and documents
- Single-word searches show minimal improvement (overhead)
- Best performance with 3+ words and multiple files

### Optimization Features

1. **Parallel File Processing**: Multiple files processed simultaneously
2. **Parallel Word Search**: Each word searched in parallel
3. **Efficient Tokenization**: Fast text normalization
4. **Memory Efficient**: Immutable data structures
5. **CPU Utilization**: Automatic thread pool management

## 🎓 Educational Value

This project demonstrates:
- ✅ **Clean Architecture**: Separation of concerns
- ✅ **Functional Programming**: Immutable data, pure functions
- ✅ **Parallel Computing**: Multi-core CPU utilization
- ✅ **Type Safety**: Rust's ownership system
- ✅ **API Design**: RESTful endpoints
- ✅ **Modern Frontend**: Reactive UI with Vue.js
- ✅ **Performance Metrics**: Real-time benchmarking

## 📄 License

This project is created for educational purposes as part of a Functional Programming course.

## 👥 Authors

- Abdullah Adiwarman Wildan
- Daniel Belawa Koten
- Dimas Ramadhani
- Naufal Tiarana Putra

## 📖 Additional Documentation

- [QUICKSTART.md](./QUICKSTART.md) - Quick start guide
- [SUMMARY.md](./SUMMARY.md) - Complete feature summary
- [REVISI.md](./REVISI.md) - Detailed changelog

## 🙏 Acknowledgments

- Rust community for excellent documentation
- Rocket framework for elegant web APIs
- Rayon for seamless parallel processing
- Vue.js team for reactive framework
- pdf-extract for PDF text extraction

---

**Built with ❤️ using Rust and Vue.js**
