---
id: FR-003
title: "Exported extract(archetype, moduleRoot, doc) → records"
artifact_type: FR
---

# FR-003 — extract(archetype, moduleRoot, doc) → records

## Statement

The WASM module SHALL export `extract(archetype, moduleRoot, doc)`
which loads the archetype's body-extraction DSL, applies it to the
parsed `doc`, and returns the resulting records + edges as a JS object.

## Acceptance Criteria

| ID | Criteria |
|----|----------|
| FR-003-AC-1 | For an archetype without `body_extraction`, the call throws `JsError` mentioning "no body extraction". |
| FR-003-AC-2 | For a valid archetype + doc, the returned object's JSON shape matches `serde_json::to_value(quire_rs::extract(...).unwrap())`. |
| FR-003-AC-3 | `extractFromBlob(archetype, moduleBlob, doc)` returns the same shape as `extract(archetype, root, doc)` when `moduleBlob` is the in-memory equivalent of `root` (FR-001 amendment, T-008). |

## Relationships

- implements: `quire-wasm/StR-001`
- wraps: `quire-rs/FR-011`, `quire-rs/FR-016`
