# S09 — 断行 wrap（run 序列 → 行）

## 1. 职责

把段内 run 序列按内容区宽度折成若干行：每行 = run 子区间（start_cp/end_cp）+ 累积宽 ≤ 内容宽；空格断点回退；超宽单词强制截断；`\r` 段末断行。

管线位置：上游 S07（内容宽）+ S08（run 宽）；下游 S10（行几何）。

## 2. 核心 Java 代码

| 类 | 路径 | 行数 | 职责 |
|----|------|------|------|
| `LineLayouter` | `layout/base/line/LineLayouter.java` | 1029 | run 放置、换行、对齐、装饰收集 |
| `TextLayouter.flow → flowLine` | `layout/base/TextLayouter.java` | 1380 | 驱动单行排版 |

TextLayouter.flow 钉死（Java L190–208）：

```text
preFlow(state)                 # PageLayouter.start，记 mStartCP
while !state.finishPage && !isComplete(state):
    flowLine(state)            # ParaLayouter.start + LineLayouter
postFlow(state)                # collectPageRange，PageLayouter.end
```

`isComplete`：`layoutState.srcList.isEmpty()`（源 CP 耗尽）。

折行伪逻辑（LineLayouter 语义，MVP 空格断行）：

```text
对段内 run 序列累积宽度：
  累积宽 ≤ 内容宽 → 吸收字符
  遇到空格 → 记断点（回退点）
  超过内容宽：
    有断点 → 回退到断点，行尾=断点前
    无断点（超宽单词）→ 强制截断到内容宽
  '\r' → 段末强制断行
```

## 3. 数据模型

| 模型 | 语义 |
|------|------|
| 行 | run 子区间：`start_cp/end_cp` + 累积宽（twip）+ 组成 run 序列 |
| `LayoutState.srcList` | 待排版源 CP 列表（耗尽即 complete） |
| `TypoTextLine.flags` | FIRST/LAST of para/page/section（分页信号，S10/S11 使用） |

## 4. 输入 / 输出

| 项 | 内容 |
|----|------|
| **I** | 段内 run 序列 + 每 run 宽度（S08）+ 内容区宽（S07） |
| **O** | 行集合：每行 = run 子区间 + 累积宽 ≤ 内容宽 |

## 5. 上下游

- **上游**：S07 内容宽、S08 run 宽度。
- **下游**：S10 行几何（行高/基线/对齐）、S11 分页。

## 6. 边界 / 失败语义 / 不变量

- MVP **空格断行**；两端对齐 / CJK 标点挤压 / bidi **延后**（后续 rustybuzz）。
- 超宽单词必须强制截断，不能无限溢出。
- 行宽 ≤ 内容宽是硬约束；验收以「每行 start/end cp 与行宽（twip）」进 Stage2。

## 7. Rust 承接与现状

wr-04：`01_run_width_cache` + `02_wrap`（layout/base/text/）—— **✗ 待 loop**。