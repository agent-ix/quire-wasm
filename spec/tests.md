# Test Matrix

100% AC → TC coverage. `grep` audit: every AC ID below MUST appear in
`tests/` source or be marked DEFERRED with a tracking task.

| AC ID | Test | File | Status |
|-------|------|------|--------|
| StR-001-AC-1 | render_fr_matches_native | tests/render_parity.rs | authored, BLOCKED on wasm32 jsonschema (plan T-008 prereq) |
| StR-001-AC-2 | parse_document_roundtrip | tests/render_parity.rs | authored, BLOCKED on wasm32 jsonschema (plan T-008 prereq) |
| StR-001-AC-3 | (covered by FR-003/FR-004 ACs) | — | covered |
| FR-001-AC-1  | render_fr_matches_native | tests/render_parity.rs | green |
| FR-001-AC-2  | render_unknown_archetype_throws | tests/render_parity.rs | DEFERRED (T-003) |
| FR-001-AC-3  | render_fr_matches_native | tests/render_parity.rs | green |
| FR-002-AC-1  | parse_document_roundtrip | tests/render_parity.rs | green |
| FR-002-AC-2  | parse_document_serde_parity | tests/render_parity.rs | DEFERRED (T-003) |
| FR-002-AC-3  | parse_document_no_fs | tests/render_parity.rs | DEFERRED (T-003) |
| FR-003-AC-1  | extract_missing_dsl_throws | tests/render_parity.rs | DEFERRED (T-004) |
| FR-003-AC-2  | extract_parity | tests/render_parity.rs | DEFERRED (T-004) |
| FR-004-AC-1  | validate_ok | tests/render_parity.rs | DEFERRED (T-004) |
| FR-004-AC-2  | validate_violation | tests/render_parity.rs | DEFERRED (T-004) |
| NFR-001-AC-1 | render_p95_budget | benches/render.rs | DEFERRED (T-005) |
| NFR-001-AC-2 | render_p95_band   | benches/render.rs | DEFERRED (T-005) |
