import type { BindingId, RequestId } from "../src/index.js";

declare const requestId: RequestId;
declare const bindingId: BindingId;

const acceptsRequestId = (_value: RequestId): void => {};
acceptsRequestId(requestId);
// @ts-expect-error BindingId must not be accepted as RequestId.
acceptsRequestId(bindingId);
