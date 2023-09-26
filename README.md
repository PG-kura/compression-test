# compression-test
複数ファイルを一個のファイルに圧縮する crate のパフォーマンス比較

data ディレクトリに .dat ファイルを置いてプログラムを動かす

## ベンチマーク実行方法
```bash
RUST_LOG=info cargo bench
```
### 出力例（引用）
```
gzip-compress           time:   [3.4034 s 3.4077 s 3.4133 s]
snappy-compress         time:   [353.57 ms 355.58 ms 357.65 ms]
zstd-compress           time:   [477.48 ms 480.90 ms 484.49 ms]
gzip-decompress         time:   [572.13 ms 575.21 ms 578.45 ms]
snappy-decompress       time:   [248.03 ms 249.65 ms 251.31 ms]
zstd-decompress         time:   [177.85 ms 179.24 ms 180.71 ms]
```

## 圧縮率の確認
```bash
RUST_LOG=info cargo run --release
```

### 出力例（引用）
```
Origin size: 435.23 MB
gzip 46.52 MB(10.69%)
snappy 81.18 MB(18.65%)
zstd 38.30 MB(8.80%)
```
