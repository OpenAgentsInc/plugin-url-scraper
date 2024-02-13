use extism_pdk::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct UrlInput {
    url: String,
}

#[plugin_fn]
pub fn fetch_url_content(input_json: String) -> FnResult<String> {
    // Deserialize the input JSON into the UrlInput struct
    let input: UrlInput = serde_json::from_str(&input_json).unwrap();

    let body = json!({}).to_string();

    // Construct the HttpRequest
    let req = HttpRequest::new(input.url)
        .with_method("GET");

    // Send the HTTP request and get the response
    let res = http::request(&req, Some(body.into_bytes())).unwrap();

    // Convert the response body to a string
    // Note: Simplified and not handling errors
    let response_body = String::from_utf8(res.body().clone()).unwrap();

    Ok(response_body)
}

