# `core::memory::raw::write`

## Signature
```safe
core::memory::raw::write(ptr: core::memory::raw::RawPtr, offset: usize, value: u8)
```

## Behavior
- Writes one byte to raw allocation at `offset`.

## Safety
- Unsafe runtime function.
- SAFE source code should call through `unsafe { ... }` context.

## Panic conditions
- null pointer
- unknown pointer
- out-of-bounds offset
