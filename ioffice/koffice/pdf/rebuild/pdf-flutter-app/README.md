# PdfRebuild Flutter

产品级跨端架构、实施阶段与测试矩阵见 [多端产品方案](../docs/product-integration/PLAN.md)。
模块边界、统一构建命令与当前能力矩阵见 [工程化说明](../docs/product-integration/ENGINEERING.md)。
Rust 作为新增产品逻辑语言的技术验证、成本与决策门槛见 [Rust 使用边界](../docs/product-integration/RUST-DECISION.md)。

Flutter 跨平台 PDF 外壳，界面和交互入口参照仓库内 Android Demo。工程同时生成 Android、iOS、Linux、macOS、Windows、Web runner。

当前真实能力：打开 PDF、渲染、连续滚动、分页、触控板/双指缩放、文本选择、长按功能菜单和另存副本。macOS 与 Android arm64 已通过 FFI 接入 rebuild 编辑内核并完成真实应用回归。Web 使用同一固定 PDFium 源码编译的产品 WASM；页面操作、渲染、文字、编辑、撤销/重做及导出通过 Node 桥接回归，浏览器应用完成打开、删除页及下载另存实测。浏览器应用还完成了文字点击输入、撤销/重做及编辑后下载；独立 PDFKit 重开确认修改。页面组合目前会拒绝已知不能保留的签名、表单、书签、标签结构、附件及所选页面批注。Windows/Linux 的产品内核尚未接入。

编辑层使用 `ksp_product_edit_*` 版本化 C ABI，macOS、Android 通过 FFI 调用，Web 通过 WASM 桥接调用。预览、抽页、删除、合并、另存使用同一产品会话的快照；磁盘写入由各平台文件适配器完成。三个已接入平台默认让阅读视图消费产品会话的渲染、文字及链接；原阅读器 PDFium 库仅在原生包保留作为调试回退。按当前要求，iOS 暂不测试验收；历史模拟器构建和测试记录见工程文档。`bash tool/verify.sh {shared|macos|android|web}` 运行当前各端门禁。

Web 固定使用产品阅读/编辑会话；本地 `pdfrx` 2.4.8 补丁跳过阅读视图的冗余 PDFium 初始化，Web 发布包已移除自带的第二份 PDFium WASM。浏览器文字输入框使用极低透明度保留 Flutter Web 的 DOM 输入宿主，完全透明会丢失键盘输入。

ABI v6 已提供当前编辑会话的任意页尺寸、整页 RGBA、可见区域分块渲染、每个 UTF-16 字符的页面坐标，以及 URI/文档内跳页链接；活动页可读取尚未导出的文字修改。`ProductPdfDocument` 将这些能力接到 `pdfrx`。macOS 与 Android 的真实 UI 集成用例已验证打开、编辑、保存、抽页；书签、旋转、渲染中的即时取消及复杂 PDF 语料仍需补齐。排查问题时可用 `--dart-define=KSP_PRODUCT_READER=false` 切回旧阅读路径。

Rust 试点位于 `pdf-rebuild/product-rust`，macOS 和 Android arm64 真机已用同一测试验证现有 C ABI 的打开、导出、重开和内存释放；`bash tool/verify.sh rust` 与连接设备后的 `bash tool/verify.sh rust-android` 可重复验证。Flutter 产品目前仍直接调用 C ABI，Rust 还不是发布物的一部分。

```bash
./tool/prepare_macos_native.sh
/Users/mac/src/flutter/bin/flutter pub get
/Users/mac/src/flutter/bin/flutter analyze
/Users/mac/src/flutter/bin/flutter test
/Users/mac/src/flutter/bin/flutter run -d macos
```

Android arm64 构建前运行 `bash tool/prepare_android_native.sh`，再运行 `flutter build apk --release --target-platform android-arm64`。若当前 shell 的 `GRADLE_USER_HOME` 被设置为字面值 `~/.gradle`，构建时显式传入 `GRADLE_USER_HOME=/Users/mac/.gradle`。准备脚本将同版本的 rebuild 内核与 PDFium 放入 APK 的 `jniLibs/arm64-v8a`；生成目录不提交到 Git。Android 真机已走通打开、文字编辑、抽页、合并和系统文档选择器另存，拉回 PDF 分别确认单页及三页输出。macOS Release 已走通打开、抽页、合并、另存的端到端流程。

Web 发布构建运行 `bash tool/verify.sh web`，它对固定产品 WASM 执行编辑和页面回归，再生成 JavaScript 与 Flutter WASM 应用。需要从固定 PDFium 源码重新生成 `web/ksp_product.mjs` 和 `web/ksp_product.wasm` 时，先准备 [产物说明](../pdf-rebuild/3rdparty/prebuilt/PRODUCT_BINARIES.md) 中锁定的 GN/Emsdk，再运行 `bash ../pdf-rebuild/scripts/build_product_web.sh`。普通 Flutter 构建使用仓库内已固定的 Web 产物。

`third_party/pdfium_dart` 保留上游 0.2.5 的 Dart 绑定与 MIT 许可，并调整构建钩子：Android arm64 与 macOS 的兼容阅读路径会加载本工程同源码编译的独立 PDFium 实例，默认产品阅读路径只调用产品会话。APK 暂含两份同源 PDFium；macOS/iOS 应用包还含 `pdfium_flutter` 的冗余 PDFium.framework。清理重复库须先覆盖书签、旋转、字体回调、失败回退及复杂文档语料。其他平台仍按上游钩子处理，待各端固定内核产物准备好后迁移。

macOS 手工测试：点击“打开”选择 PDF；进入编辑模式后直接点击蓝框内的文字，确认蓝色 caret 出现在点击字形边界并唤起输入焦点；直接输入或退格，检查预览、撤销/重做；点击另存并用 Preview 重开。完整步骤见 `docs/so-rebuild/phases/F1-macos-native-edit-adapter/MANUAL_TEST.md`。
