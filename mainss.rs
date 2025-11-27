1 #[macro_use]
2 extern crate rocket;
3 
4 use rocket::{Build, Rocket, State};
5 use rocket::serde::{Deserialize, Serialize, json::Json};
6 use rocket::http::Status;
7 use rocket::response::status;
8 use rocket_cors::{AllowedOrigins, CorsOptions};
9 use rayon::prelude::*;
10 use std::collections::HashMap;
11 use std::sync::{
12     RwLock,
13     atomic::{AtomicUsize, Ordering},
14 };
15 
16 type DocId = usize;
17 
18 #[derive(Debug, Clone)]
19 struct Document {
20     id: DocId,
21     name: String,
22     content: String,
23    
24     word_counts: HashMap<String, usize>,
25 }
26 
27 #[derive(Debug, Clone, Serialize)]
28 #[serde(crate = "rocket::serde")]
29 struct DocumentInfo {
30     id: DocId,
31     name: String,
32 }
33 
34 #[derive(Debug, Clone, Deserialize)]
35 #[serde(crate = "rocket::serde")]
36 struct UploadedFile {
37     name: String,
38     content: String,
39 }
40 struct AppState {
41     docs: RwLock<Vec<Document>>,
42     next_id: AtomicUsize,
43 }
44 
45 #[derive(Debug, Clone, Serialize)]
46 #[serde(crate = "rocket::serde")]
47 struct UploadResponse {
48     total_files: usize,
49     doc_ids: Vec<DocId>,
50 }
51 
52 #[derive(Debug, Clone, Deserialize)]
53 #[serde(crate = "rocket::serde")]
54 struct SearchRequest {
55    
56     words: Vec<String>,
57 }
58 
59 #[derive(Debug, Clone, Serialize)]
60 #[serde(crate = "rocket::serde")]
61 struct PerDocCount {
62     doc_id: DocId,
63     doc_name: String,
64     count: usize,
65 }
66 
67 #[derive(Debug, Clone, Serialize)]
68 #[serde(crate = "rocket::serde")]
69 struct WordResult {
70     word: String,
71     total_count: usize,
72     per_doc: Vec<PerDocCount>,
73 }
74 
75 #[derive(Debug, Clone, Serialize)]
76 #[serde(crate = "rocket::serde")]
77 struct SearchResponse {
78     results: Vec<WordResult>,
79 }
80 
81 #[derive(Debug, Clone, Serialize)]
82 #[serde(crate = "rocket::serde")]
83 struct DeleteResponse {
84     success: bool,
85     remaining: usize,
86 }
87 
88 #[derive(Debug, Clone, Serialize)]
89 #[serde(crate = "rocket::serde")]
90 struct DeleteAllResponse {
91     success: bool,
92     remaining: usize,
93 }
94 
95 fn normalize_token(token: &str) -> String {
96     token
97         .chars()
98         .filter(|c| c.is_alphanumeric())
99         .collect::<String>()
100         .to_lowercase()
101 }
102 
103 fn tokenize(text: &str) -> Vec<String> {
104     text.split_whitespace()
105         .map(normalize_token)
106         .filter(|w| !w.is_empty())
107         .collect()
108 }
109 
110 fn build_word_counts(text: &str) -> HashMap<String, usize> {
111     tokenize(text)
112         .into_iter()
113         .fold(HashMap::new(), |mut acc, word| {
114             *acc.entry(word).or_insert(0) += 1;
115             acc
116         })
117 }
118 
119 fn count_total_occurrences(per_doc: &[PerDocCount]) -> usize {
120     per_doc.iter().map(|pd| pd.count).sum()
121 }
122 
123 fn filter_docs_with_word<'a>(docs: &'a [Document], word: &str) -> Vec<&'a Document> {
124     docs.iter()
125         .filter(|doc| doc.word_counts.contains_key(word))
126         .collect()
127 }
128 
129 fn count_word(docs: &[Document], word: &str, index: usize, acc: usize) -> usize {
130     if index >= docs.len() {
131         return acc;
132     }
133    
134     let count = docs[index]
135         .word_counts
136         .get(word)
137         .copied()
138         .unwrap_or(0);
139    
140     count_word(docs, word, index + 1, acc + count)
141 }
142 
143 fn calculate_doc_stats(docs: &[Document]) -> (usize, usize, usize, f64) {
144     let total_docs = docs.len();
145     let total_words: usize = docs
146         .iter()
147         .map(|doc| doc.word_counts.values().sum::<usize>())
148         .sum();
149     
150     let total_bytes: usize = docs.iter().map(|d| d.content.len()).sum();
151    
152     let avg_words = if total_docs > 0 {
153         total_words as f64 / total_docs as f64
154     } else {
155         0.0
156     };
157     (total_docs, total_words, total_bytes, avg_words)
158 }
159 
160 fn search_single_word(docs: &[Document], raw_word: &str) -> WordResult {
161     let word = normalize_token(raw_word);
162     
163     let relevant_docs = filter_docs_with_word(docs, &word);
164     
165     let per_doc: Vec<PerDocCount> = relevant_docs
166         .into_iter()
167         .filter_map(|doc| {
168             let count = doc.word_counts.get(&word).copied().unwrap_or(0);
169             if count > 0 {
170                 Some(PerDocCount {
171                     doc_id: doc.id,
172                     doc_name: doc.name.clone(),
173                     count,
174                 })
175             } else {
176                 None
177             }
178         })
179         .collect();
180 
181     let total_count = count_total_occurrences(&per_doc);
182 
183     #[cfg(debug_assertions)]
184     {
185         let recursive_total = count_word(docs, &word, 0, 0);
186         debug_assert_eq!(total_count, recursive_total, 
187             "Mismatch: iterative={} vs recursive={}", total_count, recursive_total);
188     }
189 
190     WordResult {
191         word,
192         total_count,
193         per_doc,
194     }
195 }
196 
197 #[post("/upload", format = "json", data = "<files>")]
198 async fn upload_files(
199     state: &State<AppState>,
200     files: Json<Vec<UploadedFile>>,
201 ) -> Json<UploadResponse> {
202     let processed_docs: Vec<(String, String, HashMap<String, usize>)> = if files.len() >= 2 {
203         files
204             .par_iter()
205             .map(|f| {
206                 let word_counts = build_word_counts(&f.content);
207                 (f.name.clone(), f.content.clone(), word_counts)
208             })
209             .collect()
210     } else {
211         files
212             .iter()
213             .map(|f| {
214                 let word_counts = build_word_counts(&f.content);
215                 (f.name.clone(), f.content.clone(), word_counts)
216             })
217             .collect()
218     };
219 
220     let mut docs_guard = state.docs.write().expect("RwLock poisoned");
221     let new_ids: Vec<DocId> = processed_docs
222         .into_iter()
223         .map(|(name, content, word_counts)| {
224             let id = state.next_id.fetch_add(1, Ordering::Relaxed);
225             let doc = Document {
226                 id,
227                 name,
228                 content,
229                 word_counts,
230             };
231             docs_guard.push(doc);
232             id
233         })
234         .collect();
235 
236 
237     Json(UploadResponse {
238         total_files: docs_guard.len(),
239         doc_ids: new_ids,
240     })
241 }
242 
243 #[get("/docs")]
244 fn list_docs(state: &State<AppState>) -> Json<Vec<DocumentInfo>> {
245     let docs_guard = state.docs.read().expect("RwLock poisoned");
246     let list = docs_guard
247         .iter()
248         .map(|d| DocumentInfo {
249             id: d.id,
250             name: d.name.clone(),
251         })
252         .collect();
253    
254     Json(list)
255 }
256 
257 #[get("/stats")]
258 fn get_stats(state: &State<AppState>) -> Json<serde_json::Value> {
259     let docs_guard = state.docs.read().expect("RwLock poisoned");
260     let (total_docs, total_words, total_bytes, avg_words) = calculate_doc_stats(&docs_guard);
261    
262     Json(serde_json::json!({
263         "total_documents": total_docs,
264         "total_words": total_words,
265         "total_bytes": total_bytes,
266         "average_words_per_doc": avg_words,
267     }))
268 }
269 
270 #[post("/search", format = "json", data = "<req>")]
271 fn search(state: &State<AppState>, req: Json<SearchRequest>) -> Json<SearchResponse> {
272     let words: Vec<String> = req
273         .words
274         .iter()
275         .map(|w| w.trim().to_string())
276         .filter(|w| !w.is_empty())
277         .collect();
278 
279     let docs_guard = state.docs.read().expect("RwLock poisoned");
280     let results: Vec<WordResult> = if words.len() <= 1 {
281        
282         words
283             .iter()
284             .map(|w| search_single_word(&docs_guard, w))
285             .collect()
286     } else {
287         words
288             .par_iter()
289             .map(|w| search_single_word(&docs_guard, w))
290             .collect()
291     };
292 
293     Json(SearchResponse { results })
294 }
295 
296 
297 #[delete("/docs/<id>")]
298 fn delete_doc(
299     state: &State<AppState>,
300     id: DocId,
301 ) -> Result<Json<DeleteResponse>, status::Custom<String>> {
302     let mut docs = state.docs.write().expect("RwLock poisoned");
303     let before = docs.len();
304 
305     docs.retain(|d| d.id != id);
306 
307     if docs.len() == before {
308        
309         Err(status::Custom(
310             Status::NotFound,
311             format!("Document with id {} not found", id),
312         ))
313     } else {
314         Ok(Json(DeleteResponse {
315             success: true,
316             remaining: docs.len(),
317         }))
318     }
319 }
320 
321 #[delete("/docs")]
322 fn delete_all_docs(state: &State<AppState>) -> Json<DeleteAllResponse> {
323     let mut docs = state.docs.write().expect("RwLock poisoned");
324     docs.clear();
325     state.next_id.store(0, Ordering::Relaxed);
326     Json(DeleteAllResponse {
327         success: true,
328         remaining: 0,
329     })
330 }
331 
332 fn build_rocket() -> Rocket<Build> {
333     let allowed_origins = AllowedOrigins::some_exact(&[
334         "http://localhost:5173",
335         "http://127.0.0.1:5173",
336     ]);
337 
338     let cors = CorsOptions {
339         allowed_origins,
340         allow_credentials: true,
341         ..Default::default()
342     }
343     .to_cors()
344     .expect("error building CORS");
345 
346 
347     rocket::build()
348         .manage(AppState {
349             docs: RwLock::new(Vec::new()),
350             next_id: AtomicUsize::new(0),
351         })
352         .mount(
353             "/",
354             routes![
355                 upload_files,
356                 list_docs,
357                 get_stats,
358                 search,
359                 delete_doc,
360                 delete_all_docs
361             ],
362         )
363         .attach(cors)
364 }
365 
366 #[launch]
367 fn rocket() -> _ {
368     build_rocket()
369 }