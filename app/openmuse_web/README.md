# OpenMuse Web: Desktop shared workbench slice

This Flutter entry uses the same workbench layout, canvas, sidebar, file rows, editor tabs, pane menu, welcome screen, theme and Office widgets as Desktop/Mobile. The middle editor remains present by default. Web uses the shared GoTrue login UI/controller with a browser HTTP adapter. After login, the full workbench opens even when no Desktop is online; discovery, connection errors and retry remain inside the Workspace pane. The DSH pane mounts the pinned DSH Web client into a regular DOM node in the same page, without an iframe. An authenticated production Web Edge and full DSH conversation/approval validation are still pending.

Build the DSH pane module and Flutter route:

```sh
cd ../../web/dsh-pane
npm ci
npm run build
cd ../../app/openmuse_web
flutter pub get
# Production host is https://app.openmuseai.com/ . The local edge still uses /app/.
flutter build web --release --base-href /
# flutter build web --release --base-href /app/
```

For local integration, start a DSH Web server on loopback, then from the repository root run:

```sh
OPENMUSE_DSH_ORIGIN=http://127.0.0.1:12345 node web/dev-edge.mjs
```

Replace `12345` with the port of your running DSH Web server. Open `http://127.0.0.1:4174/app/`. The local edge serves Flutter at `/app/`, forwards `/gotrue/` and `/api/muse/devices` to `openmuseai.com`, `/dsh/` to the configured local DSH server, and paired Workspace/DSH root paths to the local Desktop gateway on `127.0.0.1:13180` (override with `OPENMUSE_PAIRED_GATEWAY_ORIGIN`). The proxy only listens on loopback and is **not** a production auth or paired Desktop gateway.

The browser discovers same-account online Desktop devices after login and requests a scoped grant from `/v1/account/open`. It then requests mount metadata and fetches each directory page only when that folder expands. The Desktop endpoint requires the paired grant and returns opaque references, never local paths. The production opaque E2E Edge is not part of this local proxy.

Run browser-only adapter tests with `flutter test --platform chrome test/browser_adapters_test.dart`; ordinary `flutter test` runs the VM-safe workbench tests.

The shared Flutter File Viewer displays authorized Markdown, text and images through the paired resource endpoint. PDF/video, Range and the production Web pairing Edge remain pending. The Office route factory in `lib/office_routes.dart` accepts an authorized resource handle and Engine ports; the browser Office Engine adapter remains pending. See the [UI and interaction acceptance matrix](../../docs/WEB-DESKTOP-WORKBENCH-UI-ACCEPTANCE.zh-CN.md) for passed and remaining gates.
