# `core::memory::safe::into_high`

## Signature
```safe
core::memory::safe::into_high(validated_ptr: core::memory::safe::ValidatedPtr) -> core::memory::safe::HighPtr
```

## Behavior
- Promotes validated pointer into safe allocation tracking.
- Transfers allocation ownership from raw tracker to high tracker.

## Panic conditions
- null pointer
- unknown/invalid validated pointer

## Notes
- Safe API (no `unsafe` required).
- After promotion, deallocate with `deallocate_buffer`.
