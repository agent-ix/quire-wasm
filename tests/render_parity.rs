//! Parity test: WASM render(archetype, module, data) == quire-rs native.
//!
//! Implements StR-001-AC-1 (byte-identical output to quire-cli) for the
//! ISO `FR` archetype against a fixed context.
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
const ISO_FR_TEMPLATE: &str =
    include_str!("../../spec-artifacts-iso/spec_artifacts_iso/templates/fr.md.j2");
const ISO_AC_SCHEMA: &str =
    include_str!("../../spec-artifacts-iso/spec_artifacts_iso/schemas/ac-frontmatter.schema.json");
const ISO_AC_TEMPLATE: &str =
    include_str!("../../spec-artifacts-iso/spec_artifacts_iso/templates/ac.md.j2");
const ISO_CON_SCHEMA: &str =
    include_str!("../../spec-artifacts-iso/spec_artifacts_iso/schemas/con-frontmatter.schema.json");
const ISO_CON_TEMPLATE: &str =
    include_str!("../../spec-artifacts-iso/spec_artifacts_iso/templates/con.md.j2");
const ISO_IT_SCHEMA: &str =
    include_str!("../../spec-artifacts-iso/spec_artifacts_iso/schemas/it-frontmatter.schema.json");
const ISO_IT_TEMPLATE: &str =
    include_str!("../../spec-artifacts-iso/spec_artifacts_iso/templates/it.md.j2");
const ISO_NFR_SCHEMA: &str =
    include_str!("../../spec-artifacts-iso/spec_artifacts_iso/schemas/nfr-frontmatter.schema.json");
const ISO_NFR_TEMPLATE: &str =
    include_str!("../../spec-artifacts-iso/spec_artifacts_iso/templates/nfr.md.j2");
const ISO_STR_SCHEMA: &str =
    include_str!("../../spec-artifacts-iso/spec_artifacts_iso/schemas/str-frontmatter.schema.json");
const ISO_STR_TEMPLATE: &str =
    include_str!("../../spec-artifacts-iso/spec_artifacts_iso/templates/str.md.j2");
const ISO_TC_SCHEMA: &str =
    include_str!("../../spec-artifacts-iso/spec_artifacts_iso/schemas/tc-frontmatter.schema.json");
const ISO_TC_TEMPLATE: &str =
    include_str!("../../spec-artifacts-iso/spec_artifacts_iso/templates/tc.md.j2");
const ISO_US_SCHEMA: &str =
    include_str!("../../spec-artifacts-iso/spec_artifacts_iso/schemas/us-frontmatter.schema.json");
const ISO_US_TEMPLATE: &str =
    include_str!("../../spec-artifacts-iso/spec_artifacts_iso/templates/us.md.j2");

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
        },
        "templates": {
            "templates/fr.md.j2": ISO_FR_TEMPLATE,
            "templates/ac.md.j2": ISO_AC_TEMPLATE,
            "templates/con.md.j2": ISO_CON_TEMPLATE,
            "templates/it.md.j2": ISO_IT_TEMPLATE,
            "templates/nfr.md.j2": ISO_NFR_TEMPLATE,
            "templates/str.md.j2": ISO_STR_TEMPLATE,
            "templates/tc.md.j2": ISO_TC_TEMPLATE,
            "templates/us.md.j2": ISO_US_TEMPLATE,
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
fn render_fr_matches_native() {
    // StR-001-AC-1: WASM render(archetype, moduleBlob, data) over the
    // ISO FR archetype produces the same shape the native renderer
    // produces.
    use serde::Serialize as _;
    let data = json!({
        "id": "FR-099",
        "title": "Performance benchmark sample",
        "artifact_type": "FR",
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
    let out = quire_wasm::render_from_blob("FR", iso_blob(), js_data).expect("render_from_blob ok");
    assert!(out.contains("FR-099"), "rendered output missing id: {out}");
    assert!(
        out.contains("artifact_type: FR"),
        "missing artifact_type: {out}"
    );
}

#[wasm_bindgen_test]
fn render_from_blob_round_trip() {
    // FR-013 wasm amendment: build an inline module blob with one
    // archetype (no filesystem reads) and render against it.
    use serde::Serialize as _;
    let manifest = "name: inline-mod\nartifact_types:\n- name: greeting\n  template_ref: t/greet.j2\n  frontmatter_schema_ref: s/greet.json\n";
    let schema = r#"{"type":"object","required":["who"],"properties":{"who":{"type":"string"}}}"#;
    let template = "Hello, {{ who }}!\n";
    let blob = json!({
        "manifest": manifest,
        "schemas": { "s/greet.json": schema },
        "templates": { "t/greet.j2": template }
    });
    let js_blob: JsValue = blob.serialize(&serializer()).unwrap();
    let data: JsValue = json!({"who": "Filament"}).serialize(&serializer()).unwrap();
    let out = quire_wasm::render_from_blob("greeting", js_blob, data).expect("render_from_blob ok");
    assert!(out.contains("Hello, Filament!"), "rendered output: {out}");
}

#[wasm_bindgen_test]
fn validate_from_blob_rejects_missing_required() {
    use serde::Serialize as _;
    let manifest = "name: vm\nartifact_types:\n- name: req\n  template_ref: t/r.j2\n  frontmatter_schema_ref: s/r.json\n";
    let schema = r#"{"type":"object","required":["id"],"properties":{"id":{"type":"string"}}}"#;
    let template = "{{ id }}\n";
    let blob = json!({
        "manifest": manifest,
        "schemas": { "s/r.json": schema },
        "templates": { "t/r.j2": template }
    });
    let js_blob: JsValue = blob.serialize(&serializer()).unwrap();
    let data: JsValue = json!({}).serialize(&serializer()).unwrap();
    let err = quire_wasm::validate_from_blob("req", js_blob, data);
    assert!(err.is_err(), "expected validation failure for missing 'id'");
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
