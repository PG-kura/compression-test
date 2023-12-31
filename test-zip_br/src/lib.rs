use anyhow::Result;
use brotli::{enc::writer::CompressorWriter, Decompressor};
use std::fs;
use std::io::{Cursor, Read, Write};
use std::path::Path;
use zip::{write::FileOptions, ZipArchive, ZipWriter};

pub fn archive<P>(input: common::Origin, path: P, quality: u32) -> Result<common::Archive>
where
    P: AsRef<Path>,
{
    let mut zip_buff = vec![];
    let mut cursor = Cursor::new(&mut zip_buff);
    let mut zip = ZipWriter::new(&mut cursor);
    let options = FileOptions::default();

    for (name, value) in input.files {
        zip.start_file(name, options).unwrap();
        zip.write_all(&value).unwrap();
    }

    zip.finish()?;
    drop(zip);

    let br_file = fs::File::create(&path)?;
    let mut write = CompressorWriter::new(br_file, 4096, quality, 22);
    write.write_all(&zip_buff).unwrap();
    let br_file = write.into_inner();
    drop(br_file);

    let size = fs::metadata(&path).unwrap().len();
    let archive = common::Archive {
        path: path.as_ref().to_path_buf(),
        size: size as usize,
    };
    Ok(archive)
}

pub fn extract(archive: common::Archive) -> Result<common::Origin> {
    let br_file = fs::File::open(archive.path).unwrap();
    let mut decompressor = Decompressor::new(br_file, 4096);
    let mut zip_buff = vec![];
    decompressor.read_to_end(&mut zip_buff).unwrap();
    drop(decompressor);

    let mut origin = common::Origin::default();
    let mut cursor = Cursor::new(&zip_buff);
    let mut zip = ZipArchive::new(&mut cursor)?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let name = file.name().to_string();
        let mut buf = vec![];
        file.read_to_end(&mut buf).unwrap();
        origin.insert(name, buf);
    }
    Ok(origin)
}
