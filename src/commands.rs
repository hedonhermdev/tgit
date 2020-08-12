use anyhow::{bail, Result};
use flate2::read::ZlibDecoder;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

fn decode_reader(bytes: Vec<u8>) -> Result<String> {
    let mut z = ZlibDecoder::new(&bytes[..]);
    let mut s = String::new();
    z.read_to_string(&mut s)?;
    Ok(s)
}

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
    if object_sha.len() != 40 {
        bail!("Invalid SHA: {}", &object_sha);
    }

    let (dir, file) = object_sha.split_at(3);

    let mut path_to_file = PathBuf::new();
    path_to_file.push(".git");
    path_to_file.push(dir);
    path_to_file.push(file);

    let blob = fs::read(path_to_file)?;

    let decoded_blob = decode_reader(blob)?;

    if pretty_print {
        print!("{}", decoded_blob);
    }

    Ok(())
}
