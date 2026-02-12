# `core::memory::safe::deallocate_buffer`

## Signature
```safe
core::memory::safe::deallocate_buffer(ptr: core::memory::safe::HighPtr)
```

## Behavior
- Releases a safe-tracked allocation created by `allocate_buffer` or promoted via `into_high`.

## Panic conditions
- null pointer
- unknown pointer
- pointer already deallocated

## Notes
- Safe API (no `unsafe` required to call).
- Pointer must not be reused after deallocation.
