#!/usr/bin/env node
import { readFile } from "node:fs/promises";
import { diagnosePluginV2, packageArtifactV2, runPluginTckV2, scaffoldPluginV2, validatePluginV2 } from "../dist/src/index.js";

const [command, target] = process.argv.slice(2);
const jsonFile = async path => JSON.parse(await readFile(path, "utf8"));
try {
  if (command === "init") console.log(JSON.stringify(scaffoldPluginV2(target ?? "muse.example.plugin"), null, 2));
  else if (command === "validate") console.log(JSON.stringify(validatePluginV2(await jsonFile(target)), null, 2));
  else if (command === "test") console.log(JSON.stringify(runPluginTckV2(await jsonFile(target)), null, 2));
  else if (command === "package") { const bytes = await readFile(target); console.log(JSON.stringify(packageArtifactV2(bytes))); }
  else if (command === "diagnose") console.log(JSON.stringify(diagnosePluginV2(await jsonFile(target), []), null, 2));
  else throw new Error("usage: muse-sdk init <id> | validate|test|package|diagnose <file>");
} catch (error) { process.stderr.write(`${error instanceof Error ? error.message : String(error)}\n`); process.exitCode = 1; }
