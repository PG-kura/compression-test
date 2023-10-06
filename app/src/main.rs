use anyhow::Result;
use app::load_all;
use common::Sized;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let origin = load_all("data").await?;
    log::info!("Origin size: {}", get_size_s(origin.len()));

    let archive = test_tar::archive(origin.clone(), "/tmp/test.tar").await?;
    write_log("(br.)tar", &origin, archive);
    let archive = test_zip_br::archive(origin.clone(), "/tmp/test.zip.br").await?;
    write_log("zip.br", &origin, archive);

    let compressed = test_gzip::compress(&origin)?;
    write_log("gzip", &origin, compressed);
    let compressed = test_snappy::compress(&origin)?;
    write_log("snappy", &origin, compressed);
    let compressed = test_zstd::compress(&origin)?;
    write_log("zstd", &origin, compressed);

    Ok(())
}

fn write_log<S: common::Sized>(label: &str, origin: &common::Origin, obj: S) {
    let len = obj.len();
    let len_s = get_size_s(len);
    let org_len = origin.amount_size() as f64;
    let pct = 100.0 * len as f64 / org_len;
    log::info!("{label} {len_s}({:.2}%)", pct);
}

fn get_size_s(len: usize) -> String {
    humansize::format_size(len, humansize::DECIMAL)
}
