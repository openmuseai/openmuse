import { readFile, readdir } from "node:fs/promises";
import { dirname, extname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";

const packageRoot = join(dirname(fileURLToPath(import.meta.url)), "..");
const scanRoots = [
  "src",
  "schemas",
  join("rust", "src"),
  join("rust", "host-registry", "src"),
  join("rust", "host-transport", "src"),
  join("rust", "host-policy", "src")
];
const forbiddenModule = /(?:^|[/@])(deepseek-harness|appflowy|ioffice|muse-plugins-[^/]*)(?:[/@]|$)/i;
const forbiddenProductText = [
  /\bAppFlowy\b/i,
  /\bioffice\b/i,
  /\bmuse-plugins-/i,
  /\bdocument\./i,
  /\boffice\./i,
  /\b(?:Word|PDF|PPT|Excel)\b/,
  /\b(?:Flutter|CRDT|Postgres)\b/,
  /\bchild_process\b/,
  /\bnode:fs\b/,
  /\bShell\b/
];
const allowedRuntimeDependencies = new Set(["@noble/hashes", "ajv", "ajv-formats", "canonicalize"]);

const walk = async (path) => {
  const entries = await readdir(path, { withFileTypes: true });
  const output = [];
  for (const entry of entries) {
    const child = join(path, entry.name);
    if (entry.isDirectory()) output.push(...await walk(child));
    else if ([".ts", ".rs", ".json"].includes(extname(entry.name))) output.push(child);
  }
  return output;
};

const failures = [];
for (const root of scanRoots) {
  let files;
  try {
    files = await walk(join(packageRoot, root));
  } catch (error) {
    if (error?.code === "ENOENT") continue;
    throw error;
  }
  for (const file of files) {
    const text = await readFile(file, "utf8");
    const display = relative(packageRoot, file);
    const dshAdapter = display.startsWith(join("src", "dsh") + "/");
    for (const match of text.matchAll(/(?:from\s+|import\s*\(|require\s*\()\s*["']([^"']+)["']/g)) {
      const allowedCordis = dshAdapter && match[1] === "@deepseek-ai/cordis";
      if (!allowedCordis && forbiddenModule.test(match[1])) failures.push(`${display}: forbidden module ${match[1]}`);
    }
    if (!dshAdapter && /\bCordis\b/.test(text)) failures.push(`${display}: Cordis knowledge is restricted to src/dsh`);
    if (dshAdapter && /\.(?:tools\.register|plugin\s*\()/u.test(text)) {
      failures.push(`${display}: DSH adapter must not register Tools or nested Plugins`);
    }
    for (const pattern of forbiddenProductText) {
      if (pattern.test(text)) failures.push(`${display}: forbidden product/domain text ${pattern}`);
    }
  }
}

const packageJson = JSON.parse(await readFile(join(packageRoot, "package.json"), "utf8"));
for (const dependency of Object.keys(packageJson.dependencies ?? {})) {
  if (!allowedRuntimeDependencies.has(dependency)) failures.push(`package.json: runtime dependency not allowlisted: ${dependency}`);
}

if (failures.length > 0) {
  throw new Error(`Bridge boundary violations:\n${failures.join("\n")}`);
}
console.log(`boundaries OK: ${scanRoots.join(", ")}; ${allowedRuntimeDependencies.size} runtime dependencies allowlisted`);
