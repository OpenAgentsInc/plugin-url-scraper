use extism_pdk::*;
use regex::Regex;
use serde_json::to_string;

#[plugin_fn]
pub fn extract_urls(input: String) -> FnResult<String> {
    let url_regex = Regex::new(r"https?://[^\s]+").unwrap();
    let urls: Vec<String> = url_regex.find_iter(&input)
                                    .map(|mat| mat.as_str().to_string())
                                    .collect();

    let serialized_urls = to_string(&urls).unwrap();
    Ok(serialized_urls)
}

