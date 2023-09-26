use anyhow::Result;
use std::io::{self, Read};

pub fn compress(input: &common::Origin) -> Result<common::Archive> {
    let mut buffer = Vec::<u8>::new();
    let mut header = common::Header::default();
    let mut wtr = snap::write::FrameEncoder::new(&mut buffer);

    for (name, value) in input.iter() {
        let mut cursor = io::Cursor::new(value);
        let size = io::copy(&mut cursor, &mut wtr)?;
        header.push((name.clone(), size as usize));
    }
    drop(wtr);

    let archive = common::Archive { buffer, header };
    Ok(archive)
}

pub fn decompress(archive: &common::Archive) -> Result<common::Origin> {
    let mut origin = common::Origin::default();
    let mut rdr = snap::read::FrameDecoder::new(io::Cursor::new(&archive.buffer));
    for (name, size) in archive.header.iter() {
        let mut buf = vec![0; *size];
        rdr.read_exact(&mut buf)?;
        origin.insert(name.clone(), buf);
    }
    Ok(origin)
}