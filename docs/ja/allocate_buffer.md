# `core::memory::safe::allocate_buffer`

## シグネチャ
```safe
core::memory::safe::allocate_buffer(size: usize) -> core::memory::safe::HighPtr
```

## 振る舞い
- safe 側の割り当て管理テーブルに登録されたバッファを確保
- 正常時は non-null の `HighPtr` を返す

## panic 条件
- `size == 0`

## 備考
- safe API（呼び出しに `unsafe` 不要）
- 返却されたポインタは最終的に `deallocate_buffer` で解放が必要
