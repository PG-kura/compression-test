# compression-test
複数ファイルを一個のファイルに圧縮する crate のパフォーマンス比較

data ディレクトリに .dat ファイルを置いてプログラムを動かす

## ベンチマーク実行方法
```bash
RUST_LOG=info cargo bench
```

## 実行結果のまとめ(例)
![graph](graph.svg)
