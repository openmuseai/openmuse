import {
  HARD_LIMITS,
  ProviderSchemaRegistry,
  snapshotJson,
  type BridgeLimits,
  type JsonValue,
  type SchemaResourceV1,
  type ValidatedSchema
} from "@muse/host-bridge";
import {
  assertObjectJsonSchema,
  assertSupportedJsonSchema,
  type JsonSchemaNode,
  type ObjectJsonSchema
} from "@deepseek-ai/dsh-tools";

const decoder = new TextDecoder("utf-8", { fatal: true });

const parse = (value: unknown | string | Uint8Array): unknown => {
  if (value instanceof Uint8Array) return JSON.parse(decoder.decode(value)) as unknown;
  if (typeof value === "string") return JSON.parse(value) as unknown;
  return value;
};

export interface MaterializedProviderSchema {
  readonly raw: JsonValue;
  readonly validator: ValidatedSchema;
}

export class MuseProviderSchemas {
  private readonly raw = new Map<string, JsonValue>();
  private readonly registry: ProviderSchemaRegistry;

  constructor(
    private readonly resolveSchema: ((resource: SchemaResourceV1, signal: AbortSignal) => Promise<unknown | string | Uint8Array>) | undefined,
    private readonly limits: BridgeLimits = HARD_LIMITS
  ) {
    this.registry = new ProviderSchemaRegistry({
      resolve: async (uri, expectedByteLength) => {
        if (this.resolveSchema === undefined) throw new Error("schema resolver unavailable");
        const resource = this.pending.get(`${uri}:${expectedByteLength}`);
        if (resource === undefined) throw new Error("schema resolver request was not registered");
        return resource.raw;
      }
    }, limits);
  }

  private readonly pending = new Map<string, { raw: unknown }>();

  async materialize(resource: SchemaResourceV1, signal: AbortSignal): Promise<MaterializedProviderSchema> {
    const cached = this.raw.get(resource.sha256);
    if (cached !== undefined) {
      const validator = this.registry.get(resource.sha256);
      if (validator === undefined) throw new Error("schema cache invariant violated");
      return { raw: cached, validator };
    }
    let candidate: unknown;
    if (resource.inline !== undefined) {
      candidate = resource.inline;
    } else {
      if (resource.uri === undefined || this.resolveSchema === undefined) {
        throw new Error("SCHEMA_UNAVAILABLE: URI schema requires a Plugin-provided resolver");
      }
      const key = `${resource.uri}:${resource.byteLength}`;
      candidate = parse(await this.resolveSchema(resource, signal));
      this.pending.set(key, { raw: candidate });
    }
    const raw = snapshotJson(candidate, {
      maxDepth: this.limits.maxJsonDepth,
      maxContainerChildren: this.limits.maxContainerChildren
    });
    let validator: ValidatedSchema;
    try {
      validator = await this.registry.validateSchemaResource(resource);
    } finally {
      if (resource.uri !== undefined) this.pending.delete(`${resource.uri}:${resource.byteLength}`);
    }
    this.raw.set(resource.sha256, raw);
    return { raw, validator };
  }
}

export const assertMuseToolSchemas = (
  parameters: ObjectJsonSchema,
  output: JsonSchemaNode
): void => {
  assertObjectJsonSchema(parameters);
  assertSupportedJsonSchema(output);
};
