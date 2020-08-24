use anyhow::{Context, Result};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use hex;
use std::io::{Read, Write};

use chrono;

pub fn zlib_decompress(bytes: Vec<u8>) -> Result<Vec<u8>> {
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

pub fn decode_hash(sha1_hash: &str) -> [u8; 20] {
    let mut hash_decoded: [u8; 20] = [0; 20];
    hex::decode_to_slice(sha1_hash, &mut hash_decoded[..]).expect("Invalid hex");

    return hash_decoded;
}

pub fn get_time_data() -> (String, String) {
    let now = chrono::Local::now();
    let timestamp = now.timestamp().to_string();
    let offset = now.offset().utc_minus_local().to_string();

    return (timestamp, offset)
}
