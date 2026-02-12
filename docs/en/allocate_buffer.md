# `core::memory::safe::allocate_buffer`

## Signature
```safe
core::memory::safe::allocate_buffer(size: usize) -> core::memory::safe::HighPtr
```

## Behavior
- Allocates a byte buffer tracked by safe allocation registry.
- Returns non-null `HighPtr` for valid `size`.

## Panic conditions
- `size == 0`

## Notes
- Safe API (no `unsafe` required to call).
- Returned pointer must eventually be released by `deallocate_buffer`.
