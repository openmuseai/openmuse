# OpenMuse Web: Desktop shared workbench slice

This Flutter entry uses the same workbench layout, canvas, sidebar, file rows, editor tabs, pane menu, welcome screen, theme and Office widgets as Desktop/Mobile. The middle editor remains present by default. Web uses the shared GoTrue login UI/controller with a browser HTTP adapter. After login, the full workbench opens even when no Desktop is online; discovery, connection errors and retry remain inside the Workspace pane. The DSH pane mounts the pinned DSH Web client into a regular DOM node in the same page, without an iframe. An authenticated production Web Edge and full DSH conversation/approval validation are still pending.

Build the DSH pane module and Flutter route:

```sh
cd ../../web/dsh-pane
npm ci
npm run build
cd ../../app/openmuse_web
flutter pub get
# From Muse-Client root, use the fixed-region release pipeline:
# scripts/ci/build-web.sh --region us    # https://us.openmuseai.com/app/
# scripts/ci/build-web.sh --region cn    # https://app.openmuseai.com/
```

For local integration, start a DSH Web server on loopback, then from the repository root run:

```sh
OPENMUSE_DSH_ORIGIN=http://127.0.0.1:12345 node web/dev-edge.mjs
```

Replace `12345` with the port of your running DSH Web server. Open `http://127.0.0.1:4174/app/`. The local edge serves Flutter at `/app/`, forwards `/gotrue/` and `/api/muse/devices` to `openmuseai.com`, `/dsh/` to the configured local DSH server, and paired Workspace/DSH root paths to the local Desktop gateway on `127.0.0.1:13180` (override with `OPENMUSE_PAIRED_GATEWAY_ORIGIN`). The proxy only listens on loopback and is **not** a production auth or paired Desktop gateway.

The browser discovers same-account online Desktop devices after login and requests a scoped grant from `/v1/account/open`. It then requests mount metadata and fetches each directory page only when that folder expands. The Desktop endpoint requires the paired grant and returns opaque references, never local paths. The production opaque E2E Edge is not part of this local proxy.

Run browser-only adapter tests with `flutter test --platform chrome test/browser_adapters_test.dart`; ordinary `flutter test` runs the VM-safe workbench tests.

The shared Flutter File Viewer displays authorized Markdown, text and images through the paired resource endpoint. PDF/video, Range and the production Web pairing Edge remain pending. The Office route factory in `lib/office_routes.dart` accepts an authorized resource handle and Engine ports; the browser Office Engine adapter remains pending. See the [UI and interaction acceptance matrix](../../docs/WEB-DESKTOP-WORKBENCH-UI-ACCEPTANCE.zh-CN.md) for passed and remaining gates.

## Regional releases

Production builds require validated `OPENMUSE_WEB_DATA_REGION`, `OPENMUSE_WEB_ORIGIN`, `OPENMUSE_WEB_BASE_PATH` and `OPENMUSE_RELEASE_JSON` inputs. `scripts/release/prepare-web-config.py` compiles these from the installed release and `config/web-deployments.json`. The build script and Web CI prepare them automatically; direct release builds without them fail closed before creating auth adapters. `OPENMUSE_WEB_RELEASE_FILE` selects a provisioned trusted release. The release pipeline stores each region under `dist/web/cn/` or `dist/web/us/` and packages Flutter Web engine resources locally.

`?lang=` selects an enabled UI language only. `?region=` cannot override the installed service boundary. Browser origin, mount, fixed region and separate storage identities are checked before authentication. Login displays the installed region label. Language changes update Flutter text and HTML language/direction together. The portal uses its own region selection to navigate to the matching regional host; it never transfers a login session across regions.
