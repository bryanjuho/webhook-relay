use std::error::Error;
use std::time::Duration;

use clap::Parser;
use log::{debug, error, info};
use tokio::time::sleep;

mod args;
mod client;
mod headers;

use args::Args;
use client::ApiClient;
use headers::build_headers;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let thread_count = 4;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(thread_count)
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(run_multi_thread(thread_count)).unwrap();
}

async fn run_multi_thread(thread_count: usize) -> Result<(), Box<dyn Error>> {
    let mut handles = vec![];

    info!("Starting {} worker threads..", thread_count);

    for thread_id in 0..thread_count {
        let handle = tokio::spawn(async move {
            if let Err(e) = app(thread_id as u32).await {
                eprintln!("Thread {thread_id} error: {}", e);
            }
        });
        handles.push(handle);
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    for handle in handles {
        handle.await.unwrap();
    }

    Ok(())
}

async fn app(thread_id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = ApiClient::new();

    let timeout_longpoll = Duration::from_secs(args.timeout);
    let source_headers = build_headers(&args.headers)?;

    loop {
        info!(
            "Waiting for requests.. (long polling to: {})",
            args.source_url
        );
        match client
            .poll_url(&args.source_url, &source_headers, timeout_longpoll)
            .await
        {
            Ok(response) => {
                // Received an API request from the source URL
                let request_id = response["id"].as_str().unwrap_or("");
                let request_method = response["request"]["method"].as_str().unwrap_or("");
                let request_path = response["request"]["path"].as_str().unwrap_or("");
                let request_headers = response["request"]["headers"].clone();
                let endpoint = format!("{}{}", &args.target_url, request_path);

                info!("Received {} API request: {}", request_method, request_id);
                debug!("Thread {}: Request ID: {:?}", thread_id, request_id);

                // Forward request to target URL
                match client
                    .make_request(endpoint, request_method, request_headers, None)
                    .await
                {
                    Ok(response) => {
                        info!("Forwarded to target URL, finally returning response to final URL");

                        let status_code = response.status().as_u16();
                        let data = response
                            .json::<serde_json::Value>()
                            .await
                            .unwrap_or_default();
                        let body = Some(
                            serde_json::json!({
                                "request_id": request_id,
                                "status": status_code,
                                "data": data
                            })
                            .to_string(),
                        );

                        // empty headers
                        let _headers = serde_json::json!({
                            "Content-Type": "application/json"
                        });

                        match client
                            .make_request(args.final_url.clone(), "POST", _headers, body)
                            .await
                        {
                            Ok(_) => info!("Final request completed successfully."),
                            Err(e) => error!("API request failed to final URL: {}", e),
                        }
                    }
                    Err(e) => error!("API request failed to target URL: {}", e),
                }
            }
            Err(e) => error!("Failed to poll source URL: {}", e),
        }
        sleep(Duration::from_millis(500)).await;
    }
}
