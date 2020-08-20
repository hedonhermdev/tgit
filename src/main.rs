mod blob;
mod cli;
mod commands;
mod tree;
mod commit;
mod utils;

use anyhow::Result;

fn main() -> Result<()> {
    cli::CLI::run()
}
