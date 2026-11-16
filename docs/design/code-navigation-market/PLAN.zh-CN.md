# 首期代码导航插件市场：产品与实施方案

日期：2026-11-17。范围：国内 Desktop + Android 管理同一资源宿主，代码导航服务、Dart、TypeScript/JavaScript 三个独立可安装项。复用统一资源引用与现有市场，不为 Mobile 增加语言进程。

## 产品流程

市场展示三张卡片，明确“安装到当前 Desktop”。语言扩展依赖代码导航服务，安装前展示依赖，服务默认不启用。安装只落盘并校验固定发行物，不启动 LS。详情页提供配置、检测、启用/关闭、卸载。手机远程操作必须等待 Desktop 返回真实状态。

语言配置表单包含 SDK/程序路径、参数、initializationOptions、configuration 和文件类型；内置语言模板优先，高级项经有界 Schema 校验。首期 Dart 使用宿主已有 Dart SDK，TS/JS 使用发行锁定的 TypeScript 和 typescript-language-server；不在一次代码点击中联网下载。配置中的机器路径保留在所选宿主，不同步为手机路径。远端不接受通用 shell 命令；语言扩展只允许其明确声明的可执行类型/参数。

安装、启用、运行是独立状态。服务和语言扩展都启用且当前 workspace 明确允许后，首个查询才启动 LS。关闭/卸载/修改启动配置先撤销旧请求、等待语言进程释放，再保存配置并返回成功。服务仍有已安装语言扩展时拒绝卸载并显示依赖；先卸载语言扩展，再卸载服务。卸载仅删除插件私有目录与策略，不删除 workspace 或 SDK。

## 分层及协议

1. 发行 catalog 与 artifact：三个稳定插件身份、固定版本、平台适配、依赖、摘要和许可证。首期进入应用签名覆盖的 curated market 快照；不伪称已发布到公共 npm/MCP Registry。远端市场发布使用既有签名与发行链路。
2. 独立插件管理器：不依赖 Flutter/DSH，管理按账号隔离的安装目录、原子 receipt、配置 revision、启用状态和检测。服务与语言 profile 经同一 registry 发现。
3. DSH 仅提供 resource owner adapter 和 token-gated 管理转发；源路径与 workspace 继续由宿主解析。现有 ResourceCodeNavigationProvider / 窄 RPC 不承载配置命令。
4. Desktop market dispatcher 使用独立管理接口；Mobile 经既有 paired grant 调用相同接口。请求正文不能选择账号、改变资源宿主、提交执行代码。管理请求前后重验账号/grant；配置大小有界，错误返回稳定代码。
5. 共用 Flutter 设置页面由插件拥有，Desktop/Mobile 均复用；Host 只提供路由与授权。冻结 manifest 不增加未经版本化的顶层字段。

管理操作：list / plan / install / configure / detect / enable / disable / uninstall。配置更新使用 expectedRevision，冲突拒绝；检测不等于启用。检测验证实际 SDK/LS 身份和 stdio initialize 能力，并清理临时进程。查询、检测、启停、配置诊断只记录阶段/耗时/状态，不记录代码、凭据或完整命令环境。

## 状态与性能门禁

- 未安装、已安装未配置、配置待检测、可用未启用、已启用待查询、运行中、停止中、检测失败分别呈现。
- 应用启动/安装/配置/启用均为零持久 LS 进程；关闭/卸载后无所属进程组遗留。
- 默认最多 2 个实例、8 个请求，5min 空闲释放；热查询 5s、首次 15s 截止，复用现有服务边界。
- 配置请求上限 64KiB，结果上限 1MiB；禁止无限日志/候选/队列。
- 安装和配置 UI 即时显示局部进度；检测最长 15s；管理变更等待停止确认。目标管理首帧反馈 <100ms，实测结果单独记录，未测量不声称达标。
- 真实冷/热语义准确性先验证 Dart 与 TS/JS 中文跨文件；性能 P95/RSS/CPU 继续使用既有产品门禁，不能用单次 UI 成功替代。

## 提交与验收顺序

A. 本方案落盘。B. 独立 catalog/artifact/安装配置管理器及生命周期测试。C. owner adapter 与授权管理链路。D. 共享设置 UI、双端市场入口。E. 国内正常 release 构建和安装。F. Desktop + 已连接 Android 真机：安装三项、编辑配置、实际检测、启用后零进程、首次定义查询、返回、关闭释放、按依赖卸载、重新安装；核对同一国内账号并保留正常版。

测试包含摘要不符、依赖卸载拒绝、配置冲突、账号隔离、远端越权、并发变更、停止失败不误报、无 SDK/错误路径、真实 Dart/TypeScript 检测和定义。验收保存脱敏状态和原始截图。工作区已有大量不相关修改，只暂存本任务补丁。
