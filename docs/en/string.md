# `core::types::String` (runtime)

Implemented in `src/core/types/string.rs`.

## Data model
- Backed by `core::memory::safe::HighPtr`
- Tracks `len` and `cap`
- Owns memory and releases it on drop

## Core methods
- `String::new()`
- `String::from_bytes(&[u8])`
- `String::from_text(&str)`
- `len() -> usize`
- `is_empty() -> bool`
- `as_bytes() -> Vec<u8>`
- `to_std_string() -> std::string::String`

## Mutation helpers
- `push_str(&str)`
- `push_bytes(&[u8])`
- `clear()`
- `clear_with_capacity()`
- `pop_byte() -> Option<u8>`
- `remove_byte(index) -> Option<u8>`

## Exported runtime API functions
- constructors/info:
  - `string_new`, `string_clone`, `string_len`, `string_is_empty`
- compare/search:
  - `string_concat`, `string_eq`, `string_substr`
  - `string_starts_with`, `string_ends_with`, `string_contains`
- mutation:
  - `string_push`, `string_push_bytes`, `string_push_str`
  - `string_clear`, `string_clear_with_capacity`, `string_append_bytes`
  - `string_pop`, `string_pop_n`, `string_remove`, `string_remove_range`
  - `string_insert_bytes`
- transform:
  - `string_replace`, `string_trim`, `string_trim_start`, `string_trim_end`
- split:
  - `string_split_once`, `string_split_all`, `string_split_n`
  - `string_split_found`, `string_split_left`, `string_split_right`
  - `string_list_len`, `string_list_is_empty`, `string_list_get`
- conversion:
  - `string_from_list`, `string_to_list`

## Notes
- Many range-based operations panic on invalid bounds.
- TypeChecker recognizes canonical string type names for function signatures.
