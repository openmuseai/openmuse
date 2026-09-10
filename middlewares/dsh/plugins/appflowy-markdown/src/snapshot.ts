import { createHash } from "node:crypto";
import type { JsonValue } from "@muse/host-bridge";

export const MARKDOWN_SNAPSHOT_CONTEXT_TYPE = "markdown.snapshot";
export const MARKDOWN_SNAPSHOT_DIGEST =
  "sha256:e4ea242dbdbc52565fb474cd042af4da590ae88a9d4715e9694ce031eaee03b3";
export const MARKDOWN_SNAPSHOT_MAX_BYTES = 32 * 1024;

const FORBIDDEN = /access_token|refresh_token|api[_-]?key/i;

export interface MarkdownSnapshot {
  readonly viewId: string;
  readonly workspaceId?: string;
  readonly text: string;
  readonly truncated: boolean;
  readonly byteLength: number;
}

let lastSnapshot: MarkdownSnapshot | undefined;

export const getLastMarkdownSnapshot = (): MarkdownSnapshot | undefined => lastSnapshot;

export const resetLastMarkdownSnapshot = (): void => {
  lastSnapshot = undefined;
};

const boundText = (raw: string): { text: string; truncated: boolean; byteLength: number } => {
  const buf = Buffer.from(raw, "utf8");
  if (buf.byteLength <= MARKDOWN_SNAPSHOT_MAX_BYTES) {
    return { text: raw, truncated: false, byteLength: buf.byteLength };
  }
  const sliced = buf.subarray(0, MARKDOWN_SNAPSHOT_MAX_BYTES).toString("utf8");
  return { text: sliced, truncated: true, byteLength: Buffer.byteLength(sliced) };
};

export const parseMarkdownSnapshotPayload = (payload: unknown): MarkdownSnapshot | undefined => {
  if (payload === null || typeof payload !== "object" || Array.isArray(payload)) return undefined;
  const rec = payload as Record<string, unknown>;
  const viewId = typeof rec.viewId === "string" ? rec.viewId.trim() : "";
  if (viewId.length === 0 || viewId.length > 128) return undefined;
  const workspaceId = typeof rec.workspaceId === "string" ? rec.workspaceId.trim() : "";
  const raw = typeof rec.text === "string" ? rec.text : "";
  if (raw.length === 0) return undefined;
  const bounded = boundText(raw);
  const snapshot: MarkdownSnapshot = {
    viewId,
    truncated: rec.truncated === true || bounded.truncated,
    byteLength: bounded.byteLength,
    text: bounded.text,
    ...(workspaceId.length > 0 ? { workspaceId } : {})
  };
  if (FORBIDDEN.test(JSON.stringify(snapshot))) return undefined;
  return snapshot;
};

export const rememberMarkdownSnapshot = (snapshot: MarkdownSnapshot): void => {
  lastSnapshot = snapshot;
};

export const rememberMarkdownSnapshotFromEnvelope = (envelope: unknown): boolean => {
  if (envelope === null || typeof envelope !== "object" || Array.isArray(envelope)) return false;
  const rec = envelope as Record<string, unknown>;
  if (rec.contextType !== MARKDOWN_SNAPSHOT_CONTEXT_TYPE) return false;
  const parsed = parseMarkdownSnapshotPayload(rec.payload);
  if (parsed === undefined) return false;
  lastSnapshot = parsed;
  return true;
};

export const projectSnapshotDocument = (viewId: string): JsonValue | undefined => {
  const snapshot = lastSnapshot;
  if (snapshot === undefined || snapshot.viewId !== viewId) return undefined;
  const revision = `sha256:${createHash("sha256").update(snapshot.text).digest("hex")}`;
  return {
    protocol: "muse.document/snapshot/v2",
    resourceRef: snapshot.viewId,
    revision,
    content: {
      mediaType: "text/markdown",
      text: snapshot.text,
      truncated: snapshot.truncated,
      byteLength: snapshot.byteLength
    }
  };
};

export const cloudDocumentUnwired = (result: { readonly code: string; readonly message: string }): boolean =>
  result.code === "UNAVAILABLE"
  && (result.message.includes("CLOUD_COLLAB_ADAPTER_NOT_WIRED")
    || result.message.includes("NOT_FOUND")
    || result.message.includes("unreachable")
    || result.message.includes("invalid document adapter"));
