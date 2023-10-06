# compression-test
複数ファイルを一個のファイルに圧縮する crate のパフォーマンス比較

data ディレクトリに .dat ファイルを置いてプログラムを動かす

## ベンチマーク実行方法
```bash
RUST_LOG=info cargo bench
```
### 出力例（引用）
```
(br.)tar - archive      time:   [2.7632 s 2.8876 s 3.0221 s]
(br.)tar - extract      time:   [920.22 ps 924.48 ps 931.52 ps]
zip.br - archive        time:   [4.0134 s 4.0492 s 4.0873 s]
zip.br - extract        time:   [915.04 ps 921.35 ps 928.86 ps]
gzip - compress         time:   [3.4296 s 3.4565 s 3.4894 s]
gzip - decompress       time:   [554.38 ms 556.35 ms 558.58 ms]
snappy - compress       time:   [344.87 ms 345.51 ms 346.18 ms]
snappy - decompress     time:   [237.69 ms 239.16 ms 240.71 ms]
zstd - compress         time:   [467.31 ms 469.21 ms 471.24 ms]
zstd - decompress       time:   [169.42 ms 170.63 ms 171.69 ms]
```

## 圧縮率の確認
```bash
RUST_LOG=info cargo run --release
```

### 出力例（引用）
```
Origin size: 435.23 MB
(br.)tar 32.41 MB(7.45%)
zip.br 31.70 MB(7.28%)
gzip 46.52 MB(10.69%)
snappy 81.12 MB(18.64%)
zstd 38.29 MB(8.80%)
```
