# macOS 开箱即用分发

macOS 分发对齐 Windows A 路线：安装（拷到 `/Applications` 或解压 zip）后即可运行，**不依赖本机 Node / pnpm / 源码树**。DeepSeek sidecar 打进 `OpenMuse.app/Contents/Resources/muse/`。

当前实施：npm 闭包 + 捆绑 Node + 捆绑 pnpm（一键装插件）。排除未使用的 Codex/Claude 子代理包以缩小体积。Windows 对照见 [packaging-windows.md](./packaging-windows.md)、[packaging-setup-exe-plan.md](./packaging-setup-exe-plan.md)。

## 用户拿到什么

打包完成后，在 `frontend/client/dist/macos/`：

| 产物 | 用途 |
|---|---|
| `OpenMuse.app` | 可直接打开，或拖到 `/Applications` |
| `OpenMuse-{version}-macos-universal.zip` | 下载页使用的 zip（内含 `.app`） |
| `OpenMuse-macos.zip` | 兼容别名，内容相同 |

用户只需：

1. 解压 zip，把 **OpenMuse** 拖到「应用程序」
2. 第一次从 Gatekeeper 打开：右键 → 打开
3. 第一次打开 DeepSeek 面板时填写 `DEEPSEEK_API_KEY`

用户数据在 `~/Library/Application Support/OpenMuse/`，不写进 `.app`。

`.app` 内布局（A：闭包，默认）：

```
OpenMuse.app/
└── Contents/
    ├── MacOS/OpenMuse
    └── Resources/muse/
        ├── node/bin/node          # 官方 Node 22.19.0（仅运行时二进制）
        ├── closure/               # 零符号链接 npm 闭包
        ├── closure-tarballs/
        ├── plugin-tools/          # bundled pnpm + pnpm-runner.mjs
        └── patch.yml
```

应用启动时若发现 `Contents/Resources/muse/patch.yml` 且存在 `muse/closure/`（或旧布局 `muse/dsh/`），就走打包模式。安装版 **不要** 设 `MUSE_ROOT`。

Release 包会把生产 Cloud 写进二进制（`config/openmuse-cloud.json` → `--dart-define-from-file`）：`https://openmuseai.com` 的 API / GoTrue / WebSocket。Debug 包仍走 `.env` 的 localhost。

验证：

```bash
python frontend/client/scripts/verify-macos-app.py
strings dist/macos/OpenMuse.app/Contents/MacOS/OpenMuse | grep openmuseai.com
```

## 打包机要求

- macOS（当前脚本按本机 arch 打：arm64 或 x86_64）
- Python 3.10+
- Flutter **3.27.x**（`FLUTTER_HOME`，或 `~/sdks/flutters/flutter`）
- Rust 1.85、cargo-make、`protoc-gen-dart`
- Node.js / pnpm（只用于**打包机**编 `@muse/*` 和构建闭包）
- `vendors/deepseek-harness` 可被 `build-muse-closure.py` 打包

下载页文件名用 `macos-universal`，与现有 catalog 一致。本机单架构构建仍用该文件名；真正的 universal 二进制需要额外在两台机器（或 lipo）上编 Flutter。

## 一条命令

在仓库根目录（`Muse-Clients`）：

```bash
python frontend/client/scripts/pack-macos-client.py
# 或
./frontend/client/scripts/pack-macos-client.sh
```

只换 Flutter、复用已打好的闭包：

```bash
python frontend/client/scripts/pack-macos-client.py --skip-core-build --skip-packages --reuse-runtime
```

回退整树拷贝（体积大，不推荐）：

```bash
python frontend/client/scripts/pack-macos-client.py --no-closure
```

装到本机：

```bash
./frontend/client/scripts/install-macos-client.sh
```

## 脚本

| 脚本 | 作用 |
|---|---|
| [`pack-macos-client.py`](../scripts/pack-macos-client.py) | 编 Flutter + 打进 `Resources/muse` + zip |
| [`pack-macos-client.sh`](../scripts/pack-macos-client.sh) | 转调 Python |
| [`install-macos-client.sh`](../scripts/install-macos-client.sh) | 拷到 `/Applications/OpenMuse.app` 并 ad-hoc 签名 |
| [`verify-macos-app.py`](../scripts/verify-macos-app.py) | 闭包 sidecar 启动 + 可选插件安装 |
| [`build-muse-closure.py`](../scripts/build-muse-closure.py) | 与 Windows 共用的 npm 闭包 |
| [`lib/muse_windows.py`](../scripts/lib/muse_windows.py) | 闭包暂存、捆绑 Node（macOS 用 darwin tarball） |

Ad-hoc 签名会深度签 `Contents/Frameworks` 并带上 `disable-library-validation`。否则主程序的 Hardened Runtime 会在从网上下载解压后（App Translocation）让 dyld 拒绝 `CwlCatchException.framework`，表现为「打开即闪退」。分发到外网用户仍可能被 Gatekeeper 拦截（未公证），需要右键 → 打开；正式渠道需 Apple Developer 证书与公证。
