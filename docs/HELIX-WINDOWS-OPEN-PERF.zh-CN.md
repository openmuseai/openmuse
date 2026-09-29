# Windows Helix 打开文件性能分析

2026-09-28，在 Windows Release 应用中，从 `intellij-community` 工作区依次打开 `README.md`、`SECURITY.md`，再返回 `README.md`。使用应用与 hx 共用的 `OPENMUSE_HELIX_TRACE` JSONL 跟踪，并核对应用的活动标签和 hx 子进程。测试文件是同一工作区内的普通文件；下列数字是单次实测，不能当作跨机器基准。

## 原因与测量

旧路径为每个文件创建一个 hx 进程和 Windows ConPTY。第二个文件从 `open_begin` 到可信 `first_state` 为 **2206 ms**，其中同步 `Pty.start` 到 `pty_started` 为 **1532 ms**。这段原本在 Flutter UI isolate 执行，会让窗口暂时无响应。首次插件激活还要运行一次 `hx --version`，此次耗时 **492 ms**。配置文件在激活后又写了一次。

新路径让同一 hx 处理后续文件，并通过带 token 和 PID 校验的控制通道发送 `open` 命令。两次 Release 实测的新文件切换到 `switch_ready` 分别为 **483 ms** 和 **428 ms**，约为旧路径的 **4.6–5.2 倍速度**；返回已有 buffer 为 **29 ms**。整个切换过程中每个应用实例均保持一个 hx 子进程，UI 的活动标签依次是 `README.md`、`SECURITY.md`、`README.md`。

首次打开仍需等待 Windows 创建进程和 Helix 初始化。两次新路径实测从 `open_begin` 到 `first_state` 为 **2324–2403 ms**，其中 ConPTY/进程创建为 **1593–1599 ms**。这次 `Pty.start` 在后台 Dart isolate 执行，UI isolate 可继续处理输入与帧；真实 Windows PTY 回归测试用 50 ms 计时器检查主 isolate 没有被阻塞。随包引擎跳过了运行时版本探测，插件激活从旧路径的 **539 ms** 降到 **50–70 ms**。显式指定的外部 hx 仍须经过能力探测。

hx 内部的细分 trace 显示，在该大型 Git 工作区，首次 `Document::open` 约 **8 ms**，直到取得 VCS 基线约 **359 ms**，直到文档事件及语言服务初始化约 **615 ms**；第二个文件分别约为 **5/161/410 ms**。因此后续文件切换主要受 VCS 与文档初始化影响。保留这些能力；若要继续降低 400–500 ms，应先单独验证差异基线和语言服务可以延后加载，避免影响诊断、脏状态或 Git 提示。

## 实施与复核

- 复用一个 hx 的活动 buffer，支持返回已有文件；关闭/销毁时去重清理会话。
- 将 Windows PTY 创建和其原生接收端口留在后台 isolate，转发输出、写入、缩放、退出码和终止命令。
- 随包引擎不再重复运行 `--version`，且复用激活时已写好的配置；外部引擎继续探测。
- trace 覆盖 Host 激活、PTY、首帧、可信状态、buffer 切换以及 hx 配置、VCS、文档就绪阶段。设置 `OPENMUSE_HELIX_TRACE=<jsonl 路径>` 即可启用；设置 `OPENMUSE_HELIX_REUSE=0` 可复现旧的逐文件进程路径。

本机 trace：`target/helix-baseline.jsonl`、`target/helix-worker-final.jsonl` 与最终交付构建的 `target/helix-delivery.jsonl`。已通过插件 `flutter analyze`、真实 Windows hx 的 PTY/复用/主 isolate 响应测试，以及 Release 应用的活动标签和 PID 核对。WPR 的系统性能策略启动返回 `0xc5585011`，故本次使用已有应用 trace 基建。当前 Windows 图像捕获未能显示 Flutter 绘制区域，视觉核对以 UI 可访问树中的活动标签及真实 PTY 状态为准。
