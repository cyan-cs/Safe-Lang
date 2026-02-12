# `core::memory::safe::deallocate_buffer`

## シグネチャ
```safe
core::memory::safe::deallocate_buffer(ptr: core::memory::safe::HighPtr)
```

## 振る舞い
- `allocate_buffer` または `into_high` で管理対象になった領域を解放

## panic 条件
- null ポインタ
- 未知ポインタ
- 二重解放

## 備考
- safe API（`unsafe` 不要）
- 解放後のポインタ再利用は不可
