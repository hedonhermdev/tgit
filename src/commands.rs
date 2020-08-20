use anyhow::Result;
use std::fs;
use std::path::PathBuf;

use crate::blob::Blob;
use crate::tree::Tree;
use crate::commit::Commit;

pub fn init(git_dir: Option<PathBuf>) -> Result<()> {
    let mut git_dir = git_dir;

    if git_dir.is_none() {
        git_dir = Some(PathBuf::from(".git"));
    }

    let mut path = git_dir.unwrap();

    fs::create_dir(&path)?; // .git

    path.push("objects");
    fs::create_dir(&path)?; // .git/objects

    path.pop();
    path.push("refs");
    fs::create_dir(&path)?; // .git/refs

    path.pop();
    path.push("HEAD");
    fs::write(path, "ref: refs/heads/master\n")?; // .git/HEAD

    Ok(())
}

pub fn cat_file(pretty_print: bool, object_sha: String) -> Result<()> {
    let blob = Blob::from_object_sha(object_sha)?;

    if pretty_print {
        print!("{}", blob);
    }

    Ok(())
}

pub fn hash_object(file: PathBuf, write: bool) -> Result<()> {
    let blob = Blob::new(file)?;

    if write {
        blob.write()?;
    }
    print!("{}", blob.encoded_hash());

    Ok(())
}

pub fn list_tree(tree_sha: String, name_only: bool) -> Result<()> {
    let tree = Tree::from_tree_sha(tree_sha)?;

    if name_only {
        println!("{}", tree);
    }

    Ok(())
}

pub fn write_tree() -> Result<()> {
    let tree = Tree::from_directory(PathBuf::from("./"))?;

    tree.write()?;

    println!("{}", tree.encoded_sha());

    Ok(())
}

pub fn commit_tree(tree_sha: String, parent_sha: String, message: String) -> Result<()> {
    let name = String::from("Tirth Jain");
    let email = String::from("jaintirth24@gmail.com");

    let commit = Commit::new(tree_sha, parent_sha, message, name, email)?;

    commit.write()?;

    println!("{}", commit.encoded_sha());

    Ok(())
}
