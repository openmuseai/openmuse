# 02_function_view_env — 视图状态（宽高/模式/缩放）

| 项 | 值 |
|----|----|
| feature | wr-02-viewenv |
| **入口** | `ViewEnv`（Java `layout/base/env/ViewEnv.java`）：`setViewWidth/Height`·`setViewMode`·`getViewWidth/Height`·`isPageMode`·`setViewSettings`·`onScroll`·`getViewTop` |
| 独立性 | 叶子（状态容器 + 换算） |

## 1. 功能说明
视图状态容器：MVP 钉死 **页模式（PAGEVIEW=0）**、缩放 1.0、单列；`contentWidth/Height` 由 01 的节几何按缩放换算（缇→像素 MVP 1:1 或 96dpi 缩放常量）。`changeSize(needClearFontStyleCache)` 语义：视图尺寸变化 → 置“需重排”标志（wr-05 消费）。

### 1.7 依赖与实现状态
| 依赖 | 归属 | 接口 | 已实现 | 备注 |
|------|------|------|--------|------|
| PageSetupGeom | 01（本 feature） | content 换算 | 01 先行 | — |
| IViewSettings | writer-logic（Java UI 侧） | 读取 | Rust 用扁平字段替代 | 不建 UI 依赖 |

## 3. 测试方案
### 3.2/3.4 矩阵与用例
| id | coverage | 输入摘要 | 期望 | 失败语义 |
|----|----------|----------|------|----------|
| V1 | ENV-SETGET | 宽高设 8306/13598 | 读回一致 | 存取错 |
| V2 | ENV-CONTENT | pageSetup(默认)+zoom=1 | content 8306×13958 | 换算错 |
| V3 | ENV-ZOOM | zoom=2 | content 减半（或按约定像素化） | 缩放错 |
| V4 | ENV-MODE | mode=PAGEVIEW | isPageMode true；web/phone false | 模式错 |
| V5 | ENV-RESET | 改宽后 changeSize | 重排标志置位；再读回标志可清 | 状态错 |
| V6 | ENV-SCROLL | onScroll(120)+getViewTop | top==120 | 滚动态错 |

### 3.3 可测试性
纯状态容器直测；不依赖渲染/排版器。

### 3.5/3.6/3.7/3.8
src: `libs/word-core/src/cn/wps/moffice/writer/layout/base/env/ViewEnv.rs`
test: `libs/word-core/tests/cn/wps/moffice/writer/layout/base/env/ViewEnv.rs`
```bash
cargo test --manifest-path libs/word-core/Cargo.toml --test 02_function_view_env
bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/word-core --test 02_function_view_env \
  --src src/cn/wps/moffice/writer/layout/base/env/ViewEnv.rs \
  --report libs/word-core/layout/features/wr-02-viewenv/functions/02_function_view_env/FUNCTION.md
```
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 02_function_view_env --json`)
date: 2026-08-22 02:16

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/moffice/writer/layout/base/env/ViewEnv.rs` | 64/355 | 18.0% | 82/369 | 22.2% |
| **function total** | **64/355** | **18.0%** |  |  |
<!-- coverage-report:end -->
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/word-core/src/cn/wps/moffice/writer/layout/base/env/ViewEnv.rs \
  --java koffice/writer/core/cn/wps/moffice/writer/layout/base/env/ViewEnv.java
```
校验：`// java` 保留。**本轮结果：** ViewEnv.rs + ViewMode.rs PASS。
