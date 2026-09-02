import type { Context } from "@deepseek-ai/cordis";
import { envFlagEnabled, injectAtHead } from "./html.js";
import { MOBILE_SURFACE_SNIPPET } from "./inject.js";

type WebServer = {
  tapIndex(transform: (html: string) => string): () => void;
};

type HostContext = Context & { webServer: WebServer };

export const name = "@muse/dsh-mobile-surface";
export const inject = ["webServer"];

export const apply = (ctx: Context): void => {
  if (!envFlagEnabled("MUSE_DSH_MOBILE_SURFACE_ENABLED")) return;
  const server = (ctx as HostContext).webServer;
  if (server?.tapIndex === undefined) {
    throw new Error("webServer.tapIndex is required for @muse/dsh-mobile-surface");
  }
  ctx.effect(
    () => server.tapIndex((html) => injectAtHead(html, MOBILE_SURFACE_SNIPPET)),
    "muse.dsh-mobile-surface.tapIndex",
  );
};
