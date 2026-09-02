import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const here = dirname(fileURLToPath(import.meta.url));
const pluginRoot = join(here, "..");
const museRoot = join(here, "../../../../..");
const contractPath = join(pluginRoot, "contracts/dsh-layout-contract.v1.json");
const harnessRoot = join(museRoot, "vendors/deepseek-harness");
const hasHarness = existsSync(join(harnessRoot, "packages/client/ui-layout/package.json"));

type LayoutContract = {
  dshVersion: string;
  stableAttributes: Record<string, { file: string; values?: string[] }>;
  publicSlots: Record<string, string>;
  publicServices: Record<string, string>;
  forbiddenSelectors: string[];
};

const contract = JSON.parse(readFileSync(contractPath, "utf8")) as LayoutContract;

describe.skipIf(!hasHarness)("M0 DSH layout contract", () => {
  it("pins the expected DSH client version", () => {
    const layoutPkg = JSON.parse(
      readFileSync(join(harnessRoot, "packages/client/ui-layout/package.json"), "utf8"),
    ) as { version: string };
    expect(layoutPkg.version).toBe(contract.dshVersion);
  });

  it("keeps every frozen data-* marker in the declared source file", () => {
    for (const [attr, meta] of Object.entries(contract.stableAttributes)) {
      const source = readFileSync(join(harnessRoot, meta.file), "utf8");
      expect(source, attr).toContain(attr);
      for (const value of meta.values ?? []) {
        expect(source, `${attr}=${value}`).toContain(`'${value}'`);
      }
    }
  });

  it("keeps public slot names in conversation or layout contracts", () => {
    const slots = [
      readFileSync(join(harnessRoot, "packages/client/ui-conversation/src/client/contract/slots.ts"), "utf8"),
      readFileSync(join(harnessRoot, "packages/client/ui-layout/src/client/index.ts"), "utf8"),
    ].join("\n");
    for (const name of Object.keys(contract.publicSlots)) {
      expect(slots, name).toContain(`'${name}'`);
    }
  });

  it("keeps ctx.layout methods on the layout service", () => {
    for (const [method, file] of Object.entries(contract.publicServices)) {
      const source = readFileSync(join(harnessRoot, file), "utf8");
      const fn = method.split(".").pop();
      expect(source, method).toContain(`${fn}()`);
    }
  });

  it("documents selectors that Mobile CSS must not use", () => {
    expect(contract.forbiddenSelectors.length).toBeGreaterThan(0);
  });
});
