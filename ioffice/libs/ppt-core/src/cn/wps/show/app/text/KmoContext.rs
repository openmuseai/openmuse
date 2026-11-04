#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unused_mut)]

// Java: package cn.wps.show.app.text
// Java: import cn.wps.drawing.data.table.TableStyleTextStyle
// rust: no-stmt —— `cn.wps.drawing.data.table` 在默认构建下被 `#[cfg(feature = "full")]` 关
//   （`libs/ppt-core/src/cn/wps/drawing/data/mod.rs`），Rust 侧无法命名 `TableStyleTextStyle`；
//   两个字段（`tableCellStyle` / `tableTextStyle`）与四个访问器整体 DEFERRED（见下方注记）。
//   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
// Java: import cn.wps.show.app.KmoPresentation
// rust: 偏差登记 —— Java 返回 `cn.wps.show.app.KmoPresentation`；`show/app/mod.rs` 里该模块归 SP4 波
//   正在落地，且 `ISlide` 接口（`show/app/slide/ISlide.rs`）的 `getPresentation()` 已规定返回
//   `ISlide` 文件内承载的 `KmoPresentation`（同一 Java 类的接口宿主）。本波与接口保持一致，
//   返回 `&ISlide::KmoPresentation`，接线波统一到真 `KmoPresentation` 时收敛。
//   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3
// Java: import cn.wps.show.app.slide.ISlide
use crate::cn::wps::show::app::slide::ISlide::{ISlide, KmoPresentation};
use std::rc::Rc;

// Java: public class KmoContext
pub struct KmoContext {
    // Java field: ISlide objContext = null
    //
    // rust: 偏差登记 —— Java 持有接口引用 `ISlide objContext`（`KmoSlide implements ISlide`，
    //   `KmoSlide.java:30`）；Rust 侧具体类 `cn.wps.show.app.slide.KmoSlide` 在默认构建下仍是
    //   `#[cfg(any())]` 外壳，无法命名。故按 Java 接口 `Rc<dyn ISlide>` 承载（与 `KmoFilm.parent`
    //   的 `*const dyn ISlide`、SP4 `KmoSlideBeautifyTags.mSlide` 的 `Weak<RefCell<dyn ISlide>>`
    //   同一策略）。此处 `KmoContext` 由 `KmoTextOperator` 短暂持有、不构成反指闭环，故用 `Rc`
    //   而非 `Weak`。接线波用真 `KmoSlide` 收窄。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3
    pub(crate) objContext: Option<Rc<dyn ISlide>>,
    // Java field: TableStyleTextStyle tableCellStyle = null
    //
    // DEFERRED_NON_FUNCTION: `cn.wps.drawing.data.table.TableStyleTextStyle` 默认构建下不可命名
    //   （见文件头 import 注记）。该字段被 `KmoTextOperator.mergeTextStyleInTable()`
    //   （`KmoTextOperator.java:751-791`）读取 `hasFontStyles()/fontStyles()/hasColor()/color()/
    //   hasBold()/bold()/hasItalic()/italic()`，同样整体 DEFERRED（见该文件）。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
    // Java field: TableStyleTextStyle tableTextStyle = null
    //
    // DEFERRED_NON_FUNCTION: 同上。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
    // Java field: short anchor = 0
    pub(crate) anchor: i16,
    // Java field: boolean anchorCtr = false
    pub(crate) anchorCtr: bool,
    // Java field: int slideNumber = 1
    pub(crate) slideNumber: i32,
}

// Java: public class KmoContext
impl KmoContext {
    // Java: public KmoContext(ISlide obj)
    //
    // rust: 偏差登记 —— 本波同时提供 `new()` 与 Java 同名式 `KmoContext()` 两个入口（后者转调前者）：
    //   · `new()` 是仓库既有构造器口径（`KmoFont::new` 等），且**可达调用点已存在** ——
    //     `show/app/KmoPresentation.rs:1452`（SP4）写作 `KmoContext::new(slide.clone())`；
    //   · `KmoContext()` 保留 Java 构造器名，便于逐行对照。
    //   两处 `// Java:` 计一次。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3
    pub fn new(obj: Rc<dyn ISlide>) -> Self {
      // Java: this.setContext(obj);
      let mut inst = KmoContext {
          // Java field initializer: ISlide objContext = null
          objContext: None,
          // Java field initializer: TableStyleTextStyle tableCellStyle = null（DEFERRED，见字段注释）
          // Java field initializer: TableStyleTextStyle tableTextStyle = null（DEFERRED，见字段注释）
          // Java field initializer: short anchor = 0
          anchor: 0,
          // Java field initializer: boolean anchorCtr = false
          anchorCtr: false,
          // Java field initializer: int slideNumber = 1
          slideNumber: 1,
      };
      inst.setContext(obj);
      inst
    }

    // Java: public KmoContext(ISlide obj)
    pub fn KmoContext(obj: Rc<dyn ISlide>) -> Self {
      // Java: this.setContext(obj);（Java 构造器体，与 `new()` 同体）
      Self::new(obj)
    }

    // Java: public void setContext(ISlide obj)
    pub fn setContext(&mut self, obj: Rc<dyn ISlide>) {
      // Java: this.objContext = obj;
      self.objContext = Some(obj);
    }

    // Java: public ISlide getContext()
    //
    // rust: 偏差登记 —— Java 返回 `ISlide` 引用；Rust 侧返回共享句柄 `Option<Rc<dyn ISlide>>`
    //   （`None` ≙ Java `null`），与字段同源偏差。
    pub fn getContext(&self) -> Option<Rc<dyn ISlide>> {
      // Java: return this.objContext;
      self.objContext.clone()
    }

    // Java: public void setTableCellTextStyle(TableStyleTextStyle obj)
    // DEFERRED_NON_FUNCTION: 形参类型 `TableStyleTextStyle` 默认构建下不可命名 ⇒ 方法**签名本身**
    //   无法表达。Java 体：`this.tableCellStyle = obj;`（`KmoContext.java:27-29`）。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: public void setAnchor(short anchor)
    pub fn setAnchor(&mut self, anchor: i16) {
      // Java: this.anchor = anchor;
      self.anchor = anchor;
    }

    // Java: public short getAnchor()
    pub fn getAnchor(&self) -> i16 {
      // Java: return this.anchor;
      self.anchor
    }

    // Java: public void setAnchorCtr(boolean anchorCtr)
    pub fn setAnchorCtr(&mut self, anchorCtr: bool) {
      // Java: this.anchorCtr = anchorCtr;
      self.anchorCtr = anchorCtr;
    }

    // Java: public boolean isAnchorCtr()
    pub fn isAnchorCtr(&self) -> bool {
      // Java: return this.anchorCtr;
      self.anchorCtr
    }

    // Java: public TableStyleTextStyle getTableCellTextStyle()
    // DEFERRED_NON_FUNCTION: 返回类型 `TableStyleTextStyle` 默认构建下不可命名 ⇒ 方法**签名本身**
    //   无法表达。Java 体：`return this.tableCellStyle;`（`KmoContext.java:47-49`）。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: public void setWholeTableTextStyle(TableStyleTextStyle obj)
    // DEFERRED_NON_FUNCTION: 形参类型不可命名。Java 体：`this.tableTextStyle = obj;`
    //   （`KmoContext.java:51-53`）。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: public TableStyleTextStyle getWholeTableTextStyle()
    // DEFERRED_NON_FUNCTION: 返回类型不可命名。Java 体：`return this.tableTextStyle;`
    //   （`KmoContext.java:55-57`）。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: public KmoPresentation getPresentation()
    pub fn getPresentation(&self) -> Option<&KmoPresentation> {
      // Java: return null == this.objContext ? null : this.objContext.getPresentation();
      self.objContext.as_deref().map(|objContext| objContext.getPresentation())
    }

    // Java: public void setSlideNumber(int number)
    pub fn setSlideNumber(&mut self, number: i32) {
      // Java: this.slideNumber = number;
      self.slideNumber = number;
    }

    // Java: public int getSlideNumber()
    pub fn getSlideNumber(&self) -> i32 {
      // Java: return this.slideNumber;
      self.slideNumber
    }
}
