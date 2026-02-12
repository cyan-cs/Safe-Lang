# `core::memory::raw::alloc`

## Signature
```safe
core::memory::raw::alloc(size: usize) -> core::memory::raw::RawPtr
```

## Behavior
- Allocates unmanaged raw bytes and tracks allocation size for bounds checks.

## Safety
- Unsafe runtime function.
- SAFE source code should call through `unsafe { ... }` context.

## Panic conditions
- `size == 0`

## Notes
- Returned pointer must be consumed by `raw_deallocate` or promoted via `validate_raw` -> `into_high`.
