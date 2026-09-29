# Mobile Office View Engines

> 状态：XLSX Engineering Accepted（Engine + Android/iOS App binding）
>
> 子需求：M6 XLSX view-only engine
>
> 日期：2026-09-29

## 1. 能力边界

`openmuse-office-viewers` 是无网络、无文件系统、无 Workspace/S3/账号 capability 的
Rust bytes engine。XLSX 当前只声明 `view`，不声明 `edit/export`，也不会把“能读 XML”
包装成可靠的原格式 round-trip。

输入门禁包括 64 MiB archive、128 MiB 总解压、32 MiB 单 entry、4096 entries、200 倍
压缩比、100 万 cells 和 1 MiB 单 cell。重复/绝对/反斜杠/`..` path、DOCTYPE、外部
relationship、坏 shared-string index 与倒退 cell reference 全部 fail closed。

## 2. 输出与显示模型

engine 解析 shared string、inline string、数字/布尔/公式缓存值，并把 sparse worksheet row
映射为稳定的 tab-separated view paragraphs。输出 schema 是
`openmuse.office.xlsx-inspection@1`，profile 固定为 `view-only`。

这不是 Excel layout renderer；合并单元格、图表、条件格式、宏、公式重算与打印分页不会
被错误声明为已支持。后续 App adapter 必须明确显示 view-only 和兼容性边界。

## 3. 验收

```bash
./scripts/test_office_viewers_engine.sh
./scripts/build_office_viewers_mobile_artifacts.sh
```

前者运行 Rust parser/安全 corpus/C ABI 与 Dart FFI fail-closed TCK；后者生成 Android
arm64 `.so`、iOS device/simulator XCFramework 和 SHA-256 manifest。

## 4. Mobile App binding

App 使用 `MultiFormatOfficeEngine` 精确路由 DOCX/XLSX。XLSX 通过与 DOCX 相同的
ResourceHandle audience/generation/expiry、bounded range 和 media admission 打开，但
`OfficeViewerScreen` 不取得 `OfficeResourceCommitPort`，界面固定显示“只读兼容视图”且
没有保存入口。Cloud catalog 的 XLSX item 只有在 packaged engine 存在时可点击。

Android 真机已实际加载 `libopenmuse_office_viewers.so` 并跨 Dart FFI 校验 ABI/fail-closed；
iOS unsigned arm64 build 已链接 XCFramework，并核对 ABI、inspect、free 三个最终 App
symbols。签名 iOS 真机与更大真实文件视觉 corpus 仍是发布门禁。
