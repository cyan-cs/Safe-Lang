# `core::memory::raw::read`

## Signature
```safe
core::memory::raw::read(ptr: core::memory::raw::RawPtr, offset: usize) -> u8
```

## Behavior
- Reads one byte from raw allocation at `offset`.

## Safety
- Unsafe runtime function.
- SAFE source code should call through `unsafe { ... }` context.

## Panic conditions
- null pointer
- unknown pointer
- out-of-bounds offset
