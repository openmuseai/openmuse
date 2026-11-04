# `openmuse.nativeConversation@1`

This directory is the compatibility boundary between DSH client plugins, the
OpenMuse Native Gateway, and Flutter. A plugin may describe native conversation
UI, but it cannot ship executable JavaScript, Dart, CSS, URLs, filesystem paths,
or arbitrary layout instructions to Mobile.

The Host validates manifests before negotiation. Mobile advertises exact
versioned slots and components. Negotiation produces one of:

- `native`: every required semantic is represented by supported components;
- `generic`: unsupported optional presentation is replaced by a built-in card
  or omitted according to the declared fallback;
- `web`: at least one required conversation semantic cannot safely degrade, so
  the whole DSH conversation surface uses WebView.

Commands named by a contribution are references only. A Host plugin must
separately register the command, argument JSON Schema, permissions, and Session
scope. A manifest never grants execution authority.

Hard runtime limits supplement the schema: at most 64 contributions per plugin,
256 nodes per contribution, depth 12, 64 children per collection, and bounded
rendered strings/media. Bindings may only start at `session`, `turn`, `message`,
`tool`, or `plugin.state`.
