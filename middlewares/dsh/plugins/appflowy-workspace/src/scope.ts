import type { JsonValue } from "@muse/host-bridge";

export type WorkspaceScopeResult =
  | { readonly ok: true }
  | { readonly ok: false; readonly code: "SCOPE_MISMATCH"; readonly message: string };

const workspaceIdOf = (input: JsonValue): string | undefined => {
  if (input === null || typeof input !== "object" || Array.isArray(input)) return undefined;
  const value = (input as { workspaceId?: unknown }).workspaceId;
  return typeof value === "string" && value.trim().length > 0 ? value.trim() : undefined;
};

/**
 * Invoke workspace must match the W4-bound AppFlowy workspace when both are present.
 */
export const assertWorkspaceScope = (
  boundWorkspaceId: string | undefined,
  input: JsonValue
): WorkspaceScopeResult => {
  const requested = workspaceIdOf(input);
  if (boundWorkspaceId === undefined || requested === undefined) return { ok: true };
  if (requested === boundWorkspaceId) return { ok: true };
  return {
    ok: false,
    code: "SCOPE_MISMATCH",
    message: "SCOPE_MISMATCH: invoke workspace does not match the bound AppFlowy workspace"
  };
};
