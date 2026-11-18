# Shared remote MCP App runtime

A transport and presentation adapter for explicit developer installations on macOS, Android and iOS. The Host does not contain CI project, node, workflow or release logic.

The debug Host reads `apps.json` from its private `remote-mcp-apps` directory. Each record declares `id`, `title`, `version`, `endpoint`, `uiResourceUri`, `uiFile`, `uiSha256`. The endpoint must be HTTPS or development loopback; URI credentials, query tokens and escaping UI paths are rejected. Duplicate App IDs and modified UI bytes are rejected.

Installation supplies `<id>.token` once, with app-private permissions. The Host imports it into its secure store and deletes the bootstrap after verifying durable storage. The first connection binds the installation to the current account hash. Switching accounts revokes the connection and view; another account cannot reuse that credential. Credentials never enter the reviewed HTML, bridge messages, URLs or tool schemas.

The runtime discovers standard MCP tools and verifies the advertised UI resource against the reviewed local digest before rendering. The shared SDK builds the sandboxed MCP Apps document. Each request is checked against discovered tool schemas and `ui.visibility`; only the pinned resource is readable. Desktop and Mobile share this implementation. The optional auditing hook reports method/tool names only.

The Agent router exposes `apps_list`, `apps_open`, `apps_tools_list`, `apps_call_tool`. It forwards each App's own tools without defining their business schemas. Existing active-document authorization in the resource router remains separate.

This developer profile is not a production marketplace protocol or OAuth implementation. Hosts load it only in debug builds. Production Mobile pinned packages and App Store download restrictions are unchanged. A production rollout still needs reviewed registry manifests, secure OAuth/session lifecycle and a reachable authorized backend. USB reverse forwarding is a local validation transport, not remote availability.
