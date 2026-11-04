# wr-06-typo-build — 快照组装 + 同步入口

Java：`TypoDocument.commitSnapshot` + `LayoutManager` + `FlowLayout`。  
MVP：**无 CoreThread**。`layout_document(doc, view_env) -> TypoSnapshot`。

must：把 wr-05 页流写入 wr-01 结构并 bump version。  
stub：ISnapshotListener 列表（可 Vec<fn>）。  
defer：LayoutService 全 API、Reusable 池、FirstCache。
