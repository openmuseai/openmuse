import { createMusePlugin } from "@muse/plugin-kit/dsh";
import type { MusePluginRuntimeConfig } from "@muse/plugin-kit";
import type { Context } from "@deepseek-ai/cordis";
import { appFlowyViewReferenceDefinition } from "./index.js";

const CURRENT_VIEW_SCOPE = {
  refs: { "appflowy.selection": "current" }
} as const;

export const createAppFlowyViewReferencePlugin = (
  config: Omit<MusePluginRuntimeConfig, "scopeHint"> = {}
) => createMusePlugin(appFlowyViewReferenceDefinition, {
  ...config,
  scopeHint: CURRENT_VIEW_SCOPE
});

const loaderPlugin = createAppFlowyViewReferencePlugin();
export const name = loaderPlugin.name;
export const inject = loaderPlugin.inject;
export const apply = (ctx: Context): Promise<void> => loaderPlugin.apply(ctx);
