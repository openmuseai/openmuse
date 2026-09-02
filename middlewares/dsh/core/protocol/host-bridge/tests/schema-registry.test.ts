import { describe, expect, it } from "vitest";
import {
  canonicalizeJson,
  digestSchema,
  ProviderSchemaRegistry,
  ProtocolViolation,
  validateProviderInput,
  validateProviderOutput,
  type SchemaResourceV1
} from "../src/index.js";

const schema = {
  $schema: "https://json-schema.org/draft/2020-12/schema",
  type: "object",
  required: ["value"],
  properties: { value: { type: "string", maxLength: 32 } },
  additionalProperties: false
} as const;

const resource = (): SchemaResourceV1 => ({
  sha256: digestSchema(schema),
  byteLength: canonicalizeJson(schema).byteLength,
  draft: "2020-12",
  inline: schema
});

describe("provider schema registry", () => {
  it("validates digest before caching and detaches provider values", async () => {
    const registry = new ProviderSchemaRegistry();
    await registry.validateSchemaResource(resource());
    const input = { value: "hello" };
    const detached = validateProviderInput(registry, resource().sha256, input);
    input.value = "changed";
    expect(detached).toEqual({ value: "hello" });
    expect(validateProviderOutput(registry, resource().sha256, { value: "world" })).toEqual({ value: "world" });
    expect(() => validateProviderInput(registry, resource().sha256, { value: 1 })).toThrowError(ProtocolViolation);
  });

  it("rejects byteLength and digest mismatches", async () => {
    const registry = new ProviderSchemaRegistry();
    await expect(registry.validateSchemaResource({ ...resource(), byteLength: resource().byteLength + 1 }))
      .rejects.toMatchObject({ bridgeError: { code: "SCHEMA_DIGEST_MISMATCH" } });
    await expect(registry.validateSchemaResource({
      ...resource(),
      sha256: "sha256:ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff" as ReturnType<typeof digestSchema>
    })).rejects.toMatchObject({ bridgeError: { code: "SCHEMA_DIGEST_MISMATCH" } });
  });

  it("only resolves URI through the injected resolver", async () => {
    const registryWithoutResolver = new ProviderSchemaRegistry();
    const { inline: _inline, ...baseResource } = resource();
    const uriResource: SchemaResourceV1 = { ...baseResource, uri: "https://schemas.invalid/sample.json" };
    await expect(registryWithoutResolver.validateSchemaResource(uriResource)).rejects.toMatchObject({
      bridgeError: { code: "SCHEMA_UNAVAILABLE" }
    });

    const registry = new ProviderSchemaRegistry({ resolve: async () => schema });
    expect((await registry.validateSchemaResource(uriResource)).digest).toBe(resource().sha256);
  });

  it("does not fetch external references while compiling Provider schemas", async () => {
    const external = {
      $schema: "https://json-schema.org/draft/2020-12/schema",
      $ref: "https://schemas.invalid/external.json"
    } as const;
    const externalResource: SchemaResourceV1 = {
      sha256: digestSchema(external),
      byteLength: canonicalizeJson(external).byteLength,
      draft: "2020-12",
      inline: external
    };
    await expect(new ProviderSchemaRegistry().validateSchemaResource(externalResource)).rejects.toMatchObject({
      bridgeError: { code: "SCHEMA_UNAVAILABLE" }
    });
  });
});
