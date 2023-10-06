use anyhow::Result;
use app::load_all;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn benchmark(c: &mut Criterion) -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    let origin = rt.block_on(load_all("../data"))?;

    c.bench_function("gzip - compress", |b| {
        b.iter(|| test_gzip::compress(&origin).unwrap())
    });
    let compressed = test_gzip::compress(&origin)?;
    c.bench_function("gzip - decompress", |b| {
        b.iter(|| test_gzip::decompress(&compressed).unwrap())
    });
    let compressed = test_snappy::compress(&origin)?;
    c.bench_function("snappy - decompress", |b| {
        b.iter(|| test_snappy::decompress(&compressed).unwrap())
    });
    c.bench_function("snappy - compress", |b| {
        b.iter(|| test_snappy::compress(&origin).unwrap())
    });
    let compressed = test_zstd::compress(&origin)?;
    c.bench_function("zstd - decompress", |b| {
        b.iter(|| test_zstd::decompress(&compressed).unwrap())
    });
    c.bench_function("zstd - compress", |b| {
        b.iter(|| test_zstd::compress(&origin).unwrap())
    });

    Ok(())
}

criterion_group! {
    name = benches;
    config = Criterion::default()
                .sample_size(10)
                .without_plots();
    targets = benchmark
}
criterion_main!(benches);
