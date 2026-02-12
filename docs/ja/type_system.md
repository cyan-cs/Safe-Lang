# SAFE? 型システム (v1.0)

実装: `src/type_checker/*`, parser の型ノード, `src/std_api.rs`

## 型表現
- `Type::Path(String)`
- `Type::RawPtr(Box<Type>)`
- `Type::Ref { mutable: bool, inner: Box<Type> }`

## 基本型名
- 整数: `i8`, `i16`, `i32`, `i64`, `isize`, `u8`, `u16`, `u32`, `u64`, `usize`
- `bool`, `char`, `String`, `()`

`std_api` 側の正規化で以下も既知:
- `core::types::String`, `core::types::List`
- `core::types::Option`, `core::types::Result`
- `core::memory::raw::RawPtr`
- `core::memory::safe::ValidatedPtr`
- `core::memory::safe::HighPtr`

## リテラル推論
- 整数リテラル => `i32`
- 文字列リテラル => `String`
- bool リテラル => `bool`

## 主な検査
1. 宣言
- 関数重複定義はエラー
- builtin 名の再定義はエラー
- 未知型はエラー

2. 関数呼び出し
- `print`/`printl` 以外は引数個数一致が必須
- 引数型は宣言型と一致必須
- 整数リテラルは整数型引数に適合可

3. 戻り値
- 関数末尾式から推論
- 省略時の期待型は `()`
- 末尾が整数リテラルなら任意整数戻り型に適合可

4. 制御構文
- `if` 条件は `bool` 必須
- `for` 範囲境界は整数必須
- ループ変数型は境界型から推論
- `break` / `continue` は `for` 内のみ

## 比較式
- `==` / `!=`: 同型または整数同士
- `<`, `<=`, `>`, `>=`: 整数同士のみ
- 結果型は `bool`

## ジェネリクス制約
- 構文は一般形を読めるが、意味的に許可するのは:
  - `Option<T>`
  - `Result<T, E>`
  （`core::types::...` 含む）
- それ以外（例: `List<u8>`）はエラー

## `print`/`printl` の許可型
- `String`, `core::types::String`
- `bool`, `char`
- 整数基本型
- 上記への参照

`List` など非対応型はエラーになります。
