# Muse-Clients / frontend/web 品牌化方案与实施计划

> 总册：[`../../docs/rebrand-plan.md`](../../docs/rebrand-plan.md)（若在 web 仓内阅读，见父工作区 `Muse-Clients/docs/rebrand-plan.md`）
> 对照：官网第一阶段；本树 **不重命名** npm 包、CSS 类、`APPFLOWY_*` 环境变量、DSH pluginId。
> 栈：React + Vite + TypeScript；`package.json` name 仍为 `appflowy_web_app`。
> 注意：`frontend/web/` 在 Muse-Clients 父仓是 **gitignore 的独立 git 仓库**。本文件应提交到 **web 仓**；父仓副本在 `Muse-Clients/docs/rebrand-plan-web.md`。
> 状态图例：✅ 已完成 ｜ 🔄 进行中 ｜ ⬜ 未开始 ｜ ⏸️ 等待外部输入
>
> 2026-08 dsh-office v1 **不含 Web**（当时排除 AppFlowy-Web）。官网「Start for free」已指向同域 `/app`，本轮必须做 Web 可见层。历史审计计数可参考 `local/docs/rebrand/01-BRAND-AUDIT.zh-CN.md`。

---

## 一、进度总览

| 阶段 | 状态 | 备注 |
|---|---|---|
| **W0** 基线锁定 | ⬜ | `pnpm lint` / 开发服冒烟（本轮未截图） |
| **W1** 品牌配置层 | ✅ | `brand/config.yaml` + `src/config/brand.ts` + `scripts/apply-brand.py` |
| **W2** 文档元数据 / favicon / OG | ✅ | `index.html` + `public/` Muse 资产 |
| **W3** i18n | ✅ | 全部 locale `appName`；en/zh 写死名已替换；**不改 key** |
| **W4** Logo 组件 | ✅ | `ProductLogo`；侧栏/落地/Powered by |
| **W5** 硬编码文案 | ✅ | Login / Helmet / 错误页 / AI 移动提示 |
| **W6** 外链 / 法务 / 帮助 | ✅ | 空 URL 隐藏；`af_icons` 走本仓 `/af_icons` |
| **W7** 品牌色 | ⏸️ | 与官网 P8 / Client C8 同一份 |
| **W8** Deep link 与下载链 | ✅ | `openmuse://`，保留 `appflowy-flutter://` 配置 |
| **W9** README / 部署文档 | ✅ | 开发者可见品牌 |
| **W10** 内部标识符重命名 | ⬜ 明确不做 | 类名、env、MIME、pluginId |

---

## 二、现状盘点（实施前基线）

Web 几乎未做产品品牌化，与官网 / 桌面客户端落差最大。

### 2.1 入口与 SEO（用户第一个像素）

`index.html`：

- `<title>AppFlowy</title>`
- favicon `/appflowy.ico`
- canonical / og:url `https://appflowy.com`
- og/twitter 文案与 `@appflowy`
- description 仍是上游句式

`ViewHelmet` 默认 favicon 回退 `/appflowy.svg`。

### 2.2 登录与壳

| 位置 | 问题 |
|---|---|
| `src/components/login/Login.tsx` | `{t('welcomeTo')} AppFlowy` **硬编码**；terms/privacy → appflowy.com |
| `src/assets/icons/logo.svg` | 上游花 Logo |
| `src/assets/icons/appflowy.svg` | 横向 lockup，被侧栏 / 落地 / Powered by 引用 |
| `AppFlowyPower.tsx` | 「Powered by」+ 上游 Logo，点击 appflowy.com |
| `LandingPage.tsx` / `OutlineDrawer` / `CurrentWorkspace` / `RequestAccess` 等 | 直接 `import ... appflowy.svg` |

### 2.3 i18n

`src/@types/translations/` 共 **33** 个 json。`en.json` / `zh-CN.json` 的 `appName` 仍是 `"AppFlowy"`，另有大量写死句（发布、AI、备份、订阅问卷、`web.signInAgreement` 等）。

### 2.4 外链与环境

| 类型 | 现状 | 第一阶段策略 |
|---|---|---|
| `APPFLOWY_BASE_URL` 等 | 本地指向 `localhost:3001` | **不改变量名**；值本来就可以指向自建后端 |
| `deploy/config.ts` `APPFLOWY_SITE_URL` | 默认已是 `https://openmuseai.com` | 保持 |
| 条款 / 隐私 / 帮助 / 模板 | 硬编码 appflowy.com | 改读 `brand.ts` |
| `openAppFlowySchema` | `appflowy-flutter://` | W8 与桌面并行 scheme |
| CSS `--text-featured: #9327ff`、`--border-theme-thick: #00b5ff` | 上游色 | W7 再改 |
| 类名 `appflowy-scrollbar`、`__APPFLOWY_EVENT_EMITTER__` | 内部 | 不改 |
| `@appflowyinc/editor` | npm 依赖 | 不改 |
| DSH `muse.appflowy-web` | 协议 | 不改 |

### 2.5 规模

- 含 `appflowy` 的源文件约 **403**（排除 node_modules）
- 命中约 **1612** 处；其中真正要改的是 **文案、Logo、html 元数据、外链**，大约几十个文件

---

## 三、方案原则

1. **配置先于业务**：W1 的 `brand.ts` 对齐官网 `lib/config/site.ts`（产品名、siteUrl、法务、开关）。
2. **一层一个 commit**。
3. **i18n 只改 value，不改 key**（避免 33 个 locale + `t('publish.createWithAppFlowy')` 全仓库改名）。
4. **Logo**：方形用官网 `logo.svg` 覆盖 `src/assets/icons/logo.svg`；横向 lockup 未交付前，侧栏改为「图标 + `brand.productName`」，不要继续用带「AppFlowy」字样的 SVG。
5. **空链接隐藏**（官网下载方案同一纪律）。
6. **env 名冻结**：部署文档、Docker、Vite 都认 `APPFLOWY_*`；可增加读取别名 `OPENMUSE_*`，但不要删旧名。

---

## 四、各阶段详细计划

### W0 — 基线锁定（约 0.5 天）⬜

| 动作 | 内容 |
|---|---|
| 1 | 记录 web 仓 HEAD；确认 `pnpm lint`（type-check + eslint）可跑 |
| 2 | 本地 `pnpm dev`，截图登录页、登录后侧栏、浏览器 tab 标题与 favicon |
| 3 | 记下当前 `index.html` 与 `appName` |

**验证**：开发服可打开登录页。不要在有 dev 占用时强行并行生产 build（官网 P4 事故：dev 与 build 抢产物目录）。

---

### W1 — 品牌配置层（约 0.5 天）⬜

新增 `src/config/brand.ts`（值由 `brand/config.yaml` 经 `scripts/apply-brand.py` 生成，勿在 TS 里写死产品名）：

```ts
export const brand = {
  productName: 'OpenMuse',
  siteUrl: 'https://openmuseai.com',
  appPath: '/app',
  termsUrl: 'https://openmuseai.com/terms',
  privacyUrl: 'https://openmuseai.com/privacy',
  downloadUrl: 'https://openmuseai.com/download',
  githubUrl: 'https://github.com/openmuseai/openmuse',
  twitterHandle: '',
  discordUrl: '',
  docsUrl: '',
  desktopScheme: 'openmuse://',
  legacyDesktopScheme: 'appflowy-flutter://',
} as const;

export function isConfiguredUrl(url: string | undefined): boolean {
  return typeof url === 'string' && url.trim().length > 0;
}
```

后续组件禁止再写死 `https://appflowy.com/...`。

**验证**：type-check 通过；行为暂可不变（W2 起接线）。

---

### W2 — 元数据、favicon、OG（约 0.5～1 天）⬜

| 文件 | 改动 |
|---|---|
| `index.html` | title / description / canonical / og:* / twitter:* → OpenMuse AI 与 `openmuseai.com`；favicon 改为 `/favicon.ico` 或 `/favicon.svg` |
| `public/appflowy.ico`、`public/appflowy.svg` | 换成 Muse 资产后 **改名或删旧文件**，避免书签仍请求旧名 |
| `public/og-image.png` | 用官网已裁好的 1200×630（`Muse-WebSite/public/images/og-image.png`） |
| `src/components/_shared/helmet/ViewHelmet.tsx` | 默认 favicon `/favicon.svg` |
| 若有 `manifest.webmanifest` / apple-touch | 用 `muse/web-app/` |

资产从 `Muse-WebSite/public/images/muse/favicon/` 与 `web-app/` 拷贝，不要用参考板原图。

**验证**：浏览器 tab 标题与图标；curl HTML 无 `appflowy.com`、无 `<title>AppFlowy</title>`；浅色/深色 tab 可辨认。

---

### W3 — i18n（约 1 天）⬜

目录：`src/@types/translations/`。

**3a 必须**：`en.json`、`zh-CN.json`

- `appName`: `"OpenMuse AI"`
- 所有仍写死 AppFlowy 的 **value** 改为产品名或 `@:appName`（i18next 若已支持该插值；web 的 `@:appName` 与 Flutter 类似，保持项目现有写法）
- `web.signInAgreement` 等协议句

`en.json` 已知写死项（不完全表）：

- `appName`
- `publishToTheWebHint` / `publishOnAppFlowy` / `createWithAppFlowy`
- `downloadApp`（多处）
- `web.signInAgreement`
- AI：`Ask AppFlowy AI`、`downloadAppFlowyOfflineAI`
- 备份 / 导入 AppFlowy zip
- 订阅取消问卷

**3b**：其余 31 个 locale 同步 value。可用脚本把各文件 `"appName": "AppFlowy"` 及常见短语替换；人工过 en / zh-CN 即可。

**不要改 key**（`createWithAppFlowy`、`cloudAppFlowy` 等）。

**验证**：

```bash
rg -i "appflowy" src/@types/translations/en.json src/@types/translations/zh-CN.json
```

应为空。登录页、侧栏工作区名、发布提示、404「Create with …」抽查。

---

### W4 — Logo 视觉替换（约 0.5～1 天）⬜

| 文件 | 改动 |
|---|---|
| `src/assets/icons/logo.svg` | 覆盖为官网方形 `logo.svg`（登录页已引用） |
| `src/assets/icons/appflowy.svg` | 停止作为带字 lockup；或改成纯图标并 **逐步** 让引用方改用新组件 |
| 新 `src/components/_shared/brand/ProductLogo.tsx` | 方形图标 + 可选文字 `brand.productName`；适配 currentColor / 明暗 |
| `Login.tsx` | 已有 `logo.svg`，确认尺寸（h-9 w-9） |
| `OutlineDrawer` / `CurrentWorkspace` / `LandingPage` / `RequestAccess` / `PublishMobileFolder` / `ImporterDialogContent` | 改用 `ProductLogo` |
| `AppFlowyPower.tsx` | 「Powered by OpenMuse AI」；点击 `brand.siteUrl`；可改组件文件名（非必须） |

无官方横向 SVG 时不要把 1254² 方图硬塞进 118×宽的顶栏。

**验证**：登录、登录后左上角、落地、发布页、Powered by；明 / 暗主题。

---

### W5 — 硬编码文案（约 0.5 天）⬜

i18n 清完后仍可能漏在 TSX：

| 文件 | 改动 |
|---|---|
| `src/components/login/Login.tsx` | `{t('welcomeTo')} AppFlowy` → `{t('welcomeTo')} {t('appName')}` 或单一 key |
| `NotFound.tsx` | 走 i18n，不再依赖未改的 `createWithAppFlowy` value |
| 其它 `rg "AppFlowy" src --glob '*.tsx'` | 用户可见字符串清零 |

**验证**：

```bash
rg -n "AppFlowy" src --glob '*.tsx' --glob '*.html'
```

残留应仅为类型名 / 注释 / `isAppFlowyHosted` 等内部函数（可列白名单）。

内部函数名（`isAppFlowyHosted`、`isAppFlowyFileStorageUrl`、`openAppFlowySchema`）**第一阶段允许保留**，只保证 UI 字符串已换。若 W5 顺手加 `alias` 导出，不要做破坏性重命名。

---

### W6 — 外链 / 法务 / 帮助（约 0.5 天）⏸️ 部分等待账号

全部改为读 `brand.ts`，空则隐藏：

| 文件 | 现状 |
|---|---|
| `Login.tsx` | terms / privacy → appflowy.com |
| `Help.tsx` | what-is-new、getting-started-with-appflowy |
| `ManageDataPanel.tsx` | import / backup 指南 |
| `MobileMore.tsx` | templates.appflowy.com |
| `AppFlowyPower.tsx` | appflowy.com |
| `chat/lib/utils.ts` | `https://appflowy.com/af_icons/...` **图标 CDN** |

`af_icons` 若仍托管在上游 CDN：第一阶段改为 **本仓 `public/af_icons`**（已有大量 SVG）或自建路径，不要在生产里继续打 appflowy.com 拉图标。

Discord / 文档 / Twitter：与官网 P6 相同，账号未定则隐藏。

**验证**：登录协议、帮助菜单、Powered by；Network 面板无 appflowy.com（图标 CDN 切换后）。

---

### W7 — 品牌色 ⏸️

等设计。到时改 token 源 JSON 再跑：

```bash
pnpm generate-tokens
# 或 node scripts/system-token/convert-tokens.cjs
```

生成文件 `src/styles/variables/semantic.*.css` 标明 AUTO-GENERATED，不要手改生成物。未交付前保持现有蓝 / 紫。

---

### W8 — Deep link 与下载（约 0.5 天）⬜

| 文件 | 改动 |
|---|---|
| `src/utils/url.ts` | `desktopDownloadLink` → `brand.downloadUrl`；scheme 优先 `openmuse://`，回退 `appflowy-flutter://` |
| `src/utils/open_schema.ts` | 同上 |
| `AppContextConsumer.tsx` | 写死的 `appflowy-flutter://open-page?...` 改为 brand.scheme，并保留旧 scheme 作为 fallback |

与 Client C9 **同一天联调**：Web「在应用中打开」按钮。

**验证**：未装应用时落到官网下载页；已装且注册了新 scheme 时能唤起。

---

### W9 — README / 部署文档（约 0.5 天）⬜

`README.md`、`doc/DEPLOYMENT.md`、`desktop.md` 顶部仍是「AppFlowy Web / appflowy.com」。改成 OpenMuse AI 用户向描述；保留「derived from AppFlowy-IO」的许可证声明（NOTICE）。

**验证**：新贡献者按 README 不会被带到 appflowy.com 去「官方安装」。

---

### W10 — 内部标识符：明确不做

包括但不限于：

- `package.json` `"name": "appflowy_web_app"`
- `APPFLOWY_BASE_URL` / `APPFLOWY_GOTRUE_BASE_URL` / `APPFLOWY_WS_BASE_URL`
- CSS 类 `appflowy-layout`、`appflowy-scrollbar`
- `window.__APPFLOWY_EVENT_EMITTER__`、MIME `application/x-appflowy-fragment`
- DSH `MUSE_DSH_PARENT_SOURCE = 'muse.appflowy-web'`
- npm `@appflowyinc/editor`

改这些会牵动 Docker、nginx、插件契约与剪贴板互通，不属于品牌化第一阶段。

---

## 五、实施清单（建议 commit 切分）

| Commit | 阶段 | 主要内容 |
|---|---|---|
| 1 | W1 | `src/config/brand.ts` |
| 2 | W2 | `index.html` + public 图标 / OG |
| 3 | W3a | en.json + zh-CN.json |
| 4 | W3b | 其余 translations |
| 5 | W4 | ProductLogo + 替换引用 |
| 6 | W5 | Login 等硬编码 |
| 7 | W6 | 外链 + af_icons 本地化 |
| 8 | W8 | scheme / 下载 URL |
| 9 | W9 | README |

每步：`pnpm lint`；有 dev 在跑时不要并行 `pnpm build`。

---

## 六、验证与回滚

| 命令 | 用途 |
|---|---|
| `pnpm lint` | type-check + eslint |
| `pnpm build` | 仅在 **没有** 占用同一 `dist/` 的 dev/preview 时跑 |
| `rg -i "appflowy" src/@types/translations/en.json src/@types/translations/zh-CN.json` | 主语言 |
| `rg -i "appflowy.com" src index.html --glob '!**/af_icons/**'` | 外链 |
| 浏览器 | `/` 或登录页 title、favicon、欢迎语、侧栏 Logo |

冒烟：浏览器 tab → 登录欢迎语与协议链接 → 登录后左上角 → 帮助/Powered by →（W8）打开桌面应用。

回滚：按 commit `git revert`。`public/` 二进制与 `brand.ts` 均为数据层，回滚安全。

---

## 七、风险

| 风险 | 缓解 |
|---|---|
| 只改 `appName`，Login.tsx 仍拼接硬编码 AppFlowy | W5 专门扫 TSX |
| `appflowy.svg` 是带字 lockup，直接换方图会把顶栏撑破 | W4 上组合组件，限定高度 |
| 帮助链接仍去上游文档 | W6 空 URL 隐藏 |
| `af_icons` 继续请求 appflowy.com | 改用 `/af_icons` 本地 public |
| 与桌面 scheme 不一致导致「打开应用」失败 | W8 与 C9 一起做，保留旧 scheme |
| 33 个 locale 漏网 | W3b 脚本 + en/zh 人工 |
| 父仓看不到 web 改动 | 文档已镜像到 `Muse-Clients/docs/rebrand-plan-web.md`；代码在 web 仓单独提交 |
