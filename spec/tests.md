# Test Matrix

100% AC → TC coverage. `grep` audit: every AC ID below MUST appear in
`tests/` source or be marked DEFERRED with a tracking task.

> **CR-001:** the render surface (FR-001 ACs, render-parity test,
> NFR-001 render budget) is RETIRED — quire-rs dropped render (`e0811a8`).
> Those rows are marked RETIRED, not deferred. The blob is now
> `{ manifest, schemas }`; tests live in `tests/extract_validate.rs`.

| AC ID | Test | File | Status |
|-------|------|------|--------|
| StR-001-AC-1 | (render) | — | RETIRED (CR-001 — quire-rs render removed) |
| StR-001-AC-2 | parse_document_roundtrip | tests/extract_validate.rs | green |
| StR-001-AC-3 | (covered by FR-003/FR-004 ACs) | — | covered |
| FR-001-AC-1  | (render) | — | RETIRED (CR-001) |
| FR-001-AC-2  | (render) | — | RETIRED (CR-001) |
| FR-001-AC-3  | (render) | — | RETIRED (CR-001) |
| FR-001-AC-4  | (renderFromBlob) | — | RETIRED (CR-001) |
| FR-002-AC-1  | parse_document_roundtrip | tests/extract_validate.rs | green |
| FR-002-AC-2  | parse_document_serde_parity | tests/extract_validate.rs | DEFERRED (T-003) |
| FR-002-AC-3  | parse_document_no_fs | tests/extract_validate.rs | DEFERRED (T-003) |
| FR-003-AC-1  | extract_missing_dsl_throws | tests/extract_validate.rs | DEFERRED (T-004) |
| FR-003-AC-2  | extract_from_blob_yields_records | tests/extract_validate.rs | green |
| FR-004-AC-1  | validate_from_blob_accepts_valid_fr | tests/extract_validate.rs | green |
| FR-004-AC-2  | validate_from_blob_rejects_missing_required | tests/extract_validate.rs | green |
| NFR-001-AC-1 | (render budget) | — | RETIRED (CR-001) |
| NFR-001-AC-2 | (render budget) | — | RETIRED (CR-001) |
