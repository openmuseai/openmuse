import { ContextFacetInbox, type ContextContributionEnvelopeV1 } from "@muse/plugin-facets";
import { markdownPresentationContexts } from "@muse/plugin-appflowy-markdown";

// Existing v1 contract identities, shared by Flutter and React adapters.
export const PRESENTATION_CONTRACTS = [
  ["muse.appflowy.workspace", "workspace.focus", "sha256:4c3a6bf1cd8249cc96f6aac63ad78b9be04172634f0313ce3ba52163f66043b7"],
  ["muse.appflowy.workspace", "workspace.tree.ui", "sha256:c0381f40eaaa753e7bb28e79e74c8e44573864aa89928a3ba246a524389edbda"],
  ["muse.appflowy.workspace", "workspace.catalog", "sha256:2538c0adb882241b625b48c0013e4a0dd248d8b391cac22ebdc14e154c50548e"],
  ["muse.appflowy.markdown", "markdown.surface", "sha256:780a1eed2737ec2f9e7dd1fe56cd74a3a7803aea0fc23e37837fd33d458c7b92"],
  ["muse.appflowy.markdown", "markdown.selection", "sha256:39caf711d2a5cc6a452fb9e4b5aa2f0bf2f5c695a1a751c0d06de2e7255946c0"],
  ["muse.appflowy.markdown", "markdown.viewport", "sha256:b7d780858ade44ad51997b60293220630b8a15ae411bc38bb665420c8e82c4a8"],
  ["muse.appflowy.markdown", "markdown.snapshot", "sha256:e4ea242dbdbc52565fb474cd042af4da590ae88a9d4715e9694ce031eaee03b3"],
] as const;

/** Composition root: transport decoders pass unchanged contract envelopes here. */
export function createPresentationFacetInbox(deps: {
  contribute(envelope: ContextContributionEnvelopeV1): void;
  rememberFocus(envelope: ContextContributionEnvelopeV1): void;
  pinSurface?(ref: string): void;
}): ContextFacetInbox {
  const inbox = new ContextFacetInbox();
  for (const [pluginId, contextType, schemaDigest] of PRESENTATION_CONTRACTS) {
    inbox.register({ pluginId, contextType, schemaDigest, accept: envelope => {
      const markdown = markdownPresentationContexts.find(contract => contract.contextType === contextType);
      if (markdown !== undefined) {
        if (markdown.schemaDigest !== schemaDigest) throw new Error("SCHEMA_DIGEST_MISMATCH");
        markdown.validate(envelope.payload);
      }
      deps.contribute(envelope);
      deps.rememberFocus(envelope);
      if (contextType === "workspace.focus") deps.pinSurface?.(envelope.surfaceInstanceRef);
    } });
  }
  return inbox;
}
