---
id: StR-001
title: "Editor preview is byte-identical to quire-cli render"
type: StR
---

# StR-001 — Editor preview is byte-identical to quire-cli render

## Stakeholder Need

Spec-editor's live preview MUST produce the same rendered markdown as
the `quire render` CLI for the same archetype and the same context,
byte-for-byte.

## Rationale

The current `nunjucks`-based preview produces divergent output for
non-trivial templates (whitespace, conditional blocks, custom filters),
which silently misleads authors about what their published artifact will
look like. Driving the preview through the canonical `quire-rs` pipeline
compiled to WASM removes the parallel JavaScript implementation and
guarantees the preview matches the published artifact.

## Validation Criteria

| ID | Criteria |
|----|----------|
| StR-001-AC-1 | For every ISO archetype shipped by `spec-artifacts-iso`, the WASM `render(archetype, moduleRoot, data)` returns a string equal to `quire render <archetype> --module <root> --data <ctx>` for the same context. |
| StR-001-AC-2 | The WASM `parseDocument(text)` returns a JSON shape equal to `quire-rs::parse_document(text)` serialized via serde_json. |
| StR-001-AC-3 | The WASM `extract` and `validate_archetype` surfaces honor the same error shape as `quire-rs` (mapped through `JsError` with the `QuireError` Debug payload). |

## Dependencies

- **Upstream**: `agent-ix/quire-rs` (the canonical renderer)
- **Downstream**: `agent-ix/spec-editor` (consumer of `@agent-ix/quire-wasm`)
