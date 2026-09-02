import type { Context } from "@deepseek-ai/cordis";
import { envFlagEnabled, injectAtHead } from "./html.js";
import { MOBILE_INPUT_SCRIPT } from "./inject.js";

type WebServer = {
  tapIndex(transform: (html: string) => string): () => void;
};

type HostContext = Context & { webServer: WebServer };

export const name = "@muse/dsh-mobile-input";
export const inject = ["webServer"];

export const apply = (ctx: Context): void => {
  if (!envFlagEnabled("MUSE_DSH_NATIVE_CAPABILITIES_ENABLED")) return;
  const server = (ctx as HostContext).webServer;
  if (server?.tapIndex === undefined) {
    throw new Error("webServer.tapIndex is required for @muse/dsh-mobile-input");
  }
  ctx.effect(
    () => server.tapIndex((html) => injectAtHead(html, MOBILE_INPUT_SCRIPT)),
    "muse.dsh-mobile-input.tapIndex",
  );
};
