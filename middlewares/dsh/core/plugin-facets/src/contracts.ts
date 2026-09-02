import { Ajv2020, type ErrorObject, type ValidateFunction } from "ajv/dist/2020.js";
import * as formatsModule from "ajv-formats";
import type { FormatsPlugin } from "ajv-formats";
import { digestSchema, HARD_LIMITS, snapshotJson, type JsonValue } from "@muse/host-bridge";
import pluginDescriptorSchema from "../schemas/v1/plugin-descriptor.schema.json" with { type: "json" };
import contextContributionSchema from "../schemas/v1/context-contribution.schema.json" with { type: "json" };
import domainChangeSchema from "../schemas/v1/domain-change.schema.json" with { type: "json" };
import presentationIntentSchema from "../schemas/v1/presentation-intent.schema.json" with { type: "json" };
import presentationIntentResultSchema from "../schemas/v1/presentation-intent-result.schema.json" with { type: "json" };
import type { CompositionDecision, ContractRefV1, FacetSchemaKind, FacetWireValueV1 } from "./types.js";

const schemas = {
  "plugin-descriptor": pluginDescriptorSchema,
  "context-contribution": contextContributionSchema,
  "domain-change": domainChangeSchema,
  "presentation-intent": presentationIntentSchema,
  "presentation-intent-result": presentationIntentResultSchema
} as const;

const addFormats = formatsModule.default as unknown as FormatsPlugin;
const ajv = new Ajv2020({ allErrors: true, strict: true });
addFormats(ajv);
const traversalLimits = {
  maxDepth: HARD_LIMITS.maxJsonDepth,
  maxContainerChildren: HARD_LIMITS.maxContainerChildren
};
const validators = new Map<FacetSchemaKind, ValidateFunction>(
  Object.entries(schemas).map(([kind, schema]) => [kind as FacetSchemaKind, ajv.compile(schema)])
);

export class FacetContractError extends Error {
  constructor(readonly code: "INVALID_FACET_CONTRACT", readonly issues: readonly ErrorObject[]) {
    super(`${code}: ${issues.map(issue => `${issue.instancePath || "/"} ${issue.message ?? "invalid"}`).join("; ")}`);
  }
}

export const validateFacetValue = <T extends FacetWireValueV1>(kind: FacetSchemaKind, value: unknown): T => {
  const validator = validators.get(kind);
  if (validator === undefined) throw new TypeError(`unknown Facet schema kind ${kind}`);
  if (!validator(value)) throw new FacetContractError("INVALID_FACET_CONTRACT", validator.errors ?? []);
  return snapshotJson(value, traversalLimits) as unknown as T;
};

export const facetSchemaDigest = (kind: FacetSchemaKind): string => digestSchema(schemas[kind]);

export const facetSchemaDocument = (kind: FacetSchemaKind): JsonValue => snapshotJson(schemas[kind], traversalLimits);

/** Plugin-owned payload schemas use the same digest/validator on every carrier. */
export function compileFacetPayload(schema: JsonValue): { schemaDigest: string; validate(value: unknown): void } {
  const validator = ajv.compile(snapshotJson(schema, traversalLimits) as object);
  return {
    schemaDigest: digestSchema(schema),
    validate(value) {
      if (!validator(value)) throw new FacetContractError("INVALID_FACET_CONTRACT", validator.errors ?? []);
    }
  };
}

export const decideContractEdge = (
  publication: ContractRefV1 | undefined,
  consumption: ContractRefV1 | undefined,
  scopeAllowed = true
): CompositionDecision => {
  if (publication === undefined || consumption === undefined) {
    return { status: "disabled", reasonCode: "MISSING_FACET" };
  }
  if (!scopeAllowed) return { status: "disabled", reasonCode: "SCOPE_DENIED" };
  if (publication.kind !== consumption.kind || publication.type !== consumption.type) {
    return { status: "disabled", reasonCode: "TYPE_MISMATCH" };
  }
  if (publication.schemaDigest !== consumption.schemaDigest) {
    return consumption.required
      ? { status: "disabled", reasonCode: "SCHEMA_DIGEST_MISMATCH" }
      : { status: "degraded", reasonCode: "SCHEMA_DIGEST_MISMATCH" };
  }
  return { status: "enabled" };
};
