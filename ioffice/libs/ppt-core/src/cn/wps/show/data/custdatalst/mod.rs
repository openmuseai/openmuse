// M-PPT-KMOSLIDE SP1.0（1.0.2）：`cn.wps.show.data.custdatalst` 原先整个目录只有自动生成桩，
// 且本 `mod.rs` 为空 ⇒ 两个文件在默认构建下都不可达（假编译面形态 C）。
// 现按 Java 真身重写并登记：
//   · `CustDataLst.java`（386 行，含静态内嵌类 `TagLst`）
//   · `Tag.java`（216 行）
// owner: show.data.custdatalst    wave: M-PPT-KMOSLIDE-SP1
pub mod CustDataLst;
pub mod Tag;
