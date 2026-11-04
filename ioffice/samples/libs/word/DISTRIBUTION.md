# word_render / word_editor — 供其他工程使用（分发说明）

> 目标：**开箱引入即用**——消费者工程只需声明依赖，无需内置 Rust 工具链、
> 无需手动拷贝 dylib。native 产物（`word_core_ffi`）预编译后作为 flutter asset
> 随包分发，运行时自动提取到本地缓存并 dlopen。

## 0. 现状与本次改造

| 项 | 之前 | 现在 |
|---|---|---|
| 发布形态 | `publish_to: "none"`，仅样品内 path 依赖 | 可发布包：git 依赖可直接引入；`publish_to` 已注释（发布时删除即可） |
| native 库 | 留在 `samples/docx-mvp-render/native`，靠硬编码相对路径查找 | 预编译进 `word_render/native/<platform>/`，asset 提取 + 自动加载 |
| word_editor → word_render | `path: ../word_render` | 版本约束 `^0.1.0` + 本地开发 `pubspec_overrides.yaml` |
| 仓库 | 无独立 git 仓库 | `samples/libs/word` 是独立 git 仓库（本地 git 依赖起步） |

## 1. 消费者如何引入（三种方式）

### 方式 A：本地 git 依赖（当前起步推荐）

只消费 `word_render` 的消费者：

```yaml
dependencies:
  word_render:
    git:
      url: /Users/mac/src/ioffice/samples/libs/word
      path: word_render
      ref: main            # 或具体 tag/commit
```

同时消费 `word_render` + `word_editor` 的消费者（word_editor 内部以版本约束
`^0.1.0` 引用 word_render，纯 git 场景需用 `dependency_overrides` 指回同一 git 源）：

```yaml
dependencies:
  word_editor:
    git:
      url: /Users/mac/src/ioffice/samples/libs/word
      path: word_editor
      ref: main
dependency_overrides:
  word_render:
    git:
      url: /Users/mac/src/ioffice/samples/libs/word
      path: word_render
      ref: main
```

- `url` 也可用 `file://` 或任意 git 远程（`https://`/`git@`），发布到内网 git 后即可全网使用。
- 同仓库两个包用 `path:` 区分。
- **一旦把 word_render 发布到私有 pub server（方式 B），word_editor 的 `^0.1.0`
  会自动从 hosted 解析，上述 `dependency_overrides` 即可删除**——纯 git 阶段才需要。

### 方式 B：私有 pub server（内网正式发布）

1. 删除两个包 pubspec 里的 `publish_to` 注释行（见文件内说明）；
2. 部署私有 pub server（Nexus/Artifactory 的 Dart pub 仓库，或自建）；
3. `flutter pub publish --server https://pub.example.com`（配置凭据）；
4. 消费者改为普通版本依赖：`word_render: ^0.1.0`。

### 方式 C：pub.dev 公开

- 删除 `publish_to` 注释行 → `flutter pub publish`；
- 需先过包名唯一性与描述检查；发布后全世界可用。

> **关于 maven**：`word_render`/`word_editor` 是 **Flutter/Dart 包**，只能被
> Flutter 工程通过 pub 体系消费（相当于 Dart 生态的"maven"）。它们不能装进 maven。
> 将来若有 **Java/Kotlin(Android) 纯 JVM 工程**要消费 word-core 能力，需要另走
> JNI/AAR 路线（把 `word_core_ffi` 编成 jniLibs `.so` + JNI 绑定 → maven 坐标），
> 与本文档的 Dart 包无关，见 `samples/docx-mvp-render/README.md` 后续规划。

## 2. 直接使用（示例）

```dart
import 'package:word_render/word_render.dart';

final engine = WordCoreEngine();
await engine.initFfi();            // 自动定位/提取 native 库（桌面端）
await engine.ensurePaintFont();    // 宿主字体注入，缺失时回退包内 MvpLatin

final envelope = await engine.layoutDocument(
  docx,                            // Uint8List（.docx 字节）
  view: kViewParamsA4,             // 缺省 A4（twip）
);

// envelope.pages → PageMetrics；envelope.commands → WcRenderCommand 列表
// 绘制：DocumentPainter / PagePainter（CustomPainter），命中：hitTestPage/caretFromTap
```

编辑路径：`engine.openSession(docx)` → `sessionCaretAt/sessionSetSelection/insertText...`
（详见 `word_render` README 的 Public surface）。

## 3. 平台支持矩阵（开箱程度）

| 平台 | 产物 | 状态 |
|---|---|---|
| macOS (arm64) | `native/macos/arm64/libword_core_ffi.dylib` | ✅ 已入库，开箱即用 |
| Linux (x86_64) | `native/linux/x86_64/libword_core_ffi.so` | 产物在 Linux 主机构建后入库即开箱 |
| Windows (x86_64) | `native/windows/x86_64/word_core_ffi.dll` | 同上 |
| Android | `native/android/<abi>/libword_core_ffi.so` | 产物入库后仍需消费者一次性配置 jniLibs（见下） |
| iOS | XCFramework（静态链接） | 暂未入库；FRB iOS 集成（后续做成 flutter plugin） |
| Web | — | 无需 native（golden JSON / wasm 延迟路径） |

### Android 一次性配置（消费者）

把 `~/.pub-cache/git/word-*/word_render/native/android/<abi>/*.so` 拷进 App 的
`android/app/src/main/jniLibs/<abi>/`（或 `build.gradle` 里 `jniLibs.srcDirs` 指向）。
把 `libword_core_ffi.so` 做成插件/脚本后即可免手动步骤（后续迭代）。

## 4. native 产物如何构建 / 新增平台

```bash
cd samples/libs/word
tool/build_native.sh        # 当前宿主机（macOS→arm64，已含 ad-hoc 签名）
tool/build_native.sh android
tool/build_native.sh ios    # 骨架
```

流程（每次 ABI 变更）：
1. `cargo build --release -p word_core_ffi`（crate 位于 `samples/docx-mvp-render/native`）；
2. 产物拷入 `word_render/native/<platform>/` 并提交 git；
3. `word_render/pubspec.yaml` 的 `flutter.assets` 追加该文件条目；
4. 若 ABI 兼容性有破坏性变更，`ffi_loader_io.dart` 的 `_kExtractVersion` 加一（避免旧缓存）。

## 5. 发布检查清单（给维护者）

- [ ] `flutter pub get`（两包）通过
- [ ] `dart analyze`（两包）0 issue
- [ ] `cargo build --release -p word_core_ffi` 通过后产物已入库、assets 已登记
- [ ] 一次性在干净目录用「方式 A」写一个最小 consumer 跑 `flutter pub get` + 一页渲染验证
- [ ] 发布到私有 server 时改好 `repository`/`homepage` 字段与 pub 凭据

## 6. 相关文件

- 包：`word_render/`、`word_editor/`
- native 产物：`word_render/native/`、构建脚本 `tool/build_native.sh`
- 运行时 native 定位/提取：`word_render/lib/src/ffi_loader_io.dart`（`resolveFfiLibrary`）
- 字体回退：包内 `assets/MvpLatin.ttf`；宿主字体注入 `lib/src/font_source_io.dart`