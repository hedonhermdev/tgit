use crate::utils;
use anyhow::Result;
use sha1::{Digest, Sha1};
use std::convert::TryInto;
use std::fs;
use std::path::PathBuf;
use async_trait::async_trait;

use crate::objects::Object;

pub struct Commit {
    tree_sha: [u8; 20],
    parent_sha: [u8; 20],
    message: String,
    committer: User,
    author: User,
    sha1_hash: [u8; 20],
    timestamp: String,
    tz_offset: String,
    write_data: Vec<u8>
}

#[derive(Clone)]
pub struct User {
    pub name: String,
    pub email: String,
}

#[async_trait]
impl Object for Commit {
    async fn from_object_sha(object_sha: String) -> Result<Self> {
        unimplemented!()
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

impl Commit {
    pub fn new(
        tree_sha: String,
        parent_sha: String,
        message: String,
        name: String,
        email: String,
    ) -> Result<Self> {
        let tree_sha = utils::decode_hash(&tree_sha);
        let parent_sha = utils::decode_hash(&parent_sha);

        let encoded_tree_sha = hex::encode(tree_sha);
        let encoded_parent_sha = hex::encode(parent_sha);

        let (timestamp, tz_offset) = utils::get_time_data();

        let author = User { name, email };
        let committer = author.clone();

        let formatted_string = format!(
            "tree {}\nparent {}\nauthor {} <{}> {} {}\ncommitter {} <{}> {} {}\n\n{}",
            encoded_tree_sha,
            encoded_parent_sha,
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

        let write_data = data.clone();

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
            write_data,
        })
    }



    pub fn update_refs(&self) -> Result<()> {
        let mut path = PathBuf::from(".git/refs/heads");
        fs::create_dir_all(&path)?;
        path.push("master");

        let contents = self.encoded_sha();
        fs::write(path, contents)?;

        Ok(())
    }

    pub fn encoded_sha(&self) -> String {
        hex::encode(&self.sha1_hash)
    }
}
