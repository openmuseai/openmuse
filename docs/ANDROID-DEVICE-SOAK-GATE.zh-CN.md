# Android 真机安装与 Soak Gate

## 1. Gate 边界

M7 把 Android 验收拆成两个不能互相冒充的层级：

1. **Engineering device gate**：release APK 构建/签名检查、ADB 覆盖安装、生产 composition 启动、50 次前后台、100 次方向变化、进程存活、UI 语义树以及 crash/ANR 检查。
2. **Connected product gate**：真实账号登录、真实 Cloud Workspace/DSH、30 分钟 Agent 任务、100 次 Workspace/Window 切换，以及 phone/tablet/foldable 设备矩阵。

第一层可以在没有账号或生产后端时独立重复执行；第二层必须由外部账号、服务和设备矩阵提供证据。脚本不会注入隐藏账号、fixture workspace 或绕过登录，因此通过第一层不代表完整 Android Alpha 发布门禁通过。

## 2. 可重复执行

先构建内部 Alpha，再在已解锁的 ADB 设备执行：

```bash
./scripts/build_android_alpha.sh
OPENMUSE_ANDROID_SERIAL=<serial> ./scripts/test_android_device_soak.sh
```

可配置项：

- `OPENMUSE_ANDROID_APK`：待安装 APK；
- `OPENMUSE_BACKGROUND_CYCLES`：默认 `50`；
- `OPENMUSE_ROTATION_CYCLES`：默认 `100`；
- `OPENMUSE_SOAK_SECONDS`：前台存活时间，默认 `0`；完整连接态门禁至少设为 `1800`；
- `OPENMUSE_EXPECT_SIGNED_OUT=0`：仅在外部已经提供真实登录态时关闭生产未登录断言。

运行会在 `target/android-soak/<UTC>/` 生成安装与启动输出、APK digest、设备属性、前后截图/UI XML、logcat、crash buffer、exit-info 和 `report.md`。屏幕自动旋转及方向值无论成功失败都会恢复为运行前设置。设备锁定、进程退出、前台 Activity 不符、UI 不是 OpenMuse 或发现本包 crash/ANR 时 fail closed。

## 3. 2026-09-29 真机结果

| 项目 | 结果 |
|---|---|
| 设备 | `PKM110`，Android API 36，`arm64-v8a` |
| APK | `OpenMuse-Android-Alpha-arm64.apk` |
| SHA-256 | `aeb88a68282c817fcacab6147d972d4c2def20fe758ada23cb52137eb2aceef8` |
| 覆盖安装 / 冷启动 | PASS |
| 生产 composition | PASS，显示“未登录 / 请登录以访问 Cloud Workspace” |
| 前后台 | 50/50 PASS |
| 强制方向变化 | 100/100 PASS |
| crash buffer / ANR | PASS，无本包记录 |

该结果只接受 Engineering device gate。由于没有向本次验收提供真实账号和 Cloud DSH，30 分钟 Agent 与 Workspace/Window 切换未执行，Connected product gate 仍为待验收。

## 4. 发布前剩余证据

- 使用同一份候选 artifact 在 phone、tablet、foldable 的支持矩阵重复 gate；
- 真实账号登录后验证 token refresh、断网/恢复与后台重连；
- 对真实 Cloud Workspace/DSH 执行不少于 30 分钟 Agent workload；
- 执行 100 次 Workspace/Window 切换并核对选择、revision 和 draft 不丢失；
- 完成正式 release key/AAB、升级/回滚、SBOM/notices、权限/隐私和 accessibility 门禁。
