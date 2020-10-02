use anyhow::Result;
use std::path::PathBuf;
use tokio::fs;

use crate::objects::{Object, Blob, Tree, Commit};
use crate::clone::CloneClient;


pub async fn init(git_dir: Option<PathBuf>) -> Result<()> {
    let mut git_dir = git_dir;

    if git_dir.is_none() {
        git_dir = Some(PathBuf::from(".git"));
    }

    let mut path = git_dir.unwrap();

    fs::create_dir(&path).await?; // .git

    path.push("objects");
    fs::create_dir(&path).await?; // .git/objects

    path.pop();
    path.push("refs");
    fs::create_dir(&path).await?; // .git/refs

    path.pop();
    path.push("HEAD");
    fs::write(path, "ref: refs/heads/master\n").await?; // .git/HEAD

    Ok(())
}

pub async fn cat_file(pretty_print: bool, object_sha: String) -> Result<()> {
    let blob = Blob::from_object_sha(object_sha).await?;

    if pretty_print {
        print!("{}", blob);
    }

    Ok(())
}

pub async fn hash_object(file: PathBuf, write: bool) -> Result<()> {
    let blob = Blob::new(file).await?;

    if write {
        blob.write().await?;
    }
    print!("{}", blob.encoded_hash());

    Ok(())
}

pub async fn list_tree(tree_sha: String, name_only: bool) -> Result<()> {
    let tree = Tree::from_object_sha(tree_sha).await?;

    if name_only {
        println!("{}", tree);
    }

    Ok(())
}

pub async fn write_tree() -> Result<()> {
    let tree = Tree::new(PathBuf::from("./")).await?;

    tree.write().await?;

    println!("{}", tree.encoded_hash());

    Ok(())
}

pub async fn commit_tree(tree_sha: String, parent_sha: String, message: String) -> Result<()> {
    let name = String::from("Tirth Jain");
    let email = String::from("jaintirth24@gmail.com");

    let commit = Commit::new(tree_sha, parent_sha, message, name, email)?;

    commit.write().await?;
    commit.update_refs()?;

    println!("{}", commit.encoded_sha());

    Ok(())
}

pub async fn clone(url: String, clone_dir: PathBuf) -> Result<()> {
    let client = CloneClient::new(url);

    client.clone().await?;

    Ok(())
}
