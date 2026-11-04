# wr-22-text-flow

> status: **pass** · M13 · Java `TextLayouter.java` `flow` / `doFlowLine` / `preFlow` / `postFlow`

把产品从 `LayoutDocument` 手写循环接到 **Java 正文热路径**。table / textFrame 分支 `todo!("DEFERRED_NON_FUNCTION")`（wr-27/28）。

## I/O

| I | LayoutState（src 队列、cp、ParaEnv） |
| O | 本页 TypoTextLine 流；finishPage / complete |

禁止调用 wr-04 `Wrap.rs`。
