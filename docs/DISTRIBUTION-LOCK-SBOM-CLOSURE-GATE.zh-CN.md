# OpenMuse Distribution Lock、SBOM 与闭包门禁

- 状态：Accepted（C4）
- Owner：OpenMuse Plugin Platform / Release Engineering
- 实现：`scripts/distribution_lock.py`
- Target snapshots：`distribution/targets/`

## 1. 目标与非目标

C4 把“某个目标最终允许携带哪些 Plugin artifact”变成可复现、可审计、默认拒绝的构建期事实。解析器只接受 Manifest v2 和显式 Host capability snapshot，产出同一事实的三种投影：

- `distribution-lock.json`：机器可验证的闭包与信任锚；
- SPDX 2.3 JSON：每个 artifact 的版本、license 和 SHA-256；
- `NOTICES.md`：面向发行审核的可读清单。

它不替代平台签名、公证、APK/IPA 签名、容器镜像签名或 C1 的 Ed25519 worker 验签，也不把测试 fixture 伪装成可发布 artifact。基础 App 自身及 Flutter/系统依赖仍由平台构建 SBOM 管理；本合同专门约束 Plugin artifact 闭包。

## 2. 输入、输出与确定性

解析输入为：

```text
Manifest v2 集合 + Host capability snapshot
                   │
                   ▼
       target/allowlist/kind fail-closed resolver
                   │
          ┌────────┼─────────┐
          ▼        ▼         ▼
 distribution-lock  SPDX     notices
          │
          ▼
 actual bytes + exact artifact catalog + app/image root scanner
```

Capability snapshot 必须显式给出 `profile`、精确 `{os, arch, libc}`、`allowedArtifactKinds`、`allowedPlugins` 和 `requiredPlugins`。空 allowlist 表示不允许任何 Plugin，不表示“无限制”。snapshot 的 canonical JSON SHA-256 写入 lock；验证阶段必须重新提供 snapshot，因此不能拿 macOS lock 验证 Android 包。

解析采用按 key 排序、无无关空白、UTF-8 并以换行结尾的 canonical JSON。`closureDigest` 是移除该字段后整个 lock 的 SHA-256；输入和 `SOURCE_DATE_EPOCH` 相同，lock、SBOM、notices 必须逐字节一致。

Lock 固定：

- profile 和精确 target；
- capability snapshot digest；
- Plugin id/version；
- artifact id/kind/ABI/target；
- SHA-256、SPDX license 与可选 signature 元数据。

## 3. Fail-closed 解析与闭包验证

解析阶段拒绝以下输入：

- 非 Manifest v2、重复 Plugin、重复 artifact 或不合法 SHA-256；
- target 决策缺失、重复、unsupported，或 artifact 指向 unsupported target；
- allowlist 外 Plugin（直接裁剪）、required Plugin 缺失；
- target 已支持但无可选 artifact；
- artifact kind 超出目标 capability；
- 未携带完整 signature 元数据的 `sandbox-worker`。

发布 staging 完成后，packager 必须提供 `artifact id -> 闭包根目录相对路径` 的精确 catalog，再执行 `verify`。门禁要求：

1. catalog 的 id 集合与 lock 完全相等，不能少、不能多；
2. 路径不能逃逸根目录，artifact 不能是 symlink；
3. 每个实际文件重新计算 SHA-256，必须匹配 lock；
4. lock 自身摘要与 capability snapshot 必须匹配；
5. Mobile 根目录不得出现 Node、`node_modules`、Helix/hx、Desktop DSH closure、`.exe` 或 `.dylib`；
6. Sandbox lock 只能包含 `sandbox-worker`。

扫描整个 Mobile app root 是 defense-in-depth：即使某个 Desktop runtime 没进入 artifact catalog，只要被其他打包步骤意外复制，也会使发布失败。画中画、Remote DSH UI 或纯 Dart client 不受影响；禁止的是 Desktop 可执行运行时闭包。

## 4. 发布流水线合同

开发/CI 可以用当前四个 Manifest fixture 验证 resolver 行为和 Desktop 闭包不回退，但 fixture 中的 digest/signature 只是协议测试值。生产发行必须遵循：

1. 先构建真实 artifact；
2. 由受信任 release job 计算 digest、完成 worker 签名并生成发行 Manifest v2；
3. 用目标 capability snapshot 解析 lock/SBOM/notices；
4. 将 artifact stage 到最终 App/Image root，生成精确 catalog；
5. 在签名和归档前执行 `verify`；
6. 将 lock、SBOM、notices 与最终制品共同归档。

调用示例：

```bash
python3 scripts/distribution_lock.py resolve \
  --capability distribution/targets/android-aarch64.json \
  --manifest path/to/plugin-a.manifest.json \
  --lock out/distribution-lock.json \
  --sbom out/openmuse.spdx.json \
  --notices out/NOTICES.md \
  --source-date-epoch "$SOURCE_DATE_EPOCH"

python3 scripts/distribution_lock.py verify \
  --capability distribution/targets/android-aarch64.json \
  --lock out/distribution-lock.json \
  --catalog out/artifact-catalog.json \
  --root path/to/final-app-root
```

Packager 不得在 resolver 之后隐式加入 Plugin artifact。平台原生依赖若不属于 Plugin catalog，仍必须由平台自己的 allowlist/SBOM gate 管理。

## 5. 平台策略

| Profile | 策略 |
| --- | --- |
| Desktop | 保留当前 Helix、Local DSH、Native Text Gate、Open File Viewer 闭包；后续真实 release manifest 替换 fixture digest |
| Android/iOS | 只选择明确声明 Mobile supported 的 artifact；当前四个 Desktop Plugin 均被排除 |
| Sandbox | 只允许显式 allowlist 中、已签名且 kind 为 `sandbox-worker` 的 artifact |

Target snapshot 是受评审的发行配置，不由 Plugin 自行修改。新增平台、架构或 artifact kind 必须通过 Manifest v2/C4 兼容性评审。

## 6. 验收与已知边界

统一验收入口：

```bash
scripts/test_distribution_lock.sh
```

覆盖项包括：

- Desktop 四个现有 Plugin 的解析结果与可复现 lock/SBOM/notices；
- Android 当前为空 Plugin 闭包，且 Mobile root 中 Node/Desktop DSH 被拒绝；
- 错误 target、manifest digest、实际 bytes digest、catalog 漂移和 lock 篡改 fail closed；
- capability target 不匹配被拒绝；
- Sandbox 非 worker artifact 和未签名 worker 被拒绝。

C4 不对任意文件内容做恶意代码分析，也不验证 Ed25519 密码学签名；实际验签由 C1 `ArtifactSignatureVerifier` 执行。C4 保证经过该信任链选择的身份、摘要、license 与最终闭包保持一致。
