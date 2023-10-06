use anyhow::Result;
use brotli::{enc::writer::CompressorWriter, Decompressor};
use rayon::prelude::*;
use std::fs;
use std::io::{Cursor, Read, Write};
use std::path::Path;
use tokio::sync::mpsc;

pub async fn archive<P>(input: common::Origin, path: P) -> Result<common::Archive>
where
    P: AsRef<Path>,
{
    let (sender, mut receiver) = mpsc::unbounded_channel();

    tokio::task::spawn_blocking(move || {
        input
            .files
            .par_iter()
            .for_each_with(sender, |sender, (name, value)| {
                let mut write = CompressorWriter::new(vec![], 4096, 9, 20);
                write.write_all(value).unwrap();
                let br_buff = write.into_inner();
                sender.send((name.to_string(), br_buff)).unwrap();
            });
    });

    let tar_file = fs::File::create(&path).unwrap();
    let mut builder = tar::Builder::new(tar_file);

    while let Some((name, br_buff)) = receiver.recv().await {
        let mut header = tar::Header::new_gnu();
        header.set_path(name).unwrap();
        header.set_size(br_buff.len() as u64);
        header.set_cksum();

        let mut cursor = Cursor::new(br_buff);
        builder.append(&header, &mut cursor).unwrap();
    }
    builder.finish().unwrap();

    let size = fs::metadata(&path).unwrap().len();
    let archive = common::Archive {
        path: path.as_ref().to_path_buf(),
        size: size as usize,
    };
    Ok(archive)
}

pub fn extract(archive: common::Archive) -> Result<common::Origin> {
    let mut origin = common::Origin::default();

    let file = fs::File::open(archive.path).unwrap();
    let mut archive = tar::Archive::new(file);

    for entry in archive.entries().unwrap() {
        let mut entry = entry.unwrap();
        let name = entry
            .path()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let mut br_buf = vec![];
        entry.read_to_end(&mut br_buf).unwrap();

        let cursor = Cursor::new(br_buf);
        let mut decoder = Decompressor::new(cursor, 4096);

        let mut value = vec![];
        decoder.read_to_end(&mut value).unwrap();
        origin.insert(name.to_string(), value);
    }

    Ok(origin)
}
