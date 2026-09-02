import { createMusePlugin } from "@muse/plugin-kit/dsh";
import type { MusePluginRuntimeConfig } from "@muse/plugin-kit";
import type { Context } from "@deepseek-ai/cordis";
import { appFlowyViewRenameDefinition } from "./index.js";

const CURRENT_VIEW_SCOPE = { refs: { "appflowy.selection": "current" } } as const;

export const createAppFlowyViewRenamePlugin = (
  config: Omit<MusePluginRuntimeConfig, "scopeHint"> = {}
) => createMusePlugin(appFlowyViewRenameDefinition, { ...config, scopeHint: CURRENT_VIEW_SCOPE });

const loaderPlugin = createAppFlowyViewRenamePlugin({
  ...(process.env.MUSE_PLUGIN_DIAGNOSTICS === "1"
    ? { onDiagnostic: diagnostic => process.stderr.write(`[muse-view-rename] ${JSON.stringify(diagnostic)}\n`) }
    : {})
});
export const name = loaderPlugin.name;
export const inject = loaderPlugin.inject;
export const apply = (ctx: Context): Promise<void> => loaderPlugin.apply(ctx);
