import { readFile, readdir } from "node:fs/promises";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const forbidden = [
  /from\s+["'](?:appflowy|flowy-|collab)/u,
  /from\s+["'](?:node:)?(?:fs|child_process|worker_threads|vm)/u,
  /postgres|crdt/iu
];

const walk = async directory => (await Promise.all((await readdir(directory, { withFileTypes: true })).map(async entry => {
  const path = join(directory, entry.name);
  return entry.isDirectory() ? walk(path) : [path];
}))).flat();

for (const file of await walk(fileURLToPath(new URL("../src", import.meta.url)))) {
  if (!file.endsWith(".ts")) continue;
  const source = await readFile(file, "utf8");
  for (const pattern of forbidden) {
    if (pattern.test(source)) throw new Error(`forbidden domain bypass in ${file}: ${pattern}`);
  }
}
