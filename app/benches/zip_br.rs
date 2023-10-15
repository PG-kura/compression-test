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
    let path = dir.path().join("test.br.zstd");

    for quality in 1..=6 {
        let caption = format!("zip.br - archive({})", quality);
        c.bench_function(&caption, |b| {
            b.iter(||
                test_zip_br::archive(origin.clone(), &path, quality).unwrap()
            );
        });
        let archive = common::Archive::new(&path)?;
        log::info!("zip.br - archive({}): {}", quality, archive.size);
    }
    let archive = common::Archive::new(&path)?;
    c.bench_function("zip.zstd - extract", |b| {
        b.iter(|| async {
            test_zip_br::extract(archive.clone()).unwrap()
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
