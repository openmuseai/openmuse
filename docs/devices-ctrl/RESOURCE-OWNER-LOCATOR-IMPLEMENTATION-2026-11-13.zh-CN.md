# 资源宿主解析与统一打开：实现及验收记录

日期：2026-11-13。环境：国内。测试账号为本次对话指定的账号；本文及证据不保存密码、令牌、认证文件或真实文件正文。

后续实现已补齐 Tree-sitter 与查看器文内文件链接，见 [后续实现及验收记录](FILE-VIEWER-TREE-SITTER-LINKS-2026-11-13.zh-CN.md)。本文保留上一阶段的范围和验收快照。

## Review 结论

修改前仅有部分基础能力，尚不具备完整的不完整路径解析链路：

- Flutter 对话正文只处理 canonical `openmuse-resource`；普通文件链接没有接入。
- 交付文件卡片请求 Desktop 本机 opener，不会把资源交给 Mobile 查看器。
- DSH `workspaceFiles` 可以读取执行环境文件，但没有路径后缀搜索、歧义选择与统一资源身份之间的适配。
- Host 的目录挂载和会话持久化 header 的 cwd 可能不同，不能由 Mobile 或当前 Host 选中目录推断历史会话的根目录。

本次使用已有资源中心、ResourceProvider、ResourceReference、资源 RPC 和插件 handler。没有另建 Mobile 专属的资源管理体系，也不要求资源来源必须是本地目录。

## 已实现

### 插件贡献和通用定位协议

- `ResourceLocator` 是给已配置资源宿主的未受信提示：owner/kind/target/context。它不是 ResourceIdentity，也不携带 endpoint 或授权。
- `ResourceLocatorProvider` 是 ResourceProvider 的可选能力。Hub 只路由，具体插件解释其私有 context 与 target。
- `OpenMuseResourceSourceContributor` 随插件激活注册资源源，停用时移除；账号关闭后不再注册。
- Host 提供账号隔离的私有状态存储。插件自行维护资源索引。
- 通用 RPC `locate` 继续使用现有认证通道，验证账号、workspace、来源、generation、有效期及结果所属 source；不根据提示扩大授权。
- 多来源无法唯一确定 owner 时失败，不跨设备静默搜索或猜测来源。

### DSH 宿主解析

`openmuse-dsh-bridge` 0.1.21 实现定位和有界读取，DSH Dart 插件将结果发布成统一资源引用：

1. 从 `sessionPersistence.stat` 获取会话权威 cwd，无需载入完整历史。DSH 当前 `page` 返回历史记录，不返回 header，且需要 throughSeq；不能用它获取 cwd。
2. presented/tool 优先从原始事件定位，不信任调用方替换的路径；tool/result 按 callId 找原始 tool/call 参数。
3. 支持 file URI、DSH session/absolute file address、相对路径、行列后缀和 `#L`。跨 session 的文件地址拒绝。
4. 精确路径优先；显式的、同会话的文件句柄可提供相对文件上下文。
5. 精确失败后进行路径后缀、文件名匹配；只在当前会话 workspace 搜索，跳过常见生成目录。搜索限制为 3000 个条目、20 个结果、约 1.5 秒；底层单次目录操作受读取请求超时限制，不把该搜索预算误称为硬实时保证。
6. 多匹配展示候选相对路径；搜索被截断时，即使只找到一个，也要求选择，不能宣称已证明唯一。
7. 通过 DSH 文件系统的 resolve/contains/stat/readBytes 执行，不把 sandbox 路径交给 Mac `File()`。校验根目录身份、符号链接、读取后的目标及当前会话根目录；完整读取上限 16 MiB。

资源身份与短期读取句柄分开：统一资源 ref 由插件生成随机 opaque 身份，并持久化 session 和宿主确认的相对定位器；临时 bridge 句柄过期或重启后，使用精确 canonical 定位重新获得句柄。canonical 重开禁止模糊搜索，防止原文件消失后误开同名文件。内容版本由 digest 校验，不伪造旧版本。

### 两端打开与查看器

- Desktop 的 DSH 文件打开桥接传递 session 身份和原始提示，不再由 Web 客户端提供 cwd。
- Flutter 对话的 Markdown 文件链接和行内代码文件路径进入同一资源路由。工具文件有结构化打开入口。
- Mobile 交付卡片默认进入统一资源查看器；显式 reveal 操作保留原 Desktop 行为。
- `open-file-viewer` 提供通用资源 handler，直接接收资源中心验证过的只读内容；其他宿主提供的相同媒体类型也可使用它。
- 行号由宿主解析后传入查看器，支持只读行定位；列号保留在导航数据中。
- Trace 使用现有 ConnectionTrace，新增 `resource.owner.resolve/read` 的 ready/response/decoded/end；只记录耗时、状态及字节数。

本次没有集成 tree-sitter，也没有实现通用远程目录定位或大文件分页。这些能力与宿主定位接口解耦，不能据本轮验收宣称已经完成。

## 验证

### 自动检查

- bridge Node 测试：22 个通过。覆盖真实临时文件、精确读取、后缀和文件名歧义、原始交付记录、跨 session 拒绝、目录穿越、符号链接越界、未知句柄、canonical 不模糊重绑定及显式文件上下文。
- 资源 RPC/跨端既有测试：10 个通过，包括新增非文件系统 owner、跨账号/workspace 拒绝和等待期间撤销。
- Flutter 文件链接 UI 测试：3 个通过。
- 既有会话 UI 测试：15 个通过。
- 查看插件测试：5 个通过。
- DSH source 真实 HTTP 测试：1 个通过；覆盖句柄过期和 source 重建后身份稳定。
- 插件源生命周期测试：1 个通过。
- 共享查看器行定位及原有测试：2 个通过；以上共 59 项相关测试通过。

### Desktop 实际 UI

使用指定账号登录国内正常版，真实 DSH 会话回复了精确、不完整、同名三个文件链接。验收使用隔离的合成文件，不使用真实业务文件正文。

- 点击 Exact：查看器显示 `ALPHA_FILE_OWNER_RESOLUTION_OK`。
- 点击 Incomplete（只有 `unique.txt`）：查看器显示 `UNIQUE_FILE_OWNER_RESOLUTION_OK`。
- 点击 Ambiguous（只有 `hello.txt`）：先展示 alpha/beta 两个相对路径；选择 beta 后显示 `BETA_FILE_OWNER_RESOLUTION_OK`。
- 启用持久化 source 的构建以及最终交付构建再次点击 Incomplete，均打开正确内容。

以上是实际可见 UI 结果，不是仅检查 HTTP 成功。

### Mobile：Android 模拟器真实服务端到端 UI 通过

用户已明确同意在测试模拟器上接受首次协议。验收入口调用生产 `app.main()`，使用指定账号的正常认证控制器登录，连接国内 `cn.openmuseai.com` 上的真实 Desktop；没有 mock 文件来源、会话内容或资源 RPC。密码通过仅存在于测试入口的临时 loopback 服务在内存中传入，未编译进 APK，也未保存到仓库或证据。

- 通过正常绑定面板选择 Desktop，通过底部 UI 进入任务页；会话选择使用生产控制器。
- 实际点击同一会话的 Exact 链接，断言可见 ALPHA 标记。
- 返回后实际点击 Incomplete 链接，断言可见 UNIQUE 标记。
- 实际点击 Ambiguous 链接，确认 alpha/beta 两个候选均可见且可点击；选择 beta 后断言可见 BETA 标记。
- CUA 镜像另外观察到 Mobile 只读查看器显示 `BETA_FILE_OWNER_RESOLUTION_OK`，宿主标签为实际 Mac。
- 最终测试构建 111317 的完整单次运行退出码为 0，报告 `All tests passed`。测试入口、文件链接点击和查看器内容断言均包含在该运行内。

调试轮次没有计为通过：早期测试在中文返回按钮、首次协议处理、绑定面板遮挡和页面/选择面板动画等待上失败。最终脚本等待真实可点击状态，没有关闭点击警告或跳过候选断言。既有相关组件测试 59 项与本次真实服务 UI 场景分别记录。

最终成功轮次 trace 中，Desktop 宿主解析 3 次为 2.936–28.819 ms，读取 3 次为 4.905–10.207 ms；Mobile 配对约 622 ms、DSH bootstrap 约 428 ms、首个对话帧约 506 ms。它们是不同阶段的单轮样本，不能简单相加，也不能作为冷/热启动分位数或完整点击到展示耗时。原始无正文 trace 保留全部事件，包括测试结束时被取消的会话轮询请求。

### 真机及其他平台边界

- OPPO PLP110 / Android 16 已同签覆盖更新国内正常 release 111318，保留应用数据；整个过程未安装测试入口。用户已确认账号并进入任务页。
- 真机镜像点击仍不可靠，未完成真机三类文件点击的独立可见验收。模拟器通过不能替代真机通过。
- 两台 Android 测试模拟器已恢复为正常 release 111318，与真机签名相同；检查均无 DEBUGGABLE 标志，正常 APK 不含本次集成测试入口标记。
- iOS 模拟器尝试构建失败于既有原生依赖 `muse_siyuan` 缺失，未计通过，也未移除插件规避链接失败。

## 证据与交付

- `evidence/2026-11-13/resource-owner-ui-acceptance.json`：区分 Desktop、Android 模拟器通过及真机未完成。
- `evidence/2026-11-13/mobile-resource-e2e-result.json`：真实服务 UI 成功运行报告。
- `evidence/2026-11-13/mobile-resource-e2e-trace.jsonl`、`desktop-resource-mobile-e2e-trace.jsonl`：该运行两端无正文 trace，仅保留固定诊断字段。
- `evidence/2026-11-13/desktop-resource-trace.jsonl`：开发过程的无正文 trace，包含修复前失败，不能只挑成功事件作为总体指标。
- `evidence/2026-11-13/desktop-resource-final-trace.jsonl`：持久化 source 构建的 trace。
- `evidence/2026-11-13/desktop-resource-delivered-trace.jsonl`：最终交付构建的 trace。
- `third_party/dsh/tarballs/openmuse-dsh-bridge-0.1.21.tgz`：闭包依赖已更新到同一版本；源码与打包模块一致。
- Mac 安装位置：`/Applications/OpenMuse.app`；保留原安装包备份 `OpenMuse.before-locator-20261113.app`。
- Android 正常版为 1.0.1（111318）；正常 APK：`app/openmuse_mobile/build/app/outputs/flutter-apk/app-release.apk`，发布签名指纹与原安装一致。测试用签名 APK 和临时认证材料已清理。

仓库原有大量其他修改，本轮没有清理、提交或覆盖这些工作。只读文件试验数据保留在明确命名的 acceptance 目录和会话中供继续验收。
