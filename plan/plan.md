# quire-wasm Project Plan

## Phase 1: Scaffold (THIS TURN — DONE)

- T-001 Repo skeleton (Cargo + package.json + Makefile + CI + hygiene)
- T-002 Public surface + Node parity test for `render` and `parseDocument`

Gate: `make ci` green; repo published at `agent-ix/quire-wasm`.

## Phase 2: Full surface + AC coverage

- T-003 Negative-path render/parse ACs (FR-001-AC-2, FR-002-AC-2/3)
- T-004 Extract + validate ACs (FR-003 + FR-004)

Gate: `spec/tests.md` shows zero DEFERRED rows under FR-001..004.

## Phase 3: Performance baseline

- T-005 Criterion bench + stored baseline + CI band check (NFR-001)

Gate: p95 ≤ 16 ms and regression band lane wired.

## Phase 4: npm publish

- T-006 GitHub Packages publish workflow (tag-driven, mirror agent-ix/`@agent-ix/*` pattern)
- T-007 Drop `private: true`; bump to v0.2.0

Gate: `npm view @agent-ix/quire-wasm` resolves from `npm.pkg.github.com`.

## Phase 5: Browser-only path

- T-008 In-memory module-blob loader: `Registry::from_blob(json)`
  upstreamed in quire-rs; quire-wasm exposes
  `renderFromBlob(archetype, moduleBlob, data)`
- T-009 spec-editor swap: replace `nunjucks` with `@agent-ix/quire-wasm`
  (in spec-editor repo; out of scope here)

Gate: `--target web` build runs `render` without a Node fs polyfill.
