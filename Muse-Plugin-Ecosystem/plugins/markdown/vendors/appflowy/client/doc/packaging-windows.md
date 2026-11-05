# Windows 开箱即用分发

Windows 分发对齐 macOS：安装（或解压）后即可运行，**不依赖本机 Node / 源码树**。DeepSeek sidecar 打进 `{exeDir}/muse/`，和 macOS `Muse.app/Contents/Resources/muse` 同一套运行时。

> 现状是把整棵 `node + node_modules` 拷进 `muse/`（约 2 GB），官方做法是「生产闭包 + pkg `--sea` 单文件 exe」。两种方式的逐环节对比、以及向官方路线对齐的改法见 [packaging-official-single-exe-comparison.md](./packaging-official-single-exe-comparison.md)。
>
> 当前实施（A 路线：npm 闭包 + node carrier + 捆绑 pnpm 一键装插件）见 [packaging-setup-exe-plan.md](./packaging-setup-exe-plan.md) 与 [packaging-node-carrier.md](./packaging-node-carrier.md)。

## 用户拿到什么

打包完成后，在 `frontend/client/dist/windows/`：

| 产物 | 用途 |
|---|---|
| `Muse/` | 便携目录，双击 `Muse.exe` 即可 |
| `Muse-windows-x64.zip` | 给下载页的 zip（内含 `Muse/` 目录） |
| `Muse-windows-x64-setup.exe` | 可选，Inno Setup 安装包（需本机安装 Inno Setup 6） |

用户只需：

1. 解压 zip，或运行 setup.exe（默认装到 `%LOCALAPPDATA%\Programs\Muse`）
2. 启动 **Muse**
3. 第一次打开 DeepSeek 面板时填写 `DEEPSEEK_API_KEY`

用户数据在 `%APPDATA%\Muse\`，不写进安装目录。

便携目录布局（A：闭包，默认）：

```
Muse/
├── Muse.exe
├── dart_ffi.dll
├── flutter_windows.dll
├── data/
└── muse/                 # 打进包里的 DSH 运行时（不依赖源码树）
    ├── node/node.exe     # 官方 Node 22.19.0（仅运行时二进制）
    ├── closure/          # 零符号链接 npm 闭包（含 @muse/* + dshmarket + dsh lib/bin.js）
    ├── closure-tarballs/ # 首启 / 一键装插件的 file: 源
    ├── plugin-tools/     # bundled pnpm + pnpm-runner.mjs
    └── patch.yml
```

应用启动时若发现 `exe` 旁边有 `muse/patch.yml` 且存在 `muse/closure/`（或旧布局 `muse/dsh/`），就走打包模式，用捆绑的 Node 起 sidecar。安装版 **不要** 设 `MUSE_ROOT`。

验证：

```bat
python frontend/client/scripts/verify-windows-portable.py
```

## 打包机要求

- Windows x64
- Python 3.10+
- Flutter **3.27.x**（`FLUTTER_HOME` 指向该 SDK，本机可用 `D:\muse\flutter-3.27.4`）
- Rust 1.85、cargo-make
- Visual Studio 2022+（MSVC x64 + CMake）
- Node.js / pnpm（只用于**打包机**编译 `@muse/*` 和拉 dshmarket）
- `vendors/deepseek-harness` 已 `pnpm install`
- 可选：[Inno Setup 6](https://jrsoftware.org/isinfo.php)（生成 setup.exe）

## 一条命令

在仓库根目录：

```bat
python frontend/client/scripts/pack-windows-client.py
```

只换 Flutter、复用已打好的闭包：

```bat
python frontend/client/scripts/pack-windows-client.py --skip-core-build --skip-packages --reuse-runtime
```

回退整树拷贝：

```bat
python frontend/client/scripts/pack-windows-client.py --no-closure
```

调试包（更快，体积更大）：

```bat
python frontend/client/scripts/pack-windows-client.py --debug
```

Flutter 已编过、只重打 sidecar：

```bat
python frontend/client/scripts/pack-windows-client.py --skip-app-build
```

`@muse/*` 已 `pnpm build`：

```bat
python frontend/client/scripts/pack-windows-client.py --skip-packages
```

装到当前用户并启动：

```bat
python frontend/client/scripts/install-windows-client.py --launch
```

## 脚本

全部是 Python（macOS 侧仍是 bash，Windows 侧统一 Python）：

| 脚本 | 作用 |
|---|---|
| [`pack-windows-client.py`](../scripts/pack-windows-client.py) | 编 Flutter + 打进 `muse/` + zip + 可选 Inno |
| [`install-windows-client.py`](../scripts/install-windows-client.py) | 拷到 `%LOCALAPPDATA%\Programs\Muse` 并建开始菜单 |
| [`lib/muse_windows.py`](../scripts/lib/muse_windows.py) | 路径、Node 下载、harness 拷贝、wire `@muse` |
| [`lib/wire-muse-node-modules.py`](../scripts/lib/wire-muse-node-modules.py) | 给打包的 `@muse` 包补运行时依赖（Windows 上拷贝，不依赖符号链接） |

对应 macOS（同一套闭包，运行时在 `.app/Contents/Resources/muse`）：

- [`pack-macos-client.py`](../scripts/pack-macos-client.py) / [`packaging-macos.md`](./packaging-macos.md)

## 和 macOS 的差异

- macOS 把运行时放进 `.app/Contents/Resources/muse`；Windows Flutter 没有 bundle，所以放在 **exe 同目录的 `muse/`**。
- Windows 官方 Node 是 zip（`node.exe` 在根上），脚本会额外复制一份到 `muse/node/bin/node.exe`，方便和 Unix 查找路径对齐。
- 打包 zip 不做代码签名。若要 SmartScreen 少拦，需用你们自己的 Authenticode 证书签 `Muse.exe` 和 setup.exe。

## 故障

- `DSH node_modules missing` / `error (23)` / `TimeoutError`：在 `vendors/deepseek-harness` 执行 `pnpm install`。国内访问 `registry.npmjs.org` 容易超时；pnpm 11 默认 60 秒不够下 Codex/Claude 那些上百 MB 的 `win32-x64` 包。用镜像并加长超时：

```bat
cd vendors\deepseek-harness
pnpm install --registry https://registry.npmmirror.com --fetch-timeout 1800000 --network-concurrency 1 --fetch-retries 10
```

`fetch-timeout` 必须写在命令行（或 `pnpm-workspace.yaml`）。写在用户 `~/.npmrc` 里会被 pnpm 11 忽略。
- `missing built dist for …`：不要加 `--skip-packages`，或先编 `@muse/*`。
- `Windows packs need Flutter 3.27.x`：设置 `FLUTTER_HOME` 到 3.27 SDK（本机可用 `D:\muse\flutter-3.27.4`），不要用 master/3.48。脚本读 SDK 的 `version` 文件，避免中文 Windows 下 `flutter --version` 的 GBK 解码失败。
- 安装后 DeepSeek 仍报缺脚本：确认安装目录里有 `muse/patch.yml`，且用的是本脚本打出来的包，而不是裸的 `flutter build windows` 输出。
