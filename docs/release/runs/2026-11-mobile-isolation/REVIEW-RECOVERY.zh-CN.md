# iOS 拒审整改与重新提审计划

Apple 于 2026 年 11 月 13 日拒绝 iOS 1.0.0 build 16，涉及名称中对 Muse 的引用，以及第三方 AI 数据发送的披露和事前授权。此次发布在 Mobile 文档依赖隔离的基础上增加这两条独立整改线。仅移除 SiYuan 和 GenOffice 不能视为解决名称拒审；仅更新隐私政策不能替代 App 内发送前授权。

当前发布分支为 `codex/release/mobile-1.0.2-review-recovery`，从完成 19 个分类提交的 `093207c7` 建立。规划版本仍为 1.0.2；App Store Connect 可编辑版本、发行地区及构建号在构建前锁定，不预设下一个可用 build 为 17。没有执行推送、上传、回复 Apple 或重新提审。

## 拒审事实和证据

- App ID：6827192902；版本：1.0.0；build：16。
- Submission ID：feef7887-1297-4f62-afd1-ed77242ec0f5。
- Apple Review date：2026 年 11 月 13 日；设备：iPhone 17 Pro Max。
- 4.1(a)：审核明确指出名称引用 Muse，要求移除相关内容，或提供具备分发权利的书面证据。
- 5.1.1(i) 与 5.1.2(i)：明确告知发送哪些数据、发送给谁，并在发送前获得同意；政策同步说明收集方式、用途及第三方保护措施。

来源是用户在 2026 年 11 月 17 日提供的拒审原文和截图，并非本轮登录 App Store Connect 的查询。详见 [拒审事件](evidence/apple-build16-rejection.json) 与 [截图](evidence/apple-build16-rejected.png)。旧档案中 11 月 12 日的 in_review 观察早于这次拒审，不能作为当前已通过或仍在审的依据。

Apple 官方依据：[4.1 名称和仿冒规则](https://developer.apple.com/help/app-review/guideline-reference/4-1-prevent-copycat-rejection)、[5.1 隐私与数据使用规则](https://developer.apple.com/app-store/review/guidelines/#data-collection-and-storage)。

## 整改任务与完成标准

| ID | 阶段 | 任务与完成标准 | 责任角色 | 当前状态 |
| --- | --- | --- | --- | --- |
| R01 | 01 范围 | 决定采用独立名称，或提交合法分发权利的书面证据；新名称需检索同名与相似品牌；不能假定当前 OpenMuse 名称已获接受 | 产品与品牌负责人 | 待决策 |
| R02 | 02 开发 | 依据 R01 同步 iOS 显示名、App 内标题、图标、启动页、欢迎文档、元数据、截图与支持页面；保留必要上游许可，Bundle ID 的变更另行评估 | 客户端与设计 | 待 R01 |
| R03 | 03 隐私 | 列出所有 AI 数据路径：输入文字、历史、系统上下文、文档和附件、工作区资源、工具调用及日志；逐项登记网关、模型服务商、实际接收方与用途 | 客户端与服务端 | 待办 |
| R04 | 02 开发 | 检查所有发送入口是否在联网发送或进入发送队列前经过授权；涵盖原生对话、Workbuddy、WebView、Desktop 中转、重试与插件调用 | 客户端与插件研发 | 已有部分授权实现，覆盖审计待办 |
| R05 | 02 开发 | 授权界面列出真实接收方与将发送的数据类别；未知接收方阻断；取消不发送；切换 provider、网关、账号或范围后重新授权；支持撤回 | 客户端研发 | 现有原生实现和核心测试通过，范围及撤回待验 |
| R06 | 03 隐私 | 同步中英文隐私政策、App Privacy 问卷、SDK/数据清单和支持页面；第三方保护承诺需有事实和条款依据，不虚构不留存或不训练声明 | 产品与隐私负责人 | 待办 |
| R07 | 02 开发 | 完成 M02 至 M09 的依赖隔离、旧标识、升级迁移、SmartDocument 来源及许可证审查 | 文档与客户端研发 | 部分完成，沿用主待办 |
| R08 | 04 测试 | 录制全新安装的首次 AI 发送、拒绝、切换接收方、资源附加、撤回、退出登录和升级路径；断言未经许可时模型及工具端无数据请求；补 iPhone 17 Pro Max 场景 | 测试 | 待真机验收 |
| R09 | 05 构建 | 核对各渠道已使用构建号，冻结跨仓库源码、工具链和依赖；正式签名构建，不复用 build 16 的二进制或旧缓存 | 构建与发布 | 待办 |
| R10 | 06 二进制 | 对最终 IPA/AAB/APK 检查被排除包名、字符串、NOTICES、原生库和嵌套资源；从分发包验证名称、授权界面、权限及行为 | 构建与测试 | 待正式包 |
| R11 | 07 物料 | 编写逐条对应 4.1 和 5.1 的英文审核说明，提供实际修改、授权触发路径、演示账号与可重复步骤；附所需权利材料，更新真实功能截图 | 产品与发布 | 待整改完成 |
| R12 | 08 至 10 | 所有前置阶段通过后上传新 build，完成 TestFlight 安装验证，再正式回复与重新提审；登记新 submission/build ID 与实际状态 | 发布负责人 | 待前置阶段 |

任务中的角色尚未指派个人。R01 的名称或权利路线由产品及权利持有人决定；本轮不会替用户宣称拥有 Muse 权利。

## 提交基线与检查结果

[分类提交清单](evidence/workspace-commits.json) 记录了本次 19 个提交。候选文本凭据模式检查没有匹配，但这不是完整安全审计。

已通过的定向检查：移动端 32 项、AI 发送授权 3 项、资源关闭 19 项、Mobile 排除边界 6 项、流水线关口 12 项、配置签名与文档地址契约 2 项。配置契约已补上 documentBaseUrl，与客户端校验一致。完整应用测试、真机行为、版权来源审查与签名包仍是后续关口；这些定向测试不代替 R08 至 R10。

本地另有 1145 个未跟踪档案文件未纳入这 19 个提交，包括原始日志、旧原生库、历史构建与商店材料、自动生成图和旧插件归档。文件保留原处；[本地档案清单](evidence/local-artifacts.json) 只登记路径与大小，不将它们视为已审查发布输入。相邻插件仓库也未冻结，因此 sourceFrozen 保持 false。

## 本轮阶段安排

1. **范围与整改设计**：完成 R01、R03，锁定名称路线、真实数据流、版本和渠道。01-scope 保持 partial。
2. **实现与隐私闭环**：完成 R02、R04 至 R07，关闭 M05 至 M08 等 P0。02-development 与 03-privacy 正在进行。
3. **发布验证**：完成 R08 至 R11，冻结源码并生成正式包；通过 04 至 07 阶段。
4. **重新提审**：执行 R12。08-upload 与 09-submit 当前 pending，不以计划代替实际平台结果。

按实际完成情况更新 [结构化任务](tasks.json) 与 `stages/*.json`。执行前检查：

```sh
python3 scripts/release/pipeline.py plan --run docs/release/runs/2026-11-mobile-isolation
python3 scripts/release/pipeline.py check --run docs/release/runs/2026-11-mobile-isolation
python3 scripts/release/pipeline.py check --run docs/release/runs/2026-11-mobile-isolation --gate 09-submit
```

当前提审关口应返回阻断；只有真实的阶段通过证据、冻结源码及正式制品齐备后才放行。
