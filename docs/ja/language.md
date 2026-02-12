# SAFE? 言語仕様 (v1.0)

このページは `src/lexer` と `src/parser`、および後段パスの現在実装に基づく仕様です。

## コンパイルフロー
1. CLI で `import "path.safe"` を展開（循環検出あり）
2. Lexer
3. Parser
4. Molding（正規化と安全ルール検証）
5. TypeChecker
6. Rust Codegen

## トップレベル項目
- `safe fn name(args...) { ... }`
- `raw fn name(args...) { ... }`
- `fn name(args...) { ... }`（`safe` 扱い）
- `alias short = target`

注: 現在の parser はトップレベルで function と alias のみ受理します。

## 文
- `let name = expr`
- `let name: Type = expr`
- `const name = expr`
- `const name: Type = expr`
- `if cond { ... } else { ... }`
- `for high_i in start..end { ... }`
- `for high_i in start..=end { ... }`
- `break`
- `continue`
- 式文

## 式
- 関数呼び出し: `name(arg1, arg2, ...)`
- 変数: `name`
- リテラル: 整数 / 文字列 / 真偽値
- 比較: `==`, `!=`, `<`, `<=`, `>`, `>=`
- 参照: `&expr`, `&mut expr`
- `unsafe { ... }` ブロック式

## 型
- 基本型: `i8 i16 i32 i64 isize u8 u16 u32 u64 usize bool char ()`
- パス型: `String`, `core::types::String` など
- 生ポインタ: `*T`
- 参照: `&T`, `&mut T`
- スライス風パス: `[T]`, `&[T]`, `&mut [T]`
- ジェネリクス構文は以下のみ実用サポート:
  - `Option<T>`
  - `Result<T, E>`

## 文字列とコメント
- 通常文字列: `"text"`（`\\n`, `\\r`, `\\t`, `\\"`, `\\\\`, `\\0`）
- 通常文字列は生改行を許可しません
- raw string: `r#"..."#`（複数行可）
- コメント:
  - `// ...`
  - `/* ... */`

## 現在の制限
- 宣言後代入（`=`）は未対応（`let`/`const` のみ）
- `match` / `while` / `if let` 未対応
- `Option`/`Result` 以外のユーザー向けジェネリクスは未対応
