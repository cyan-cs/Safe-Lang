# `core::memory::raw::deallocate`

## Signature
```safe
core::memory::raw::deallocate(ptr: core::memory::raw::RawPtr)
```

## Behavior
- Frees a raw allocation previously returned by `raw_alloc`.

## Safety
- Unsafe runtime function.
- SAFE source code should call through `unsafe { ... }` context.

## Panic conditions
- null pointer
- unknown pointer
- pointer already deallocated

## Notes
- After `validate_raw` -> `into_high`, ownership has moved to high allocator tracking.
- Do not raw-deallocate pointers already promoted into high.
