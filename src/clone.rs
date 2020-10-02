use anyhow::{bail, Result};
use bytes;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use reqwest::{Client, StatusCode, Url};
use std::i32;

use crate::packfile;
use crate::utils;

pub struct Ref {
    name: String,
    hash: String,
}

impl Ref {
    pub fn parse_pkt_line(data: &str) -> Self {
        let (len, rest) = data.split_at(4);

        let _len = i32::from_str_radix(len, 16);

        let mut iter = rest.split(" ");
        let hash = iter.next().expect("Invalid pkt line").to_string();
        let name = iter.next().expect("Invalid pkt line").to_string();

        Self { name, hash }
    }
}

pub struct CloneClient {
    url: String,
    client: Client,
}

impl CloneClient {
    pub fn new(url: String) -> Self {
        let client = Client::new();

        let mut url = url;
        if url.ends_with(".git") {
            url.push_str("/");
        }
        if !url.ends_with(".git/") {
            url.push_str(".git/");
        }

        Self { url, client }
    }

    pub async fn discover_refs(&self) -> Result<Vec<Ref>> {
        println!("{}", self.url);
        let url = Url::parse(&self.url)?.join("info/refs?service=git-upload-pack")?;
        println!("{}", url.as_str());

        let resp = self.client.get(url).send().await?;
        println!("{}", resp.status());

        if !(resp.status() == StatusCode::OK || resp.status() == StatusCode::NOT_MODIFIED) {
            bail!("Unable to find repository!");
        }

        let content = resp.text().await?;

        let mut refs: Vec<Ref> = vec![];

        for line in content.lines() {
            if line.contains("refs/") {
                let branch_ref = Ref::parse_pkt_line(line);
                refs.push(branch_ref);
            }
        }

        Ok(refs)
    }

    pub async fn request_ref(&self, req_ref: Ref) -> Result<()> {
        let mut pkt_line = String::from("0032want ");
        pkt_line.push_str(&req_ref.hash);
        pkt_line.push('\n');
        pkt_line.push_str("00000009done\n");

        let url = Url::parse(&self.url)?.join("git-upload-pack")?;

        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            "application/x-git-upload-pack-request"
                .parse()
                .expect("Invalid content type"),
        );

        let response = self
            .client
            .post(url)
            .body(pkt_line)
            .headers(headers)
            .send()
            .await?;

        let data = response.bytes().await?;
        let packfile = packfile::Packfile::parse_data(&data);

        Ok(())
    }

    pub async fn clone(&self) -> Result<()> {
        let branch_refs = self.discover_refs().await?;

        for branch in branch_refs {
            println!("{}", branch.name);
            let data = self.request_ref(branch).await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_packfile() {
        let data = fs::read(
            "./tempgit/.git/objects/pack/pack-4e3c870cda81214366531c32ed63a52dbebc56fd.pack",
        );
    }
}
