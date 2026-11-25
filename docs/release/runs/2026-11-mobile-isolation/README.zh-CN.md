# Mobile 文档依赖隔离版本发布范围

本轮从上架流水线的需求范围与开发阶段启动，目标是让 Mobile 安装包使用 SmartDocument 作为本机结构化文档和数据集组件，将 SiYuan、GenOffice 及旧 Office 原生实现留在独立插件侧。保留资源协议、插件注册、MCP 和配对 Desktop 连接能力。移出实现及随包资源，不以改包名或删除上游署名替代隔离。

规划版本为 **1.0.2**，拟覆盖国内 Android、中国区 iOS、Google Play 和美区 iOS；版本和渠道待产品负责人锁定，构建号须查询渠道后分配。既有 1.0.0 和 1.0.1 发布记录不变。本轮尚未冻结源码、生成正式包、上传或提审。

## Apple 拒审整改

2026 年 11 月 13 日，iOS 1.0.0 build 16 因名称中的 Muse 引用和 AI 数据分享披露及事前授权被拒。已从分类提交基线建立发布分支；本轮增加品牌与隐私两条 P0 整改线，详见 [重新提审计划](REVIEW-RECOVERY.zh-CN.md)。

## 功能边界

| 范围 | 本轮行为 | 验收要求 |
| --- | --- | --- |
| 本机文档和数据集 | 复用已经接入的 SmartDocument 与 Dataset 插件组合 | 新建、打开、编辑、保存、重启恢复、只读和权限撤销均通过 |
| 云文档 | 保留通用云连接、同步和资源所有权检查 | 真机验证云文档和附件，不绕过权限或跨资源复用会话 |
| SiYuan | 不编译、不注册、不携带其客户端插件；源码保留在现有插件目录 | Dart 解析依赖、iOS Pods、最终包均不含被排除实现 |
| GenOffice 和旧 Office | 不随包预装 Slides MCPB，不携带演示 PPTX 引擎与市场记录 | 首次启动不自动解包或安装旧引擎，离线功能不再承诺 Slides 本机编辑 |
| 通用插件连接 | 保留 MCP、WebView、资源协议、配对 Desktop 与 SDK | 插件缺失时提供明确不可用状态；不自动下载旧依赖作为兜底 |
| 其他基础能力 | 保留登录、会话、文件查看、语音及工作台 | 受影响功能回归，发布材料只描述实际支持能力 |
| 旧数据 | 不删除既有用户文档 | 明确旧格式、旧安装插件和缓存的迁移或不可用提示；禁止静默改写文档 |

SmartDocument 不等于完全自研无依赖。当前实际依赖包含 AppFlowy 编辑器、协作内核、Yrs、数学及日历组件；需要独立核对来源、修改范围和分发义务。

## 待办与验收

负责人列为所需角色，尚未指派个人。P0 全部完成后才能冻结代码和构建商店候选包。

| ID | 优先级 | 任务和完成条件 | 角色 | 状态 |
| --- | --- | --- | --- | --- |
| M01 | P0 | 锁定版本、渠道、设备范围与各渠道递增构建号 | 产品与发布 | 待办 |
| M02 | P0 | Mobile 不依赖共享 SiYuan 插件组合，直接组合通用 Notes 与 SmartDocument | 客户端研发 | 已改造，依赖检查通过 |
| M03 | P0 | 移除 Slides 随包预装、PPTX 演示资产和商店目录记录 | 客户端研发 | 已改造，最终包待验 |
| M04 | P0 | 用 SmartDocument 验收本机及云文档生命周期、嵌入、附件、撤权和只读 | 客户端与测试 | 已有接入，完整真机回归待办 |
| M05 | P0 | 处理升级后遗留 Slides 安装目录、恢复记录、旧格式入口与不可用提示，保留用户内容 | 插件与客户端研发 | 待办 |
| M06 | P0 | 审查 SmartDocument Rust 与 Dataset 中的 SiYuan URI 兼容字符串；确定移除或迁移方式并回归旧链接 | 文档研发 | 待办，严格包检查可能拦截 |
| M07 | P0 | 核对 SmartDocument 参考目录与实际编译代码的来源、许可证、衍生关系及必要署名；交付逐组件结论 | 文档研发与版权负责人 | 初查发现上游依赖，审查待办 |
| M08 | P0 | 生成 Dart、Cargo、Pod、Gradle、字体、WASM、Web 资源的构建时依赖清单和摘要 | 构建与版权负责人 | 待办，当前证据仅为源码层初查 |
| M09 | P0 | 检查 AAB/APK/IPA 的路径、文件内容、压缩 NOTICES、AOT/DEX、原生符号和嵌套 MCPB；不得包含被排除实现的标识 | 构建与测试 | 检查器已增强，最终包待验 |
| M10 | P0 | 冻结跨仓库源码和工具链，清理构建缓存后分渠道重建；核对原生库输入和 16 KiB 对齐 | 构建 | 待办，工作区已有大量未提交变更 |
| M11 | P0 | 更新 SDK/隐私清单、协议、截图、欢迎指南、商店功能说明及审核说明 | 产品与发布 | 待办 |
| M12 | P1 | 独立插件的版本、来源、许可、权限、安装卸载和连接兼容测试，宿主不内置其产品实现 | 插件研发 | 原实现保留，独立分发验收待办 |

## 版权检查边界

被排除的实现应连同对应包、离线资产和专属元数据一起移出发行闭包。仍在分发的开源代码、字体或资源必须保留其必要许可与署名；若必要声明包含目标标识，不能只删声明，应先查明来源并替换或移出对应实现。扫描器故意不豁免版权清单。

SmartDocument 的 vendor 目录含 AppFlowy 编辑器双许可文本以及 SiYuan 参考目录；参考目录的存在不能直接证明其代码被编译，也不能直接证明没有衍生代码。M07 必须完成代码来源与构建输入对应审查。字符串零命中只是一项技术验收，不能证明版权归属。

## 执行与关口

按 `stages/*.json` 保存十二个阶段的真实状态。当前范围为 partial，开发与隐私为 in_progress，其余阶段 pending。源码冻结、来源审查、真机回归和最终签名包检查完成前，不进入上传和提审。

```sh
python3 scripts/release/pipeline.py plan --run docs/release/runs/2026-11-mobile-isolation
python3 scripts/release/test_mobile_closure.py
python3 scripts/release/verify-mobile-closure.py app/openmuse_mobile/.dart_tool/package_config.json
python3 scripts/release/verify-mobile-closure.py /path/to/final.apk
python3 scripts/release/pipeline.py check --run docs/release/runs/2026-11-mobile-isolation --gate 09-submit
```

源码初查见 [source-audit.json](evidence/source-audit.json)。测试结果见 [verification.json](evidence/verification.json)。
