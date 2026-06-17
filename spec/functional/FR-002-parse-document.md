---
id: FR-002
title: "Exported parseDocument(text) → JSON"
type: FR
---

# FR-002 — parseDocument(text) → JSON

## Description

The WASM module SHALL export `parseDocument(text)` returning the
`QuireDocument` JSON shape from `quire-rs::parse_document`. This
function is filesystem-free and is the hot path for the editor's
per-keystroke structural validation.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-002-AC-1 | `parseDocument("# Heading\n\nbody")` returns an object with a `sections` array. | Test |
| FR-002-AC-2 | Parse output is structurally equal to `serde_json::to_value(quire_rs::parse_document(text))`. | Test |
| FR-002-AC-3 | Function performs no filesystem access (works in `--target web` without a WASI shim). | Test |

## Dependencies

- **Upstream**: `quire-wasm/StR-001` (implements), `quire-rs/FR-005` (wraps)
- **Downstream**: `tests/extract_validate.rs` parity coverage
