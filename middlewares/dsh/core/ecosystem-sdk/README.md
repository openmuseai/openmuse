# @muse/ecosystem-sdk

Public v2 surface for Plugin manifest validation, deterministic composition planning, artifact hashing, diagnostics and cross-platform TCK.

```bash
pnpm build
node bin/muse-sdk.mjs init org.example.plugin
node bin/muse-sdk.mjs validate ../../plugins/appflowy-database/muse.plugin.json
node bin/muse-sdk.mjs test ../../plugins/appflowy-database/muse.plugin.json
```

Generated plugins import only package exports. Domain contracts remain plugin-owned; Bridge transports opaque operation IDs and payloads.
