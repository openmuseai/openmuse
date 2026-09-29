# Mobile Office View Engines

> 状态：XLSX/PPTX/PDF Engineering Accepted（Engine + Android/iOS App binding）
>
> 子需求：M6 XLSX/PPTX view-only engine、PDF text-view-only engine
>
> 日期：2026-09-29

## 1. 能力边界

`openmuse-office-viewers` 是无网络、无文件系统、无 Workspace/S3/账号 capability 的
Rust bytes engine。XLSX/PPTX/PDF 当前只声明 `view`，不声明 `edit/export`，也不会把“能解析
容器或文本”包装成可靠的原格式 round-trip。

输入门禁包括 64 MiB archive、128 MiB 总解压、32 MiB 单 entry、4096 entries、200 倍
压缩比、100 万 cells 和 1 MiB 单 cell。重复/绝对/反斜杠/`..` path、DOCTYPE、外部
relationship、坏 shared-string index 与倒退 cell reference 全部 fail closed。

PDF 单独执行 64 MiB 文件、20 万 objects、1 万 pages、16 MiB 提取文本门禁；加密文档以及
`OpenAction`、JavaScript、Launch、URI、嵌入文件、XFA/AcroForm、RichMedia 等主动内容全部
fail closed。解析器不访问网络或文件系统。

## 2. 输出与显示模型

engine 解析 shared string、inline string、数字/布尔/公式缓存值，并把 sparse worksheet row
映射为稳定的 tab-separated view paragraphs。输出 schema 是
`openmuse.office.xlsx-inspection@1`，profile 固定为 `view-only`。

这不是 Excel layout renderer；合并单元格、图表、条件格式、宏、公式重算与打印分页不会
被错误声明为已支持。后续 App adapter 必须明确显示 view-only 和兼容性边界。

PPTX 按 `presentation.xml` 的 slide order 和 relationship 精确定位实际 slide part，提取
DrawingML paragraph/text run 为稳定的 `Slide N` 文本视图。它不会声称支持母版视觉还原、
动画、视频、图表渲染、字体替换或可编辑 round-trip；所有 `.rels` 仍执行外链拒绝。

PDF 输出 schema 为 `openmuse.office.pdf-inspection@1`，profile 固定为
`text-view-only`，按页提供可提取文本。这不是 PDF layout renderer：不承诺字体、坐标、图片、
表单、批注、分页视觉还原或扫描件 OCR。当前仍是进程内解析；大规模敌意 PDF fuzz corpus、
独立 worker 崩溃隔离和真实文件视觉对照是发行门禁，而不是本 Engine 子需求的伪完成项。

## 3. 验收

```bash
./scripts/test_office_viewers_engine.sh
./scripts/build_office_viewers_mobile_artifacts.sh
```

前者运行 Rust parser/安全 corpus/C ABI 与 Dart FFI fail-closed TCK；后者生成 Android
arm64 `.so`、iOS device/simulator XCFramework 和 SHA-256 manifest。

## 4. Mobile App binding

App 使用 `MultiFormatOfficeEngine` 精确路由 DOCX/XLSX/PPTX/PDF。三个只读格式通过与 DOCX 相同的
ResourceHandle audience/generation/expiry、bounded range 和 media admission 打开，但
`OfficeViewerScreen` 不取得 `OfficeResourceCommitPort`，界面固定显示“只读兼容视图”且
没有保存入口。PDF 还会显式标为“文本兼容只读视图”；Cloud catalog item 只有在对应的
packaged engine capability 存在时可点击。

Android 真机已实际加载 `libopenmuse_office_viewers.so` 并跨 Dart FFI 调用 XLSX/PPTX/PDF
inspect fail-closed；iOS unsigned arm64 build 已链接 XCFramework，并核对 ABI、三个 inspect
和 free 的最终 App symbols。签名 iOS 真机、敌意 PDF fuzz/worker 隔离与更大真实文件视觉
corpus 仍是发布门禁。
