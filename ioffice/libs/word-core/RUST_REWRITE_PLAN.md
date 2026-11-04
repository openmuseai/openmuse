# Word-Core Rust 重写总方案（渐进式 · interface_rewriter.py）

本文档基于 `koffice/writer/core-xxx` 的模块架构，采用 **渐进式重写** 策略：
- 使用 `plugin_java2Rust/interface_rewriter.py` 脚本辅助翻译
- 每一步都确保 **编译通过**（`cargo check`）
- 接口文件先用脚本重写，再逐步补全方法体

---

## 0. 总体进度面板

| Java 代码库 | Java 文件数 | 重写方式 | 进度 |
|:------------|----------:|:-------:|:----:|
| **core-data/src** (data) | 362 | 渐进式 | 待开始 |
| **core/src** (model) | 863 | 渐进式 | 待开始 |
| **core-io/src** (parse) | 942 | 渐进式 | 待开始 |
| **core-layout/src** (layout) | 470 | 渐进式 | 待开始 |
| **core-cache/src** (cache) | 95 | 渐进式 | 待开始 |
| **core-render/src** (render) | 69 | 渐进式 | 待开始 |
| **core-service/src** (service) | 54 | 渐进式 | 待开始 |
| **合计** | **2855** | | |

**最后更新**: 2026-05-11 | **状态**: 计划制定完成，待开始执行

---

## 1. 重写策略

### 1.1 渐进式重写原则

1. **每步编译通过**: 每完成一批文件，必须 `cargo check` 通过
2. **接口先行**: 先用 `interface_rewriter.py` 生成接口骨架（struct/trait/enum + 方法签名 + `todo!()` 方法体）
3. **逐步填充**: 按依赖顺序逐步实现方法体
4. **最小可编译单元**: 每次重写以 Java 代码文件为单位，按依赖关系排序

### 1.2 重写工具

```bash
# 全文件接口重写
python plugin_java2Rust/interface_rewriter.py \
  --src <java_dir> --scene "接口" --recursive false --mode full

# 代码块重写（标记 // REWRITE_THIS_BLOCK）
python plugin_java2Rust/interface_rewriter.py \
  --src <java_dir> --scene "接口" --recursive false --mode block
```

### 1.3 目录映射规则

Java 包路径去除 `cn/wps/moffice/writer/` 前缀 → Rust module 路径：

```
Java: cn.wps.moffice.writer.data.value.XXX
Rust: word_core::data::value::xxx

Java: cn.wps.moffice.writer.core.XXX
Rust: word_core::core::xxx

Java: cn.wps.moffice.writer.layout.base.XXX
Rust: word_core::layout::base::xxx
```

---

## 2. 功能优先级

| 优先级 | 功能分类 | 说明 | 阶段文档 |
|:------:|---------|------|---------|
| **P1** | 基础数据模型 | `core-data/*` — 前置依赖 | `rewrite_phases/phase1_data.md` |
| **P2** | 核心渲染管线 | core模型 → IO解析 → cache → layout → render | `rewrite_phases/phase2_pipeline.md` |
| **P3** | HitTest 等基础功能 | `service/hittest`, `service/locate`, `service/drawing` | `rewrite_phases/phase3_hittest.md` |
| **P4** | 其他功能 | async, selection, protection, 编辑辅助等 | `rewrite_phases/phase4_others.md` |

### 依赖关系

```
P1: data (value → field → table → math → autonum → theme → docfield → root)
        ↓
P2: core模型 (env/param/message → 小模块 → core根 → table/shape/list)
        ↓
    IO (docxReader → xml07/dml/vml)
        ↓
    cache (root → style → disk → firstcache)
        ↓
    layout (base → line/env/graph → table/drawing/math → pageview/phoneview)
        ↓
    render (base → renderer → drawing → cache/tool)
        ↓
P3: service (hittest → locate → drawing)
        ↓
P4: async, selection, protection, 编辑功能
```

---

## 3. 各阶段概要

### Phase 1: 基础数据模型 (P1) — 362 文件

详见 `rewrite_phases/phase1_data.md`

| 步骤 | 目录 | Java 文件 | 依赖 | 说明 |
|------|------|----------:|------|------|
| 1.1 | data/value | 114 | — | 属性值对象，最底层 |
| 1.2 | data/math | 15 | — | 数学公式数据 |
| 1.3 | data/theme | 8 | — | 主题数据 |
| 1.4 | data/autonum | 13 | — | 自动编号 |
| 1.5 | data/docfield | 5 | — | 文档字段数据 |
| 1.6 | data/field | 56 | value | 字段代码 |
| 1.7 | data/table | 30 | value | 表格数据 |
| 1.8 | data/ 根 | 69 | value, field, table | PLC/PieceTable/属性表 |
| 1.9 | data/backup | 28 | data根 | 延后（非核心） |
| 1.10 | data/internal_v2 | 17 | data根 | 延后（非核心） |

### Phase 2: 核心渲染管线 (P2) — 2439 文件

详见 `rewrite_phases/phase2_pipeline.md`

分为 5 个子阶段：
- **2A**: 核心文档模型 (core/src) — 863 文件
- **2B**: IO 解析 (core-io/src) — 942 文件（优先 docxReader 74 文件）
- **2C**: 排版快照 (core-cache/src) — 95 文件
- **2D**: 排版引擎 (core-layout/src) — 470 文件
- **2E**: 渲染层 (core-render/src) — 69 文件

### Phase 3: HitTest + 定位 (P3) — 33 文件

详见 `rewrite_phases/phase3_hittest.md`

| 步骤 | 目录 | Java 文件 | 依赖 |
|------|------|----------:|------|
| 3.1 | service/hittest | 9 | layout, cache |
| 3.2 | service/locate | 8 | layout, cache |
| 3.3 | service/drawing | 5 | layout, shape |
| 3.4 | service/ 根 | 21 | layout, render |

### Phase 4: 其他功能 (P4) — 延后

详见 `rewrite_phases/phase4_others.md`

---

## 4. 编译验证检查点

每个步骤完成后运行：

```bash
cd libs/word-core && cargo check
```

确保编译通过后再进入下一步。

---

## 文档日期
- **编写日期**: 2026-05-11
- **策略**: 渐进式重写，interface_rewriter.py 辅助翻译
- **状态**: 计划制定完成，待开始执行
