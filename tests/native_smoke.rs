//! Native (non-wasm) smoke test: proves the binding glue compiles and
//! the underlying quire-rs parse/validate pipeline returns sensible
//! output against the in-repo `spec-artifacts-iso` module.
//!
//! The render/templating feature was removed from quire-rs (commit
//! `e0811a8`); this lane exercises parseDocument and schema validation
//! under `cargo test`.

#![cfg(not(target_arch = "wasm32"))]

use std::path::PathBuf;

use serde_json::json;

fn module_root() -> PathBuf {
    let manifest = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest).join("../spec-artifacts-iso/spec_artifacts_iso")
}

#[test]
fn parse_document_returns_sections() {
    // parseDocument is fs-free; safe to run unconditionally.
    let md = "---\nid: FR-099\n---\n\n# Heading\n\nbody.\n";
    let doc = quire_rs::parse_document(md);
    let v = serde_json::to_value(&doc).unwrap();
    assert!(v.get("sections").is_some());
}

#[test]
fn validate_fr_smoke_via_quire_rs() {
    // Only run if the sibling module is checked out alongside this
    // repo. CI's setup explicitly checks both out; locally this is
    // best-effort.
    let root = module_root();
    if !root.join("manifest.yaml").exists() {
        eprintln!("skip: sibling spec-artifacts-iso not present at {root:?}");
        return;
    }
    let registry = quire_rs::Registry::load_module(root.as_path()).expect("registry load");
    let archetype = registry.archetype("FR").expect("FR archetype present");
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
    quire_rs::validate(archetype, &data).expect("valid FR should pass schema validation");
}
