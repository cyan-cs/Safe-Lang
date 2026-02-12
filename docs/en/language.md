# SAFE? Language Reference (v1.0)

This page documents the currently implemented language surface in `src/lexer`, `src/parser`, and downstream passes.

## Compilation flow
1. CLI import expansion (`import "path.safe"`) with cycle detection.
2. Lexing to tokens.
3. Parsing to AST.
4. Molding (alias expansion, normalization, safety rules).
5. Type checking.
6. Rust code generation.

## Top-level items
- `safe fn name(args...) { ... }`
- `raw fn name(args...) { ... }`
- `fn name(args...) { ... }` (defaults to `safe`)
- `alias short = target`

Note: parser currently accepts only functions and aliases at top level.

## Statements
- `let name = expr`
- `let name: Type = expr`
- `const name = expr`
- `const name: Type = expr`
- `if cond { ... } else { ... }`
- `for high_i in start..end { ... }`
- `for high_i in start..=end { ... }`
- `break`
- `continue`
- expression statement

## Expressions
- Function call: `name(arg1, arg2, ...)`
- Variable: `name`
- Literals: integer, string, bool (`true` / `false`)
- Comparisons: `==`, `!=`, `<`, `<=`, `>`, `>=`
- Reference: `&expr`, `&mut expr`
- Unsafe block expression: `unsafe { ... }`

## Types
- Primitive names: `i8 i16 i32 i64 isize u8 u16 u32 u64 usize bool char ()`
- Path type: `String`, `core::types::String`, etc.
- Raw pointer: `*T`
- Reference: `&T`, `&mut T`
- Slice-like path forms are parsed (`[T]`, `&[T]`, `&mut [T]`) and handled as path-style types.
- Generic syntax is accepted only for:
  - `Option<T>`
  - `Result<T, E>`
  and canonical `core::types::Option<T>`, `core::types::Result<T, E>`.

## Strings and comments
- Normal string literals: `"text"` with escapes (`\n`, `\r`, `\t`, `\"`, `\\`, `\0`).
- Normal strings reject raw newlines.
- Raw strings: `r#"..."#` (hash count may vary), multiline allowed.
- Comments:
  - line: `// ...`
  - block: `/* ... */`

## Current limitations
- No assignment operator after declaration (`let`/`const` only).
- No `match`, no `while`, no pattern matching (`if let`).
- No user-level generic types beyond Option/Result syntax.
