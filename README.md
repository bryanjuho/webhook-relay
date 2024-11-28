# Webhook Relay

A Rust CLI tool that polls a URL at specified intervals and forwards successful responses to a target URL.

## Features

- Configurable polling interval
- Source URL polling with GET requests
- Forward responses to target URL with POST requests
- Command line interface with help text
- Error handling and logging

## Usage

```bash
# Basic usage with default source URL
cargo run -- -t https://target-api.com

# Custom source URL and polling interval
cargo run -- -u https://source-api.com -t https://target-api.com -p 10

# Show help
cargo run -- --help