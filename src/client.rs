use reqwest::{header::HeaderMap, Client};
pub struct ApiClient {
    client: Client,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn poll_url(&self, url: &str, headers: &HeaderMap) -> Result<String, reqwest::Error> {
        let response = self.client.get(url).headers(headers.clone()).send().await?;
        if response.status().is_success() {
            response.text().await
        } else {
            Err(response.error_for_status().unwrap_err())
        }
    }

    pub async fn post_response(
        &self,
        target_url: &str,
        headers: &HeaderMap,
        source_response: &str,
    ) -> Result<(), reqwest::Error> {
        let response = self
            .client
            .post(target_url)
            .headers(headers.clone())
            .body(source_response.to_string())
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(response.error_for_status().unwrap_err())
        }
    }
}
