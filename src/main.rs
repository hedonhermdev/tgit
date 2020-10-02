mod cli;
mod clone;
mod commands;
mod objects;
mod packfile;
mod utils;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    cli::CLI::run().await
}
