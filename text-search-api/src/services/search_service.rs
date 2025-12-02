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
                    Some(PerDocCount {
                        doc_id: doc.id,
                        doc_name: doc.name.clone(),
                        count,
                    })
                } else {
                    None
                }
            })
        })
        .collect();

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

fn calculate_total_count(per_doc: &[PerDocCount]) -> usize {
    per_doc.iter().map(|pd| pd.count).sum()
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
