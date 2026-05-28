# quire-wasm

WebAssembly bindings for [quire-rs](https://github.com/agent-ix/quire-rs) —
the rendering, parsing, extraction, and validation engine that powers
the Filament/Quire spec-artifact ecosystem.

**Purpose:** replace the `nunjucks`-based live preview in `spec-editor`
with the canonical renderer, so the editor preview is byte-identical to
the `quire render` CLI and the Python reference renderer.

## Public surface

```ts
import init, {
  render,
  parseDocument,
  extract,
  validate_archetype as validate,
} from "@agent-ix/quire-wasm";

await init(); // load the .wasm

const md = render("FR", "/path/to/module", { id: "FR-001", /* ... */ });
const doc = parseDocument(md);
const records = extract("FR", "/path/to/module", md);
validate("FR", "/path/to/module", { id: "FR-001", /* ... */ }); // throws on violation
```

### Filesystem shape (v0.1 ADR)

The `moduleRoot` parameter is a string path resolved against the host
filesystem. This works under `wasm-pack --target nodejs` (real fs) and
in any embedding that polyfills `fs` via WASI.

Browser-only builds (`--target web` without a WASI shim) currently
cannot call `render`, `extract`, or `validate` — `parseDocument` is
filesystem-free and works everywhere. spec-editor will run the wasm in
a Node worker for v0; an in-memory module-blob loader is tracked in
`plan/tasks/README.md`.

## Install

```bash
npm install @agent-ix/quire-wasm   # from GitHub Packages
```

## Build

```bash
make build       # wasm-pack build --target web --release  (browser)
make build-node  # wasm-pack build --target nodejs --release (Node)
make test        # wasm-pack test --node (parity tests against quire-rs)
make ci          # full local CI gate (fmt + lint + test + deny + audit)
```

`wasm-pack` is required: `cargo install wasm-pack`.

## License

MIT
