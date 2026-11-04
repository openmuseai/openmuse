# wr-05-page-flow — 分页

Java：`TextLayouter.flow`（L190-208）+ `finishPage` + `PageViewLayouter.flowToPage`。

```text
preFlow
while !finishPage && !srcEmpty:
    flowLine   # wr-04 已给出下一行高度
    if y + lineHeight > content_height: finishPage
postFlow
```

must：页高耗尽分页；源耗尽 complete；分页符 `\f`（若 rope 含该 char）。  
stub：寡妇孤儿 processPagination。  
defer：分节换页细节、分栏、表跨页、IPageGrid。
