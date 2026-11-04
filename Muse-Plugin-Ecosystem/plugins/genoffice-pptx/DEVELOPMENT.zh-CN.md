# GenOffice PPTX 开发、更新与验收

## 环境及目录

Node 22+、npm、Python 3。独立开发不需要 Muse-Client 源码。首次克隆：

```sh
git submodule update --init --recursive -- plugins/genoffice-pptx/vendor/genoffice
cd plugins/genoffice-pptx
npm ci
npm run pack:desktop
npm test
```

- `vendor/genoffice/`：固定上游原版代码，更新通过 submodule 提交完成，不在提取副本中改写渲染逻辑。
- `web/src/runtime.ts`：插件自己的浏览器 preload 胶水、文件生命周期和 MCP Apps SDK。
- `web/src/font-metrics.ts`：原版 OpentypeMetrics 与可分发字体绑定。
- `web/shims/`：浏览器与 Electron/Node 环境的最小绑定；不能加入 Host 私有命令。
- `src/server_factory.ts`：标准 MCP 工具及 UI resource 声明。
- `dist/genoffice-pptx-0.4.0.mcpb`：同一插件 ID、同一审核包，在 Desktop 和 Android 使用。

每次改业务或资源必须提升 `package.json` 版本、同步服务器/App SDK 版本、重新生成 lock 与包。不可覆盖已经审核的版本字节。旧 `flutter_renderer/` 不在新包内，也不应接入 Host。

## 原版基准及回归

```sh
node scripts/build_reference.mjs
node dist/reference.cjs /path/Sample_ppt.pptx dist/reference-tree.json
GENOFFICE_WEB_ENTRY=web/src/reference-ui.tsx GENOFFICE_WEB_OUT=dist/reference-web node scripts/build_web.mjs
node scripts/render_reference.mjs dist/reference-pngs
```

脚本生成原版 Konva 的全部参考 PNG 后：

```sh
node scripts/verify_web.mjs /path/Sample_ppt.pptx dist/verification dist/reference-pngs
```

未传参考目录时仍检查全部页渲染、真实文本编辑、标准 download-file 保存和重开，并输出 Desktop/Mobile 视口截图。Chrome 路径可通过 `CHROME_PATH` 设置。原始样本不被修改；编辑结果只写输出目录。命令行 PDF 导出仅作额外测试，不能代替屏幕布局基准。见 `DESKTOP_PIPELINE.zh-CN.md`。

## 上传、审核和发布

公开市场 `https://plugins.openmuseai.com`。管理写接口仅绑定服务器回环地址。使用已有运维身份建立隧道：

```sh
ssh -N -L 18795:127.0.0.1:18794 <user>@<market-host>
```

构建并提交（默认选择兼容矩阵最新客户端）：

```sh
python3 scripts/release.py \
  --registry http://127.0.0.1:18795 \
  --public-origin https://plugins.openmuseai.com \
  --remote https://plugins.openmuseai.com/pptx/mcp
```

令牌通过 `MARKET_DEVELOPER_TOKEN` 或安全交互输入，不能提交到 Git。`--build-only` 只构建；`scripts/publish.py` 可单独上传现成包。流程是 `PUT /packages/{sha256}.mcpb` → `POST /v0.1/publish` → `pending`，不会自动通过审核。

部署前必须从已审核包提取 UI：

```sh
npm run build:http
python3 scripts/stage_service.py dist/genoffice-pptx-0.4.0.mcpb
```

建议对审核包的实际 UI 再执行验收，而不是使用后续的开发构建：

```sh
MCP_APP_HTML=dist/release-service/dist/web/index.html \
  node scripts/verify_web.mjs /path/Sample_ppt.pptx dist/reviewed-verification dist/reference-pngs
```

将 `dist/release-service/dist/` 部署到独立 PPTX 服务目录。不能再从后续 `npm test` / `build:web` 的 `dist/web` 拷贝 UI：构建器可能生成等价但字节不同的输出，安装校验会拒绝。生产配置见市场的 `production.compose.yaml` 中 `pptx` 服务：只监听回环 18797，有内存/CPU/PID 预算。Nginx `/pptx/mcp` 转发到该服务 `/mcp`。**远端 resource HTML 必须与审核包的 web/index.html 字节相同**；不热换 UI。

审核员核对来源、许可、SHA、逐页差异和两端实机证据后，使用独立市场后台，或：

```sh
python3 scripts/review.py <id> approve \
  --registry http://127.0.0.1:18795 \
  --note 'Checked original-pipeline rendering, edit/save/reopen and both native hosts'
```

公开 `GET /v0.1/servers` 只展示已上架版本。`review.py <id> delist --note ...` 下架；`relist` 恢复。已有用户保留本地安装，不远程删除其文件。

## 两端人工生命周期验收

1. 安装本次**完整主应用**，不是 smoke harness；记录 App/APK 和 MCPB 摘要。
2. 在生产市场刷新，安装/升级 GenOffice PPTX 0.4.0。
3. macOS：Workspace 打开样本；原版工具栏、左侧 23 页缩略图、首屏和第 4/23 页正常。编辑文字/备注、保存、关闭重开。
4. Android：市场安装后，从工作台 Workspace 点同一文件，或市场“打开/使用”选择文件。在应用内渲染和切页；断网后已安装 UI 仍运行。手机仅重排工具栏和缩略图，不另写 OOXML 解释器。
5. Mobile 从远端工作区打开的文档导出到 App Documents；本地文件选择器授予的路径可回写。回写到远端 Workspace 的统一资源写协议仍需单独验证，不能用“已导出”冒充远端保存。
6. 后台下架，刷新后新设备不可安装；已安装副本继续可用。重新上架后再次安装确认。

## 故障定位

- 空白：先看 JS 错误。WebKit 必须有安全文档来源（Web Crypto）和 sandbox 内的 Storage 适配；不要改 PPTX 引擎规避。
- MCP “connection closed”：自包含 UI 大于 10 MiB，Host 标准资源读取预算需足够；本次 128 MiB，包仍小于市场 32 MiB 限额。
- Android OOM：不要将大 HTML/base64 文件用 loadDataWithBaseURL 多次复制；使用 AssetLoader 流式离线资源。
- 无匹配打开方式：核对安装回执、标准 contentMediaType 发现缓存和 MIME 识别；不要按插件名称硬编码分发。
- 安装报 UI 不同：部署服务与审核包不是同一构建，必须发布新版本。
- 中文或字体差异：核对字体授权及可用字体；保留证据，不能伪称逐像素一致。

## 0.5.0 Agent 集成

View 工具、DSH 适配边界与更新流程见 [Agent 集成指南](AGENT_INTEGRATION.zh-CN.md)，真实 Desktop 对话、保存、重开、标签会话及生产发布证据见 [0.5.0 验收记录](evidence/0.5.0/ACCEPTANCE.zh-CN.md)。

## 0.6.1：大文件能力调用兼容性

`slides-read/v1` 的合同版本升级到 `1.0.1`。公开输入采用有长度上限的字符串及标准 `contentEncoding: base64` / `contentMediaType`，不生成重复分组的 base64 正则。此前约 5.5 MiB PPTX 经 Dart JSON Schema 校验器会触发正则栈溢出，被误报为校验深度超限。Provider 仍通过有大小上限的 Buffer 解码及 canonical base64 回编码比较严格拒绝非法输入，Host 不跳过 Schema 校验、不增加 PPTX 特判。原版 React/Konva 渲染管线保持不变。

跨插件回归必须经过真实 Host 能力网关，而不只直接调用 MCP Server：选择资源、绑定 Provider、校验工具输入输出，再检查 23 页样例结果。DevKit 应从市场更新 Provider 并重新解析依赖，旧锁文件不会悄悄改成新版本。
