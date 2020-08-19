#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;

mod commands;
mod cli;
mod tree;
mod blob;
mod utils;

use anyhow::Result;

fn main() -> Result<()>{
    cli::CLI::run()
}
