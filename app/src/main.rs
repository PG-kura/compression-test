use anyhow::Result;
use app::load_all;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    log::info!("Read data...");
    let origin = load_all("data").await?;
    log::info!("Origin size: {}", &origin);

    let write_log = |label, archive: common::Archive| {
        let org_len = origin.amount_size() as f64;
        let pct = 100.0 * archive.len() as f64 / org_len;
        log::info!("{label} {archive}({:.2}%)", pct);
    };

    let archive = test_gzip::compress(&origin)?;
    write_log("gzip", archive);
    let archive = test_snappy::compress(&origin)?;
    write_log("snappy", archive);
    let archive = test_zstd::compress(&origin)?;
    write_log("zstd", archive);

    Ok(())
}
