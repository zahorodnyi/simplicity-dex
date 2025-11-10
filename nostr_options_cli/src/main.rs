use clap::Parser;
use global_utils::logger::init_logger;
use simplicity_dex::cli::Cli;
use tracing::instrument;

#[tokio::main]
#[instrument]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    let _logger_guard = init_logger();
    let cli = Cli::parse();
    cli.process().await?;
    Ok(())
}
