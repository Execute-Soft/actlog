mod app;
mod cli;
mod commands;
mod error;

use clap::Parser;
use cli::Cli;
use error::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let cli = Cli::parse();
    app::run(cli).await
}
