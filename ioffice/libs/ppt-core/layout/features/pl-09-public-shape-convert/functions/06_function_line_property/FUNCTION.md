# 06_function_line_property — Outline → LineProperty

> feature: pl-09-public-shape-convert ｜ 状态：**pass** ｜ Java 管线：S5 步骤 5.5 线条

## 0. 管线能力解锁

实现后：公开 Shape 的 stroke 是冻结 `LineProperty`（宽、端、接、dash、箭头、线填充）。`ln.fill.type==5`（noFill）→ **整条 lineProperty=null**（Java `:264-265`），渲染器零推导。

线宽下限：`min_width = round(fromEmuX(12700))`；`hasWidth` 后 `max(min_width, width)`；若 `getWidth()<=0.75` 再钳到 min（`:19-26`）。

## 1. 功能说明

### 1.1 I/O 契约

| 项 | 契约 |
|----|------|
| **输入** | `shapeProperties.hasLn()` + `Outline`（width/compound/cap/join/fill/headEnd/tailEnd/presetDash）+ `ColorInterpretCtx` + `IMediaLib` |
| **输出** | `dstShape.setLineProperty(LineProperty\|null)` |
| **错误** | `hasLn` 但既无 fill 也无已有 lineProperty → Java `:263` 整段不进，保持原 lineProperty。不 panic |

装配（`ConvertTool.java:260-276`）：

```
if hasLn:
  if hasFill || lineProperty != null:
    if hasFill && fill.type==5: lineProperty = null
    else: new LineProperty if null; LineStyleConvert.convertToLineProperty(...)
    setLineProperty(lineProperty)
```

`LineStyleConvert.convertToLineProperty` 还处理：compound 0–4、cap 0–2、join 0–2、fill 0/1/4/5、箭头 type≠0、presetDash 0–10。

### 1.4 上游
pp-10 `OutlineObj`；05 的 FillConvert 被线 fill 调用（solid/grad）。

### 1.5 下游
pr-02 stroke；命中描边宽度。

### 1.6 不变量
1. 12700 EMU 下限（round fromEmuX）。
2. type 5 → property **null**，不是 width=0 的对象。
3. 不在本层改 path。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `LineStyleConvert.convertToLineProperty` | 本 function | width/cap/join/dash/arrow/fill | ✅ `convert/mod.rs` | pattern 线填 | 禁止项目 stub |
| `LineProperty` | drawing | setWidth/Color/… | ⚠️ crate 有类型 | — | 禁止新线类型名 |
| `UnitsConverter.fromEmuX(12700)` | pl-01 | min_width | ✅ | — | 禁止改 12700 |
| `OutlineObj` | pp-10 | width/cap/join/dash/fill | ✅ | 真 Outline KoBin | — |
| `FillConvert` 线填 | 05 | solid 线色 | ✅ | grad 线 | — |

## 2. 核心代码与函数改写列表

- **本 function：** `convertToLineProperty`；`convetToArrow`；ConvertTool `:260-276` 装配（含 type 5 → null）。
- **停车：** 线 pattern/grad 完整（type 1/4 签名保留）。
- **禁改：** pr-06 对 `prst==20` 默认黑描边的产品补丁（那是指令层缺口；本层冻结后 pr-02 应改读 LineProperty）。

## 3. 测试方案

### 3.1 代码审计与测试面

已读：`LineStyleConvert.java:15-197`；`ConvertTool.java:260-276`；Rust 无 LineStyleConvert.rs。

| symbol | 路径 | 输入 | 输出 | 分支 |
|--------|------|------|------|------|
| min_width | :16/19-26 | width 缺 / 很小 / 很大 | ≥ min；大值保留 | 钳位 |
| noFill 线 | :264 type 5 | — | lineProperty=null | 关键 |
| compound/cap/join | switch 0–4/0–2/0–2 | 透传 | |
| dash | presetDash 0–10 | setDashing | |
| arrow | type≠0 | start/end Arrow | type==0 不设 |
| 无 ln | hasLn false | 不改 | |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| LP-MIN | min_width 公式 | fromEmuX(12700).round | 是 | LP1 |
| LP-CLAMP | width&lt;min / ≤0.75 | 钳到 min | 是 | LP2 |
| LP-WIDE | width≫min | 保留换算值 | 是 | LP3 |
| LP-NOFILL | fill.type==5 | property null | 是 | LP4 |
| LP-SOLID | 线 solid | color+width | 是 | LP5 |
| LP-CAPJOIN | cap/join/compound | 透传 | 是 | LP6 |
| LP-DASH | presetDash | 0 与 3 | 是 | LP7 |
| LP-ARROW | headEnd type=1 | startArrow | 是 | LP8 |
| LP-NOLN | !hasLn | 保持 null | 是 | LP9 |

### 3.3 可测试性
LP1 Split 可绿（UnitsConverter + round）。其余 ignore `spec: LineStyleConvert 未接线`。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 | 失败语义 |
|----|------|------|------|------|----------|
| LP1 | LP-MIN | metrics | round(fromEmuX(12700)) | 与 Java min_width 同公式 | 用 1px 硬编码 |
| LP2 | LP-CLAMP | width=1 EMU | getWidth==min | 变成 0 看不见线 | |
| LP3 | LP-WIDE | width=1270000 | &gt; min 且 = fromEmuX | 被钳死 | |
| LP4 | LP-NOFILL | ln fill type 5 | getLineProperty null | width=0 对象 | |
| LP5 | LP-SOLID | 线色+宽 | LineProperty fill+color | 丢色 | |
| LP6 | LP-CAPJOIN | cap=1 join=2 compound=1 | 同号 | switch 漏 case | |
| LP7 | LP-DASH | dash=0 与 3 | dashing 同 | 全当实线 | |
| LP8 | LP-ARROW | head type=1 | startArrow 非空 | type0 也造箭头 | |
| LP9 | LP-NOLN | !hasLn | line null | 默认黑 1px | |

### 3.5 位置
```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/convert/LineStyleConvert.rs（loop 新建）
       src/cn/wps/show/convert/ConvertTool.rs
test:  tests/cn/wps/show/convert/06_function_line_property.rs
```

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 06_function_line_property

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 06_function_line_property \
  --src src/cn/wps/show/convert/LineStyleConvert.rs \
  --src src/cn/wps/show/convert/ConvertTool.rs \
  --report libs/ppt-core/layout/features/pl-09-public-shape-convert/functions/06_function_line_property/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 06_function_line_property --json`)
date: 2026-09-11 10:19

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/convert/ConvertTool.rs` | 146/604 | 24.2% | 333/1205 | 27.6% |
| `src/cn/wps/show/convert/LineStyleConvert.rs` | 63/243 | 25.9% | 82/194 | 42.3% |
| **function total** | **209/847** | **24.7%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/convert/LineStyleConvert.rs \
  --java koffice/presentation/core/cn/wps/show/convert/LineStyleConvert.java
```

### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/convert/LineStyleConvert.java \
  --rs libs/ppt-core/src/cn/wps/show/convert/LineStyleConvert.rs \
  --report libs/ppt-core/layout/features/pl-09-public-shape-convert/functions/06_function_line_property/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-11 02:19

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `LineStyleConvert` | `koffice/presentation/core/cn/wps/show/convert/LineStyleConvert.java` | `libs/ppt-core/src/cn/wps/show/convert/LineStyleConvert.rs` | **PASS** | `convertToLineProperty`, `convetToArrow` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
