# Mobile App Root 与共享 Host Shell

> 状态：Accepted
>
> 子需求：M0
>
> 日期：2026-09-29

`openmuse_host_shell` 只包含 theme、route/application chrome 与 session/catalog/capability 注入合同。Desktop 继续使用原有 Plugin composition，但 theme 已从共享包导入；`openmuse_mobile` 只依赖共享 shell，不 import DSH、Helix、PTY、Node 或任何具体 Plugin 实现。

Mobile composition 通过端口注入登录状态、Cloud/Paired Workspace catalog 和 capability snapshot。当前 fixture lock 只用于开发；未来 C4 distribution lock 替换数据源，不改变 UI 依赖方向。

统一验收：

```bash
./scripts/test_mobile_app_root.sh
```

验收构建 Android APK 与 iOS `--no-codesign`，扫描 APK 排除 Desktop runtime，并运行共享 shell、Mobile 和 Desktop 全套 Flutter analyze/test，确保抽取无回归。
