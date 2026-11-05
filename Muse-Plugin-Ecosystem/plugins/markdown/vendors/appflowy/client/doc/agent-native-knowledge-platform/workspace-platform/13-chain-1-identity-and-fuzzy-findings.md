# Chain ① — live findings: the identity defect, the CDP harness, and the fuzzy-resolution requirement

Status: **open work**, recorded 2026-09-21 while implementing the workspace-platform/12 plan.
Owner-visible symptom, reproduction method, root cause found so far, and the follow-up requirement.
Companion documents: `09-dsh-binding-product-prd.md`, `10-dsh-binding-architecture-design.md`,
`11-dsh-binding-development-plan-and-test-matrix.md`, `12-host-bridge-e2e-verification-plan.md`.

## 1. The symptom, captured from the real UI

For the workspace `D:\agentic\src\openmuse-io\vendors\helix`, clicking a file reference in the DSH
conversation stream (`docs\architecture\08-view-support.md`) renders, inside the panel:

```
无法打开文件
path open failed: path open failed: HOST_LOCATOR_FAILED:
  DSH invocation source must identify a session or agent
```

Points that matter:

- The click was a **genuine, session-bound UI click** (the panel's own handler), not a synthetic
  HTTP call. So the identity is genuinely missing on the wire — this is not an artifact of how the
  request was issued.
- The failure happens at **Host admission**, *before* any path resolution. Path completeness is
  therefore **not** the cause of *this* message.
- `path open failed:` is duplicated in the rendered text — likely a second, smaller defect
  (an error already wrapped once gets wrapped again).

## 2. How to reproduce it from now on (CDP harness)

Synthetic OS input cannot reach the WebView2 panel: mouse and keyboard injection both proven
ineffective (a click on a file link did nothing; a typed character never appeared in the composer).
The Flutter surface is the opposite: synthetic **mouse** works, synthetic **keyboard** does not.

Workaround, proven on 2026-09-21: launch the app with WebView2 remote debugging enabled, then drive
the panel through Chromium's own input pipeline and read its DOM.

```powershell
$env:WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS = '--remote-debugging-port=9222'
Start-Process 'D:\install\OpenMuse\OpenMuse.exe' -WorkingDirectory 'D:\install\OpenMuse'
```

```js
// node (>=22 has global WebSocket): list targets, then evaluate in the panel
const list = await (await fetch('http://127.0.0.1:9222/json/list')).json();
const page = list.find(t => t.type === 'page');
const ws = new WebSocket(page.webSocketDebuggerUrl);
// Runtime.evaluate { returnByValue: true } to click and to read rendered text
//   Array.from(document.querySelectorAll('button')).find(e => /08-view-support/.test(e.textContent)).click()
```

This is a **launch-time environment variable only** — no product change. Useful because it makes
both chains verifiable on the real UI *and* returns the rendered failure text instead of a guess
from a screenshot.

## 3. Requirement from the project owner (must be implemented)

> For the workspace above, opening `docs\architecture\08-view-support.md` from the conversation
> reports "not found" because that path is incomplete. The **Host** therefore needs a middleware
> that can resolve a **partial path — even just a file name — by fuzzy matching.**

Design constraints agreed so far:

- Resolution stays on the **Host**; DSH forwards the reference as a mount-relative locator and must
  not gain a device-path fallback.
- Exact match is tried first and must keep today's behaviour byte-for-byte; fuzzy matching is a
  strict fallback.
- Search inside the granted Mount only, with explicit bounds (depth, entries scanned, candidates)
  and symlink containment checked as strictly as the existing resolver does.
- Ambiguity **fails closed** with a distinct, readable reason code (candidate count included);
  ranking must be deterministic so the same input always resolves to the same file.
- Because the minted `resourceRef` is derived from the relative path, the fuzzy hit must be
  resolved **before** minting, or the dispatch must carry the resolved exact relative path —
  otherwise the Flutter surface still answers `RESOURCE_NOT_FOUND`.

## 4. Open question that decides whether suffix matching is even needed for that path

What is the **root of the `helix` Mount**?

- If it is `D:\agentic\src\openmuse-io\vendors\helix`, then `docs\architecture\08-view-support.md`
  is already complete, and the "incomplete path" case is the bare-filename reference
  (`08-view-support.md`), which the panel also renders as a button.
- If it is a higher directory (e.g. `D:\agentic\src\openmuse-io`), the path is missing the
  `vendors/helix/` prefix and suffix matching is exactly what is required for it.

The mount for that workspace was observed as
`mount:0d712bedcae03f633e14f16b232bd467d2e37cb5722905ee7ec713ad473912bf` (`displayName: helix`,
binding revision 3). **Confirm the root before finalising the matching rule.**

## 5. Order of work

1. Fix the DSH→Host invocation **identity** propagation (`HOST_LOCATOR_FAILED`) — it blocks chain ①
   entirely and hides everything else.
2. Then the Host-side **fuzzy/partial path resolution** above, covering the bare-filename case.
3. Re-verify chain ① through the **real UI click** (CDP harness, §2), asserting a Host tab opens.
4. Chain ② (Host copy → DSH paste showing a reference block with metaData) still needs its own
   real-UI pass: the Host editor's copy can be triggered with a mouse drag plus the selection
   toolbar's copy action (synthetic mouse works on Flutter), and the paste can be delivered into the
   panel through CDP.

---

# Host-side partial-path resolution (items 2–5) — implemented, awaiting the rebuild

Author: the Host/Rust workstream. The DSH identity defect (§1) and `open-intent.ts` / `opener.ts` /
`panel-routes.ts` / `locator.ts` are **not** touched here.

## Answer to §4 (the open question)

The `helix` Mount root **is** `D:\agentic\src\openmuse-io\vendors\helix`. Verified from the sidecar
log (revisions 3/4/6):

```
[resolve.legacy] { mountRef: 'mount:0d712bed…', displayName: 'helix',
                   target: 'D:\\agentic\\src\\openmuse-io\\vendors\\helix\\docs\\architecture\\08-view-support.md',
                   bindingRevision: 6 }
```

So for that citation the locator path is already complete, and the case fuzzy matching exists for is a
**genuinely partial** reference — the bare file name `08-view-support.md`, or
`architecture/08-view-support.md`.

## Where a partial path comes from, and where it is resolved

`@muse/plugin-appflowy-workspace/resolve.ts` resolves a legacy `{ path, cwd }` **lexically** when the
entry does not exist (`canonicalLegacyPath`), so a relative citation becomes
`<active-mount>/08-view-support.md`, passes containment, and `mountRelativePath` yields the locator
`{ mountRef, relativePath: "08-view-support.md" }`. DSH therefore already forwards the partial
reference as a **mount-relative locator** (requirement 4) with no device-path fallback — no DSH change
was needed, and none was made.

The Host cannot open that path (it does not exist), so the resolution point is **presentation time**,
inside the Rust provider:

1. the minted path, exactly, when it is a file (`Exact`);
2. otherwise a segment-suffix match, then a file-name match, **inside the granted Mount**;
3. otherwise the minted path is dispatched unchanged and the surface answers `RESOURCE_NOT_FOUND`.

The **ref stays opaque and is still minted from the partial path**, so the
`muse.mount-relative-path/v1` anchor keeps re-deriving the same ref after locator-table expiry
(`LOCATOR_TTL_MS = 30 min`), and the deterministic search then yields the same file. Only the
**dispatch** carries the resolved exact path.

## Ranking, bounds, ambiguity

- Rank key `(tier, extra_segments)`: suffix tier beats name tier; the shallowest wins; the sample order
  is lexicographic for stability but is **never** used as a tie-break.
- Bounds: `max_depth: 12`, `max_entries: 20_000` per pass, `max_candidates: 32`, heavy directories
  (`.git`, `node_modules`, `target`, `build`, `dist`, `vendor`, …) skipped until a second pass.
- Containment: canonical-root comparison, per-segment validation (`..`, `.`, absolute, drive letter,
  UNC, NUL, empty segments refused) and symlinks that resolve outside the Mount are rejected.
- Ambiguity fails closed: `result: "failed"`, `errorCode: "RESOURCE_AMBIGUOUS_<count>"`,
  `retryable: false`, **no** surface dispatch. `_32` means "at least 32" (candidate bound hit).
  The v2 failure receipt carries no `resourceRef`, so the count is the only channel — which is why it
  is in the code.

## Files

- `frontend/rust-lib/flowy-core/src/muse_partial_path.rs` (new): the search, its limits and 16 tests.
- `frontend/rust-lib/flowy-core/src/muse_presentation.rs`: `MuseMountRootCatalog` seam
  (`install_/clear_/muse_mount_root_catalog_available`), `ResolveOutcome` (`Target` / `Unresolved` /
  `Ambiguous`), the dispatch-time resolution and the `RESOURCE_AMBIGUOUS_<n>` receipt; +5 tests.
- `frontend/rust-lib/dart-ffi/src/muse_presentation_ffi.rs`:
  `muse_presentation_publish_mount_roots` (+1 test).
- `frontend/appflowy_flutter/lib/plugins/resource_surface/muse_presentation_channel.dart`:
  `publishMountRoots` on the transport interface and the host.
- `frontend/appflowy_flutter/lib/plugins/dsh_agent/dsh_workspace_bridge.dart`: pushes the local Mount
  directories on every publish (`_publish`), empty catalog included.
- Dart tests: `test/plugins/resource_surface/muse_presentation_channel_test.dart`,
  `test/workspace_platform/dsh_binding_bridge_test.dart`.

## Verification (executed)

```
cargo test -p flowy-core --lib muse   -> test result: ok. 67 passed; 0 failed   (was 46)
cargo test -p dart-ffi --lib muse     -> test result: ok.  6 passed; 0 failed
flutter test test/plugins/resource_surface/muse_presentation_channel_test.dart \
             test/workspace_platform/dsh_binding_bridge_test.dart -> All tests passed! (+18)
flutter analyze <the four touched Dart files> -> No issues found!
```

Not yet verifiable without the rebuild: the real click. The running app serves the prebuilt closure
(`D:\install\OpenMuse\muse\closure`), so the new Host code is not in it; no app build was performed.

