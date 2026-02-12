# `core::memory::safe::validate_raw`

## Signature
```safe
core::memory::safe::validate_raw(raw_ptr: core::memory::raw::RawPtr) -> core::memory::safe::ValidatedPtr
```

## Behavior
- Validates raw pointer shape for promotion path.
- Requires pointer to reference a currently tracked raw allocation.

## Panic conditions
- null pointer
- unknown pointer

## Notes
- Safe API (no `unsafe` required).
- This does not deallocate or copy memory.
