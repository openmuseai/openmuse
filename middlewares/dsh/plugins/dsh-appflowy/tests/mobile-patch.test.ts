import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const patch = readFileSync(
  join(dirname(fileURLToPath(import.meta.url)), "../cordis.patch.yml"),
  "utf8",
);

describe("mobile presentation patch", () => {
  it("inserts surface and input plugins after the webview encoder", () => {
    expect(patch).toContain("name: '@muse/dsh-mobile-surface'");
    expect(patch).toContain("name: '@muse/dsh-mobile-input'");
    expect(patch.indexOf("@muse/dsh-appflowy/webview")).toBeLessThan(
      patch.indexOf("@muse/dsh-mobile-surface"),
    );
  });
});
