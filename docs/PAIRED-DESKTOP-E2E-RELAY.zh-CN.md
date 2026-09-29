# Paired Desktop E2E Channel 与 Opaque Relay

> 状态：Engineering Accepted；Mobile 平台密钥存储已实现，native crypto bridge/真实 Relay 部署门禁待完成
>
> 子需求：M5 integration increment
>
> 日期：2026-09-29

## 1. 领域隔离

`openmuse-paired-relay` 是 Rust 领域核，不依赖 Flutter、Workspace 存储、DSH、S3
或 Relay SDK。平台 adapter 负责从 Android Keystore / iOS Keychain / Desktop
Keystore 提供随机设备 seed；私钥、派生 channel key 与明文 frame 不进入 Dart。

Flutter `PairedDesktopDshConnector` 与 `PairedDesktopResourcePort` 只持有
`PairedGrant`、presence 和 Rust/platform transport port。它们复用 M2 的
`DshRuntimeConnector` 和 M3 的 `ResourceRangePort`，不建立第二套 DSH/Resource
状态机，也不存在“Desktop 离线则改连同名 Cloud Workspace”的分支。

## 2. 配对协议

1. 每台设备由平台 Keystore 持有 32-byte seed，领域核通过域分离派生 Ed25519
   signing key 与 X25519 agreement key。
2. 设备 offer 签名覆盖 account/device refs、两个 public key、随机 nonce 与账号注册
   generation。
3. 自签名不能证明“同账号”。双方还必须取得账号服务的 `DeviceRegistration`，逐字段
   匹配 public key、generation、revocation 和 account/device refs。
4. 双方按 device ref 确定性排序完整签名 offer，计算相同 transcript 和 6 位人工 SAS。
   用户在两端确认相同 SAS 后才继续。
5. X25519 shared secret 经 transcript-salted HKDF 派生两个方向独立的 AEAD key 和
   nonce prefix。Rust drop path 清零 shared/channel key material。

## 3. Relay envelope

业务 frame 使用 ChaCha20-Poly1305。AAD 绑定 `channelRef`、sender、recipient 和严格
递增 sequence；nonce 是方向独立 prefix + sequence。接收端只接受期望的下一个
sequence，因此篡改、重放、乱序、跨 channel 和错收件人全部 fail closed。

单帧密文上限为 64 KiB + tag。大文件必须走 range/stream 并等待 relay queue 消费，
不能全量 collect。参考 `OpaqueRelay` 只保存：

- outbound connection 的 account/device/generation；
- channel/sender/recipient/sequence；
- ciphertext 与密文字节指标。

Relay 不持有 channel key、Workspace grant、resource ref、DSH token 或业务明文。

## 4. Workspace grant

Grant 同时绑定 account、mobile device、desktop device、workspace、permission、TTL 和
generation。`read/propose/apply` 分开授权；revoke、过期、设备 replaced/sleep/offline、
错误 generation 与跨 Workspace 都拒绝。配对成功本身不授予任何 Workspace 权限。

## 5. 验收

统一入口：

```bash
./scripts/test_paired_relay.sh
```

7 项 Rust TCK 覆盖签名/SAS/可信注册、双向 E2E、Relay opaque routing、篡改/重放/
乱序、frame bound、离线和 grant scope；Dart TCK 覆盖 M2/M3 adapter 在发请求前执行
grant，以及没有 Cloud fallback。

## 6. Mobile 平台密钥边界

Flutter 只依赖 `DeviceKeyStorePort`，并且只接收 `keyRef/storage/hardwareBacked/created`
四个描述字段；adapter 对包含 `seed/privateKey/secret` 的平台响应 fail closed。

- Android 使用 Android Keystore 中的 AES-256-GCM wrapping key，加密随机 32-byte seed 后
  存入 app-private `SharedPreferences`。返回值报告 wrapping key 是否由安全硬件承载；
- iOS 使用 non-synchronizable、`AfterFirstUnlockThisDeviceOnly` 的 Keychain generic
  password 保存随机 32-byte seed，不参与 iCloud 同步；
- account/device ref 仅用于导出稳定 SHA-256 `keyRef`，删除操作只接受严格格式的
  `device-key:<64 hex>`；
- 两端 seed 临时 buffer 在原生创建路径完成后清零，MethodChannel 从不返回 seed。

Android 真机门禁：

```bash
OPENMUSE_ANDROID_SERIAL=<serial> ./scripts/test_mobile_device_keystore_android.sh
```

门禁验证相同 identity 的 keyRef 稳定、第二次 ensure 不轮换、删除后重新创建，以及 Dart
可见对象保持 opaque。iOS 当前由 `scripts/test_ios_beta.sh` 做无签名 arm64 编译门禁；
Keychain 真机行为仍需 Apple 签名设备证据。

这个增量只完成 at-rest key storage。iOS Secure Enclave 不原生支持本协议采用的
Ed25519/X25519，因此下一增量必须让 Swift/Kotlin 在原生边界内取 seed 并调用 Rust crypto
bridge；不能为了接入而把 seed 返回 Dart。

## 7. 剩余发布门禁

本增量没有把本机参考队列伪装成生产 Relay。Production Accepted 还需要：

- Desktop Keystore adapter、Mobile/Desktop 备份策略与 key rotation；
- 账号服务签发、防回滚和撤销 `DeviceRegistration`；
- 真实 WSS/QUIC outbound relay 的限流、backpressure、离线队列上限和多地域故障；
- Rust Core 到 platform/Desktop 的最小 native bridge 与内存清零审计；
- 第三方密码学评审、移动端抓包、代理/恶意 relay 和设备被替换演练。

这些需要平台签名环境和部署基础设施；在证据完成前，M5 维持 Engineering Accepted。
