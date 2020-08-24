use crate::utils;
use anyhow::{bail, Result};
use sha1::{Digest, Sha1};
use std::convert::TryInto;
use std::fmt;
use std::fmt::Display;
use tokio::fs;
use std::path::PathBuf;

pub struct Blob {
    contents: Vec<u8>,
    sha1_hash: [u8; 20],
    write_data: Vec<u8>
}

impl Blob {
    pub async fn new(file: PathBuf) -> Result<Self> {
        let mut file_data = fs::read(file).await?;
        let size = file_data.len().to_string();

        let contents = file_data.clone();

        let mut write_data = String::new();
        write_data.push_str("blob");
        write_data.push(' ');
        write_data.push_str(&size);
        write_data.push('\0');
        let mut write_data = write_data.into_bytes();
        write_data.append(&mut file_data);
        let sha1_hash = Sha1::digest(&write_data);
        let sha1_hash: [u8; 20] = sha1_hash.try_into()?;

        return Ok(Self {
            contents,
            sha1_hash,
            write_data
        });
    }

    pub async fn from_object_sha(object_sha: String) -> Result<Self> {
        if object_sha.len() != 40 {
            bail!("Invalid SHA: {}", &object_sha);
        }

        let (dir, file) = object_sha.split_at(2);

        let mut path_to_file = PathBuf::new();
        path_to_file.push(".git/objects");
        path_to_file.push(dir);
        path_to_file.push(file);

        let file = fs::read(path_to_file).await?;
        let write_data = utils::zlib_decompress(file)?;

        let contents_ref = write_data.split(|x| *x == 0x00u8).nth(1);

        let contents: Vec<u8>;
        if contents_ref.is_some() {
            contents = contents_ref.unwrap().to_vec();
        } else {
            contents = Vec::new();
        }

        let sha1_hash = utils::decode_hash(&object_sha);

        Ok(Self {
            contents,
            sha1_hash,
            write_data
        })
    }

    pub fn sha1_hash(&self) -> [u8; 20] {
        let mut hash: [u8; 20] = [0; 20];
        hash.copy_from_slice(&self.sha1_hash);

        hash
    }

    pub fn encoded_hash(&self) -> String {
        hex::encode(&self.sha1_hash)
    }

    pub async fn write(&self) -> Result<PathBuf> {
        let mut path = PathBuf::from(".git/objects");

        let blob_hex = hex::encode(self.sha1_hash);
        let (dirname, filename) = blob_hex.split_at(2);

        path.push(dirname);

        fs::create_dir_all(&path).await?;
        path.push(filename);

        let encoded_content = utils::zlib_compress(&self.write_data)?;

        fs::write(&path, encoded_content).await?;

        Ok(path)
    }
}

impl Display for Blob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = String::from_utf8_lossy(&self.contents);

        f.write_fmt(format_args!("{}", out))
    }
}
