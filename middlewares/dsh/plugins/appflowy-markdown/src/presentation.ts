import { compileFacetPayload } from "@muse/plugin-facets";
import surface from "../schemas/v1/surface-context.schema.json" with { type: "json" };
import selection from "../schemas/v1/selection-context.schema.json" with { type: "json" };
import viewport from "../schemas/v1/viewport-context.schema.json" with { type: "json" };

export const markdownPresentationContexts = [
  { contextType: "markdown.surface", ...compileFacetPayload(surface) },
  { contextType: "markdown.selection", ...compileFacetPayload(selection) },
  { contextType: "markdown.viewport", ...compileFacetPayload(viewport) },
] as const;
