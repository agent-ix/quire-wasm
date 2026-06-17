---
id: FR-001
title: "Exported render(archetype, moduleRoot, data) → markdown"
type: FR
---

# FR-001 — render(archetype, moduleRoot, data) → markdown

> **RETIRED (CR-001).** The render/templating feature was removed from
> `quire-rs` (commit `e0811a8`): `render_by_name` is gone and
> `Registry::from_inline_parts(manifest, schemas)` no longer takes a
> templates map. The `render` and `renderFromBlob` WASM exports, the
> `templates` field of `moduleBlob`, and the render-parity tests
> (`tests/render_parity.rs`) are removed. The surviving surface is
> parseDocument ([FR-002](./FR-002-parse-document.md)), extract / extractFromBlob ([FR-003](./FR-003-extract.md)), and
> validate / validateFromBlob ([FR-004](./FR-004-validate.md)). The text below is retained for
> historical context only and is no longer normative.

## Description

The WASM module SHALL export a function `render` that takes an
archetype name, a module-root path string, and a JS data object, and
returns the rendered markdown as a JS string. On any quire-rs error,
the function SHALL throw a `JsError` whose message carries the
canonical diagnostic shape.

## Module-blob shape (T-008 amendment, v0.1.0)

The WASM module SHALL additionally export `renderFromBlob(archetype, moduleBlob, data)` for the browser / `--target web` lane. `moduleBlob` is a JS object of shape:

```js
{
  manifest: "<raw manifest.yaml text>",
  schemas:  { "<relative schema_ref>": "<schema json text>", ... },
  templates:{ "<relative template_ref>": "<jinja source>", ... }
}
```

`renderFromBlob` builds an in-memory `Registry` via `quire_rs::Registry::from_inline_parts` (FR-013 wasm amendment) and renders without touching the host filesystem. The `render` (path-rooted) export remains for `--target nodejs` consumers that already hold the module on disk.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-001-AC-1 | `render("FR", root, ctx)` returns a non-empty string containing every required frontmatter field from `ctx`. | Test |
| FR-001-AC-2 | Invalid `archetype` throws `JsError` mentioning the archetype name. | Test |
| FR-001-AC-3 | Output matches `quire-rs::render_by_name(...).markdown` byte-for-byte ([StR-001-AC-1](../stakeholder/StR-001-byte-identical-preview.md)). | Test |
| FR-001-AC-4 | `renderFromBlob("FR", moduleBlob, ctx)` returns the same string as `render("FR", root, ctx)` when `moduleBlob` is constructed from the same module files; verified under `wasm-pack test --node`. | Test |

## Dependencies

- **Upstream**: [StR-001](../stakeholder/StR-001-byte-identical-preview.md) (implements), `quire-rs/FR-001` (wraps the retired native render)
- **Downstream**: none — surface RETIRED (CR-001), no longer normative
