import { Ajv2020, type ErrorObject, type ValidateFunction } from "ajv/dist/2020.js";
import * as formatsModule from "ajv-formats";
import type { FormatsPlugin } from "ajv-formats";
import { parseOpaqueId } from "../contract/brands.js";
import { snapshotJson } from "../contract/json.js";
import { isHelloKind, isMessageKind, MESSAGE_SCHEMA_REFS, type MessageKind } from "../contract/methods.js";
import { ENVELOPE_SCHEMA_ID, PROTOCOL_SCHEMA_DOCUMENTS } from "../contract/schema-documents.js";
import type {
  BridgeLimits,
  BridgeMessageV1,
  JsonObject,
  JsonValue,
  NegotiatedProtocol,
  ResponseCorrelation,
  WireEnvelope
} from "../contract/types.js";
import { canonicalizeJson } from "./canonical.js";
import { ProtocolViolation } from "./errors.js";
import { assertWithinHardLimits, HARD_LIMITS } from "./limits.js";

const decoder = new TextDecoder("utf-8", { fatal: true });
const encoder = new TextEncoder();
const addFormats = formatsModule.default as unknown as FormatsPlugin;

const ajv = new Ajv2020({ allErrors: true, strict: true, strictRequired: false, validateFormats: true });
addFormats(ajv);
for (const schema of PROTOCOL_SCHEMA_DOCUMENTS) ajv.addSchema(schema);

const envelopeValidator = ajv.getSchema(ENVELOPE_SCHEMA_ID);
if (envelopeValidator === undefined) throw new Error("Bridge envelope schema was not registered");
const payloadValidators = new Map<MessageKind, ValidateFunction>();
for (const [kind, schemaRef] of Object.entries(MESSAGE_SCHEMA_REFS) as [MessageKind, string][]) {
  payloadValidators.set(kind, ajv.compile({ $ref: schemaRef }));
}

const firstValidationError = (errors: ErrorObject[] | null | undefined): Readonly<Record<string, JsonValue>> => {
  const first = errors?.[0];
  if (first === undefined) return Object.freeze({ reason: "schema validation failed" });
  return Object.freeze({
    field: (first.instancePath || "$unknown").slice(0, 256),
    reason: (first.message ?? first.keyword).slice(0, 1024)
  });
};

const parseWireValue = (raw: unknown, limits: BridgeLimits): JsonValue => {
  let parsed: unknown = raw;
  if (raw instanceof Uint8Array) {
    if (raw.byteLength > limits.maxMessageBytes) {
      throw new ProtocolViolation("INVALID_ENVELOPE", "logical message exceeds negotiated byte limit", {
        limit: limits.maxMessageBytes
      });
    }
    try {
      parsed = JSON.parse(decoder.decode(raw)) as unknown;
    } catch {
      throw new ProtocolViolation("INVALID_ENVELOPE", "logical message is not valid UTF-8 JSON");
    }
  } else if (typeof raw === "string") {
    if (encoder.encode(raw).byteLength > limits.maxMessageBytes) {
      throw new ProtocolViolation("INVALID_ENVELOPE", "logical message exceeds negotiated byte limit", {
        limit: limits.maxMessageBytes
      });
    }
    try {
      parsed = JSON.parse(raw) as unknown;
    } catch {
      throw new ProtocolViolation("INVALID_ENVELOPE", "logical message is not valid JSON");
    }
  }
  try {
    const detached = snapshotJson(parsed, {
      maxDepth: limits.maxJsonDepth,
      maxContainerChildren: limits.maxContainerChildren
    });
    if (canonicalizeJson(detached).byteLength > limits.maxMessageBytes) {
      throw new ProtocolViolation("INVALID_ENVELOPE", "logical message exceeds negotiated byte limit", {
        limit: limits.maxMessageBytes
      });
    }
    return detached;
  } catch (error) {
    if (error instanceof ProtocolViolation) throw error;
    throw new ProtocolViolation("INVALID_ENVELOPE", "logical message is outside the lossless JSON subset");
  }
};

const asObject = (value: unknown): JsonObject | undefined =>
  typeof value === "object" && value !== null && !Array.isArray(value) ? value as JsonObject : undefined;

const unsafeProviderKeys = new Set([
  "authorization",
  "cookie",
  "password",
  "rawbytes",
  "secret",
  "stack",
  "stacktrace",
  "token"
]);

const containsUnsafeProviderDetail = (value: JsonValue): boolean => {
  if (Array.isArray(value)) return value.some(containsUnsafeProviderDetail);
  const object = asObject(value);
  if (object === undefined) return false;
  return Object.entries(object).some(([key, child]) => {
    const normalized = key.replaceAll(/[-_.]/g, "").toLowerCase();
    return unsafeProviderKeys.has(normalized) ||
      ["authorization", "cookie", "password", "secret", "stacktrace", "token"].some((suffix) =>
        normalized.endsWith(suffix)
      ) ||
      containsUnsafeProviderDetail(child);
  });
};

const checkSpecificLimits = (kind: MessageKind, payload: JsonValue, limits: BridgeLimits): void => {
  const object = asObject(payload);
  if (object === undefined) return;
  if (kind === "invoke.request" && Object.hasOwn(object, "input")) {
    if (canonicalizeJson(object.input).byteLength > limits.maxInputBytes) {
      throw new ProtocolViolation("INPUT_INVALID", "provider input exceeds negotiated byte limit");
    }
  }
  if (kind === "invoke.response" && object.ok === true && Object.hasOwn(object, "value")) {
    if (canonicalizeJson(object.value).byteLength > limits.maxOutputBytes) {
      throw new ProtocolViolation("OUTPUT_INVALID", "provider output exceeds negotiated byte limit");
    }
  }
  const error = asObject(object.error);
  if (error?.kind === "provider") {
    const message = error.message;
    if (typeof message === "string" && (/\n\s*at\s/.test(message) || /stack\s*trace/i.test(message))) {
      throw new ProtocolViolation("INVALID_ENVELOPE", "provider error message contains stack information");
    }
    if (Object.hasOwn(error, "details") && (
      canonicalizeJson(error.details).byteLength > limits.maxErrorDetailsBytes ||
      containsUnsafeProviderDetail(error.details as JsonValue)
    )) {
      throw new ProtocolViolation("INVALID_ENVELOPE", "provider error details fail safety or byte limits");
    }
  }
  if (kind === "bridge.event") {
    const data = asObject(object.data);
    if (data?.eventKind === "provider.event" && Object.hasOwn(data, "payload")) {
      if (canonicalizeJson(data.payload).byteLength > limits.maxEventPayloadBytes) {
        throw new ProtocolViolation("INVALID_ENVELOPE", "provider event payload exceeds negotiated byte limit");
      }
    }
  }
  if (kind === "discover.response" && canonicalizeJson(payload).byteLength > limits.maxDiscoverPageBytes) {
    throw new ProtocolViolation("INVALID_ENVELOPE", "discovery page exceeds negotiated byte limit");
  }
};

const projectEnvelope = (value: JsonObject): WireEnvelope => {
  if (!isMessageKind(value.kind)) throw new ProtocolViolation("INVALID_ENVELOPE", "message kind is not recognized");
  return Object.freeze({
    protocol: "muse-bridge" as const,
    major: 1 as const,
    minor: value.minor as number,
    kind: value.kind,
    ...(value.requestId === undefined ? {} : { requestId: parseOpaqueId(value.requestId, "RequestId") }),
    ...(value.hostSessionId === undefined
      ? {}
      : { hostSessionId: parseOpaqueId(value.hostSessionId, "HostSessionId") }),
    sentAt: value.sentAt as number,
    payload: value.payload as JsonValue,
    ...(value.extensions === undefined
      ? {}
      : { extensions: value.extensions as Readonly<Record<string, JsonValue>> })
  });
};

export const decodeEnvelope = (raw: unknown, limits: BridgeLimits = HARD_LIMITS): WireEnvelope => {
  assertWithinHardLimits(limits);
  const value = parseWireValue(raw, limits);
  const object = asObject(value);
  if (object?.protocol === "muse-bridge" && typeof object.major === "number" && object.major !== 1) {
    throw new ProtocolViolation("UNSUPPORTED_PROTOCOL", "Muse Bridge protocol major is not supported");
  }
  if (!envelopeValidator(value)) {
    throw new ProtocolViolation("INVALID_ENVELOPE", "logical envelope failed validation", firstValidationError(envelopeValidator.errors));
  }
  return projectEnvelope(value as JsonObject);
};

export const decodeMessage = (
  raw: unknown,
  negotiated?: NegotiatedProtocol,
  limits: BridgeLimits = HARD_LIMITS
): BridgeMessageV1 => {
  const envelope = decodeEnvelope(raw, limits);
  if (!isHelloKind(envelope.kind)) {
    if (negotiated === undefined) throw new ProtocolViolation("HANDSHAKE_REQUIRED", "hello must complete before this message");
    if (envelope.major !== negotiated.major || envelope.minor !== negotiated.minor) {
      throw new ProtocolViolation("UNSUPPORTED_PROTOCOL", "message version differs from the negotiated protocol");
    }
    if (envelope.hostSessionId !== negotiated.hostSessionId) {
      throw new ProtocolViolation("HOST_SESSION_EXPIRED", "message host session is not current");
    }
  }
  const validatePayload = payloadValidators.get(envelope.kind);
  if (validatePayload === undefined || !validatePayload(envelope.payload)) {
    throw new ProtocolViolation("INVALID_ENVELOPE", "message payload failed validation", firstValidationError(validatePayload?.errors));
  }
  checkSpecificLimits(envelope.kind, envelope.payload, limits);
  return envelope as unknown as BridgeMessageV1;
};

export const assertResponseMatches = (
  response: { readonly requestId?: ResponseCorrelation["requestId"]; readonly hostSessionId?: ResponseCorrelation["hostSessionId"] },
  expected: ResponseCorrelation
): void => {
  if (response.requestId !== expected.requestId) {
    throw new ProtocolViolation("REQUEST_ID_MISMATCH", "response requestId does not match the initiating request");
  }
  if (expected.hostSessionId !== undefined && response.hostSessionId !== expected.hostSessionId) {
    throw new ProtocolViolation("HOST_SESSION_EXPIRED", "response belongs to a different host session");
  }
};
