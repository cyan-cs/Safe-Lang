# SAFE? Molding (v1.0)

Implemented in `src/molding/*`. This pass runs before type checking.

## Purpose
- Normalize AST to a canonical form used by TypeChecker and CodeGenerator.
- Enforce boundary and naming rules early.

## Phase 1: alias expansion
- Loads aliases from source (`alias a = b`) and optional `rules.safe` file.
- Rejects:
  - duplicate alias names
  - alias cycles
  - alias target containing `unsafe`
- Expands alias references in function call names.
- Removes alias items from final AST.

## Phase 2: normalization
- Normalizes type aliases:
  - `HighPtr` -> `core::memory::safe::HighPtr`
  - `ValidatedPtr` -> `core::memory::safe::ValidatedPtr`
  - `RawPtr` -> `core::memory::raw::RawPtr`
- Normalizes known API function names to canonical names via `std_api`:
  - example: `raw_alloc` -> `core::memory::raw::alloc`

## Phase 3: explicit unsafe boundary
- Detects raw operations by name:
  - prefix `raw_`
  - contains `::raw::`
  - calls to user-defined `raw fn`
- If such call appears in safe context, expression is wrapped as `unsafe { ... }`.
- Verifies no raw call remains outside unsafe context.

## Phase 4: rule verification
- Rule 4: variable names are globally unique in one source file.
- Rule 5:
  - outside unsafe: only `high_*`
  - inside unsafe: `raw_*`, `validated_*`, `high_*`
- Rule 6 inside unsafe:
  - `validated_*` must be assigned from `validate_raw(raw_*)`
  - `high_*` must be assigned from `into_high(validated_*)`

## Notes
- Molding enforces naming/safety policy, not full semantic typing.
- Any phase error aborts compilation before type checking.
