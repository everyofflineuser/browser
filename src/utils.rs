use percent_encoding::percent_encode;

use crate::app::SEARCH_ENGINE;

pub fn normalize_url(input: &str) -> String {
    let input = input.trim().to_lowercase();
    
    if let Ok(url) = url::Url::parse(&input) {
        return url.to_string();
    }

    if input.contains('.') && !input.contains(' ') {
        return format!("https://{}", input);
    }

    let encoded_query = percent_encode(input.as_bytes(), percent_encoding::NON_ALPHANUMERIC).to_string();
    format!("{}{}", SEARCH_ENGINE, encoded_query)
}