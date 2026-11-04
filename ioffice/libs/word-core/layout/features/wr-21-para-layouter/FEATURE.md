# wr-21-para-layouter

> status: **pass** · M12 · Java `ParaLayouter.java`

PAPX → 段级排版环境：段前/段后（217/214 + auto 219/216=280）、行距 220、缩进 208–213、jc 193。行框塌缩公式已在 `LineBox`：`max(0, paraBefore − prevParaAfter)`（对标 Java `Math.max(0, cur.paraBefore - pre.paraAfter)`），禁止改成 `max(before, after)`。产品 `para_papx_from` 的简化公式在本 feature 被替换。

## I/O

| I | 当前/上一/下一段 PropertySet + 是否段首行 |
| O | ParaEnv：alignment、indents、before/after、lineSpacing |

**must：** 正文纯文本规则。**defer：** AutoNum 串（wc-08）、kinsoku 行为（wr-26，只读标志）。
