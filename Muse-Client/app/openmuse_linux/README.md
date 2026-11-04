# OpenMuse Linux

Linux 原生客户端工程，位于主仓库 `app/openmuse_linux`，使用主仓库的插件、SDK、字体、Helix 源码及 DSH 锁文件。这里没有嵌套 Git 仓库；随 `Muse-Client` 统一版本管理。

产品定义见 [产品方案](docs/PRODUCT.zh-CN.md)，平台边界和构建链见 [技术方案](docs/ARCHITECTURE.zh-CN.md)。实际构建结果、架构及验收范围见 [构建记录](docs/BUILD-REPORT.zh-CN.md)。

## 环境

- Debian 12 或 Ubuntu 24.04，x86_64 / aarch64 原生构建。
- Flutter **3.44.2** / Dart **3.12.2**，Rust **1.90.0**（与 Helix 固定工具链一致）。
- GTK 3、WebKitGTK 4.1（>= 2.40）、Secret Service。普通用户桌面运行，需要可用的系统密钥环。
- 中文界面依赖 Noto CJK 系统字体，deb 会声明 `fonts-noto-cjk`；最小容器需先安装该字体包。
- Node **22.19.0** 自动下载并校验仓库保留的官方 SHA256；与 DSH 一起打入包，终端用户不必安装 Node。
- Python >= 3.11，构建时需要访问 npm、Cargo 注册表和固定版本的 Tree-sitter 仓库。

## 构建

从主仓库根目录执行：

```bash
rustup toolchain install 1.90.0 --profile minimal
bash app/openmuse_linux/tool/bootstrap.sh
bash app/openmuse_linux/tool/build.sh
```

也可在 macOS 或 Linux 用 Docker 构建。目标架构取决于容器架构，默认是当前机器的架构：

```bash
docker build -f app/openmuse_linux/tool/Dockerfile -t openmuse-linux-builder .
test -f ../Muse-Plugin-Ecosystem/plugins/markdown/flutter_runtime/pubspec.yaml
docker run --rm --shm-size=1g \
  -e PUB_HOSTED_URL=https://pub.flutter-io.cn \
  -v "$PWD:/workspace/Muse-Client" \
  -v "$PWD/../Muse-Plugin-Ecosystem:/workspace/Muse-Plugin-Ecosystem:ro" \
  -v openmuse-linux-pub:/root/.pub-cache \
  -v openmuse-linux-cargo-registry:/root/.cargo/registry \
  -v openmuse-linux-cargo-git:/root/.cargo/git \
  -w /workspace/Muse-Client \
  openmuse-linux-builder bash app/openmuse_linux/tool/build.sh --region us
```

当前插件闭包还依赖相邻的 `Muse-Plugin-Ecosystem/plugins/markdown/flutter_runtime`，需保留上述目录关系。锁文件记录的 hosted registry 为 `pub.flutter-io.cn`；构建时使用同一 registry，保留 `--enforce-lockfile`，不要通过升级依赖绕过校验。跨 OS 共用 bind checkout 会重新生成 `.dart_tool`；隔离构建应只同步所需源码，不复制整个工作区的 `tmp`、安装包、其他端的 build 或 Rust target。建议开始前预留至少 20 GiB 主机磁盘空间。

产物在 `app/openmuse_linux/build/dist/{amd64,arm64}/{cn,us}/`，包含 `.deb`、`.tar.gz`、`SHA256SUMS`、`build-info.json`。包元数据通过实际 ELF 依赖生成；必须遵守其中的 glibc 和系统库版本要求。Docker 基线为 Debian 12，不能据此声称支持 Ubuntu 22.04。

`--debug` 生成调试程序；`--skip-tests` 仅跳过 Dart 测试，仍运行分析及 DSH 运行时验证；`--without-grammars` 仅用于快速验证编辑器启动，构建记录会标注缺少语法解析器，不能作为完整发行包验收。

GitHub Actions 入口为主仓库 `.github/workflows/linux-build.yml`，分别在原生 amd64 / arm64 runner 的 Debian 容器里构建。

## 安装和运行

```bash
cd app/openmuse_linux/build/dist/arm64  # x86_64 改为 amd64
sha256sum --check SHA256SUMS
sudo apt install ./OpenMuse-linux-*.deb
openmuse
```

也可解压 `.tar.gz`，运行 `OpenMuse/openmuse_linux`。便携包仍依赖系统 GTK、WebKitGTK 与密钥环。系统应用菜单提供 OpenMuse 入口。升级使用新的 `.deb` 覆盖安装；首版不提供应用内更新安装器。

## 源码基线

Linux 工作台以 `cbd243e35446dbded217d7c1676128d2c5c5af41` 的 `app/openmuse_host/lib` 为初始基线，位于本工程 `lib/`。未引入原工作区尚未提交的 Office 改造。外部 `appflowy_editor_native_plugin` 依赖已移除，Markdown 使用 Helix 和公共预览组件，安装的 MCP App 使用通用适配器。

公共能力仍直接引用 `../../packages`、`../../plugins`。后续应把两个 Host 的公共服务提取成共享包，避免长期手动同步整个 `lib/`。更新基线时先对照原提交做差异审查，再运行本工程完整测试，不能直接覆盖 Linux 适配。

Web 菜单与远程工作区修复记录：[WEB-REMOTE-VERIFICATION.zh-CN.md](docs/WEB-REMOTE-VERIFICATION.zh-CN.md)。
