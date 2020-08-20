#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;

mod blob;
mod cli;
mod commands;
mod tree;
mod utils;

use anyhow::Result;

fn main() -> Result<()> {
    cli::CLI::run()
}
