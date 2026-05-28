---
id: FR-002
title: "Exported parseDocument(text) → JSON"
artifact_type: FR
---

# FR-002 — parseDocument(text) → JSON

## Statement

The WASM module SHALL export `parseDocument(text)` returning the
`QuireDocument` JSON shape from `quire-rs::parse_document`. This
function is filesystem-free and is the hot path for the editor's
per-keystroke structural validation.

## Acceptance Criteria

| ID | Criteria |
|----|----------|
| FR-002-AC-1 | `parseDocument("# Heading\n\nbody")` returns an object with a `sections` array. |
| FR-002-AC-2 | Parse output is structurally equal to `serde_json::to_value(quire_rs::parse_document(text))`. |
| FR-002-AC-3 | Function performs no filesystem access (works in `--target web` without a WASI shim). |

## Relationships

- implements: `quire-wasm/StR-001`
- wraps: `quire-rs/FR-005`
