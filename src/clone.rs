use reqwest::{Url, Client, StatusCode};
use anyhow::{Result, bail};

use crate::utils;

pub struct CloneClient {
    client: Client
}

impl CloneClient {
    pub fn new() -> Self {
        let client = Client::new();

        Self {
            client
        }
    }

    pub async fn request_refs(&self, url: String) -> Result<()> {
        let params = [("service", "git-upload-pack")];
        
        let mut url = url;
        if url.ends_with(".git") {
            url.push_str("/");
        }
        if !url.ends_with(".git/") {
            url.push_str(".git/");
        }

        let url = Url::parse(&url)?
            .join("info/refs")?;
        println!("{}", url.as_str());

        let resp = self.client.get(url).query(&params).send().await?;
        if resp.status() != StatusCode::OK || resp.status() != StatusCode::NOT_MODIFIED {
            bail!("Unable to find repository!");
        }

        let content = resp.text().await?;

        for line in content.lines() {
            if line.contains("refs/") {
                let branch_ref = Ref::parse(line);
            }
        }

        Ok(())
    }
}

struct Ref {
    name: String,
    sha1_hash: [u8; 20],
}

impl Ref {
    pub fn parse(data: &str) -> Self {
        let mut iter = data.split(" ");
        let hash = iter.next().expect("Invalid ref");
        let name = iter.next().expect("Invalid ref").to_string();

        let sha1_hash = utils::decode_hash(hash);

        Self {
            name,
            sha1_hash
        }
    }
}
