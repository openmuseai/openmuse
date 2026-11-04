# pp-13-document-assembly — 文档级装配

把 pp-12 的 package、pp-06 的 presentation rId、pp-07 的 part reader、pp-08 的阅读顺序、
pp-02 的结构树以及 pl-02/03 的几何/继承结果装配为整份可分页文档。该 feature 不增强文字效果，
终点是 `PptxDocument { slides[] }`，下游仅为 pr-04 多页协议。
