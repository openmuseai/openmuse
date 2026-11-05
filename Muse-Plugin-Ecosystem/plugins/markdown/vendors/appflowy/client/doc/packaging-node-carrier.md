# node carrier 实施：捆绑 pnpm + `.desktop-bin` shim（B 路线）

> 状态：**已实施并端到端验证**。已把 dsh-desktop（`vendors/dsh-desktop`）的
> 「一键装插件」机制移植到 Muse 的 Windows 打包脚本：
> 捆绑 node + 捆绑 pnpm + `pnpm-runner.mjs` + `$DSH_HOME/.desktop-bin` shim。
> 运行时形态走 B（node carrier）：node.exe + pnpm.cjs（≈ dsh-desktop 形态）。
> 与 SEA 路线的取舍见 [packaging-official-single-exe-comparison.md](./packaging-official-single-exe-comparison.md) 4.5–4.6。

## 为什么是这套

dsh-desktop（官方 Electron 桌面端）的一键装插件链路（证据在 `vendors/dsh-desktop`）：

```
市场 UI → dshmarket（运行在 harness 内）
  → 重新 spawn 当前 CLI：`<dsh> plugin --profile <p> <pnpm 参数>`
  → CLI 的 plugin 子命令（apps/cli/src/plugin.ts）在 $DSH_HOME/profiles/<p> 里跑 `pnpm`
  → `pnpm` 是 $DSH_HOME/.desktop-bin/pnpm.cmd → `"<bundled node>" pnpm-runner.mjs <pnpm.cjs> %*`
  → pnpm-runner.mjs 包一层 Windows 锁恢复 / idle 超时 / 进程树清理后调真正的 pnpm
```

关键点：

- **node 与 pnpm 都随包携带**：官方用 npm 依赖 `"node": "24.9.0"` + `"pnpm": "10.34.5"`
  （node_modules/node/bin/node.exe + node_modules/pnpm/bin/pnpm.cjs），**用户机器零安装**。
- **shim 写在用户区** `$DSH_HOME/.desktop-bin/`，运行时生成，指向包内**绝对路径**；
  该目录 + node 目录前置进 harness 子进程 PATH（profile-plugin-command.ts 同款）。
- **pnpm-runner.mjs 是所有 pnpm 操作的统一入口**：Windows 锁重命名恢复
  （`<pkg>_tmp_<pid>_<n>` → `<pkg>` 换名 EPERM 时重试、把阻塞目录挪成 `.dsh-old-<ts>`
  再装、taskkill 杀进程树）、5 分钟 idle 超时、git prepare 审批映射、
  generation-projection 隔离（本移植暂不启用投影隔离字段）。

## Muse 侧落地清单（已完成）

| 项 | 位置 | 状态 |
|---|---|---|
| `pnpm-runner.mjs`（移植，marker 前缀 `muse pnpm runner:`，knob 环境变量 `MUSE_PNPM_*` 兼容 `DSH_DESKTOP_PNPM_*`） | `frontend/client/scripts/lib/pnpm-runner.mjs` | 完成 |
| 捆绑 pnpm（`npm pack pnpm@10.34.5 --cache <repo>` → `muse/plugin-tools/pnpm/` 全包 + runner） | `muse_windows.stage_plugin_tools()` | 完成 |
| `.desktop-bin` shim（**绝对路径**，win: pnpm.cmd/node.cmd；posix: pnpm/node） | `muse_windows.write_desktop_bin_shims()` + `dsh_sidecar.dart::ensureDesktopBinShims` | 完成 |
| pack 流程集成 | `pack-windows-client.py` reuse 分支 + `stage_dsh_runtime` + `assert_packed_runtime` | 完成 |
| sidecar 环境：PATH 前置 `.desktop-bin` + node 目录；`npm_config_side_effects_cache=false`、`PNPM_CONFIG_SIDE_EFFECTS_CACHE=false` | `dsh_sidecar.dart::_spawn` | 完成 |
| 顺带修复：dshmarket peers 修复 junction 安全；`_extract_npm_tarball` 去 mkdtemp/清只读位；临时目录锚进 repo cache | `muse_windows.py` | 完成 |
| 端到端验证 | 见下 | 完成（附一处遗留发现） |

**注意：`vendors/deepseek-harness` 是到 `D:\agentic\deepseek-harness` 的 junction，属另一仓库，
实施只读它、不修改。CLI 宿主（`E2E_CLI_ROOT`）与 shim 指向的 bundled node/pnpm 分离。**

## 闭包里程碑（2026-09-03 追加）：npm tarball 闭包 + 可分发 zip

在移植一键装插件机制之后，按「改走零符号链接闭包」推进，产出：

| 项 | 结果 |
|---|---|
| 全量构建 | `DSH_BUILD_CLIENT_PROFILE=official pnpm run build`（pack.ts 校验 official record；build 含 web/client 面） |
| tarball | harness `scripts/release/pack.ts --family dsh（242）/vendor（9）`，**pack.ts 会清空 --out，故每 family 独立子目录** |
| 闭包 | `frontend/client/scripts/build-muse-closure.py`：manifest 含 265 依赖（dsh+vendor tarballs + @muse file: + dshmarket@latest 1.41.0），`npm install --omit=dev` → **772 packages**，**零符号链接**（@muse file: 装的 junction 已解引用）；`dshmarket` 旧版(1.31.1) peer 与 alpha.5 冲突 → 升级 latest + `overrides.dshmarket.@deepseek-ai/dsh-settings` |
| 产物 | `frontend/client/dist/windows/DSH-Office-windows-x64-closure.zip`：`muse/{node/node.exe, closure/node_modules+package.json, plugin-tools, patch.yml}`，**370 MB / 35,356 文件**（旧整树 zip 601 MB 且 junction 内容缺失、解压 2.2 GB） |
| Flutter 适配 | `dsh_sidecar.dart`/`dsh_runtime.dart`：bundled 分支检测 `closure/node_modules/@deepseek-ai/dsh/lib/bin.js`，有则直接 `node <closure 入口> web --patch …`（无 tsx）；`looksLikeBundleRoot` 兼容 closure 布局 |
| 结构校验 | @deepseek-ai 252、dsh lib/bin.js、@muse 13（0 reparse）、dshmarket 1.41、web-frontend dist、cordis 全在 |

### 启动验证状态（遗留项：profile 解析通道）

`node <closure/bin.js> web --patch <muse/patch.yml>` 已能启动到 Loader（web 基础 bundles 加载），
卡在 **patch 裸名插件（@muse/*、dshmarket）从 profile 目录解析**：
- Loader 的 `healProfilesModuleFallback` 只镜像 dsh 安装闭包，且**写符号链接**——本机 Node 22.19
  ESM 解析拒绝 node_modules 里的符号链接条目（实体拷贝 probe 通过、链接 probe 失败）；
- 手工向 `$DSH_HOME/profiles/{,web}/node_modules` 种拷贝会被 **loader 启动时清理**（目录归它管，pnpm 管理的条目除外）。

**结论/下一步（官方通道）**：patch 裸名插件应以 **`dsh plugin --profile web install`/`add` 经 bundled pnpm
装进 profile**（pnpm 默认 hardlink→实体目录，loader 不清理；这正是 dshmarket 安装链路已覆盖的路径，
闭包 tarballs 已就位可直接作 file: 源）。完成该步后闭包即可全量启动；侧车在 closure 模式下无需
手工种子。

## 分发与安装器（exe 直装）方案 —— 规划，未实施

目标：用户拿到一个 **setup.exe 双击即装**（普通用户），保留 zip 绿色版（可选）。现状基底：
- 闭包 zip 已产出（370 MB）；Inno 配置已存在
  「`frontend/client/frontend/scripts/windows_installer/inno_setup_config.iss`」：
  `PrivilegesRequired=lowest`（装到 `%LOCALAPPDATA%\Programs\DSH Office`，免 UAC）、LZMA2、
  桌面+开始菜单快捷方式、安装后启动、HKCU `dsh-office://` 协议；
- 唯一硬件缺口：**打包机未装 Inno Setup 6（ISCC.exe）**，pack 脚本会自动跳过 installer；
- 用户数据不写入安装目录（`%APPDATA%\DSH Office\Muse\`），与安装位置解耦。

### 方案 A（近期推荐）：安装器打包「闭包便携目录」

`setup.exe ≈ 320–350 MB（LZMA2 压 closure 布局）`，装完即跑。要做的事：

1. **ISCC 就绪**：打包机/CI 安装 Inno Setup 6（可静默下载安装），解除 `pack-windows-client.py` 的 installer 跳过。
2. **闭包进 pack 流程**：把 `build-muse-closure.py` 的输出挂进 `stage_dsh_runtime`（产出
   `dist/windows/DSH Office/muse/{node,closure,plugin-tools,patch.yml}`），.iss 的
   `Source: "DSH Office\*"` 原样打包即可（旧 `muse/dsh` 2 GB 已不存在）。
3. **首启 profile 初始化（前置条件）**：patch 裸名插件（@muse/*、dshmarket）需在首次启动时
   经 bundled pnpm 装入 `$DSH_HOME/profiles/web`（`dsh plugin --profile web install`，
   file: 指向闭包 tarballs；幂等）。否则装完打开市场/插件面板不完整——**这是「直装即可用」
   的硬前提**（见「闭包里程碑」遗留项）。
4. **校验矩阵（装好后跑）**：干净用户装 setup → 启动 → sidecar 就绪（3080）→ 市场可见 →
   捆绑 pnpm 装一个插件 → 卸载不残留（用户数据保留在 %APPDATA%）。
5. **签名**：Authenticode 签 `dsh-office.exe` + `setup.exe`（SmartScreen/Defender）；
   `node.exe` 官方已签名；`pnpm` sidecar 需一起签或随包时间戳；Defender 对 standalone exe 会拦。
6. **更新**：客户端已带 WinSparkle（`auto_updater_windows_plugin.dll`）——安装版走 AppCast 推新
   setup；便携版另行。升级时 `UsePreviousAppDir=no` + 覆盖安装，用户数据不受影响。
7. **体积裁剪（可选，产品决策）**：闭包剔除 `dsh-subagent-codex`/`-claude-code`
   （codex/claude SDK 原生包，解包 ~500 MB）→ setup 可降到 ~200 MB；前提是市场不依赖这两个子代理。

### 方案 B（中期）：运行时单文件化（pkg --sea），安装器形态不变

把闭包用 `@yao-pkg/pkg@6.21.0 --sea`（bin=闭包 `lib/bin.js`，assets glob 覆盖
js/json/md/yml/`.node`/`.dll` + `dsh-web-frontend/dist`）打成
`deepseek-harness-sdk-runtime-win-x64.exe`（~174 MB 量级）+ `-rg.exe` sidecar；
安装器里放 exe + plugin-tools（pnpm runner 装插件）+ patch.yml，不再放 1.3 GB 目录。
收益：安装体积 ~200 MB、拷贝/启动快、AV 只扫一个 exe、内置集只读。
成本：win-x64 真机重打、node-pty ConPTY 进 VFS、profile 插件安装仍走 bundled pnpm sidecar
（VFS 装不进）。路线不变：**先 A 拿到直装闭环，再 B 砍体积**。

### 方案 C（不推荐）：MSIX/AppX 单包
签名/旁加载限制与市场、插件用户态生态冲突，无收益。

**结论**：近端子目标 = A（ISCC 就绪 + 闭包入 pack + 首启 profile 初始化 + 签名），
产出 `DSH-Office-windows-x64-setup.exe`；中期做 B。

> **完整路线方案**（步骤 0 剔除 codex/claude 载荷 + A 直装闭环 + B 单文件化，
> 含文件级动作、验证矩阵、体积与风险）见
> [packaging-setup-exe-plan.md](./packaging-setup-exe-plan.md)。

### 已知要点（脚本已固化）

- pack.ts 校验 official 构建记录；构建命令 `DSH_BUILD_CLIENT_PROFILE=official pnpm run build`。
- pack.ts 内部 spawn 裸 `pnpm`（Node 不认 .cmd）→ 需 `pnpm.exe` shim 前置 PATH（`tmp/pnpm-shim/`，C# 编译）。
- `--skip-pack` 保留 tarballs、刷新 dshmarket@latest。
- 闭包解包 1,045 MB（含 codex/claude SDK 原生包，与 dsh-desktop 全量发行一致；可按需裁剪）。

## 端到端验证（2026-09-03 实测）

1. **打包**：`build_muse_packages(skip_tests)`（13 个 @muse 包 + harness host libs）成功；
   `pack-windows-client.py --skip-app-build --skip-zip --skip-installer --skip-packages
   --reuse-runtime` 重新阶段化 dist（@muse 刷新 + wire + dshmarket + plugin-tools + patch.yml），
   `assert_packed_runtime` 通过（bundled node v22.19.0）。
2. **一键装插件 E2E（PASS）**：隔离 `DSH_HOME=tmp/e2e-dsh-home`，
   shim=`pnpm.cmd`→`"<bundled node>" pnpm-runner.mjs <pnpm.cjs>`；
   `node --import tsx/esm <E2E_CLI_ROOT>/apps/cli/src/bin.ts plugin --profile web add picocolors@1.1.1`
   → **exit 0**、`Done in … using pnpm v10.34.5`、profile deps `picocolors: 1.1.1`、
   `profiles/web/node_modules/picocolors` 落地、dsh 的 bundle reconcile 提示正常；
   再以 `cmd /c pnpm --version`（profile cwd）确认走 shim 的 pnpm 版本 == 捆绑的 10.34.5。
   脚本：`tmp/e2e-plugin-install.py`。
3. **Windows 锁恢复**：`pnpm-runner.mjs` 纯函数冒烟 PASS
   （`lockedRenameTarget` 正确拒绝非 node_modules 误报、识别 `_tmp_` 重命名；
   `sidelinePath`/`MARKER`/`RUNNER_PATH` 正确）——`tmp/smoke-runner-fns.py`。
   真实 EPERM 锁竞争无法确定性复现；以 dsh-desktop 的历史实现 + 移植保真 + 上述真实
   install 走 runner 为准。
4. **启动不受装包影响**：web profile `--dump-config`（测试 DSH_HOME，已装包在场）exit 0、
   组合树完整。**注**：完整 web 启动需要 web 面 client bundles（`pnpm run build`），
   本次只编了 host 面，故用 dump-config 替代真启动。

### 验证中发现并处理的打包树缺陷

- **整树 junction 拷贝的去重会指向缺依赖的副本**（copy_tree 的 DFS 先扫到 `packages/…`
  里的共享包，把它当唯一副本，顶层 `node_modules/tsx` 被重链到该副本，而副本缺
  `node_modules/esbuild`）→ CLI（tsx）无法启动，报
  `Cannot find package 'esbuild' imported from …\packages\workflow\workflow-worker-thread\node_modules\tsx\…`。
  临时手段：把顶层 tsx junction 重指 `.pnpm/tsx@4.22.4/node_modules/tsx`，并在真实
  tsx 位置补 `node_modules/esbuild` junction。**这是「整树拷贝 + junction」路线的
  根本缺陷证据，最终消除要靠闭包 deploy（deploy 树零符号链接）或 SEA VFS。**
- **zip 分层实测（2026-09-03）**：`DSH-Office-windows-x64.zip` 压缩后 **601 MB**
  （71,274 条目；解压总 ~1.8 GB，其中 muse/ 1,686 MB）。**该 zip 不可直接分发**：
  junction 去重让每个真实目录只打一次，顶层 `node_modules/@deepseek-ai/*`、
  `node_modules/tsx`、`@vscode`、`node-pty` 等 relink 位置在 zip 里 **0 条目**，
  解压后这些目录缺失、应用无法启动；要把 junction 树正确展平成 zip，必须把每个
  链接位实体化（体积会膨胀回数 GB）。旧 `zip_folder` 的 rglob 版本则相反——junction
  目录完全不被遍历，zip 缺全部链接内容。**当前可运行产物是便携目录
  `dist/windows/DSH Office/`（junction 原地生效）；正确的小体积 zip 必须来自
  零符号链接的闭包/SEA 树，不是整树 junction 拷贝。**
- E2E 的 CLI 宿主因此用 `E2E_CLI_ROOT=D:\agentic\deepseek-harness`（真实 pnpm 树），
  shim/bundled node/bundled pnpm 仍全部来自打包产物——被验证的机制不受影响。

## 沙箱注意（本机）

受限模式下 Node `child_process.spawn` 捕获子进程输出（`stdio: 'pipe'`）会 EPERM
（dsh-desktop runner / tsx/esbuild 均实测命中）；`%TEMP%`/mkdtemp（0700）目录不可再进入。
本次按规则升级（danger-full-access）执行构建与 E2E；代码侧已把临时目录锚进 repo cache、
`_extract_npm_tarball` 不再用 mkdtemp。