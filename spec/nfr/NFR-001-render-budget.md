---
id: NFR-001
title: "Per-keystroke render fits within editor debounce window"
artifact_type: NFR
---

# NFR-001 — Per-keystroke render fits within editor debounce window

## Statement

A single `render` call against any ISO archetype with a typical
context (≤ 4 KiB JSON) SHALL complete in ≤ 16 ms on a baseline
developer machine (Apple M1 / Ryzen 7 5800X equivalent), measured at
the WASM ABI boundary. This is the per-frame budget that lets
spec-editor keep its 60 fps debounce window without dropping frames
during typing.

## Acceptance Criteria

| ID | Criteria |
|----|----------|
| NFR-001-AC-1 | A criterion-style bench (added in plan/tasks/T-005) reports p95 ≤ 16 ms across 1000 runs of `render("FR", ...)`. |
| NFR-001-AC-2 | Regression band of +10% over a stored baseline; CI lane gates on the band (lands with T-005). |

## Verification

- **Method:** wasm-pack-driven benchmark (cargo bench under
  `wasm-pack test --node --release`) run on CI's ubuntu-24.04 runner;
  baseline stored under `spec/assets/perf-baseline.json`.
- **Evidence:** `make bench` output + the stored baseline JSON.
