use crate::commands;
use std::path::PathBuf;
use structopt::StructOpt;

use anyhow::Result;

#[derive(Debug, StructOpt)]
#[structopt(name = "TGit", about = "HedonHermDev's implementation of Git")]
pub enum CLI {
    #[structopt(name = "init", about = "Initialize an empty git repository")]
    Init { git_dir: Option<PathBuf> },

    #[structopt(name = "cat-file", about = "Cat the contents of a git object")]
    CatFile {
        #[structopt(
            name = "pretty_print",
            short = "p",
            about = "Pretty print the contents"
        )]
        pretty_print: bool,

        #[structopt(name = "OBJECT SHA")]
        object_sha: String,
    },

    #[structopt(
        name = "hash-object",
        about = "Hash the contents of the given file to a git object"
    )]
    HashObject {
        #[structopt(name = "FILE")]
        file: PathBuf,

        #[structopt(name = "write", short = "w")]
        write: bool,
    },

    #[structopt(name = "ls-tree", about = "List a git tree")]
    ListTree {
        #[structopt(name = "TREE SHA")]
        tree_sha: String,

        #[structopt(long = "name-only")]
        name_only: bool,
    },

    #[structopt(name = "write-tree", about = "Write the working tree")]
    WriteTree,

    CommitTree {
        #[structopt(name = "TREE SHA")]
        tree_sha: String,

        #[structopt(name = "parent_sha", short = "p")]
        parent_sha: String,

        #[structopt(name = "message", short = "m")]
        message: String,
    },

    #[structopt(name = "clone", about = "Clone a remote repository")]
    Clone {
        #[structopt(name = "URL")]
        url: String,

        #[structopt(name = "CLONE DIR")]
        clone_dir: PathBuf,
    },
}

impl CLI {
    pub async fn run() -> Result<()> {
        let args: Self = Self::from_args();

        match args {
            CLI::Init { git_dir } => commands::init(git_dir).await,
            CLI::CatFile {
                pretty_print,
                object_sha,
            } => commands::cat_file(pretty_print, object_sha).await,
            CLI::HashObject { file, write } => commands::hash_object(file, write).await,
            CLI::ListTree {
                tree_sha,
                name_only,
            } => commands::list_tree(tree_sha, name_only).await,
            CLI::WriteTree => commands::write_tree().await,
            CLI::CommitTree {
                tree_sha,
                parent_sha,
                message,
            } => commands::commit_tree(tree_sha, parent_sha, message).await,
            CLI::Clone { url, clone_dir } => commands::clone(url, clone_dir).await,
        }
    }
}
