# native/ — 预编译产物随包分发（flutter asset）

每个平台的 release 产物以**版本化二进制**提交进本仓库，随 `word_render` 包一起被
消费者拉取（git 依赖 clone 整个仓库；flutter asset 声明的文件被打进应用产物，
运行时由 `ffi_loader_io.resolveFfiLibrary` 提取到本地缓存目录再 dlopen）。

- `macos/arm64/libword_core_ffi.dylib` — ✅ 已入库（ad-hoc 签名；macOS 上提取后还会再签一次）
- `linux/` — 在 Linux 主机跑 `tool/build_native.sh` 后入库
- `windows/` — 在 Windows 主机跑 `tool/build_native.sh` 后入库
- `android/` — `tool/build_native.sh android`（需 cargo-ndk）；消费者见 README「Android」
- `ios/` — 静态链接（XCFramework），暂未入库；见 README「iOS」

平台产物新增入库后，务必在 `word_render/pubspec.yaml` 的 `flutter.assets` 追加对应条目
（缺失文件会让 flutter build 失败），并在 `lib/src/ffi_loader_io.dart` 确认提取逻辑覆盖该平台。

产物来源：`samples/docx-mvp-render/native`（crate `word_core_ffi`，FRB 2.12.0）。
重新生成：`tool/build_native.sh [host|android|ios]`。