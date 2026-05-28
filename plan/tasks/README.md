# Tasks

| ID | Title | Phase | Status |
|----|-------|-------|--------|
| T-001 | Repo skeleton (Cargo + package.json + Makefile + CI + hygiene) | 1 | DONE |
| T-002 | Public surface + Node parity test for `render` and `parseDocument` | 1 | DONE |
| T-003 | Negative-path render/parse ACs | 2 | TODO |
| T-004 | Extract + validate ACs | 2 | TODO |
| T-005 | Criterion bench + baseline + CI band (NFR-001) | 3 | TODO |
| T-006 | GitHub Packages publish workflow (tag-driven) | 4 | TODO |
| T-007 | Drop `private: true`, bump to v0.2.0 | 4 | TODO |
| T-008 | In-memory module-blob loader (cross-repo: quire-rs + quire-wasm) | 5 | DONE |
| T-009 | spec-editor swap: nunjucks → @agent-ix/quire-wasm (in spec-editor repo) | 5 | FOLLOW-UP |

## RESOLVED — jsonschema 0.18 on wasm32 (T-008)

Resolved in `quire-rs v0.3.1` via option (1) above: a `wasm` Cargo
feature that drops the `jsonschema/resolve-file` activation. quire-wasm
now depends on `quire-rs = { features = ["wasm"], default-features = false }`
and `wasm-pack build --target web --release` succeeds. `wasm-pack test
--node` runs all four `render_parity.rs` tests, including the
previously-blocked StR-001-AC-1 parity test (the test bundles the ISO
module via `include_str!` and renders through `renderFromBlob`).

## In-memory module loader (T-008) — DONE

Upstream API landed as `quire_rs::Registry::from_inline_parts(manifest_yaml, schemas, templates)` in v0.3.1 (FR-013 wasm amendment). quire-wasm exposes:

- `renderFromBlob(archetype, moduleBlob, data) -> markdown`
- `extractFromBlob(archetype, moduleBlob, doc) -> { records, diagnostics }`
- `validateFromBlob(archetype, moduleBlob, data) -> undefined | throw`

`moduleBlob` JS shape:

```js
{
  manifest:  "<raw manifest.yaml>",
  schemas:   { "schemas/fr-frontmatter.schema.json": "<json>", ... },
  templates: { "templates/fr.md.j2": "<jinja>", ... }
}
```

The original `render` / `extract` / `validate_archetype` path-rooted
exports remain for `--target nodejs` consumers that already hold the
module on disk.

## spec-editor swap (T-009) — follow-up

Filed as a follow-up rather than landing here: the swap touches the
spec-editor app (separate repo) and benefits from coordination with the
editor team. The exact import-line change is:

```diff
- import nunjucks from "nunjucks";
+ import * as quire from "@agent-ix/quire-wasm";
```

…with the live-preview render switching from `nunjucks.renderString(template, ctx)` to `quire.renderFromBlob(archetype, moduleBlob, ctx)`, where `moduleBlob` is built once at editor startup from the active module's `manifest.yaml`, `schemas/*.json`, and `templates/*.j2`.
