# S07 — 排版装配 + 视图参数（页几何/模式）

## 1. 职责

把 TextDocument（S06）+ 视图设置装配成流水线：`LayoutService → LayoutManager`（`TypoDocument(textDoc, notInitSnapshot=true)` + `ViewEnvFactory.create(IViewSettings)` + `FlowLayout`），并创建 `PageViewLayouter`（`MODE_PAGE=0`）。确定内容区宽高（twip）与排版模式。

管线位置：上游 S06；下游 S08（度量）、S09–S11（排版）。

## 2. 核心 Java 代码

| 类 | 路径 | 行数 | 职责 |
|----|------|------|------|
| `LayoutService` | `writer-logic/.../service/LayoutService.java` | 1887 | App API：快照、命中、滚动 |
| `LayoutManager` | `writer-logic/.../core/LayoutManager.java` | 246 | 装配 TypoDocument+ViewEnv+FlowLayout |
| `ViewEnvFactory` | `writer/core/.../layout/base/env/` | — | `create(IViewSettings, …)` → ViewEnv |
| `FlowLayout` | `writer-logic/.../core/FlowLayout.java` | — | `createDocLayouter(mode)` → PageViewLayouter |
| `IViewSettings` | `writer/core/.../service/IViewSettings.java` | 295 | 缩放/模式 → ViewEnv |

LayoutManager 构造（钉死顺序，Java L141–151）：

```java
mTypoDocument = new TypoDocument(textDocument, true);      // 不立即 init snapshot
mLayoutExtraStatus = new LayoutExtraStatus(...);
mViewEnv  = ViewEnvFactory.create(viewSettings, ...);
mFlowLayout = new FlowLayout(mTypoDocument, mViewEnv, cache);
viewSettings.setViewEnv(mViewEnv);
```

`FlowLayout.createDocLayouter`（Java L356–377）按 mode 选 layouter：`0 → PageViewLayouter`（must）；1/2/3/4 → Web/Phone/PhoneScroll/Share（defer）。

## 3. 数据模型

| 模型 | 语义 |
|------|------|
| `ViewEnv` | 视口/模式/气球/修订开关；`mViewMode=0`（MODE_PAGE）+ 页宽高 |
| `PageSetup` | 节页尺寸（twip）：默认 `(11906, 16838, 1800, 1800, 1440, 1440)` → 内容区 **8306×13598** |
| `FlowLayout` | TypoDocument + ViewEnv + cache 的排版驱动器 |

## 4. 输入 / 输出

| 项 | 内容 |
|----|------|
| **I** | TextDocument（S06）+ 视图设置（宽/高/缩放/mode）+ 节 PageSetup（PLCSection） |
| **O** | `ViewEnv`（内容区宽高，twip）、`FlowLayout` + `PageViewLayouter` 就绪、排版范围（整文档/增量） |

## 5. 上下游

- **上游**：S06 TextDocument。
- **下游**：S08 度量（拿 run 样式）、S09–S11（内容区宽度约束断行/分页）。

## 6. 边界 / 失败语义 / 不变量

- MVP **同步全量**排版：无 CoreThread、无增量；`IViewSettings` 用常量替代。
- 内容宽 = 页宽 − 左/右边距（`11906−1800−1800=8306`）；内容高 = `16838−1440−1440=13598`。
- 单位一律 twip；不得混入 px。

## 7. Rust 承接与现状

wr-02：`01_page_setup`（默认 PageSetup → 内容区 8306×13598 twip）+ `02_view_env`（MODE_PAGE，zoom=1）—— **✗ 待 loop**（layout/base/env/ViewEnv.rs 待建）。