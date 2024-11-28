use std::time::Duration;
use tokio::time::sleep;

mod args;
mod client;
mod headers;

use args::Args;
use clap::Parser;
use client::ApiClient;

use headers::build_headers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = ApiClient::new();
    
    let poll_interval = Duration::from_secs(args.interval);
    let source_headers = build_headers(&args.source_headers)?;
    let target_headers = build_headers(&args.target_headers)?;

    println!(
        "Starting to poll {} every {} seconds...",
        args.source_url, args.interval
    );
    println!("Will post to {} on success", args.target_url);

    loop {
        match client.poll_url(&args.source_url, &source_headers).await {
            Ok(text) => {
                println!("Received response: {}", text);

                match client
                    .post_response(&args.target_url, &target_headers, &text)
                    .await
                {
                    Ok(_) => println!("Sent to target URL"),
                    Err(e) => println!("Failed to POST to target URL: {}", e),
                }
            }
            Err(e) => println!("Request failed: {}", e),
        }

        sleep(poll_interval).await;
    }
}
