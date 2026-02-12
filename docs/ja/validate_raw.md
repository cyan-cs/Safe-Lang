# `core::memory::safe::validate_raw`

## シグネチャ
```safe
core::memory::safe::validate_raw(raw_ptr: core::memory::raw::RawPtr) -> core::memory::safe::ValidatedPtr
```

## 振る舞い
- raw ポインタを昇格前検証し `ValidatedPtr` を返す
- 現在 tracking 中の raw 割り当てであることが必要

## panic 条件
- null ポインタ
- 未知ポインタ

## 備考
- safe API（`unsafe` 不要）
- この段階ではメモリ解放やコピーはしない
