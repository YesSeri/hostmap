use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, command};

#[derive(Parser, Debug)]
#[command(name = "app")]
#[command(version, about = "Server/Scraper runner")]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Args, Debug)]
pub struct ActivationLoggerArgs {
    #[arg(long, help = "Path to the log file where activations will be stored")]
    pub activation_log_file: PathBuf,

    #[arg(long, help = "URL path to the endpoint that receives activation logs")]
    pub url_path: String,

    #[arg(
        long,
        default_value = "0.0.0.0",
        help = "IP address of the server to bind to (default: 0.0.0.0)"
    )]
    pub server_ip: String,

    #[arg(long, help = "Port of the server to bind to")]
    pub port: usize,
}
#[derive(Args, Debug)]
pub struct ServerArgs {
    #[arg(long, help = "The database URL to connect to")]
    pub database_url: String,
    #[arg(
        long,
        help = "File with api key used for communication between scraper and CI server with mappings to server"
    )]
    pub api_key_file: PathBuf,
    #[arg(
        long = "grouping-key",
        help = "Default key from metadata to group by on the frontpage"
    )]
    pub default_grouping_key: Option<String>,
    #[arg(long, default_value = "127.0.0.1", help = "url run the server on")]
    pub url: String,
    #[arg(long, default_value = "3000", help = "port to run server on")]
    pub port: u16,
    #[arg(long, help = "metadata columns to show in the frontpage table")]
    pub columns: Option<Vec<String>>,
    #[arg(
        long,
        help = "link to git repo(github, gitlab, etc), commit hash will be appended, https://github.com/foo-user/repo-name/commit"
    )]
    pub repo_url: String,
}

#[derive(Args, Debug)]
pub struct ScraperArgs {
    #[arg(long)]
    pub hosts_file: PathBuf,
    #[arg(long, help = "wait time in between requests")]
    pub scrape_interval: u64,
    #[arg(
        long,
        default_value_t = 8,
        help = "number of requests being sent each <scrape_interval> + latency from slowest"
    )]
    pub concurrent_requests: usize,
    #[arg(long, help = "port that activation logger nginx proxy runs on")]
    pub activation_logger_port: usize,
    #[arg(
        long,
        help = "File with api key used for communication between scraper and CI server with mappings to server"
    )]
    pub api_key_file: PathBuf,
    #[arg(
        long,
        default_value = "http://localhost:3000",
        help = "url of server to send scraped activations to(the server you started with `hostmap server ...`"
    )]
    pub url: String,
}
#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    ActivationLogger(ActivationLoggerArgs),
    Server(ServerArgs),
    Scraper(ScraperArgs),
}
