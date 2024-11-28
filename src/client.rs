use reqwest::Client;

pub struct ApiClient {
    client: Client,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn poll_url(&self, url: &str) -> Result<String, reqwest::Error> {
        let response = self.client.get(url).send().await?;
        if response.status().is_success() {
            response.text().await
        } else {
            Err(response.error_for_status().unwrap_err())
        }
    }

    pub async fn post_response(&self, target_url: &str, source_response: &str) -> Result<(), reqwest::Error> {
        let response = self.client
            .post(target_url)
            .header("Content-Type", "application/json")
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