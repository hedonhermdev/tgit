#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;

mod commands;
mod cli;

use anyhow::Result;

fn main() -> Result<()>{
    // let args: Vec<String> = env::args().collect();
    // if args.len() == 1 {
    //     println!("No command specified");
    //     return 
    // }
    // if args[1] == "init" {
    //     fs::create_dir(".git").unwrap();
    //     fs::create_dir(".git/objects").unwrap();
    //     fs::create_dir(".git/refs").unwrap();
    //     fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
    //     println!("Initialized git directory")
    // } else if args[1] == "cat-file"{
    //     println!("{}, {}", args[2], args[3]);
    //     if args[2] == "p"
    // } else {
    //     println!("unknown command: {}", args[1])
    // }

    cli::CLI::run()
       
}
