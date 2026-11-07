# Muse asset ports (internal preview)

Pure Dart, content-independent identity/integrity reference and repository/cache ports. No Flutter, CRDT, document tree, file paths, auth token, endpoint or account implementation enters this package. A reference grants no access; repositories resolve it against current authorization. Domain plugins decide allowed media, dimensions and size budgets.

Caches must be scoped by account/workspace/resource capabilities. Previously downloaded bytes are a local copy, never proof of current remote permission. A `FORBIDDEN` or `NOT_FOUND` response must not fall back to the cached copy. The smart-document adapter implements explicit offline fallback for network unavailability only.

Source deletion and blob garbage collection are separate concerns. Removing an embedding must not delete the source resource, another document's attachment, or a retained historical version.
