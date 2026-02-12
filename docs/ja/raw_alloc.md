# `core::memory::raw::alloc`

## シグネチャ
```safe
core::memory::raw::alloc(size: usize) -> core::memory::raw::RawPtr
```

## 振る舞い
- unmanaged な raw バイト列を確保
- 範囲検証用にサイズを tracking

## 安全性
- runtime 関数としては unsafe
- SAFE ソースでは `unsafe { ... }` 文脈で呼び出す

## panic 条件
- `size == 0`

## 備考
- 解放は `raw_deallocate`、または `validate_raw -> into_high` で昇格
