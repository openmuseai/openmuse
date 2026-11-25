# OpenMuse HarmonyOS 原生工程

这是与 `../openmuse_mobile` 并列的 HarmonyOS Stage / ArkTS 工程。华为开发者中心已登记 APP ID `6917620162625592085`，包名 `com.openmuseai.hm`，所属项目 `OpenMuse`，设备类型为手机、平板、PC/2in1、智慧屏、智能手表。

## 当前范围

本工程提供可供 DevEco Studio 导入的应用结构、统一包名、品牌图标和原生入口页。它是鸿蒙移植的起点，尚未移植 Flutter 客户端的登录、工作空间、编辑、录音和设备功能；也尚未在各设备上构建、运行和测试。华为后台勾选全部设备类型只登记目标设备，不代表现有界面已适配或满足各类设备的上架要求。

## 导入与签名

1. 从华为官方渠道安装 DevEco Studio 和与项目兼容的 HarmonyOS SDK。
2. 打开本目录作为工程，同步 OHPM/Hvigor。SDK 版本以实际 IDE 安装版本为准，必要时调整 `build-profile.json5` 的 `compatibleSdkVersion`。
3. 在 DevEco Studio 的项目签名设置中配置发布证书、私钥及发布 Profile。正式密钥、密码和签名配置仅保存在公司受控位置，不提交仓库。
4. 用正式签名的发布 HAP/APP 验证包名和证书，再用 `../../scripts/filing/app-filing-identity.sh harmony --cert /正式发布证书.cer` 导出备案公钥与 MD5。

## 下一步实现

- 将 `../openmuse_mobile` 的账号、工作空间、文档及设备功能按鸿蒙原生能力移植。
- 分别实现与测试手机、平板、PC、智慧屏和手表界面；手表与智慧屏可能需要独立页面与交互方案。
- 完成权限、隐私、账号删除、商店素材和正式签名的发布验证。

备案字段与签名状态见 `../openmuse_mobile/release_materials/01-备案信息表.zh-CN.md`。

## 首次协议界面（2026-11-03）

入口已接入 `LegalWelcome.ets`：品牌与协议面板、宽屏双栏、系统深浅色和英文资源。同意默认关闭，写入并刷新本地 Preferences 成功后才展示内部移植入口。当前仍没有业务服务，后续服务只能在已同意分支构造。鸿蒙暂仅有中国大陆发行配置。

本机没有 DevEco Studio/SDK，未执行 ArkTS 编译或设备验证，不能将该页面视为已通过鸿蒙上架审核。需要补齐 SDK 后验证手机、平板、PC 上的布局、大字、焦点、协议打开和存储失败。智慧屏和手表仍需单独适配。
