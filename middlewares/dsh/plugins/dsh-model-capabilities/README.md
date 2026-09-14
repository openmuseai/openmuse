# dsh-model-capabilities

> 模型能力 —— 在 **Models 设置页原生卡片内** 为自定义 `llm-pi-ai` 模型配置：
> 思考强度档位（reasoningEfforts）、提供方默认思考强度（reasoning）、
> 支持模态（input / defaultInput）、网关兼容开关（compat）、
> 通用请求头（headers，按需手动添加；值支持 `{{session}}` 占位符）。
> 占位符在 wire 层**按 DSH 会话替换**为当前会话 id 的 SHA-256 稳定标识
> （opencode Go 会话亲和：一个 DSH 会话一个令牌，跨轮次不变）；
> 命中亲和 host 且未自建该头时也会自动注入。全部配置经官方 `settings.mutate`
> 写入 `settings.yaml`，**无需手改配置**。

A DeepSeek Harness (DSH) web plugin — Host + Web UI track.

- 挂载点：官方扩展位 `settings.models.provider-card`（keyed by `llm-pi-ai`），
  即 Models 设置页每张 pi-ai 提供方卡片内的适配器扩展区。
- 数据属主：`llm-pi-ai` 设置节（pi-ai 适配器）。浏览器面通过官方设置通路
  Host 同源 HTTP 桥（GET/POST `/model-capabilities`）经 `ctx.settings` 读写 —— 与官方
  同一套 revision 防冲突 + pi-ai config schema 校验
  （`assertServiceable`：非法的协议/档位组合会在写入处就被拒绝）。
- Host 面另有 wire 层会话注入（`llm/stream` waterfall + fetch 包装），
  不写入任何持久数据（`llm-pi-ai` 命名空间属主是 pi-ai 适配器）。

## 字段对照

| UI 控件 | 写入 settings.yaml |
|---|---|
| 模型 · 模态（继承 / 仅文本 / 文本+图像） | `providers.<route>.models[i].input` |
| 模型 · 思考强度（未设置 / 禁用 / 标准档位 / 自定义） | `models[i].reasoningEfforts`（`false` / 档位字典；`off` 可留空=不发送） |
| 提供方 · 默认思考强度 | `providers.<route>.reasoning` |
| 提供方 · 默认模态 | `providers.<route>.defaultInput` |
| 提供方 · 请求头（按需手动添加） | `providers.<route>.headers`（名称统一小写；值支持 `{{session}}` 占位符 = 每请求自动填入当前会话 SHA-256 标识；`user-agent` 由 DSH 归属头占用，不可设置） |
| 兼容设置：developer 角色 / reasoning_effort / 输出上限字段 / 思考格式 | `providers.<route>.compat.{supportsDeveloperRole,supportsReasoningEffort,maxTokensField,thinkingFormat}` |

## 安装

```sh
# GitHub 固定提交（推荐；先到 Releases/Tags 拿 40 位 commit，或直接用 tag）
dsh plugin --profile web add github:fuzz1og/dsh-model-capabilities#<40位commit>
# 例（tag 对应的提交同样可用 40 位 sha）：
#   dsh plugin --profile web add github:fuzz1og/dsh-model-capabilities#$(git ls-remote https://github.com/fuzz1og/dsh-model-capabilities.git refs/tags/v0.2.0 | cut -c1-40)

# 本地开发安装（从仓库根目录；保持目录在位）
dsh plugin --profile web add ./

# 验证组合与行解析
dsh --profile web --dump-config
dsh --profile web
```

安装后**重启 web profile**（Settings → 重启 或 `dsh --profile web`）生效。

## 使用

1. Settings → Models：添加/编辑一个自定义（llm-pi-ai）提供方，保存后卡片内出现「模型能力」卡片。
2. 展开每个模型行：选择模态与思考强度；需要非标准线上拼写时选「自定义」逐档填写（如 `max → ultra`）。
3. 需要随会话变化的请求头（如 opencode Go 的 `x-opencode-session`）：展开「请求头」
   →「添加请求头」，名称填 `x-opencode-session`，**值填 `{{session}}`**——每次请求
   由 DSH 自动替换为当前会话的 SHA-256 稳定标识；固定值则原样发送。点「应用能力配置」。
4. 点火失败（网关 400）时，在「兼容设置」里按上游文档修正，例如
   `developer 角色: 不支持（用 system）`、`maxTokensField: max_tokens`。
5. 点「应用能力配置」→ 写入成功显示绿色提示；冲突/校验拒绝会显示原因（冲突后视图自动刷新，可直接重试）。

## opencode Go 会话亲和（x-opencode-session，每 DSH 会话一个）

[opencode 官方文档](https://opencode.ai/docs/go/) 对 Go 套餐的使用方有三条要求：
不产生滥用流量、**正确标识自身（不用泛化 User-Agent）**、**携带 `x-opencode-session` 头**
（网关按会话做亲和路由并优化 prompt caching），否则**账号可能被标记**。

本插件的对应关系：

- **身份要求已由 DSH 满足**：`dsh-llm` 的归属头机制对每个提供方请求发送
  `user-agent: deepseek-harness/<版本> (+https://github.com/deepseek-ai/deepseek-harness)`，
  且 `user-agent` 列为保留头——用户配置不能覆盖它，也不需要伪装 opencode 客户端。
- **自建头优先：`{{session}}` 占位符（推荐的精确控制方式）**：在卡片「请求头」区
  自建一行（如 `x-opencode-session`），值填 `{{session}}`——**每次请求由插件在
  wire 层替换为当前 DSH 会话的 SHA-256 稳定标识**（`dsh-` + 12 位 base36，
  同会话跨轮次不变、异会话互不相同、不可逆推）。占位符可出现在任意头的
  任意位置（含多次出现）；对 host 名单没有要求（任何提供方都可用）；
  固定值则**原样发送**——自建值（占位或固定）优先于自动注入。
  非会话流（无 sessionId 可用，如模型目录拉取）中，占位符用该 host 最近
  会话的令牌兜底；完全无令牌可用的极端情况下该头被**丢弃**（绝不把字面
  `{{session}}` 发给网关）。
- **自动注入（未自建该头时的缺省）**：提供方 `baseURL` 的 host 命中亲和名单
  （默认 `opencode.ai`，含子域）且卡片里未自建 `x-opencode-session` 时，
  wire 层自动盖写同一个会话令牌。三块机制：`llm/stream` waterfall（跨插件
  须 global 注册）读每流 `sessionId`（dsh-agent-loop 注入）→ **同步**登记
  per-host FIFO 台账 → `globalThis.fetch` 包装消费并盖章。配对依据：适配器
  `maxRetries: 0`（每流恰一次 wire fetch）且 waterfall 回调先于该流 fetch
  同步执行；队空回退 host 最近令牌（标题生成等辅助调用延续同桶）。**并发
  边界（如实说明）**：同 host 异会话的流真正并发交错时 FIFO 可能瞬时互换
  令牌，下一次请求自纠；AsyncLocalStorage 实测会在 pi-ai 适配器内部
  await 链丢失，故不采用。三条协议路线（openai-completions /
  openai-responses / anthropic-messages）都走同一 wire，全部生效；
  非会话出站请求（网页检索等）不受影响。
- **与同类插件的关系**：与 dsh.pub 上 `dsh-opencode-session-id` 类 wire 层注入插件
  功能重叠；同时安装时后安装的 fetch 包装在外层生效（会互相覆盖同名头）。

## 配置

默认零配置。在 profile 的 `cordis.patch.yml` 中可按 id `model-capabilities`
覆盖行 config（覆盖为**整值替换**，非深合并）：

```yaml
- id: model-capabilities
  name: 'dsh-model-capabilities'
  config:
    hosts: ['opencode.ai', 'my-mirror.example']   # 追加会话注入的 host 后缀（默认已含 opencode.ai）
    disableSessionAffinity: true                   # 关闭 wire 层会话注入（仅保留手动请求头）
```

## 兼容性

- 目标 DSH：`0.1.2-rc.1`（实测运行中）；`0.1.3-alpha.1` 经源码级核对：
  `settings.models.provider-card` 槽位契约、`llm-pi-ai` 设置节 schema、
  settings 服务通路均无变更（仅 discovery 增强，与本插件互补不重叠）。
- `providers.<route>.headers` 字段在 `0.1.2-rc.1` 的 pi-ai schema 中源码级核实
  （`z.dict(z.string())` + `assertValidHeaders` Fetch 合法性校验；经
  `requestHeaders(profile.headers)` 最后合并，`user-agent` 为保留头）。
- 会话注入链路在 `0.1.2-rc.1` 源码级核实 + 本地 mock 网关实测：agent loop
  请求带 `sessionId` → `LlmRuntime.stream` 经 `llm/stream` waterfall（跨插件
  监听须 global 注册；入参冻结只读）→ pi-ai 适配器以 `requestHeaders(profile.headers)`
  自建头且 `maxRetries: 0`（每流恰一次 wire fetch）→ waterfall 同步登记 +
  fetch 包装消费的 per-host FIFO 台账；实测不同会话令牌不同、同会话
  （含标题生成等辅助调用）令牌相同。
- 依赖客户端运行时与官方 `settings.models.provider-card` 槽位（0.1.x 系列）；
  若上游改列槽位协议，需按新契约调整注册。
- UI 基于官方 `@deepseek-ai/dsh-client-ui-primitives`（Button / Pill / Input /
  Menu / DisclosureRow / StateDot）与 `--dsw-*` 令牌，浅色/深色自动跟随应用主题。

## 卸载 / 停用

```sh
dsh plugin --profile web remove dsh-model-capabilities
# 或仅禁用行：在 profiles/web/cordis.patch.yml 按 id 覆盖
# - id: model-capabilities
#   name: 'dsh-model-capabilities'
#   disabled: true
```

## 开发

```text
lib/index.js     Host 面（最小）
lib/client.js    Web 客户端（window.__ModuleLoader__ 懒加载 CJS factory 产物）
cordis.patch.yml Bundle patch（挂载行）
```

- Host 面无构建步骤（提交即产物）。
- 客户端按 DSH web 客户端模块系统的 factory 契约手写生成：`window.__ModuleLoader__.load({ id, factory })`；`require('react')` / `require('@deepseek-ai/dsh-client-ui-primitives')` 在浏览器端由模块系统解析。
- 修改后只需替换 `lib/client.js` 并重启 profile。

## License

MIT
