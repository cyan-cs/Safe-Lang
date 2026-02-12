# `core::types::String` (runtime)

実装: `src/core/types/string.rs`

## データモデル
- `core::memory::safe::HighPtr` を内部保持
- `len` / `cap` を管理
- Drop 時にメモリ解放

## 主要メソッド
- `String::new()`
- `String::from_bytes(&[u8])`
- `String::from_text(&str)`
- `len() -> usize`
- `is_empty() -> bool`
- `as_bytes() -> Vec<u8>`
- `to_std_string() -> std::string::String`

## 変更系
- `push_str(&str)`
- `push_bytes(&[u8])`
- `clear()`
- `clear_with_capacity()`
- `pop_byte() -> Option<u8>`
- `remove_byte(index) -> Option<u8>`

## 公開 runtime 関数
- 構築/情報:
  - `string_new`, `string_clone`, `string_len`, `string_is_empty`
- 比較/検索:
  - `string_concat`, `string_eq`, `string_substr`
  - `string_starts_with`, `string_ends_with`, `string_contains`
- 変更:
  - `string_push`, `string_push_bytes`, `string_push_str`
  - `string_clear`, `string_clear_with_capacity`, `string_append_bytes`
  - `string_pop`, `string_pop_n`, `string_remove`, `string_remove_range`
  - `string_insert_bytes`
- 変換:
  - `string_replace`, `string_trim`, `string_trim_start`, `string_trim_end`
- 分割:
  - `string_split_once`, `string_split_all`, `string_split_n`
  - `string_split_found`, `string_split_left`, `string_split_right`
  - `string_list_len`, `string_list_is_empty`, `string_list_get`
- 相互変換:
  - `string_from_list`, `string_to_list`

## 備考
- 範囲操作の一部は不正境界で panic
- TypeChecker は canonical 名を型照合に使用
