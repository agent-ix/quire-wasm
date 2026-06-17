---
id: SPEC-001
title: "quire-wasm — Specification"
type: Spec
---

# quire-wasm — Specification

## Purpose

Expose `quire-rs` (parseDocument, extract, validate) as a WebAssembly
module so that `spec-editor` and other browser/Node consumers parse and
validate spec artifacts using the **canonical** pipeline rather than a
parallel JavaScript implementation.

> **CR-001 (RETIRED render surface).** The render/templating feature was
> removed from `quire-rs` (commit `e0811a8`); `from_inline_parts` no
> longer accepts a templates map. The `render`/`renderFromBlob` exports
> and the render-parity tests are retired. [FR-001](./functional/FR-001-render.md) is RETIRED (see
> [FR-001](./functional/FR-001-render.md)); the module blob is now
> `{ manifest, schemas }`. The parse/extract/validate surface is
> unaffected.

## Scope

In scope:

- WASM cdylib wrapping the `quire-rs` parse/extract/validate functions
- npm package `@agent-ix/quire-wasm` (private to GitHub Packages at v0)
- Parity test: WASM extract/validate == `quire-rs` native for ISO archetypes

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
