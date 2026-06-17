---
id: FR-003
title: "Exported extract(archetype, moduleRoot, doc) → records"
type: FR
---

# FR-003 — extract(archetype, moduleRoot, doc) → records

## Description

The WASM module SHALL export `extract(archetype, moduleRoot, doc)`
which loads the archetype's body-extraction DSL, applies it to the
parsed `doc`, and returns the resulting records + edges as a JS object.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-003-AC-1 | For an archetype without `body_extraction`, the call throws `JsError` mentioning "no body extraction". | Test |
| FR-003-AC-2 | For a valid archetype + doc, the returned object's JSON shape matches `serde_json::to_value(quire_rs::extract(...).unwrap())`. | Test |
| FR-003-AC-3 | `extractFromBlob(archetype, moduleBlob, doc)` returns the same shape as `extract(archetype, root, doc)` when `moduleBlob` is the in-memory equivalent of `root` (FR-001 amendment, T-008). | Test |

## Dependencies

- **Upstream**: `quire-wasm/StR-001` (implements), `quire-rs/FR-011`, `quire-rs/FR-016` (wraps)
- **Downstream**: `tests/extract_validate.rs` extract coverage
