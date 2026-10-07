# OpenMuse CLI 插件、底部控制台与可扩展命令协议

状态：P0 CLI 与 Easel 可插拔流水线已在 macOS 通过 DSH 会话验证（2026-10-06）。具体进度与下一阶段见[实施计划](OPENMUSE-CLI-PLUGIN-IMPLEMENTATION-PLAN.zh-CN.md)。Host Gateway、统一 Job、Mobile 同一 receipt 与正式发布仍是后续关卡。

关联：[远程 Surface 实施计划](REMOTE-SURFACE-IMPLEMENTATION-PLAN.zh-CN.md)、[Mobile 通用预览与控制协议](MOBILE-REMOTE-PLUGIN-SURFACE-PROTOCOL.zh-CN.md)、[DSH Workspace 执行面](DSH-WORKSPACE-EXECUTION-PLANE-FEASIBILITY.zh-CN.md)、[X4 CLI Registry](PLUGIN-CLI-REGISTRY-ARTIFACT-RESOLVER.zh-CN.md)。

## 1. 产品形态与可行性

`com.openmuse.cli` 是可停用的 OpenMuse 插件。它提供编辑区下方可调整高度的控制台、终端会话，以及可独立在 OS 终端运行的 `openmuse` 命令入口。控制台默认可用，但它的 UI 和 PTY 进程不属于 Host 启动的必要路径：插件故障或停用时，编辑器、DSH、其他业务插件和 Mobile 远程 Surface 继续工作。无 GUI 的 Linux 只加载 CLI 插件的命令入口和 Host 连接器，不创建 Flutter 控制台。

用户可在控制台为一个 tab 选择本机 shell：macOS `zsh`、Linux `bash`、Windows `PowerShell`（优先 `pwsh`，没有时可选 Windows PowerShell）。shell tab 接受普通系统命令，包括 `openmuse ...`。OpenMuse 插件贡献的命令由 `openmuse` 分发器解析，例如 `openmuse easel douyin plan`；插件不能覆盖 `openmuse plugin install` 等核心命令。Shell 选择只影响交互式命令行语法；程序化调用始终传结构化 argv，避免多层 shell 转义。

技术上可行，当前已完成主编辑区下方 CLI 面板与 Easel 离线命令闭环。Host 仍只把 `cli.console` 组合到主编辑区，尚无通用 dock 布局；X4 Registry 仍服务于 Agent Sandbox，人类 CLI 使用独立发现器。现有实现还未接 Host Gateway、统一 Job 和真实发布，因此不能由 `openmuse easel douyin plan` 成功推断完整社媒流水线可用。

## 2. 三条入口，共用一个领域命令服务

```text
Desktop 底部控制台 ─ shell PTY ─ openmuse launcher ┐
OS Terminal / Linux SSH ─────── openmuse launcher ──┼─ Host CLI Gateway ─ Broker / Domain Service
DSH Bash ─ WSR scoped openmuse launcher ────────────┘          │
Mobile remote-workbench ─ remote-surface/control ──────────────┘
                                                               ├─ Plugin Authority / CLI Registry
                                                               ├─ Workspace / Resource / Job Authority
                                                               └─ Easel worker 等业务执行体
```

Host/Broker 持有安装、授权、资源和任务状态；CLI 插件持有输入/输出和 shell 会话，不复制业务实现。Desktop UI、OS 命令行、DSH 和 Mobile 对同一任务看到同一 `jobRef`、状态、receipt。Mobile 仍通过 remote-surface 协议操控业务界面，不把手机变成任意 Desktop shell；其控制动作最终映射到同一个领域命令服务。CLI 不负责渲染 Mobile 组件树，remote-surface 不负责解释终端文本。

区分三类调用：

| 类型 | 入口与身份 | 执行位置 | 约束 |
| --- | --- | --- | --- |
| 本机 shell 命令 | 用户打开的 PTY；遵循当前 OS 用户权限 | 本机 shell | 明确标记为本机执行；不能伪称它天然满足 WSR 隔离 |
| OpenMuse 领域命令 | `openmuse` argv 或远程 Surface action；Host 验证 actor、Workspace、grant | Host 服务或受控插件 Worker | 参数 schema、权限、审计、幂等和外部副作用规则相同 |
| DSH Agent 命令 | WSR Lease 内的 scoped launcher；Agent 身份 | Sandbox/受控 Worker | 取 Lease Registry 与 Host grant 交集；默认没有安装插件或真发权限 |

`openmuse` launcher 连接当前用户的 Host 服务。macOS/Linux 优先用户权限的 Unix socket，Windows 用命名管道；headless Linux 可由 CLI 启动/连接用户级 Host daemon。Host 统一持久化和加锁，CLI 不直接写 `settings-v1.json`、安装目录或 Job ledger。现有 `openmuse_plugin.dart install` 直接更新 JSON 文件，只是过渡实现；迁移时保留为兼容入口，内部改为调用 Host 安装服务。

## 3. CLI 插件本身的贡献

插件 ID `com.openmuse.cli`。建议拆成同一插件的 target artifact：Desktop Flutter 底部面板、平台 PTY 适配、原生 `openmuse` launcher；Linux headless 只需要 launcher 和 Host Gateway 客户端。Host 的命令服务和核心命令注册不随 CLI 插件停用；停用仅撤销它贡献的控制台/命令入口。外部已启动的 CLI 请求完成或收到明确取消，不能让卸载中断 Host 的其他操作。

Desktop 布局增加真正的 `bottomPanel` 插槽，位于编辑区下方、状态栏上方。控制台有多个 tab、工作目录、shell profile、会话退出码、任务输出；支持键盘聚焦、复制粘贴、窗口 resize、进程树停止、恢复最近 tab 配置。UI 挂载和 PTY 生命周期由 CLI 插件负责；Host 只给它受限的面板注册与命令服务客户端。可复用 `plugins/helix` 的 PTY/xterm 经验，但要抽取共享终端库或实现独立会话，避免控制台误操作 Helix 编辑进程。面板关闭可选择保留或终止会话，应用退出必须清理子进程；运行中的后台任务交给 Host Job Service 而非悬挂 PTY。

平台规则：shell 可执行文件需由 Host 检测并由用户显式选择；默认分别是 `zsh`、`bash`、`pwsh`/Windows PowerShell。Windows 的 PowerShell 和 DSH 的 Bash 是不同执行语义，不能以其中一个冒充另一个。每个 tab 明确 `cwd`、环境变量来源和登录/非登录模式；敏感 Host token 不注入 shell 环境。`openmuse` 使用短期本机连接凭证，经 Host 校验 OS 用户/会话；Agent 的 launcher 只能拿 Lease 范围内的 endpoint。插件 Worker 不继承可安装插件的管理员凭证。

## 4. 命令协议与插件扩展

### 4.1 命令地址和结构化调用

用户形式：`openmuse <group> <namespace> <command> [flags]`，例如 `openmuse easel douyin plan --title ...`。内部 identity 固定为 `group/namespace/command`，与 X4 一致；顶层 `plugin`、`workspace`、`job`、`auth`、`surface` 等核心 group 由 Host 保留。显示别名只由 Host 分配，碰撞时确定性拒绝并提示完整插件 ID；不能按安装顺序抢占命令。高级形式 `openmuse plugin invoke com.openmuse.easel douyin/plan --input @plan.json` 避免别名歧义。为了像普通安装软件一样直接敲 `easel ...`，可由 CLI 插件为用户主动启用且无冲突的插件创建指向 `openmuse easel ...` 的轻量 shim；shim 放在 OpenMuse 管理的 bin 目录，用户决定是否加入系统 PATH，停用插件即撤销它。插件包自身不得任意修改系统 PATH 或覆盖已有可执行文件。

Gateway 请求至少携带 `commandId`、规范化 JSON 参数、actor/session、workspaceRef、requestId、预期 revision、Registry generation；响应为 typed result 或 `jobRef`/receipt。输出支持默认可读文本与 `--json` 稳定 schema；结果写 stdout，诊断写 stderr，退出码区分参数错误、未安装/未授权、冲突、执行失败、结果未知。长任务用 `--wait` 或 `openmuse job watch <ref>`，终端断开不取消已提交任务。命令发现、帮助和补全来自同一 Registry，例如 `openmuse help easel`、`openmuse completion zsh`。

建议的声明示意（**目标 schema，不是当前 Manifest v2 可接受的 JSON**）：

```json
{
  "contributes": {
    "cli": [{
      "group": "easel",
      "namespace": "douyin",
      "command": "plan",
      "audiences": ["human", "agent"],
      "input_schema": "openmuse.easel.douyin.plan.input/v1",
      "output_schema": "openmuse.easel.douyin.plan.output/v1",
      "effects": ["read-workspace"],
      "required_permissions": ["workspace.context.read"],
      "worker_artifact": "easel-worker"
    }]
  }
}
```

当前先引入并行的 `cli` 声明，并已更新 JSON Schema、Dart/Rust 解析器与测试；它只服务于用户主动调用的实验性本机命令。后续需将人类 CLI 与现有 `agent_cli` 投影到同一个经验证的命令 identity 和 Handler，再补齐授权、Registry TCK 与 Host Gateway。X4 只接受符合 target、digest、signature、ABI、license、grant 的 `sandbox-worker`；当前 Easel 包只有 `runtime-closure`，没有 `agent_cli` 声明，故不能直接进入 X4 Registry。人类 CLI 与 Agent CLI 可共享协议，但 grant、可见命令和执行世界必须分别计算。

插件命令在独立 Worker 中运行，通过 Host 提供的资源句柄与服务调用工作；插件停用、升级或崩溃时只撤销自己的命令和会话，其他命令继续可用。安装后的启用状态不等于授予全部命令权限。Registry 每次解析产生 digest/generation；运行中的 Job 固定版本与 Worker，新增调用使用最新 generation。插件更新时旧命令地址如不兼容，应显式保留旧版本或返回迁移错误。

### 4.2 外部副作用和 Mobile 对齐

Easel `plan/preview` 与 `publish.commit` 分开。真发须绑定账号、Workspace、预览 fingerprint、资源 revision、一次性确认令牌和幂等键；CLI、Desktop 和 Mobile 都必须经 Host 的同一确认接口。`--yes` 只可省略纯本地可逆操作的交互提示，不能绕过真发确认。响应丢失先查 receipt；查不到返回 `outcome_unknown`，不自动重发。CLI 的 `job show` 与 Mobile 的 snapshot/event 读取同一 ledger。社媒 cookie、浏览器 profile 路径、平台 AppSecret 不出 Desktop Worker。

## 5. 命令树与迁移顺序

目标命令树应覆盖 Host 已提供的**领域操作**，不承诺把每个 UI 点击坐标暴露为 CLI：

```text
openmuse status | doctor | auth ...
openmuse plugin list | inspect | install | enable | disable | update | uninstall
openmuse workspace list | open | status | checkpoint | history | sync
openmuse resource list | inspect | import | export
openmuse job list | show | watch | cancel
openmuse surface list | inspect
openmuse easel douyin plan | preview | account | publish
```

上述命令需要随对应 Host 能力逐步落地；`auth`、`workspace`、`resource` 等未实现的动作不得提供只打印成功的占位命令。建议按以下关卡实施：

1. **CLI 插件与 Host Gateway**：完成底部 dock、PTY、平台 shell、headless launcher、连接/身份、`status/help`；验证停用 CLI 插件不影响编辑器和其他插件。
2. **安装命令迁移**：把当前 `pack/install` 接到 Host 服务，增加 `plugin list/inspect/enable/disable`，事务化安装和审计。`pack` 是开发者命令；`install` 使用目录 SHA 和签名证据，不在 UI/CLI 两处维护状态文件。
3. **通用 Registry**：Manifest 命令贡献、target artifact、参数/输出 schema、效果和授权、命令补全；用一个无社媒副作用的 Easel `plan` Worker 验证装载、停用、升级和命名冲突。
4. **领域能力覆盖**：Workspace、Resource、Job 依次接入；同一动作在 UI/CLI/Mobile 返回同一 Ref 和 receipt。DSH 通过 WSR 的独立 scoped launcher 进入同一命令协议。
5. **Easel 发布与 Linux**：固定 Python 依赖和平台浏览器运行时，先经测试账号走预览/确认/回读；提供 Linux artifact 并在无 Flutter UI 的 Linux Host 验证安装、调用和卸载。Linux 能否执行某社媒 publisher 要按平台逐项验证，不能由 CLI 支持 Linux 推出 Easel 全平台可用。

验收矩阵：控制台 shell 的 cwd/resize/退出和快捷键；macOS zsh、Linux Bash、Windows PowerShell 命令解析；CLI/UI 并发安装只产生一份一致回执；插件停用撤销自身命令但其他插件可用；Agent 缺 grant 不能经 Bash 调用用户命令；同一 Easel preview 从 CLI 和 Mobile 取得同一任务状态；真发断线不重复提交。远程 Surface 的 RS0–RS7 fake 验收仍按[既有矩阵](REMOTE-SURFACE-IMPLEMENTATION-PLAN.zh-CN.md)执行，不等同真实 Easel 发布。

## 6. 当前实现：插件隔离、安装引导与 DSH 流水线

`com.openmuse.easel` 0.4.2 不随 Host 内置发行。Easel 的打包器、Python 安装引导、媒体加工与社媒脚本位于 `plugins/easel`；Host 不导入 Easel 类型，不生成 Easel 命令，也不再显示硬编码 Easel 入口。`third_party/Easel` 仍是上游源码，打包器从中选取抖音与知乎脚本及其直接依赖。旧 OpenClaw Web 桥从 DSH 插件移除，运行时不启动它。

Host 的通用安装协议读取 `runtime.prepare` 步骤：先验证 catalog 包 SHA、清单 artifact digest 与目标平台，再把 artifact 写入指定工作区，执行插件声明的 Python 入口。入口由 Easel 自己实现：创建隔离 venv、安装 `Pillow` 与固定版 Playwright、下载 Chromium/Headless Shell、检查 FFmpeg。安装引导返回非零时不写 receipt；再次安装会修复半成品 venv。当前仍须在线取得 PyPI/浏览器依赖，尚无离线 wheel/browser 闭包和发行签名。Host 负责协议、路径与进程边界，不含依赖列表或社媒业务规则。

Manifest 的 `contributes.cli` 声明命令地址、说明、允许的值选项与开关、输入/输出 schema、效果、入口 artifact。`openmuse commands --json` 输出 `openmuse.cli-discovery/v1`，供 DSH 发现。DSH 插件向 `DSH_HOME/skills/openmuse-cli/SKILL.md` 写通用调用说明，并将打包 CLI 路径放入 sidecar 的 PATH；DSH 用 Bash 启动子进程、读 UTF-8 stdout/stderr 和退出码。当前实验通过 Bash 工具启动，不等于 P2 的 Agent CLI grant/Job 服务：未来须按 WSR Lease 控制文件、网络和发布副作用。

本机安装及验证顺序：

```bash
cd plugins/easel
/Users/mac/src/flutter/bin/dart run bin/pack.dart --easel /Users/mac/src/openmuse-io/Muse-Client/third_party/Easel --output /Users/mac/src/openmuse-io/Muse-Client/dist/easel
cd ../../app/openmuse_host
/Users/mac/src/flutter/bin/dart run bin/openmuse.dart plugin uninstall --plugin com.openmuse.easel
OPENMUSE_PYTHON=/Users/mac/.local/bin/python3.11 /Users/mac/src/flutter/bin/dart run bin/openmuse.dart plugin install --catalog /Users/mac/src/openmuse-io/Muse-Client/dist/easel/catalog.json --plugin com.openmuse.easel --workspace '/Users/mac/Library/Application Support/com.openmuseai.office/OpenMuse/Workspace/plugins/com.openmuse.easel'
/Users/mac/src/flutter/bin/dart run bin/openmuse.dart list
/Users/mac/src/flutter/bin/dart run bin/openmuse.dart commands --json
/Users/mac/src/flutter/bin/dart run bin/openmuse.dart easel douyin check
```

2026-10-06 的 DSH headless 会话 `session-17e39e13-1cdd-4a02-bfa2-8ff57dea2554` 先读取 `commands --json`，随后经 `openmuse easel video process` 把 `/Users/mac/Documents/OpenMuse/Cap 2026-10-05 at 23.38.12.mp4` 加工为 1920×1080、38.501 秒 H.264/AAC 视频，同时生成 OpenMuse 封面 PNG；DSH 捕获 `rendering → ready` 输出和退出码 0，使用 `ffprobe` 复核，又运行 `easel douyin plan` 得到离线发布预览，退出码 0。按用户最新要求没有执行 `--exec` 或正式发布。会话事件见本机 `/tmp/openmuse-dsh-easel-session.jsonl`。

控制台支持 `openmuse list` 简写，顶部 `+` 创建独立 shell tab，拖动编辑器与终端间的 12px 手柄可改高度；折叠按钮移到手柄，避免遮挡 `+`。Debug App 的原生 CLI launcher 已更新，完整 Flutter UI 改动需重启更新后的 App 才能看到。

### 6.1 知乎命令与授权弹窗（0.4.2）

Easel 包增加 `easel zhihu inspect/check/selftest/plan/whoami/login/publish`。`inspect --article <绝对路径>` 读取 Markdown、核对标题及本地图片，输出结构化 JSON `imageCount/missingImages/canPublishRequestedArticle/blockingIssue`。知乎上游 `web_publisher.py` 当前只能填入纯文本正文，不能上传图文稿的内嵌图片，因此 `publish` 的命令说明和 `inspect` 结果都明确阻断“图文已发布”的误报；也不允许把 Markdown 当作 `--media` 伪装成图片上传。知乎网页选择器尚须在已授权账号上验证。

授权由 Easel 自己的 `runtime/easel_social.py` 启动 Playwright，默认将 QR、状态文件和浏览器 profile 写在 Easel 工作区。它发出 `openmuse.plugin-interaction/v1` 的 `image.challenge` 事件到工作区 `state/interactions`，含插件 ID、标题、图片路径、状态路径和时间戳。通用 DSH 面板监听已安装插件的交互目录，核对 receipt 和路径均在插件工作区，再用 Flutter 弹窗显示图片与状态；二维码刷新时重读图片字节，不依赖文件路径缓存。Host 与 DSH 面板不包含知乎/抖音的登录规则。通用协议可由 Mobile remote surface 转发，但本版尚未接移动端授权 UI。

本机真实验证：知乎 `selftest`、`check` 均退出 0；`login` 可从当前网络取得 139×138 的扫码 canvas，并在新版 OpenMuse 桌面工作台弹窗展示。报告图文稿 `docs/meterails/OPENMUSE-VS-DSH-知乎图文.md` 含 4 张存在的图片，`inspect` 返回 `stage=ready`、`canPublishRequestedArticle=false`、明确的内嵌图片阻断原因。DSH headless 会话 `/tmp/openmuse-dsh-zhihu-session.jsonl` 已经自行发现知乎命令、执行健康检查与 `plan`，没有转到抖音，也没有发布。该会话从仓库根目录启动，DSH `workspace-write` 只允许写其不可变 cwd；调用 `login` 写插件工作区时被 Seatbelt 拒绝。随后从 Easel 工作区启动的新 DSH 会话 `/tmp/openmuse-dsh-zhihu-workspace-session.jsonl` 自行依次运行 `commands --json → zhihu inspect → check → plan → login`，授权事件被桌面窗口消费并弹出可扫描二维码；30 秒无人扫码后状态正确转为 `expired`、CLI 退出 1，全程未发布。长期方案需要 Host 授权的 CLI Job Broker，使插件工作区作为受控写入范围，不能通过给整个 DSH 会话永久 `danger-full-access` 解决。
