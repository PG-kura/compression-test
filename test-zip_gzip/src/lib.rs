use anyhow::Result;
use std::fs;
use std::io::{Cursor, Read, Write};
use std::path::Path;
use zip::{write::FileOptions, ZipArchive, ZipWriter};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};

pub fn archive<P>(input: common::Origin, path: P, level: u32) -> Result<common::Archive>
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

    // IO は余計だが、tar 側と合わせるためにファイルに書き出す。
    // tar 側は出力先をファイル以外にすることが出来ないので、zip 側が合わせてやる必要がある。
    // また、想定する用途ではファイルに書き出すことが求められる。
    let gzip_file = fs::File::create(&path).unwrap();
    let mut encoder = GzEncoder::new(gzip_file, Compression::new(level));
    encoder.write_all(&zip_buff)?;
    encoder.finish()?;

    let size = fs::metadata(&path).unwrap().len();
    let archive = common::Archive {
        path: path.as_ref().to_path_buf(),
        size: size as usize,
    };
    Ok(archive)
}

pub fn extract(archive: common::Archive) -> Result<common::Origin> {
    let gzip_file = fs::File::open(archive.path).unwrap();
    let mut decoder = GzDecoder::new(gzip_file);
    let mut zip_buff = vec![];
    decoder.read_to_end(&mut zip_buff)?;
    drop(decoder);

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
