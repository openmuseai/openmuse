<!--
Thanks for contributing! Quick check before you submit / 提交前快速自查
-->

### ⚠️ Adding a plugin to the catalog? Wrong repo. / 收录插件或提交截图？走错仓库了

**This repo is the market app.** The plugin list — and `data/screenshots.json` — live in the catalog repo:

👉 **https://github.com/awesome-dsh-plugin/awesome-dsh-plugin**

Plugin entries and screenshots merged there reach the market automatically on the next daily registry refresh; nothing needs to change here.

**本仓库是市场应用本身。** 插件收录条目与 `data/screenshots.json` 都在收录列表仓库（链接同上）。在那边合并后，市场会随每日 registry 刷新自动展示，本仓库无需改动。

---

### For code changes / 代码改动

- [ ] `npm test` and `npm run check` pass locally / 本地通过
- [ ] Tests cover the change — for a bug fix, one that fails without it / 新增测试覆盖改动；bug 修复请确保测试在修复前是红的
- [ ] No version bump in `package.json` — releases are a maintainer action / 不要改版本号，发版由维护者操作
- [ ] `client/client.js` rebuilt with `npm run build:client` if you touched `src/client/` / 改动前端后重建产物

<!-- What changed and why / 改动内容与原因 -->
