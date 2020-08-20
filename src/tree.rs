use crate::utils;
use anyhow::{bail, Result};
use sha1::{Digest, Sha1};
use std::convert::TryInto;
use std::fmt;
use std::fmt::Display;
use std::fs;
use std::io::{BufRead, Cursor, Read};
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

use crate::blob::Blob;

#[derive(Debug, Clone)]
pub struct Tree {
    entries: Vec<TreeEntry>,
    sha1_hash: [u8; 20],
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

    pub fn from_file(path: PathBuf) -> Result<Self> {
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
            let blob = Blob::new(path.clone())?;
            sha1_hash = blob.sha1_hash();
        } else if filetype.is_symlink() {
            // symlink -> blob
            mode.push_str("120000");
            let blob = Blob::new(path.clone())?;
            sha1_hash = blob.sha1_hash();
        } else if filetype.is_dir() {
            // dir -> tree (recursion)
            mode.push_str("040000");
            let tree = Tree::from_directory(path.clone())?;
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

impl Tree {
    pub fn from_tree_sha(tree_sha: String) -> Result<Self> {
        if tree_sha.len() != 40 {
            bail!("Invalid SHA {}", &tree_sha);
        }

        let (dir, file) = tree_sha.split_at(2);

        let mut path_to_file = PathBuf::new();
        path_to_file.push(".git/objects");
        path_to_file.push(dir);
        path_to_file.push(file);

        let data = utils::zlib_decompress(path_to_file)?;

        let mut cursor = Cursor::new(data);

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

        let sha1_hash = utils::decode_hash(tree_sha);

        Ok(Self { entries, sha1_hash })
    }

    pub fn from_directory(path: PathBuf) -> Result<Self> {
        let mut paths: Vec<PathBuf> = Vec::new();

        for entry in fs::read_dir(path)? {
            let path = entry?.path();
            if path == PathBuf::from("./.git") {
                continue
            }
            paths.push(path);
        }

        let mut entries: Vec<TreeEntry> = Vec::new();

        for path in paths {
            let entry = TreeEntry::from_file(path)?;

            entries.push(entry);
        }

        let mut entries_data = Vec::new();

        for entry in entries.clone() {
            entries_data.extend(entry.data());
        }

        let length = entries_data.len();

        let mut data = Vec::new();

        data.extend_from_slice("tree".as_bytes());
        data.push(0x20u8);
        data.extend_from_slice(length.to_string().as_bytes());
        data.extend(entries_data);

        let sha1_hash = Sha1::digest(&data);
        let sha1_hash: [u8; 20] = sha1_hash.try_into()?;

        Ok(Self { entries, sha1_hash })
    }

    pub fn sha1_hash(&self) -> [u8; 20] {
        let mut hash: [u8; 20] = [0; 20];
        hash.copy_from_slice(&self.sha1_hash);

        hash
    }

    pub fn encoded_sha(&self) -> String {
        hex::encode(self.sha1_hash)
    }

    pub fn data(&self) -> Vec<u8> {
        let mut entries_data = Vec::new();

        for entry in self.entries.clone() {
            entries_data.extend(entry.data());
        }

        let length = entries_data.len();

        let mut data = Vec::new();

        data.extend_from_slice("tree".as_bytes());
        data.push(0x20u8);
        data.extend_from_slice(length.to_string().as_bytes());
        data.push(0x00u8);
        data.extend(entries_data);

        println!("{}", String::from_utf8_lossy(&data));

        data
    }

    pub fn write(&self) -> Result<()> {
        let mut path = PathBuf::from(".git/objects");

        let blob_hex = hex::encode(self.sha1_hash);
        let (dirname, filename) = blob_hex.split_at(2);

        path.push(dirname);

        fs::create_dir_all(&path)?;
        path.push(filename);


        let encoded_data = utils::zlib_compress(&self.data())?;
        fs::write(path, encoded_data)?;

        Ok(())
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
