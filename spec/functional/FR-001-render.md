---
id: FR-001
title: "Exported render(archetype, moduleRoot, data) → markdown"
artifact_type: FR
---

# FR-001 — render(archetype, moduleRoot, data) → markdown

## Statement

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

| ID | Criteria |
|----|----------|
| FR-001-AC-1 | `render("FR", root, ctx)` returns a non-empty string containing every required frontmatter field from `ctx`. |
| FR-001-AC-2 | Invalid `archetype` throws `JsError` mentioning the archetype name. |
| FR-001-AC-3 | Output matches `quire-rs::render_by_name(...).markdown` byte-for-byte (StR-001-AC-1). |
| FR-001-AC-4 | `renderFromBlob("FR", moduleBlob, ctx)` returns the same string as `render("FR", root, ctx)` when `moduleBlob` is constructed from the same module files; verified under `wasm-pack test --node`. |

## Relationships

- implements: `quire-wasm/StR-001`
- wraps: `quire-rs/FR-001`
