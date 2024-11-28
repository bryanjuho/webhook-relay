use std::time::Duration;
use tokio::time::sleep;

mod args;
mod client;
mod headers;

use args::Args;
use clap::Parser;
use client::ApiClient;
use log::{info, error};

use headers::build_headers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args = Args::parse();
    let client = ApiClient::new();
    
    let poll_interval = Duration::from_secs(args.interval);
    let source_headers = build_headers(&args.source_headers)?;
    let target_headers = build_headers(&args.target_headers)?;

    info!("Polling source URL: {} every {} seconds", args.source_url, args.interval);

    loop {
        match client.poll_url(&args.source_url, &source_headers).await {
            Ok(text) => {
                info!("Received response from source URL.");

                match client
                    .post_response(&args.target_url, &target_headers, &text)
                    .await
                {
                    Ok(_) => info!("POST complete to target URL {}", &args.target_url),
                    Err(e) => error!("POST failed to target URL: {}", e),
                }
            }
            Err(e) => error!("Failed to poll source URL: {}", e),
        }

        sleep(poll_interval).await;
    }
}
