mod blob;
mod cli;
mod commands;
mod tree;
mod commit;
mod utils;
mod clone;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    cli::CLI::run().await
}
