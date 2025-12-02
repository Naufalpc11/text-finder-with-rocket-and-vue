pub mod document_routes;
pub mod search_routes;

pub use document_routes::{list_docs, get_stats};
pub use search_routes::search;
