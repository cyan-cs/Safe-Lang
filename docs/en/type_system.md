# SAFE? Type System (v1.0)

Implemented by `src/type_checker/*`, parser type nodes, and `src/std_api.rs`.

## Type representation
- `Type::Path(String)`
- `Type::RawPtr(Box<Type>)`
- `Type::Ref { mutable: bool, inner: Box<Type> }`

## Built-in primitive type names
- Integers: `i8`, `i16`, `i32`, `i64`, `isize`, `u8`, `u16`, `u32`, `u64`, `usize`
- `bool`, `char`, `String`, `()`

`std_api` canonicalization also recognizes:
- `core::types::String`, `core::types::List`
- `core::types::Option`, `core::types::Result`
- `core::memory::raw::RawPtr`
- `core::memory::safe::ValidatedPtr`
- `core::memory::safe::HighPtr`

## Literal inference
- integer literal => `i32`
- string literal => `String`
- bool literal => `bool`

## Function/type checks
1. Declarations:
- Duplicate function definitions are rejected.
- Built-in API names cannot be redefined.
- Unknown type names are rejected.

2. Calls:
- Argument count must match exactly, except variadic `print`/`printl`.
- Argument type must match declared type.
- Integer literals can satisfy integer-typed parameters.

3. Returns:
- Function return is inferred from the block tail expression.
- If no explicit return type, expected return is `()`.
- Integer tail literal is accepted for any integer return type.

4. Conditions and loops:
- `if` condition must be `bool`.
- `for` bounds must be integer-compatible.
- Loop variable type is inferred from bounds.

5. Control-flow statements:
- `break` and `continue` are valid only inside `for` loops.

## Comparison typing
- `==` / `!=`: operands must be same type or both integer-compatible.
- `<` `<=` `>` `>=`: both operands must be integer-compatible.
- Comparison result type is `bool`.

## Generic syntax support
- Parser accepts generic-looking path types.
- TypeChecker allows generic semantics only for:
  - `Option<T>`
  - `Result<T, E>`
  and canonical equivalents under `core::types`.
- Other generic type uses are rejected (for example `List<u8>`).

## Printability rules (`print` / `printl`)
Supported printable types:
- `String`, `core::types::String`
- `bool`, `char`
- integer primitives
- references to printable types

Rejected:
- raw pointers and non-printable structured values (for example `List`).
