# 05_function_fill_base — Fill → FillBase（含 useBgFill）

> feature: pl-09-public-shape-convert ｜ 状态：**pass** ｜ Java 管线：S5 步骤 5.5 填充

## 0. 管线能力解锁

实现后：公开 Shape 的 fill 是冻结的 `FillBase`（solid/blip MVP），`useBgFill` 在转换时刻展开为背景 fill + `setBgWidth/Height`。渲染器不再走 `ShapePr.follow` 或 theme `fillAt`。

## 1. 功能说明

### 1.1 I/O 契约

| 项 | 契约 |
|----|------|
| **输入** | `KmoShowShape` 的 fill 来源：`hasUseBgFill` → slide `Background.fill` 或 `bgRef→theme.fillAt`；否则 `shapeProperties.fill()`；`ColorInterpretCtx` |
| **输出** | `dstShape.setFill(FillBase)` 或 null（`filterFill` 命中则强制 null） |
| **错误** | `convertToFillBase` 不识别的 type（含 5 noFill）→ 返回 null，再尝试 overlay。不 panic |

Java fill.type（`FillConvert.convertToFillBase:47-80`）：

| type | 含义 | MVP |
|---|---|---|
| 0 | solid | must |
| 1 | pattern | defer（filter 4098 可测跳过） |
| 2 | blip | must |
| 3 | grpFill | 递归 parent fill（must 最小：有 parent 则转父 fill） |
| 4 | grad | defer（不得画成实色冒充） |
| 5 | noFill | 不进 convertToFillBase；调用方当无 fill |

`filterFill`（`ConvertTool.java:1409-1417`）：type 1 → `isFilterPatternFill`；type 2 → `isFilterBlipFill`；其余 false。

useBgFill（`:199-245`）：`supportUseBgFill` 时 `setUseBgFill(true)` + slide 宽高 `fromEmuX/Y`。

### 1.2 / 1.3
入口：`FillConvert.convertToFillBase(Fill, KmoShowShape, ColorInterpretCtx)` 与三参 media 重载。装配在 `shapePropertiesToPublicShape:197-258`。

### 1.4 上游
pp-10 `FillObj.fill_type/color/blip_*`；pp-16 theme `fillAt`（bgRef）；pp-18 `media_bytes`（blip）。

### 1.5 下游
pr-02 `getFill()`；06 不读 fill 决策（线的 type 5 在 ln.fill）。

### 1.6 不变量
1. solid → `SolidFill` 颜色来自 ColorInterpreter（MVP 可先透传 srgb）。
2. filter blip → `setFill(null)`，即使已有 fill。
3. 渐变不得写成 solid。
4. textBox+BlipFill → `setIsInTextBox(true)`（`:256`）。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `FillConvert.convertToFillBase` | 本 function | type 分派 | ✅ `convert/mod.rs` 热路径 type 0/2/3 + useBgFill | pattern/grad 细节 | 禁止项目 stub |
| `FillBase`/`SolidFill`/`BlipFill` | drawing | setColor / blip | ⚠️ 类型在 drawing | PPT BlipFillPPT | 禁止另造 Fill 枚举冒充 |
| `layout_shape.FillObj` | pp-10 | fill_type/color/media | ✅ | 真 `data.fill.Fill` | 禁止改 handler |
| `filterFill` | ConvertTool | FilterInterpretCtx | ❌ | — | 禁止改 4097/4098 |
| `ColorInterpretCtx` | radapter | withPhColor | cfg full stub | 完整主题色 | MVP 透传已解析 ARGB |
| overlay / fillOverlay blend | ConvertTool:224 | — | defer | — | 不假装 overlay |

## 2. 核心代码与函数改写列表

- **本 function：** `convertToFillBase` 两重载；`convertToSolidFill`；`convertToBlipFill`；装配段 useBgFill / filterFill / textBox blip 标记。
- **停车：** `convertToGradFill` / `convertToPatternFill` 签名保留 DEFERRED；effect overlay。
- **禁改：** `rendercommand.rs` 实色捷径；禁止 `--force` PptxrSlide。

## 3. 测试方案

### 3.1 代码审计与测试面

已读：`FillConvert.java:47-80` type 0/2/4/1/3；`ConvertTool.java:197-258` 来源优先级 useBgFill > shapePr.fill > overlay；`filterFill:1409`；`FillObj` `putType`。

| symbol | 路径 | 输入 | 输出 | 分支 |
|--------|------|------|------|------|
| solid | type 0 | srgb | SolidFill | must |
| blip | type 2 | embed/media | BlipFill | must |
| noFill | 无 hasFill | — | fill 仍 null | |
| filter blip | flag 4097 | type 2 | setFill null | |
| useBgFill | isUseBgFill | bg fill | FillBase + bg 尺寸 | |
| grpFill | type 3 | parent solid | 父 fill | |
| grad | type 4 | — | 非 solid（None 或 GradFill） | 禁冒充 |
| textBox blip | isTextBox | BlipFill | isInTextBox | |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| FB-SOLID | convertToSolidFill | type 0 | 是 | FB1 |
| FB-BLIP | convertToBlipFill | type 2 | 是 | FB2 |
| FB-NONE | 无 fill | null | 是 | FB3 |
| FB-FILTER | filterFill blip | null fill | 是 | FB4 |
| FB-BG | useBgFill | bg 冻结 | 是 | FB5 |
| FB-GRP | type 3 | 父 fill | 是 | FB6 |
| FB-GRAD | type 4 | 非 solid | 是 | FB7 |
| FB-TB | textBox+blip | isInTextBox | 是 | FB8 |

### 3.3 可测试性
fixture `FillObj { fill_type, color, blip_embed }`。FB1–FB8 ignore 至 FillConvert 接线。Split 无已落地 fill convert API 可绿。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 | 失败语义 |
|----|------|------|------|------|----------|
| FB1 | FB-SOLID | type=0 color=已知 | getFill Solid 同色 | 丢 alpha | |
| FB2 | FB-BLIP | type=2 media | BlipFill 非空 | 画成实色 | |
| FB3 | FB-NONE | has_fill=false | getFill null | 默认白 | |
| FB4 | FB-FILTER | FILTER_BLIP_FILL | fill null | 仍画图 | |
| FB5 | FB-BG | useBgFill + bg solid | useBgFill true + bgW/H | 仍用 sp fill | |
| FB6 | FB-GRP | type=3 + parent | 同父 solid | 空 fill | |
| FB7 | FB-GRAD | type=4 | 不是 SolidFill | 用 gs[0] 冒充实色 | |
| FB8 | FB-TB | isTextBox+blip | isInTextBox | 漏标记 | |

### 3.5 位置
```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/convert/FillConvert.rs（loop 新建交错模板）
       src/cn/wps/show/convert/ConvertTool.rs（装配段）
test:  tests/cn/wps/show/convert/05_function_fill_base.rs
```

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 05_function_fill_base

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 05_function_fill_base \
  --src src/cn/wps/show/convert/FillConvert.rs \
  --src src/cn/wps/show/convert/ConvertTool.rs \
  --report libs/ppt-core/layout/features/pl-09-public-shape-convert/functions/05_function_fill_base/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 05_function_fill_base --json`)
date: 2026-09-11 10:19

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/convert/ConvertTool.rs` | 155/604 | 25.7% | 353/1205 | 29.3% |
| `src/cn/wps/show/convert/FillConvert.rs` | 47/89 | 52.8% | 57/113 | 50.4% |
| **function total** | **202/693** | **29.1%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/convert/FillConvert.rs \
  --java koffice/presentation/core/cn/wps/show/convert/FillConvert.java
```

### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/convert/FillConvert.java \
  --rs libs/ppt-core/src/cn/wps/show/convert/FillConvert.rs \
  --report libs/ppt-core/layout/features/pl-09-public-shape-convert/functions/05_function_fill_base/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-11 02:19

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `FillConvert` | `koffice/presentation/core/cn/wps/show/convert/FillConvert.java` | `libs/ppt-core/src/cn/wps/show/convert/FillConvert.rs` | **PASS** | `convertToFillBase`, `convertToSolidFill`, `convertToBlipFill`, `convertToGradFill`, `convertToPatternFill`, `getAlphaColorTransform` | `getFillFlip`, `getFillAlignment`, `getPatternPrst` | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
