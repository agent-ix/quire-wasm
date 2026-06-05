# quire-wasm

WebAssembly bindings for [quire-rs](https://github.com/agent-ix/quire-rs) —
the parsing, extraction, and validation engine that powers the
Filament/Quire spec-artifact ecosystem.

**Purpose:** give `spec-editor` and other browser/Node consumers the
canonical parse/extract/validate pipeline so the editor agrees
byte-for-byte with the `quire` CLI and the Python reference parser.

> **No render surface.** The render/templating feature was removed from
> `quire-rs` (commit `e0811a8`). There is no `render` or `renderFromBlob`
> export; the module blob is `{ manifest, schemas }` (no `templates`).

## Public surface

```ts
import init, {
  parseDocument,
  extract,
  validate_archetype as validate,
  extractFromBlob,
  validateFromBlob,
} from "@agent-ix/quire-wasm";

await init(); // load the .wasm

const doc = parseDocument(md);

// Filesystem-rooted (Node target):
const records = extract("FR", "/path/to/module", md);
validate("FR", "/path/to/module", { id: "FR-001", /* ... */ }); // throws on violation

// In-memory module blob (browser / --target web):
const moduleBlob = {
  manifest: "<raw manifest.yaml text>",
  schemas: { "schemas/fr-frontmatter.schema.json": "<schema json>" },
};
const recs = extractFromBlob("FR", moduleBlob, md);
validateFromBlob("FR", moduleBlob, { id: "FR-001", /* ... */ });
```

### Two shapes

- **Module-blob** (`extractFromBlob`, `validateFromBlob`): pass an
  in-memory `{ manifest, schemas }` object — the filesystem-free path
  used by `--target web`. Builds an inline `Registry` via
  `quire_rs::Registry::from_inline_parts(manifest, schemas)`.
- **Filesystem-rooted** (`extract`, `validate_archetype`): pass a
  `moduleRoot` path string. Works under `wasm-pack --target nodejs`
  (real fs) and any embedding that polyfills `fs` via WASI.

`parseDocument` is filesystem-free and works everywhere.

## Install

```bash
npm install @agent-ix/quire-wasm   # from GitHub Packages
```

## Build

```bash
make build       # wasm-pack build --target web --release  (browser)
make build-node  # wasm-pack build --target nodejs --release (Node)
make test        # wasm-pack test --node (extract/validate parity vs quire-rs)
make ci          # full local CI gate (fmt + lint + test + deny + audit)
```

`wasm-pack` is required: `cargo install wasm-pack`.

## License

MIT
