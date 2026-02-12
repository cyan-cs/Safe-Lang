# SAFE? Runtime APIs (v1.0)

This page indexes memory and core runtime APIs that the checker/codegen know.

## Memory boundary API
- `allocate_buffer(size: usize) -> core::memory::safe::HighPtr`
- `deallocate_buffer(ptr: core::memory::safe::HighPtr)`
- `raw_alloc(size: usize) -> core::memory::raw::RawPtr`
- `raw_deallocate(ptr: core::memory::raw::RawPtr)`
- `raw_write(ptr: core::memory::raw::RawPtr, offset: usize, value: u8)`
- `raw_read(ptr: core::memory::raw::RawPtr, offset: usize) -> u8`
- `validate_raw(ptr: core::memory::raw::RawPtr) -> core::memory::safe::ValidatedPtr`
- `into_high(ptr: core::memory::safe::ValidatedPtr) -> core::memory::safe::HighPtr`

Details:
- `docs/en/allocate_buffer.md`
- `docs/en/deallocate_buffer.md`
- `docs/en/raw_alloc.md`
- `docs/en/raw_deallocate.md`
- `docs/en/raw_write.md`
- `docs/en/raw_read.md`
- `docs/en/validate_raw.md`
- `docs/en/into_high.md`

## Runtime high-level types
- `core::types::String`: `docs/en/string.md`
- `core::types::List`: `docs/en/list.md`
- `core::types::Option<T>`, `core::types::Result<T, E>`:
  runtime types exist; language-facing builtins are currently specialized
  (`*_u8`, `*_u8_i32`) in `src/std_api.rs`.

## Print functions
- `print(...)`
- `printl(...)`
- Canonical runtime implementations:
  - `core::types::print`
  - `core::types::printl`

## Example
- `docs/en/std_usage.md`
