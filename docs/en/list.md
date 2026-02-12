# `core::types::List` (runtime)

Implemented in `src/core/types/list.rs`.

## Data model
- Dynamic byte list (`u8` elements).
- Backed by `core::memory::safe::HighPtr`.
- Owns allocation and releases it on drop.

## Core methods
- `List::new()`
- `len() -> usize`
- `is_empty() -> bool`
- `push(u8)`
- `get(usize) -> Option<u8>`
- `to_vec() -> Vec<u8>`

## Exported runtime API functions
- `list_new() -> List`
- `list_len(&List) -> usize`
- `list_is_empty(&List) -> bool`
- `list_push_u8(&mut List, u8)`
- `list_get_u8(&List, usize) -> Option<u8>`
- `list_push_bytes(&mut List, &List)`

## Notes
- Out-of-range `get` returns `Option::None`.
- Capacity grows automatically by doubling strategy.
