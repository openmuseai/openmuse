# 打包方式对比：当前 DSH Office Windows 包 vs 官方 SDK 运行时（pkg --sea 单文件 exe）

> 结论先行：官方做法**不是**把整棵 node + node_modules 拷进安装目录，而是
> 「生产闭包（pnpm deploy）+ `@yao-pkg/pkg --sea` 打成单文件 exe」。
> DSH Office 当前是「官方 Node zip + 整棵 harness（含全部 dev 工具与上百 MB 原生包）」，
> 实测 `muse/` 约 **2.0 GB**；官方闭包 exe 是 **一两百 MB 量级** 的单个可执行文件。
> 本文逐个环节对比两条链路，并给出 Office 侧对齐 SEA 的具体改法。

参考实现都在 `vendors/deepseek-harness`（下文简称「harness」）：

| 文件 | 作用 |
|---|---|
| `scripts/build-exe-for-python-sdk.ts` | 构建入口：校验闭包 → `pnpm run build` → `pnpm deploy` → `pkg --sea` |
| `python/sdk-runtime/package.json`（`dsh-python-runtime-closure`） | 打进 exe 的插件清单；加一行依赖再打包 = 加一个预制插件 |
| `python/sdk-runtime/README.md` / `README.zh.md` | 运行时、profile、外部插件怎么解析 |
| `python/development.md` | 怎么编：`pnpm exec tsx scripts/build-exe-for-python-sdk.ts --targets=node24-win-x64` |
| `.agents/notes/implemented/architecture/2026-07-10-single-file-executable-sdk-runtime-distribution.md` | 为什么用 pkg SEA、为什么不用裸 Node SEA / pkg 标准模式 |
| `.agents/notes/implemented/architecture/2026-08-23-python-sdk-windows-x64-runtime.md` | Windows 只发 x64、ConPTY addon、`-rg.exe` sidecar |

---

## 1. 当前 DSH Office 打包链路（现状）

### 1.1 流水线

入口：`frontend/client/scripts/pack-windows-client.py`，核心逻辑在
`frontend/client/scripts/lib/muse_windows.py::stage_dsh_runtime()`：

1. **下载官方 Node zip**：`stage_node()` 拉 `node-v22.19.0-win-x64.zip`，整包解进
   `{exeDir}/muse/node/`（连 `npm/`、`npx/`、`corepack/`、`nodevars.bat` 一起，运行时只用 `node.exe`）。
2. **整棵拷贝 harness**：`copy_tree(harness → muse/dsh)`，只排除
   `.git/website/python/coverage/.turbo/.cache`。**包含** `node_modules`（dev 依赖全量）、
   `apps/packages/scripts/docs/snapshots/native/vendor`、全部测试与构建工具。
   实测拷进来：`.pnpm` 虚拟仓库 879 MB；rolldown、vitepress、sharp、vitest、playwright、
   oxlint(+binding)、lefthook(+win32-x64)、tsdown、typescript、mermaid 等整条
   dev/构建/文档链；`@openai/codex@0.149.1`(+win32-x64)、
   `@anthropic-ai/claude-agent-sdk-win32-x64@0.3.241` 这类上百 MB 原生包；
   以及 `node-pty`、`@vscode/ripgrep`。
3. **拷贝 @muse 包**：`copy_dsh_packages()` 把 13 个构建好的 `@muse/*`（dist 产物）
   同时拷进 `muse/packages/`（源码形态副本）和 `muse/dsh/node_modules/@muse/`（Esm 解析要用）。
4. **补依赖**：`wire-muse-node-modules.py` 逐个 @muse 包解析依赖（@muse 兄弟包、harness 别名、
   用 createRequire 从 dsh-tools 解析、或从 Muse workspace 的 node_modules 拷贝 + hoist 兄弟包）。
5. **塞市场**：`stage_dshmarket()` 用 `npm pack dshmarket@1.31.1` 解包进
   `node_modules/dshmarket`，再 `repair_dshmarket_peers()` 把 `vendor/cordis`、
   `vendor/schemastery`、`packages/settings/settings` 的**源码树**拷进
   `dshmarket/node_modules/@deepseek-ai/`。
6. `cordis.patch.yml`（`middlewares/dsh/plugins/dsh-appflowy/cordis.patch.yml`）拷成 `muse/patch.yml`。

### 1.2 运行时启动（Flutter sidecar）

`frontend/client/frontend/appflowy_flutter/lib/plugins/dsh_agent/dsh_sidecar.dart`：

```
muse/node/node.exe --import tsx/esm apps/cli/src/bin.ts \
  --profile web --patch <muse>/patch.yml --host 127.0.0.1 --port 3080
```

- cwd = `muse/dsh`，即**用 tsx 直跑 TypeScript 源码**，不跑编译产物 `lib/`。
- 环境：`DSH_HOME=%APPDATA%\DSH Office\Muse\dsh`、`NODE_PATH=muse/dsh/node_modules`、
  把 node 目录加进 `PATH`（给 dshmarket 调 node/corepack/pnpm 用）。
- 启动时 Flutter 还把 `@muse/*` 与 `dshmarket` 以**符号链接**种进
  `$DSH_HOME/profiles/{,web}/node_modules`（ESM 从文件真实路径解析，`NODE_PATH` 无效）。

### 1.3 产物与校验

- `dist/windows/DSH Office/`（便携目录）+ `DSH-Office-windows-x64.zip` + 可选 Inno setup。
- 实测体积：`DSH Office` 共 **2,199 MB**，其中 `muse/` **2,068 MB**；Flutter 本体约 130 MB。
- 校验 `assert_packed_runtime()`：只查 4 个文件存在 + `node --version`，无运行时黑盒验证。

### 1.4 现状代价

| 代价 | 证据 |
|---|---|
| 体积 GB 级 | `muse/` 2.0 GB；zip/安装包/更新都随之变大 |
| dev 工具进包 | lefthook / oxlint / playwright / vitest / rolldown / vitepress / sharp 实测在包内 |
| 上百 MB 原生包进包 | codex win32-x64、claude-agent-sdk win32-x64 实测在包内 |
| 运行时跑源码 | tsx 直跑 `apps/cli/src/bin.ts`；冷启动按需编译 ESM，依赖完整源码树 |
| junction 脆弱 | `copy_tree` 靠重新建 junction 控制爆炸，但 zip（`rglob` 不跟链接）、Inno、资源管理器再拷贝都会要么打爆要么断链 |
| 双份 @muse | `packages/` + `node_modules/@muse` 两处副本，靠 `wire-muse-node-modules.py` 逐个补依赖，脆弱 |
| 外部下载 | 每台打包机现下 Node zip + npm pack dshmarket + 修 peer |

---

## 2. 官方 SEA 单文件链路

### 2.1 核心产物

```
dist-exe/deepseek-harness-sdk-runtime-win-x64.exe        # 单文件，自含 Node 24 运行时
dist-exe/deepseek-harness-sdk-runtime-win-x64-rg.exe     # 旁边的 ripgrep sidecar
```

- exe 内部是一个 **`/snapshot` 虚拟文件系统（VFS）**，装着**真实的包树**（编译后各包的
  `lib/` + 真实 `node_modules`），ESM 入口原样交给 Node 默认 loader，**无任何转译**。
- exe 是**闭包**：启动哪些插件完全由 exe **外面**的 `cordis.yml` / `--patch` 决定；
  VFS 里没有的包名直接 fail-loud，不需要 allowlist 代码。
  ⚠️ 注意「配置决定启动」只在**闭包候选集内**成立：VFS 里有什么，配置才能点谁；
  事后新增插件**不能**写进 VFS，只能落用户区 `$DSH_HOME/profiles/` 的真实磁盘 ESM 代理层（见 2.4）。
  也就是说配置决定的是「闭包里的哪些插件启动」，不是「任何插件都能被点名启动」。
- 官方只在 Windows 发 **x64**（`python/sdk-runtime/platforms.json` 只有 `win-x64`）；
  pkg `--sea` 一次只打一个 target，原生 addon 必须在对应真机构建。

### 2.2 闭包清单 = 一个纯依赖 manifest

`python/sdk-runtime/package.json`（`dsh-python-runtime-closure`，零代码）：
约 **130 个 workspace 生产依赖**，全是 @deepseek-ai 插件 + cordis/schemastery/cosmokit。
加一个预制插件 = **加一行依赖再打包**。

对照当前包：闭包**不含** lefthook/oxlint/playwright/tsx/tsdown 等 dev 工具；
也**不含** `dsh-subagent-codex` / `dsh-subagent-claude-code` —— 所以
`@openai/codex`、`@anthropic-ai/claude-agent-sdk` 默认不进 exe
（`dsh-hooks-codex`/`dsh-hooks-claude-code` 在闭包里，但它们只依赖 schemastery，不拉 SDK）。
`verify-runtime-closure.ts` 在打包前校验：读全部 preset 的 `agent.cordis.yml`，
要求每个激活插件都以 `workspace:` 依赖出现在闭包 manifest。

### 2.3 构建流水线（`scripts/build-exe-for-python-sdk.ts`）

1. **校验闭包**：`pnpm run verify-runtime-closure`。
2. **构建**：`pnpm run build`（tsc 出 `lib/types`，tsdown 出 `lib` bundle）。
3. **deploy 出无符号链接的 lib 树**（关键命令，实测为证）：
   ```
   pnpm --filter dsh-python-runtime-closure deploy --legacy --prod \
     --config.node-linker=hoisted --config.auto-install-peers=false \
     --config.link-workspace-packages=true <python/sdk-runtime/src/deepseek_harness_runtime/runtime/node/>
   ```
   随后 `restoreLegacyHoists()` 补回 legacy deploy 漏掉的直接 workspace 包，
   `materializeStagedLinks()` 把剩余符号链接物化为真实文件并删 `.bin`，**失败即中止**；
   最终树**零符号链接**。
4. **注入 pkg 配置**：`bin = node_modules/@deepseek-ai/dsh/lib/bin.js`（编译后的入口），
   `pkg.assets` 覆盖动态读取的资源（`ASSET_GLOBS`）：
   `package.json`、`node_modules/**/*.{js,cjs,mjs,json,md}`、
   `*.{dylib,dll,node,so,so.*,wasm}`、`*.{yaml,yml}`、
   `dsh-web-frontend/dist/**/*`、`dsh-skill-badge/assets/**/*`。
5. **原生 addon**：`prepareNativePty()` —— win-x64 把 node-pty 的
   `prebuilds/win32-x64/conpty.node` + `conpty_console_list.node` 放进 VFS；
   linux 在 manylinux 容器重编；mac 另带 `-spawn-helper`。
6. **pkg --sea 打包**：`pnpm dlx @yao-pkg/pkg@6.21.0 <staging> --sea --targets node24-win-x64
   --output dist-exe/deepseek-harness-sdk-runtime-win-x64.exe`。
   每个 target 一次调用；Windows 必须在 **win-x64 Node** 上构建（脚本强校验 host==target）。
7. **rg sidecar**：把 `@vscode/ripgrep-win32-x64/bin/rg.exe` 拷成

   `deepseek-harness-sdk-runtime-win-x64-rg.exe` 放在 exe 旁；`process.pkg` 存在时选 sidecar，
   普通 Node 执行直接用 `@vscode/ripgrep`。
8. **同步进 Python wheel** 分发（`deepseek-harness-runtime-bin`，tag `py3-none-win_amd64`）；
   代码里 `dist-exe/` 保留上传副本。

### 2.4 运行时与分发形态

- **不依赖系统 Node**；pip 装 wheel 即可用。
- `$DSH_HOME/profiles/node_modules` 下维护**真实 ESM 代理包**（符号链接进不了 VFS，
  官方已处理）——内置插件与外部插件共享同一个 Cordis 实例；装外部插件用
  `dsh plugin --profile <name> ...`（需本机 pnpm），普通执行不需要。
- 仓库里另有一份**仅开发用**的 node carrier（`python/sdk-runtime/src/.../runtime/node/`，
  deploy 产物本身），用系统 Node ≥22.19 跑 `node_modules/@deepseek-ai/dsh/lib/bin.js`；
  wheel / 正式分发**不带**它，也不会自动选它（`DSH_RUNTIME_MODE=node` 才选）。
- 验证：每平台 installed-wheel 黑盒 —— 干净 venv 装 wheel、证明产物来源、
  跑完整 keyless 场景（SDK、自定义配置、MCP、原生工具、直接 JSON-RPC、提交快照），
  trusted PR 再跑真 provider 工具。体积：`2026-07-10` note 实测 macos-arm64 **174 MB** 量级。

### 2.5 为什么不是别的路线（note 里的 Alternatives）

- **裸 Node SEA**：入口必须是单 CJS，blob 没有文件系统也没有模块解析，动态 import 裸包名
  无处解析；只能把插件静态编进入口手工注册，绕开标准解析，违背「配置决定一切」。
- **pkg 标准模式**：esbuild 把 ESM 转 CJS + V8 字节码，`import()` 全抛
  `ERR_VM_DYNAMIC_IMPORT_CALLBACK_MISSING`；还依赖社区补丁 Node 二进制。
- 实测通过的是 **pkg `--sea`**：VFS 内 ESM 动态 import（含 top-level await）、CJS 互操作、
  `node:sqlite`、VFS 外 on-disk ESM 的 `ctx.baseUrl` 通道全过。

---

## 3. 逐维度对比

| 维度 | 当前 DSH Office（Windows） | 官方 SDK 运行时（pkg --sea） |
|---|---|---|
| 包进什么 | 整棵 harness：源码 + dev 工具 + 上百 MB 原生包 + 双份 @muse + dshmarket | 闭包 manifest 的生产插件 lib 树，唯一清单 `dsh-python-runtime-closure` |
| 运行载体 | `node.exe` + tsx 直跑 `apps/cli/src/bin.ts`（TS 源码） | 单 exe：Node 24 运行时 + `/snapshot` VFS 内编译后 `lib/bin.js` |
| 是否要 Node | 自带整棵 Node zip（含 npm/npx/corepack） | 完全不需要 |
| 体积 | `muse/` 2.0 GB，目录 + junction | exe ~174 MB 量级 + `-rg.exe` sidecar；zip 更小 |
| dev 工具/大原生包 | 进包（实测 lefthook/oxlint/playwright/rolldown/vitepress/codex/claude SDK） | 不进闭包（codex/claude SDK 因 `dsh-subagent-*` 不在闭包而自然排除） |
| 插件集合定义 | 「vendor checkout 里装了什么就是什么」+ `wire-muse-node-modules.py` 手工补依赖 | 一份纯依赖 manifest，加一行 = 加一个插件 |
| 配置决定启动 | patch.yml 在包外，`--patch` 传入 | 相同：patch/cordis.yml 在 exe 外，VFS 缺包名 fail-loud |
| 动态读的资源 | 真实文件系统，随意读 | asset glob 显式打进 VFS（js/json/md/yaml/native/前端 dist/skill 资源） |
| 原生 addon | 依赖 vendored node_modules 里的现成 prebuild | 打包时显式 stage（win-x64: conpty.node + conpty_console_list.node，须 win-x64 真机） |
| ripgrep | `@vscode/ripgrep` 包直接跑 | `-rg.exe` sidecar（VFS 内 spawn 不了外部二进制） |
| 符号链接 | 包内保留 junction 结构，zip/安装器/资源管理器易打爆或断链 | deploy 物化后**零符号链接**；用户区 ESM 代理解决 VFS 限制 |
| 分发 | zip / Inno setup，手动拷贝 | wheel（win_amd64），pip 安装 |
| 校验 | 4 个文件存在 + node --version | 每平台 installed-wheel 黑盒 + 平台快照 + 真 provider |
| 用户数据 | `%APPDATA%\DSH Office\Muse\dsh` | 显式 DSH_HOME（Office 对齐后不变） |
| 平台 | Windows x64（另有 macOS 整套同构脚本） | 官方 4 target；Windows 只 x64 |

---

## 4. Office 侧对齐方案

对齐 SEA 路线，**不再拷整棵 node_modules**。预制插件进闭包，不再外挂源码树。

### 4.1 闭包

- 新建 Office 闭包 manifest，或直接在 `dsh-python-runtime-closure` 上加依赖，
  除官方生产插件外再加：
  - `@muse/host-bridge`、`@muse/plugin-facets`、`@muse/plugin-kit`、
    `@muse/plugin-graph`、`@muse/context-broker`、`@muse/contract-document`、
    `@muse/dsh-appflowy`、`@muse/plugin-appflowy-{workspace,markdown,view-reference,view-rename}`、
    `@muse/dsh-mobile-surface`、`@muse/dsh-mobile-input`（当前 PACKAGE_DIRS 的 13 个）；
  - 市场：`dshmarket`（corda 的 `cordis.patch.yml` 现在 `insert` 的就是它）。
- `@muse/*` 的外部依赖（`ajv`、`ajv-formats`、`@noble/hashes`、`canonicalize`）
  会随 `pnpm deploy` 自动进入闭包，不用再靠 `wire-muse-node-modules.py` 手工补。
- **前置条件**：@muse 包必须能被 harness 的 pnpm workspace 解析（成为 workspace 成员，
  或以本地 `file:`/发布后的 semver 依赖引入），且各自先 `pnpm build` 出 `dist/`——
  SEA 跑的是编译后的 `node_modules/@deepseek-ai/dsh/lib/bin.js`，**不能再 tsx 跑源码**；
  `package.json` 的 `files` 要覆盖 dist 与运行时读取的资源。

### 4.2 启动

- patch.yml 留在 exe 外（和现在 `{exeDir}/muse/patch.yml` 一样）：
  ```
  {exeDir}/muse/deepseek-harness-sdk-runtime-win-x64.exe web --patch <patch.yml> \
    --host 127.0.0.1 --port 3080
  ```
  `web` 是 CLI 的硬编码别名（`apps/cli/src/args.ts`：`dsh web` ≡ `--profile web`），
  与现在的 `--profile web --patch ...` 完全同语法。
- Flutter sidecar（`dsh_sidecar.dart`) 把 `_startDshCli`/`_spawn`
  换成直接 spawn exe；`_seedMuseModules`/`_seedDshMarket`
  对**预制插件不再需要**（VFS 内解析）；外部安装的社区插件走官方
  `$DSH_HOME/profiles/node_modules` ESM 代理（官方已处理「符号链接进不了 VFS」）。
- 用户数据仍是 `%APPDATA%\DSH Office\Muse\`；装新插件需要本机 pnpm，预制插件不需要。

### 4.3 注意事项

- **终端用户 vs 插件安装**：用户装完 exe 后，日常使用、预制插件、市场浏览都**不需要 pnpm**
  （VFS 内插件直接启动）；只有「在应用里新装一个社区插件」（dshmarket 安装 /
  `dsh plugin --profile <name> add ...`）才需要包管理器。若市场安装是**必选功能**，
  主方案见 4.4（捆绑独立 pnpm sidecar）——用户仍然**不需要预先安装任何东西**。
- Windows 官方只发 **x64**；原生 addon（pty、sqlite 内建、koffi 等需要在 VFS 内动态 import 的包）
  必须在 **win-x64 真机** 上编进闭包，或出现在 asset glob。

### 4.4 一键安装社区插件（必选）的 SEA 落法

结论：**一键安装可以做到，且不需要用户装任何东西** —— 关键事实是
**pnpm 的 standalone / `@pnpm/exe` / pnpm 12（Rust 原生版）可以完全不带 Node.js 运行**
（官方安装文档原话：You may install pnpm even if you don't have Node.js installed；
pnpm 12 由 standalone 脚本装的版本 runs without Node.js）。所以 SEA 下只需把**独立 pnpm
作为一个真实 sidecar 文件**放在 exe 旁（和 `-rg.exe`、macOS `-spawn-helper` 同一类模式——
VFS 里的东西无法被 spawn，插件管理必须走真实文件）。

安装链（用户点一下）已经全部由 dshmarket 1.31.1 实现，与打包方式无关：

```
市场 UI（Flutter WebView）
  → dshmarket（运行在 exe 内）
  → runDshPlugin：重新 spawn 当前 CLI（SEA 下就是 exe 自己）
      deepseek-harness-sdk-runtime-win-x64.exe plugin --profile web add <pkg>
      （dsh-cli.js:683：spawn(file, [...args, 'plugin', '--profile', profile, ...pluginArgs])）
  → exe 的 plugin 子命令（apps/cli/src/plugin.ts）在 $DSH_HOME/profiles/web 里跑 `pnpm ...`
  → pnpm 安装到 profile 目录，--reporter=ndjson 进度回传 UI，bundle 清单 reconcile，profile 热重载
```

**SEA 下唯一要补的缺口 = 「pnpm 从哪来」**：

- Flutter sidecar 启动 exe 时把 `muse/plugin-tools/`（捆绑的独立 pnpm）**加进 PATH**
  （现在就是这么给 node 目录加 PATH 的，同一个模式）；exe 的 `plugin` 子命令和
  dshmarket 的 PATH 探测都能直接命中，**第一次也不会有「隐性装 pnpm」的等待**。
- dshmarket 已经内置了桌面契约：`dsh-cli.js` 的 Desktop 模式（`"The service is backed by
  Desktop's packaged pnpm; system discovery and global provisioning are neither needed nor
  allowed in this mode"）——`probePnpm`/`provisionPnpm` 直接 no-op，装插件全部走
  runPlugin；也就是它**本来就假设桌面端自带 pnpm**。当前包只是「借 node 里的
  npm/corepack 现场供给」，SEA 换成「直接捆绑独立 pnpm」，语义更干净。
- 兼容层不用重写：pnpm 9/10/11 差异、host peer 注入（`@deepseek-ai/*` 不在 npm、pnpm 会去
  404）、ndjson 进度解析、代理翻译、镜像回退、限时重试（`install.js`/`pnpm-compat.js`）
  都是 dshmarket 现成的，与运行时是 VFS 还是真目录无关。

体积与杂项：

- sidecar 体量：独立 pnpm（内嵌 node 的 standalone / pnpm 12 原生）为几 MB～几十 MB 量级，
  **不破坏单文件 exe 的叙事**（exe 仍是唯一运行时；sidecar 是官方已认可的 sidecar 模式的
  第三次复制）。实现时选型：pnpm 11 standalone（内嵌 node，跑任意 postinstall 最稳）vs
  pnpm 12 原生（更小更快，但依赖 node 的 install 脚本要用 `pnpm runtime` 兜底），用
  `pnpm --version` 冒烟 + 装一个带 postinstall/native 的样例插件验证一次即可。
- pnpm 官方文档警告：Windows 上 standalone exe 可能被 Defender 拦截 —— 需要与
  `dsh-office.exe`/setup 同一套 **Authenticode 签名**策略签 pnpm sidecar。
- pnpm store 落在用户区（`%LOCALAPPDATA%\pnpm` / `~/.pnpm-store`），不写安装目录，
  与「用户数据在 %APPDATA%」一致。
- 备选（不推荐作主路径）：市场服务端预解析依赖、客户端只下载解压 tarball ——
  能彻底消灭客户端包管理器，但放弃 dshmarket/pnpm 现成的兼容层，改动面大；
  可作为「受限网络下预置下载」的补充。
- 选型参照：官方桌面端实测用 **bundled node 24.9.0 + pnpm 10.34.5（npm 依赖形式）**
  （见 4.5），不必追 standalone 二进制的新鲜度。

### 4.5 官方桌面端（dsh-desktop）的实际做法：node carrier + 捆绑 pnpm，而非 SEA

`vendors/dsh-desktop`（官方 Electron 桌面端）**没有用 pkg --sea**，走
「npm 闭包 + 自带 node + 自带 pnpm」路线，而且**已把一键装插件做成了产品功能**：

1. **运行时 = npm 闭包**：harness 用 `scripts/release/pack.ts` 打成
   `packages/harness-0.1.2-alpha.4/npm-dsh/`（243 个 @deepseek-ai tarball）+
   `npm-vendor/`（10 个 cordis/schemastery/cosmokit 等），合计仅 **8 MB** 签进仓库；
   `package.json` 以 `file:` 依赖逐一引用（245+ 项），`npm ci` 把编译后的闭包装进
   应用 `node_modules`（生产依赖，无 dev 工具、无 tsx 源码）。
2. **自带真实 Node**：`"node": "24.9.0"` 作为 npm 依赖（官方 node 包带平台二进制），
   安装包里是 `node_modules/node/bin/node.exe`（`bundledNodePath()`，src/main/index.ts:550）。
3. **启动 harness**：`<bundled node> harness-node-entry.mjs <node_modules/@deepseek-ai/dsh/lib/bin.js>
   web --no-open --host 127.0.0.1 --port <p> --patch dsh-desktop.patch.yml`
   （`src/main/runtime/harness-runtime.ts`）——跑编译后的 `lib/bin.js`，patch 在包外。
4. **一键装插件 = 捆绑 pnpm + `$DSH_HOME/.desktop-bin` shim + lock-recovery runner**：
   - `"pnpm": "10.34.5"` 同为 npm 依赖 → `node_modules/pnpm/bin/pnpm.cjs` 进包；
   - `packages/dsh-desktop-market-installer/pnpm-runner.mjs` 是**所有 pnpm 操作的统一
     入口**：Windows 锁重命名恢复（pnpm `_tmp_*→<pkg>` 换名 EPERM → 重试 → 把阻塞目录
     挪成 `.dsh-old-<ts>` 让 pnpm 重装 → taskkill 杀进程树）、5 分钟 idle 超时、
     git prepare 审批映射、generation-projection 隔离（装插件时先摘掉 profile manifest
     里冷启动投影的包名，装完恢复）；
   - 运行期往 `$DSH_HOME/.desktop-bin/` 写 `pnpm.cmd`/`node.cmd` shim：
     `"<bundled node>" [pnpm-runner.mjs] <pnpm.cjs> %*`，并把该目录 + node 目录前置进
     harness 子进程 PATH（`src/main/runtime/profile-plugin-command.ts`）→ 之后
     dshmarket / `dsh plugin --profile <p> add <pkg>`（CLI 在 profile 目录跑 `pnpm`）
     命中的就是捆绑的 node+pnpm；
   - profile 首次启动走 `dsh plugin --profile web install --no-frozen-lockfile`
     （把核心 bundles 装进用户区 profile）。
5. **市场内建**：`dshmarket` 以 `file:` 依赖内建（`build:market` 用 tsc 编
   `packages/dshmarket`），与 `@deepseek-ai/dsh-base`、`@deepseek-ai/dsh-web-app`
   同属 CORE_BUNDLES（harness-runtime.ts:633，故障恢复时不允许卸载）。

**对 Office 的落地映射**：

- 官方桌面端证明了「**node 与 pnpm 都作为 npm 依赖随包携带**」是桌面端一键安装的
  标准做法——正是 4.4 方案 A 的官方实现，且不用 standalone pnpm（node 反正要带，
  pnpm.cjs 在它上面跑即可），选型以「node 24.9.0 + pnpm 10.34.5」为参照。
- 可直接搬运的资产：`pnpm-runner.mjs`（纯 node 脚本）、`.desktop-bin` shim 写法、
  `dsh plugin --profile <p> install/add` 调用链、dshmarket 内建 +
  「Desktop's packaged pnpm」契约（dshmarket 1.31.1 Desktop 模式已按此设计）。
- 与 Muse 的差异：dsh-desktop 用 Electron 的 `app.isPackaged/resourcesPath` 定位资源，
  Muse 用 `{exeDir}/muse/`；dsh-desktop 的运行时是「app node_modules 里的 npm 闭包」，
  Muse 若走 SEA exe 则 VFS 内解析、只需为插件管理保留 node+pnpm sidecar；
  若走 node carrier 过渡，则与 dsh-desktop 形态几乎一致（闭包树 + node.exe + pnpm.cjs）。
- 体积参考：闭包 tarball 合计仅 8 MB（dsh-web-frontend 1.4 MB 为最大单包）；
  整包（Electron + node + 闭包 node_modules）仍在几百 MB 量级，比当前 2.0 GB
  整树拷贝小一个数量级，且不含任何 dev 工具。

### 4.6 SEA+sidecar vs node carrier：逐维对比

实测基准（本机）：`node.exe` 本体 **81 MB**；官方 Node 整包（含 npm/corepack）解压
**175 MB / 2314 文件**；SEA exe 参考 **~174 MB**（官方实测 macos-arm64）；闭包 tarball
压缩态 **8 MB**（dsh-desktop）；安装后的闭包 node_modules ≈ SEA 打进 exe 的那棵树
（百 MB 量级）。

| 维度 | A. SEA exe + 插件管理 sidecar | B. node carrier（dsh-desktop 形态） |
|---|---|---|
| 目录体积 | exe ~174 MB + sidecar：standalone pnpm（几~几十 MB）**或** node.exe+pnpm.cjs（+81 MB）→ **~180–260 MB** | 闭包树（~175 MB）+ node.exe（81 MB）+ pnpm.cjs 几 MB → **~260 MB** |
| 压缩后（zip/安装包） | ~80–120 MB | ~110–150 MB |
| 文件数 | 2–3 个（exe + rg + pnpm） | 数千个 JS（AV/资源管理器要逐文件扫） |
| 启动 | 单文件 mmap + VFS 内读；AV 只扫 1 个 exe；**冷启动通常更快** | node 逐文件读盘；AV 按文件扫；首次启动 profile 还需 `plugin install` 物化 |
| 运行中开销 | VFS 读 vs 磁盘读，差异很小；大 blob 常驻页缓存 | 常规磁盘读；与 SEA 相当 |
| 插件操作 | 每次装插件多 spawn 一个 sidecar 进程（毫秒级，可忽略） | 复用同一个 node.exe，链路最短 |
| 内置集只读性 | ✅ VFS 只读——内置插件**不可被改/被 pnpm 误升级**（dsh-desktop 专门做 generation projection 隔离防的正是这类事，SEA 从根上消灭） | ❌ 磁盘普通 JS，可被用户/工具/进程直接改；dsh-desktop 为此写了 profile-repair/consistency/cleanup 一整套修复机制 |
| 篡改/完整性 | 一个 exe，Authenticode 签名 + AV 识别；改 exe 即失效 | 篡改单个 JS 文件即可注入；靠整体签名/锁目录，管控粒度差 |
| 插件安全模型 | 一致：第三方插件装进用户区 profile，凭 authorization/permission 管 | 一致 |
| 失败模式 | 单文件坏=整体换；无“某个插件文件被删导致半瘫” | 任意一文件坏/被隔离/被清理工具删都可能崩，需要修复机制兜底 |
| 排障 | 错误路径是 `/snapshot/...`，调试要懂 VFS | 常规文件路径，贴近开发环境 |
| 更新 | 换一个 exe，原子；内置集变更必须发版 | 整包更新为主；可单文件 hotfix（小众场景） |
| 发版证据 | 闭包校验 + 每平台黑盒（官方已建全） | lockfile + file: tarball 可复现（dsh-desktop 已建全） |
| 资产复用 | pnpm-runner.mjs/.desktop-bin 逻辑可搬，但 sidecar 无 node 时要把 runner 改指 standalone pnpm | 与 dsh-desktop **几乎逐字复刻**，官方验证最充分 |
| 平台 | 官方仅 win-x64（省 arm64） | 随 node 平台走（含 mac arm64/x64） |

结论与建议：

- 日常运行与插件体验两条路**等价**（都跑编译后 lib/bin.js、都零用户安装、都一键装插件）。
- 体积上 A 显式更小（standalone pnpm 形态 ~180 MB、node sidecar 形态 ~260 MB），
  B 与 A(nodesidecar) 基本同体量，差别只在「闭包树塞进 exe 还是放磁盘」。
- 安全/可靠性上 A 更硬：内置集只读、单文件完整性、无「磁盘树被弄坏」一类故障面；
  B 更“常规”、更贴近开发、出问题可手工修文件。
- **建议路径**：先用 B（node carrier，≈ dsh-desktop，风险最小、资产复用最多）把
  「一键装插件」跑通并验证 Windows 锁恢复；再跨到 A —— 用 SEA exe 换掉闭包树，
  保留同一套 node+pnpm sidecar（81 MB）作为插件管理通道，即可同时拿到
  「单文件只读主集」+「官方验证过的插件安装链」。若极在意体积，再优化 sidecar 为
  standalone pnpm 并改造 pnpm-runner 指过去。
- `verify-runtime-closure` 会遍历 manifest 覆盖的 workspace 包并校验其非可选 peer——
  @muse 包进入 workspace 后要满足 harness 的依赖约束（如 `@deepseek-ai/cordis` peer）。
- web 前端资源：`dsh-web-frontend/dist` 已在官方 asset glob 里；Muse WebView 走同一
  3080 端口不变。

---

## 5. 过渡方案（比现在小、比 SEA 快）

只 `pnpm deploy --prod` 闭包那份**无符号链接的 lib 树**，仍带一个 `node.exe` 跑
`node_modules/@deepseek-ai/dsh/lib/bin.js` —— 这就是官方文档里的 **node carrier**
（仓库里 `python/sdk-runtime/src/.../runtime/node/` 的形态，Office 版可放在
`{exeDir}/muse/{node.exe,node_modules/...}`）。

- 已经是编译后产物、零符号链接、无 dev 工具、无 codex/claude；
- 体积从 2.0 GB 降到几百 MB 量级（仍不如 exe 小）；
- 启动命令从 `node --import tsx/esm apps/cli/src/bin.ts` 换成
  `node.exe node_modules/@deepseek-ai/dsh/lib/bin.js web --patch <patch.yml> ...`。

正式分发仍应以 **pkg --sea 单文件** 为准。

> **当前实施选择**（经 4.5 对照 dsh-desktop 后确定）：先走 B（node carrier）把
> 「一键装插件」跑通（捆绑 pnpm + `.desktop-bin` shim + pnpm-runner.mjs），
> 再跨到 SEA exe + 同一套 node+pnpm sidecar。实施状态见
> [packaging-node-carrier.md](./packaging-node-carrier.md)。

---

## 6. 迁移 checklist

- [ ] 让 13 个 `@muse/*` 包能被 harness pnpm workspace 解析（workspace:` 或 file: 依赖）
- [ ] 在 `dsh-python-runtime-closure`（或新建 Office 闭包）加 `@muse/*` + `dshmarket` 依赖
- [ ] 补 `@muse/*` 的 `files` 字段，确保 dist 与运行时资源进包
- [ ] `pnpm exec tsx scripts/build-exe-for-python-sdk.ts --targets=node24-win-x64`
      （win-x64 真机；首次需 `~/.pkg-cache` 下载官方 Node 基座）
- [ ] `muse/` 目录瘦身为：exe + `-rg.exe` + `patch.yml`（+ 可选 node carrier 过渡）
- [ ] `dsh_sidecar.dart` 改为 spawn exe；删 `_seedMuseModules`/`_seedDshMarket`
- [ ] 黑盒验证：启动 web profile → 3080 就绪 → @muse 插件加载 → dshmarket 可见 →
      外部插件安装（需 pnpm）仍走官方 ESM 代理
- [ ] zip/Inno 流程不变（拷的是小目录，junction 问题消失）