use anyhow::{bail, Result};
use async_recursion::async_recursion;
use sha1::{Digest, Sha1};
use std::convert::TryInto;
use std::fmt;
use std::fmt::Display;
use std::io::{BufRead, Cursor, Read};
// use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use tokio::fs;
use async_trait::async_trait;

use crate::utils;
use crate::objects::Object;
use crate::objects::blob::Blob;

#[derive(Debug, Clone)]
pub struct Tree {
    entries: Vec<TreeEntry>,
    sha1_hash: [u8; 20],
    write_data: Vec<u8>,
}

#[derive(Debug, Clone)]
struct TreeEntry {
    mode: String,
    name: String,
    sha1_hash: [u8; 20],
}


impl TreeEntry {
    pub fn new(mode: String, name: String, sha1_hash: [u8; 20]) -> Self {
        TreeEntry {
            mode,
            name,
            sha1_hash,
        }
    }

    pub async fn from_file(path: PathBuf) -> Result<Self> {
        let metadata = path.metadata()?;

        let filetype = metadata.file_type();

        let mut mode = String::new();
        let mut sha1_hash: [u8; 20] = [0; 20];

        if filetype.is_file() {
            // file -> blob
            let unix_mode = metadata.mode();
            let is_executable = (unix_mode & 0o001) != 0;
            if is_executable {
                mode.push_str("100755");
            } else {
                mode.push_str("100644")
            }
            let blob = Blob::new(path.clone()).await?;
            sha1_hash = blob.sha1_hash();
        } else if filetype.is_symlink() {
            // symlink -> blob
            mode.push_str("120000");
            let blob = Blob::new(path.clone()).await?;
            sha1_hash = blob.sha1_hash();
        } else if filetype.is_dir() {
            // dir -> tree (recursion)
            mode.push_str("040000");
            let tree = Tree::new(path.clone()).await?;
            sha1_hash = tree.sha1_hash();
        }

        let name = path
            .file_name()
            .expect("Expected a name")
            .to_str()
            .expect("Invalid filename");
        let name = String::from(name);

        Ok(Self {
            mode,
            name,
            sha1_hash,
        })
    }

    pub fn data(&self) -> Vec<u8> {
        let mut data = Vec::new();

        data.extend_from_slice(self.mode.as_bytes());
        data.push(0x20u8);
        data.extend_from_slice(self.name.as_bytes());
        data.push(0x00u8);
        data.extend_from_slice(&self.sha1_hash);

        data
    }
}

#[async_trait]
impl Object for Tree {

    async fn from_object_sha(object_sha: String) -> Result<Self> {
        if object_sha.len() != 40 {
            bail!("Invalid SHA");
        }

        let (dir, file) = object_sha.split_at(2);

        let mut path_to_file = PathBuf::new();
        path_to_file.push(".git/objects");
        path_to_file.push(dir);
        path_to_file.push(file);

        let file = fs::read(path_to_file).await?;
        let write_data = utils::zlib_decompress(file)?;

        let mut cursor = Cursor::new(write_data.clone());

        cursor.read_until(0x20u8, &mut Vec::new())?; // tree header

        let mut length = Vec::new();
        cursor.read_until(0x00u8, &mut length)?; //tree length

        let mut entries: Vec<TreeEntry> = Vec::new();

        loop {
            let mut mode = Vec::new();
            let num_read = cursor.read_until(0x20u8, &mut mode)?; // mode

            if num_read == 0 {
                break;
            }

            let mode = String::from_utf8(mode)?;

            let mut name = Vec::new();
            cursor.read_until(0x00, &mut name)?;
            let mut name = String::from_utf8(name)?;
            name.pop();

            let mut sha1_hash: [u8; 20] = [0; 20];
            cursor.read_exact(&mut sha1_hash)?;

            let tree_entry = TreeEntry::new(mode, name, sha1_hash);

            entries.push(tree_entry);
        }

        let sha1_hash = utils::decode_hash(&object_sha);

        Ok(Self {
            entries,
            sha1_hash,
            write_data,
        })
    }

    fn sha1_hash(&self) -> [u8; 20] {
        let mut hash: [u8; 20] = [0; 20];
        hash.copy_from_slice(&self.sha1_hash);
        hash
    }

    fn write_data(&self) -> &Vec<u8> {
        &self.write_data
    }
}

impl Tree {

    #[async_recursion]
    pub async fn new(path: PathBuf) -> Result<Self> {
        let mut paths: Vec<PathBuf> = Vec::new();

        let mut dir = fs::read_dir(path).await?;
        while let Some(entry) = dir.next_entry().await? {
            let fpath = entry.path();
            if fpath.starts_with("./.git") {
                continue;
            }
            paths.push(fpath)
        }

        let mut entries: Vec<TreeEntry> = Vec::new();

        for path in paths {
            let entry = TreeEntry::from_file(path).await?;

            entries.push(entry);
        }

        entries.sort_by(|a, b| a.name.cmp(&b.name));


        let mut entries_data = Vec::new();

        for entry in &entries.clone() {
            entries_data.extend(entry.data());
        }

        let length = entries_data.len();

        let mut data = Vec::new();

        data.extend_from_slice("tree".as_bytes());
        data.push(0x20u8);
        data.extend_from_slice(length.to_string().as_bytes());
        data.push(0x00u8);
        data.extend(entries_data);

        let write_data = data;

        let sha1_hash = Sha1::digest(&write_data);
        let sha1_hash: [u8; 20] = sha1_hash.try_into()?;

        Ok(Self { entries, sha1_hash, write_data })
    }



}

impl Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut names: Vec<String> = Vec::new();

        for entry in self.entries.clone() {
            names.push(entry.name);
        }

        let names = names.join("\n");

        f.write_fmt(format_args!("{}", names))
    }
}
