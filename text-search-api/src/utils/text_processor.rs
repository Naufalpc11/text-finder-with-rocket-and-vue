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
}// halo bagaimana

pub fn build_word_counts(text: &str) -> HashMap<String, usize> {
    tokenize(text)
        .into_iter()
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        })
}
