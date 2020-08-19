use anyhow::Result;
use std::fs;
use std::path::PathBuf;

use crate::tree::Tree;
use crate::blob::Blob;


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
