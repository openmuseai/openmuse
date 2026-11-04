# pdf-rebuild — PDFium 底座上的 WPS PDF 编辑层重建工程

目标：以**开源 PDFium** 为底座，功能等价地重建 WPS `pdf-core`（libkwopdf.so）中
**自研的编辑功能**（批注/表单/文本编辑等），通过 Java JNI 契约驱动并与原始
`.so` 行为对拍。

## 架构

```
┌─────────────────────────────────────────────────────────┐
│ pdf-sdk (cn.wps.moffice.pdf.core)  ← 仓库内已有的 Java 层 │
│   即"可执行规范"：977 个 JNI 方法签名 + Java 语义          │
├─────────────────────────────────────────────────────────┤
│ 本仓库重建层 (ksp::)                                     │
│   JNI 契约 / HandleRegistry / Document→Page / FreeText       │
│   功能等价重写（非逐字节）                                 │
├─────────────────────────────────────────────────────────┤
│ PDFium（开源，BSD-3）                                    │
│   3rdparty/pdfium 源码构建（host macOS / Android NDK）    │
└─────────────────────────────────────────────────────────┘
```

## 证据与方法

- 原始对象：`pdf-core/jni/arm64-v8a/libkwopdf.so`（无混淆，RTTI 完整，960 个 JNI 导出）
- 还原工具链：Ghidra 10.4 headless（`/tmp/ghidra_10.4_PUBLIC`）+ objdump 汇编核实
- 已完成：KPath 类完整闭环（127 项测试通过，见 `../kpath_rebuild/`）

## 目录

| 路径 | 内容 |
|---|---|
| `3rdparty/pdfium/` | PDFium 源码（gclient 管理依赖） |
| `src/ksp/runtime/` | 类型化句柄、错误规范和 Document/Page 生命周期 |
| `src/ksp/jni/` | WPS JNI 契约与 Java 字段写回 |
| `src/ksp/freetext/` | FreeText 批注重建实现 |
| `harness/java/` | Java JNI 测试驱动 |
| `scripts/` | 构建脚本 |
| `docs/` | 规格文档（反编译证据、行为对照） |

## 状态

- [x] 反编译管线就绪（Ghidra + 汇编核实）
- [x] PDFium 底座：预编译 chromium/8021（源码构建需 googlesource，见 docs/03）
- [x] FreeText 行为规格（docs/02-freetext-spec.md）
- [x] FreeText 重建 + JNI 胶水（build/libkspfreetext.dylib）
- [x] Java harness + 行为对拍：`PASS=19 FAIL=0`（含 PDF 字典字节级断言）
- [x] HandleRegistry：类型/所属文档/代际校验，ASan/UBSan `PASS=25 FAIL=0`
- [x] Document→Page：打开、密码重开、页数、取页、页尺寸和级联关闭，host `PASS=43 FAIL=0`
- [x] **Android 真机差分**（docs/04）：原 libkwopdf.so 与重建层在同一真机同一操作序列下，
      颜色/字号 round-trip 逐值一致（ffff0000 / ff00ff00 / 24.0）
- [ ] PDFium 源码构建需 googlesource（已改用官方预编译 chromium/8021，路线见 docs/03）

## 验证

```bash
./scripts/run_tests.sh
./scripts/build_freetext_android.sh
./scripts/build_apk.sh
./scripts/run_diff.sh
```

下一个用户可见闭环是选区；实施顺序为 TextPage、坐标转换、点选/扩选、高亮与复制。
