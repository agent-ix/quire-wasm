---
id: FR-005
title: "Exported extractSemantic(request) → semantic-v1 record"
type: FR
---

# FR-005 — extractSemantic(request)

## Description

The WASM module SHALL export `extractSemantic`, which runs the quire-rs
semantic extraction (quire-rs FR-069..FR-072) over one markdown document and
returns the `semantic-v1` record as a JS object. The binding converts input and
output only; extraction policy lives in quire-rs.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-005-AC-1 | For every case of quire-rs `tests/fixtures/semantic/cases.json`, `extractSemantic` returns the JSON value recorded for that case in quire-rs `cases.expected.json`; the fixtures are read from the quire-rs dependency, not copied (quire-rs TC-1636). | Test |
| FR-005-AC-2 | If the request's `module.contractVersion` is not the version quire-rs supports, then `extractSemantic` throws a `JsError` whose message starts with `semantic.unsupported-contract-version` and returns no record. | Test |

## Dependencies

- **Upstream**: [StR-001](../stakeholder/StR-001-byte-identical-preview.md) (implements), quire-rs FR-072 (wraps)
- **Downstream**: `tests/semantic_parity.rs`
