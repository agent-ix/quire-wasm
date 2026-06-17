---
id: NFR-001
title: "Per-keystroke render fits within editor debounce window"
type: NFR
---

# NFR-001 — Per-keystroke render fits within editor debounce window

## Statement

A single `render` call against any ISO archetype with a typical
context (≤ 4 KiB JSON) SHALL complete in ≤ 16 ms on a baseline
developer machine (Apple M1 / Ryzen 7 5800X equivalent), measured at
the WASM ABI boundary. This is the per-frame budget that lets
spec-editor keep its 60 fps debounce window without dropping frames
during typing.

## Measurement and Evaluation

The render budget is evaluated by benchmarking a single `render` call at
the WASM ABI boundary against the per-frame budget below.

| Metric | Target | Threshold | Method |
|--------|--------|-----------|--------|
| Per-call render latency (p95) | ≤ 16 ms | 16 ms hard ceiling across 1000 runs of `render("FR", ...)` | Criterion-style bench (plan/tasks/T-005) under `wasm-pack test --node --release` |
| Latency regression vs baseline | within stored baseline | +10% band over `spec/assets/perf-baseline.json` | CI lane gates on the regression band (lands with T-005) |

## Verification

- **Method:** wasm-pack-driven benchmark (cargo bench under
  `wasm-pack test --node --release`) run on CI's ubuntu-24.04 runner;
  baseline stored under `spec/assets/perf-baseline.json`.
- **Evidence:** `make bench` output + the stored baseline JSON.
