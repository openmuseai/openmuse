# `@openmuse/dsh-workspace-runtime`

DSH 0.1.7 remote Provider group for one OpenMuse Workspace Sandbox execution
world. It replaces `ctx.fs`, `ctx.subprocess`, and `ctx.sandbox` through Cordis
rows and does not patch the Agent Loop or model-facing tools.

The package is transport-neutral. A Host connector supplies a short-lived,
audience-bound `ControlAttachment` and a transport that maps control RPC,
bounded calls, streaming text, process streams, PTY streams, cancellation, and watch events to the Workspace
Sandbox service. Target keys and attachment tokens are opaque. Host paths are
never accepted or returned; `processPath` values are paths inside the remote
execution world.

Each exported Cordis Provider accepts the same full `{ transport, attachment,
audience, now? }` config, so the three rows in `cordis.patch.yml` can be loaded
independently by DSH. The package root also exposes `apply()` for programmatic
composition and shares one control object across the three services.

`RemoteSandboxProvider` never returns unconfined argv. It asks the remote
control plane for a single-use launch token and returns an OpenMuse runner argv.
The remote subprocess endpoint consumes that token in the same runtime and must
fail closed unless the requested policy has full substrate enforcement.
