# 应用中心人工审核入口

状态：**用户已批准实施（2026-11-02）**。本页保留原设计审核基线；Flutter 实现、真机截图与产物见 [实施与验收记录](IMPLEMENTATION.zh-CN.md)。

- [实际 iPhone 页面图库](implementation.html) / [实施与验收记录](IMPLEMENTATION.zh-CN.md)
- [产品范围、命名、状态规则、框架对应与验收清单](PRODUCT.zh-CN.md)
- [可点击原型](index.html)；本机预览运行于 <http://127.0.0.1:16085/>。
- 本地重新打开：在仓库根目录运行 `python3 -m http.server 16085 --bind 127.0.0.1 --directory docs/design/mobile-app-center`。

## 方案

底部：Home / Tasks / Apps / Account。

Apps 顶部：Overview（默认，承接 Workbench）/ Discover。

My Plugins：Account 下的独立管理页。

## 审核截图

| 场景 | 截图 |
| --- | --- |
| 中文默认概览，底部四项 | [概览](screenshots/01-overview.png) |
| Discover 内容结构 | [发现](screenshots/02-discover.png) |
| Account 新入口 | [我的](screenshots/03-account.png) |
| My Plugins 二级页 | [我的插件](screenshots/04-my-plugins.png) |
| 演示安装完成后的详情 | [安装详情](screenshots/05-installed-detail.png) |
| 英文深色 | [English / Dark](screenshots/06-english-dark.png) |
| 错误状态与重试 | [错误状态](screenshots/07-error-state.png) |

## 原型自查

已通过浏览器实际点击检查：栏目切换、底部四项、整理后离开提示与放弃、Account → My Plugins → Discover → 返回、模拟安装中到安装完成、英文深色、错误与重试。390px 手机框的导航为四个底部项与两个顶栏，审稿页没有横向溢出。JavaScript 语法检查通过。

这些检查仅针对 HTML 设计稿，不是 Flutter 功能实现或真机验收。空白/加载/离线可通过原型控制器查看；插件执行、目录来源与模型设置只展示入口，真实权限、尺寸菜单和完整无障碍验收按产品稿后续实现。

已按用户批准采用 Overview 命名，以及会话内恢复上次栏目、重复点击 Apps 回到 Overview 的规则。
