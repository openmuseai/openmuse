# 可选代码导航：M1 实施记录

日期：2026-11-15。源代码提交：`50bf7f5f`。范围：独立服务内核与语言进程生命周期；不是 Mobile 功能已发布声明。

## 已实现

代码位于 [plugins/code-navigation-service](../../../plugins/code-navigation-service/README.zh-CN.md)。

- 独立 headless plugin lifecycle：activate / deactivate、enable / disable / stopAll，默认关闭，启用不启动语言进程。不依赖 DSH/Flutter，不开放网络监听。
- 受信资源 owner seam：执行时读取、版本/坐标检查、查询后及目标映射后校验；输出只经 owner 逐个授权的资源引用。语言服务命令只来自宿主 profiles。
- 账号/授权代际分区 + workspace + language profile 的惰性实例池；默认 2 个进程组、8 个请求，实例内串行，空闲 5 分钟回收。实例满时仅淘汰空闲实例，全忙返回 BUSY。
- 5 秒热请求 / 15 秒冷请求截止、排队取消及时返回、保留排队 admission 直到清理完成以避免反复取消造成无限堆积；初始化/查询取消后清理实例并阻止迟到结果。
- 固定 vscode-jsonrpc 8.2.1 stdio backend，支持 definition / references / implementation / hover，校验 UTF-16 与文档同步能力，归一化 Location/LocationLink，默认 1MiB 源文档 / 100 目标并标记截断。
- 优雅退出后清理语言服务器进程组及后代；停止失败保留可见实例并允许重试。无 shell 启动，不继承模型凭据环境；服务不执行服务器发起的 workspace edit。
- 按请求生成 traceId 的 queue/initialize/query/total 耗时，仅记录时间、状态、冷/热与实例数，无代码或绝对路径。跨设备 trace 父子关系仍待 M2 adapter 接线。

## 测试

`npm test`：21 项通过，包含真实 subprocess 的正常退出、worker 后代清理、无响应超时和 UTF-8 编码拒绝。其余用受控 backend 验证并发/限流、启用惰性、版本、撤销、目标授权、截断、空闲与停止失败。

`npm run test:language`：1 项真实 TypeScript LSP 测试通过，包含多次查询：中文文件定义、引用、hover、emoji 前缀之后的 UTF-16 坐标和进程复用。TypeScript 5.9.3 / typescript-language-server 6.1.0 均锁定为 devDependency，不作为产品自动安装语言集。未验收 Dart 或 Windows 实际语言服务。

既有基线回归：资源 trust 7、RPC/相对链接 2、共享 Viewer 5、DSH bridge 23 项均通过。Viewer 首次重跑遇到已有 native hook 缓存引用缺失 dylib，删除该包单个生成的 output.json 后重建通过，未改第三方依赖缓存或源码。

## 本地性能基准

报告：[2026-11-15-local-service-benchmark.json](evidence/2026-11-15-local-service-benchmark.json)。命令：

```sh
node plugins/code-navigation-service/tool/benchmark.mjs docs/design/code-navigation/evidence/2026-11-15-local-service-benchmark.json
```

macOS arm64 / Node 24.16.0；固定两文件、三行源代码样例，中文目标文件。30 次新进程查询与 30 次同进程热查询，逐次验证目标确为定义文件，结束时实例数为零。报告包含逐次时间、P50/P95/最大值及服务源码 SHA-256。

这只是本地服务基准，文件系统缓存已热；不包含 Mobile 触摸、网络、资源 RPC 或预览渲染，不代表大型仓库、端到端 P95 或内存/CPU 目标已达标。实际数值以 JSON 为准。

## 剩余发布门禁

M2：统一资源能力与安全 RPC、DSH/挂载目录 adapter、generation/版本/目标引用验证、跨设备 trace。

M3：独立插件装配与设置持久化/开关确认、能力发现、Mobile/ Desktop 触摸预览、范围定位、导航历史、符号目录与错误状态。

M4：国内 Desktop + OPPO 正常版 UI 验收、大型仓库冷/热测试、RSS/CPU 观测、传输级大响应限制、Windows 进程树清理、产品发行 SBOM 与语言配置门禁。现有后端受信语言程序不是操作系统沙箱；不把进程数上限称为硬内存限制。

## 工作区与提交范围

当前分支保留原工作区，不新建 worktree。已将明确归属的资源 owner 定位、共享 Tree-sitter、DSH bridge、Viewer handler、DSH source 和以前的 OPPO 验收证据拆成独立提交。SDK 混合文件只提交相关补丁。大量协作编辑、发布、文档重组等未提交改动保持原状，未混入本功能，也未执行 push。
