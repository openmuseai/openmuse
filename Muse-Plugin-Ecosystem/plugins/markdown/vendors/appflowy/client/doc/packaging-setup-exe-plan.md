# DSH Office 直装化路线方案（A：setup.exe 直装闭环 → B：单文件 exe）

> 目标：用户拿到 `DSH-Office-windows-x64-setup.exe` **双击直装、开箱即用**（打开即有 DeepSeek 面板与
> 市场、可一键装插件）；中期把运行时砍到「一个 exe 量级」。A = 闭包运行时 + 重编后的 Flutter
> GUI，装完**不依赖源码树 / 本机 Node**。B（pkg `--sea` 单文件）未实施。
> 现状基线见 [packaging-node-carrier.md](./packaging-node-carrier.md)。

## 开箱失败根因（2026-09-04）

已产出的 `DSH-Office-windows-x64-setup.exe` **运行时是闭包**（`{app}/muse/{node,closure,plugin-tools,patch.yml}` 齐全），但安装包里的 `dsh-office.exe` / `data/app.so` 仍是 **2026-09-03 10:51 的旧 Dart AOT**：

- 旧 `bundleRoot()` **只认 macOS** `Contents/Resources/muse`，Windows 上直接返回 null；
- `resolve()` 于是去扫 `middlewares/dsh` + `frontend/client` 源码树；
- 安装目录没有仓库 → 面板报 `Bad state: Cannot find the Muse repo … Set MUSE_ROOT …`。

这不是闭包缺文件，而是 **GUI 没编进「旁边就是 muse/」这条路径**。A 的 Done 标准必须包含：**重编 Flutter 后再打 setup**，并用 `verify-windows-portable.py` 从安装位启动闭包 sidecar（不设 `MUSE_ROOT`）。

## 实施状态（2026-09-04 实测，A 阶段）

| 项 | 状态 | 实测/说明 |
|---|---|---|
| 步骤 0：剔除 codex/claude 载荷 | ✅ | `EXCLUDED_PACKAGES`；闭包 node_modules **1046 → 341 MB** |
| 闭包自足化 | ✅ | @muse 打成 tarball（删 `link:`/`workspace:`），`npm install --legacy-peer-deps` |
| 便携目录组装 | ✅ | `muse_windows.stage_closure()` 写入 **muse/ 本身**；pack 默认 `--closure` |
| pack 默认闭包 | ✅ | `pack-windows-client.py --closure`（`--no-closure` 回退整树）；`--reuse-runtime` 识别 `closure/` |
| 首启 profile 插件通道 | ✅ | 只种子 **`profiles/web/node_modules`**（不要写 `profiles/node_modules`，否则 Loader `healProfilesModuleFallback` 对 js-yaml 等非 proxy 目录 fail-loud）；51 包实体拷贝 |
| Flutter 适配 | ✅ 已重编 | `data/app.so` 2026-09-04 10:10；Windows `{exeDir}/muse`；闭包入口 `node <closure>/lib/bin.js web --patch …`；AOT 含 `closure/node_modules/@deepseek-ai/dsh` |
| setup.exe | ✅ 已用新 AOT 重打 | `dist/windows/DSH-Office-windows-x64-setup.exe` **121.5 MB**（Inno 6.7.3 portable，GitHub `is-6_7_3`） |
| 安装 / sidecar 验证 | ✅ | 静默装到临时目录 → `verify-windows-portable.py`：**BOOT PASS :3092** + **PLUGIN PASS picocolors**；GUI 启动 sidecar 命令为捆绑 `node …/lib/bin.js web --patch muse/patch.yml`，**不再报 MUSE_ROOT** |
| 签名 / 干净 VM GUI / 更新通道 | ⏳ | Authenticode；干净机双击面板；WinSparkle AppCast |

### A 阶段选定的关键机制（决定，可回退）

- **运行时分发**：`muse/{node/node.exe, closure/node_modules, closure-tarballs, plugin-tools, patch.yml}`（无整树拷贝、无 junction）。
- **GUI 定位运行时**：优先 `{exeDir}/muse`（或 `MUSE_BUNDLE_ROOT`）。只有旁边没有合法 bundle 时才扫源码树。安装版 **禁止** 再要求 `MUSE_ROOT`。
- **首启解析**：patch 裸名插件（@muse/*、dshmarket）由侧车以**实体拷贝**种子进 profile `node_modules`（从闭包递归复制其 deps；一次性 marker）。
- **闭包构建**：`build-muse-closure.py`（official-profile 构建校验 → pack.ts 每 family 独立目录 → @muse tarball 重写 → `npm install --legacy-peer-deps` → 零符号链接）。
- **一键装插件**：`.desktop-bin` shim → bundled node + `pnpm-runner.mjs` + bundled pnpm。

### A 的打包命令

仓库根目录，闭包已在 `tmp/muse-closure` 或便携目录里时：

```bat
python frontend/client/scripts/pack-windows-client.py --skip-core-build --skip-packages --reuse-runtime --skip-zip
```

- `--closure` 默认开启：组装 `muse/closure` 而不是整棵 harness。
- `--reuse-runtime`：保留已验证的闭包树，只换新的 `dsh-office.exe` + `data/app.so`。
- `--skip-core-build`：Rust `dart_ffi` 未改时跳过 cargo-make。
- `--no-closure`：回退 `stage_dsh_runtime` 整树拷贝。
- `--rebuild-closure`：强制重跑 `build-muse-closure.py`。

验证（便携目录或安装目录）：

```bat
python frontend/client/scripts/verify-windows-portable.py
python frontend/client/scripts/verify-windows-portable.py --root "%LOCALAPPDATA%\Programs\DSH Office"
```

### A 的剩余收尾

1. 把新 `setup.exe` 覆盖装到 `%LOCALAPPDATA%\Programs\DSH Office`（旧安装包里的 `app.so` 仍是 09-03 构建，会继续报 MUSE_ROOT）。
2. 签名（A.4）后在**干净 VM** 跑 A.5 矩阵（GUI 双击 → 面板 → 市场可见 → 装插件）。
3. WinSparkle AppCast 接新版本 setup。

> B 阶段（pkg --sea 单文件运行时）保持原方案不变，待 A 全绿后再推进。

## 步骤 0：剔除 codex / claude SDK 载体包（先行，低风险）

- **动作**：`frontend/client/scripts/build-muse-closure.py` 的 deps 收集处加排除集
  `{'@deepseek-ai/dsh-subagent-codex', '@deepseek-ai/dsh-subagent-claude-code'}`（两个一起剔，
  避免 `dsh-subagent-claude-code` 依赖 `dsh-subagent-codex` 悬空），重新生成 manifest +
  `npm install`（tarballs 已缓存，~20 s，**无需重打 tarballs**）。
- **影响（已查证，零业务影响）**：
  - 默认预设 `standard/agent.cordis.yml:199-218` 的 codex/claude 工具行 **`disabled: true`**，
    注释明示「Production dsh does not install these optional providers」；
  - 全仓仅 `subagent-claude-code → subagent-codex` 一条内部依赖（一起剔）；
  - subagent 服务按名注册 provider，未挂载即 `NO_PROVIDER` fail-loud，与官方最小闭包一致；
  - `dsh-hooks-codex/-claude-code` 保留（只依赖 schemastery，桥接外部 CLI，与包内 SDK 无关）。
- **收益**：解包 1046 → ~350 MB；zip 370 → ~150–180 MB；后续 setup/单文件同比例缩小。
- **验收**：剔除后 `npm install` 成功 → `dsh --profile web --dump-config` exit 0（组合完好）→
  体积复测 → 一键装插件 E2E 复跑一次。

---

## A 阶段：`setup.exe` 直装闭环

### A.1 ISCC 就绪

- 打包机/CI（win-x64）安装 **Inno Setup 6**（`ISCC.exe`，可静默：
  `setup.exe /VERYSILENT /SUPPRESSMSGBOXES /NORESTART`；CI 用 choco/winget 或官方安装器）。
- `muse_windows.find_iscc()` 已实现自动探测；装好后 `pack-windows-client.py` 的 installer 分支
  不再跳过。`.iss` 模板位置：`frontend/client/frontend/scripts/windows_installer/inno_setup_config.iss`
  （已具备：每用户安装 `PrivilegesRequired=lowest`、`{localappdata}\Programs\DSH Office`、
  LZMA2、快捷方式、装后启动、HKCU `dsh-office://` 协议、卸载）。

### A.2 闭包进入 pack 流程

- `muse_windows.py` 新增 `stage_closure()`：把 `build-muse-closure.py --out` 的输出组装为
  `dist/windows/DSH Office/muse/`：
  ```
  muse/
  ├── node/node.exe                    # 捆绑 Node 22.19（app + 插件管理共用）
  ├── closure/
  │   ├── package.json
  │   └── node_modules/                # 零符号链接闭包（772 包，剔除后 ~350 MB）
  │       └── @deepseek-ai/dsh/lib/bin.js   # 运行时入口
  ├── closure-tarballs/                # 部署用 tarballs（242+9+13+dshmarket ≈ 10 MB，用于首启 profile 安装）
  ├── plugin-tools/                    # bundled pnpm + pnpm-runner.mjs + .desktop-bin shim
  └── patch.yml
  ```
- `build-muse-closure.py` 顺带把 **@muse 13 包打成 tarballs**（`npm pack <middlewares 目录>`，
  ~1–2 MB）放进 `closure-tarballs/muse/`，供首启 profile 装插件用（file: 源）。
- `pack-windows-client.py`：默认 `--closure`（`--no-closure` 回退整树）；`assert_packed_runtime`
  按闭包或 legacy 二选一校验（闭包入口
  `muse/closure/node_modules/@deepseek-ai/dsh/lib/bin.js` + `@muse/dsh-appflowy` + dshmarket）。
- `.iss` 的 `Source: "DSH Office\*"` 无需改：打包的就是该目录。
- `ensure_iscc()`：本机没有 Inno 时拉 portable 6.7.3 到 `dist/cache/innosetup/`。

### A.3 首启 profile 初始化（「开箱即用」的硬前提）

现状：patch 裸名插件（`@muse/*`、`dshmarket`）从 profile 解析不到——Loader 的
`healProfilesModuleFallback` 只镜像 dsh 安装闭包且写符号链接（本机 Node 22.19 ESM 拒绝
node_modules 里的链接条目）；手工往 profiles 种拷贝会被 Loader 清理。**正确通道 = 用 bundled
pnpm 把插件装进 profile（实体硬链接目录，Loader 不清理）**，这正是 dshmarket 安装链路已验证的路径。

首启流程（Flutter sidecar `ensureStarted` 内，幂等、失败不阻塞主流程）：

1. 若 `$DSH_HOME/profiles/web/package.json` 的 `dependencies` 已含 `@muse/*` + `dshmarket` → 跳过。
2. 否则写入 profile manifest：`dependencies` = `{ "@muse/dsh-appflowy": "file:<muse>/closure-tarballs/muse/dsh-appflowy.tgz", …（13 个）, "dshmarket": "file:<muse>/closure-tarballs/dshmarket-XXX.tgz" }`
   （全程绝对路径）。
3. 经 `.desktop-bin` shim 执行：
   `"<muse>/node/node.exe" <closure>/node_modules/@deepseek-ai/dsh/lib/bin.js plugin --profile web install`
   （即官方 `buildProfileInstallArguments`：`install --no-frozen-lockfile`；若失败则退化为
   `add <pkg>` 逐包，复用 dshmarket 已处理的 pnpm 兼容层思路）。
4. 校验：`profiles/web/node_modules/@muse` 存在且非 reparse；`--dump-config` 组合含 @muse 行。
5. 失败策略：记录日志 + 面板提示「插件未初始化」，不阻断侧车启动（首要保证 DeepSeek 可用）。

> 备选（若 3 的 profile-install 在 file: 路径上再踩坑）：profile manifest 改用
> `dsh.profile.bundles` 声明 @muse bundle 包（bundle 型插件会自动 reconcile）——由实施期实测决定。

### A.4 签名

- Authenticode 签发：`dsh-office.exe`、`DSH-Office-windows-x64-setup.exe`、`plugin-tools/` 内
  pnpm 相关可执行（standalone/runner 无签名时 Defender 可能拦截，官方文档已提示）；
  `node.exe` 官方已签名；时间戳服务（RFC 3161）。
- EV 证书可选（提升 SmartScreen 信任）；未签名版继续保留给内部测试。
- 安装目录写入时不触发 UAC（每用户安装）。

### A.5 验证矩阵（A 的 Done 标准）

| 场景 | 验收 |
|---|---|
| 干净用户（无 Node/pnpm）装 setup | 启动 → sidecar 就绪（3080）→ DeepSeek 面板输入 key → 会话可用 |
| 首次启动初始化 | profiles/web 含 @muse + dshmarket（实体目录）；市场页可见；一键装一个社区插件成功 |
| 二次启动 | 初始化幂等跳过，启动不变慢 |
| 覆盖升级 | 旧版本上装新版 setup；用户数据（%APPDATA%）保留 |
| 卸载 | 程序清除干净；用户数据保留（设计如此，文档注明） |
| 便携 zip 并存 | zip 版与安装版不冲突（共用 %APPDATA%） |
| 签名校验 | setup.exe/dsh-office.exe 签名有效；Defender 不拦 |

产物：`frontend/client/dist/windows/DSH-Office-windows-x64-setup.exe`（LZMA2，
剔除两包后预期 **~150–180 MB**）。

---

## B 阶段：运行时单文件化（pkg --sea）

### B.1 打包

- 复用 A 的闭包（`closure-tarballs` + `closure/node_modules`）：
  `pnpm dlx @yao-pkg/pkg@6.21.0 <closure 根> --sea --targets node24-win-x64
  --output dist-exe/deepseek-harness-sdk-runtime-win-x64.exe`
- 入口：官方 `bin = node_modules/@deepseek-ai/dsh/lib/bin.js`；`pkg.assets` 显式覆盖：
  `node_modules/**/*.{js,cjs,mjs,json,md,yaml,yml}`、`*.{node,dll,so,dylib,wasm}`、
  `node_modules/@deepseek-ai/dsh-web-frontend/dist/**/*`（web UI 资源）、
  `dsh-skill-badge/assets/**/*`、闭包内与 profile 初始化相关的 `cordis.patch.yml`/README。
- 原生：node-pty `prebuilds/win32-x64/{conpty.node,conpty_console_list.node}` 进 VFS；
  `node:sqlite` 内建；koffi 预编译二进制随资产进包；ripgrep 走
  `deepseek-harness-sdk-runtime-win-x64-rg.exe` sidecar（`process.pkg` 选择）。
- **win-x64 真机构建**（官方限制：Windows 仅 x64、本机构建）。

### B.2 插件管理在单文件下的形态

- SEA 的 VFS **不能被 spawn**：bundled pnpm 必须是真实 sidecar。两种形态：
  - **推荐（对齐 dsh-desktop）**：保留 `plugin-tools/node/node.exe`（81 MB）+ pnpm.cjs +
    pnpm-runner.mjs；`.desktop-bin` shim → `"<sidecar node>" pnpm-runner.mjs <pnpm.cjs>`，
    exe 的 `plugin` 子命令、dshmarket 的 PATH 探测全部沿用 A 已验证链路。
  - 更瘦备选：standalone pnpm（不带 node，~几～几十 MB），但 pnpm-runner.mjs 是 node 脚本，
    需把 runner 改造为直接 spawn standalone pnpm，锁恢复逻辑保留——收益 ~50 MB，成本改造。
- 运行时（`{exeDir}/muse/`）：
  ```
  deepseek-harness-sdk-runtime-win-x64.exe + -rg.exe   # 运行时（~174 MB 量级）
  plugin-tools/{node/node.exe, pnpm, pnpm-runner.mjs}   # 插件管理含 sidecar
  patch.yml
  closure-tarballs/                                     # 首启 profile 安装源
  ```
- 首启初始化、A.5 验证矩阵不变；Flutter sidecar 启动命令换成
  `<exe> web --patch <patch.yml> --no-open --host 127.0.0.1 --port 3080`。

### B.3 体积与构成（预期）

| 形态 | 目录/文件 | 安装包（LZMA2 / 压缩） |
|---|---|---|
| 现状闭包布局 | 1,046 MB（剔除前）/ ~350 MB（剔除后） | zip 370 / ~150–180 MB |
| **B：单文件运行时 + pnpm sidecar** | exe ~174 MB + node 81 MB + rg + tarballs ~10 MB | **setup ~120–150 MB** |
| 更瘦（standalone pnpm） | exe + standalone pnpm + rg | ~120 MB 内 |

### B.4 风险与验证

- VFS 内 ESM 动态 import：官方 `--sea` 已实测通过（含 top-level await），闭包内无转译；
- worker（dsh-workflow/-code-runtime worker-thread）：官方已处理
  `fileURLToPath → Worker` 的 VFS 路径，进闭包即可；
- 每包 `files` 覆盖（tsdown 共享 chunk）：打包前用 `verify` 相关门禁对齐 A 的闭包；
- 验收：A.5 矩阵全部在单文件形态复跑 + `mod` 校验 web 前端资源从 VFS 可读。

---

## 发布/交付物清单

- A 完成：`DSH-Office-windows-x64-setup.exe`（直装闭环）、`DSH-Office-windows-x64-(closure).zip`（绿色版）、
  安装版更新走 WinSparkle AppCast；`frontend/client/doc/packaging-windows.md` 产物表同步。
- B 完成：setup 内含单活 exe + sidecar；zip 版同构。
- 证书：生产签名证书就绪后接入 A.4；发布前跑 A.5 全矩阵 + 一键装插件 E2E。

## 待决策项（实施前拍板）

1. 是否保留便携 zip（绿色版）作为正式分发之一（推荐保留）。
2. pnpm sidecar 形态：保留 node.exe（推荐，dsh-desktop 同款、零改造）vs standalone pnpm（更瘦、改 runner）。
3. B 阶段是否把 `plugin-tools` 的 node 也去掉（仅当市场安装可完全迁到 server-side 方案，暂不建议）。
4. codex/claude 两包剔除后，若将来要做「本地子代理」功能，需反悔清单 + 重新启用（低代价）。