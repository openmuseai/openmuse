# wc-10-subdocument-pipeline

> status: **pass**（01–12 全部 pass；产品 parse→layout 经 `open_text_document` / `&dyn Document`）

**I：** `TextDocument._create` 已能 `new SubDocument(parent, type)`（wc-02 09/10）。  
**O：** `SubDocument` 去掉就地编译影子，字段与 `Document` trait 指向 **同名真实模块**；layout 经 `&dyn Document` 读到与 Java 同一套 getter。

切分与顺序：[FUNCTIONS.md](./FUNCTIONS.md) · [functions.csv](./functions.csv)

不重做：rope/CHPX/PAPX/CPieceTable/type0 `PLCSection`（wc-02 `09`）。
