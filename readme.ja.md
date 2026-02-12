# SAFE? ドキュメント（v1.0）

このディレクトリは、現在の実装に対応する**公開用ドキュメント一式**です。  
仕様とドキュメントの内容が食い違う場合は、`src/` 内の実装を正とします。

## 正式仕様（Canonical specs）
- 言語表面仕様・構文: `docs/en/language.md`
- セーフティモデルと強制ルール: `docs/en/safety_model.md`
- 型システムおよび型検査の挙動: `docs/en/type_system.md`
- モールディング工程と正規化: `docs/en/molding.md`
- CLI の挙動: `docs/en/cli.md`

## ランタイム API
- ランタイム API 概要: `docs/en/std_memory.md`
- 境界使用例: `docs/en/std_usage.md`
- `allocate_buffer`: `docs/en/allocate_buffer.md`
- `deallocate_buffer`: `docs/en/deallocate_buffer.md`
- `raw_alloc`: `docs/en/raw_alloc.md`
- `raw_deallocate`: `docs/en/raw_deallocate.md`
- `raw_write`: `docs/en/raw_write.md`
- `raw_read`: `docs/en/raw_read.md`
- `validate_raw`: `docs/en/validate_raw.md`
- `into_high`: `docs/en/into_high.md`
- `core::types::String`: `docs/en/string.md`
- `core::types::List`: `docs/en/list.md`

## リリースノート
- 日本語: `docs/ja/release_notes_v1.0_ja.md`
- 英語: `docs/en/release_notes_v1.0_en.md`
