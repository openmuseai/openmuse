# AppFlowy object transport (internal preview)

Pure Dart package, no Flutter/editor or document schema dependency. Reuses the existing Cloud collab read/create and full-sync protocol. Each content domain supplies its own `CollaborationBinding`, original `collabType`, encoder, validator and merge engine. Objects may be database roots, independent rows or document roots; resource-file identity is not imposed here.

`exchange` means the existing server accepted an exchange, **not durable persistence acknowledgement**. WebSocket subscriptions, awareness, ordered reconnect, revocation and durable receipts are separate open work. Credentials and endpoint are injected capabilities and never portable content.

The smart-document provider is a small domain adapter with `collabType: 0`. This extraction does not enable database support or certify other content domains automatically.
