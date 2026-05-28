//! WebAssembly bindings for quire-rs.
//!
//! Exposes the same render/parseDocument/extract/validate surface as the
//! Python `quire` module so spec-editor's live preview can call into the
//! same engine that powers `quire-cli` and the Python reference
//! renderer (no `nunjucks` runtime divergence).
//!
//! ## Two shapes
//!
//! 1. **Module-blob** (`renderFromBlob`, `extractFromBlob`,
//!    `validateFromBlob`): accept a single `moduleBlob` JS object of
//!    shape `{ manifest: string, schemas: { "<ref>": string },
//!    templates: { "<ref>": string } }` — the filesystem-free path used
//!    by `--target web`. The host bundles the module's files into the
//!    blob once, then drives render/extract/validate inline.
//!
//! 2. **Filesystem-rooted** (`render`, `extract`, `validateArchetype`):
//!    accept a `moduleRoot` path string. These keep working under
//!    `--target nodejs` for tools that already live on disk.
//!
//! `parseDocument` is pure string-in / JSON-out and identical in both
//! shapes.

use std::collections::BTreeMap;

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

#[cfg(not(target_arch = "wasm32"))]
fn registry_from_module(module_root: &str) -> Result<Registry, JsError> {
    let path = std::path::Path::new(module_root);
    Registry::load_module(path).map_err(|e| JsError::new(&format!("registry load failed: {e:?}")))
}

#[cfg(target_arch = "wasm32")]
fn registry_from_module(module_root: &str) -> Result<Registry, JsError> {
    // Under --target web/bundler the host filesystem is typically
    // unavailable. Node/WASI hosts can still call this — but the
    // canonical web path is via `*_from_blob`.
    let path = std::path::Path::new(module_root);
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

/// Deserialize a JS `moduleBlob` object into the three inputs
/// `Registry::from_inline_parts` expects: the raw manifest bytes plus
/// `BTreeMap`s of relative-ref → source for schemas and templates.
#[allow(clippy::type_complexity)]
fn parts_from_blob(
    blob: JsValue,
) -> Result<(Vec<u8>, BTreeMap<String, String>, BTreeMap<String, String>), JsError> {
    #[derive(serde::Deserialize)]
    struct Blob {
        manifest: String,
        #[serde(default)]
        schemas: BTreeMap<String, String>,
        #[serde(default)]
        templates: BTreeMap<String, String>,
    }
    let parsed: Blob = serde_wasm_bindgen::from_value(blob)
        .map_err(|e| JsError::new(&format!("invalid moduleBlob: {e}")))?;
    Ok((
        parsed.manifest.into_bytes(),
        parsed.schemas,
        parsed.templates,
    ))
}

fn registry_from_blob(blob: JsValue) -> Result<Registry, JsError> {
    let (manifest, schemas, templates) = parts_from_blob(blob)?;
    Registry::from_inline_parts(&manifest, &schemas, &templates)
        .map_err(|e| JsError::new(&format!("registry inline load failed: {e:?}")))
}

// ============================================================================
// Filesystem-rooted surface (Node-target convenience).
// ============================================================================

/// Render `archetype` from the module rooted at `moduleRoot` against
/// `data`. Returns the rendered markdown string.
///
/// Mirrors `quire render <archetype> --module <root> --data <ctx>`.
#[wasm_bindgen]
pub fn render(archetype: &str, module_root: &str, data: JsValue) -> Result<String, JsError> {
    let registry = registry_from_module(module_root)?;
    render_with_registry(&registry, archetype, data)
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
/// return the extraction result (records + diagnostics) as a JS object.
#[wasm_bindgen]
pub fn extract(archetype: &str, module_root: &str, doc: &str) -> Result<JsValue, JsError> {
    let registry = registry_from_module(module_root)?;
    extract_with_registry(&registry, archetype, doc)
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
    validate_with_registry(&registry, archetype, data)
}

// ============================================================================
// Module-blob surface (browser / --target web).
// ============================================================================

/// Render from an in-memory `moduleBlob` (no filesystem access).
///
/// `moduleBlob` shape:
/// ```json
/// {
///   "manifest": "name: spec_artifacts_iso\nartifact_types:\n- ...",
///   "schemas":  { "schemas/FR-frontmatter.schema.json": "{...}" },
///   "templates":{ "templates/FR.md.j2": "..." }
/// }
/// ```
#[wasm_bindgen(js_name = renderFromBlob)]
pub fn render_from_blob(
    archetype: &str,
    module_blob: JsValue,
    data: JsValue,
) -> Result<String, JsError> {
    let registry = registry_from_blob(module_blob)?;
    render_with_registry(&registry, archetype, data)
}

/// Extract from an in-memory module blob. See [`render_from_blob`] for
/// the blob shape.
#[wasm_bindgen(js_name = extractFromBlob)]
pub fn extract_from_blob(
    archetype: &str,
    module_blob: JsValue,
    doc: &str,
) -> Result<JsValue, JsError> {
    let registry = registry_from_blob(module_blob)?;
    extract_with_registry(&registry, archetype, doc)
}

/// Validate `data` against an archetype defined inside `moduleBlob`.
/// See [`render_from_blob`] for the blob shape.
#[wasm_bindgen(js_name = validateFromBlob)]
pub fn validate_from_blob(
    archetype: &str,
    module_blob: JsValue,
    data: JsValue,
) -> Result<(), JsError> {
    let registry = registry_from_blob(module_blob)?;
    validate_with_registry(&registry, archetype, data)
}

// ============================================================================
// Shared adapters
// ============================================================================

fn render_with_registry(
    registry: &Registry,
    archetype: &str,
    data: JsValue,
) -> Result<String, JsError> {
    let value = data_from_js(data)?;
    let out = render_by_name(registry, archetype, &value)
        .map_err(|e| JsError::new(&format!("render failed: {e:?}")))?;
    Ok(out.markdown)
}

fn extract_with_registry(
    registry: &Registry,
    archetype: &str,
    doc: &str,
) -> Result<JsValue, JsError> {
    let archetype_ref = registry
        .archetype(archetype)
        .ok_or_else(|| JsError::new(&format!("unknown archetype: {archetype}")))?;
    let dsl: &ExtractionDsl = archetype_ref
        .body_extraction()
        .ok_or_else(|| JsError::new(&format!("archetype '{archetype}' has no body extraction")))?;
    let parsed = rs_parse_document(doc);
    let result =
        rs_extract(&parsed, dsl).map_err(|e| JsError::new(&format!("extract failed: {e:?}")))?;
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

fn validate_with_registry(
    registry: &Registry,
    archetype: &str,
    data: JsValue,
) -> Result<(), JsError> {
    let archetype_ref = registry
        .archetype(archetype)
        .ok_or_else(|| JsError::new(&format!("unknown archetype: {archetype}")))?;
    let value = data_from_js(data)?;
    validate(archetype_ref, &value)
        .map_err(|e| JsError::new(&format!("validation failed: {e:?}")))?;
    Ok(())
}
