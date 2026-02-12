# `core::memory::safe::into_high`

## シグネチャ
```safe
core::memory::safe::into_high(validated_ptr: core::memory::safe::ValidatedPtr) -> core::memory::safe::HighPtr
```

## 振る舞い
- 検証済みポインタを high 側の管理に昇格
- raw 側 tracking から high 側 tracking へ所有権を移す

## panic 条件
- null ポインタ
- 無効な validated ポインタ

## 備考
- safe API（`unsafe` 不要）
- 昇格後は `deallocate_buffer` で解放する
