# SAFE? Runtime 利用例 (v1.0)

## 境界サンプル
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

## 実装との対応
- raw 操作は `unsafe` 内のみ
- 昇格は `raw -> validated -> high`
- `into_high` は raw 側トラッキングから high 側へ所有権移動
- `HighPtr` は `deallocate_buffer` で解放

## 追加参照
- `examples/memory_boundary.safe`
- `examples/mem_copy.safe`
- `examples/packet_parse.safe`
- `examples/standard_types.rs`
