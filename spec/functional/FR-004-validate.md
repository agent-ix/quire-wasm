---
id: FR-004
title: "Exported validate_archetype(archetype, moduleRoot, data) → ok | throw"
type: FR
---

# FR-004 — validate_archetype(archetype, moduleRoot, data)

## Description

The WASM module SHALL export `validate_archetype` which runs the
archetype's compiled JSON schema against `data` and either returns
`undefined` (success) or throws `JsError` carrying the violation chain.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-004-AC-1 | A schema-conforming `data` resolves with no thrown error. | Test |
| FR-004-AC-2 | A schema-violating `data` throws `JsError` whose message contains the violation pointer. | Test |
| FR-004-AC-3 | `validateFromBlob(archetype, moduleBlob, data)` rejects schema-violating data identically to `validate_archetype(archetype, root, data)` (FR-001 amendment, T-008). | Test |

## Dependencies

- **Upstream**: `quire-wasm/StR-001` (implements), `quire-rs/FR-002` (wraps)
- **Downstream**: `tests/extract_validate.rs` validate coverage
