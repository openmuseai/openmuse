import { readFile } from "node:fs/promises";
import { describe, expect, it } from "vitest";
import {
  decideContractEdge,
  facetSchemaDigest,
  validateFacetValue,
  type ContractRefV1,
  type FacetSchemaKind
} from "../src/index.js";

interface Fixture {
  readonly name: string;
  readonly kind: FacetSchemaKind;
  readonly valid: boolean;
  readonly value: unknown;
}

const fixtures = JSON.parse(
  await readFile(new URL("../fixtures/v1/messages.json", import.meta.url), "utf8")
) as Fixture[];
const expectedDigests = JSON.parse(
  await readFile(new URL("../fixtures/v1/schema-digests.json", import.meta.url), "utf8")
) as Record<FacetSchemaKind, string>;

describe("Facet v1 shared contract", () => {
  for (const fixture of fixtures) {
    it(fixture.name, () => {
      const run = (): unknown => validateFacetValue(fixture.kind, fixture.value);
      if (fixture.valid) expect(run()).toEqual(fixture.value);
      else expect(run).toThrowError(/INVALID_FACET_CONTRACT/u);
    });
  }

  it("produces stable domain-separated schema digests", () => {
    const kinds: FacetSchemaKind[] = [
      "plugin-descriptor", "context-contribution", "domain-change",
      "presentation-intent", "presentation-intent-result"
    ];
    const values = kinds.map(facetSchemaDigest);
    expect(Object.fromEntries(kinds.map(kind => [kind, facetSchemaDigest(kind)]))).toEqual(expectedDigests);
    expect(new Set(values).size).toBe(kinds.length);
    for (const value of values) expect(value).toMatch(/^sha256:[0-9a-f]{64}$/u);
  });
});

describe("contract edge composition", () => {
  const published: ContractRefV1 = {
    kind: "context",
    type: "markdown.selection.v1",
    schemaDigest: `sha256:${"a".repeat(64)}`,
    required: false
  };

  it("enables only exact type and digest matches", () => {
    expect(decideContractEdge(published, { ...published, required: true })).toEqual({ status: "enabled" });
    expect(decideContractEdge(published, { ...published, schemaDigest: `sha256:${"b".repeat(64)}`, required: true }))
      .toEqual({ status: "disabled", reasonCode: "SCHEMA_DIGEST_MISMATCH" });
  });

  it("degrades an optional consumer and isolates denied scope", () => {
    expect(decideContractEdge(published, { ...published, schemaDigest: `sha256:${"b".repeat(64)}` }))
      .toEqual({ status: "degraded", reasonCode: "SCHEMA_DIGEST_MISMATCH" });
    expect(decideContractEdge(published, published, false))
      .toEqual({ status: "disabled", reasonCode: "SCOPE_DENIED" });
  });
});
