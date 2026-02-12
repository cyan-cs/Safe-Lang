# SAFE? 安全モデル (v1.0)

この文書は `src/molding/*` と runtime メモリ実装に基づく、実装準拠の安全モデルです。

## 境界モデル
SAFE? は 3 つのポインタ状態を使います。
- `core::memory::raw::RawPtr`
- `core::memory::safe::ValidatedPtr`
- `core::memory::safe::HighPtr`

昇格経路:
`RawPtr -> validate_raw(...) -> ValidatedPtr -> into_high(...) -> HighPtr`

## 強制ポイント
1. Molding phase 3
- raw 操作（`raw_` / `::raw::` / `raw fn` 呼び出し）を検出
- safe 文脈での raw 呼び出し式は `unsafe { ... }` で自動ラップ
- `unsafe` 外に raw 操作が残っていないことを検証

2. Molding phase 4
- Rule 4（名前一意性）: 変数名はソース全体で重複不可
- Rule 5（接頭辞ルール）
  - `unsafe` 外: `high_` 必須
  - `unsafe` 内: `raw_` / `validated_` / `high_`
- Rule 6（昇格ルール）
  - `validated_*` は `validate_raw(raw_*)` で生成
  - `high_*` は `into_high(validated_*)` で生成

3. TypeChecker
- 境界違反は主に Molding で止め、TypeChecker は正規化済み AST を検査

## unsafe スコープ
- `unsafe { ... }` は AST 上のブロック式
- `raw fn` 本体は unsafe 文脈として扱われる
- nested unsafe は許可

## Runtime 側の失敗（panic）
- 0 サイズ確保
- null ポインタ
- 不正 / 二重解放ポインタ
- 範囲外 read/write
- 無効な `into_high` 入力

これは型システムの失敗ではなく実行時失敗です。
