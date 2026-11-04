# ppt-core

PPT 内核改写。MVP **三层**（对齐 writer wc + wr）：

| 层 | 目录 | 前缀 |
|---|---|---|
| Parse | [parser/features](./parser/features/README.md) | `pp-*` |
| Layout | [layout](./layout/README.md) | `pl-*` |
| Render | [render](./render/README.md) | `pr-*` |

总览与 Java 绘制热路径：[MVP.md](./MVP.md)
