# `core::memory::raw::deallocate`

## シグネチャ
```safe
core::memory::raw::deallocate(ptr: core::memory::raw::RawPtr)
```

## 振る舞い
- `raw_alloc` で確保した raw 領域を解放

## 安全性
- runtime 関数としては unsafe
- SAFE ソースでは `unsafe { ... }` 文脈で呼び出す

## panic 条件
- null ポインタ
- 未知ポインタ
- 二重解放

## 備考
- `into_high` 後のポインタは raw 側で再解放してはいけません
