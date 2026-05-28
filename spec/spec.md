# quire-wasm — Specification

## Purpose

Expose `quire-rs` (render, parseDocument, extract, validate) as a
WebAssembly module so that `spec-editor` and other browser/Node
consumers render spec artifacts using the **canonical** pipeline rather
than a parallel JavaScript template engine (`nunjucks`).

Eliminating `nunjucks` removes a class of preview-vs-published drift
where the editor renders one document but `quire-cli` (and the Python
reference renderer) produce another.

## Scope

In scope:

- WASM cdylib wrapping the four `quire-rs` public functions
- npm package `@agent-ix/quire-wasm` (private to GitHub Packages at v0)
- Parity test: WASM render output == `quire-rs` native for ISO archetypes
- Per-keystroke render budget within a debounce window (NFR-001)

Out of scope (this repo):

- The `spec-editor` integration itself (one-line npm dep swap, planned
  as a follow-up in the editor repo)
- A pure-browser build with no fs (tracked as a follow-up under
  "in-memory module-blob loader" in `plan/tasks/README.md`)

## Stakeholder Requirements

See `spec/stakeholder/`.

## Functional Requirements

See `spec/functional/`. Each FR maps 1:1 to an exported WASM function.

## Non-Functional Requirements

See `spec/nfr/`.
