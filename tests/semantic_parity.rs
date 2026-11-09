//! `extractSemantic` parity with quire-rs (FR-072-AC-6): every case of
//! quire-rs's `tests/fixtures/semantic/cases.json` must produce the JSON value
//! quire-rs records in `cases.expected.json`.
//!
//! The fixtures are read from the quire-rs checkout cargo resolved for the git
//! dependency; nothing is copied into this repository.

#![cfg(not(target_arch = "wasm32"))]

use std::path::PathBuf;
use std::process::Command;

use serde_json::{json, Value};

/// `tests/fixtures/semantic` inside the quire-rs package cargo resolved.
fn fixtures_dir() -> PathBuf {
    let out = Command::new(env!("CARGO"))
        .args(["metadata", "--format-version", "1"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("cargo metadata runs");
    assert!(out.status.success(), "cargo metadata failed");
    let meta: Value = serde_json::from_slice(&out.stdout).expect("metadata is JSON");
    let manifest = meta["packages"]
        .as_array()
        .expect("packages")
        .iter()
        .find(|p| p["name"] == "quire-rs")
        .expect("quire-rs is a dependency")["manifest_path"]
        .as_str()
        .expect("manifest_path")
        .to_string();
    PathBuf::from(manifest)
        .parent()
        .expect("manifest has a parent")
        .join("tests/fixtures/semantic")
}

fn read_json(name: &str) -> Value {
    let path = fixtures_dir().join(name);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path:?}: {e}"));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("{path:?}: {e}"))
}

// FR-005-AC-1
#[test]
fn extract_semantic_matches_quire_rs_for_every_case() {
    let cases = read_json("cases.json");
    let expected = read_json("cases.expected.json");
    let cases = cases["cases"].as_array().expect("cases array");
    assert!(!cases.is_empty());
    for case in cases {
        let name = case["name"].as_str().expect("case name");
        let record =
            quire_wasm::semantic_record(&case["input"]).unwrap_or_else(|e| panic!("{name}: {e}"));
        assert_eq!(record, expected[name], "{name}");
    }
}

// FR-005-AC-2
#[test]
fn unsupported_contract_version_is_refused() {
    let mut input = read_json("cases.json")["cases"][0]["input"].clone();
    input["module"]["contractVersion"] = json!("9.9.9");
    let err = quire_wasm::semantic_record(&input).expect_err("refused");
    assert!(
        err.starts_with("semantic.unsupported-contract-version"),
        "{err}"
    );
}
