# SAFE? Runtime API (v1.0)

TypeChecker/Codegen が認識する runtime API の一覧です。

## メモリ境界 API
- `allocate_buffer(size: usize) -> core::memory::safe::HighPtr`
- `deallocate_buffer(ptr: core::memory::safe::HighPtr)`
- `raw_alloc(size: usize) -> core::memory::raw::RawPtr`
- `raw_deallocate(ptr: core::memory::raw::RawPtr)`
- `raw_write(ptr: core::memory::raw::RawPtr, offset: usize, value: u8)`
- `raw_read(ptr: core::memory::raw::RawPtr, offset: usize) -> u8`
- `validate_raw(ptr: core::memory::raw::RawPtr) -> core::memory::safe::ValidatedPtr`
- `into_high(ptr: core::memory::safe::ValidatedPtr) -> core::memory::safe::HighPtr`

詳細:
- `docs/ja/allocate_buffer.md`
- `docs/ja/deallocate_buffer.md`
- `docs/ja/raw_alloc.md`
- `docs/ja/raw_deallocate.md`
- `docs/ja/raw_write.md`
- `docs/ja/raw_read.md`
- `docs/ja/validate_raw.md`
- `docs/ja/into_high.md`

## 高水準型
- `core::types::String`: `docs/ja/string.md`
- `core::types::List`: `docs/ja/list.md`
- `core::types::Option<T>`, `core::types::Result<T, E>`:
  runtime 型は存在。言語側の builtin は現状 `*_u8`, `*_u8_i32`。

## 出力関数
- `print(...)`
- `printl(...)`
- canonical:
  - `core::types::print`
  - `core::types::printl`

## 例
- `docs/ja/std_usage.md`
