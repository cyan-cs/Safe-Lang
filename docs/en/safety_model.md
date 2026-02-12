# SAFE? Safety Model (v1.0)

This document describes behavior enforced by `src/molding/*` and runtime memory modules.

## Core boundary
SAFE? uses three runtime pointer states:
- `core::memory::raw::RawPtr`
- `core::memory::safe::ValidatedPtr`
- `core::memory::safe::HighPtr`

Intended promotion path is:
`RawPtr -> validate_raw(...) -> ValidatedPtr -> into_high(...) -> HighPtr`

## Enforcement points
1. Molding phase 3:
- Detects raw operations (`raw_*`, `::raw::`, or calls to `raw fn` definitions).
- If a raw call expression appears outside unsafe context, it is auto-wrapped into `unsafe { ... }`.
- Verifies no raw operation remains outside unsafe context.

2. Molding phase 4:
- Rule 4 (name uniqueness): variable names must be unique globally in the source file.
- Rule 5 (prefix policy):
  - outside unsafe: name must start with `high_`
  - inside unsafe: name must start with `raw_`, `validated_`, or `high_`
- Rule 6 (promotion policy inside unsafe):
  - `validated_*` must be created by `validate_raw(raw_*)`
  - `high_*` must be created by `into_high(validated_*)`

3. Type checking:
- Raw/validated-like types are rejected outside unsafe contexts by molding before type checking continues.

## Unsafe scope model
- `unsafe { ... }` is an expression block in the AST.
- `raw fn` bodies are treated as unsafe context for molding checks.
- Nested unsafe blocks are allowed.

## Runtime safety checks (panic conditions)
Memory APIs fail fast for invalid operations:
- zero-sized allocation
- null pointer inputs
- unknown/double-freed pointers
- out-of-bounds read/write
- invalid `into_high` promotion input

These are runtime failures, not silent recovery paths.

## Example
```safe
safe fn boundary_demo() {
    let high_size: usize = 4
    let high_buf = allocate_buffer(high_size)

    unsafe {
        let raw_ptr = raw_alloc(high_size)
        raw_write(raw_ptr, 0, 65)
        let validated_ptr = validate_raw(raw_ptr)
        let high_from_raw = into_high(validated_ptr)
        deallocate_buffer(high_from_raw)
    }

    deallocate_buffer(high_buf)
}
```
