# schema-codegen

FlatBuffersスキーマ(`.fbs`)を単一の起点(Single Source of Truth)として、
Rust構造体(ECS Component等)とC++ヘッダ / cxxブリッジ定義を自動生成するツール。

`Engine-Agnostic-Rust-Template` 本体プロジェクトから独立させ、
将来的に他のシステムプロジェクトからも再利用できることを意図している。

## 構成

- `crates/schema-ir` — `.fbs` + 独自アノテーションをパースし、言語非依存の中間表現(IR)に変換
- `crates/codegen-rust` — IR → Rust構造体を生成
- `crates/codegen-cxx` — IR → C++ヘッダ / cxxブリッジ定義を生成
- `crates/cli` — 上記を束ねる実行バイナリ
- `python/` — スキーマ検証のプロトタイピング、IRのデバッグ可視化、ビルドオーケストレーション(nox)

## 現状

ビルドシステムの設定のみ。実装は未着手(すべてのcrateはplaceholder)。

## 本体プロジェクトからの参照方法(想定)

```toml
[dependencies]
schema-codegen-cli = { git = "https://github.com/you/schema-codegen", tag = "v0.1.0" }
```

開発中はローカルpathへ一時的に差し替える:

```toml
[patch."https://github.com/you/schema-codegen"]
schema-codegen-cli = { path = "../schema-codegen/crates/cli" }
```
