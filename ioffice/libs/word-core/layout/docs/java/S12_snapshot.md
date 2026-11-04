# S12 — 快照组装：TypoDocument / TypoSnapshot（版本 + CPRef）

## 1. 职责

把 S11 的页序列组装成只读快照：`TypographyDocument.newSnapshot → 排版写入 → commitSnapshot`，`TypoSnapshot` 维护页列表、CPRef 注册表、版本；`ISnapshotListener` 通知。渲染/命中只读源；编辑翻译（updatePageRanges）维护 cp。

管线位置：上游 S11；下游 S13（定位）、S14（渲染）。

## 2. 核心 Java 代码

| 类 | 行数 | 职责 |
|----|------|------|
| `TypoDocument` | 402 | 当前 snapshot、listener、commitSnapshot |
| `TypoSnapshot` | 2104 | 版本、页列表、CPRef 表、fork/refcount |
| `TypoRangeBase` | 129 | `startCP=cpRef+Δstart`，`endCP=cpRef+Δend`，半开 `[start,end)` |
| `TypoPage` | 738 | 边距、行表、documentType、flags |
| `TypoLayoutPage` | 1029 | 主文档页 |
| `TypoTextLine` | 658 | 行框、ascent/descent、graph 下标、flags |
| `TypoPageLinesIterator` | 221 | 页内行游标 |

Snapshot 生命周期（钉死）：

```text
newSnapshot() / fork
  → layout 写入 forked snapshot
  → beforeCommit(version)
  → commitSnapshot：替换 current，通知 ISnapshotListener
  → 读者 getSnapshot()（clone+refcount）→ 用完 release()
```

## 3. 数据模型

| 模型 | 语义 |
|------|------|
| `TypoSnapshot` | 页集合 + CPRef 注册表 + 版本（VER_FIRSTCACHE→VALIDBEGIN） |
| CPRef | cp 引用（相对 Δstart/Δend），宽度优化；语义 = 绝对 cp |
| `TypoPage/TypoLayoutPage` | 页几何 + 行表 |
| `TypoTextLine` | 行框 + flags（FIRST/LAST para/page/section） |

CP 模型（钉死）：

- 全文线性 CP，与 rope/CHPX/PAPX 同一套。
- 行通常没有 RangeBase 槽，由 `TypoPara` graph index `[OFFSET_GRAPH_INDEX_START, END]` 反查。MVP 可在行上直接存 `start_cp/end_cp`，Stage2 JSON 用同一字段名。

## 4. 输入 / 输出

| 项 | 内容 |
|----|------|
| **I** | 页序列（S11）+ 文本引用（cp） |
| **O** | **TypoSnapshot**（页集合 + CPRef 注册表 + 版本）→ 渲染/命中只读源 |

## 5. 上下游

- **上游**：S11 页序列。
- **下游**：S13 定位、S14 渲染指令化。

## 6. 边界 / 失败语义 / 不变量

- MVP 单线程、无 fork：`layout()` 直接替换 `current: TypoSnapshot`；listener 用 `Fn()` 回调。
- Java arena `int pointer` 内存块**不复刻**；Rust owned struct，但对外契约（getStartCP/getEndCP、页序、行几何、flag）与 Java 对齐。
- 半开区间 `[start, end)`；`contains(cp)`: `cp>=start && cp<end`。

## 7. Rust 承接与现状

wr-01（`01_typo_snapshot`..`05_typo_run`，cache/ 已有 .rs，**typo_page pass**）+ wr-06（`01_typo_document` `02_snapshot_build` 待 loop）—— **◐ wr-01 部分 pass**。Stage2 JSON（页序/行 start_cp/end_cp/x/y/w/h ±0.01px）。