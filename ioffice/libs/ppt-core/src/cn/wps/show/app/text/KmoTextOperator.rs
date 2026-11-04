#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unused_mut)]

// Java: package cn.wps.show.app.text
// Java: import cn.wps.drawing.DG
// rust: no-stmt —— 仅 `isBeautifyClearProp()` 使用（`DG.TAG_UNIT_TYPE.isPicture/isDecorative`），
//   该方法整体 DEFERRED（见下）。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
// Java: import cn.wps.drawing.app.DgStore
// Java: import cn.wps.drawing.app.KmoProtocol
// rust: no-stmt —— 规范路径 `cn.wps.drawing.app.KmoProtocol` 在默认构建下被
//   `#[cfg(feature = "full")]` 关（`drawing/app/mod.rs`）⇒ 本类 `implements KmoProtocol` 的
//   接口实现面 DEFERRED（见文件末注记）。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
// Java: import cn.wps.drawing.app.graphic.GraphicObject
// Java: import cn.wps.drawing.app.shape.DgShape
// rust: no-stmt —— 规范路径 `cn.wps.drawing.app.shape.DgShape` 在默认构建下不可达（`drawing/app/shape`
//   只登记 `DgGroup`/`dg_group_hosts`）⇒ 字段 `dgShape` 与 `save()` 的 `Shape.open(store.query(id))`
//   面 DEFERRED。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
// Java: import cn.wps.drawing.app.shape.DgShapeStyle
// java: import cn.wps.drawing.app.table.KmoTable
// rust: no-stmt —— 仅作空宿主可达（`app/shape/kmo_show_shape_hosts.rs`），无 `selectedCellRange()`/
//   `getCell()` ⇒ `init()` 的表格分支与 `KmoCellTextOperator` 的表遍历 DEFERRED。
//   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
// Java: import cn.wps.drawing.app.text.KmoTextBody
// rust: 偏差登记 —— Java `cn.wps.drawing.app.text.KmoTextBody` 默认构建下 `#[cfg(any())]`，且 Rust 侧
//   5748 行里 1709 处 `TODO_RUST_CODE`（不可编译）。本类**文本体面**（`txBody` 字段 + 全部经它转发的
//   文本读写/格式化成员）整体 DEFERRED。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
// Java: import cn.wps.drawing.app.util.KmoRange
// Java: import cn.wps.drawing.data.basic.IndexRange
// rust: no-stmt —— **SP3b 阻塞项（实测，R9 口径）**：`IndexRange` 的真移植在
//   `drawing/data/basic/IndexRange.rs`，但被 `drawing/data/basic/mod.rs:3` 的
//   `#[cfg(feature = "full")]` 关住，而 `full` 不可用。实测「只 un-gate 该行」会引入 10 个
//   E0423（该文件把 `KoZone` 当值用，默认构建可达的 `KoZone` 却是 `&'static KoZone` 引用版）
//   ⇒ `IndexRange` 在默认构建下不可命名 ⇒ `range(int)` 与 `getParagraphRange(int,int)` 的内体
//   一并 DEFERRED（Java 体原文已保留）。修复动作见 `KmoTextHyphen.rs` 文件头注记。
//   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
// Java: import cn.wps.drawing.data.color.Color
use crate::cn::wps::drawing::data::color::Color::Color;
// Java: import cn.wps.drawing.data.fill.Fill
use crate::cn::wps::drawing::data::fill::Fill::Fill;
// Java: import cn.wps.drawing.data.shape.Shape
// Java: import cn.wps.drawing.data.style.FontCollection
// Java: import cn.wps.drawing.data.style.FontRef
// Java: import cn.wps.drawing.data.style.FontScheme
// Java: import cn.wps.drawing.data.table.TableStyleTextStyle
// Java: import cn.wps.drawing.data.table.ThemeableFontStyles
// rust: no-stmt —— `FontCollection`/`FontRef`/`FontScheme`/`ThemeableFontStyles`/`TableStyleTextStyle`
//   默认构建下不可达 ⇒ `mergeTextStyleInTable()`/`mergefontformat()` 两个族 DEFERRED。
//   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
// Java: import cn.wps.drawing.data.text.TextBody
// Java: import cn.wps.drawing.data.text.TextBodyProperties
// Java: import cn.wps.drawing.data.text.TextCharacterProperties
// Java: import cn.wps.drawing.data.text.TextListStyle
// Java: import cn.wps.drawing.data.text.TextParagraph
// Java: import cn.wps.drawing.data.text.TextParagraphProperties
// rust: no-stmt —— `cn.wps.drawing.data.text.*` 真身在 `#[cfg(feature = "full")]` 后（`full` 不可用）
//   ⇒ 样式/属性面整体 DEFERRED。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
// Java: import cn.wps.kfc.numfmt.util.DateUtil
// rust: no-stmt —— `DateUtil` 默认构建下整类缺失 ⇒ `resetDateFieldTextRuns()` 的 Excel 日期换算
//   DEFERRED（不擅自用 `std::time` 近似，避免口径漂移）。
//   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
// Java: import cn.wps.kso.kproto.KoAr
use crate::cn::wps::kso::kproto::KoAr::KoAr;
// Java: import cn.wps.kso.kproto.KoBin
use crate::cn::wps::kso::kproto::KoBin::KoBin as KoBinTrait;
// Java: import cn.wps.show.app.selection.ClearPropInfo
// rust: no-stmt —— `cn.wps.show.app.selection.ClearPropInfo` 默认构建下整类缺失 ⇒
//   `restoreClearProp(ClearPropInfo)` 形参类型不可命名，方法 DEFERRED。
//   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
// Java: import cn.wps.show.app.shape.KmoShowShape
use crate::cn::wps::show::app::shape::KmoShowShape::KmoShowShape;
// Java: import cn.wps.show.app.shape.PlaceHolderMerger
// rust: no-stmt —— Rust 侧位于 `show/layout/PlaceHolderMerger.rs`，但其入参绑定
//   `util/layout_shape::KmoShowShape` 影子，与本类的 `show/app/shape` 真身不同型
//   ⇒ `textBodyProperties()` 的合并分支 DEFERRED。
//   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
// Java: import cn.wps.show.app.slide.ISlide
// Java: import cn.wps.show.app.slide.KmoSlide
// Java: import cn.wps.show.app.slide.KmoSlideLayout
// Java: import cn.wps.show.app.slide.KmoSlideMaster
// rust: no-stmt —— 具体类默认构建下不可命名 ⇒ `getSlide()`/`getSlideLayout()`/`getSlideMaster()`/
//   `findMaster()` 与依其建立的 `textListStyle*()` 族 DEFERRED。
//   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
// Java: import cn.wps.show.app.tag.KmoShapeBeautifyTags
// Java: import cn.wps.show.app.util.TagUtil
// rust: no-stmt —— 两者默认构建下不可达 ⇒ `isBeautifyClearProp()`/`getClearPropType()` DEFERRED。
//   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
// Java: import cn.wps.show.data.custdatalst.CustDataLst
// rust: no-stmt —— 内嵌类型 `CustDataLst.TagLst` 不可命名 ⇒ `deleteClearProp(int)` 返回类型无法表达。
//   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
// Java: import cn.wps.show.data.shape.ShowShape
use crate::cn::wps::show::data::shape::ShowShape::ShowShape;
// Java: import java.util.ArrayList
// Java: import java.util.Date
// rust: no-stmt —— `resetDateFieldTextRuns()` 的 `new Date()` 随 `DateUtil` 一并 DEFERRED。
// Java: import java.util.List
// rust: no-stmt —— `List<Object>` 的取值映射见 `data()`/`insert()` 注记。
// Java: （同包）cn.wps.show.app.text.KmoContext
use super::KmoContext::KmoContext;
// Java: （同包）cn.wps.show.app.text.KmoPargraphRange
use super::KmoPargraphRange::KmoPargraphRange;
use std::cell::RefCell;
use std::rc::Rc;

// Java field: private ThreadLocal<Fill> solidFillThreadLocal = new ThreadLocal()
//
// rust: 偏差登记 —— Java `ThreadLocal<Fill>` 在同一线程内保存**同一** `Fill` 对象引用；
//   Rust `thread_local!` 的借用无法带出闭包，故改为 `Rc<RefCell<Fill>>` 承载「同一线程内同一对象」
//   的同一性语义（`solidFill()` 返回该 `Rc`）。
//   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3
thread_local! {
    static SOLID_FILL: RefCell<Option<Rc<RefCell<Fill>>>> = const { RefCell::new(None) };
}

// Java: public class KmoTextOperator implements KmoProtocol
//
// 【SP3 落地范围】字段、两个可命名构造路径、`hasClearProp()`/`phType()`/`phIndex()`/`shape()`/
//   `setContext()`/`save()` 的委派链、`getParagraphRange()` 全文、`fontClr()`/`setFontClr()`/
//   `solidFill()`、`needToMeasureCache()` 前三段为真实实现；其余按上游缺失逐项 DEFERRED
//   （均带 owner/wave，含 Java 体原文，供 SP3b 直接接回）。
// owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3
pub struct KmoTextOperator {
    // Java field: private Integer typePh = -1
    pub(crate) typePh: i32,
    // Java field: private Integer indexPh = -1
    pub(crate) indexPh: i32,
    // Java field: private DgShape dgShape = null
    //
    // DEFERRED_NON_FUNCTION: `cn.wps.drawing.app.shape.DgShape` 默认构建下不可达（见 import 注记）
    //   ⇒ 字段类型无法命名；`getClearPropType()`/`restoreClearProp()`/`deleteClearProp()`/
    //   `init()` 的 else 分支/`measureFontColorCache()`/`resetTextWrappingTypeWhenAutoFit()`
    //   均依赖它。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
    // Java field: private KmoShowShape shape = null
    pub(crate) shape: Option<KmoShowShape>,
    // Java field: private KmoContext context = null
    pub(crate) context: Option<KmoContext>,
    // Java field: private KmoTextBody txBody = null
    //
    // DEFERRED_NON_FUNCTION: `cn.wps.drawing.app.text.KmoTextBody` 默认构建下不可命名（见 import 注记）
    //   ⇒ 字段类型无法命名。`textBody()`/`setTextbody()` 及全部文本读写成员依赖它。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
    // Java field: private ThreadLocal<Fill> solidFillThreadLocal = new ThreadLocal()
    // rust: 已提为模块级 `SOLID_FILL`（见上方注记）
    // Java field: private Boolean isMeasureCache = false
    pub(crate) isMeasureCache: bool,
    // Java field: private Boolean isMeasureFontColorCache = false
    pub(crate) isMeasureFontColorCache: bool,
    // Java field: private Color fontClrLocal = null
    pub(crate) fontClrLocal: Option<Color>,
    // Java field: private TextListStyle textListStyleShapeLocal = null
    //
    // DEFERRED_NON_FUNCTION: `TextListStyle` 默认构建下不可命名 ⇒ 字段类型无法命名；连带
    //   `textListStyleShape()`/`setTextListStyleShape()`/`textListStyleLayout()`/
    //   `textListStyleMaster()`/`textListStyleMasterBody()`/`textListStyleDefault()`/
    //   `mergeformat()`/`followProp()`/`mergeListStyle()`/`measureShapeListStyle()`/
    //   `needToChangeFontColor()` 整体 DEFERRED。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
    // Java field: private TextListStyle textListStyleInShape = null
    //
    // DEFERRED_NON_FUNCTION: 同上。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
}

// Java: public class KmoTextOperator implements KmoProtocol
impl Default for KmoTextOperator {
    // Java: KmoTextOperator()（隐式；Java 侧三个 public 构造器最终都落到 `this.init(..)` + 字段赋值）
    fn default() -> Self {
        KmoTextOperator {
            // Java field initializer: private Integer typePh = -1
            typePh: -1,
            // Java field initializer: private Integer indexPh = -1
            indexPh: -1,
            // Java field initializer: private DgShape dgShape = null（DEFERRED，见字段注记）
            // Java field initializer: private KmoShowShape shape = null
            shape: None,
            // Java field initializer: private KmoContext context = null
            context: None,
            // Java field initializer: private KmoTextBody txBody = null（DEFERRED，见字段注记）
            // Java field initializer: private Boolean isMeasureCache = false
            isMeasureCache: false,
            // Java field initializer: private Boolean isMeasureFontColorCache = false
            isMeasureFontColorCache: false,
            // Java field initializer: private Color fontClrLocal = null
            fontClrLocal: None,
            // Java field initializer: private TextListStyle textListStyleShapeLocal = null（DEFERRED）
            // Java field initializer: private TextListStyle textListStyleInShape = null（DEFERRED）
        }
    }
}

// Java: public class KmoTextOperator implements KmoProtocol
impl KmoTextOperator {
    // Java: public KmoTextOperator(KmoShowShape shape)
    pub fn new__kmoshowshape(shape: KmoShowShape) -> Self {
      // Java: this.init(shape);
      let mut inst = Self::default();
      inst.init(&shape);
      // Java: this.shape = shape;
      inst.shape = Some(shape);
      inst
    }

    // Java: public KmoTextOperator(int typePh, int idxPh, KmoTextBody textBody)
    // DEFERRED_NON_FUNCTION: 形参类型 `KmoTextBody` 默认构建下不可命名 ⇒ 构造器**签名本身**无法表达。
    //   Java 体：`this.typePh = typePh; this.indexPh = idxPh; this.setTextbody(textBody);`
    //   （`KmoTextOperator.java:64-68`）。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: public KmoTextOperator(int typePh, int idxPh, KmoTextBody textBody, KmoShowShape table)
    // DEFERRED_NON_FUNCTION: 同上，且末句 `this.shape = table;` 之外的 `setTextbody` 面不可达。
    //   Java 体见 `KmoTextOperator.java:70-75`。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: public int getClearPropType()
    pub fn getClearPropType(&self) -> i32 {
      // Java: if (this.dgShape == null) { return 0; }
      // Java: else
      // Java: {
      // Java:    int ret = 0;
      // Java:    if (this.dgShape.hasDescr() && this.dgShape.descr() != null && this.dgShape.descr().contains("#clear"))
      // Java:    {
      // Java:       ret = 2;
      // Java:    }
      // Java:    if (this.isBeautifyClearProp(this.shape))
      // Java:    {
      // Java:       if (2 == ret) { ret = 3; } else { ret = 1; }
      // Java:    }
      // Java:    return ret;
      // Java: }
      // DEFERRED_NON_FUNCTION: `dgShape`（`cn.wps.drawing.app.shape.DgShape`）字段类型不可命名，
      //   且 `isBeautifyClearProp()` 依赖缺失的 `KmoShapeBeautifyTags`/`TagUtil`。
      //   Java 体见 `KmoTextOperator.java:77-96`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: private boolean isBeautifyClearProp(KmoShowShape shape)
    //
    // rust: Java `private` → Rust 侧放开为 `pub`（登记），便于 SP3b 与验收单测直达。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3
    pub fn isBeautifyClearProp(&self, shape: &KmoShowShape) -> bool {
      // Java: if (!shape.hasCustDataLst()) { return false; }
      // Java: else
      // Java: {
      // Java:    CustDataLst custDataLst = shape.custDataLst();
      // Java:    if (!custDataLst.hasTags()) { return false; }
      // Java:    else
      // Java:    {
      // Java:       KmoShapeBeautifyTags shapeTags = new KmoShapeBeautifyTags(shape);
      // Java:       if (shapeTags.hasBeautifyTags() && !shapeTags.simulateTags())
      // Java:       {
      // Java:          int selfType = shapeTags.selfUnitType();
      // Java:          if (!DG.TAG_UNIT_TYPE.isPicture(selfType) && !DG.TAG_UNIT_TYPE.isDecorative(selfType))
      // Java:          {
      // Java:             KmoTextOperator textOperator = shape.getTextOperator();
      // Java:             if (textOperator != null && textOperator.length() > 1)
      // Java:             {
      // Java:                String txt = textOperator.text(0, Integer.MAX_VALUE);
      // Java:                String presetTxt = shapeTags.presetText();
      // Java:                return TagUtil.strEqualIgnoreSpaceChar(txt, presetTxt);
      // Java:             }
      // Java:             else { return false; }
      // Java:          }
      // Java:          else { return false; }
      // Java:       }
      // Java:       else { return false; }
      // Java:    }
      // Java: }
      // DEFERRED_NON_FUNCTION: `KmoShapeBeautifyTags`（741 行）/`TagUtil`/`DG.TAG_UNIT_TYPE` 默认构建下
      //   均不可达。Java 体见 `KmoTextOperator.java:98-126`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public boolean hasClearProp()
    pub fn hasClearProp(&self) -> bool {
      // Java: return 0 != this.getClearPropType();
      //
      // rust: `getClearPropType()` 当前 DEFERRED（见其注记）；本句为真实直译，SP3b 接回后即刻生效。
      0 != self.getClearPropType()
    }

    // Java: public void restoreClearProp(ClearPropInfo info)
    // DEFERRED_NON_FUNCTION: 形参类型 `cn.wps.show.app.selection.ClearPropInfo` 默认构建下整类缺失
    //   ⇒ 方法**签名本身**无法表达。Java 体（`KmoTextOperator.java:132-140`）：
    //   `if (this.dgShape != null && !this.hasClearProp()) {
    //       if (2 == info.mType || 3 == info.mType) { String descr = this.dgShape.descr();
    //          this.dgShape.setDescr(descr + "clear#"); } }`
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: public CustDataLst.TagLst deleteClearProp(int type)
    // DEFERRED_NON_FUNCTION: 返回类型 `CustDataLst.TagLst` 默认构建下不可命名 ⇒ 方法**签名本身**
    //   无法表达。Java 体（`KmoTextOperator.java:142-153`）：`if (this.dgShape == null) return null;
    //   if (2 == type || 3 == type) { String descr = this.dgShape.descr();
    //   this.dgShape.setDescr(descr.replace("#clear", "")); } return null;`
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: private void init(KmoShowShape shape)
    //
    // rust: Java `private` → Rust 侧放开为 `pub`（登记）。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3
    pub fn init(&mut self, shape: &KmoShowShape) {
      // Java: if (shape.isPlaceHolder())
      if shape.isPlaceHolder() {
         // Java: ShowShape.Placeholder ph = shape.ph();
         // Java: this.indexPh = ph.idx();
         // Java: this.typePh = ph.type();
         // DEFERRED_NON_FUNCTION: `KmoShowShape.ph()`（`KmoShowShape.rs:1153`）自身为
         //   DEFERRED_NON_FUNCTION，其返回类型是 `kmo_show_shape_hosts::Placeholder`（空宿主，
         //   无 `idx()`/`type()`）⇒ 两句取占位符类型/序号的赋值无法表达。
         //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      }
      // Java: if (shape.type() == 4)
      if shape.r#type() == 4 {
         // Java: GraphicObject graphicObject = shape.graphic();
         // Java: if (1 == graphicObject.type())
         // Java: {
         // Java:    KmoTable table = (KmoTable)shape.graphic();
         // Java:    KmoRange seletedCell = table.selectedCellRange();
         // Java:    KmoTextBody txBody = null;
         // Java:    if (null != seletedCell) { txBody = table.getCell(seletedCell.topRow, seletedCell.leftCol).getTextBody(); }
         // Java:    this.setTextbody(txBody);
         // Java:    this.typePh = 10;
         // Java: }
         // DEFERRED_NON_FUNCTION: `KmoTable`/`KmoRange` 默认构建下仅空宿主/不可达，
         //   且 `setTextbody()` 面（`KmoTextBody`）不可命名。Java 体见 `KmoTextOperator.java:162-174`。
         //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      // Java: else
      } else {
         // Java: this.dgShape = shape.getDgShape();
         // Java: TextBody textBody = this.dgShape.textBody();
         // Java: if (null != textBody) { this.updateProtocol(textBody.save()); }
         // Java: else { this.updateProtocol(TextBody.create().save()); }
         // DEFERRED_NON_FUNCTION: `DgShape` 不可命名（字段注记），`KmoShowShape.getDgShape()`
         //   （`KmoShowShape.rs:4714`）自身亦为 DEFERRED_NON_FUNCTION；`updateProtocol()` 面随
         //   `KmoTextBody` 不可命名。Java 体见 `KmoTextOperator.java:175-183`。
         //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      }
    }

    // Java: public KoBin protocol()
    //
    // rust: Java `KoBin` 在 Rust 侧是 trait（`ko/kproto/KoBin.rs`），返回 `Option<Box<dyn KoBin>>`
    //   （`None` ≙ Java `null`）。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3
    pub fn protocol(&self) -> Option<Box<dyn KoBinTrait>> {
      // Java: KmoTextBody txBody = this.textBody();
      // Java: return null == txBody ? null : txBody.protocol();
      // DEFERRED_NON_FUNCTION: `textBody()` 面不可命名。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public void updateProtocol(KoBin data)
    pub fn updateProtocol(&mut self, data: &dyn KoBinTrait) {
      // Java: KmoTextBody txBody = new KmoTextBody();
      // Java: txBody.updateProtocol(data);
      // Java: this.setTextbody(txBody);
      // DEFERRED_NON_FUNCTION: `KmoTextBody` 不可命名。Java 体见 `KmoTextOperator.java:192-196`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public void setContext(KmoContext context)
    pub fn setContext(&mut self, context: KmoContext) {
      // Java: this.context = context;
      self.context = Some(context);
    }

    // Java: public KmoTextBody getTextBody()
    // DEFERRED_NON_FUNCTION: 返回类型 `KmoTextBody` 默认构建下不可命名 ⇒ 方法**签名本身**无法表达。
    //   Java 体：`return this.textBody();`（`KmoTextOperator.java:202-204`）。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: public boolean hasParagraphList()
    pub fn hasParagraphList(&self) -> bool {
      // Java: return null != this.txBody ? this.txBody.hasParagraphList() : false;
      // DEFERRED_NON_FUNCTION: `txBody` 字段/类型不可命名。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public int cp(int type)
    pub fn cp(&self, r#type: i32) -> i32 {
      // Java: KmoTextBody txBody = this.textBody();
      // Java: return null == txBody ? 0 : txBody.cp(type);
      // DEFERRED_NON_FUNCTION: `txBody` 面不可命名。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public int length()
    pub fn length(&self) -> i32 {
      // Java: KmoTextBody txBody = this.textBody();
      // Java: return null == txBody ? 0 : txBody.length();
      // DEFERRED_NON_FUNCTION: `txBody` 面不可命名。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public int phType()
    pub fn phType(&self) -> i32 {
      // Java: return this.typePh;
      self.typePh
    }

    // Java: public int phIndex()
    pub fn phIndex(&self) -> i32 {
      // Java: return this.indexPh;
      self.indexPh
    }

    // Java: public List<TextListStyle> textListStyleFollows()
    // DEFERRED_NON_FUNCTION: 元素类型 `TextListStyle` 默认构建下不可命名 ⇒ 方法**签名本身**无法表达。
    //   Java 体见 `KmoTextOperator.java:228-268`（逐级加入 shape → layout → masterBody → master
    //   或 default 的样式快照）。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: public boolean insert(int param1, String param2)
    pub fn insert__int_string(&mut self, param1: i32, param2: String) -> bool {
      // Java: KmoTextBody txBody = this.textBody();
      // Java: return txBody != null && txBody.insert(param1, param2);
      // DEFERRED_NON_FUNCTION: `txBody` 面不可命名。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public boolean insert(int cp, List<Object> dataList)
    //
    // rust: 偏差登记 —— Java `List<Object>` 在 Rust 侧按仓库既有口径映射为
    //   `Vec<Box<dyn std::any::Any + Send + Sync>>`（见 `ConvertTool.rs`）。
    pub fn insert__int_list_boolean(&mut self, cp: i32, dataList: Vec<Box<dyn std::any::Any + Send + Sync>>, bCovertToRunList: bool) -> bool {
      // Java: KmoTextBody txBody = this.textBody();
      // Java: if (null == txBody) { return false; }
      // Java: else
      // Java: {
      // Java:    boolean bCovertToRunList = false;
      // Java:    if (2 == this.phType() || 0 == this.phType()) { bCovertToRunList = true; }
      // Java:    return txBody.insert(cp, dataList, bCovertToRunList);
      // Java: }
      // DEFERRED_NON_FUNCTION: `txBody` 面不可命名。注意 Java 第二个重载是
      //   `insert(int cp, List<Object> dataList)`（两参），内部再算 `bCovertToRunList`；
      //   Rust 侧因 `KmoTextBody.insert(cp, list, bool)` 已是三参形态，故本方法保留三参形态并在
      //   接线时由调用方按 `phType()` 预计算（SP3b 决定是否保留两参重载）。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public boolean delete(int start, int end)
    pub fn delete(&mut self, start: i32, end: i32) -> bool {
      // Java: KmoTextBody txBody = this.textBody();
      // Java: return null == txBody ? false : txBody.delete(start, end);
      // DEFERRED_NON_FUNCTION: `txBody` 面不可命名。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public boolean format(int cp, int len, KoAr format, int mode)
    pub fn format__int_int_koar_int(&mut self, cp: i32, len: i32, format: Box<dyn KoAr>, mode: i32) -> bool {
      // Java: KmoTextBody txBody = this.textBody();
      // Java: return null == txBody ? false : txBody.format(cp, len, format, mode);
      // DEFERRED_NON_FUNCTION: `txBody` 面不可命名。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public boolean format(KoAr format, int mode)
    pub fn format__koar_int(&mut self, format: Box<dyn KoAr>, mode: i32) -> bool {
      // Java: KmoTextBody txBody = this.textBody();
      // Java: return null == txBody ? false : txBody.format(format, mode);
      // DEFERRED_NON_FUNCTION: `txBody` 面不可命名。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public boolean seek(int cp, boolean isExpandEnd)
    //
    // rust: `KmoTextHyphen::checkHyphen()` 的真实调用点（本波已实现，见 `KmoTextHyphen.rs`）。
    pub fn seek(&mut self, cp: i32, isExpandEnd: bool) -> bool {
      // Java: KmoTextBody txBody = this.textBody();
      // Java: return null == txBody ? false : txBody.seek(cp, isExpandEnd);
      // DEFERRED_NON_FUNCTION: `txBody` 面不可命名。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public boolean prev(int type, boolean isParagraphLimited)
    pub fn prev(&mut self, r#type: i32, isParagraphLimited: bool) -> bool {
      // Java: KmoTextBody txBody = this.textBody();
      // Java: return null == txBody ? false : txBody.prev(type, isParagraphLimited);
      // DEFERRED_NON_FUNCTION: `txBody` 面不可命名。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public boolean next(int type, boolean isParagraphLimited)
    pub fn next(&mut self, r#type: i32, isParagraphLimited: bool) -> bool {
      // Java: KmoTextBody txBody = this.textBody();
      // Java: return null == txBody ? false : txBody.next(type, isParagraphLimited);
      // DEFERRED_NON_FUNCTION: `txBody` 面不可命名。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public boolean hasText()
    pub fn hasText(&self) -> bool {
      // Java: KmoTextBody txBody = this.textBody();
      // Java: return null == txBody ? false : txBody.hasText();
      // DEFERRED_NON_FUNCTION: `txBody` 面不可命名。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public String text(int type)
    pub fn text__int(&self, r#type: i32) -> String {
      // Java: KmoTextBody txBody = this.textBody();
      // Java: return null == txBody ? "" : txBody.text(type);
      // DEFERRED_NON_FUNCTION: `txBody` 面不可命名。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public String text(int start, int end)
    pub fn text__int_int(&self, start: i32, end: i32) -> String {
      // Java: KmoTextBody txBody = this.textBody();
      // Java: return null == txBody ? "" : txBody.text(start, end);
      // DEFERRED_NON_FUNCTION: `txBody` 面不可命名。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public TextCharacterProperties characterProperties()
    // DEFERRED_NON_FUNCTION: 返回类型 `TextCharacterProperties` 默认构建下不可命名 ⇒ 方法**签名本身**
    //   无法表达。Java 体：`KmoTextBody txBody = this.textBody();
    //   return null == txBody ? TextCharacterProperties.create() : txBody.characterProperties();`
    //   （`KmoTextOperator.java:334-337`）。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: public List<Object> data(int start, int end)
    //
    // rust: `List<Object>` 取值映射同 `insert()` 注记。
    pub fn data(&self, start: i32, end: i32) -> Vec<Box<dyn std::any::Any + Send + Sync>> {
      // Java: KmoTextBody txBody = this.textBody();
      // Java: return (List<Object>)(null == txBody ? new ArrayList() : txBody.data(start, end));
      // DEFERRED_NON_FUNCTION: `txBody` 面不可命名。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public TextParagraph paragraph()
    // DEFERRED_NON_FUNCTION: 返回类型 `TextParagraph` 默认构建下不可命名 ⇒ 方法**签名本身**无法表达。
    //   Java 体：`KmoTextBody txBody = this.textBody();
    //   return null == txBody ? TextParagraph.create() : txBody.paragraph();`
    //   （`KmoTextOperator.java:344-347`）。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: public IndexRange range(int type)
    //
    // DEFERRED_NON_FUNCTION: 返回类型 `cn.wps.drawing.data.basic.IndexRange` 默认构建下不可命名
    //   （见文件头 import 注记）⇒ 方法**签名本身**无法表达。Java 体
    //   （`KmoTextOperator.java:349-352`）：`KmoTextBody txBody = this.textBody();
    //    return null == txBody ? IndexRange.create() : txBody.range(type);`
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: public KoAr format(int type, boolean isMerge)
    //
    // rust: Java 返回 `KoAr`（属性对象，按 `type` 是 `TextCharacterProperties` 或
    //   `TextParagraphProperties`）；Rust 侧以 `Option<Box<dyn KoAr>>` 承载（`None` ≙ Java null）。
    pub fn format__int_boolean(&mut self, r#type: i32, isMerge: bool) -> Option<Box<dyn KoAr>> {
      // Java: KmoTextBody txBody = this.textBody();
      // Java: if (null == txBody)
      // Java: {
      // Java:    return (KoAr)(1 == type ? TextCharacterProperties.create() : TextParagraphProperties.create());
      // Java: }
      // Java: else
      // Java: {
      // Java:    KoAr prop = txBody.format(type);
      // Java:    boolean isNeedToChangeFontColor = false;
      // Java:    if (isMerge && null != this.context)
      // Java:    {
      // Java:       if (this.needToMeasureCache()) { this.measureTextBodyCache(this.context); }
      // Java:       isNeedToChangeFontColor = this.needToChangeFontColor(type, prop);
      // Java:       this.mergeformat(type, this.outlineLevel(), prop);
      // Java:    }
      // Java:    if (isNeedToChangeFontColor)
      // Java:    {
      // Java:       Fill fill = Fill.create();
      // Java:       fill.putType(0);
      // Java:       fill.writeSolidFill(Color.open(this.fontClr().save()));
      // Java:       fill.save();
      // Java:       ((TextCharacterProperties)prop).writeFill(fill);
      // Java:    }
      // Java:    return prop;
      // Java: }
      // DEFERRED_NON_FUNCTION: `txBody`/`TextCharacterProperties`/`TextParagraphProperties`/
      //   `TextListStyle` 均不可命名；`measureTextBodyCache()`/`needToChangeFontColor()`/
      //   `mergeformat()`/`outlineLevel()` 随之 DEFERRED。`Fill`/`Color` 两句本身可达，接线时可直接接回。
      //   Java 体见 `KmoTextOperator.java:354-380`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public KoAr formatWithOutPara(int type, int outlineLvl, boolean isMerge)
    pub fn formatWithOutPara(&mut self, r#type: i32, outlineLvl: i32, isMerge: bool) -> Option<Box<dyn KoAr>> {
      // Java: KmoTextBody txBody = this.textBody();
      // Java: if (null == txBody)
      // Java: {
      // Java:    return (KoAr)(1 == type ? TextCharacterProperties.create() : TextParagraphProperties.create());
      // Java: }
      // Java: else
      // Java: {
      // Java:    KoAr prop;
      // Java:    if (1 == type) { prop = TextCharacterProperties.create(); } else { prop = TextParagraphProperties.create(); }
      // Java:    boolean isNeedToChangeFontColor = false;
      // Java:    if (isMerge && null != this.context)
      // Java:    {
      // Java:       if (this.needToMeasureCache()) { this.measureTextBodyCache(this.context); }
      // Java:       isNeedToChangeFontColor = this.needToChangeFontColor(type, prop);
      // Java:       this.mergeformat(type, outlineLvl, prop, false);
      // Java:    }
      // Java:    if (isNeedToChangeFontColor)
      // Java:    {
      // Java:       Fill fill = Fill.create();
      // Java:       fill.putType(0);
      // Java:       fill.writeSolidFill(Color.open(this.fontClr().save()));
      // Java:       fill.save();
      // Java:       ((TextCharacterProperties)prop).writeFill(fill);
      // Java:    }
      // Java:    return prop;
      // Java: }
      // DEFERRED_NON_FUNCTION: 同 `format(int, boolean)`。Java 体见 `KmoTextOperator.java:382-414`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public TextBodyProperties textBodyProperties(boolean isMerge)
    // DEFERRED_NON_FUNCTION: 返回类型 `TextBodyProperties` 默认构建下不可命名 ⇒ 方法**签名本身**
    //   无法表达。Java 体（`KmoTextOperator.java:416-425`）：
    //   `if (null == this.shape) return TextBodyProperties.create();
    //    else if (isMerge && -1 != this.typePh) return PlaceHolderMerger.mergeTextBodyPr(this.shape);
    //    else { KmoTextBody txBody = this.textBody();
    //           return null != txBody ? txBody.textBodyProperties() : TextBodyProperties.create(); }`
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: public boolean putAnchor(int anchor)
    pub fn putAnchor(&mut self, anchor: i32) -> bool {
      // Java: TextBodyProperties bodyProperties = this.textBodyProperties(false);
      // Java: bodyProperties.putAnchor(anchor);
      // Java: return this.format(bodyProperties, 0);
      // DEFERRED_NON_FUNCTION: `textBodyProperties()` 面不可命名。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public int getAnchor(KmoShowShape shape)
    pub fn getAnchor(&mut self, shape: Option<&KmoShowShape>) -> i32 {
      // Java: TextBodyProperties bodyProperties = this.textBodyProperties(true);
      // Java: return !bodyProperties.hasAnchor() ? 0 : bodyProperties.anchor();
      // DEFERRED_NON_FUNCTION: `textBodyProperties()` 面不可命名。注意 Java 该重载虽接收
      //   `KmoShowShape shape`，体内并未使用该形参（保持原样直译判据）。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public boolean putAnchorCenter(boolean isAnchorCtr)
    pub fn putAnchorCenter(&mut self, isAnchorCtr: bool) -> bool {
      // Java: TextBodyProperties bodyProperties = this.textBodyProperties(false);
      // Java: if (isAnchorCtr) { bodyProperties.putAnchorCenter(true); }
      // Java: else { bodyProperties.deleteAnchorCenter(); }
      // Java: return this.format(bodyProperties, 0);
      // DEFERRED_NON_FUNCTION: `textBodyProperties()` 面不可命名。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public Boolean isAnchorCenter(KmoShowShape shape)
    pub fn isAnchorCenter(&mut self, shape: Option<&KmoShowShape>) -> bool {
      // Java: if (null != shape && shape.type() == 1)
      // Java: {
      // Java:    TextBodyProperties bodyProperties = this.textBodyProperties(false);
      // Java:    if (null != bodyProperties && bodyProperties.hasAnchorCenter())
      // Java:    {
      // Java:       return bodyProperties.isAnchorCenter();
      // Java:    }
      // Java: }
      // Java: return false;
      // DEFERRED_NON_FUNCTION: `textBodyProperties()` 面不可命名。Java 返回 `Boolean`（装箱），
      //   但恒非 null（末句 `return false;`），Rust 侧按 `bool` 直译。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public int textDirection(KmoShowShape shape)
    pub fn textDirection(&mut self, shape: Option<&KmoShowShape>) -> i32 {
      // Java: if (null != shape && shape.type() == 1)
      // Java: {
      // Java:    TextBodyProperties bodyProperties = this.textBodyProperties(true);
      // Java:    if (bodyProperties.hasVerticalText()) { return bodyProperties.verticalText(); }
      // Java: }
      // Java: return 0;
      // DEFERRED_NON_FUNCTION: `textBodyProperties()` 面不可命名。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public boolean putTextDirection(int textDirection)
    pub fn putTextDirection(&mut self, textDirection: i32) -> bool {
      // Java: if (null != this.shape && this.shape.type() == 1)
      // Java: {
      // Java:    TextBodyProperties bodyProperties = this.textBodyProperties(false);
      // Java:    bodyProperties.putVerticalText(textDirection);
      // Java:    return this.format(bodyProperties, 0);
      // Java: }
      // Java: else { return true; }
      // DEFERRED_NON_FUNCTION: `textBodyProperties()` 面不可命名。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public KmoShowShape shape()
    pub fn shape(&self) -> Option<&KmoShowShape> {
      // Java: return this.shape;
      self.shape.as_ref()
    }

    // Java: public boolean save(KmoShowShape shape)
    pub fn save(&mut self, shape: Option<&KmoShowShape>) -> bool {
      // Java: return this.save(shape, true);
      self.save__kmoshowshape_boolean(shape, true)
    }

    // Java: public boolean saveWithoutDirty(KmoShowShape shape)
    pub fn saveWithoutDirty(&mut self, shape: Option<&KmoShowShape>) -> bool {
      // Java: return this.save(shape, false);
      self.save__kmoshowshape_boolean(shape, false)
    }

    // Java: private boolean save(KmoShowShape shape, boolean isSetDirty)
    //
    // rust: Java `private` → Rust 侧放开为 `pub`（登记），便于 SP3b 与验收单测直达。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3
    pub fn save__kmoshowshape_boolean(&mut self, shape: Option<&KmoShowShape>, isSetDirty: bool) -> bool {
      // Java: if (null == shape) { return false; }
      // Java: else
      // Java: {
      // Java:    if (shape.type() == 1)
      // Java:    {
      // Java:       DgStore store = shape.parent().getPresentation().drawingStore();
      // Java:       Shape sp = Shape.open(store.query(shape.id()));
      // Java:       KmoTextBody txBody = this.textBody();
      // Java:       if (null != txBody)
      // Java:       {
      // Java:          sp.textShape().writeTextBody(TextBody.open(txBody.protocol()));
      // Java:          store.update(shape.id(), sp.save());
      // Java:       }
      // Java:    }
      // Java:    if (isSetDirty) { shape.parent().getPresentation().setDirty(); }
      // Java:    shape.parent().eventListeners().onShapeContentsChanged(3, shape);
      // Java:    shape.parent().eventListeners().onSlideContentsChanged();
      // Java:    return true;
      // Java: }
      // DEFERRED_NON_FUNCTION: 依赖 `DgStore`/`Shape`（`cn.wps.drawing.data.shape`）/
      //   `KmoTextBody.protocol()` 与 `ISlide` 接口尚缺的 `eventListeners()`/`setDirty()`/
      //   `drawingStore()` 面；`shape.parent()` 只给出 `*const dyn ISlide`。
      //   Java 体见 `KmoTextOperator.java:493-515`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public KmoPargraphRange getParagraphRange(int start, int end)
    pub fn getParagraphRange(&mut self, start: i32, end: i32) -> KmoPargraphRange {
      // Java: KmoPargraphRange pargraphRange = new KmoPargraphRange();
      // Java: int idxParagraph = 0;
      // Java: int startParagraph = 0;
      // Java: int endParagraph = 0;
      // Java: this.seek(0, false);
      // Java: do
      // Java: {
      // Java:    IndexRange range = this.range(0);
      // Java:    startParagraph = range.start();
      // Java:    endParagraph = range.end();
      // Java:    if (start < endParagraph) { break; }
      // Java:    ++idxParagraph;
      // Java: }
      // Java: while(this.next(0, false));
      // Java: pargraphRange.setStartIndex(idxParagraph);
      // Java: pargraphRange.setStartOffset(Math.max(0, start - startParagraph));
      // Java: if (endParagraph < end)
      // Java: {
      // Java:    while(this.next(0, false))
      // Java:    {
      // Java:       ++idxParagraph;
      // Java:       IndexRange range = this.range(0);
      // Java:       startParagraph = range.start();
      // Java:       endParagraph = range.end();
      // Java:       if (end <= endParagraph) { break; }
      // Java:    }
      // Java: }
      // Java: pargraphRange.setEndIndex(idxParagraph);
      // Java: pargraphRange.setEndOffset(Math.max(0, end - startParagraph + 1));
      // Java: return pargraphRange;
      // DEFERRED_NON_FUNCTION: 内体的 `this.range(0)` 返回 `IndexRange`（默认构建下不可命名），
      //   `this.seek()`/`this.next()` 亦随 `KmoTextBody` DEFERRED。返回类型
      //   `KmoPargraphRange` 本波已完整落地，故本方法保留签名、只停车内体；
      //   `IndexRange` 一落地即可按上方 Java 体逐句接回（`std::cmp::max` 对应 `Math.max`）。
      //   Java 体见 `KmoTextOperator.java:517-552`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      let _ = (start, end);
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: private void measureTextBodyCache(KmoContext context)
    pub fn measureTextBodyCache(&mut self, context: &KmoContext) {
      // Java: this.measureShapeListStyle();
      // Java: if (null != this.shape)
      // Java: {
      // Java:    ISlide slide = this.shape.parent();
      // Java:    if (null != slide)
      // Java:    {
      // Java:       switch (slide.type())
      // Java:       {
      // Java:          case 0: { KmoSlide slideTmp = (KmoSlide)slide;
      // Java:                    slideTmp.textStyleManager().meansureTextStyle(this.phType(), this.phIndex(), this.shape); break; }
      // Java:          case 1: { KmoSlideLayout slideTmp = (KmoSlideLayout)slide;
      // Java:                    slideTmp.textStyleManager().meansureTextStyle(this.phType(), this.phIndex(), this.shape); break; }
      // Java:          case 2: { KmoSlideMaster slideTmp = (KmoSlideMaster)slide;
      // Java:                    slideTmp.textStyleManager().meansureTextStyle(this.phType(), this.phIndex(), this.shape); }
      // Java:       }
      // Java:    }
      // Java: }
      // Java: else if (null != context)
      // Java: {
      // Java:    ISlide slide = context.getContext();
      // Java:    if (null != slide)
      // Java:    {
      // Java:       switch (slide.type())
      // Java:       {
      // Java:          case 0: { KmoSlide slideTmp = (KmoSlide)slide;
      // Java:                    slideTmp.textStyleManager().meansureTextStyle(this.phType(), this.phIndex(), context); break; }
      // Java:          case 1: { KmoSlideLayout slideTmp = (KmoSlideLayout)slide;
      // Java:                    slideTmp.textStyleManager().meansureTextStyle(this.phType(), this.phIndex(), context); break; }
      // Java:          case 2: { KmoSlideMaster slideTmp = (KmoSlideMaster)slide;
      // Java:                    slideTmp.textStyleManager().meansureTextStyle(this.phType(), this.phIndex(), context); }
      // Java:       }
      // Java:    }
      // Java: }
      // Java: if (!this.isMeasureFontColorCache) { this.measureFontColorCache(); }
      // Java: this.isMeasureCache = true;
      // DEFERRED_NON_FUNCTION: 三处 `(KmoSlide|KmoSlideLayout|KmoSlideMaster)slide` 具体类强转
      //   在默认构建下不可命名（且其 `textStyleManager()` 未接线）；`measureShapeListStyle()` 随
      //   `TextListStyle` 不可命名。Java 体见 `KmoTextOperator.java:554-603`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: private void mergeformat(int type, int lvl, KoAr format)
    pub fn mergeformat__int_int_koar(&mut self, r#type: i32, lvl: i32, format: Box<dyn KoAr>) {
      // Java: this.mergeformat(type, lvl, format, true);
      self.mergeformat__int_int_koar_boolean(r#type, lvl, format, true)
    }

    // Java: private void mergeformat(int type, int lvl, KoAr format, boolean followDefRpr)
    pub fn mergeformat__int_int_koar_boolean(&mut self, r#type: i32, lvl: i32, format: Box<dyn KoAr>, followDefRpr: bool) {
      // Java: int typePh = this.phType();
      // Java: KoAr mergeformat = format;
      // Java: if (1 == type && followDefRpr)
      // Java: {
      // Java:    TextParagraphProperties paraProp = (TextParagraphProperties)this.txBody.format(0);
      // Java:    if (paraProp.hasDefRPr()) { mergeformat = paraProp.defRPr();
      // Java:       ((TextCharacterProperties)format).follow((TextCharacterProperties)mergeformat); }
      // Java: }
      // Java: TextListStyle textListStyleShape = this.textListStyleShape();
      // Java: if (null != textListStyleShape) { mergeformat = this.mergeListStyle(type, lvl, textListStyleShape, mergeformat); }
      // Java: else
      // Java: {
      // Java:    KoAr followProp = this.followProp(type, lvl);
      // Java:    if (null != followProp)
      // Java:    {
      // Java:       if (0 == type) { ((TextParagraphProperties)mergeformat).follow((TextParagraphProperties)followProp); }
      // Java:       else { ((TextCharacterProperties)mergeformat).follow((TextCharacterProperties)followProp); }
      // Java:       if (10 == typePh && 1 == type) { this.mergeTextStyleInTable(type, lvl, mergeformat); }
      // Java:       return;
      // Java:    }
      // Java: }
      // Java: if (0 <= typePh)
      // Java: {
      // Java:    TextListStyle textListStyleLayout = this.textListStyleLayout();
      // Java:    if (null != textListStyleLayout) { mergeformat = this.mergeListStyle(type, lvl, textListStyleLayout, mergeformat); }
      // Java:    TextListStyle textListStyleMasterBody = this.textListStyleMasterBody();
      // Java:    if (null != textListStyleMasterBody) { mergeformat = this.mergeListStyle(type, lvl, textListStyleMasterBody, mergeformat); }
      // Java:    TextListStyle textListStyleMaster = this.textListStyleMaster();
      // Java:    if (null != textListStyleMaster) { this.mergeListStyle(type, lvl, textListStyleMaster, mergeformat); }
      // Java: }
      // Java: else
      // Java: {
      // Java:    TextListStyle textListStyleDefault = this.textListStyleDefault();
      // Java:    if (null != textListStyleDefault) { this.mergeListStyle(type, lvl, textListStyleDefault, mergeformat); }
      // Java: }
      // Java: if (10 == typePh && 1 == type) { this.mergeTextStyleInTable(type, lvl, format); }
      // DEFERRED_NON_FUNCTION: `TextListStyle`/`TextParagraphProperties`/`TextCharacterProperties`
      //   不可命名（`txBody`、`textListStyleShape()`、`mergeListStyle()`、`followProp()` 随之）。
      //   Java 体见 `KmoTextOperator.java:609-666`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: private KoAr followProp(int type, int lvl)
    pub fn followProp__int_int(&mut self, r#type: i32, lvl: i32) -> Option<Box<dyn KoAr>> {
      // Java: KoAr followProp = null;
      // Java: TextListStyle textListStyleShape = this.textListStyleShape();
      // Java: if (null != textListStyleShape) { followProp = this.followProp(type, lvl, textListStyleShape); }
      // Java: if (null != followProp) { return followProp; }
      // Java: else
      // Java: {
      // Java:    int typePh = this.phType();
      // Java:    if (0 <= typePh)
      // Java:    {
      // Java:       TextListStyle textListStyleLayout = this.textListStyleLayout();
      // Java:       if (null != textListStyleLayout) { followProp = this.followProp(type, lvl, textListStyleLayout); }
      // Java:       TextListStyle textListStyleMasterBody = this.textListStyleMasterBody();
      // Java:       if (null == followProp && null != textListStyleMasterBody) { followProp = this.followProp(type, lvl, textListStyleMasterBody); }
      // Java:       TextListStyle textListStyleMaster = this.textListStyleMaster();
      // Java:       if (null == followProp && null != textListStyleMaster) { followProp = this.followProp(type, lvl, textListStyleMaster); }
      // Java:    }
      // Java:    else
      // Java:    {
      // Java:       TextListStyle textListStyleDefault = this.textListStyleDefault();
      // Java:       if (null != textListStyleDefault) { followProp = this.followProp(type, lvl, textListStyleDefault); }
      // Java:    }
      // Java:    return followProp;
      // Java: }
      // DEFERRED_NON_FUNCTION: `TextListStyle` 不可命名。Java 体见 `KmoTextOperator.java:668-703`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: private KoAr followProp(int type, int lvl, TextListStyle textListStyle)
    // DEFERRED_NON_FUNCTION: 形参类型 `TextListStyle` 不可命名 ⇒ 方法**签名本身**无法表达。
    //   Java 体见 `KmoTextOperator.java:705-720`（取 `getParagraphProperties(lvl, listStyle)`，
    //   判 `hasParent()` 后返回 `paraProp` 或 `paraProp.defRPr()`）。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: private KoAr mergeListStyle(int type, int lvl, TextListStyle textListStyle, KoAr format)
    // DEFERRED_NON_FUNCTION: 形参类型 `TextListStyle` 不可命名 ⇒ 方法**签名本身**无法表达。
    //   Java 体见 `KmoTextOperator.java:722-745`。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: private KoAr mergeTextStyleInTable(int type, int lvl, KoAr format)
    pub fn mergeTextStyleInTable(&mut self, r#type: i32, lvl: i32, format: Box<dyn KoAr>) -> Box<dyn KoAr> {
      // Java: if (null == this.context) { return format; }
      // Java: else
      // Java: {
      // Java:    TableStyleTextStyle cellTextStyle = this.context.getTableCellTextStyle();
      // Java:    TableStyleTextStyle tableTextStyle = this.context.getWholeTableTextStyle();
      // Java:    FontRef fontReferenceCell = null; FontRef fontReferenceWhole = null;
      // Java:    ThemeableFontStyles fontStylesCell = null; ThemeableFontStyles fontStylesWhole = null;
      // Java:    if (null != cellTextStyle && cellTextStyle.hasFontStyles()) { fontStylesCell = cellTextStyle.fontStyles(); }
      // Java:    if (null != fontStylesCell && fontStylesCell.hasFontRef()) { fontReferenceCell = fontStylesCell.fontRef(); }
      // Java:    if (null != tableTextStyle && tableTextStyle.hasFontStyles()) { fontStylesWhole = tableTextStyle.fontStyles(); }
      // Java:    if (null != fontStylesWhole && fontStylesWhole.hasFontRef()) { fontReferenceWhole = fontStylesWhole.fontRef(); }
      // Java:    TextCharacterProperties tmpProp = TextCharacterProperties.open(((TextCharacterProperties)format).save());
      // Java:    if (!tmpProp.hasFill())
      // Java:    {
      // Java:       Color fontcolor = null;
      // Java:       if (null != cellTextStyle && cellTextStyle.hasColor()) { fontcolor = cellTextStyle.color(); }
      // Java:       else if (null != fontReferenceCell && fontReferenceCell.hasColor()) { fontcolor = fontReferenceCell.color(); }
      // Java:       else if (null != tableTextStyle && tableTextStyle.hasColor()) { fontcolor = tableTextStyle.color(); }
      // Java:       else if (null != fontReferenceWhole && fontReferenceWhole.hasColor()) { fontcolor = fontReferenceWhole.color(); }
      // Java:       if (null != fontcolor)
      // Java:       {
      // Java:          ((TextCharacterProperties)format).fill().putType(0);
      // Java:          ((TextCharacterProperties)format).fill().forceWriteSolidFill(fontcolor);
      // Java:       }
      // Java:    }
      // Java:    if (!tmpProp.hasOptions() || !tmpProp.options().hasBold())
      // Java:    {
      // Java:       if (null != cellTextStyle && cellTextStyle.hasBold() && 2 == cellTextStyle.bold())
      // Java:       { ((TextCharacterProperties)format).options().putBold(true); }
      // Java:       else if (null != tableTextStyle && tableTextStyle.hasBold() && 2 == tableTextStyle.bold())
      // Java:       { ((TextCharacterProperties)format).options().putBold(true); }
      // Java:    }
      // Java:    if (!tmpProp.hasOptions() || !tmpProp.options().hasItalics())
      // Java:    {
      // Java:       if (null != cellTextStyle && cellTextStyle.hasItalic() && 2 == cellTextStyle.italic())
      // Java:       { ((TextCharacterProperties)format).options().putItalics(true); }
      // Java:       else if (null != tableTextStyle && tableTextStyle.hasItalic() && 2 == tableTextStyle.italic())
      // Java:       { ((TextCharacterProperties)format).options().putItalics(true); }
      // Java:    }
      // Java:    FontCollection fontCollectionCell = null; FontCollection fontCollectionWhole = null;
      // Java:    if (null != fontStylesCell && fontStylesCell.hasFont()) { fontCollectionCell = fontStylesCell.font(); }
      // Java:    if (null != fontStylesWhole && fontStylesWhole.hasFont()) { fontCollectionWhole = fontStylesWhole.font(); }
      // Java:    if (!tmpProp.hasLatinFont() && !tmpProp.hasEastAsianFont() && !tmpProp.hasComplexScriptFont())
      // Java:    {
      // Java:       if (null != fontCollectionCell) { this.mergefontformat(fontCollectionCell, (TextCharacterProperties)format); }
      // Java:       else if (null != fontReferenceCell) { this.mergefontformat(fontReferenceCell, (TextCharacterProperties)format); }
      // Java:       else if (null != fontCollectionWhole) { this.mergefontformat(fontCollectionWhole, (TextCharacterProperties)format); }
      // Java:       else if (null != fontReferenceWhole) { this.mergefontformat(fontReferenceWhole, (TextCharacterProperties)format); }
      // Java:    }
      // Java:    return format;
      // Java: }
      // DEFERRED_NON_FUNCTION: `TableStyleTextStyle`/`FontRef`/`FontCollection`/`ThemeableFontStyles`/
      //   `TextCharacterProperties` 默认构建下均不可命名，且 `KmoContext` 的对应访问器亦 DEFERRED。
      //   Java 体见 `KmoTextOperator.java:747-832`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      let _ = (r#type, lvl, format);
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: private void measureShapeListStyle()
    pub fn measureShapeListStyle(&mut self) {
      // Java: KmoTextBody txBody = this.textBody();
      // Java: if (txBody.hasListStyle()) { this.setTextListStyleShape(TextListStyle.open(txBody.textListStyle().save())); }
      // DEFERRED_NON_FUNCTION: `txBody`/`TextListStyle` 不可命名。
      //   Java 体见 `KmoTextOperator.java:834-840`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: private KmoSlideMaster findMaster(KmoContext context)
    // DEFERRED_NON_FUNCTION: 返回类型 `KmoSlideMaster` 不可命名 ⇒ 方法**签名本身**无法表达。
    //   Java 体见 `KmoTextOperator.java:842-865`（先 `PlaceHolderMerger.findMaster(this.shape)`，
    //   再按 `objContext.type()` 走 `KmoSlide.layout().master()` / `KmoSlideLayout.master()`）。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: private int outlineLevel()
    pub fn outlineLevel(&self) -> i32 {
      // Java: KmoTextBody txBody = this.textBody();
      // Java: return null != txBody ? ((TextParagraphProperties)txBody.format(0)).lvl() : 0;
      // DEFERRED_NON_FUNCTION: `txBody`/`TextParagraphProperties` 不可命名。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: private TextParagraphProperties getParagraphProperties(int lvl, TextListStyle textListStyle)
    // DEFERRED_NON_FUNCTION: 形参/返回类型均不可命名 ⇒ 方法**签名本身**无法表达。Java 体是一个
    //   `switch (lvl)` 0..8 取 `lvlNpPr()`、`default` 取 `defpPr()` 的九级+默认查表
    //   （`KmoTextOperator.java:872-926`）。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: private void saveTextListStyle(int lvl, TextListStyle textListStyle, TextParagraphProperties properties)
    // DEFERRED_NON_FUNCTION: 形参类型不可命名 ⇒ 方法**签名本身**无法表达。Java 体是 `switch (lvl)`
    //   0..8 写 `writeLvlNpPr(properties)`、`default` 写 `writeDefpPr(properties)`
    //   （`KmoTextOperator.java:928-961`）。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: private void mergefontformat(FontRef fontReference, TextCharacterProperties runformat)
    // DEFERRED_NON_FUNCTION: 形参类型 `FontRef`/`TextCharacterProperties` 不可命名 ⇒ 方法**签名本身**
    //   无法表达。Java 体见 `KmoTextOperator.java:963-980`（经 `findMaster().theme().getFontScheme()`
    //   按 `fontReference.idx()` 取 `major()/minor()` 后转下一重载）。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: private void mergefontformat(FontCollection fontCollection, TextCharacterProperties runformat)
    // DEFERRED_NON_FUNCTION: 形参类型 `FontCollection`/`TextCharacterProperties` 不可命名 ⇒ 方法
    //   **签名本身**无法表达。Java 体见 `KmoTextOperator.java:982-997`（`hasLatin/hasEa/hasCs` 三路
    //   `forceWriteLatinFont/forceWriteEastAsianFont/forceWriteComplexScriptFont`）。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: public boolean resetDateFieldTextRuns(boolean isSave)
    pub fn resetDateFieldTextRuns(&mut self, isSave: bool) -> bool {
      // Java: double day = DateUtil.getExcelDate(new Date(), false);
      // Java: KmoTextBody txBody = this.textBody();
      // Java: if (null == txBody) { return false; }
      // Java: else if (txBody.resetFieldTextRuns(0, day, -1)) { return isSave ? this.save(this.shape, false) : true; }
      // Java: else { return false; }
      // DEFERRED_NON_FUNCTION: `DateUtil`（`cn.wps.kfc.numfmt.util.DateUtil`）默认构建下整类缺失，
      //   `txBody` 面不可命名。Java 体见 `KmoTextOperator.java:999-1009`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      let _ = isSave;
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public boolean resetSlideNumFieldTextRuns(int slideIndex, boolean isSave)
    pub fn resetSlideNumFieldTextRuns(&mut self, slideIndex: i32, isSave: bool) -> bool {
      // Java: KmoTextBody txBody = this.textBody();
      // Java: if (null == txBody) { return false; }
      // Java: else if (txBody.resetFieldTextRuns(1, (double)0.0F, slideIndex + 1)) { return isSave ? this.save(this.shape, false) : true; }
      // Java: else { return false; }
      // DEFERRED_NON_FUNCTION: `txBody` 面不可命名。
      //   Java 体见 `KmoTextOperator.java:1011-1020`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      let _ = (slideIndex, isSave);
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: public boolean resetSlideNumAndDateTimeFileTextRuns(int slideIndex, boolean isSave)
    pub fn resetSlideNumAndDateTimeFileTextRuns(&mut self, slideIndex: i32, isSave: bool) -> bool {
      // Java: double day = DateUtil.getExcelDate(new Date(), false);
      // Java: KmoTextBody txBody = this.textBody();
      // Java: if (null == txBody) { return false; }
      // Java: else { return txBody.resetFieldTextRuns(2, day, slideIndex + 1) && isSave ? this.save(this.shape, false) : false; }
      // DEFERRED_NON_FUNCTION: `DateUtil` 缺失、`txBody` 面不可命名。
      //   Java 体见 `KmoTextOperator.java:1022-1030`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      let _ = (slideIndex, isSave);
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: private KmoTextBody textBody()
    // DEFERRED_NON_FUNCTION: 返回类型 `KmoTextBody` 不可命名 ⇒ 方法**签名本身**无法表达。
    //   Java 体：`return this.txBody;`（`KmoTextOperator.java:1032-1034`）。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: private void setTextbody(KmoTextBody textbody)
    // DEFERRED_NON_FUNCTION: 形参类型 `KmoTextBody` 不可命名 ⇒ 方法**签名本身**无法表达。
    //   Java 体：`if (null != textbody) { textbody.clear(); this.txBody = textbody; }`
    //   （`KmoTextOperator.java:1036-1041`）。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: private Color fontClr()
    pub fn fontClr(&mut self) -> Option<&Color> {
      // Java: if (!this.isMeasureFontColorCache) { this.measureFontColorCache(); }
      if !self.isMeasureFontColorCache {
         // Java: this.measureFontColorCache();
         self.measureFontColorCache();
      }
      // Java: return this.fontClrLocal;
      self.fontClrLocal.as_ref()
    }

    // Java: private void setFontClr(Color fontClr)
    pub fn setFontClr(&mut self, fontClr: Color) {
      // Java: this.fontClrLocal = fontClr;
      self.fontClrLocal = Some(fontClr);
    }

    // Java: private KmoSlide getSlide()
    // DEFERRED_NON_FUNCTION: 返回类型 `KmoSlide` 不可命名 ⇒ 方法**签名本身**无法表达。Java 体见
    //   `KmoTextOperator.java:1055-1071`（先看 `context.getContext().type() == 0`，再看
    //   `shape.parent().type() == 0`，均强转 `(KmoSlide)`）。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: private KmoSlideLayout getSlideLayout()
    // DEFERRED_NON_FUNCTION: 返回类型 `KmoSlideLayout` 不可命名 ⇒ 方法**签名本身**无法表达。
    //   Java 体见 `KmoTextOperator.java:1073-1089`。owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: private KmoSlideMaster getSlideMaster()
    // DEFERRED_NON_FUNCTION: 返回类型 `KmoSlideMaster` 不可命名。Java 体注意末段判据是
    //   `if (1 == slide.type())`（原文如此，非 2），见 `KmoTextOperator.java:1091-1107`。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: private TextListStyle textListStyleShape()
    // DEFERRED_NON_FUNCTION: 返回类型 `TextListStyle` 不可命名 ⇒ 方法**签名本身**无法表达。
    //   Java 体：`return this.textListStyleShapeLocal;`（`KmoTextOperator.java:1109-1111`）。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: private void setTextListStyleShape(TextListStyle textListStyleShape)
    // DEFERRED_NON_FUNCTION: 形参类型不可命名 ⇒ 方法**签名本身**无法表达。Java 体：
    //   `this.textListStyleShapeLocal = textListStyleShape;
    //    this.textListStyleInShape = TextListStyle.create();
    //    this.textListStyleInShape.load(textListStyleShape);`（`KmoTextOperator.java:1113-1117`）。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: private TextListStyle textListStyleLayout()
    // DEFERRED_NON_FUNCTION: 返回类型不可命名，且体内 `slide.textStyleManager().getStyleInLayoutPh(..)`
    //   依赖未接线的 `KmoSlide`。Java 体见 `KmoTextOperator.java:1119-1122`。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: private TextListStyle textListStyleMaster()
    // DEFERRED_NON_FUNCTION: 返回类型不可命名。Java 体见 `KmoTextOperator.java:1124-1137`
    //   （slide → slideLayout → slideMaster 三级回退，各自 `textStyleManager().getStyleInMaster(type)`）。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: private TextListStyle textListStyleMasterBody()
    // DEFERRED_NON_FUNCTION: 返回类型不可命名。Java 体见 `KmoTextOperator.java:1139-1147`。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: private TextListStyle textListStyleDefault()
    // DEFERRED_NON_FUNCTION: 返回类型不可命名。Java 体见 `KmoTextOperator.java:1149-1162`
    //   （slide → slideLayout → slideMaster 三级的 `textStyleManager().defaultStyle()`）。
    //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）

    // Java: private Fill solidFill()
    //
    // rust: 偏差登记 —— 返回 `Rc<RefCell<Fill>>` 以承载 Java `ThreadLocal<Fill>` 的同一性语义
    //   （见模块级 `SOLID_FILL` 注记）。Java 侧本方法在当前文件中无调用点（死私有码），
    //   保留以供 SP3b 接线时按 Java 语义使用。
    pub fn solidFill(&self) -> Rc<RefCell<Fill>> {
      // Java: Fill fill = (Fill)this.solidFillThreadLocal.get();
      // Java: if (null == fill)
      // Java: {
      // Java:    fill = Fill.create();
      // Java:    fill.putType(0);
      // Java:    fill.save();
      // Java:    this.solidFillThreadLocal.set(fill);
      // Java: }
      // Java: return fill;
      SOLID_FILL.with(|cell| {
         let mut slot = cell.borrow_mut();
         if slot.is_none() {
            let mut fill = Fill::create();
            fill.putType(0);
            fill.save();
            *slot = Some(Rc::new(RefCell::new(fill)));
         }
         slot.as_ref().unwrap().clone()
      })
    }

    // Java: private void measureFontColorCache()
    pub fn measureFontColorCache(&mut self) {
      // Java: if (null != this.dgShape)
      // Java: {
      // Java:    if (this.dgShape.hasStyle())
      // Java:    {
      // Java:       DgShapeStyle shapeStyle = this.dgShape.shapeStyle();
      // Java:       if (shapeStyle.hasFontRef() && shapeStyle.fontRef().hasColor())
      // Java:       { this.setFontClr(Color.open(shapeStyle.fontRef().color().save())); }
      // Java:    }
      // Java:    this.isMeasureFontColorCache = true;
      // Java: }
      // DEFERRED_NON_FUNCTION: `dgShape`（`DgShape`）与 `DgShapeStyle` 默认构建下不可命名。
      //   Java 体见 `KmoTextOperator.java:1176-1187`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: private boolean needToChangeFontColor(int type, KoAr kr)
    pub fn needToChangeFontColor(&mut self, r#type: i32, kr: Box<dyn KoAr>) -> bool {
      // Java: if (1 == type && null != this.fontClr())
      // Java: {
      // Java:    boolean isNeedToChangeFontColor = false;
      // Java:    TextCharacterProperties prop = (TextCharacterProperties)kr;
      // Java:    if (!prop.hasFill())
      // Java:    {
      // Java:       if (null == this.textListStyleInShape) { isNeedToChangeFontColor = true; }
      // Java:       else
      // Java:       {
      // Java:          TextParagraphProperties paraPr = this.getParagraphProperties(this.outlineLevel(), this.textListStyleInShape);
      // Java:          if (paraPr != null && paraPr.hasDefRPr())
      // Java:          {
      // Java:             if (paraPr.hasDefRPr() && !paraPr.defRPr().hasFill()) { isNeedToChangeFontColor = true; }
      // Java:          }
      // Java:          else { isNeedToChangeFontColor = true; }
      // Java:       }
      // Java:    }
      // Java:    return isNeedToChangeFontColor;
      // Java: }
      // Java: else { return false; }
      // DEFERRED_NON_FUNCTION: `TextCharacterProperties`/`TextParagraphProperties` 与
      //   `textListStyleInShape` 不可命名。Java 体见 `KmoTextOperator.java:1189-1212`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      let _ = (r#type, kr);
      todo!("DEFERRED_NON_FUNCTION")
    }

    // Java: private boolean needToMeasureCache()
    pub fn needToMeasureCache(&self) -> bool {
      // Java: if (this.context == null)
      if self.context.is_none() {
         // Java: return false;
         return false;
      // Java: else if (!this.isMeasureCache)
      } else if !self.isMeasureCache {
         // Java: return true;
         return true;
      // Java: else
      } else {
         // Java: ISlide slide = this.context.getContext();
         let slide = self.context.as_ref().unwrap().getContext();
         // Java: return slide != null && slide.type() == 0 ? ((KmoSlide)slide).textStyleManager().isCleared() : false;
         if let Some(slide) = slide {
            if slide.r#type() == 0 {
               // DEFERRED_NON_FUNCTION: `(KmoSlide)slide` 具体类强转默认构建下不可命名
               //   （`KmoSlide.textStyleManager().isCleared()`）。
               //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
               todo!("DEFERRED_NON_FUNCTION")
            }
         }
         // Java: （slide == null 或 type != 0 分支）
         false
      }
    }

    // Java: public void resetTextWrappingTypeWhenAutoFit()
    pub fn resetTextWrappingTypeWhenAutoFit(&mut self) {
      // Java: if (this.dgShape != null && this.shape != null)
      // Java: {
      // Java:    TextBodyProperties textBodyProperties = this.textBodyProperties(true);
      // Java:    if (textBodyProperties.hasTextWrappingType() && textBodyProperties.textWrappingType() == 0
      // Java:        && textBodyProperties.hasTextAutofit() && textBodyProperties.textAutofit().type() == 2)
      // Java:    {
      // Java:       textBodyProperties.putTextWrappingType(1);
      // Java:       textBodyProperties.save();
      // Java:       this.dgShape.writeBodyPr(textBodyProperties);
      // Java:    }
      // Java: }
      // DEFERRED_NON_FUNCTION: `dgShape`/`TextBodyProperties` 不可命名（`textBodyProperties()` 连带）。
      //   Java 体见 `KmoTextOperator.java:1225-1235`。
      //   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
      todo!("DEFERRED_NON_FUNCTION")
    }
}

// Java: public class KmoTextOperator implements KmoProtocol
//
// DEFERRED_NON_FUNCTION: `implements KmoProtocol` 的接口实现面整体停车 —— 规范路径
//   `cn.wps.drawing.app.KmoProtocol` 在默认构建下被 `#[cfg(feature = "full")]` 关
//   （`drawing/app/mod.rs`）；本类已如实保留 `protocol()`/`updateProtocol()` 两个成员签名，
//   待上游 trait 落到默认构建后 `impl KmoProtocol for KmoTextOperator` 一句即可接上
//   （`KmoProtocol.rs` 要求 `fn protocol(&self) -> Box<dyn KoBin>` 与
//   `fn updateProtocol(&mut self, var1: &dyn KoBin)`，与本类两成员同型）。
//   owner: show.app.text    wave: M-PPT-KMOSLIDE-SP3（SP3b）
