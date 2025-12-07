use crate::models::document::Document;
use crate::models::response::{PerDocCount, WordResult};
use crate::utils::normalize_token;
use rayon::prelude::*;

pub fn split_query_into_words(query: &str) -> Vec<String> {
    query
        .split_whitespace()
        .map(|w| w.trim().to_string())
        .filter(|w| !w.is_empty())
        .collect()
}

pub fn search_words_parallel(docs: &[Document], words: &[String]) -> Vec<WordResult> {
    words
        .par_iter()
        .map(|w| search_single_word(docs, w))
        .collect()
}

pub fn search_words_sequential(docs: &[Document], words: &[String]) -> Vec<WordResult> {
    words
        .iter()
        .map(|w| search_single_word(docs, w))
        .collect()
}

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

    let total_count = calculate_total_count(&per_doc);
    //kompilasi pas debug
    #[cfg(debug_assertions)]
    {
        let recursive_total = count_word_recursive(docs, &word, 0, 0);
        debug_assert_eq!(
            total_count, recursive_total,
            "Mismatch: iterative={} vs recursive={}",
            total_count, recursive_total
        );
    }
    //kirim ke fe
    WordResult {
        word,
        total_count,
        per_doc,
    }
}

fn calculate_total_count(per_doc: &[PerDocCount]) -> usize {
    per_doc.iter().map(|pd| pd.count).sum()
}

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