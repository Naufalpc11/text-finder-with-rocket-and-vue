use std::collections::HashMap;

pub fn normalize_token(token: &str) -> String {
    token
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase()
}

pub fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(normalize_token)
        .filter(|w| !w.is_empty())
        .collect()
}

pub fn build_word_counts(text: &str) -> HashMap<String, usize> {
    tokenize(text)
        .into_iter()
        .fold(HashMap::new(), |acc, word| {
            let word_key = word.clone();
            let count = acc.get(&word_key).copied().unwrap_or(0) + 1;
            acc.into_iter()
                .filter(move |(k, _)| k != &word_key)
                .chain(std::iter::once((word, count)))
                .collect()
        })
}