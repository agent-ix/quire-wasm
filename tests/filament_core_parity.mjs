import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

import init, {
  extractCoreData,
  extractCoreDataFromMarkdown,
  extractFilamentCore,
} from "../pkg/quire_wasm.js";

const wasmPath = fileURLToPath(new URL("../pkg/quire_wasm_bg.wasm", import.meta.url));
await init({ module_or_path: readFileSync(wasmPath) });

const fixturesPath = resolve(
  fileURLToPath(new URL("..", import.meta.url)),
  "../quire-rs/tests/fixtures/filament_core/graph_cases.json",
);
const fixtures = JSON.parse(readFileSync(fixturesPath, "utf8"));

const python = process.env.QUIRE_PYTHON ?? "python3";
const pythonCode = `
import json
import sys
import quire

payload = json.load(sys.stdin)
result = quire.extract_core_data(payload)
json.dump(result, sys.stdout, sort_keys=True, separators=(",", ":"))
`;

function sortJson(value) {
  if (Array.isArray(value)) {
    return value.map(sortJson);
  }
  if (value && typeof value === "object") {
    return Object.fromEntries(
      Object.entries(value)
        .sort(([left], [right]) => left.localeCompare(right))
        .map(([key, item]) => [key, sortJson(item)]),
    );
  }
  return value;
}

function stable(value) {
  return JSON.stringify(sortJson(value));
}

for (const fixture of fixtures) {
  const wasm = extractFilamentCore(fixture.input);
  const pythonResult = spawnSync(python, ["-c", pythonCode], {
    input: JSON.stringify(fixture.input),
    encoding: "utf8",
  });

  if (pythonResult.status !== 0) {
    throw new Error(
      `${fixture.name}: Python extraction failed\n${pythonResult.stderr}`,
    );
  }

  const py = JSON.parse(pythonResult.stdout);
  if (stable(wasm) !== stable(py)) {
    throw new Error(
      `${fixture.name}: WASM/Python output mismatch\nWASM: ${stable(wasm)}\nPython: ${stable(py)}`,
    );
  }
}

const aliasRequest = fixtures[0].input;
if (stable(extractCoreData(aliasRequest)) !== stable(extractFilamentCore(aliasRequest))) {
  throw new Error("extractCoreData alias diverges from extractFilamentCore");
}
if (
  stable(extractCoreDataFromMarkdown(aliasRequest)) !== stable(extractFilamentCore(aliasRequest))
) {
  throw new Error("extractCoreDataFromMarkdown alias diverges from extractFilamentCore");
}

console.log(`filament core WASM/Python parity passed (${fixtures.length} fixtures)`);
