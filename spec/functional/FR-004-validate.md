---
id: FR-004
title: "Exported validate_archetype(archetype, moduleRoot, data) → ok | throw"
artifact_type: FR
---

# FR-004 — validate_archetype(archetype, moduleRoot, data)

## Statement

The WASM module SHALL export `validate_archetype` which runs the
archetype's compiled JSON schema against `data` and either returns
`undefined` (success) or throws `JsError` carrying the violation chain.

## Acceptance Criteria

| ID | Criteria |
|----|----------|
| FR-004-AC-1 | A schema-conforming `data` resolves with no thrown error. |
| FR-004-AC-2 | A schema-violating `data` throws `JsError` whose message contains the violation pointer. |
| FR-004-AC-3 | `validateFromBlob(archetype, moduleBlob, data)` rejects schema-violating data identically to `validate_archetype(archetype, root, data)` (FR-001 amendment, T-008). |

## Relationships

- implements: `quire-wasm/StR-001`
- wraps: `quire-rs/FR-002`
