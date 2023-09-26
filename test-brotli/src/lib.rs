use anyhow::Result;
use brotli::{enc::BrotliEncoderParams, BrotliCompress, Decompressor};
use std::io::{self, Read};

pub fn compress(input: &common::Origin) -> Result<common::Archive> {
    let mut header = common::Header::default();

    let mut buffer = Vec::new();
    let mut params = BrotliEncoderParams::default();

    params.quality = 4;

    for (name, value) in input.iter() {
        let mut cursor = io::Cursor::new(value);
        let size = BrotliCompress(&mut cursor, &mut buffer, &params)?;
        header.push((name.clone(), size as usize));
    }

    let archive = common::Archive { buffer, header };
    Ok(archive)
}

pub fn decompress(archive: &common::Archive) -> Result<common::Origin> {
    let mut origin = common::Origin::default();
    let len = archive.buffer.len();
    let mut reader = Decompressor::new(io::Cursor::new(&archive.buffer), len);
    for (name, size) in archive.header.iter() {
        let mut buf = vec![0; *size];
        reader.read_exact(&mut buf)?;
        origin.insert(name.clone(), buf);
    }
    Ok(origin)
}
