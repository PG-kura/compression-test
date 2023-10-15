use anyhow::Result;
use app::load_all;
use criterion::{criterion_group, criterion_main, Criterion};
use tempdir::TempDir;
use tokio::runtime::Runtime;

pub fn benchmark(c: &mut Criterion) -> Result<()> {
    env_logger::init();
    let rt = Runtime::new()?;
    let origin = rt.block_on(load_all("../data"))?;

    let dir = TempDir::new("test").unwrap();
    let path = dir.path().join("test.zip.zstd");

    c.bench_function("zip.snap - archive", |b| {
        b.iter(||
            test_zip_snappy::archive(origin.clone(), &path).unwrap()
        );
    });
    let archive = common::Archive::new(&path)?;
    log::info!("zip.snap - archive: {}", archive.size);

    let archive = common::Archive::new(&path)?;
    c.bench_function("zip.snap - extract", |b| {
        b.iter(|| async {
            test_zip_snappy::extract(archive.clone()).unwrap()
        });
    });

    Ok(())
}

criterion_group! {
    name = benches;
    config = Criterion::default()
                .sample_size(30)
                .without_plots();
    targets = benchmark
}
criterion_main!(benches);
