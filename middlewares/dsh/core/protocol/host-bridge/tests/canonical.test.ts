import canonicalCases from "../fixtures/v1/canonical.json" with { type: "json" };
import { describe, expect, it } from "vitest";
import { canonicalJsonText, digestGrant, digestInput, digestSchema } from "../src/index.js";

describe("RFC 8785 canonical JSON and domain-separated digests", () => {
  for (const fixture of canonicalCases) {
    it(fixture.name, () => {
      expect(canonicalJsonText(fixture.value)).toBe(fixture.canonical);
      expect(digestSchema(fixture.value)).toBe(fixture.schemaDigest);
      expect(digestInput(fixture.value)).toBe(fixture.inputDigest);
      expect(digestGrant(fixture.value)).toBe(fixture.grantDigest);
    });
  }

  it("uses distinct digest domains for the same JSON", () => {
    const value = { same: true };
    expect(new Set([digestSchema(value), digestInput(value), digestGrant(value)]).size).toBe(3);
  });
});
