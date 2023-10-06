use anyhow::Result;
use rayon::prelude::*;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::io::Read;
use std::path::Path;
use tokio::sync::mpsc;

// `path` の directory にある ".dat" ファイルを読む。
//
// ファイルは以下を繰り返す独自フォーマット。
// * size (4 byte) - unsigned int
// * bin (`size` byte) - MessagePack-ed bin
pub async fn load_all(path: impl AsRef<Path> + fmt::Display) -> Result<common::Origin> {
    let (sender, mut receiver) = mpsc::unbounded_channel();

    let files = collect_files(path)?;

    files
        .into_par_iter()
        .for_each_with(sender, |sender, (name, mut file)| {
            let mut size_buff = [0; 4];
            let mut v = vec![];
            while let Ok(_) = file.read_exact(&mut size_buff) {
                let len = u32::from_be_bytes(size_buff);
                let mut buf = vec![0; len as usize];
                file.read_exact(&mut buf).unwrap();
                v.push(buf);
            }
            sender.send((name, v.concat())).unwrap();
        });

    let mut res = common::Origin::default();
    while let Some((name, value)) = receiver.recv().await {
        res.insert(name, value);
    }

    Ok(res)
}

fn collect_files<P>(path: P) -> Result<HashMap<String, fs::File>>
where
    P: AsRef<Path>,
{
    let mut res = HashMap::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        // 拡張子を確認
        if entry.path().extension() != Some(OsStr::new("dat")) {
            continue;
        }
        // ファイル名を取得
        let file_name = entry.file_name().into_string().unwrap();
        let file = fs::File::open(entry.path())?;
        res.insert(file_name, file);
    }
    Ok(res)
}
