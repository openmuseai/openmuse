import { createHash } from "node:crypto";
import { isOpaqueId, type ClientCorrelation } from "../index.js";
import type { DshInvocationSource } from "./types.js";

const projectRef = (namespace: string, value: string): string => {
  if (isOpaqueId(value)) return value;
  const digest = createHash("sha256").update(namespace).update("\0").update(value).digest("hex");
  return `${namespace}.${digest.slice(0, 32)}`;
};

const positiveBoundary = (value: number | undefined, name: string): string | undefined => {
  if (value === undefined) return undefined;
  if (!Number.isSafeInteger(value) || value < 0) {
    throw new TypeError(`${name} must be a non-negative safe integer`);
  }
  return `${name}.${value}`;
};

/** Map only public DSH identities; no Agent/Fiber/UI object crosses the Bridge. */
export const clientCorrelationFromDsh = (source: DshInvocationSource): ClientCorrelation => {
  const session = source.sessionId ?? source.agent?.id;
  if (session === undefined || session.length === 0) {
    throw new TypeError("DSH invocation source must identify a session or agent");
  }
  const turnRef = positiveBoundary(source.turn, "turn");
  const stepRef = positiveBoundary(source.step, "step");
  return Object.freeze({
    sessionRef: projectRef("session", session),
    ...(turnRef === undefined ? {} : { turnRef }),
    ...(stepRef === undefined ? {} : { stepRef }),
    ...(source.toolCallId === undefined
      ? {}
      : { toolCallRef: projectRef("toolcall", source.toolCallId) })
  });
};
