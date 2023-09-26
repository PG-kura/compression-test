use anyhow::Result;
use app::load_all;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn benchmark(c: &mut Criterion) -> Result<()> {
    env_logger::init();

    log::info!("Read data...");
    let rt = tokio::runtime::Runtime::new()?;
    let origin = rt.block_on(load_all("../data"))?;

    log::info!("Start benchmark...");
    c.bench_function("gzip-compress", |b| {
        b.iter(|| test_gzip::compress(&origin).unwrap())
    });
    c.bench_function("snappy-compress", |b| {
        b.iter(|| test_snappy::compress(&origin).unwrap())
    });
    c.bench_function("zstd-compress", |b| {
        b.iter(|| test_zstd::compress(&origin).unwrap())
    });

    let archive = test_gzip::compress(&origin)?;
    c.bench_function("gzip-decompress", |b| {
        b.iter(|| test_gzip::decompress(&archive).unwrap())
    });
    let archive = test_snappy::compress(&origin)?;
    c.bench_function("snappy-decompress", |b| {
        b.iter(|| test_snappy::decompress(&archive).unwrap())
    });
    let archive = test_zstd::compress(&origin)?;
    c.bench_function("zstd-decompress", |b| {
        b.iter(|| test_zstd::decompress(&archive).unwrap())
    });

    Ok(())
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
