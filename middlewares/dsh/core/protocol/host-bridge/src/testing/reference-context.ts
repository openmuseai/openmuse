import { parseOpaqueId, type BindingId, type HostGeneration, type HostSessionId } from "../contract/brands.js";
import type { BindingSnapshot, BoundOperation } from "../contract/types.js";

export interface ReferenceBindingOptions {
  readonly now: number;
  readonly expiresInMs?: number;
  readonly revoked?: boolean;
  readonly bindingId?: string;
  readonly generation?: string;
  readonly scopeRef?: string;
  readonly operations: readonly BoundOperation[];
}

export const referenceHostSessionId = parseOpaqueId("host-session.reference", "HostSessionId") as HostSessionId;
export const referenceHostGeneration = parseOpaqueId("host-generation.reference", "HostGeneration") as HostGeneration;

export const createReferenceBinding = (options: ReferenceBindingOptions): BindingSnapshot => {
  const operations = new Map(options.operations.map((operation) => [operation.operationId, operation]));
  return Object.freeze({
    bindingId: parseOpaqueId(options.bindingId ?? "binding.reference", "BindingId") as BindingId,
    descriptorRevision: "revision.reference",
    hostGeneration: parseOpaqueId(options.generation ?? referenceHostGeneration, "HostGeneration") as HostGeneration,
    scopeRef: options.scopeRef ?? "scope.reference",
    expiresAt: options.now + (options.expiresInMs ?? 60_000),
    revoked: options.revoked ?? false,
    operations
  });
};
