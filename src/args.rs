use clap::Parser;

#[derive(Parser)]
#[command(name = "Webhook Relay")]
#[command(about = "CLI tool to polls a URL and sends to another URL on success")]
pub struct Args {
    // Poll Interval (s)
    #[arg(
        short,
        long,
        default_value_t = 5,
        help = "Interval in seconds between polls",
        value_name = "SECONDS"
    )]
    pub poll_interval: u64,

    // URL to poll (GET request)
    #[arg(
        short,
        long,
        default_value = "https://synapse.api.test.datamaker.io",
        help = "Source URL to poll for data",
        value_name = "SOURCE_URL"
    )]
    pub url: String,

    // URL to post to on successful poll
    #[arg(
        short = 't',
        long,
        default_value = "http://localhost:8000",
        help = "Target URL to forward the response to",
        value_name = "TARGET_URL"
    )]
    pub target_url: String,
}
