# S5 Canonical Programs

- `binary_patch.safe`: raw byte patching and explicit `Raw -> Validated -> High`.
- `mem_copy.safe`: minimal raw memory copy pattern and safe promotion.
- `buffer_guard.safe`: wrapping raw ownership transfer behind a dedicated function.
- `packet_parse.safe`: low-level packet read sequence with bounded unsafe block.
- `builder_bridge.safe`: safe builder API backed by raw initialization.
- `option_result_ok.safe`: minimal success flow with `Option` and `Result`.
- `option_result_fail.safe`: explicit failure flow via `unwrap` on `None` / `Err`.
- `memory_boundary.safe`: baseline unsafe boundary example.
- `standard_types.rs`: runtime-side `String` / `List` usage without pointer exposure.
