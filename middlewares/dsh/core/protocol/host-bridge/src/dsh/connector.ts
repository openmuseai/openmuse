import { Context, Service } from "@deepseek-ai/cordis";
import type { MuseHostConnector, MuseHostConnectorResult } from "./types.js";

declare module "@deepseek-ai/cordis" {
  interface Context {
    museHostConnector: MuseHostConnector;
  }
}

/** Platform launcher implementation point; the native host owns endpoint/nonce acquisition. */
export abstract class MuseHostConnectorService extends Service implements MuseHostConnector {
  constructor(ctx: Context) {
    super(ctx, "museHostConnector");
  }

  abstract open(signal: AbortSignal): Promise<MuseHostConnectorResult>;
}
