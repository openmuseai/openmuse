# Stubs crate policy

`libs/core-libs/stubs` may only host **platform** types:

| Allowed roots | Notes |
|---------------|--------|
| `java`, `javax` | JDK |
| `android` | Android Framework / SDK |
| `kotlin`, `com`, `org` | Approved third-party / SDK; prefer real crates when they exist |

**Not allowed:** project types (`cn.wps.*`, anything with real Java under `koffice/**`), flat stubs (`package ;` / `package OuterClass;`), or new top-level modules under `stubs/src` outside the whitelist.

## Cleanup status (inventory)

- **Done (batch 1):** Removed all project top-level Rust stubs (`PLC*`, `Interval*`, `TextDocument`, …) and the entire `stubs/src/cn` tree.
- **Done:** Retargeted `CloneNotSupportedException` → `stubs::java::lang::CloneNotSupportedException`.
- **Done:** `INotifyTextRope` / `IPapxTable` / `PLCCommentRef` / `DeleteRangeReason` / `ThaiCorrectionTool` imports → real crates.
- **Remaining debt:** many `use stubs::cn::…` in `ss-core` (and similar) still reference **missing** modules — migrate to real crates then shrink `stubs-legacy-use-roots.allowlist`.

## Freeze files (shrink-only)

| File | Purpose |
|------|---------|
| `stubs-legacy-toplevel.allowlist` | Existing non-whitelist top-level Rust modules (now empty) |
| `stubs-legacy-use-roots.allowlist` | Existing `use stubs::<root>` roots outside whitelist |
| `stubs-legacy-java-paths.allowlist` | Existing non-platform / illegal-package Java stub paths |

Do **not** add entries. Remove entries when imports are migrated to real crates and files are deleted.

## Check

```bash
node plugin_java2Rust/scripts/check-stubs-policy.mjs
# or
npm run check:stubs-policy --prefix plugin_java2Rust
```
