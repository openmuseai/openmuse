import bindSchema from "../../schemas/v1/bind.schema.json" with { type: "json" };
import cancelSchema from "../../schemas/v1/cancel.schema.json" with { type: "json" };
import commonSchema from "../../schemas/v1/common.schema.json" with { type: "json" };
import discoverSchema from "../../schemas/v1/discover.schema.json" with { type: "json" };
import envelopeSchema from "../../schemas/v1/envelope.schema.json" with { type: "json" };
import errorSchema from "../../schemas/v1/error.schema.json" with { type: "json" };
import helloSchema from "../../schemas/v1/hello.schema.json" with { type: "json" };
import invokeSchema from "../../schemas/v1/invoke.schema.json" with { type: "json" };
import policySchema from "../../schemas/v1/policy.schema.json" with { type: "json" };
import statusSchema from "../../schemas/v1/status.schema.json" with { type: "json" };
import subscribeSchema from "../../schemas/v1/subscribe.schema.json" with { type: "json" };

export const PROTOCOL_SCHEMA_DOCUMENTS: readonly object[] = Object.freeze([
  commonSchema,
  errorSchema,
  envelopeSchema,
  helloSchema,
  discoverSchema,
  bindSchema,
  invokeSchema,
  subscribeSchema,
  policySchema,
  cancelSchema,
  statusSchema
]);

export const ENVELOPE_SCHEMA_ID = "https://muse.dev/schemas/bridge/v1/envelope.schema.json";
