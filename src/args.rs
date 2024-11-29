use clap::Parser;

#[derive(Parser)]
#[command(name = "Webhook Relay")]
#[command(about = "CLI tool to polls a URL and sends to another URL on success")]
pub struct Args {
    // Poll Interval (s)
    #[arg(
        short,
        long,
        default_value_t = 59,
        help = "Long poll timeout in seconds",
        value_name = "TIMEOUT"
    )]
    pub timeout: u64,

    // Source URL to poll (GET request)
    #[arg(
        long,
        default_value = "https://synapse.api.test.datamaker.io",
        help = "Source URL to poll for data",
        value_name = "SOURCE_URL"
    )]
    pub source_url: String,

    // URL to post to on successful poll
    #[arg(
        long,
        default_value = "http://localhost:8000",
        help = "Target URL to forward the response to",
        value_name = "TARGET_URL"
    )]
    pub target_url: String,

    #[arg(
        long,
        default_value = "http://localhost:8000",
        help = "Final URL to return the response to",
        value_name = "FINAL_URL"
    )]
    pub final_url: String,

    #[arg(
        short = 'H',
        long = "headers",
        help = "Headers for Source URL as JSON string (e.g. '{\"Authorization\": \"Bearer token\", \"X-Custom\": \"value\"}'"
    )]
    pub headers: Option<String>,
}
