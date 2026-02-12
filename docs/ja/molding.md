# SAFE? Molding (v1.0)

実装: `src/molding/*`  
TypeChecker 前に実行される正規化 + ルール検証パスです。

## 目的
- TypeChecker/Codegen が共有する正規 AST を作る
- 境界ルール違反を早期検出する

## Phase 1: alias 展開
- ソース内 `alias a = b` と `rules.safe` の alias を読み込み
- 禁止:
  - alias 名の重複
  - alias cycle
  - `target` に `unsafe` を含む定義
- 関数呼び出し名を展開し、AST の alias 項目を削除

## Phase 2: 正規化
- 型名正規化:
  - `HighPtr` -> `core::memory::safe::HighPtr`
  - `ValidatedPtr` -> `core::memory::safe::ValidatedPtr`
  - `RawPtr` -> `core::memory::raw::RawPtr`
- builtin 呼び出し名を canonical 名へ変換
  - 例: `raw_alloc` -> `core::memory::raw::alloc`

## Phase 3: unsafe 境界補助
- raw 操作を検出（`raw_` / `::raw::` / `raw fn` 呼び出し）
- safe 文脈の raw 呼び出し式を `unsafe { ... }` で包む
- 最終的に `unsafe` 外 raw 呼び出しがないことを検証

## Phase 4: ルール検証
- Rule 4: 変数名の全体一意性
- Rule 5:
  - `unsafe` 外: `high_` 必須
  - `unsafe` 内: `raw_` / `validated_` / `high_`
- Rule 6（`unsafe` 内）:
  - `validated_*` は `validate_raw(raw_*)`
  - `high_*` は `into_high(validated_*)`

## 備考
- Molding は型推論そのものではなく、境界・命名・正規化を担当
- いずれかの phase で失敗したらそこでコンパイル停止
