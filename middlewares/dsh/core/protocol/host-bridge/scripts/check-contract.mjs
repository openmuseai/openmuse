import { readFile, readdir } from "node:fs/promises";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { sha256 } from "@noble/hashes/sha2.js";
import { bytesToHex, concatBytes, utf8ToBytes } from "@noble/hashes/utils.js";
import Ajv2020 from "ajv/dist/2020.js";
import addFormats from "ajv-formats";
import canonicalize from "canonicalize";

const packageRoot = join(dirname(fileURLToPath(import.meta.url)), "..");
const schemaDir = join(packageRoot, "schemas", "v1");
const manifest = JSON.parse(await readFile(join(schemaDir, "protocol-manifest.json"), "utf8"));
const ajv = new Ajv2020({ allErrors: true, strict: true, strictRequired: false, validateFormats: true });
addFormats(ajv);

const digestSchema = (schema) => {
  const bytes = concatBytes(utf8ToBytes("muse-schema-v1\0"), utf8ToBytes(canonicalize(schema)));
  return `sha256:${bytesToHex(sha256(bytes))}`;
};

const documents = new Map();
for (const [name, entry] of Object.entries(manifest.schemas)) {
  const schema = JSON.parse(await readFile(join(schemaDir, entry.file), "utf8"));
  if (schema.$id !== entry.id) {
    throw new Error(`${name}: manifest id ${entry.id} does not match schema $id ${schema.$id}`);
  }
  if (!ajv.validateSchema(schema)) {
    throw new Error(`${name}: invalid JSON Schema: ${ajv.errorsText(ajv.errors)}`);
  }
  const actualDigest = digestSchema(schema);
  if (actualDigest !== entry.digest) {
    throw new Error(`${name}: digest mismatch; manifest=${entry.digest} actual=${actualDigest}`);
  }
  documents.set(entry.id, schema);
}

for (const schema of documents.values()) {
  ajv.addSchema(schema);
}

const envelope = documents.get(manifest.schemas.envelope.id);
const envelopeKinds = new Set(envelope.properties.kind.enum);
const manifestKinds = new Set(Object.keys(manifest.messages));
if (envelopeKinds.size !== manifestKinds.size || [...envelopeKinds].some((kind) => !manifestKinds.has(kind))) {
  throw new Error("envelope kind enum and protocol manifest message map differ");
}

for (const [kind, entry] of Object.entries(manifest.messages)) {
  try {
    ajv.compile({ $ref: entry.schema });
  } catch (error) {
    throw new Error(`${kind}: payload schema cannot compile: ${String(error)}`, { cause: error });
  }
}

const expectedMethods = Object.fromEntries(
  Object.entries(manifest.messages).map(([kind, entry]) => [kind, entry.schema])
);
const typeScriptMethodsSource = await readFile(join(packageRoot, "src", "contract", "methods.ts"), "utf8");
const typeScriptMethods = Object.fromEntries(
  [...typeScriptMethodsSource.matchAll(/^\s+"([^"]+)":\s+"([^"]+)",?$/gm)].map((match) => [match[1], match[2]])
);
if (JSON.stringify(typeScriptMethods) !== JSON.stringify(expectedMethods)) {
  throw new Error("TypeScript message/schema map differs from protocol manifest");
}

const rustCodecSource = await readFile(join(packageRoot, "rust", "src", "codec.rs"), "utf8");
const rustMethods = Object.fromEntries(
  [...rustCodecSource.matchAll(/\(\s*"([^"]+)"\s*,\s*"([^"]+)"\s*,?\s*\)/g)].map((match) => [match[1], match[2]])
);
if (JSON.stringify(rustMethods) !== JSON.stringify(expectedMethods)) {
  throw new Error("Rust message/schema map differs from protocol manifest");
}

const schemaFiles = (await readdir(schemaDir)).filter((name) => name.endsWith(".schema.json")).sort();
const manifestFiles = Object.values(manifest.schemas).map((entry) => entry.file).sort();
if (JSON.stringify(schemaFiles) !== JSON.stringify(manifestFiles)) {
  throw new Error(`manifest schema file set differs: files=${schemaFiles.join(",")} manifest=${manifestFiles.join(",")}`);
}

console.log(`contract OK: ${manifestKinds.size} message kinds, ${documents.size} schema documents`);
