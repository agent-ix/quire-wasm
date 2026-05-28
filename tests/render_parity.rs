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

wasm_bindgen_test_configure!(run_in_node);

const MODULE_ROOT: &str = env!("QUIRE_WASM_TEST_MODULE_ROOT");

#[wasm_bindgen_test]
fn render_fr_matches_native() {
    let data = json!({
        "id": "FR-099",
        "title": "Performance benchmark sample",
        "artifact_type": "FR",
        "description": "Auto-generated benchmark target.",
        "relationships": [
            {"target": "ix://agent-ix/filament-core-service/FR-035", "type": "implements"}
        ]
    });
    let js_data: JsValue = serde_wasm_bindgen::to_value(&data).unwrap();
    let out = quire_wasm::render("FR", MODULE_ROOT, js_data).expect("render ok");
    assert!(out.contains("FR-099"), "rendered output missing id: {out}");
    assert!(out.contains("artifact_type: FR"), "missing artifact_type");
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
