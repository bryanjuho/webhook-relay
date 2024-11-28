use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::Value;
use std::collections::HashMap;

fn parse_headers(json_str: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let value: Value = serde_json::from_str(json_str)?;
    let mut headers = HashMap::new();

    if let Value::Object(map) = value {
        for (key, value) in map {
            if let Value::String(val) = value {
                headers.insert(key, val);
            } else {
                headers.insert(key, value.to_string());
            }
        }
    }

    Ok(headers)
}

pub fn build_headers(headers: &Option<String>) -> Result<HeaderMap, Box<dyn std::error::Error>> {
    let mut parsed_headers = HeaderMap::new();

    parsed_headers.insert(
        "Content-Type",
        HeaderValue::from_static("application/json")
    );

    if let Some(header_json) = headers {
        let header_map = parse_headers(header_json)?;

        for (name, value) in header_map {
            if let (Ok(header_name), Ok(header_value)) = (
                HeaderName::from_bytes(name.as_bytes()),
                HeaderValue::from_str(&value),
            ) {
                parsed_headers.insert(header_name, header_value);
            }
        }
    }

    Ok(parsed_headers)
}
