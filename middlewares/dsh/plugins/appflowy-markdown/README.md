# AppFlowy Markdown Plugin

This Cordis plugin contributes Tools for the Host-selected AppFlowy document:

- `muse_appflowy_read_current_markdown`
- `muse_appflowy_propose_markdown_edit`
- `muse_appflowy_apply_markdown_edit`

The Tools never accept a document, View, workspace, actor, or grant identifier from the model. Writes are `propose` then policy-gated `apply`; the Host keeps View identity and compare-and-set state.
