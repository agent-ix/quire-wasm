import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

import init, {
  extractCoreData,
  extractCoreDataFromMarkdown,
  extractFilamentCore,
} from "../pkg/quire_wasm.js";

const wasmPath = fileURLToPath(new URL("../pkg/quire_wasm_bg.wasm", import.meta.url));
await init({ module_or_path: readFileSync(wasmPath) });

const objectTypes = {
  capability: {
    name: "capability",
    schema: { type: "object", additionalProperties: true },
    allowedLinks: {},
    bodyExtraction: null,
    hasPlugin: false,
    moduleId: null,
  },
  endpoint: {
    name: "endpoint",
    schema: {
      type: "object",
      required: ["method", "target"],
      properties: {
        method: { type: "string" },
        target: { type: "string" },
      },
    },
    allowedLinks: {},
    bodyExtraction: {
      yield_pattern: {
        match: {
          method: {
            from: "section_body",
            after_heading: "Endpoint",
            regex: "^(GET)",
            required: true,
          },
          target: {
            from: "section_body",
            after_heading: "Endpoint",
            regex: "(/payments)",
            required: true,
          },
        },
      },
      emit_edges: [{ type: "serves", target: "ix://agent-ix/example/API" }],
    },
    hasPlugin: false,
    moduleId: null,
  },
  plugin: {
    name: "plugin",
    schema: { type: "object", additionalProperties: true },
    allowedLinks: {},
    bodyExtraction: null,
    hasPlugin: true,
    moduleId: null,
  },
};

function request(markdown, objectTypesForFixture = [objectTypes.capability]) {
  return {
    projectId: "project",
    documentId: "doc-1",
    artifactId: "artifact-1",
    relPath: "spec/FR-001.md",
    markdown,
    repoName: "example",
    objectTypes: objectTypesForFixture,
  };
}

const fixtures = [
  {
    name: "tier1-frontmatter",
    request: request(
      "---\nid: FR-001\ntitle: Pay vendors\nobject: capability\npriority: P0\n---\n# Body\n",
    ),
  },
  {
    name: "tier2-dsl",
    request: request(
      "---\nid: API-001\ntitle: Payments API\nobject: endpoint\n---\n## Endpoint\nGET /payments\n",
      [objectTypes.endpoint],
    ),
  },
  {
    name: "unknown-object",
    request: request("---\nid: FR-001\nobject: missing\n---\n# Body\n"),
  },
  {
    name: "relationship-and-body-links",
    request: request(
      "---\nid: FR-001\nobject: capability\ndepends_on:\n  - FR-002\n---\nSee [US-001](ix://agent-ix/example/US-001).\n",
    ),
  },
  {
    name: "duplicate-edge",
    request: request(
      "---\nid: FR-001\nobject: capability\nrelationships:\n  - target: ix://agent-ix/example/FR-002\n    type: references\n---\nSee [FR-002](ix://agent-ix/example/FR-002).\n",
    ),
  },
  {
    name: "malformed-ix-uri",
    request: request(
      "---\nid: FR-001\nobject: capability\nrelationships:\n  - target: ix://agent-ix\n    type: references\n---\n# Body\n",
    ),
  },
  {
    name: "plugin-error",
    request: request("---\nid: PLUG-001\nobject: plugin\n---\n# Body\n", [objectTypes.plugin]),
  },
];

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
  const wasm = extractFilamentCore(fixture.request);
  const pythonResult = spawnSync(python, ["-c", pythonCode], {
    input: JSON.stringify(fixture.request),
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

const aliasRequest = fixtures[0].request;
if (stable(extractCoreData(aliasRequest)) !== stable(extractFilamentCore(aliasRequest))) {
  throw new Error("extractCoreData alias diverges from extractFilamentCore");
}
if (
  stable(extractCoreDataFromMarkdown(aliasRequest)) !== stable(extractFilamentCore(aliasRequest))
) {
  throw new Error("extractCoreDataFromMarkdown alias diverges from extractFilamentCore");
}

console.log(`filament core WASM/Python parity passed (${fixtures.length} fixtures)`);
