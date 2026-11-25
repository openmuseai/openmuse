# 首发组成与依赖清单（实施批次 1）

日期：2026-10-30。移动候选版本 `1.0.0 (5)`。本清单区分源码装配、候选包验证和生产发布；不代表商店审核通过。机器可读目录见 `official-launch-plugins.json`。

| 模块 | Android / iOS 装配 | 首发能力边界 | 验收入口 |
| --- | --- | --- | --- |
| Host / WorkBuddy | 保留 | 地区选择、账户、对话草稿、工作台、设置 | `globalization_acceptance_test.dart` |
| 认证 | `com.openmuse.auth.gotrue` | 美国端登录、恢复、退出；账户删除走真实 Cloud API | 同上及账户删除测试 |
| 云工作区 | `com.openmuse.workspace.cloud` | 已有工作区/资源/远端会话 | 真机账号路径；连接服务测试 |
| 设备配对 | `com.openmuse.workspace.paired.mobile` | 设备注册、目录、原生加密及已有配对协议 | 设备目录和 device_keystore 集成测试；真实 Desktop 配对另记 |
| 远程工作台 | `com.openmuse.remote-workbench` | 保留已实现入口，不扩大权限 | 现有 remote_workbench 测试 |
| 文件与定时任务 | `com.openmuse.mobile.workspace-files` / `com.openmuse.mobile.scheduled-tasks` | 保留文件授权、调用及已有时区语义 | 插件/工作台测试及真机联验 |
| DSH Agent | `com.openmuse.dsh-mobile-agent` | 原生对话、审批、问答、工具过程和产物；历史内容不翻译 | 共享 14 项对话测试及真机路径 |
| 语音 | `com.openmuse.speech.input` | 保留麦克风入口和本机 `zh14m` 模型；仅按普通话能力验收，不能据英文 UI 宣称英语识别 | 现有 ASR 用例；最终模型安装/权限仍须渠道联验 |
| 市场 | `com.openmuse.mobile-plugin-market` | 保留发现、安装、既有安装和源管理；正式随包目录不推荐开发示例和旧 PPTX 文本阅读器 | 市场、离线安装和源管理测试 |
| Word/PDF/表格 | 保留签名产品资源和接口 | 商店工程无内核依赖、原生链接、自动加载或入口；实验页面移至独立 `openmuse_mobile_office_lab` | `verify-mobile-closure.py`、最终 AAB/IPA 扫描 |
| GenOffice | 官方拟发布 `io.openmuse/genoffice-pptx@0.8.0` | 独立离线 MCP Apps 的 PPTX 路径；通过逐端编辑/撤销/保存验收才登记就绪 | `genoffice_globalization_acceptance_test.dart` |

桌面参考装配：主 Host 和 Linux 均单独核对，默认组合包含 DSH、CLI、Helix、市场、文件查看和开发诊断；账户、Cloud、配对由启动层装配。共享原生对话和 Cloud 文案已整改，但桌面 Office 原生链路、系统菜单和最终包尚未在本批完成验收，不据移动成功宣称桌面通过。

Web 参考装配：共享认证/配置/语言与固定地区 Web 部署，DSH 仍是同页挂载的原客户端。共享消息契约更新到 revision 2；Web DSH 语言桥、浏览器矩阵和最终区域部署尚未在本批签收。

正式供给目录与用户已有安装分开管理。`assets/official-market.json` 不提供旧开发示例；原示例目录、已安装资源及第三方源机制保留。线上 MCP 官方目录仍需由发布侧应用首发清单，并发布 GenOffice 0.8.0；客户端不通过隐藏服务器条目或清空用户缓存伪造供给整改。

跨仓库依赖：商店移动工程已不依赖 `ioffice` 或旧 GenOffice Flutter 只读预览。实验包保留其原依赖；GenOffice 验收 fixture 在构建外暂存到开发应用私有目录，绝不作为商店测试凭据/样本文稿资产随包。

本批实测：Android（Pixel 9）与 iOS（iPhone Air）的 GenOffice 0.8.0 均通过原生 WebView 安装、编辑、切换语言保留文稿、撤销及保存后 PPTX 内容检查。两端另通过真实账号 grant、公网 relay 及 Linux 受控文件读取，以及独立 fixture 账号永久删除和后端清理。独立 keystore 协议的完整安全验收另记；官方线上目录尚未应用首发清单。详见 [移动验收记录](../app/openmuse_mobile/release_materials/14-全球化移动验收-2026-10-30.zh-CN.md)。
