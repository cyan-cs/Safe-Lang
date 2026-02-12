# `core::types::List` (runtime)

実装: `src/core/types/list.rs`

## データモデル
- `u8` 専用の可変長リスト
- `core::memory::safe::HighPtr` バックエンド
- Drop 時にメモリ解放

## 主要メソッド
- `List::new()`
- `len() -> usize`
- `is_empty() -> bool`
- `push(u8)`
- `get(usize) -> Option<u8>`
- `to_vec() -> Vec<u8>`

## 公開 runtime 関数
- `list_new() -> List`
- `list_len(&List) -> usize`
- `list_is_empty(&List) -> bool`
- `list_push_u8(&mut List, u8)`
- `list_get_u8(&List, usize) -> Option<u8>`
- `list_push_bytes(&mut List, &List)`

## 備考
- 範囲外 `get` は `Option::None`
- 容量は倍々で拡張
