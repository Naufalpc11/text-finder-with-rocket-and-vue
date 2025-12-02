# Quick Start Guide

## Prerequisites
- Rust (latest stable) - https://rustup.rs/
- Node.js (v18+) - https://nodejs.org/
- npm or yarn

## Step 1: Start Backend (Rust + Rocket)

```powershell
# Navigate to backend directory
cd text-search-api

# Build (first time only - downloads dependencies)
cargo build

# Run the server
cargo run
```

**Expected Output:**
```
Rocket has launched from http://127.0.0.1:8000
```

The backend will be available at **http://localhost:8000**

### Backend Endpoints:
- `POST /upload` - Upload PDF files
- `POST /search` - Search words with benchmark
- `GET /docs` - List uploaded documents
- `GET /stats` - Get statistics
- `DELETE /docs/<id>` - Delete specific document
- `DELETE /docs` - Delete all documents

## Step 2: Start Frontend (Vue.js)

**Open a NEW terminal window**, then:

```powershell
# Navigate to frontend directory
cd text-search-ui

# Install dependencies (first time only)
npm install

# Run development server
npm run dev
```

**Expected Output:**
```
VITE v... ready in ... ms

  ➜  Local:   http://localhost:5173/
  ➜  Network: use --host to expose
```

The UI will be available at **http://localhost:5173**

## Step 3: Use the Application

1. **Open browser**: Go to http://localhost:5173
2. **Upload files**: 
   - Click "Pilih File PDF"
   - Select 1-6 PDF files
   - Files will be uploaded automatically
3. **Search words**:
   - Type words in the search field (e.g., "Kami Tidur Makan Nasi")
   - Press Enter or click "Cari" button
4. **View results**:
   - See benchmark comparison (parallel vs sequential)
   - View word counts per document
   - Check which documents contain which words

## Testing the Features

### Test PDF Support
1. Upload a PDF file
2. Notice the PDF icon (📕)
3. Search for words in the PDF
4. Results should show the PDF filename

### Test Multiple Words
1. Upload 2-3 files
2. Type multiple words: `data mining algorithm`
3. See results for each word separately
4. Each word shows total count and per-document breakdown

### Test Benchmark
1. Upload multiple files
2. Search for 3+ words
3. Look at the "Performa Benchmark" section
4. Compare parallel vs sequential times
5. See the speedup ratio

### Expected Benchmark Results
- **For 1 word**: Speedup ~1x (sequential is fine)
- **For 2+ words**: Speedup 1.5x - 3x (parallel is faster)
- **Large files**: Higher speedup

## Troubleshooting

### Backend Won't Start
```powershell
# Check if port 8000 is in use
netstat -ano | findstr :8000

# Kill the process using the port (if needed)
taskkill /PID <process_id> /F

# Try again
cargo run
```

### Frontend Won't Start
```powershell
# Clear node_modules and reinstall
Remove-Item -Recurse -Force node_modules
npm install

# Try again
npm run dev
```

### CORS Errors
Make sure:
1. Backend is running on port 8000
2. Frontend is running on port 5173
3. Both are on localhost/127.0.0.1

### PDF Not Working
- Ensure the PDF contains text (not just images)
- Some PDFs are scanned images - these won't work
- Try with a text-based PDF first

## File Size Limits
- Backend limit: 1 MB per file (configurable in Rocket)
- Frontend: No specific limit, but large files may be slow
- Recommended: Keep files under 1 MB for best performance

## Development Commands

### Backend
```powershell
# Check code without building
cargo check

# Run tests
cargo test

# Build for production
cargo build --release

# Run production build
.\target\release\text-search-api.exe
```

### Frontend
```powershell
# Build for production
npm run build

# Preview production build
npm run preview

# Lint code
npm run lint
```

## Performance Tips

1. **For best parallel performance**: Upload multiple files and search for 3+ words
2. **For fastest results**: Keep files small (under 500 KB)
3. **For PDF performance**: Text-based PDFs work better than scanned ones

## Next Steps

- Check `SUMMARY.md` for complete feature list
- Check `REVISI.md` for detailed changes
- Explore the code structure in `src/` directories
- Modify and experiment with the code

## Need Help?

Common issues:
1. **Rust not installed**: Install from https://rustup.rs/
2. **Node.js not installed**: Install from https://nodejs.org/
3. **Port already in use**: Change port in backend or frontend config
4. **Dependencies failing**: Check internet connection and try again

---

**Happy coding! 🚀**
