use anyhow::Result;
use app::load_all;
use criterion::{criterion_group, criterion_main, Criterion};
use tempdir::TempDir;
use tokio::runtime::Runtime;

pub fn benchmark(c: &mut Criterion) -> Result<()> {
    let rt = Runtime::new()?;
    let origin = rt.block_on(load_all("../data"))?;

    let dir = TempDir::new("test").unwrap();

    let path = dir.path().join("test.tar");
    c.bench_function("(br.)tar - archive", |b| {
        b.to_async(&rt).iter(|| async {
            test_tar::archive(origin.clone(), &path).await.unwrap();
        });
    });
    let archive = common::Archive::new(&path)?;
    c.bench_function("(br.)tar - extract", |b| {
        b.iter(|| async {
            test_tar::extract(archive.clone()).unwrap();
        });
    });

    let path = dir.path().join("test.zip.br");
    c.bench_function("zip.br - archive", |b| {
        b.to_async(&rt).iter(|| async {
            test_zip_br::archive(origin.clone(), &path).await.unwrap();
        });
    });
    let archive = common::Archive::new(&path)?;
    c.bench_function("zip.br - extract", |b| {
        b.iter(|| async {
            test_zip_br::extract(archive.clone()).unwrap();
        });
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
