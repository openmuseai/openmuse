# Mobile Office DOCX Rust Engine

> 状态：DOCX Engineering Accepted；App bundling/signing/UI corpus gate 待完成
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

## 6. 剩余门禁

- 将 `.so`/XCFramework 纳入 C4 native artifact kind、签名、SBOM 和 notices；
- Flutter FFI adapter 的平台加载、生命周期、isolate/crash/低内存恢复；
- 在 App 中用真实 ResourceHandle、expectedRevision/receipt 走完打开—编辑—保存；
- 来自多 Office 版本/字体/语言/损坏样本的扩展 corpus 与视觉分页基线；
- x86_64 Android（若产品支持）、iOS 签名 archive 和真机性能数据。

在这些完成前，DOCX 不进入生产菜单；其他格式更不能继承 DOCX 的验收结论。
