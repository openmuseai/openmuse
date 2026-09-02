import { readFile, readdir } from "node:fs/promises";
import { join } from "node:path";

const root = new URL("..", import.meta.url).pathname;
const schemaDir = join(root, "schemas", "v1");
const forbidden = ["markdownBlock", "wordSection", "excelRange", "selectedText", "formula"];
const files = (await readdir(schemaDir)).filter(file => file.endsWith(".schema.json")).sort();
if (files.length !== 5) throw new Error(`expected 5 v1 schemas, got ${files.length}`);
for (const file of files) {
  const text = await readFile(join(schemaDir, file), "utf8");
  JSON.parse(text);
  for (const token of forbidden) {
    if (text.includes(token)) throw new Error(`${file} leaks domain token ${token}`);
  }
}
const fixtures = JSON.parse(await readFile(join(root, "fixtures", "v1", "messages.json"), "utf8"));
if (!Array.isArray(fixtures) || fixtures.length < 10) throw new Error("contract fixtures are incomplete");
process.stdout.write(`Facet contract OK: ${files.length} schemas, ${fixtures.length} fixtures\n`);
