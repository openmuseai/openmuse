import {
  DesktopMuseHostTransport,
  HARD_LIMITS,
  InProcessMuseHostTransport,
  type RuntimeProof
} from "@muse/host-bridge";
import { readFile, stat } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { MuseHostConnectorService, setMuseApprovalProofRequester } from "@muse/host-bridge/dsh";
import { createCloudMarkdownProvider, e2eMarkdownProvider } from "@muse/plugin-appflowy-markdown/host";
import { getLastWorkspaceHint } from "@muse/plugin-appflowy-workspace";
import { createCloudWorkspaceProvider, e2eWorkspaceProvider } from "@muse/plugin-appflowy-workspace/host";
import { createHostHmacApprovalRequester } from "./approval.js";
import {
  COMPOSITION_HOST_GENERATION,
  InProcessCompositionHandler
} from "./composition-host.js";
import { getLastDeviceAuth, getLastDocumentFocus } from "./session.js";

const inProcessLimits = {
  maxPayloadBytes: HARD_LIMITS.maxMessageBytes,
  maxResponseBytes: HARD_LIMITS.maxMessageBytes,
  maxConcurrentRequests: 32,
  maxConcurrentStreams: 4,
  maxStreamFrameBytes: HARD_LIMITS.maxEventPayloadBytes,
  maxConnections: 4,
  maxConnectionAgeMs: 300_000,
  maxDeadlineHorizonMs: 300_000
} as const;

const invokeSlots = () => ({
  boundWorkspaceId: () => getLastWorkspaceHint()?.appflowyWorkspaceId,
  documentFocus: () => getLastDocumentFocus(),
  deviceAuth: () => getLastDeviceAuth()
});

const cloudBaseUrl = (env: NodeJS.ProcessEnv = process.env): string | undefined => {
  const raw = env.MUSE_DOCUMENT_CLOUD_URL?.trim();
  return raw ? raw.replace(/\/$/, "") : undefined;
};

const openInProcess = (
  cloud?: { readonly baseUrl: string; readonly accessToken?: string }
) => {
  const providers = cloud === undefined
    ? [e2eMarkdownProvider, e2eWorkspaceProvider]
    : [createCloudMarkdownProvider(), createCloudWorkspaceProvider()];
  const transport = new InProcessMuseHostTransport(
    new InProcessCompositionHandler(providers, {
      ...(cloud === undefined ? {} : { cloudBaseUrl: cloud.baseUrl }),
      ...(cloud?.accessToken === undefined ? {} : { accessToken: cloud.accessToken }),
      slots: invokeSlots()
    }),
    inProcessLimits,
    COMPOSITION_HOST_GENERATION
  );
  return transport;
};

/** E2E connector: InProcess Host assembled from markdown + workspace providers. */
export class InProcessAppFlowyConnector extends MuseHostConnectorService {
  open(): Promise<{ transport: InProcessMuseHostTransport; proof: RuntimeProof }> {
    const transport = openInProcess();
    return Promise.resolve({
      transport,
      proof: { runtimeInstanceId: "runtime.dsh-appflowy", nonce: transport.launch.nonce }
    });
  }
}

interface NativeLaunch {
  readonly endpoint: string;
  readonly nonce: string;
  readonly hostGeneration: string;
  readonly runtimeInstanceId: string;
}

const launchPath = (): string => process.env.MUSE_APPFLOWY_LAUNCH_FILE
  ?? join(tmpdir(), `appflowy-muse-host-${process.getuid?.() ?? 0}.json`);

const validOpaque = (value: unknown): value is string =>
  typeof value === "string" && /^[A-Za-z0-9._~-]{1,128}$/u.test(value);

const loadNativeLaunch = async (): Promise<NativeLaunch> => {
  const path = launchPath();
  const metadata = await stat(path);
  if (!metadata.isFile() || (process.getuid !== undefined && metadata.uid !== process.getuid())) {
    throw new Error("AppFlowy Muse launch descriptor is not owned by the current user");
  }
  if ((metadata.mode & 0o077) !== 0) throw new Error("AppFlowy Muse launch descriptor is not private");
  const value = JSON.parse(await readFile(path, "utf8")) as Partial<NativeLaunch>;
  if (
    typeof value.endpoint !== "string" || !value.endpoint.startsWith("/")
    || !validOpaque(value.hostGeneration) || !validOpaque(value.runtimeInstanceId)
    || typeof value.nonce !== "string" || value.nonce.length < 32
  ) throw new Error("AppFlowy Muse launch descriptor is invalid");
  return value as NativeLaunch;
};

/** Production connector: UDS on Desktop; InProcess Cloud Host assembled from Plugin providers. */
export default class AppFlowyConnector extends MuseHostConnectorService {
  constructor(ctx: ConstructorParameters<typeof MuseHostConnectorService>[0]) {
    super(ctx);
    setMuseApprovalProofRequester(ctx, createHostHmacApprovalRequester());
  }

  async open(): Promise<{ transport: DesktopMuseHostTransport | InProcessMuseHostTransport; proof: RuntimeProof }> {
    const cloudUrl = cloudBaseUrl();
    if (cloudUrl !== undefined) {
      const transport = openInProcess({
        baseUrl: cloudUrl,
        ...(process.env.MUSE_DOCUMENT_CLOUD_TOKEN === undefined
          ? {}
          : { accessToken: process.env.MUSE_DOCUMENT_CLOUD_TOKEN })
      });
      return {
        transport,
        proof: { runtimeInstanceId: "runtime.dsh-appflowy-cloud", nonce: transport.launch.nonce }
      };
    }
    const launch = await loadNativeLaunch();
    return {
      transport: new DesktopMuseHostTransport({
        endpoint: launch.endpoint,
        nonce: launch.nonce,
        hostGeneration: launch.hostGeneration
      }),
      proof: { runtimeInstanceId: launch.runtimeInstanceId, nonce: launch.nonce }
    };
  }
}
