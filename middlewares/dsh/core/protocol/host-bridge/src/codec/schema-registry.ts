import { Ajv2020, type AnySchema, type ValidateFunction } from "ajv/dist/2020.js";
import { snapshotJson } from "../contract/json.js";
import type { BridgeErrorCode, BridgeLimits, JsonValue, SchemaResourceV1 } from "../contract/types.js";
import { canonicalizeJson } from "./canonical.js";
import { digestSchema } from "./digest.js";
import { ProtocolViolation } from "./errors.js";
import { assertWithinHardLimits, HARD_LIMITS } from "./limits.js";

export interface SchemaResolver {
  resolve(uri: string, expectedByteLength: number): Promise<unknown | string | Uint8Array>;
}

export interface ValidatedSchema {
  readonly digest: SchemaResourceV1["sha256"];
  readonly byteLength: number;
  validate(value: unknown, maxBytes: number, errorCode: "INPUT_INVALID" | "OUTPUT_INVALID"): JsonValue;
}

const decoder = new TextDecoder("utf-8", { fatal: true });

const parseResolvedSchema = (raw: unknown | string | Uint8Array, maxBytes: number): unknown => {
  if (raw instanceof Uint8Array) {
    if (raw.byteLength > maxBytes) throw new ProtocolViolation("SCHEMA_UNAVAILABLE", "resolved schema exceeds byte limit");
    try {
      return JSON.parse(decoder.decode(raw)) as unknown;
    } catch {
      throw new ProtocolViolation("SCHEMA_UNAVAILABLE", "resolved schema is not valid UTF-8 JSON");
    }
  }
  if (typeof raw === "string") {
    if (new TextEncoder().encode(raw).byteLength > maxBytes) {
      throw new ProtocolViolation("SCHEMA_UNAVAILABLE", "resolved schema exceeds byte limit");
    }
    try {
      return JSON.parse(raw) as unknown;
    } catch {
      throw new ProtocolViolation("SCHEMA_UNAVAILABLE", "resolved schema is not valid JSON");
    }
  }
  return raw;
};

const validationMessage = (validate: ValidateFunction): string => {
  const first = validate.errors?.[0];
  if (first === undefined) return "value does not satisfy schema";
  return `${first.instancePath || "$"}: ${first.message ?? first.keyword}`.slice(0, 1024);
};

export class ProviderSchemaRegistry {
  readonly #cache = new Map<string, ValidatedSchema>();

  public constructor(
    private readonly resolver?: SchemaResolver,
    private readonly limits: BridgeLimits = HARD_LIMITS
  ) {
    assertWithinHardLimits(limits);
  }

  public get(digest: string): ValidatedSchema | undefined {
    return this.#cache.get(digest);
  }

  public async validateSchemaResource(resource: SchemaResourceV1): Promise<ValidatedSchema> {
    const cached = this.#cache.get(resource.sha256);
    if (cached !== undefined) return cached;

    const hasInline = Object.hasOwn(resource, "inline");
    let candidate: unknown;
    let ceiling: number;
    if (hasInline) {
      candidate = resource.inline;
      ceiling = this.limits.maxInlineSchemaBytes;
    } else {
      if (resource.uri === undefined || this.resolver === undefined) {
        throw new ProtocolViolation("SCHEMA_UNAVAILABLE", "schema URI has no policy-provided resolver");
      }
      candidate = parseResolvedSchema(
        await this.resolver.resolve(resource.uri, resource.byteLength),
        this.limits.maxResolvedSchemaBytes
      );
      ceiling = this.limits.maxResolvedSchemaBytes;
    }

    const snapshot = snapshotJson(candidate, {
      maxDepth: this.limits.maxJsonDepth,
      maxContainerChildren: this.limits.maxContainerChildren
    });
    const canonicalBytes = canonicalizeJson(snapshot);
    if (canonicalBytes.byteLength > ceiling) {
      throw new ProtocolViolation("SCHEMA_UNAVAILABLE", "schema canonical form exceeds byte limit");
    }
    if (canonicalBytes.byteLength !== resource.byteLength) {
      throw new ProtocolViolation("SCHEMA_DIGEST_MISMATCH", "schema byteLength does not match canonical content");
    }
    if (digestSchema(snapshot) !== resource.sha256) {
      throw new ProtocolViolation("SCHEMA_DIGEST_MISMATCH", "schema digest does not match canonical content");
    }
    if (typeof snapshot !== "boolean" && (typeof snapshot !== "object" || snapshot === null || Array.isArray(snapshot))) {
      throw new ProtocolViolation("SCHEMA_UNAVAILABLE", "schema root must be a boolean or object");
    }
    if (
      typeof snapshot === "object" &&
      snapshot !== null &&
      !Array.isArray(snapshot) &&
      (snapshot as Readonly<Record<string, JsonValue>>).$schema !== "https://json-schema.org/draft/2020-12/schema"
    ) {
      throw new ProtocolViolation("SCHEMA_UNAVAILABLE", "schema must declare JSON Schema draft 2020-12");
    }

    const ajv = new Ajv2020({ allErrors: true, strict: true, strictRequired: false, validateFormats: false });
    const schema = snapshot as AnySchema;
    if (!ajv.validateSchema(schema)) {
      throw new ProtocolViolation("SCHEMA_UNAVAILABLE", "schema fails the draft 2020-12 meta-schema");
    }
    let validate: ValidateFunction;
    try {
      validate = ajv.compile(schema);
    } catch {
      throw new ProtocolViolation("SCHEMA_UNAVAILABLE", "schema contains an unavailable external reference");
    }

    const validated: ValidatedSchema = Object.freeze({
      digest: resource.sha256,
      byteLength: canonicalBytes.byteLength,
      validate: (value: unknown, maxBytes: number, errorCode: "INPUT_INVALID" | "OUTPUT_INVALID"): JsonValue => {
        const detached = snapshotJson(value, {
          maxDepth: this.limits.maxJsonDepth,
          maxContainerChildren: this.limits.maxContainerChildren
        });
        if (canonicalizeJson(detached).byteLength > maxBytes) {
          throw new ProtocolViolation(errorCode, "provider value exceeds negotiated byte limit");
        }
        if (!validate(detached)) throw new ProtocolViolation(errorCode, validationMessage(validate));
        return detached;
      }
    });
    this.#cache.set(resource.sha256, validated);
    return validated;
  }

  public validateByDigest(
    digest: string,
    value: unknown,
    maxBytes: number,
    errorCode: Extract<BridgeErrorCode, "INPUT_INVALID" | "OUTPUT_INVALID">
  ): JsonValue {
    const schema = this.#cache.get(digest);
    if (schema === undefined) throw new ProtocolViolation("SCHEMA_UNAVAILABLE", "schema is not present in the validated cache");
    return schema.validate(value, maxBytes, errorCode);
  }
}

export const validateProviderInput = (
  registry: ProviderSchemaRegistry,
  digest: string,
  input: unknown,
  limits: BridgeLimits = HARD_LIMITS
): JsonValue => registry.validateByDigest(digest, input, limits.maxInputBytes, "INPUT_INVALID");

export const validateProviderOutput = (
  registry: ProviderSchemaRegistry,
  digest: string,
  output: unknown,
  limits: BridgeLimits = HARD_LIMITS
): JsonValue => registry.validateByDigest(digest, output, limits.maxOutputBytes, "OUTPUT_INVALID");
