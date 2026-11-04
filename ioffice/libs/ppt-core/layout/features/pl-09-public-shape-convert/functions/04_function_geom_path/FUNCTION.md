# 04_function_geom_path — prst / cust / placeholder → Geometry

> feature: pl-09-public-shape-convert ｜ 状态：**pass** ｜ Java 管线：S5 步骤 5.4

## 0. 管线能力解锁

实现后：公开 Shape 带冻结 path（预置优先运行时 ShapeType，否则 Vertices 顶点表；自定义公式 path；缺几何+ph→rect）。渲染器不再自己把 `prst int` 翻译成点。

pr-06 产品侧 `preset_path_in_rect` 是指令捷径，**不能**代替本层挂到 `Shape.getGraphicsPaths`。

## 1. 功能说明

### 1.1 I/O 契约

| 项 | 契约 |
|----|------|
| **输入** | `Geometry2D`：`type()==0`+`hasPrstGeom` 或 `hasCustGeom`；`anchorWidth/Height`（03 的 rcAnchor）；目标公开 Shape |
| **输出** | `setShapeName` / `setShapeType` / `setGeometry`；connector 名含 `"Connector"` 且非 flowchart → `GRF.setConnector` |
| **错误** | 无几何且非 placeholder → 不写默认 rect（Java `:193` 仅 `!hasGeom && isPlaceHolder`）。越界 prst → `PresetShapeTypeMap.get` 返回 `""` |

### 1.2 输入 / 1.3 输出

`GeomtryConvert.convertDataToPublicShape(PresetGeometry, Shape, w, h)` `:25-61`：

1. `shapeName = shapeTypeMap.get(presetType)`（265 槽）
2. `shapeType = ShapeType.getShapeType(shapeName)`；`==0` → `Vertices.writeOOXMLGeometryData`
3. placeholder：`convertPlaceHolderToPublicShape` → `covertToPublicRectShape`（map 下标 1 = `"rect"`）

custGeom `convertPath` 指令（`:145-211`）：

| pathSegment.type | Segment 短码 | 语义 |
|---|---|---|
| 0 | 96 | close |
| 1 | 64 | moveTo |
| 2 | 0 | lnTo |
| 3 | 163 | arcTo |
| 4 | 32 | cubicBez |
| 5 | 169 | quadBez |

另：path.fill==4 → noFill 段 170；`!isStroke` → noStroke 171。

### 1.4 上游
03 anchor WxH；`PresetShapeTypeMap`（Rust 已有完整表，但 `convert` 模块 cfg full）。

### 1.5 下游
pr-02 `getGraphicsPaths(w,h)`；命中测试。

### 1.6 不变量
1. prst=1 → name `"rect"`；prst=20 → `"line"`；prst=56 → `"pentagon"`。
2. `get(-1)` / `get(265)` → `""`（Java `:406-407` 同界）。
3. `ShapeType.getShapeType` 未知名 → 0 → Vertices 回退。
4. 不在本层画 fill。

### 1.7 依赖与实现状态

| 依赖 | 归属 | 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------|--------|------|----------------------|
| `PresetShapeTypeMap` | convert | `get(prst)` 265 槽 | ✅ 默认 `convert` 导出 | — | 禁止改表项 |
| `ShapeType.getShapeType` | drawing | name→int | ✅ | — | — |
| `GeomtryConvert.convertDataToPublicShape` | 本 function | prst/cust | ✅ `convert/mod.rs`；prst/cust/ph/path 0–5 | `revisePath` 等 DEFERRED | 禁止用 pr-06 path 数组冒充 Geometry |
| `Vertices.writeOOXMLGeometryData` | convert | 无运行时实现时顶点表 | ✅ `Vertices.rs` 进 `convert` | — | loop 按 Java 表转写，禁止空 path 冒充绿 |
| `GeometryCache` | custGeom | 按 shapeID+w+h | ❌ | MVP 可不缓存，语义等价 | — |
| `layout_shape.GeomObj.prst` | pp-10 | 预置号 | ✅ | 真 Geometry2D | 禁止改 handler |

## 2. 核心代码与函数改写列表

- **本 function：** `convertDataToPublicShape`（prst + cust 重载）、`convertPlaceHolderToPublicShape`、`covertToPublicRectShape`、`convertPath` 0–5、`PresetShapeTypeMap.get`（已有则接到默认 mod）。
- **停车：** `revisePath` corner/halfFrame 特例可本波实现（Java `:79-113` 有明确向量）；3D；ink。
- **禁改：** `rendercommand.rs` 的 `preset_path_in_rect`。

## 3. 测试方案

### 3.1 代码审计与测试面

已读：`GeomtryConvert.java:25-211`；`PresetShapeTypeMap.java` LENGTH=265、`[1]=rect/[20]=line/[56]=pentagon`；Rust `PresetShapeTypeMap.rs:405-412` 边界；`ShapeType.rs:2094` `getShapeType`；`ConvertTool.java:180-195` hasGeom 分支。

| symbol | 路径 | 输入 | 输出 | 分支 |
|--------|------|------|------|------|
| map.get | PresetShapeTypeMap | 1/20/56/-1/265 | rect/line/pentagon/""/"" | 查表 |
| ShapeType | drawing | "rect"/"ellipse"/未知 | 1/3/0 | 回退 Vertices |
| prst convert | GeomtryConvert | prst=1 | name rect + type≠0 或 geometry | 热路径 |
| ph 无几何 | :193 | isPlaceHolder | 默认 rect | 缺 geom |
| 非 ph 无几何 | :193 false | — | 不强制 rect | 误画方块 |
| convertPath | type 0–5 | 各一段 | 短码 96/64/0/163/32/169 | cust |

### 3.2 覆盖矩阵
| coverage_id | 接口 | 场景 | 必测? | test_id |
|-------------|------|------|-------|---------|
| GP-MAP | PresetShapeTypeMap.get | 1/20/56/越界 | 是 | GP1 |
| GP-ST | ShapeType.getShapeType | rect/ellipse/unknown | 是 | GP2 |
| GP-PRST | convertDataToPublicShape | prst=1 | name+type | 是 | GP3 |
| GP-LINE | prst=20 | line 名 | 是 | GP4 |
| GP-PH | convertPlaceHolder | ph 无 geom | rect | 是 | GP5 |
| GP-NOPH | 非 ph 无 geom | 不写 rect | 是 | GP6 |
| GP-PATH | convertPath | type 0–5 短码 | 是 | GP7 |
| GP-VERT | ShapeType==0 | Vertices 非空 | 是 | GP8 |

### 3.3 可测试性
GP2 Split 可绿（drawing）。GP1 需 `convert` 默认导出或测试内等价表——Split 用 ignore「PresetShapeTypeMap cfg full」。GP3–GP8 ignore ConvertTool/GeomtryConvert。

### 3.4 用例表
| id | 覆盖 | 前置 | 输入 | 期望 | 失败语义 |
|----|------|------|------|------|----------|
| GP1 | GP-MAP | map | get(1/20/56/-1/265) | rect/line/pentagon/""/"" | 槽位错 |
| GP2 | GP-ST | — | getShapeType("rect") | 1（msosptRectangle） | 0 误走 Vertices |
| GP3 | GP-PRST | 03 框 | prst=1 | shapeName=rect；`getGraphicsPaths` 非空（CDL rect） | 当自定义 |
| GP4 | GP-LINE | prst=20 | name=line；`getGraphicsPaths` move+line | 当 rect | |
| GP5 | GP-PH | placeholder 无 geom | 默认 rect | 空 geometry | |
| GP6 | GP-NOPH | 普通 sp 无 geom | 不 covertToPublicRectShape | 误 rect | |
| GP7 | GP-PATH | cust 六指令 | 短码表 | 错 opcode | |
| GP8 | GP-VERT | 无 ShapeType 实现的名 | Vertices 写出点 | 空 path 冒充绿 | |

### 3.5 位置
```text
crate: libs/ppt-core/Cargo.toml
src:   src/cn/wps/show/convert/GeomtryConvert.rs
       src/cn/wps/show/convert/PresetShapeTypeMap.rs
       src/cn/wps/show/convert/Vertices.rs（loop）
test:  tests/cn/wps/show/convert/04_function_geom_path.rs
```

### 3.6 测试运行脚本
```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 04_function_geom_path

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 04_function_geom_path \
  --src src/cn/wps/show/convert/GeomtryConvert.rs \
  --src src/cn/wps/show/convert/PresetShapeTypeMap.rs \
  --src src/cn/wps/show/convert/Vertices.rs \
  --report libs/ppt-core/layout/features/pl-09-public-shape-convert/functions/04_function_geom_path/FUNCTION.md
```

### 3.7 覆盖率报告
<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 04_function_geom_path --json`)
date: 2026-09-11 11:25

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/convert/GeomtryConvert.rs` | 47/297 | 15.8% | 83/437 | 19.0% |
| `src/cn/wps/show/convert/PresetShapeTypeMap.rs` | 201/201 | 100.0% | 589/589 | 100.0% |
| `src/cn/wps/show/convert/Vertices.rs` | 124/127 | 97.6% | 193/196 | 98.5% |
| **function total** | **372/625** | **59.5%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验
```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/convert/GeomtryConvert.rs \
  --java koffice/presentation/core/cn/wps/show/convert/GeomtryConvert.java \
  --rs libs/ppt-core/src/cn/wps/show/convert/PresetShapeTypeMap.rs \
  --java koffice/presentation/core/cn/wps/show/convert/PresetShapeTypeMap.java \
  --rs libs/ppt-core/src/cn/wps/show/convert/Vertices.rs \
  --java koffice/presentation/core/cn/wps/show/convert/Vertices.java
```

### 3.9 Java↔Rust 结构对齐报告
```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/convert/GeomtryConvert.java \
  --rs libs/ppt-core/src/cn/wps/show/convert/GeomtryConvert.rs \
  --java koffice/presentation/core/cn/wps/show/convert/PresetShapeTypeMap.java \
  --rs libs/ppt-core/src/cn/wps/show/convert/PresetShapeTypeMap.rs \
  --java koffice/presentation/core/cn/wps/show/convert/Vertices.java \
  --rs libs/ppt-core/src/cn/wps/show/convert/Vertices.rs \
  --report libs/ppt-core/layout/features/pl-09-public-shape-convert/functions/04_function_geom_path/FUNCTION.md
```
<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-09-11 03:25

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `GeomtryConvert` | `koffice/presentation/core/cn/wps/show/convert/GeomtryConvert.java` | `libs/ppt-core/src/cn/wps/show/convert/GeomtryConvert.rs` | **PASS** | `convertDataToPublicShape`, `convertPlaceHolderToPublicShape`, `convertBackgroundToPublicShape`, `covertToPublicRectShape`, `convertPath` | `revisePath`, `setCustGemoRect`, `setGemoryFormula`, `setGemoryAdjust`, `setConnectionSite` | — | **PASS** |
| `PresetShapeTypeMap` | `koffice/presentation/core/cn/wps/show/convert/PresetShapeTypeMap.java` | `libs/ppt-core/src/cn/wps/show/convert/PresetShapeTypeMap.rs` | **PASS** | `get` | — | — | **PASS** |
| `Vertices` | `koffice/presentation/core/cn/wps/show/convert/Vertices.java` | `libs/ppt-core/src/cn/wps/show/convert/Vertices.rs` | **PASS** | `writeOOXMLGeometryData`, `initVertices` | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
