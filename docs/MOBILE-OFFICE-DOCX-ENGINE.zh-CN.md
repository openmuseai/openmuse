# Mobile Office DOCX Rust Engine

> 状态：DOCX Engineering Accepted；Android/iOS App bundling 与 Android 真机 FFI 已通过，签名/UI corpus gate 待完成
>
> 子需求：M6 first-format increment
>
> 日期：2026-09-29

## 1. 逐格式结论

M6 不再只有抽象 capability 表。`openmuse-office-docx` 提供首个真实 Rust Office
Engine slice；Word/DOCX 的状态提升为 Engineering Accepted。Sheet、Slides 和 PDF
仍是未实现，不进入 Manifest、菜单或 distribution lock。

DOCX engine 不持有 Workspace、S3、DSH、账号、网络或文件系统 capability。调用方从
M3 `ResourceHandle` 取得有界 bytes，再通过 C ABI 传入 Engine；导出 bytes 由 Resource
Authority 使用 `expectedRevision` 提交并返回真实 receipt。Flutter 不再伪造
`revision:<length>`。

## 2. Engine profile 与 capability

- `simple-text`：只包含 document/body/paragraph/run/text/tab/break/sectPr 的保守子集，
  注册 `view/edit/export`；导出为新的最小 DOCX，重新解析后的段落文本必须完全相同。
- `view-only`：table、drawing、hyperlink、样式属性及任何未明确允许的 OOXML element
  都会降级，只注册 `view`；不允许原格式导出。
- 复杂文档不会因“能够提取部分文字”就宣称可编辑。字体、分页、公式、批注、嵌入对象
  和 tracked changes 仍属于后续独立 corpus gate。

Flutter 的 `OfficeFormatRegistry` 取 artifact 静态 capability 与文档动态 capability 的
交集；artifact 必须精确匹配 platform、ABI，且 digest 是 64 位小写 SHA-256。重复或
模糊 artifact fail closed。

## 3. 输入安全

Rust Core 对 archive 大小、entry 数、单 entry、总解压量和压缩比设上限；拒绝绝对
路径、`..`、反斜杠、重复 entry、外部 relationship、DTD/自定义 entity 和损坏 ZIP/XML。
默认不联网，也没有解析远端图片/模板的代码路径。C ABI 用 `catch_unwind` 隔离 panic，
所有返回 buffer 必须显式 free。

## 4. ABI 与平台制品

ABI header：`crates/openmuse-office-docx/include/openmuse_docx.h`，当前版本 1，导出：

- `openmuse_docx_abi_version`；
- `openmuse_docx_inspect`；
- `openmuse_docx_export_simple`；
- `openmuse_docx_buffer_free`。

可复现构建入口：

```bash
./scripts/test_office_docx_engine.sh
./scripts/build_office_docx_mobile_artifacts.sh
```

第二个脚本实际生成 Android arm64 `libopenmuse_office_docx.so` 和 iOS device + arm64
simulator `OpenMuseDocx.xcframework`，并输出相对路径 SHA-256 manifest。制品位于
`target/office-docx`，不会把本机构建产物提交进源码仓库。

## 5. 当前验收证据

6 项 Rust TCK 覆盖：Unicode/tab/break 文本、simple profile DOCX round-trip、复杂文档
view-only gate、损坏/外链/DTD 拒绝、traversal/非可移植 entry path、archive limit 与真实 C ABI ownership/free。
Android ELF 已验证为 AArch64 且包含全部四个 ABI symbol；XCFramework 同时包含
`ios-arm64` 和 `ios-arm64-simulator` static library/header slice。

`openmuse_office_docx` Dart package 只实现 `OfficeEnginePort` 的 bytes ABI，不取得任何
Workspace、Resource、S3、账号、DSH、文件系统或网络 handle。native buffer 在 Dart 复制
完成后必定调用 Rust free，输入 malloc 也在 `finally` 释放；未知 schema、profile、capability
或 ABI 均 fail closed。

App packaging gate 已进一步通过：

- Android Gradle 从 `target/office-docx/android` 打包 arm64 `.so`，release APK 中四个符号可见；
- `scripts/test_mobile_docx_android.sh` 在 PKM110 真机上实际加载 `.so`、读取 ABI 并跨 FFI
  触发受控错误，测试通过且进程未崩溃；
- iOS Runner 链接 XCFramework、启动时校验 ABI，unsigned arm64 bundle 中可见 inspect/ABI
  符号；
- production composition 只有在 native engine 成功加载时才声明内部
  `office.docx.engine` capability，打包/ABI 错误不会影响 Host 其余功能。

`OfficeResourceTransaction` 已补齐应用层事务：校验 audience/generation/expiry/media type
和 64 MiB 上限，按 512 KiB range 读取，Engine 只收到复制后的 bytes；保存时只有
simple-text 的 edit/export capability 可进入导出，并把 bytes、`expectedRevision`、
idempotency key 和 generation 交给独立 `OfficeResourceCommitPort`。跨资源、短 range、
过期/超限 handle、stale revision、view-only export 和伪造/晚到 receipt 全部失败。

Cloud adapter 以 `application/octet-stream` 向 `/v1/resources/commit` 流式提交 bytes，
CAS 元数据位于专用 header，Bearer token 仍只属于 adapter。loopback HTTP TCK 已验证
请求体和 receipt；这不把 Cloud/S3 凭据交给 DOCX engine。

`DocxEditorScreen` 是独立 Flutter presentation adapter，只接收 ResourceHandle、Engine、
range port 和 commit port。simple-text 文档提供逐段编辑与 receipt-backed 保存；view-only
文档的输入框只读且根本不渲染保存按钮；打开失败和提交冲突不会伪造成功 revision。
Widget TCK 已覆盖可编辑保存与复杂文档降级。该组件不自行列举 Workspace 或构造 handle，
后续只能由通过授权的 Resource catalog 路由进入。

## 6. 剩余门禁

- 将 `.so`/XCFramework 纳入正式 release 签名、SBOM 和 notices；
- FFI isolate/crash/低内存恢复和更大规模 fuzz/corpus；
- 将已通过 TCK 的 DOCX editor 接入真实账号下的 Resource catalog 路由与授权 handle；
- 来自多 Office 版本/字体/语言/损坏样本的扩展 corpus 与视觉分页基线；
- x86_64 Android（若产品支持）、iOS 签名 archive 和真机性能数据。

在这些完成前，DOCX engine 虽随内部候选包分发，但不进入生产菜单；其他格式更不能继承 DOCX 的验收结论。
