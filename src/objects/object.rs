use crate::utils;
use anyhow::Result;
use async_trait::async_trait;
use hex;
use std::fmt;
use std::fmt::Display;
use std::path::PathBuf;
use tokio::fs;

#[async_trait]
pub trait Object {

    async fn from_object_sha(object_sha: String) -> Result<Self>
    where
        Self: Sized;

    fn sha1_hash(&self) -> [u8; 20];

    fn write_data(&self) -> &Vec<u8>;

    async fn write(&self) -> Result<PathBuf> {
        let mut path = PathBuf::from(".git/objects");

        let blob_hex = hex::encode(self.sha1_hash());
        let (dirname, filename) = blob_hex.split_at(2);

        path.push(dirname);

        fs::create_dir_all(&path).await?;
        path.push(filename);

        let encoded_content = utils::zlib_compress(&self.write_data())?;

        fs::write(&path, encoded_content).await?;

        Ok(path)
    }

    fn encoded_hash(&self) -> String {
        hex::encode(&self.sha1_hash())
    }

}

impl Display for dyn Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.encoded_hash()))
    }
}
