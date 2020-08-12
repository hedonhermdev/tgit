use crate::commands;
use structopt::StructOpt;
use std::path::PathBuf;

use anyhow::Result;

#[derive(Debug, StructOpt)]
#[structopt(name="TGit", about="HedonHermDev's implementation of Git")]
pub enum CLI {
    #[structopt(name = "init", about = "Initialize an empty git repository")]
    Init{
        git_dir: Option<PathBuf>
    },

    #[structopt(name = "cat-file", about = "Cat the contents of a git object")]
    CatFile {
        #[structopt(name = "pretty_print", short = "p", about= "Pretty print the contents")]
        pretty_print: bool,

        #[structopt(name = "OBJECT SHA")]
        object_sha: String,
    },
}

impl CLI {
    pub fn run() -> Result<()> {
        let args: Self = Self::from_args();

        match args {
            CLI::Init{git_dir} => commands::init(git_dir),
            CLI::CatFile{pretty_print, object_sha} => commands::cat_file(pretty_print, object_sha)
        }
    }
}
