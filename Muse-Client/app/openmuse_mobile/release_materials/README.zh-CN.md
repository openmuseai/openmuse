> 后续版本执行入口：[12 阶段上架流水线](../../../docs/release/pipeline/README.zh-CN.md)；[本次海外首发归档](../../../docs/release/runs/2026-11-overseas/README.zh-CN.md)。本目录保留备案与早期准备资料。

# OpenMuse Mobile 备案与上架材料包

更新：2026-10-31。对应工程：`Muse-Client/app/openmuse_mobile` 和 `Muse-Client/app/openmuse_harmony`。此目录是**填写草稿与证据索引**；带“待核实”的项目不能原样提交。用户已确认 APP 备案完成，但尚未提供 APP 备案号及查询结果；此前“待核发”表述须按实际备案记录更新，不能用网站备案号代填。

## 已由工程确认

| 项目 | 值 | 来源 |
| --- | --- | --- |
| 应用名称 | OpenMuse | Android Manifest、iOS Info.plist |
| 备案拟用主体 | 东莞市成宣电子科技有限公司 | 用户提供的营业执照照片；提交前复核原件 |
| 公开联系邮箱 | `developer@openmuseai.com` | 用户提供；应用“关于”和已上线的账户删除页面均显示 |
| Android 包名 | `com.openmuseai.mobile` | `android/app/build.gradle.kts` |
| iOS Bundle ID | `com.openmuseai.mobile` | `ios/Runner.xcodeproj/project.pbxproj` |
| HarmonyOS 包名 | `com.openmuseai.hm` | `../../openmuse_harmony/AppScope/app.json5`；华为 APP ID `6917620162625592085` |
| Android 发布签名 | 正式 RSA 4096 位证书；MD5 `28b84f070b519d0ca878ba69df666b51` | 已从正式签名 release APK 复核；完整公钥见 `07-Android-正式证书备案值.txt` |
| Apple Team ID | `SDPXS943DW` | Apple Developer 组织 Team |
| Apple 显式 App ID | `com.openmuseai.mobile` | Apple Developer 已注册为 OpenMuse Mobile |
| App Store Connect | OpenMuse，Apple ID `6827192902`，SKU `openmuse-mobile-ios` | 已创建 iOS 应用记录；2026-10-29 美国英文名称已改为 OpenMuse AI，主语言英语（美国） |
| iOS 发布签名 | Apple Distribution，证书 ID `YVB44BW7QS`，到期日 2027-10-26 | 本机登录钥匙串中有匹配私钥；公钥和 SHA1 见 `06-iOS-正式证书备案值.txt` |
| App Store 发布描述文件 | `OpenMuse Mobile App Store 2026`，ID `7UYBDAQT7B` | 绑定上述 App ID 和发布证书，已安装在 Xcode |
| HarmonyOS 发布签名 | `OpenMuse HarmonyOS Release 2026`，至 2029-10-27；MD5 `8e6a7e8b223ac0e9263289e1d0228dd6` | 华为已签发；公钥和 SHA1 见 `08-鸿蒙-正式证书备案值.txt` |
| HarmonyOS 发布 Profile | `OpenMuse HarmonyOS App Store 2026` | 已绑定正式 APP ID `6917620162625592085` 和发布证书 |
| 当前候选版本 | `1.0.0`，构建号 `6` | 沿用现有商店发布记录；本次正式签名候选仍为 internal-test-only |
| 主服务域名 | `openmuseai.com` | `lib/main.dart` 默认云端及认证地址 |
| 已备案网站 | 成宣妙思灵感创作平台，`openmuseai.com`，`粤ICP备2024331713号-4` | 用户提供的网站备案信息；官网与 `www` 页脚已悬挂该网站备案号；APP 备案号尚待提供/核对 |
| 隐私政策 | https://openmuseai.com/zh/privacy；https://openmuseai.com/en/privacy | `lib/main.dart` 按语言选择 |
| 服务条款 | https://openmuseai.com/zh/terms；https://openmuseai.com/en/terms | `lib/main.dart` 按语言选择 |
| 支持网址 | https://openmuseai.com/zh/support；https://openmuseai.com/en/support | 2026-10-28 已部署并返回 HTTP 200 |
| 账户删除说明 | https://openmuseai.com/zh/account-deletion；https://openmuseai.com/en/account-deletion | 2026-10-28 已部署并返回 HTTP 200 |
| 适用平台 | Android、iOS/iPadOS；鸿蒙原生工程尚在移植 | HarmonyOS APP ID 已登记五类设备，但不代表各类设备应用已完成或可上架 |

`com.openmuseai.mobile` 是基于域名的合理命名；**域名对应本身不保证可上架**。创建 Apple App ID、Android 市场记录与正式签名后，须保证各处标识完全一致。已生成的旧包名 APK 和未签名 iOS 构建不能用于新标识的备案或正式上架。

2026-10-30 的全球化整改、真机验收、候选制品与剩余阻断见 [本次移动验收记录](14-全球化移动验收-2026-10-30.zh-CN.md)。

2026-10-31 的 Mobile 界面、Account 衍生页及语音模型管理整改、模拟器验收与 `1.0.0 (6)` 制品见 [界面整改记录](16-Mobile-界面整改与验收-2026-10-31.zh-CN.md)。两端真机界面与模型下载已补验通过，主应用均恢复为构建 6；麦克风转写和实际渠道分发等原有发布关口继续保留。

## 本目录

- [01-备案信息表.zh-CN.md](01-备案信息表.zh-CN.md)：按用户截图的 Android / iOS / 鸿蒙字段整理。
- [02-商店文案.zh-CN-en.md](02-商店文案.zh-CN-en.md)：国内市场、海外 App Store 的中英文文案与审核说明。
- [03-隐私与数据申报.zh-CN.md](03-隐私与数据申报.zh-CN.md)：权限、数据、SDK 与商店隐私问卷工作底稿。
- [04-上架前清单.zh-CN.md](04-上架前清单.zh-CN.md)：需要企业、签名与真实设备配合完成的事项。
- [05-正式签名证书准备.zh-CN.md](05-正式签名证书准备.zh-CN.md)：三平台发布证书与 Profile 的位置、备份要求和验收条件。
- [06-iOS-正式证书备案值.txt](06-iOS-正式证书备案值.txt)：正式 Apple Distribution 证书公钥、SHA1 和证书有效期；仅含公开证书信息。
- [07-Android-正式证书备案值.txt](07-Android-正式证书备案值.txt)：从正式签名 APK 提取的公钥、MD5、SHA1、SHA256。
- [08-鸿蒙-正式证书备案值.txt](08-鸿蒙-正式证书备案值.txt)：华为正式发布证书的公钥、MD5、SHA1、SHA256；华为控制台“备案信息”显示 SHA1，接入平台截图要求 MD5。
- [09-国内海外上架流水线.zh-CN.md](09-国内海外上架流水线.zh-CN.md)：按国内 Android、Google Play、中国区及海外 App Store、鸿蒙分线执行的构建、必检、提审和发布关口。
- [10-单次发布记录模板.zh-CN.md](10-单次发布记录模板.zh-CN.md)：单次发布的版本、制品、签名、备案、商店审核与放量证据记录。
- [11-2026-10-28-内部候选构建记录.zh-CN.md](11-2026-10-28-内部候选构建记录.zh-CN.md)：已生成的国内 APK、Google Play AAB、iOS IPA、签名摘要及仍待完成的上架关口。
- [12-域名与联网审计.zh-CN.md](12-域名与联网审计.zh-CN.md)：从生产源码整理的候选域名、动态配置与正式抓包封版方法。
- [13-Google-Play-上架操作记录-2026-10-28.zh-CN.md](13-Google-Play-上架操作记录-2026-10-28.zh-CN.md)：本次 Google Play Console 实际操作、账号和应用 ID、验证状态、候选 AAB 与人工接续步骤。
- [14-Apple-美区上架操作记录-2026-10-29.zh-CN.md](14-Apple-美区上架操作记录-2026-10-29.zh-CN.md)：OpenMuse AI 美国首发的名称、地区、英文配置、Apple 验证错误修复及人工接续步骤；隐私答复已发布，构建 4 已通过 App Store Connect 模式上传，待 Apple 处理及剩余上架检查；尚未送审。
- [15-Apple-US-store-metadata.en.json](15-Apple-US-store-metadata.en.json)：本次英文商品页和审核备注草稿，可人工复制；不含审核账号密码。
- `scripts/release/build-android-store.sh`（仓库根目录）：在受控本机生成正式签名的国内 APK 或 Google Play AAB **候选包**，核对证书并输出 SHA-256 元数据；商店提审仍须完成 09 的全部关口。
- `scripts/release/build-ios-store.sh`（仓库根目录）：使用备案 Apple Distribution 证书与 App Store Profile 导出并核验 iOS IPA；2026-10-28 已构建通过，钥匙串已允许系统 `codesign` 使用这张私钥。
- `assets/AppleDistribution-2026.cer`：本次发布证书的公开部分；私钥仅在本机登录钥匙串中，不在仓库内。
- `assets/OpenMuse-Android-Release-2026.cer` 与 `assets/OpenMuse-HarmonyOS-Release-2026.cer`：正式发布证书公开部分；私钥和口令仅在本机受控目录及登录钥匙串。
- `assets/app-store-connect-created.png`：App Store Connect 中 OpenMuse iOS 1.0 应用记录的核验截图，当前状态为“准备提交”。
- `assets/huawei-harmony-app-id-created.png` 与 `assets/huawei-harmony-profile-created.png`：鸿蒙 APP ID 和发布 Profile 的控制台核验截图。
- `assets/openmuseai-com-icp-footer-20261027.jpg`：2026-10-27 生产官网中文首页页脚悬挂网站备案号的核验截图。
- [App 图标（PNG，1024×1024）](assets/app-icon-1024.png)：与当前 iOS 工程和鸿蒙工程中的图标一致，RGB、不含透明通道。可作为备案及商店素材的主图；其他尺寸按目标商店要求输出并核验。

![OpenMuse App 图标](assets/app-icon-1024.png)

## 提交顺序

1. 确认营业执照主体、域名备案主体与 Apple 开发者组织一致；核实 App 名称和分类。
2. 三平台正式证书已固定，用 `scripts/filing/app-filing-identity.sh` 复核公钥与指纹。只提交**正式发布证书**产生的值；iOS IPA 已完成归档验签，鸿蒙 HAP 签名验收仍待完成。
3. 根据已完成的 APP 备案记录核对编号、主体、包名和签名；将 APP 备案编号在 App 显著位置展示，并按平台要求填入商店。
4. 修复清单中的隐私与审核阻断项，使用正式签名包和真实设备截屏、检测。
5. 将同一套名称、包名、主体、备案编号和隐私信息填入国内市场及 App Store Connect；中国大陆 App Store 区域另核对 ICP/APP 备案匹配。

## 依据

- [工信部《关于开展移动互联网应用程序备案工作的通知》](https://www.miit.gov.cn/jgsj/xgj/wjfb/art/2023/art_dd783a581c9644a4aee10afa582811db.html)
- [工信部互联网信息服务备案登记表填写说明](https://xzca.miit.gov.cn/bsfw/bszn/art/2024/art_5d3cc7f452c04fdd97b6881a20b656f9.html)
- [Apple App 信息及中国大陆供应要求](https://developer.apple.com/cn/help/app-store-connect/reference/app-information/app-information)
- [Apple App Review Guidelines](https://developer.apple.com/app-store/review/guidelines/)

