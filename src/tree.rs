use anyhow::{ Result, bail };
use std::io::{Read, BufRead, Cursor};
use std::path::PathBuf;
use crate::utils;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Tree {
    length: i32,
    entries: Vec<TreeEntry>
}


#[derive(Debug, Clone)]
struct TreeEntry {
    mode: String,
    name: String,
    sha1_hash: [u8; 20]
}


impl TreeEntry{
    pub fn new(mode: String, name: String, sha1_hash: [u8; 20]) -> Self {
        TreeEntry{
            mode,
            name,
            sha1_hash
        }
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
        cursor.read_until(0x00u8, &mut length)?;//tree length
        let mut length = String::from_utf8(length)?;
        length.pop();
        let length = length.as_str().parse::<i32>()?;
    
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
            let name = String::from_utf8(name)?;
            name.pop();

            let mut sha1_hash: [u8; 20] = [0; 20];
            cursor.read_exact(&mut sha1_hash)?;

            let tree_entry = TreeEntry::new(mode, name, sha1_hash);

            entries.push(tree_entry);
        }

        Ok(Self{
            length,
            entries
        })
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
