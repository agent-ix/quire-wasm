//! WebAssembly bindings for quire-rs.
//!
//! Exposes four public functions mirroring the Python `quire` module
//! surface — `render`, `parseDocument`, `extract`, `validate` — so that
//! spec-editor's live preview can call into the same rendering pipeline
//! that quire-cli and the Python reference renderer use, without the
//! `nunjucks` runtime-divergence risk (Q5 parent plan).
//!
//! ## Filesystem shape
//!
//! Per the Q5 scaffold ADR, the `moduleRoot` parameter is a string path
//! resolved against the host's filesystem (real fs under
//! `--target nodejs`, polyfilled under `--target bundler` with a WASI
//! shim — out of scope for v0). The browser-side spec-editor will run
//! the wasm in a Node worker until the in-memory module-blob loader
//! lands; see `plan/tasks/README.md` § "in-memory module loader".
//!
//! Errors map: `quire_rs::QuireError` → `JsError` with the canonical
//! diagnostic-shaped string (NFR parity with quire-cli stderr).

use std::path::Path;

use serde_json::Value;
use wasm_bindgen::prelude::*;

use quire_rs::{
    extract as rs_extract, parse_document as rs_parse_document, render_by_name, validate,
    ExtractionDsl, Registry,
};

/// Install a panic hook that surfaces Rust panics as console.error in JS.
/// No-op when the `panic-hook` feature is disabled.
#[wasm_bindgen(start)]
pub fn _start() {
    #[cfg(feature = "panic-hook")]
    console_error_panic_hook::set_once();
}

fn registry_from_module(module_root: &str) -> Result<Registry, JsError> {
    let path = Path::new(module_root);
    Registry::load_module(path).map_err(|e| JsError::new(&format!("registry load failed: {e:?}")))
}

fn data_from_js(data: JsValue) -> Result<Value, JsError> {
    serde_wasm_bindgen::from_value(data)
        .map_err(|e| JsError::new(&format!("invalid data payload: {e}")))
}

fn value_to_js(value: &Value) -> Result<JsValue, JsError> {
    serde_wasm_bindgen::to_value(value)
        .map_err(|e| JsError::new(&format!("serialization failed: {e}")))
}

/// Render `archetype` from the module rooted at `moduleRoot` against
/// `data` (a JS object matching the archetype schema). Returns the
/// rendered markdown string.
///
/// Mirrors `quire render <archetype> --module <root> --data <ctx>`.
#[wasm_bindgen]
pub fn render(archetype: &str, module_root: &str, data: JsValue) -> Result<String, JsError> {
    let registry = registry_from_module(module_root)?;
    let value = data_from_js(data)?;
    let out = render_by_name(&registry, archetype, &value)
        .map_err(|e| JsError::new(&format!("render failed: {e:?}")))?;
    Ok(out.markdown)
}

/// Parse a markdown document into the `QuireDocument` JSON shape.
/// No filesystem access; pure string-in/JSON-out (live-preview hot path).
#[wasm_bindgen(js_name = parseDocument)]
pub fn parse_document(text: &str) -> Result<JsValue, JsError> {
    let doc = rs_parse_document(text);
    let v = serde_json::to_value(&doc)
        .map_err(|e| JsError::new(&format!("parse serialization failed: {e}")))?;
    value_to_js(&v)
}

/// Run the archetype's body-extraction DSL over `doc` markdown and
/// return the extraction result (records + edges) as a JS object.
#[wasm_bindgen]
pub fn extract(archetype: &str, module_root: &str, doc: &str) -> Result<JsValue, JsError> {
    let registry = registry_from_module(module_root)?;
    let archetype_ref = registry
        .archetype(archetype)
        .ok_or_else(|| JsError::new(&format!("unknown archetype: {archetype}")))?;
    let dsl: &ExtractionDsl = archetype_ref
        .body_extraction()
        .ok_or_else(|| JsError::new(&format!("archetype '{archetype}' has no body extraction")))?;
    let parsed = rs_parse_document(doc);
    let result =
        rs_extract(&parsed, dsl).map_err(|e| JsError::new(&format!("extract failed: {e:?}")))?;
    // ExtractionResult doesn't implement Serialize, so build the JSON
    // shape by hand from its public fields. Diagnostics serialize via
    // their Debug representation pending a quire-rs Serialize impl
    // (tracked in plan/T-008's prereq list).
    let records: Vec<Value> = result
        .records
        .iter()
        .map(|m| Value::Object(m.clone()))
        .collect();
    let diagnostics: Vec<Value> = result
        .diagnostics
        .iter()
        .map(|d| Value::String(format!("{d:?}")))
        .collect();
    let v = serde_json::json!({
        "records": records,
        "diagnostics": diagnostics,
    });
    value_to_js(&v)
}

/// Validate `data` against the archetype schema. Returns undefined on
/// success; throws JsError carrying the violation chain on failure.
#[wasm_bindgen]
pub fn validate_archetype(
    archetype: &str,
    module_root: &str,
    data: JsValue,
) -> Result<(), JsError> {
    let registry = registry_from_module(module_root)?;
    let archetype_ref = registry
        .archetype(archetype)
        .ok_or_else(|| JsError::new(&format!("unknown archetype: {archetype}")))?;
    let value = data_from_js(data)?;
    validate(archetype_ref, &value)
        .map_err(|e| JsError::new(&format!("validation failed: {e:?}")))?;
    Ok(())
}
