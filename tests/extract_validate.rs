//! WASM parity tests: parseDocument / extractFromBlob / validateFromBlob
//! against the canonical quire-rs engine.
//!
//! The render/templating feature was removed from quire-rs (commit
//! `e0811a8`), so there is no `render`/`renderFromBlob` surface to test.
//! The module blob is now `{ manifest, schemas }` (no templates map),
//! matching the render-free `Registry::from_inline_parts(manifest,
//! schemas)` signature.
//!
//! Runs under `wasm-pack test --node` (Node target — real fs, no
//! browser polyfill needed).

#![cfg(target_arch = "wasm32")]

use serde_json::json;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

// Build-time-baked ISO module fixture. `include_str!` resolves on the
// host (where the filesystem exists), so the resulting wasm-pack test
// runs against the sibling spec-artifacts-iso module without needing
// any FS syscall at runtime — proving the FR-013 wasm amendment.
const ISO_MANIFEST: &str =
    include_str!("../../spec-artifacts-iso/spec_artifacts_iso/manifest.yaml");
const ISO_FR_SCHEMA: &str =
    include_str!("../../spec-artifacts-iso/spec_artifacts_iso/schemas/fr-frontmatter.schema.json");
const ISO_AC_SCHEMA: &str =
    include_str!("../../spec-artifacts-iso/spec_artifacts_iso/schemas/ac-frontmatter.schema.json");
const ISO_CON_SCHEMA: &str =
    include_str!("../../spec-artifacts-iso/spec_artifacts_iso/schemas/con-frontmatter.schema.json");
const ISO_IT_SCHEMA: &str =
    include_str!("../../spec-artifacts-iso/spec_artifacts_iso/schemas/it-frontmatter.schema.json");
const ISO_NFR_SCHEMA: &str =
    include_str!("../../spec-artifacts-iso/spec_artifacts_iso/schemas/nfr-frontmatter.schema.json");
const ISO_STR_SCHEMA: &str =
    include_str!("../../spec-artifacts-iso/spec_artifacts_iso/schemas/str-frontmatter.schema.json");
const ISO_TC_SCHEMA: &str =
    include_str!("../../spec-artifacts-iso/spec_artifacts_iso/schemas/tc-frontmatter.schema.json");
const ISO_US_SCHEMA: &str =
    include_str!("../../spec-artifacts-iso/spec_artifacts_iso/schemas/us-frontmatter.schema.json");

/// Build the render-free ISO module blob: `{ manifest, schemas }`.
fn iso_blob() -> JsValue {
    use serde::Serialize as _;
    let blob = json!({
        "manifest": ISO_MANIFEST,
        "schemas": {
            "schemas/fr-frontmatter.schema.json": ISO_FR_SCHEMA,
            "schemas/ac-frontmatter.schema.json": ISO_AC_SCHEMA,
            "schemas/con-frontmatter.schema.json": ISO_CON_SCHEMA,
            "schemas/it-frontmatter.schema.json": ISO_IT_SCHEMA,
            "schemas/nfr-frontmatter.schema.json": ISO_NFR_SCHEMA,
            "schemas/str-frontmatter.schema.json": ISO_STR_SCHEMA,
            "schemas/tc-frontmatter.schema.json": ISO_TC_SCHEMA,
            "schemas/us-frontmatter.schema.json": ISO_US_SCHEMA,
        }
    });
    blob.serialize(&serializer()).expect("blob to JsValue")
}

/// Use `json_compatible()` so nested objects become real JS objects
/// (the default `Serializer` emits ES6 `Map`s, which our `from_value`
/// struct deserializer rejects).
fn serializer() -> serde_wasm_bindgen::Serializer {
    serde_wasm_bindgen::Serializer::json_compatible()
}

#[wasm_bindgen_test]
fn validate_from_blob_accepts_valid_fr() {
    // FR-004-AC-1: a well-formed FR context passes the schema bound in
    // the inline ISO module blob.
    use serde::Serialize as _;
    let data = json!({
        "id": "FR-099",
        "title": "Performance benchmark sample",
        "type": "FR",
        "object": "core/scheduler",
        "description": "Auto-generated benchmark target.",
        "relationships": [
            {
                "target": "ix://agent-ix/filament-core-service/FR-035",
                "type": "implements",
                "cardinality": "1..1"
            }
        ]
    });
    let js_data: JsValue = data.serialize(&serializer()).unwrap();
    quire_wasm::validate_from_blob("FR", iso_blob(), js_data).expect("valid FR should pass");
}

#[wasm_bindgen_test]
fn validate_from_blob_rejects_missing_required() {
    // FR-004-AC-2: a context missing a required field throws.
    use serde::Serialize as _;
    let manifest = "name: vm\nartifact_types:\n- name: req\n  frontmatter_schema_ref: s/r.json\n";
    let schema = r#"{"type":"object","required":["id"],"properties":{"id":{"type":"string"}}}"#;
    let blob = json!({
        "manifest": manifest,
        "schemas": { "s/r.json": schema },
    });
    let js_blob: JsValue = blob.serialize(&serializer()).unwrap();
    let data: JsValue = json!({}).serialize(&serializer()).unwrap();
    let err = quire_wasm::validate_from_blob("req", js_blob, data);
    assert!(err.is_err(), "expected validation failure for missing 'id'");
}

#[wasm_bindgen_test]
fn extract_from_blob_yields_records() {
    // FR-003-AC-2: run an archetype body-extraction DSL over a document
    // via the inline blob path; the result carries records/diagnostics.
    use serde::Serialize as _;
    let manifest = concat!(
        "name: em\n",
        "artifact_types:\n",
        "- name: note\n",
        "  frontmatter_schema_ref: s/n.json\n",
        "  body_extraction:\n",
        "    yield_pattern:\n",
        "      match:\n",
        "        body:\n",
        "          from: section_body\n",
        "          after_heading: Body\n",
        "          required: true\n",
    );
    let schema = r#"{"type":"object","required":["id"],"properties":{"id":{"type":"string"}}}"#;
    let blob = json!({
        "manifest": manifest,
        "schemas": { "s/n.json": schema },
    });
    let js_blob: JsValue = blob.serialize(&serializer()).unwrap();
    let doc = "---\nid: N-1\n---\n\n## Body\n\nhello.\n";
    let out = quire_wasm::extract_from_blob("note", js_blob, doc).expect("extract ok");
    let value: serde_json::Value = serde_wasm_bindgen::from_value(out).unwrap();
    assert!(value.get("records").is_some(), "missing records: {value}");
    assert!(
        value.get("diagnostics").is_some(),
        "missing diagnostics: {value}"
    );
}

#[wasm_bindgen_test]
fn parse_document_roundtrip() {
    let md = "---\nid: FR-099\n---\n\n# Heading\n\nbody.\n";
    let v = quire_wasm::parse_document(md).expect("parse ok");
    let value: serde_json::Value = serde_wasm_bindgen::from_value(v).unwrap();
    assert!(
        value.get("sections").is_some(),
        "missing sections in parse output"
    );
}
