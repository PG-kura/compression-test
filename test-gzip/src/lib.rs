use anyhow::Result;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{self, Read};

pub fn compress(input: &common::Origin) -> Result<common::Archive> {
    let mut header = common::Header::default();
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

    for (name, value) in input.iter() {
        let mut cursor = io::Cursor::new(value);
        let size = io::copy(&mut cursor, &mut encoder)?;
        header.push((name.clone(), size as usize));
    }
    let buffer = encoder.finish()?;
    let archive = common::Archive { buffer, header };
    Ok(archive)
}

pub fn decompress(archive: &common::Archive) -> Result<common::Origin> {
    let mut origin = common::Origin::default();

    let mut decoder = GzDecoder::new(io::Cursor::new(&archive.buffer));
    for (name, size) in archive.header.iter() {
        let mut buf = vec![0; *size];
        decoder.read_exact(&mut buf)?;
        origin.insert(name.clone(), buf);
    }

    Ok(origin)
}
