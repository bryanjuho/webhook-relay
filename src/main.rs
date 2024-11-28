use std::time::Duration;
use tokio::time::sleep;

mod args;
mod client;

use args::Args;
use client::ApiClient;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = ApiClient::new();
    let poll_interval = Duration::from_secs(args.poll_interval);

    println!("Starting to poll {} every {} seconds...", args.url, args.poll_interval);
    println!("Will post to {} on success", args.target_url);

    loop {
        match client.poll_url(&args.url).await {
            Ok(text) => {
                println!("Received response: {}", text);
                
                match client.post_response(&args.target_url, &text).await {
                    Ok(_) => println!("Sent to target URL"),
                    Err(e) => println!("Failed to POST to target URL: {}", e),
                }
            }
            Err(e) => println!("Request failed: {}", e),
        }

        sleep(poll_interval).await;
    }
}