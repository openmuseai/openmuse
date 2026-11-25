# 可选代码导航：技术方案

日期：2026-11-15。关联：[产品稿](PRODUCT.zh-CN.md)、[实施计划](PLAN.zh-CN.md)。

2026-11-16 更新：通用可选能力/RPC、DSH 本地 owner adapter 和 Viewer 导航已接通；当前实现与未达门禁以 [验收记录](ACCEPTANCE-2026-11-16.zh-CN.md) 为准。下文“当前事实”是最初设计时的基线。

## 当前事实与架构决策

当前资源框架为 muse_resource_center → ResourceProvider/ResourceReference → Plugin SDK 会话 → open-file-viewer。已有 owner-resolved locateRelative、账号/代际校验、跨设备资源 RPC、行号定位与 native Tree-sitter 高亮。符号点击、语义查询、目标范围、导航历史尚未接通。

仓库保留 @deepseek-ai/dsh-lsp / dsh-lsp-stdio 0.1.0-rc.7 tarball，但当前 third_party/dsh/node_modules 的 DSH 为 0.2.0-rc.2，未安装这些 LSP 包。因此不能直接把旧 seam 作为当前发布依赖；不降级/修改 DSH 来启用它。

采用独立 Node 服务插件 plugins/code-navigation-service。运行时无需 DSH、Flutter、模型、网络端口或登录凭据。使用固定版本 Microsoft vscode-jsonrpc 承载标准 stdio LSP，不自行实现协议 framing。语言程序由宿主显式配置，独立安装；开发测试依赖不进入产品默认语言集。参考：[Microsoft 实现](https://github.com/microsoft/vscode-languageserver-node)、[LSP 3.17 模型](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/metaModel/metaModel.json)。

## 分层

1. Viewer 插件：触摸命中、选择、预览、导航栈和局部状态，不能拥有进程或语言命令。
2. Resource Hub / SDK：可选代码导航能力、确切 source 路由、鉴权、取消与资源引用验证；Host 不硬编码语言或 DSH。
3. 资源宿主 adapter：按 source reference 找到 authoritative workspace、文件、版本；独立授权目标并返回统一资源引用。DSH 只是其中一个 adapter，不依赖 session 当前显示目录猜路径。
4. 独立代码导航服务：策略、实例池、队列、截止时间、生命周期、能力与结果归一化。
5. backend：标准 LSP stdio；未来可替换为容器 LSP / SCIP / 搜索 Provider。精确/语法/文本搜索结果分别标记。

启动时仅注册插件和能力。默认关闭；启用只改变策略。首次请求创建实例，按账号授权分区 + canonical workspace + language profile + generation 单飞。进程运行在资源宿主的执行世界；本地 Node adapter 不能解释容器路径，容器要提供自己的 backend / resource adapter。

## 服务接口与资源边界

调用：query({reference, version, operation, position, requestId}, {signal, access})。

operation：definition / references / implementation / hover。坐标统一零基 UTF-16；Viewer 展示层行号一基，仅在边界转换。没有通用 executeCommand 或任意 JSON-RPC 透传。

服务构造时安装受信 resourceOwner 与语言 profiles。owner.authorizeAndRead 在请求执行时恢复 {partition, workspaceKey, rootUri, uri, languageId, profile, version, text}；请求不能设置宿主路径/命令。owner.assertCurrent 在查询后和目标解析后再次验证授权及版本。owner.resolveLocation 对每个候选重新授权，返回 {reference, version, range, selectionRange, label}；未经授权的外部/虚拟 URI 不回传。

远端 adapter 的 access 来源于现有 Resource RPC peer 授权，不接受正文传入的伪造账号。所有请求包含当前 generation。权限变更/退出账号触发该分区停止并撤销迟到结果。未来 RPC 扩展必须在 await 前后校验 peer，且只提供 narrow navigation/status/configure/cancel，不开放 language command。

只对匹配版本只读文件提供精确导航。首期不管理编辑器未保存 buffer；不匹配返回 VERSION_MISMATCH。历史快照、跨仓库依赖需资源源单独提供版本与授权，不能退回最新文件后仍称精确。

## 生命周期与负载

状态：disabled → idle → starting → ready/busy → idle → stopping → idle；故障退回可诊断状态。关闭进入 disabled，撤销 epoch、取消运行/排队、清除 idle timer、等待启动完成或中止、终止全部后代。停止全部保留 enabled 策略；切换账号等价于关闭对应分区。

每实例串行化 didOpen/query/didClose，避免不同快照覆盖同一 document。复用进程，不复用未保存的文档状态。选择 UTF-16 并检查 initialize 的 positionEncoding / sync 能力。读取配置只允许预设值，拒绝服务器发起的 applyEdit/executeCommand；不静默下载依赖或联网安装。

默认 maxInstances=2、maxPending=8、idleMs=300000、queryTimeoutMs=5000、startupTimeoutMs=15000、maxDocumentBytes=1048576、maxResults=100。池满优先淘汰空闲实例；全忙返回 BUSY。首期不建立无限的全库索引与预热任务。

运行取消通过 JSON-RPC cancellation token 转发；不能保证第三方服务器响应取消，因此超时/运行中取消会停止该实例以解除队头阻塞。新请求可重启，但本次请求不自动无限重试。进程启动失败不能影响其他 workspace。使用无 shell 的注册命令与参数；POSIX 进程组终止，Windows 采用树终止并需独立验收。

RSS/CPU 是观测指标而非硬隔离承诺。OS/容器内存上限由执行 Provider 实现；首期先实现进程数/并发/文档大小/结果数/截止/空闲硬边界。大响应消息的传输级上限、后台 RSS 采样及语言服务器恶意输出门禁必须在发布前落实。

## Trace 与故障

跨设备贯通 requestId/traceId：tap → hitTest → rpc → authorize/read → queue → start/initialize → query → targetAuthorize → previewRender。至少记录 cold、queueMs、initializeMs、queryMs、totalMs、resultCount、truncated、cancel/timeout/disabled、实例数量。

默认 trace 只含阶段、时间和状态，不含源码、hover 正文、绝对路径、凭据。冷启动失败区分未配置、可执行文件缺失、initialize 不兼容；无定义不是服务错误。取消结果不弹故障。

## 发布门禁

单元与真实 subprocess 测试验证资源状态机；真实 TypeScript LSP 跨文件 smoke 单独计数。fake LSP 只能证明 wire 与故障行为，不能冒充语言准确率。完整 Mobile 功能必须经过 generic RPC/DSH adapter/Viewer UI 接线后，再在国内 Desktop + OPPO 正常版做 E2E；服务包测试不代表 UI 通过。
