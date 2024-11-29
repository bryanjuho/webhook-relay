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
cargo run -- \
 --source-url http://10.0.0.4:8000/agents/a3c050a4a18da626091184a31c4c5aeca3482362/get_request/ \
 --target-url http://10.0.0.4:8005/ \
 --final-url http://10.0.0.4:8000/agents/a3c050a4a18da626091184a31c4c5aeca3482362/set_response/