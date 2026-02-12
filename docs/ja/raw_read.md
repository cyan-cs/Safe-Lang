# `core::memory::raw::read`

## シグネチャ
```safe
core::memory::raw::read(ptr: core::memory::raw::RawPtr, offset: usize) -> u8
```

## 振る舞い
- raw 領域の `offset` 位置から 1 byte 読み取り

## 安全性
- runtime 関数としては unsafe
- SAFE ソースでは `unsafe { ... }` 文脈で呼び出す

## panic 条件
- null ポインタ
- 未知ポインタ
- 範囲外 offset
