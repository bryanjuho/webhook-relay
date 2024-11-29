use std::time::Duration;

use reqwest::{header::{HeaderMap, HeaderName, HeaderValue}, Client, StatusCode, Error};
use serde_json::Value;
use tokio::time::timeout;
use log::{info, error};

pub struct ApiClient {
    client: Client,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn poll_url(&self, url: &str, headers: &HeaderMap, timeout_longpoll: Duration) -> Result<Value, Error> {
        
        loop {
            match timeout(timeout_longpoll, 
                self.client.get(url)
                    .headers(headers.clone())
                    .send()
            ).await {
                Ok(response_result) => {
                    match response_result {
                        Ok(response) => {
                            if response.status().is_success() {
                                
                                if let Ok(json) = response.json::<Value>().await {
                                    // If we got a non-empty JSON response, return it
                                    if !json.is_null() {
                                        return Ok(json);
                                    }
                                }
                            } 
                            // 408 is the status code for a timeout
                            else if response.status().eq(&StatusCode::REQUEST_TIMEOUT) {
                                info!("Connection timed out after 59 seconds, retrying...");
                            }
                        }
                        Err(e) => {
                            error!("Connection error: {}, retrying...", e);
                            tokio::time::sleep(Duration::from_secs(5)).await;
                        }
                    }
                }
                Err(_) => {
                    error!("Unexpected connection error, retrying...");
                }
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    pub async fn make_request(
        &self,
        endpoint: String,
        method: &str,
        headers: serde_json::Value,
        body: Option<String>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        // Convert JSON headers to HeaderMap
        let mut header_map = HeaderMap::new();
        if let serde_json::Value::Object(header_obj) = headers {
            for (key, value) in header_obj {
                if let Some(val_str) = value.as_str() {
                    if let Ok(header_name) = HeaderName::from_bytes(key.as_bytes()) {
                        if let Ok(header_value) = HeaderValue::from_str(val_str) {
                            header_map.insert(header_name, header_value);
                        }
                    }
                }
            }
        }
    
        // Build the request based on the method
        let request = match method.to_uppercase().as_str() {
            "GET" => self.client.get(&endpoint),
            "POST" => self.client.post(&endpoint),
            "PUT" => self.client.put(&endpoint),
            "DELETE" => self.client.delete(&endpoint),
            "PATCH" => self.client.patch(&endpoint),
            _ => {
                let err = reqwest::Client::new()
                    .get("http://dummy")
                    .build()
                    .unwrap_err();
                return Err(err);
            }
        };
    
        // Add headers and body to the request
        let request = request.headers(header_map);
        let request = if let Some(body_content) = body {
            request.body(body_content)
        } else {
            request
        };
    
        // Send the request and get JSON response
        let response = request.send().await?;
        Ok(response)
    }
}
