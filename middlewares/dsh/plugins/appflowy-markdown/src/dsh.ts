import { createMusePlugin } from "@muse/plugin-kit/dsh";
import type { MusePluginRuntimeConfig } from "@muse/plugin-kit";
import type { Context } from "@deepseek-ai/cordis";
import type { ContextContributionEnvelopeV1 } from "@muse/plugin-facets";
import "@muse/context-broker/dsh";
import { appFlowyMarkdownDefinition } from "./index.js";

const CURRENT_VIEW_SCOPE = { refs: { "appflowy.selection": "current" } } as const;

export const createAppFlowyMarkdownPlugin = (
  config: Omit<MusePluginRuntimeConfig, "scopeHint"> = {}
) => createMusePlugin(appFlowyMarkdownDefinition, { ...config, scopeHint: CURRENT_VIEW_SCOPE });

const loaderPlugin = createAppFlowyMarkdownPlugin({
  ...(process.env.MUSE_PLUGIN_DIAGNOSTICS === "1"
    ? { onDiagnostic: diagnostic => process.stderr.write(`[muse-markdown] ${JSON.stringify(diagnostic)}\n`) }
    : {})
});
export const name = loaderPlugin.name;
export const inject = [...loaderPlugin.inject, "museContextBroker"];

const SURFACE_DIGEST = "sha256:780a1eed2737ec2f9e7dd1fe56cd74a3a7803aea0fc23e37837fd33d458c7b92";
const SELECTION_DIGEST = "sha256:39caf711d2a5cc6a452fb9e4b5aa2f0bf2f5c695a1a751c0d06de2e7255946c0";
const VIEWPORT_DIGEST = "sha256:b7d780858ade44ad51997b60293220630b8a15ae411bc38bb665420c8e82c4a8";

const clean = (value: unknown, max: number): string =>
  typeof value === "string" ? value.replace(/[<>\u0000-\u001f]/gu, " ").slice(0, max) : "";

const payload = (envelope: ContextContributionEnvelopeV1): Record<string, unknown> =>
  envelope.payload !== null && typeof envelope.payload === "object" && !Array.isArray(envelope.payload)
    ? envelope.payload as Record<string, unknown>
    : {};

export const apply = async (ctx: Context): Promise<void> => {
  await loaderPlugin.apply(ctx);
  ctx.effect(() => {
    const disposers = [
      ctx.museContextBroker.registerProjection({
        pluginId: "muse.appflowy.markdown",
        contextType: "markdown.surface",
        schemaDigest: SURFACE_DIGEST,
        priority: 100,
        maxTokens: 120,
        render: envelope => {
          const value = payload(envelope);
          return `Current AppFlowy Markdown document: title="${clean(value.title, 256)}", mode=${clean(value.mode, 16)}, readOnly=${value.readOnly === true}.`;
        }
      }),
      ctx.museContextBroker.registerProjection({
        pluginId: "muse.appflowy.markdown",
        contextType: "markdown.selection",
        schemaDigest: SELECTION_DIGEST,
        priority: 90,
        maxTokens: 540,
        render: envelope => {
          const value = payload(envelope);
          const selected = clean(value.selectedText, 2048);
          return value.collapsed === true
            ? "The Markdown caret is collapsed; no text is selected."
            : `The user selected this bounded Markdown text:\n${selected}`;
        }
      }),
      ctx.museContextBroker.registerProjection({
        pluginId: "muse.appflowy.markdown",
        contextType: "markdown.viewport",
        schemaDigest: VIEWPORT_DIGEST,
        priority: 50,
        maxTokens: 160,
        render: envelope => {
          const value = payload(envelope);
          const headings = Array.isArray(value.visibleHeadingRefs)
            ? value.visibleHeadingRefs.slice(0, 32).map(item => clean(item, 128)).join(", ")
            : "";
          return `Visible Markdown block range: ${clean(value.firstVisibleBlockRef, 128)}..${clean(value.lastVisibleBlockRef, 128)}; visible headings: ${headings || "none"}.`;
        }
      })
    ];
    return () => disposers.reverse().forEach(dispose => dispose());
  }, "muse-markdown:context-projections");
};
