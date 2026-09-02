import { readFile, readdir } from "node:fs/promises";
import { dirname, extname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const walk = async path => (await readdir(path, { withFileTypes: true })).flatMap(entry => {
  const child = join(path, entry.name);
  return entry.isDirectory() ? [walk(child)] : [child];
});
const flatten = async values => (await Promise.all(values)).flat(Infinity);
const files = (await flatten(await walk(join(root, "src"))))
  .filter(file => [".ts", ".json"].includes(extname(file)));
const failures = [];
for (const file of files) {
  const text = await readFile(file, "utf8");
  const display = relative(root, file);
  if (/AppFlowy|ioffice|document\.|office\.|CRDT|Postgres/u.test(text)) {
    failures.push(`${display}: generic Plugin Kit contains product/domain knowledge`);
  }
  if (/from\s+["']@muse\/host-bridge\/src/u.test(text)) {
    failures.push(`${display}: imports a Bridge private path`);
  }
}
if (failures.length > 0) throw new Error(`Plugin Kit boundary violations:\n${failures.join("\n")}`);
console.log(`boundaries OK: ${files.length} source files; only public Bridge seam is allowed`);
