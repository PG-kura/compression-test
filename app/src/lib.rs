use anyhow::Result;
use std::ffi::OsStr;
use std::fmt;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncReadExt;

// `path` の directory にある ".dat" ファイルを読む。
//
// ファイルは以下を繰り返す独自フォーマット。
// * size (4 byte) - unsigned int
// * bin (`size` byte) - MessagePack-ed bin
pub async fn load_all(path: impl AsRef<Path> + fmt::Display) -> Result<common::Origin> {
    log::info!("dir path is {}", path);
    let mut files = fs::read_dir(path).await?;

    let mut res = common::Origin::default();
    while let Some(file) = files.next_entry().await? {
        // 拡張子を確認
        if file.path().extension() != Some(OsStr::new("dat")) {
            continue;
        }
        // ファイル名を取得
        let file_name = file.file_name().into_string().unwrap();
        log::info!("file is {:?}", file_name);

        let mut file = fs::File::open(file.path()).await?;
        let mut v = vec![];
        while let Ok(len) = file.read_u32().await {
            let mut buf = vec![0; len as usize];
            file.read_exact(&mut buf).await?;
            v.push(buf);
        }
        res.insert(file_name, v.concat());
    }

    Ok(res)
}
