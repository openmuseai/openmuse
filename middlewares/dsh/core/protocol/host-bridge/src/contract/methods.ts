export const MESSAGE_SCHEMA_REFS = {
  "hello.request": "https://muse.dev/schemas/bridge/v1/hello.schema.json#/$defs/request",
  "hello.response": "https://muse.dev/schemas/bridge/v1/hello.schema.json#/$defs/response",
  "discover.request": "https://muse.dev/schemas/bridge/v1/discover.schema.json#/$defs/request",
  "discover.response": "https://muse.dev/schemas/bridge/v1/discover.schema.json#/$defs/response",
  "bind.request": "https://muse.dev/schemas/bridge/v1/bind.schema.json#/$defs/request",
  "bind.response": "https://muse.dev/schemas/bridge/v1/bind.schema.json#/$defs/response",
  "invoke.request": "https://muse.dev/schemas/bridge/v1/invoke.schema.json#/$defs/request",
  "invoke.response": "https://muse.dev/schemas/bridge/v1/invoke.schema.json#/$defs/response",
  "subscribe.request": "https://muse.dev/schemas/bridge/v1/subscribe.schema.json#/$defs/request",
  "subscribe.response": "https://muse.dev/schemas/bridge/v1/subscribe.schema.json#/$defs/response",
  "bridge.event": "https://muse.dev/schemas/bridge/v1/subscribe.schema.json#/$defs/event",
  "policy.evaluate.request": "https://muse.dev/schemas/bridge/v1/policy.schema.json#/$defs/evaluateRequest",
  "policy.evaluate.response": "https://muse.dev/schemas/bridge/v1/policy.schema.json#/$defs/evaluateResponse",
  "policy.finalize.request": "https://muse.dev/schemas/bridge/v1/policy.schema.json#/$defs/finalizeRequest",
  "policy.finalize.response": "https://muse.dev/schemas/bridge/v1/policy.schema.json#/$defs/finalizeResponse",
  "cancel.request": "https://muse.dev/schemas/bridge/v1/cancel.schema.json#/$defs/request",
  "cancel.response": "https://muse.dev/schemas/bridge/v1/cancel.schema.json#/$defs/response",
  "status.request": "https://muse.dev/schemas/bridge/v1/status.schema.json#/$defs/request",
  "status.response": "https://muse.dev/schemas/bridge/v1/status.schema.json#/$defs/response"
} as const;

export type MessageKind = keyof typeof MESSAGE_SCHEMA_REFS;

export const isMessageKind = (value: unknown): value is MessageKind =>
  typeof value === "string" && Object.hasOwn(MESSAGE_SCHEMA_REFS, value);

export const isHelloKind = (kind: MessageKind): boolean => kind === "hello.request" || kind === "hello.response";
export const isResponseKind = (kind: MessageKind): boolean => kind.endsWith(".response");
