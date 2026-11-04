# 01_function_sw_reader_map — 解析标签枚举表

> feature: pp-01-pptx-parse（PPTX parser 基础契约）
> 状态：**pass**（55 tests；函数源码 line/region coverage 100%；layout/alignment PASS）
> 计划来源：[FUNCTIONS.md](../../FUNCTIONS.md) §1 行 01

## 0. 管线能力解锁

已实现并验收：全链 XML token→enum 查表可用（pptxr/dmlr 各 handler 的归一源），属性/枚举统一映射、不再散落魔法数。缺失时断在哪：枚举不可对齐、handler 落值无据，parse 结果无法进模型。
管线环节：S0 parse（master 文档 `libs/ppt-core/PIPELINE_LAYOUT_RENDER_PLAN.md` §1-S0 / §2.1）。

## 功能说明

PPTX 解析器全局常量查表：把 XML 属性字符串（layout 类型、slide size、transition 枚举、placeholder
类型、动画/时间节点枚举、打印枚举等）映射为整数枚举。pptxr / dmlr 各处理器读取这些表做
`tag → enum` 归一，避免散落魔法数。纯静态只读，无 I/O、无状态变更。

## 核心代码

| | 路径 | 规模 |
|--|------|------|
| Java | `koffice/presentation/core/cn/wps/show/pptxr/util/SWReaderMap.java` | 627 行 |
| Rust | `libs/ppt-core/src/cn/wps/show/pptxr/util/SWReaderMap.rs` | 555 行 |

Rust 侧：`pub struct SWReaderMap;`（命名空间壳）+ 54 个 `pub fn get_<map>() ->
&'static HashMap<&'static str, i32>`，`OnceLock` 惰性初始化，`make_map!` 宏建表。字段名与 Java
静态字段一一对应；条目与 Java 字面量逐对一致（测试断言由此机械生成）。

## 1.7 依赖与实现状态

| 依赖 | 归属 | 本 function 需要的接口 | 已实现 | 暂缓 | 禁止本 function 去改 |
|------|------|------------------------|--------|------|----------------------|
| `std::collections::HashMap` | std | 建表/查表 | ✅ | | |
| `std::sync::OnceLock` | std | 惰性初始化 | ✅ | | |
| `make_map!` 宏 | 同文件 | 建表 | ✅ | | |

无 crate 内依赖（不触碰 kso/drawing/show 其它模块）。✓ loop 可从 01 开始。

## 3. 测试方案

### 3.0 编译面裁决

`picparser` 在 `ppt-core/src` 和本 function 测试中均不可达，原 Cargo 清单却无条件编译它；
现已将该依赖改为 opt-in（`full` feature）。同时，尚未进入 MVP function 队列的完整模块树由
`full` 控制，默认构建面只包含当前已验收的 parser 基础模块。`cargo check` 与本 function 测试均通过。

### 3.1 代码审计与测试面

读自 `SWReaderMap.java` 方法体与 `SWReaderMap.rs` 实现（逐对核对）：

| symbol（Rust getter） | Java 字段 | 条目数 | 输入 | 输出/副作用 | 分支 |
|---|---|---|---|---|---|
| `get_conformanceClassMap` | conformanceClassMap | 2 | tag str | i32 查表 | 命中/未命中 |
| `get_slideLayoutTypeMap` | slideLayoutTypeMap | 36 | layout 类型名 | i32 | 命中/未命中 |
| `get_slideSizeMap` | slideSizeMap | 16 | slide size 名 | i32 | 命中/未命中 |
| `get_placeholderSizeMap` | placeholderSizeMap | 3 | 占位符尺寸 | i32 | 命中/未命中 |
| `get_transitionSpeedMap` | transitionSpeedMap | 3 | 切换速度 | i32 | 命中/未命中 |
| `get_transitionSideDirectionMap` | transitionSideDirectionMap | 4 | 切换方向 | i32 | 命中/未命中 |
| `get_transitionCornerDirectionMap` | transitionCornerDirectionMap | 4 | 切换方向 | i32 | 命中/未命中 |
| `get_transitionEightDirMap` | transitionEightDirMap | 8 | 八方向 | i32 | 命中/未命中 |
| `get_transitionInOutDirectionMap` | transitionInOutDirectionMap | 2 | 进/出 | i32 | 命中/未命中 |
| `get_transitionLeftRightDirectionMap` | transitionLeftRightDirectionMap | 2 | 左/右 | i32 | 命中/未命中 |
| `get_transitionCornerAndCenterDirectionMap` | transitionCornerAndCenterDirectionMap | 5 | 角/中心 | i32 | 命中/未命中 |
| `get_transitionPatternMap` | transitionPatternMap | 2 | 图案 | i32 | 命中/未命中 |
| `get_transitionShredPatternMap` | transitionShredPatternMap | 2 | 撕碎图案 | i32 | 命中/未命中 |
| `get_placeholderTypeMap` | placeholderTypeMap | 16 | 占位符类型 | i32 | 命中/未命中 |
| `get_timeNodeFillTypeMap` | timeNodeFillTypeMap | 4 | 时间节点 fill | i32 | 命中/未命中 |
| `get_timeNodeMasterRelationMap` | timeNodeMasterRelationMap | 3 | 主从关系 | i32 | 命中/未命中 |
| `get_timeNodeTypeMap` | timeNodeTypeMap | 9 | 时间节点类型 | i32 | 命中/未命中 |
| `get_timeNodePresetClassMap` | timeNodePresetClassMap | 7 | 预设类 | i32 | 命中/未命中 |
| `get_timeNodeRestartTypeMap` | timeNodeRestartTypeMap | 3 | 重启类型 | i32 | 命中/未命中 |
| `get_timeNodeSyncTypeMap` | timeNodeSyncTypeMap | 2 | 同步类型 | i32 | 命中/未命中 |
| `get_animBehaviorCalcModeMap` | animBehaviorCalcModeMap | 3 | 动画计算模式 | i32 | 命中/未命中 |
| `get_animBehaviorValueTypeMap` | animBehaviorValueTypeMap | 3 | 值类型 | i32 | 命中/未命中 |
| `get_behaviorAccumulateTypeMap` | behaviorAccumulateTypeMap | 2 | 累积类型 | i32 | 命中/未命中 |
| `get_behaviorAdditiveTypeMap` | behaviorAdditiveTypeMap | 5 | 加性类型 | i32 | 命中/未命中 |
| `get_behaviorOverrideTypeMap` | behaviorOverrideTypeMap | 2 | 覆盖类型 | i32 | 命中/未命中 |
| `get_behaviorTransformTypeMap` | behaviorTransformTypeMap | 2 | 变换类型 | i32 | 命中/未命中 |
| `get_chartBuildStepMap` | chartBuildStepMap | 6 | 图表构建步 | i32 | 命中/未命中 |
| `get_dgmBuildStepMap` | dgmBuildStepMap | 2 | 图构建步 | i32 | 命中/未命中 |
| `get_chartSubelementType` | chartSubelementType | 5 | 图表子元素 | i32 | 命中/未命中 |
| `get_animColorSpaceMap` | animColorSpaceMap | 2 | 颜色空间 | i32 | 命中/未命中 |
| `get_animColorDirectionMap` | animColorDirectionMap | 2 | 颜色方向 | i32 | 命中/未命中 |
| `get_animEffectTransitionMap` | animEffectTransitionMap | 3 | 效果切换 | i32 | 命中/未命中 |
| `get_animMotionBehaviorOriginMap` | animMotionBehaviorOriginMap | 2 | 运动原点 | i32 | 命中/未命中 |
| `get_animMotionPathEditModeMap` | animMotionPathEditModeMap | 2 | 路径编辑 | i32 | 命中/未命中 |
| `get_commandTypeMap` | commandTypeMap | 3 | 命令类型 | i32 | 命中/未命中 |
| `get_previousActionTypeMap` | previousActionTypeMap | 2 | 前动作 | i32 | 命中/未命中 |
| `get_nextActionTypeMap` | nextActionTypeMap | 2 | 后动作 | i32 | 命中/未命中 |
| `get_triggerEventMap` | triggerEventMap | 12 | 触发事件 | i32 | 命中/未命中 |
| `get_paraBuildMap` | paraBuildMap | 4 | 段落构建 | i32 | 命中/未命中 |
| `get_diagramBuildMap` | diagramBuildMap | 17 个唯一键（Java 重复 `put("cw", 7)`） | 图构建 | i32 | 命中/未命中 |
| `get_oleChartBuildMap` | oleChartBuildMap | 5 | OLE 图表构建 | i32 | 命中/未命中 |
| `get_animationDgmBuildMap` | animationDgmBuildMap | 4 | 动画图构建 | i32 | 命中/未命中 |
| `get_animationChartBuildMap` | animationChartBuildMap | 5 | 动画图表构建 | i32 | 命中/未命中 |
| `get_iteratorTypeMap` | iteratorTypeMap | 3 | 迭代类型 | i32 | 命中/未命中 |
| `get_triggerRuntimeNodeMap` | triggerRuntimeNodeMap | 3 | 触发节点 | i32 | 命中/未命中 |
| `get_photoAlbumFrameShapeMap` | photoAlbumFrameShapeMap | 7 | 相册框形 | i32 | 命中/未命中 |
| `get_photoAlbumLayoutMap` | photoAlbumLayoutMap | 7 | 相册布局 | i32 | 命中/未命中 |
| `get_viewTypeMap` | viewTypeMap | 8 | 视图类型 | i32 | 命中/未命中 |
| `get_splitterBarStateMap` | splitterBarStateMap | 3 | 分隔条状态 | i32 | 命中/未命中 |
| `get_behaviorAttributeNameMap` | behaviorAttributeNameMap | 117 | 行为属性名 | i32 | 命中/未命中 |
| `get_presetTransitionMap` | presetTransitionMap | 12 | 预设切换 | i32 | 命中/未命中 |
| `get_printWhatMap` | printWhatMap | 9 | 打印内容 | i32 | 命中/未命中 |
| `get_printColorModeMap` | printColorModeMap | 3 | 打印色彩模式 | i32 | 命中/未命中 |
| `get_oleObjectFollowColorSchemeMap` | oleObjectFollowColorSchemeMap | 3 | OLE 跟随配色 | i32 | 命中/未命中 |

读取范围：Java 完整方法体（54 个 init 块）、Rust 54 个 getter、调用方（pptxr/dmlr 处理器对
`SWReaderMap::get_<map>()` 的读取约定：`map.get(tag)`，无值时走默认分支——本波只锁表本身）。

### 3.2 覆盖矩阵

| coverage_id | 接口/代码路径 | 场景类别 | 必测? | test_id / 未覆盖理由 |
|-------------|---------------|----------|-------|----------------------|
| C01 | 54 个 getter 的 `get_or_init` 首次初始化 | 正常主路径 | ✅ | T-META-1（表齐全 54 张） |
| C02 | 每张表全部条目 `key → value` 与 Java 字面量一致 | 交互契约 | ✅ | T-MAP-{54 个 map_* 测试}（每对断言） |
| C03 | 每张表条目数 == Java 字面量条目数 | 边界 | ✅ | T-MAP-*（`len` 断言） |
| C04 | 表中没有的键 → `get` 返回 None（未知 tag 走调用方默认分支） | 失败语义 | ✅ | T-MAP-*（`__no_such_tag__` 断言） |
| C05 | `get_or_init` 幂等（重复调用返回同一实例） | 状态转换 | ✅ | T-REPEAT-1 |
| C06 | 大表（behaviorAttributeNameMap 117 条）全量对 | 边界 | ✅ | T-MAP-behavior_attribute_name_map |

### 3.3 可测试性

纯静态只读：无 fixture、无状态、无 IO。测试只 `use ppt_core::…SWReaderMap::SWReaderMap` 并调各
getter；断言常量全部机械提取自 `SWReaderMap.java` 字面量（生成脚本见 tests 文件头注释）。

### 3.4 用例表

| id | 覆盖项 | 前置状态 | 输入摘要 | 期望输出/副作用 | 失败语义 |
|----|--------|----------|----------|-----------------|----------|
| T-META-1 | 54 张表齐全 | 无 | `EXPECTED_MAPS == 54` | 编译级常量一致 | 表被删/增时暴露 |
| T-MAP-<n> | 每张表逐对 | 无 | 每张表全部 `(key, val)`（Java 字面量） | `m.get(key) == Some(val)`；`m.len()` 一致 | 任一映射漂移即红 |
| T-MAP-<n> | 未知键 | 无 | `"__no_such_tag__"` | `contains_key == false` | 假想 tag 被误收录 |
| T-REPEAT-1 | 幂等 | 首次调用后 | 再次调用同一 getter | 两引用 `ptr::eq`（同一 `&'static`） | 每次重建则违背惰性单例 |

测试文件：`tests/cn/wps/show/pptxr/util/SWReaderMap.rs`（54 个 `map_*` 测试 + 1 个表清单/幂等测试）。
`map_*` 依次覆盖 C02/C03/C04；清单测试覆盖 C01/C05；大表由
`map_behavior_attribute_name_map` 覆盖 C06。

### 3.5 测试代码位置

```text
crate: libs/ppt-core/Cargo.toml
src:   libs/ppt-core/src/cn/wps/show/pptxr/util/SWReaderMap.rs
test:  libs/ppt-core/tests/cn/wps/show/pptxr/util/SWReaderMap.rs
[[test]] name = "01_function_sw_reader_map"  →  cargo test --test 01_function_sw_reader_map
```

### 3.6 测试运行脚本

```bash
cargo test --manifest-path libs/ppt-core/Cargo.toml --test 01_function_sw_reader_map

bash .cursor/skills/feature-function/scripts/function-coverage.sh \
  --crate libs/ppt-core --test 01_function_sw_reader_map \
  --src src/cn/wps/show/pptxr/util/SWReaderMap.rs \
  --report libs/ppt-core/parser/features/pp-01-pptx-parse/functions/01_function_sw_reader_map/FUNCTION.md
```

工具缺失先：`rustup component add llvm-tools-preview && cargo install cargo-llvm-cov --locked`。

### 3.7 覆盖率报告

<!-- coverage-report:begin -->
tool: cargo-llvm-cov (`cargo llvm-cov --test 01_function_sw_reader_map --json`)
date: 2026-08-28 12:24

| src | lines covered | line % | regions covered | region % |
|-----|---------------|--------|-----------------|----------|
| `src/cn/wps/show/pptxr/util/SWReaderMap.rs` | 369/369 | 100.0% | 619/619 | 100.0% |
| **function total** | **369/369** | **100.0%** |  |  |
<!-- coverage-report:end -->

### 3.8 模板布局校验（必跑，PASS 才能标 pass）

```bash
node .cursor/skills/function-java2rust/scripts/check-template-layout.mjs \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/util/SWReaderMap.rs \
  --java koffice/presentation/core/cn/wps/show/pptxr/util/SWReaderMap.java
```

校验点：`// Java:` 行数不降；活成员注释与 Rust 签名相邻；Java 静态字段（54 个 Map）在模板中均有对应注释。

### 3.9 Java↔Rust 结构对齐报告（必跑，overall PASS 才能标 pass）

```bash
node .cursor/skills/feature-function/scripts/java-rust-align-report.mjs \
  --java koffice/presentation/core/cn/wps/show/pptxr/util/SWReaderMap.java \
  --rs libs/ppt-core/src/cn/wps/show/pptxr/util/SWReaderMap.rs \
  --report libs/ppt-core/parser/features/pp-01-pptx-parse/functions/01_function_sw_reader_map/FUNCTION.md
```

说明：Java 侧为 `public static final Map` **字段**（无方法）；Rust 侧以同名 getter 暴露（字段→getter
的结构性适配）。对齐报告按 Java **字段** 一行记录映射，layout 必须 PASS。

<!-- java-rust-align:begin -->
tool: java-rust-align-report.mjs
date: 2026-08-28 04:31

| Java class | Java file | Rust file | layout (`// java`) | implemented | `todo!("DEFERRED_NON_FUNCTION")` | missing in Rust | result |
|---|---|---|---|---|---|---|---|
| `SWReaderMap` | `koffice/presentation/core/cn/wps/show/pptxr/util/SWReaderMap.java` | `libs/ppt-core/src/cn/wps/show/pptxr/util/SWReaderMap.rs` | **PASS** | — | — | — | **PASS** |

**overall: PASS** — every Java class has layout PASS and a Rust counterpart for each method.
<!-- java-rust-align:end -->
