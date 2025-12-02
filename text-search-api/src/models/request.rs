use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SearchRequest {
    pub query: String,
}
