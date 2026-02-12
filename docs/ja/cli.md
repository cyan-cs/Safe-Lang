# SAFE? CLI 仕様 (v1.0)

実装: `src/cli.rs`

## コマンド
- `safe build <file.safe>`
- `safe init`
- `safe init <project-name>`

## `safe build <file.safe>`
- エントリーパスを解決
- `import "relative.safe"` を再帰展開
- import 循環を検出してエラー
- lex/parse/mold/type-check/codegen を実行
- エントリーと同階層に `<entry>.rs` を出力

注:
- import は行単位構文 `import "path.safe"` のみ
- import 行は結合後ソースから除去されます

## `safe init`
- 現在ディレクトリを SAFE プロジェクト初期化
- `src/` がなければ作成
- 以下を「未存在時のみ」作成:
  - `Safe.toml`
  - `src/main.safe`

## `safe init <project-name>`
- 新規ディレクトリを作成して同様に初期化
- 既存ディレクトリ名ならエラー

## 不正引数時の usage
- `safe build <file.safe>`
- `safe init`
- `safe init <project-name>`
