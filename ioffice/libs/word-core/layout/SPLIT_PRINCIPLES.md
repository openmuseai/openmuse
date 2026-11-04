# Layout Feature / Function 拆分原理

> 对照 parse：`libs/word-core/parser/features/` 的 wc-01..05。  
> 同一套 skill：`feature-structure-analysis`（类图）→ `feature-function`（I/O 切 function）→ `function-java2rust`（改写）→ `feature-cargo-gate`。

---

## 1. 为何还是先拆 feature

Java writer layout 不是一个类，而是 **装配（logic）+ 排版算法（core/layout）+ 快照内存（cache）+ Android 绘制（render）** 四坨。按类批量 java2rust 会：

- 把 TypoSnapshot 的 2104 行竞技场和 LineLayouter 的断行缠在一次 PR 里；
- 无法在「还没有 Canvas」时验收排版；
- 把 Android `ITextView` 拖进内核。

Parse 的成功经验是：**feature = 管线上一段可验收的 I/O**，不是一个 Java 包。layout 沿用：

| feature | I/O（输入 → 输出） | 类只是实现细节 |
|---------|-------------------|----------------|
| wr-01 | 数字/几何 → Typo 结构可查询 | TypoPage/TextLine/RangeBase |
| wr-02 | 节属性/常量 → ViewEnv 内容区 | ViewEnv, PageSetup |
| wr-03 | 字符串+样式 → 宽度/字形度量 | ITypeface, FontStyleFactory |
| wr-04 | 度量后的 run + 行宽 → 行列表 | LineLayouter |
| wr-05 | 行列表 + 页高 → 页列表 | TextLayouter.flow / finishPage |
| wr-06 | 页流 → 可提交的 Snapshot | TypoDocument.commit |
| wr-07 | Snapshot → RenderCommand[] | IRenderDrawer 语义，无 Canvas |
| wr-08 | commands → 屏幕像素 | Flutter，不进 word-core 热路径 |

**切在 I/O，不切在类。** 禁止「一个 feature = 整个 `layout/base` 目录」。

---

## 2. 与 parse 的对应关系

| parse | layout |
|-------|--------|
| `parser/features/wc-XX/` | `layout/features/wr-XX/` |
| `STRUCTURE_ANALYSIS.md` 类图 | 同左 |
| `FUNCTIONS.md` I/O 组合 | 同左 |
| `functions.csv` + `FUNCTION.md` §1.7/§3 | 同左 |
| 测试镜像 `crate/tests/<relpath>` | 同左，crate 仍是 `libs/word-core`（wr-08 Flutter 另仓/另包） |
| `todo!("DEFERRED_NON_FUNCTION")` + 布局校验 | 同左 |
| 禁止工程 stub 假装绿 | 同左 |

前缀 **wr-** = writer render/layout，避免和 wc- 解析序号撞车。

---

## 3. 依赖序（就绪才编号）

```text
parse wc-05 ──┐
              ├─ wr-01 Typo 结构（无算法，可并行 parse 收口）
              └─ wr-02 ViewEnv（无算法，可并行）
                    ↓
                 wr-03 度量（依赖 font crate；不依赖完整 Typo 算法）
                    ↓
                 wr-04 断行（依赖 wr-03 宽度）
                    ↓
                 wr-05 分页（依赖 wr-04 行）
                    ↓
                 wr-06 快照组装（依赖 wr-01 结构 + wr-05 页流）
                    ↓
                 wr-07 RenderCommand（依赖 wr-06 Snapshot）
                    ↓
                 wr-08 Flutter（依赖 wr-07 指令；可先用 fixture 指令画 Hello）
```

**行级叶子波（wr-09..16，function 已 pass）** 见 [LINE_PARITY_PLAN.md](./LINE_PARITY_PLAN.md)。  
**现行热路径波（wr-21..24）** 见 [state-java-l2-parity/NEXT_PLAN.md](../state-java-l2-parity/NEXT_PLAN.md)。切点仍是 I/O。禁止把 Java LineLayouter 塞回 wr-04 greedy wrap。

**允许并行：** wr-01 ∥ wr-02 ∥ wr-07 指令 enum 草案 ∥ wr-08 用手工 fixture 指令画一页。  
**不允许：** wr-04 在 wr-03 度量未绿时用假宽度假装断行正确（可用固定宽度 fixture，但 FUNCTION.md 必须写明上游是 fixture 不是 font）。

---

## 4. Function 切分规则（I/O）

每个 function 必须：

1. **一个对外入口**（构造 / `flow` / `measure` / `emit`），输入输出可 JSON 化。
2. **测 N 只用声明的上游输出或 fixture**，不跑完整 `DocxReader.read()`（wr-08 编排 function 除外）。
3. **覆盖矩阵**映射公开入口、分支、边界、失败、cleanup。
4. **§3.6 cargo test + §3.7 llvm-cov 本 function src + §3.8 模板布局 PASS**。

**不要**按 Java 内部 private helper 切 function（例如 `setHigh2Bytes` 不是 function）。  
**不要**把 `CoreThread` 做成 function——MVP 删除该 I/O（改为同步），在 STRUCTURE 里标 defer。

---

## 5. must-write / stub / defer（每个 STRUCTURE_ANALYSIS 必写）

| 标记 | 含义 |
|------|------|
| **must-write** | 本 feature 验收依赖；完整转写，禁止空壳 |
| **stub** | 保留 Java 交错模板 + 签名，`todo!("DEFERRED_NON_FUNCTION")` 停车；调用方不得依赖其语义 |
| **defer** | 本 feature 不建文件；写进「禁止本 function 去改」 |

例：wr-01 must-write `TypoTextLine` 几何字段；stub `TypoSnapshot.alloc(int)` 竞技场；defer `TypoDrawing`。

---

## 6. 跨平台特殊规则（parse 没有的）

1. **禁止**在 word-core 增加 `android::*` / `flutter_engine` 依赖。
2. Java `IRenderDrawer` 的实现类 `RenderDrawer` **不进 wr-07 种子**；进的是 **指令枚举 + 从 Typo 遍历 emit**。
3. 度量唯一字形源：`libs/core-libs/thirdparty/font`（FreeType）。禁止 `Paint.measureText`。
4. 坐标 twip；zoom/density 只在 wr-08 或 ViewEnv 入参出现一次。
5. Java `TypoSnapshot` 指针模型：**语义对齐，布局不对齐**。Stage2 JSON 比内存 dump。

---

## 7. 每个 feature 交付物清单（与 wc-05 相同）

```text
layout/features/<feature-id>/
  STRUCTURE_ANALYSIS.md     # 类图、流程、must/stub/defer、Java↔Rust 表
  FUNCTIONS.md              # I/O 组合、热路径、编译面
  functions.csv             # order, function_id, …, status
  functions/<id>/FUNCTION.md
  docs/                     # 可选深读
```

Loop 完成才把 `functions.csv` 的 status 改为 `pass`。Split 阶段全是 `spec`。
