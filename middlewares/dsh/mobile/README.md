# mobile

移动端**宿主侧**（Flutter / Dart）。`plugins/` 里跑在 DSH Web 内的移动端适配插件（`dsh-mobile-surface`、`dsh-mobile-input`），这里是承载它们的 WebView 宿主本身。

| 目录 | 包 | 角色 |
|---|---|---|
| [`muse-dsh-mobile`](muse-dsh-mobile/README.md) | Flutter `muse_dsh_mobile` | Host-agnostic Flutter 库：内嵌 Remote DSH Web Client（WebView、导航、就绪、native-capability 端口）。只通过 host 端口与宿主 App 通信，不 import AppFlowy、Cloud auth 或宿主原生插件 |

不是 npm 包，不遵循 `@muse/*` 命名，依赖由 `pubspec.yaml` 管理；契约文件随包携带在 `muse-dsh-mobile/contracts/`。对应 Web 侧语义见 [plugins/dsh-mobile-surface](../plugins/dsh-mobile-surface/README.md) 与 [plugins/dsh-mobile-input](../plugins/dsh-mobile-input/README.md)。