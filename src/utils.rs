use anyhow::{Context, Result};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use hex;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

pub fn zlib_decompress(path: PathBuf) -> Result<Vec<u8>> {
    let bytes = fs::read(path).context("Failed to read file")?;

    let mut z = ZlibDecoder::new(&bytes[..]);
    let mut b = Vec::new();
    z.read_to_end(&mut b).context("Failed to decompress file")?;

    Ok(b)
}

pub fn zlib_compress(content: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(content)?;

    let compressed = encoder.finish()?;

    Ok(compressed)
}

pub fn decode_hash(sha1_hash: String) -> [u8; 20] {
    let mut hash_decoded: [u8; 20] = [0; 20];
    hex::decode_to_slice(sha1_hash, &mut hash_decoded[..]).expect("Invalid hex");

    return hash_decoded;
}
