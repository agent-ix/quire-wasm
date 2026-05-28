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

## Acceptance Criteria

| ID | Criteria |
|----|----------|
| FR-001-AC-1 | `render("FR", root, ctx)` returns a non-empty string containing every required frontmatter field from `ctx`. |
| FR-001-AC-2 | Invalid `archetype` throws `JsError` mentioning the archetype name. |
| FR-001-AC-3 | Output matches `quire-rs::render_by_name(...).markdown` byte-for-byte (StR-001-AC-1). |

## Relationships

- implements: `quire-wasm/StR-001`
- wraps: `quire-rs/FR-001`
