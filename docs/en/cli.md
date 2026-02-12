# SAFE? CLI Reference (v1.0)

Implemented in `src/cli.rs`.

## Commands
- `safe build <file.safe>`
- `safe init`
- `safe init <project-name>`

## `safe build <file.safe>`
- Resolves the input path (`canonicalize`).
- Recursively expands `import "relative.safe"` lines.
- Detects import cycles and fails with an error chain.
- Runs compile pipeline (lex/parse/mold/type-check/codegen).
- Writes generated Rust next to the entry file (`<entry>.rs`).

Notes:
- Import syntax is line-based and exact: `import "path.safe"`.
- Import lines are removed from merged source before parsing.

## `safe init`
- Initializes current directory as a SAFE project.
- Creates `src/` if missing.
- Writes files only if missing:
  - `Safe.toml`
  - `src/main.safe`

## `safe init <project-name>`
- Creates a new directory then runs the same initialization.
- Fails if directory already exists.

## Usage text
If arguments are invalid, CLI returns:
`Usage:`
- `safe build <file.safe>`
- `safe init`
- `safe init <project-name>`
