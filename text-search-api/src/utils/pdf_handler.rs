use base64::{Engine as _, engine::general_purpose};

pub fn extract_text_from_pdf(base64_content: &str) -> Result<String, String> {
    let pdf_bytes = general_purpose::STANDARD
        .decode(base64_content)
        .map_err(|e| format!("Failed to decode base64: {}", e))?;
    
    pdf_extract::extract_text_from_mem(&pdf_bytes)
        .map_err(|e| format!("Failed to extract PDF text: {}", e))
}
