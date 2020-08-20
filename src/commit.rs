use crate::utils;
use anyhow::Result;
use sha1::{Digest, Sha1};
use std::convert::TryInto;
use std::fs;
use std::path::PathBuf;

pub struct Commit {
    tree_sha: [u8; 20],
    parent_sha: [u8; 20],
    message: String,
    committer: User,
    author: User,
    sha1_hash: [u8; 20],
    timestamp: String,
    tz_offset: String,
}

#[derive(Clone)]
pub struct User {
    pub name: String,
    pub email: String,
}

impl Commit {
    pub fn new(
        tree_sha: String,
        parent_sha: String,
        message: String,
        name: String,
        email: String,
    ) -> Result<Self> {
        let tree_sha = utils::decode_hash(tree_sha);
        let parent_sha = utils::decode_hash(parent_sha);

        let commit_data = String::new();

        let encoded_tree_sha = hex::encode(tree_sha);

        let (timestamp, tz_offset) = utils::get_time_data();

        let author = User { name, email };
        let committer = author.clone();

        let formatted_string = format!(
            "tree {}\nauthor {} <{}> {} {}\ncommitter {} <{}> {} {}\n\n{}",
            encoded_tree_sha,
            author.name,
            author.email,
            timestamp,
            tz_offset,
            committer.name,
            committer.email,
            timestamp,
            tz_offset,
            message
        );

        let length = formatted_string.len().to_string();

        let mut data = Vec::new();
        data.extend_from_slice("commit".as_bytes());
        data.push(0x20u8);
        data.extend_from_slice(length.as_bytes());
        data.push(0x00);
        data.extend_from_slice(formatted_string.as_bytes());

        let sha1_hash = Sha1::digest(&data);
        let sha1_hash: [u8; 20] = sha1_hash.try_into()?;


        Ok(Self {
            tree_sha,
            parent_sha,
            message,
            author,
            committer,
            sha1_hash,
            timestamp,
            tz_offset,
        })
    }

    pub fn write(&self) -> Result<()> {
        let commit_data = String::new();

        let encoded_tree_sha = hex::encode(self.tree_sha);

        let formatted_string = format!(
            "tree {}\nauthor {} <{}> {} {}\ncommitter {} <{}> {} {}\n\n{}",
            encoded_tree_sha,
            self.author.name,
            self.author.email,
            self.timestamp,
            self.tz_offset,
            self.committer.name,
            self.committer.email,
            self.timestamp,
            self.tz_offset,
            self.message
        );

        let length = formatted_string.len().to_string();

        let mut data = Vec::new();
        data.extend_from_slice("commit".as_bytes());
        data.push(0x20u8);
        data.extend_from_slice(length.as_bytes());
        data.push(0x00);
        data.extend_from_slice(formatted_string.as_bytes());

        let sha1_hash = Sha1::digest(&data);
        let sha1_hash: [u8; 20] = sha1_hash.try_into()?;

        let sha1_hex = hex::encode(self.sha1_hash);
        let (dirname, filename) = sha1_hex.split_at(2);

        let mut path = PathBuf::from(".git/objects");

        path.push(dirname);

        fs::create_dir_all(&path)?;
        path.push(filename);

        let encoded_content = utils::zlib_compress(&data)?;

        fs::write(&path, encoded_content)?;

        Ok(())
    }

    pub fn encoded_sha(&self) -> String {
        
        hex::encode(&self.sha1_hash)
    }
}
