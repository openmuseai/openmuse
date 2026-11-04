# 03_function_picture_shape — p:pic leaf commit

> feature: pp-02-shape-tree-parse ｜ 状态：**pass**

## 0. 管线能力解锁

已实现并验收：p:pic 事件提交 type=2 的 Shape row 与 ShowShape protocol，关联外部 audio/video 时经 custom-data 把云 share id 写入 media library。缺失时断在哪：图片形状树节点缺失（首波按占位矩形画也需该节点），pl-02/pl-06/pr-01 输入断。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 1. 功能说明

把 `p:pic` 事件写入 preallocated picture：non-visual properties、blip fill、shape properties、
style 和 extension list；结束时提交 type=2 的 `Shape` row 与 `ShowShape` protocol。若 picture
关联外部 audio/video 且 custom-data 提供云资源 share id，还要把 share id 写入 media library。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|---|---|---|---|---|---|
| non-visual producer | pp-02/01 | picture nvPr metadata | **pass**；使用 frozen upstream contract | 无 | 不重复实现 nvPr |
| shape extension output | pp-02/02 | ext-list handler | **pass**；使用 frozen upstream contract | 无 | 不复制 ext parser |
| blip/shape/style child handlers | DML parser | typed handler | 当前多为删除/rewrite/deferred | 各 DML function | 不伪造成功 handler |
| media library | show model/app | lookup media id、external flag、setShareId | 当前 Rust app 面被裁剪 | model/media feature | 不实现下载或文件 I/O |
| callback/XML seam | pp-01 + kfc | GVML policy、element dispatch | callback **pass**；XML 使用事件 contract | XML reader 上游 | 不启动完整 XML |

## 2. 核心代码与函数改写列表

| 角色 | 路径 |
|---|---|
| Java/Rust | `.../show/pptxr/handler/shape/SwPictureHandler.java/.rs` |
| callers | `SwGroupShapeHandler`、`SwShapeAlertnateContentHandler`、`ControlListHandler` |

改写列表：outer ctor/init/enterSubElement/endElement；nested
`NonVisualPicturePropertiesHandler`、`AlternateContentHandler`、`FallBackHandler` 的全部方法。

## 3. 测试方案

### 3.1 代码审计与测试面

| symbol | 输入/状态 | 输出/副作用 | 分支 |
|---|---|---|---|
| ctor/init | picture | type=2、id | 首次/复用 |
| enterSubElement | AlternateContent/ext/spPr/style/nvPicPr/blipFill/unknown | child handler | 每个 case/default |
| fallback | fallback blipFill | child handler | present/unknown |
| endElement | mode + media state | commit、style merge、share id | GVML；audio/video；external/internal；tag missing |

### 3.2 覆盖矩阵

| coverage_id | 路径 | 类别 | 必测? | test_id |
|---|---|---|---|---|
| C01 | ctor/init type/id | 状态 | 是 | T01 |
| C02 | 主 child dispatch | 分支 | 是 | T02 |
| C03 | AlternateContent fallback | 兼容分支 | 是 | T03 |
| C04 | normal/GVML commit | 模式 | 是 | T04–T05 |
| C05 | no media/audio/video selection | 分支 | 是 | T06–T08 |
| C06 | media missing/internal/external | 失败/分支 | 是 | T09–T11 |
| C07 | share id missing/empty/non-empty | 边界 | 是 | T12 |
| C08 | repeated init | cleanup | 是 | T13 |

### 3.3 可测试性

使用合法最小 picture、Recording `DgStore`/media library/callback 与 frozen 01/02 输出；通过事件
seam 驱动，不读取图片文件。媒体断言只锁 id 选择和 share-id 写入，不断言下载/解码。

### 3.4 用例表

| id | 覆盖项 | 输入摘要 | 期望 |
|---|---|---|---|
| T01 | C01 | id=7 | type=2、cNvPr.id=7 |
| T02 | C02 | 六类 child + unknown | 正确 handler/None |
| T03 | C03 | choice ignored + fallback blip | fallback blip handler |
| T04–T05 | C04 | normal/GVML | commit；GVML 额外 merge/delete |
| T06–T08 | C05 | none/audio/video | 仅有媒体时选正确 media id |
| T09–T11 | C06 | missing/internal/external | 仅 external 进入 share-id 分支 |
| T12 | C07 | tag absent/empty/value | 仅非空 value 写入一次 |
| T13 | C08 | init 第二张 picture | 不泄漏首张状态 |

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwPictureHandler.rs
test:  libs/ppt-core/tests/cn/wps/show/pptxr/handler/shape/SwPictureHandler.rs
[[test]] name = "03_function_picture_shape"
```

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 03_function_picture_shape
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 03_function_picture_shape \
  --src src/cn/wps/show/pptxr/handler/shape/SwPictureHandler.rs \
  --report libs/ppt-core/parser/features/pp-02-shape-tree-parse/functions/03_function_picture_shape/FUNCTION.md
```

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 03_function_picture_shape --json`)
date: 2026-08-28 14:58

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/handler/shape/SwPictureHandler.rs` | 230/240 | 95.8% | 223/235 | 94.9% |
| **function total** | **230/240** | **95.8%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwPictureHandler.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwPictureHandler.java
```

Loop 结果：**PASS**（100 条 `// Java:` 模板锚点保留且顺序对齐）。

### 3.9 Java↔Rust 结构对齐报告

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwPictureHandler.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwPictureHandler.rs \
  --report libs/ppt-core/parser/features/pp-02-shape-tree-parse/functions/03_function_picture_shape/FUNCTION.md
```

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-28 06:58

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SwPictureHandler` | `koffice/presentation/core/cn/wps/show/pptxr/handler/shape/SwPictureHandler.java` | `libs/ppt-core/src/cn/wps/show/pptxr/handler/shape/SwPictureHandler.rs` | **PASS** | `SwPictureHandler`, `init`, `enterSubElement`, `endElement` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
