# SAFE? Runtime Usage Example (v1.0)

## Boundary example
```safe
safe fn demo() {
    let high_size: usize = 8
    let high_buf = allocate_buffer(high_size)

    unsafe {
        let raw_ptr = raw_alloc(high_size)
        raw_write(raw_ptr, 0, 65)
        raw_write(raw_ptr, 1, 66)

        let validated_ptr = validate_raw(raw_ptr)
        let high_from_raw = into_high(validated_ptr)
        deallocate_buffer(high_from_raw)
    }

    deallocate_buffer(high_buf)
}
```

## Why this matches implementation
- `raw_*` calls are unsafe-only.
- Promotion follows `raw -> validated -> high`.
- `into_high` transfers ownership of the tracked raw allocation into high allocation tracking.
- `deallocate_buffer` is required to release `HighPtr` allocations.

## Additional references
- `examples/memory_boundary.safe`
- `examples/mem_copy.safe`
- `examples/packet_parse.safe`
- `examples/standard_types.rs`
