# `core::memory::raw::write`

## シグネチャ
```safe
core::memory::raw::write(ptr: core::memory::raw::RawPtr, offset: usize, value: u8)
```

## 振る舞い
- raw 領域の `offset` 位置に 1 byte 書き込み

## 安全性
- runtime 関数としては unsafe
- SAFE ソースでは `unsafe { ... }` 文脈で呼び出す

## panic 条件
- null ポインタ
- 未知ポインタ
- 範囲外 offset
