# Tasks

| ID | Title | Phase | Status |
|----|-------|-------|--------|
| T-001 | Repo skeleton (Cargo + package.json + Makefile + CI + hygiene) | 1 | DONE |
| T-002 | Public surface + Node parity test for `render` and `parseDocument` | 1 | DONE |
| T-003 | Negative-path render/parse ACs | 2 | TODO |
| T-004 | Extract + validate ACs | 2 | TODO |
| T-005 | Criterion bench + baseline + CI band (NFR-001) | 3 | TODO |
| T-006 | GitHub Packages publish workflow (tag-driven) | 4 | TODO |
| T-007 | Drop `private: true`, bump to v0.2.0 | 4 | TODO |
| T-008 | In-memory module-blob loader (cross-repo: quire-rs + quire-wasm) | 5 | TODO |
| T-009 | spec-editor swap: nunjucks → @agent-ix/quire-wasm (in spec-editor repo) | 5 | TODO |

## Known blocker — jsonschema 0.18 on wasm32

`quire-rs` depends on `jsonschema = "0.18"` with `resolve-file`, which
calls `url::Url::to_file_path` — a method `url` only exposes under
`cfg(any(unix, windows))`. On `wasm32-unknown-unknown` the trait method
is missing and `cargo build --target wasm32-unknown-unknown` fails
inside `jsonschema`.

Workarounds (any one unblocks T-002's wasm-pack lane):

1. Upstream a `wasm` feature in `quire-rs` that drops the
   `resolve-file` activation of `jsonschema` (and disables FS-backed
   `$ref` resolution on wasm).
2. Wait on `jsonschema 0.20+` which restructures the resolver.
3. Vendor a minimal validator wrapper inside `quire-wasm` that skips
   `$ref` resolution.

Until then `cargo check --lib` passes on the **native** target (proving
the binding code compiles and links against `quire-rs`), and
`wasm-pack test --node` is gated behind T-008's prereq.

## In-memory module loader (T-008) — design note

The scaffold turn (T-001..002) ships a WASM whose `moduleRoot` is a
filesystem path. This works for Node consumers (real fs) and CI parity
tests, but blocks a pure-browser build under `--target web`.

T-008 will upstream a `Registry::from_blob(json: &Value) -> Registry`
constructor in `quire-rs` that takes the manifest + schemas + templates
inline, bypassing the `loader::*` filesystem walk. quire-wasm then
exposes `renderFromBlob`, `extractFromBlob`, `validateFromBlob` that
match the FS-shape ADR (a) in `README.md`.

Rationale: the spec-editor already holds the active module in memory,
so a JSON blob is the natural interchange. The current path-string
shape stays for Node tooling that already lives on disk.
