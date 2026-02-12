# SAFE? v1.0 Release Notes (English)

Release date: 2026-02-12

## Summary
SAFE? v1.0 is the first stable release with a fixed `Raw -> Validated -> High` boundary.  
The project now aligns language behavior, type checking, runtime APIs, and CLI around one rule: safe by default, explicit and localized low-level operations.

## Highlights
1. CLI
- `safe build <file.safe>`: transpile with import expansion
- `safe init`: initialize current directory
- `safe init <project-name>`: create and initialize a new project

2. Import resolution
- Recursive `import "x.safe"` resolution
- Dependency cycle detection to prevent infinite loops

3. Core language syntax
- Variables: `let`, `const`
- Control flow: `if / else`, `for`, `break`, `continue`
- Comments: `//`, `/* ... */`
- Strings:
  - Standard strings do not allow raw newlines
  - `\n` escapes supported
  - raw strings (`r#"..."#`) supported

4. Output functions
- `print(...)`: no trailing newline
- `printl(...)`: auto trailing newline
- Variadic arguments supported
- Automatic printable conversion for core primitive values

5. Safety model enforcement
- Raw operations are blocked outside `unsafe` blocks
- Naming and transition rules (`raw_`, `validated_`, `high_`) are enforced by Molding + TypeChecker

6. Runtime standard types
- `core::types::String`
- `core::types::List`
- Minimal ADTs: `Option<T>`, `Result<T, E>` (limited v1.0 scope)

## Compatibility Policy (v1.x)
- v1.0 safety boundaries are frozen.
- v1.x should remain additive and backward-compatible.
- Any change that weakens boundary guarantees is a major-version concern.

## Known v1.0 Limits
- No full generics system, borrow/lifetime/effect system
- No advanced pattern matching (`match`, `if let`)

## References
- Safety model: `docs/en/safety_model.md`
- Type system baseline: `docs/en/type_system.md`
- Molding behavior: `docs/en/molding.md`
- Examples: `examples/README.md`
