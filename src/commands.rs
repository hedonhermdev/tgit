use anyhow::{bail, Result};
use flate2::read::ZlibDecoder;
use sha1::{Sha1, Digest};
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use hex;

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

    let (dir, file) = object_sha.split_at(2);

    let mut path_to_file = PathBuf::new();
    path_to_file.push(".git/objects");
    path_to_file.push(dir);
    path_to_file.push(file);

    let blob = fs::read(path_to_file)?;

    let decoded_blob = decode_reader(blob)?;

    if pretty_print {
        let contents = decoded_blob.split('\0').nth(1);
        if contents.is_some() {
            print!("{}", contents.unwrap());
        }
    }

    Ok(())
}

pub fn hash_object(file: PathBuf, write: bool) -> Result<()> {
    let mut contents = fs::read(file)?;
    let size = contents.len().to_string();
    let object_type = String::from("blob");

    let mut blob_contents = String::new();
    
    blob_contents.push_str(&object_type);
    blob_contents.push(' ');
    blob_contents.push_str(&size);
    blob_contents.push('\0');

    let mut blob_contents = blob_contents.into_bytes();
    blob_contents.append(&mut contents);
    
    let blob_hash = Sha1::digest(&blob_contents);
    let blob_hex = hex::encode(blob_hash);

    println!("{}", blob_hex);

    if write {
        let mut path = PathBuf::from(".git/objects");

        let (dirname, filename) = blob_hex.split_at(2);

        path.push(dirname);

        fs::create_dir_all(&path)?;

        path.push(filename);

        fs::write(&path, blob_contents)?;
    }

    Ok(())
}

