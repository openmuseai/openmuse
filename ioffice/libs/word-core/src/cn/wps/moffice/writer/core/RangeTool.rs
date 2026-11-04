// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.base.io.ControlChar
use util::base::io::ControlChar::*;
// Java: import cn.wps.collections.textrope.ITextRope
use crate::cn::wps::collections::textrope::ITextRope::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.helper.HelperHeaderFooter
use crate::cn::wps::moffice::writer::core::helper::HelperHeaderFooter::*;
// Java: import cn.wps.moffice.writer.core.property.IPropertyCalculate
use crate::cn::wps::moffice::writer::core::property::IPropertyCalculate::*;
// Java: import cn.wps.moffice.writer.core.property.PropertyCalculateFactory
// rust: Factory/Impl 尚未登记（模板缺陷）；`getTargetHightLightColor` 按 pid=15 从 run 属性读取。
// use crate::cn::wps::moffice::writer::core::property::PropertyCalculateFactory::*;
// Java: import cn.wps.moffice.writer.core.table.KColRange
// CARGO_BLOCKER: KColRange not generated
// use crate::cn::wps::moffice::writer::core::table::KColRange::*;
// Java: import cn.wps.moffice.writer.core.table.KTableRangeBase
// rust: `KTableRangeBase` 已在 `table/mod.rs` 延后（依赖 `data::table`）。
// use crate::cn::wps::moffice::writer::core::table::KTableRangeBase::*;
// Java: import cn.wps.moffice.writer.core.KHeaderFooterIndex
use crate::cn::wps::moffice::writer::core::KHeaderFooterIndex::*;
// Java: import cn.wps.moffice.writer.data.DefaultValue
use crate::cn::wps::moffice::writer::data::DefaultValue::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::*;
// Java: import cn.wps.moffice.writer.data.ITable
use crate::cn::wps::moffice::writer::data::ITable::*;
// Disambiguate nested Entry traits (IChpxTable/IPapxTable/ITable)
use crate::cn::wps::moffice::writer::data::IChpxTable::Entry as ChpxEntry;
use crate::cn::wps::moffice::writer::data::IPapxTable::Entry as PapxEntry;
use crate::cn::wps::moffice::writer::data::ITable::Entry as ITableEntry;
// Java: import cn.wps.moffice.writer.data.MathTool
use crate::cn::wps::moffice::writer::data::MathTool::*;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::*;
// Java: import cn.wps.moffice.writer.data.PLCField
use crate::cn::wps::moffice::writer::data::PLCField::*;
// Java: import cn.wps.moffice.writer.data.PLCFootnoteEndnoteRef
use crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteRef::*;
// Java: import cn.wps.moffice.writer.data.PLCFootnoteEndnoteTxt
use crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteTxt::*;
// Java: import cn.wps.moffice.writer.data.PLCMathFunction
use crate::cn::wps::moffice::writer::data::PLCMathFunction::*;
// Java: import cn.wps.moffice.writer.data.PLCMathStart
use crate::cn::wps::moffice::writer::data::PLCMathStart::*;
// Java: import cn.wps.moffice.writer.data.PLCSection
use crate::cn::wps::moffice::writer::data::PLCSection::*;
// Java: import cn.wps.moffice.writer.data.PLCTextboxText
use crate::cn::wps::moffice::writer::data::PLCTextboxText::*;
// Java: import cn.wps.moffice.writer.data.RANGE
use crate::cn::wps::moffice::writer::data::RANGE::*;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::*;
// Java: import cn.wps.moffice.writer.data.TableTool
use crate::cn::wps::moffice::writer::data::TableTool::*;
// Java: import cn.wps.moffice.writer.data.field.KFieldType
use crate::cn::wps::moffice::writer::data::field::KFieldType::*;
// Java: import cn.wps.moffice.writer.data.value.Revision
use crate::cn::wps::moffice::writer::data::value::Revision::*;
// Java: import cn.wps.util.concurrent.locks.RangeReadLock
use crate::cn::wps::util::concurrent::locks::RangeReadLock::*;
// Java: import java.util.ArrayList
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.SelectionType
use crate::cn::wps::moffice::writer::core::SelectionType::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.core.WriterModelException
use crate::cn::wps::moffice::writer::core::WriterModelException::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
// Java: import java.lang.Math
use stubs::java::lang::Math::*;
// Java: import java.lang.Character
use stubs::java::lang::Character::*;

fn chpx_eq(a: &dyn ChpxEntry, b: &dyn ChpxEntry) -> bool {
    a.position() == b.position() && a.isEnd() == b.isEnd()
}
fn papx_eq(a: &dyn PapxEntry, b: &dyn PapxEntry) -> bool {
    a.position() == b.position() && a.isEnd() == b.isEnd()
}
fn itable_eq(a: &dyn ITableEntry, b: &dyn ITableEntry) -> bool {
    a.position() == b.position() && a.isEnd() == b.isEnd()
}
fn chpx_seek(table: *mut dyn IChpxTable, cp: i32) -> Option<Box<dyn ChpxEntry>> {
    unsafe { IChpxTable::seek(&*table, cp) }
}
fn papx_seek(table: *mut dyn IPapxTable, cp: i32) -> Option<Box<dyn PapxEntry>> {
    unsafe { IPapxTable::seek(&*table, cp) }
}
// rust: ★T7-9 —— `Document` 门面真实化后，PLC 访问器返回 `Option<*mut T>`（真实类型）
//   而非 `Option<*mut ()>` ⇒ 本辅助函数改为**对目标类型泛型**，不再需要调用方
//   先擦除再还原（旧版 `Option<*mut ()>` + 内部 `as *mut T` 正是假编译面的来源）。
fn plc_as_mut<'a, T>(p: Option<*mut T>) -> Option<&'a mut T> {
    p.and_then(|ptr| unsafe { ptr.as_mut() })
}
fn field_mut(document: &dyn Document) -> Option<&mut crate::cn::wps::moffice::writer::data::PLCField::PLCField> {
    document.getPLCField().and_then(|p| unsafe { p.as_mut() })
}
fn hf_range(
    document: &dyn Document,
    section: &crate::cn::wps::moffice::writer::data::PLCSection::SectionNode,
    header: bool,
    index: crate::cn::wps::moffice::writer::core::KHeaderFooterIndex::KHeaderFooterIndex,
) -> i64 {
    if header {
        HelperHeaderFooter::getRangeHeader(document, section, index)
            .expect("getRangeHeader")
            .getRange()
    } else {
        HelperHeaderFooter::getRangeFooter(document, section, index)
            .expect("getRangeFooter")
            .getRange()
    }
}

// java public class RangeTool
#[derive(Default)]
pub struct RangeTool {
}

// java public class RangeTool
impl RangeTool {
    // java static int getRangeLevel(Document document, int cp)
    pub fn getRangeLevel(document: &dyn Document, cp: i32) -> i32 {
      let mut paragraph: Box<dyn PapxEntry> =
        papx_seek(document.getPapxTable().expect("getPapxTable"), cp).expect("paragraph");
      // java PropertySet prop = paragraph.getProperty();
      let mut prop: PropertySet = PapxEntry::getProperty(&*paragraph).expect("prop").clone();
      // java if (!prop.getBoolean(223, false))
      if !prop.getBoolean__int_boolean(223, false) {
        return 0;
      } else {
        // java int level = prop.getInt(224, 1);
        let mut level: i32 = prop.getInt__int_int(224, 1);
        // java if (prop.getBoolean(225, false))
        if prop.getBoolean__int_boolean(225, false) {
          { let _tmp = level; level -= 1; _tmp };
        }
        return level;
      }
    }

    // java public static void normalize2(Document document, RANGE range)
    pub fn normalize2(document: &mut dyn Document, range: Option<&mut RANGE>) {
      if let Some(range) = range {
        if range.start < range.end {
          let mut startLevel: i32 = Self::getRangeLevel(document, range.start);
          let mut endLevel: i32 = Self::getRangeLevel(document, range.end - 1);
          if startLevel != endLevel {
            if startLevel < endLevel {
              let mut rgCell: i64 = TableTool::getCellRange(document, range.end - 1, endLevel).expect("rgCell");
              range.start = Math::max(RANGEFactory::getStart(rgCell), range.start);
              range.end = Math::min(RANGEFactory::getEnd(rgCell), range.end);
            } else {
              let mut rgCell: i64 = TableTool::getCellRange(document, range.start, startLevel).expect("rgCell");
              range.start = Math::max(RANGEFactory::getStart(rgCell), range.start);
              range.end = Math::min(RANGEFactory::getEnd(rgCell), range.end);
            }
          } else {
            if startLevel == 0 {
              return;
            }
            let mut rgCell: i64 = TableTool::getCellRange(document, range.start, startLevel).expect("rgCell");
            range.start = Math::max(RANGEFactory::getStart(rgCell), range.start);
            range.end = Math::min(RANGEFactory::getEnd(rgCell), range.end);
          }
        }
      }
    }

    // java public static SelectionType normalize3(Document document, RANGE range)
    pub fn normalize3(document: &mut dyn Document, range: &mut RANGE) -> SelectionType {
      let mut readLock: RangeReadLock = document.readLock();
      let mut var8: SelectionType = SelectionType::NONE;
      // java KTableRangeBase.TableRangeContext context = KTableRangeBase.normalize(document, range);
      // rust: KTableRangeBase/KColRange 已在 table/mod.rs 延后。
      let _ = range;
      readLock.unlock();
      return var8;
    }

    // java public static Long getHeaderFooterRangeContainsCp(Document document, int cp)
    pub fn getHeaderFooterRangeContainsCp(document: &dyn Document, cp: i32) -> Option<i64> {
      // java if (cp >= 0 && cp <= document.getLength())
      if cp >= 0 && cp <= document.getLength() {
        // java if (2 == document.getType())
        if 2 == document.getType() {
          // java TextDocument textDocument = document.getTextDocument();
          let mut textDocument: &dyn SoftTextDocument = document.getTextDocument().expect("getTextDocument");
          // java PLCSection plcSection = textDocument.getMainDocument().getPLCSection();
          let mut plcSection: &PLCSection = textDocument.getMainDocument().and_then(|d| plc_as_mut::<PLCSection>(d.getPLCSection())).expect("getPLCSection");
          // java int size = plcSection.size();
          let mut size: i32 = plcSection.size();
          let mut i: i32 = 0;
          // java for(int i = 0; i < size; ++i)
          while i < size {
            // java PLCSection.SectionNode sectionNode = (PLCSection.SectionNode)plcSection.get(i);
            // rust: 后接 `// java (null != sectionNode)` ⇒ 保持 `Option`。
            let mut sectionNode: crate::cn::wps::moffice::writer::data::PLCSection::SectionNode = as_section(plcSection.get(i));
            if sectionNode.getEvenHeader().ptr.is_null() && sectionNode.getOddHeader().ptr.is_null() && sectionNode.getEvenFooter().ptr.is_null() && sectionNode.getOddFooter().ptr.is_null() && sectionNode.getFirstHeader().ptr.is_null() && sectionNode.getFirstFooter().ptr.is_null() {
              // java return null;
              return None;
            }
            // java long range = HelperHeaderFooter.getRangeHeader(document, sectionNode, KHeaderFooterIndex.HeaderFooterPrimary).getRange();
            let mut range: i64 = HelperHeaderFooter::getRangeHeader(document, &sectionNode, KHeaderFooterIndex::HeaderFooterPrimary).expect("getRangeHeader").getRange();
            // java if (RANGEFactory.contains(range, cp))
            if RANGEFactory::contains(range, cp) {
              // java return range;
              return Some(range);
            }
            // java range = HelperHeaderFooter.getRangeHeader(document, sectionNode, KHeaderFooterIndex.HeaderFooterFirstPage).getRange();
            range = HelperHeaderFooter::getRangeHeader(document, &sectionNode, KHeaderFooterIndex::HeaderFooterFirstPage).expect("getRangeHeader").getRange();
            // java if (RANGEFactory.contains(range, cp))
            if RANGEFactory::contains(range, cp) {
              // java return range;
              return Some(range);
            }
            // java range = HelperHeaderFooter.getRangeHeader(document, sectionNode, KHeaderFooterIndex.HeaderFooterEvenPages).getRange();
            range = HelperHeaderFooter::getRangeHeader(document, &sectionNode, KHeaderFooterIndex::HeaderFooterEvenPages).expect("getRangeHeader").getRange();
            // java if (RANGEFactory.contains(range, cp))
            if RANGEFactory::contains(range, cp) {
              // java return range;
              return Some(range);
            }
            // java range = HelperHeaderFooter.getRangeFooter(document, sectionNode, KHeaderFooterIndex.HeaderFooterPrimary).getRange();
            range = HelperHeaderFooter::getRangeFooter(document, &sectionNode, KHeaderFooterIndex::HeaderFooterPrimary).expect("getRangeFooter").getRange();
            // java if (RANGEFactory.contains(range, cp))
            if RANGEFactory::contains(range, cp) {
              // java return range;
              return Some(range);
            }
            // java range = HelperHeaderFooter.getRangeFooter(document, sectionNode, KHeaderFooterIndex.HeaderFooterFirstPage).getRange();
            range = HelperHeaderFooter::getRangeFooter(document, &sectionNode, KHeaderFooterIndex::HeaderFooterFirstPage).expect("getRangeFooter").getRange();
            // java if (RANGEFactory.contains(range, cp))
            if RANGEFactory::contains(range, cp) {
              // java return range;
              return Some(range);
            }
            // java range = HelperHeaderFooter.getRangeFooter(document, sectionNode, KHeaderFooterIndex.HeaderFooterEvenPages).getRange();
            range = HelperHeaderFooter::getRangeFooter(document, &sectionNode, KHeaderFooterIndex::HeaderFooterEvenPages).expect("getRangeFooter").getRange();
            // java if (RANGEFactory.contains(range, cp))
            if RANGEFactory::contains(range, cp) {
              // java return range;
              return Some(range);
            }
            i += 1;
          }
        }
        // java Assert.fail("It should not reach here!");
        Assert::fail__string(&"It should not reach here!");
        // java return null;
        return None;
      }
      // java else
      else {
        // java return null;
        return None;
      }
    }

    // java public static Long getEndNoteRangeContainsCp(Document document, int cp)
    pub fn getEndNoteRangeContainsCp(document: &dyn Document, cp: i32) -> Option<i64> {
      // java if (cp >= 0 && cp <= document.getLength())
      if cp >= 0 && cp <= document.getLength() {
        // java if (4 == document.getType())
        if 4 == document.getType() {
          // java PLCFootnoteEndnoteTxt endNoteText = document.getEndnoteTxt();
          let mut endNoteText: &PLCFootnoteEndnoteTxt = plc_as_mut::<PLCFootnoteEndnoteTxt>(document.getEndnoteTxt()).expect("getEndnoteTxt");
          // java PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode node = endNoteText.findFootnoteEndnoteTxtContainsCP(cp);
          // rust: 后接 `// java if (null != node)` ⇒ 保持 `Option`。
          let mut node: Option<crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteTxt::FootnoteEndnoteTxtNode> = endNoteText.findFootnoteEndnoteTxtContainsCP(cp);
          // java if (null != node)
          if node.is_some() {
                // java return endNoteText.getRange(node.expect("node"));
                return Some(endNoteText.getRange(&node.expect("node")));
          }
        }
        // java return null;
        return None;
      }
      // java else
      else {
        // java return null;
        return None;
      }
    }

    // java public static Long getFootNoteRangeContainsCp(Document document, int cp)
    pub fn getFootNoteRangeContainsCp(document: &dyn Document, cp: i32) -> Option<i64> {
      // java if (cp >= 0 && cp <= document.getLength())
      if cp >= 0 && cp <= document.getLength() {
        // java if (1 == document.getType())
        if 1 == document.getType() {
          // java PLCFootnoteEndnoteTxt footNoteTxt = document.getFootnoteTxt();
          let mut footNoteTxt: &PLCFootnoteEndnoteTxt = plc_as_mut::<PLCFootnoteEndnoteTxt>(document.getFootnoteTxt()).expect("getFootnoteTxt");
          // java PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode node = footNoteTxt.findFootnoteEndnoteTxtContainsCP(cp);
          // rust: 后接 `// java (null != node)` ⇒ 保持 `Option`。
          let mut node: Option<crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteTxt::FootnoteEndnoteTxtNode> = footNoteTxt.findFootnoteEndnoteTxtContainsCP(cp);
          if node.is_some() {
                // java return footNoteTxt.getRange(node);
                return Some(footNoteTxt.getRange(&node.expect("node")));
          }
        }
        // java return null;
        return None;
      }
      // java else
      else {
        // java return null;
        return None;
      }
    }

    // java public static Long getTextboxNodeRangeContainsCp(Document document, int cp)
    pub fn getTextboxNodeRangeContainsCp(document: &dyn Document, cp: i32) -> Option<i64> {
      // java if (cp >= 0 && cp <= document.getLength())
      if cp >= 0 && cp <= document.getLength() {
        // java int type = document.getType();
        let mut r#type: i32 = document.getType();
        // java if (type == 5)
        if r#type == 5 {
          // java PLCTextboxText textbox = document.getTextboxText();
          // rust: 后接 `// java (null != textbox)` ⇒ 保持 `Option`。
          let mut textbox: Option<&PLCTextboxText> = plc_as_mut::<PLCTextboxText>(document.getTextboxText()).map(|p| &*p);
          if textbox.is_some() {
            // java PLCTextboxText.TextboxTextNode node = textbox.findTextboxByCP(cp);
            // rust: 后接 `// java (null != node)` ⇒ 保持 `Option`。
            let mut node: Option<crate::cn::wps::moffice::writer::data::PLCTextboxText::TextboxTextNode> = textbox.as_ref().and_then(|tb| tb.findTextboxByCP(cp));
            if node.is_some() {
                  // java long rg = textbox.getRange(node);
                  let mut rg: i64 = textbox.as_ref().expect("textbox").getRange(&node.expect("node"));
                  // java return rg;
                  return Some(rg);
            }
          }
        }
        // java return null;
        return None;
      }
      // java else
      else {
        // java return null;
        return None;
      }
    }

    // java public static void checkRange(Document document, int cp, RANGE rg, boolean forward)
    pub fn checkRange__document_int_range_boolean(document: &dyn Document, cp: i32, rg: &mut RANGE, forward: bool) {
      // java int type = document.getType();
      let mut r#type: i32 = document.getType();
      // java Long range = null;
      // rust: Java 声明为 `Long`（可空）⇒ 对应 `Option<i64>`（下文有 null 检查）。
      let mut range: Option<i64> = None;
      // java if (type == 5)
      if r#type == 5 {
        // java range = getTextboxNodeRangeContainsCp(document, cp);
        range = Self::getTextboxNodeRangeContainsCp(document, cp);
      }
      // java else if (type == 2)
      else if r#type == 2 {
        // java range = getHeaderFooterRangeContainsCp(document, cp);
        range = Self::getHeaderFooterRangeContainsCp(document, cp);
      }
      // java else if (type == 1)
      else if r#type == 1 {
        // java range = getFootNoteRangeContainsCp(document, cp);
        range = Self::getFootNoteRangeContainsCp(document, cp);
      }
      // java else if (type == 4)
      else if r#type == 4 {
        // java range = getEndNoteRangeContainsCp(document, cp);
        range = Self::getEndNoteRangeContainsCp(document, cp);
      }
      // java if (null != range)
      if range.is_some() {
        // rust: Java 的 `Long` 自动拆箱 ⇒ 分支内以具体 `i64` 绑定，
        //   使后续 `RANGEFactory::getStart/getEnd(range)` 等调用类型一致。
        let range: i64 = range.expect("range");
        // java int start = rg.start;
        let mut start: i32 = rg.start;
        // java int end = rg.end;
        let mut end: i32 = rg.end;
        // java if (forward)
        if forward {
          // java rg.start = Math.max(start, RANGEFactory.getStart(range));
          rg.start = Math::max(start, RANGEFactory::getStart(range));
          // java rg.end = cp;
          rg.end = cp;
        }
        // java else
        else {
          // java rg.end = Math.min(end, RANGEFactory.getEnd(range) - 1);
          rg.end = Math::min(end, RANGEFactory::getEnd(range) - 1);
          // java rg.start = cp;
          rg.start = cp;
        }
      }
    }

    // java public static void checkRange(Document document, int cp, RANGE rg)
    pub fn checkRange__document_int_range(document: &dyn Document, cp: i32, rg: &mut RANGE) {
      // java int type = document.getType();
      let mut r#type: i32 = document.getType();
      // java Long range = getRangeContainsCp(document, cp);
      // rust: Java 声明为 `Long range`（可空）⇒ 对应 `Option<i64>`（下一条即 null 检查）。
      let mut range: Option<i64> = Self::getRangeContainsCp(document, cp);
      // java if (null != range)
      if range.is_some() {
        // rust: Java 的 `Long` 自动拆箱 ⇒ 分支内以具体 `i64` 绑定，
        //   使后续 `RANGEFactory::getStart/getEnd(range)` 等调用类型一致。
        let range: i64 = range.expect("range");
        // java int start = rg.start;
        let mut start: i32 = rg.start;
        // java int end = rg.end;
        let mut end: i32 = rg.end;
        // java boolean isEndNoteOrFootNoteSubDocument = type == 4 || type == 1;
        let mut isEndNoteOrFootNoteSubDocument: bool = r#type == 4 || r#type == 1;
        // java if (start == end)
        if start == end {
          // java if (start < RANGEFactory.getStart(range))
          if start < RANGEFactory::getStart(range) {
            // java rg.start = RANGEFactory.getStart(range);
            rg.start = RANGEFactory::getStart(range);
            // java rg.end = rg.start;
            rg.end = rg.start;
          }
          // java else
          else {
            // java if (isEndNoteOrFootNoteSubDocument)
            if isEndNoteOrFootNoteSubDocument {
              // java rg.end = Math.min(end, RANGEFactory.getEnd(range) - 1);
              rg.end = Math::min(end, RANGEFactory::getEnd(range) - 1);
            }
            // java else
            else {
              // java rg.end = Math.min(end, RANGEFactory.getEnd(range) - 2);
              rg.end = Math::min(end, RANGEFactory::getEnd(range) - 2);
            }
            // java rg.start = rg.end;
            rg.start = rg.end;
          }
        }
        // java else
        else {
          // java rg.start = Math.max(start, RANGEFactory.getStart(range));
          rg.start = Math::max(start, RANGEFactory::getStart(range));
          // java if (isEndNoteOrFootNoteSubDocument)
          if isEndNoteOrFootNoteSubDocument {
            // java rg.end = Math.min(end, RANGEFactory.getEnd(range));
            rg.end = Math::min(end, RANGEFactory::getEnd(range));
          }
          // java else
          else {
            // java rg.end = Math.min(end, RANGEFactory.getEnd(range) - 1);
            rg.end = Math::min(end, RANGEFactory::getEnd(range) - 1);
          }
        }
      }
    }

    // java public static boolean isTableCellOrRowPara(Document document, IPapxTable.Entry paragraph)
    pub fn isTableCellOrRowPara(document: &mut dyn Document, paragraph: &mut dyn PapxEntry) -> bool {
      // java return TableTool.isCellEndOrRowEnd(document, paragraph.endCp() - 1);
      return TableTool::isCellEndOrRowEnd(document, paragraph.endCp() - 1);
    }

    // java public static boolean isRunSpec(Document document, IChpxTable.Entry run)
    pub fn isRunSpec(document: &dyn Document, run: &dyn ChpxEntry) -> bool {
      // java Assert.assertTrue("run should not be the end of the chpxTable!", !run.isEnd());
      Assert::assertTrue__string_boolean(&"run should not be the end of the chpxTable!", !run.isEnd());
      // java PropertySet runProp = run.getProperty();
      // rust: `Entry`（Chpx/Papx）均含 `getProperty` ⇒ UFCS 指定 Chpx 版本；
      //   门面返回 `Option<&PropertySet>`，Java 侧紧随断言保证非 null。
      let mut runProp: PropertySet = <dyn ChpxEntry as ChpxEntry>::getProperty(run).expect("runProp").clone();
      // java return runProp.getBoolean__int_boolean(28, false);
      return runProp.getBoolean__int_boolean(28, false);
    }

    // java public static boolean isRunDelete(IChpxTable.Entry run)
    pub fn isRunDelete__entry(run: &dyn ChpxEntry) -> bool {
      // java Assert.assertTrue("run should not be the end of the chpxTable!", !run.isEnd());
      Assert::assertTrue__string_boolean(&"run should not be the end of the chpxTable!", !run.isEnd());
      // java if (run.isEnd())
      if run.isEnd() {
        // java return false;
        return false;
      } else {
        // java PropertySet runProp = run.getProperty();
        // rust: `Entry`（Chpx/Papx）均含 `getProperty` ⇒ UFCS 指定 Chpx 版本；
      //   门面返回 `Option<&PropertySet>`，Java 侧紧随断言保证非 null。
      let mut runProp: PropertySet = <dyn ChpxEntry as ChpxEntry>::getProperty(run).expect("runProp").clone();
        // java Object revision = runProp.getValue(45);
        // rust: 后接 `// java (null != revision)` ⇒ 保持 `Option`。
        let mut revision = runProp.getValue(45);
        let mut fRevisionDelete: bool = runProp.getBoolean__int_boolean(54, false);
        // java return null != revision && fRevisionDelete;
        return revision.is_some() && fRevisionDelete;
      }
    }

    // java public static boolean isRunInsert(IChpxTable.Entry run)
    pub fn isRunInsert(run: &dyn ChpxEntry) -> bool {
      // java Assert.assertTrue("run should not be the end of the chpxTable!", !run.isEnd());
      Assert::assertTrue__string_boolean(&"run should not be the end of the chpxTable!", !run.isEnd());
      // java PropertySet runProp = run.getProperty();
      // rust: `Entry`（Chpx/Papx）均含 `getProperty` ⇒ UFCS 指定 Chpx 版本；
      //   门面返回 `Option<&PropertySet>`，Java 侧紧随断言保证非 null。
      let mut runProp: PropertySet = <dyn ChpxEntry as ChpxEntry>::getProperty(run).expect("runProp").clone();
      // java Object revision = runProp.getValue(46);
      // rust: 后接 `// java (null != revision)` ⇒ 保持 `Option`。
      let mut revision = runProp.getValue(46);
      let mut fRevisionDelete: bool = runProp.getBoolean__int_boolean(55, false);
      // java return null != revision && fRevisionDelete;
      return revision.is_some() && fRevisionDelete;
    }

    // java public static IChpxTable.Entry[] getChpxEntries(Document document, long range)
    pub fn getChpxEntries(document: &dyn Document, range: i64) -> Vec<Box<dyn ChpxEntry>> {
      // java IChpxTable chpxTable = document.getChpxTable();
      // rust: 门面返回擦除指针 `Option<*mut dyn IChpxTable>` ⇒ 保持裸指针并解引用。
      let mut chpxTable: *mut dyn IChpxTable = document.getChpxTable().expect("getChpxTable");
      // java Assert.assertNotNull("chpxTable should not be null.", chpxTable);
      Assert::assertNotNull__string_object(&"chpxTable should not be null.", Some(&chpxTable));
      // java IChpxTable.Entry s = chpxTable.seek(RANGEFactory.getStart(range));
      let mut s: Box<dyn ChpxEntry> = IChpxTable::seek(unsafe { &*chpxTable }, RANGEFactory::getStart(range)).expect("s");
      // java IChpxTable.Entry e = chpxTable.seek(RANGEFactory.getEnd(range));
      let mut e: Box<dyn ChpxEntry> = IChpxTable::seek(unsafe { &*chpxTable }, RANGEFactory::getEnd(range)).expect("e");
      // java if (!e.isEnd() && e.position() != RANGEFactory.getEnd(range))
      if !e.isEnd() && e.position() != RANGEFactory::getEnd(range) {
        // java e = e.getNext();
        e = ChpxEntry::getNext(&*e).expect("getNext");
      }
      // java if (!e.isEnd() && s == e)
      if !e.isEnd() && chpx_eq(&*s, &*e) {
        // java e = e.getNext();
        e = ChpxEntry::getNext(&*e).expect("getNext");
      }
      // java return toArray(s, e);
      return Self::toArray_owned_chpx(s, e);
    }

    fn toArray_owned_chpx(s: Box<dyn ChpxEntry>, e: Box<dyn ChpxEntry>) -> Vec<Box<dyn ChpxEntry>> {
      let mut entries: Vec<Box<dyn ChpxEntry>> = Vec::new();
      let mut cur = Some(s);
      while let Some(node) = cur {
        if chpx_eq(&*node, &*e) { break; }
        let next = ChpxEntry::getNext(&*node);
        entries.push(node);
        cur = next;
      }
      entries
    }

    // java public static IChpxTable.Entry[] toArray(IChpxTable.Entry s, IChpxTable.Entry e)
    pub fn toArray__chpxentry_chpxentry(s: &dyn ChpxEntry, e: &dyn ChpxEntry) -> Vec<Box<dyn ChpxEntry>> {
      let mut n: i32 = Self::count__entry_entry(s, e);
      let mut entries: Vec<Box<dyn ChpxEntry>> = Vec::with_capacity(n as usize);
      let mut cur: Option<Box<dyn ChpxEntry>> = None;
      loop {
        let r: &dyn ChpxEntry = match &cur { Some(b) => &**b, None => s };
        if chpx_eq(r, e) { break; }
        let next = ChpxEntry::getNext(r);
        if let Some(owned) = cur.take() {
          entries.push(owned);
        } else {
          break;
        }
        cur = next;
        if cur.is_none() { break; }
      }
      let _ = n;
      return entries;
    }

    // java public static int count(ITable.Entry s, ITable.Entry e, int count)
    pub fn count__entry_entry_int(s: &dyn ITableEntry, e: &dyn ITableEntry, count: i32) -> i32 {
      let mut n: i32 = 0;
      let mut cur: Option<Box<dyn ITableEntry>> = None;
      loop {
        let r: &dyn ITableEntry = match &cur { Some(b) => &**b, None => s };
        if itable_eq(r, e) || n >= count { break; }
        n += 1;
        cur = ITableEntry::getNext(r);
        if cur.is_none() { break; }
      }
      return n;
    }

    // java public static IPapxTable.Entry[] toArray(IPapxTable.Entry s, IPapxTable.Entry e, int count)
    pub fn toArray__entry_entry_int(s: &dyn PapxEntry, e: &dyn PapxEntry, count: i32) -> Vec<Box<dyn PapxEntry>> {
      let mut n: i32 = Self::count__entry_entry_int(s, e, count);
      let mut entries: Vec<Box<dyn PapxEntry>> = Vec::with_capacity(n as usize);
      let mut i: i32 = 0;
      let mut cur: Option<Box<dyn PapxEntry>> = None;
      while i < n {
        let r: &dyn PapxEntry = match &cur { Some(b) => &**b, None => s };
        let next = PapxEntry::getNext(r);
        if let Some(owned) = cur.take() {
          entries.push(owned);
        } else {
          break;
        }
        cur = next;
        i += 1;
      }
      return entries;
    }

    // java public static IPapxTable.Entry[] toArray(IPapxTable.Entry s, IPapxTable.Entry e)
    pub fn toArray__papxentry_papxentry(s: &dyn PapxEntry, e: &dyn PapxEntry) -> Vec<Box<dyn PapxEntry>> {
      let mut n: i32 = Self::count__entry_entry(s, e);
      let mut entries: Vec<Box<dyn PapxEntry>> = Vec::with_capacity(n as usize);
      let mut cur: Option<Box<dyn PapxEntry>> = None;
      loop {
        let r: &dyn PapxEntry = match &cur { Some(b) => &**b, None => s };
        if papx_eq(r, e) { break; }
        let next = PapxEntry::getNext(r);
        if let Some(owned) = cur.take() {
          entries.push(owned);
        } else {
          break;
        }
        cur = next;
        if cur.is_none() { break; }
      }
      let _ = n;
      return entries;
    }

    // java public static int count(ITable.Entry s, ITable.Entry e)
    pub fn count__entry_entry(s: &dyn ITableEntry, e: &dyn ITableEntry) -> i32 {
      let mut n: i32 = 0;
      let mut cur: Option<Box<dyn ITableEntry>> = None;
      loop {
        let r: &dyn ITableEntry = match &cur { Some(b) => &**b, None => s };
        if itable_eq(r, e) { break; }
        n += 1;
        cur = ITableEntry::getNext(r);
        if cur.is_none() { break; }
      }
      return n;
    }

    // java public static boolean isContainsParagraphEnd(Document document, int start, int end)
    pub fn isContainsParagraphEnd(document: &dyn Document, start: i32, end: i32) -> bool {
      // java IPapxTable papx = document.getPapxTable();
      // rust: 门面返回擦除指针 `Option<*mut dyn IPapxTable>` ⇒ 保持裸指针并解引用。
      let mut papx: *mut dyn IPapxTable = document.getPapxTable().expect("getPapxTable");
      // java IPapxTable.Entry para = papx.seek(start);
      let mut para: Box<dyn PapxEntry> = IPapxTable::seek(unsafe { &*papx }, start).expect("para");
      // java return end >= para.endCp();
      return end >= para.endCp();
    }

    // java public static int getParagraphLength(Document document, int start)
    pub fn getParagraphLength(document: &dyn Document, start: i32) -> i32 {
      // java IPapxTable papx = document.getPapxTable();
      // rust: 门面返回擦除指针 `Option<*mut dyn IPapxTable>` ⇒ 保持裸指针并解引用。
      let mut papx: *mut dyn IPapxTable = document.getPapxTable().expect("getPapxTable");
      // java IPapxTable.Entry para = papx.seek(start);
      let mut para: Box<dyn PapxEntry> = IPapxTable::seek(unsafe { &*papx }, start).expect("para");
      // java return para.length();
      return para.length();
    }

    // java public static boolean isBeginOfSubDocumentsPLCNode(Document document, int cp)
    pub fn isBeginOfSubDocumentsPLCNode(document: &dyn Document, cp: i32) -> bool {
      // java if (cp == 0)
      if cp == 0 {
        // java return true;
        return true;
      }
      // java else if (cp == document.getLength())
      else if cp == document.getLength() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java Long range = getRangeContainsCp(document, cp);
        // rust: Java 声明为 `Long range`（可空）⇒ 对应 `Option<i64>`（下一条即 null 检查）。
      let mut range: Option<i64> = Self::getRangeContainsCp(document, cp);
        // java if (null != range)
        if range.is_some() {
          // rust: Java 的 `Long` 自动拆箱 ⇒ 分支内以具体 `i64` 绑定，
          //   使后续 `RANGEFactory::getStart/getEnd(range)` 等调用类型一致。
          let range: i64 = range.expect("range");
          // java return RANGEFactory.getStart(range) == cp;
          return RANGEFactory::getStart(range) == cp;
        }
        // java else
        else {
          // java return false;
          return false;
        }
      }
    }

    // java public static boolean isInvisibaleCP(Document document, int cp)
    pub fn isInvisibaleCP(document: &dyn Document, cp: i32) -> bool {
      // java return inFieldCodeOrIsInlineShape(document, cp) || isRunDelete(document, cp) || isLinkInvisibaleCP(document, cp) || MathTool.isInMath(document, cp);
      return Self::inFieldCodeOrIsInlineShape(document, cp) || Self::isRunDelete__document_int(document, cp) || Self::isLinkInvisibaleCP(document, cp) || MathTool::isInMath(document, cp);
    }

    // java public static boolean isSerialBreak(Document document, int cp)
    pub fn isSerialBreak(document: &mut dyn Document, cp: i32) -> bool {
      // java if (cp > 0)
      if cp > 0 {
        // java char preChar = document.charAt(cp - 1);
        let mut preChar: char = document.charAt(cp - 1);
        // java char thisChar = document.charAt(cp);
        let mut thisChar: char = document.charAt(cp);
        // java if (isParaBreak(thisChar) && isParaBreak(preChar))
        if Self::isParaBreak(thisChar) && Self::isParaBreak(preChar) {
          // java return true;
          return true;
        }
      }
      // java return false;
      return false;
    }

    // java private static boolean isParaBreak(char ch)
    pub fn isParaBreak(ch: char) -> bool {
      // java switch (ch)
      match ch {
        // java case '\u000b': case '\r':
        '\u{000b}' | '\r' => {
          // java return true;
          return true;
        }
        // java default:
        _ => {
          // java return false;
          return false;
        }
      }
    }

    // java private static boolean isLinkInvisibaleCP(Document document, int cp)
    pub fn isLinkInvisibaleCP(document: &dyn Document, cp: i32) -> bool {
      // java int type = document.getType();
      let mut r#type: i32 = document.getType();
      // java Long range = null;
      // rust: Java 声明为 `Long`（可空）⇒ 对应 `Option<i64>`（下文有 null 检查）。
      let mut range: Option<i64> = None;
      // java if (type == 5)
      if r#type == 5 {
        // java range = getTextboxNodeRangeContainsCp(document, cp);
        range = Self::getTextboxNodeRangeContainsCp(document, cp);
      }
      // java else if (type == 2)
      else if r#type == 2 {
        // java range = getHeaderFooterRangeContainsCp(document, cp);
        range = Self::getHeaderFooterRangeContainsCp(document, cp);
      }
      // java if (null != range)
      if range.is_some() {
        // rust: Java 的 `Long` 自动拆箱 ⇒ 分支内以具体 `i64` 绑定，
        //   使后续 `RANGEFactory::getStart/getEnd(range)` 等调用类型一致。
        let range: i64 = range.expect("range");
        // java return cp == RANGEFactory.getEnd(range) - 1;
        return cp == RANGEFactory::getEnd(range) - 1;
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java private static boolean isRunDelete(Document document, int cp)
    pub fn isRunDelete__document_int(document: &dyn Document, cp: i32) -> bool {
      // java IChpxTable.Entry entry = document.getChpxTable().seek(cp);
      let mut entry: Box<dyn ChpxEntry> = chpx_seek(document.getChpxTable().expect("getChpxTable"), cp).expect("entry");
      // java return isRunDelete(entry);
      return Self::isRunDelete__entry(&*entry);
    }

    // java private static boolean inFieldCodeOrIsInlineShape(Document document, int cp)
    pub fn inFieldCodeOrIsInlineShape(document: &dyn Document, cp: i32) -> bool {
      // java PLCField.FieldTriple field = document.getPLCField().expect("getPLCField").getContainingField(cp).expect("field");
      let mut field = field_mut(document).and_then(|f| f.getContainingField(cp));
      // java if (field == null)
      if field.is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java return RANGEFactory.contains(field.getCodeRange(), cp) || field.getFieldType() == KFieldType.FieldShape.getFlt();
        return field.as_ref().map(|f| RANGEFactory::contains(f.getCodeRange(), cp) || f.getFieldType() == KFieldType::FieldShape.getFlt()).unwrap_or(false);
      }
    }

    // java public static boolean isEndOfSubDocumentsPLCNode(Document document, int cp)
    pub fn isEndOfSubDocumentsPLCNode(document: &dyn Document, cp: i32) -> bool {
      // java if (cp == 0)
      if cp == 0 {
        // java return false;
        return false;
      }
      // java else if (cp == document.getLength())
      else if cp == document.getLength() {
        // java return true;
        return true;
      }
      // java else
      else {
        // java Long range = getRangeContainsCp(document, cp);
        // rust: Java 声明为 `Long range`（可空）⇒ 对应 `Option<i64>`（下一条即 null 检查）。
      let mut range: Option<i64> = Self::getRangeContainsCp(document, cp);
        // java if (null == range)
        if range.is_none() {
          // java return false;
          return false;
        }
        // java else
        else {
          let range: i64 = range.expect("range");
          // java int type = document.getType();
          let mut r#type: i32 = document.getType();
          // java boolean isEndNoteOrFootNoteSubDocument = type == 4 || type == 1;
          let mut isEndNoteOrFootNoteSubDocument: bool = r#type == 4 || r#type == 1;
          // java if (!isEndNoteOrFootNoteSubDocument)
          if !isEndNoteOrFootNoteSubDocument {
            // java return RANGEFactory.getEnd(range) - 1 == cp;
            return RANGEFactory::getEnd(range) - 1 == cp;
          }
          // java else
          else {
            // java return RANGEFactory.getStart(range) == cp || cp == document.getLength();
            return RANGEFactory::getStart(range) == cp || cp == document.getLength();
          }
        }
      }
    }

    // java public static int getMaxLengthByCp(Document document, int cp)
    pub fn getMaxLengthByCp(document: &dyn Document, cp: i32) -> i32 {
      // java int type = document.getType();
      let mut r#type: i32 = document.getType();
      // java Long range = getRangeContainsCp(document, cp);
      // rust: Java 声明为 `Long range`（可空）⇒ 对应 `Option<i64>`（下一条即 null 检查）。
      let mut range: Option<i64> = Self::getRangeContainsCp(document, cp);
      // java if (null == range)
      if range.is_none() {
        // java return document.getLength();
        return document.getLength();
      }
      // java else
      else {
        // rust: Java 的 `Long` 自动拆箱 ⇒ 此处以具体 `i64` 绑定。
        let range: i64 = range.expect("range");
        // java boolean isEndNoteOrFootNoteSubDocument = type == 4 || type == 1;
        let mut isEndNoteOrFootNoteSubDocument: bool = r#type == 4 || r#type == 1;
        // java return isEndNoteOrFootNoteSubDocument ? RANGEFactory.getEnd(range) : RANGEFactory.getEnd(range) - 1;
        return (if isEndNoteOrFootNoteSubDocument { RANGEFactory::getEnd(range) } else { RANGEFactory::getEnd(range) - 1 });
      }
    }

    // java private static Long getRangeContainsCp(Document document, int cp)
    pub fn getRangeContainsCp(document: &dyn Document, cp: i32) -> Option<i64> {
      // java if (cp >= 0 && cp <= document.getLength())
      if cp >= 0 && cp <= document.getLength() {
        // java int type = document.getType();
        let mut r#type: i32 = document.getType();
        // java Long range = null;
        // rust: Java 声明为 `Long`（可空）⇒ `Option<i64>`。
        let mut range: Option<i64> = None;
        // java if (type == 5)
        if r#type == 5 {
          // java range = getTextboxNodeRangeContainsCp(document, cp);
          range = Self::getTextboxNodeRangeContainsCp(document, cp);
        }
        // java else if (type == 2)
        else if r#type == 2 {
          // java range = getHeaderFooterRangeContainsCp(document, cp);
          range = Self::getHeaderFooterRangeContainsCp(document, cp);
        }
        // java else if (type == 4)
        else if r#type == 4 {
          // java range = getEndNoteRangeContainsCp(document, cp);
          range = Self::getEndNoteRangeContainsCp(document, cp);
        }
        // java else if (type == 1)
        else if r#type == 1 {
          // java range = getFootNoteRangeContainsCp(document, cp);
          range = Self::getFootNoteRangeContainsCp(document, cp);
        }
        // java return range;
        return range;
      }
      // java else
      else {
        // java return null;
        return None;
      }
    }

    // java public static ArrayList<PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode> getFootnoteRefs(Document document, int start, int end)
    pub fn getFootnoteRefs(document: &dyn Document, start: i32, end: i32) -> Vec<crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode> {
      // java Assert.assertTrue("document should be main document!", document.getType() == 0);
      Assert::assertTrue__string_boolean(&"document should be main document!", document.getType() == 0);
      // java PLCFootnoteEndnoteRef plcFootRef = document.getFootnoteRef();
      let mut plcFootRef: &PLCFootnoteEndnoteRef = plc_as_mut::<PLCFootnoteEndnoteRef>(document.getFootnoteRef()).expect("getFootnoteRef");
      let mut lists: Vec<crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode> = Vec::new();
      let mut entry: Node = plcFootRef.findUpper(start);
      while entry.getCP() < end && !plcFootRef.isHead(entry.clone()) {
        lists.push(FootnoteEndnoteRefNode::fromNode(entry.clone()));
        entry = entry.getNext();
      }
      // java return lists;
      return lists;
    }

    // java public static ArrayList<PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode> getEndnoteRefs(Document document, int start, int end)
    pub fn getEndnoteRefs(document: &dyn Document, start: i32, end: i32) -> Vec<crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode> {
      // java Assert.assertTrue("document should be main document!", document.getType() == 0);
      Assert::assertTrue__string_boolean(&"document should be main document!", document.getType() == 0);
      // java PLCFootnoteEndnoteRef plcEndRef = document.getEndnoteRef();
      let mut plcEndRef: &PLCFootnoteEndnoteRef = plc_as_mut::<PLCFootnoteEndnoteRef>(document.getEndnoteRef()).expect("getEndnoteRef");
      let mut lists: Vec<crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode> = Vec::new();
      let mut entry: Node = plcEndRef.findUpper(start);
      while entry.getCP() < end && !plcEndRef.isHead(entry.clone()) {
        lists.push(FootnoteEndnoteRefNode::fromNode(entry.clone()));
        entry = entry.getNext();
      }
      // java return lists;
      return lists;
    }

    // java public static boolean expandToSelectMathFunction(Document document, int cp, boolean bBackspace, RANGE range)
    pub fn expandToSelectMathFunction(document: &dyn Document, cp: i32, bBackspace: bool, range: &mut RANGE) -> bool {
      // java if (document.getPLCMathFunction() == null)
      if document.getPLCMathFunction().is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java int newCp = bBackspace ? cp - 1 : cp;
        let mut newCp: i32 = (if bBackspace { cp - 1 } else { cp });
        // java PLCMathFunction.MathNode startNode = null;
        let mut startNode: Option<crate::cn::wps::moffice::writer::data::PLCMathFunction::MathStartNode> = None;
        let mut endNode: Option<crate::cn::wps::moffice::writer::data::PLCMathFunction::MathNode> = None;
        // java if (!bBackspace && MathTool.isMathBeginPlaceholder(document, newCp))
        if !bBackspace && MathTool::isMathBeginPlaceholder(document, newCp) {
          // java startNode = MathTool.getMathStartNodeByCp(document, newCp);
          startNode = MathTool::getMathStartNodeByCp(document, newCp);
          // java if (startNode == null)
          if startNode.is_none() {
            // java return false;
            return false;
          }
          // java endNode = startNode.getEndNode();
          endNode = startNode.as_ref().and_then(|n| n.getEndNode());
        }
        // java else if (bBackspace && MathTool.isMathEndPlaceholder(document, newCp))
        else if bBackspace && MathTool::isMathEndPlaceholder(document, newCp) {
          // java endNode = MathTool.getMathEndNodeByCp(document, newCp);
          endNode = MathTool::getMathEndNodeByCp(document, newCp);
          // java if (endNode == null)
          if endNode.is_none() {
            // java return false;
            return false;
          }
          // java startNode = endNode.getStartNode();
          startNode = endNode.as_ref().and_then(|n| n.getStartNode());
        }
        // java if (null != startNode && null != endNode)
        if startNode.is_some() && endNode.is_some() {
          // java range.start = startNode.getCP();
          range.start = startNode.as_ref().unwrap().getCP();
          range.end = endNode.as_ref().unwrap().getCP() + 1;
          // java return true;
          return true;
        }
        // java else
        else {
          // java return false;
          return false;
        }
      }
    }

    // java public static boolean expandToSelectOMath(Document document, int cp, boolean bBackspace, RANGE range)
    pub fn expandToSelectOMath(document: &dyn Document, cp: i32, bBackspace: bool, range: &mut RANGE) -> bool {
      // java int start = bBackspace ? cp - 1 : cp;
      let mut start: i32 = (if bBackspace { cp - 1 } else { cp });
      // java PLCMathStart oMathStarts = document.getOMathStarts();
      // rust: 后接 `// java (null != oMathStarts)` ⇒ 保持 `Option`。
      let mut oMathStarts = document.getOMathStarts();
      if oMathStarts.is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java PLCMathStart.OMathStartNode startNode = document.getOMathStarts().findClose(start);
        // rust: 后接 `// java (null != startNode)` ⇒ 保持 `Option`。
        let mut startNode = oMathStarts.and_then(|p| unsafe { (*p).findClose(start) });
        if startNode.is_none() {
          // java return false;
          return false;
        }
        // java else
        else {
          // java RANGE mathRange = RANGE.obtain(startNode.getCP(), startNode.getEnd().getCP());
          let mut mathRange: RANGE = RANGE::obtain__int_int(startNode.as_ref().unwrap().getCP(), startNode.as_ref().unwrap().getEnd().map(|e| e.getCP()).unwrap_or(0));
          // java boolean var8;
          let mut var8: bool;
              // java if (!mathRange.hasIntersection(range.start, range.end) || mathRange.start == range.start && mathRange.end == range.end)
              if !mathRange.hasIntersection(range.start, range.end) || mathRange.start == range.start && mathRange.end == range.end {
                // java return false;
                return false;
              }
              // java range.start = Math.min(mathRange.start, range.start);
              range.start = Math::min(mathRange.start, range.start);
              // java range.end = Math.max(mathRange.end, range.end);
              range.end = Math::max(mathRange.end, range.end);
              // java var8 = true;
              var8 = true;
                // java mathRange.recycle();
                mathRange.recycle();
          // java return var8;
          return var8;
        }
      }
    }

    // java public static boolean expandToSelectField(Document document, int cp, boolean bBackspace, RANGE fieldRange)
    pub fn expandToSelectField__document_int_boolean_range(document: &mut dyn Document, cp: i32, bBackspace: bool, fieldRange: &mut RANGE) -> bool {
      // java int newCp = bBackspace ? cp - 1 : cp;
      let mut newCp: i32 = (if bBackspace { cp - 1 } else { cp });
      // java PLCField.FieldTriple field = document.getPLCField().expect("getPLCField").getContainingField(newCp).expect("field");
      let mut field = field_mut(document).and_then(|f| f.getContainingField(newCp));
      // java if (field != null)
      if let Some(ref field) = field {
        // java if (null == field.getResultRange())
        if field.getResultRange().is_none() {
          // java fieldRange.start = field.getStartCP();
          fieldRange.start = field.getStartCP();
          // java fieldRange.end = field.getEnd();
          fieldRange.end = field.getEnd();
          // java return true;
          return true;
        }
        // java if (bBackspace)
        if bBackspace {
          // java if (cp == field.getEnd() || cp == field.getSepCP() + 1 || cp > field.getStartCP() && cp <= field.getSepCP())
          if cp == field.getEnd() || cp == field.getSepCP() + 1 || cp > field.getStartCP() && cp <= field.getSepCP() {
            // java fieldRange.start = field.getStartCP();
            fieldRange.start = field.getStartCP();
            fieldRange.end = field.getEnd();
            // java return true;
            return true;
          }
        }
        // java else if (cp == field.getEndCP() || cp >= field.getStartCP() && cp <= field.getSepCP())
        else if cp == field.getEndCP() || cp >= field.getStartCP() && cp <= field.getSepCP() {
          // java fieldRange.start = field.getStartCP();
          fieldRange.start = field.getStartCP();
          // java fieldRange.end = field.getEnd();
          fieldRange.end = field.getEnd();
          // java return true;
          return true;
        }
      }
      // java else if (bBackspace && document.charAt(newCp) == 21)
      else if bBackspace && document.charAt(newCp) == '\u{15}' {
        // java char[] buffer = new char[20];
        let mut buffer: Vec<char> = vec![Default::default(); 20usize];
        // java int len = newCp - Math.max(newCp - 20, 0);
        let mut len: i32 = newCp - Math::max(newCp - 20, 0);
        // java int var13 = Math.max(newCp - 20, 0);
        let mut var13: i32 = Math::max(newCp - 20, 0);
        // java boolean fSkip = false;
        let mut fSkip: bool = false;
        let mut count: i32 = 0;
        // java for(int count = 0; len > 0 var13 -= len)
        while len > 0 {
          // java document.getText(var13, var13 + len, buffer, 0);
          document.getText(var13, var13 + len, &mut buffer, 0);
          let mut i: i32 = len - 1;
          // java for(int i = len - 1; i >= 0 --i)
          while i >= 0 {
            // java if (!fSkip && buffer[i] == 19)
            if !fSkip && buffer[(i) as usize] == '\u{13}' {
              // java fieldRange.start = var13 + i;
              fieldRange.start = var13 + i;
              // java fieldRange.end = newCp;
              fieldRange.end = newCp;
              // java return true;
              return true;
            }
            // java if (buffer[i] == 19)
            if buffer[(i) as usize] == '\u{13}' {
              // java --count;
              { let _tmp = count; count -= 1; _tmp };
              // java if (count == 0)
              if count == 0 {
                // java fSkip = false;
                fSkip = false;
              }
            }
            // java else if (buffer[i] == 21)
            else if buffer[(i) as usize] == '\u{15}' {
              // java fSkip = true;
              fSkip = true;
              // java ++count;
              { let _tmp = count; count += 1; _tmp };
            }
          }
          // java len = var13 - Math.max(var13 - 20, 0);
          len = var13 - Math::max(var13 - 20, 0);
        }
      }
      // java else if (!bBackspace && document.charAt(newCp) == 19)
      else if !bBackspace && document.charAt(newCp) == '\u{13}' {
        // java char[] buffer = new char[20];
        let mut buffer: Vec<char> = vec![Default::default(); 20usize];
        // java int start = newCp + 1;
        let mut start: i32 = newCp + 1;
        // java int len = Math.min(start + 20, document.getLength()) - start;
        let mut len: i32 = Math::min(start + 20, document.getLength()) - start;
        // java boolean fSkip = false;
        let mut fSkip: bool = false;
        let mut count: i32 = 0;
        // java for(int count = 0; len > 0 len = Math.min(start + 20, document.getLength()) - start)
        while len > 0 {
          // java document.getText(start, start + len, buffer, 0);
          document.getText(start, start + len, &mut buffer, 0);
          let mut i: i32 = 0;
          // java for(int i = 0; i < len; ++i)
          while i < len {
            // java if (!fSkip && buffer[i] == 21)
            if !fSkip && buffer[(i) as usize] == '\u{15}' {
              // java fieldRange.start = newCp;
              fieldRange.start = newCp;
              // java fieldRange.end = start + i + 1;
              fieldRange.end = start + i + 1;
              // java return true;
              return true;
            }
            // java if (buffer[i] == 21)
            if buffer[(i) as usize] == '\u{15}' {
              // java --count;
              { let _tmp = count; count -= 1; _tmp };
              // java if (count == 0)
              if count == 0 {
                // java fSkip = false;
                fSkip = false;
              }
            }
            // java else if (buffer[i] == 19)
            else if buffer[(i) as usize] == '\u{13}' {
              // java fSkip = true;
              fSkip = true;
              // java ++count;
              { let _tmp = count; count += 1; _tmp };
            }
          }
          // java start += len;
          start = start + len;
        }
      }
      // java return false;
      return false;
    }

    // java public static boolean isInHeader(Document document, int cp)
    pub fn isInHeader(document: &dyn Document, cp: i32) -> bool {
      // java if (cp >= 0 && cp <= document.getLength())
      if cp >= 0 && cp <= document.getLength() {
        // java if (2 == document.getType())
        if 2 == document.getType() {
          // java TextDocument textDocument = document.getTextDocument();
          let mut textDocument: &dyn SoftTextDocument = document.getTextDocument().expect("getTextDocument");
          // java PLCSection plcSection = textDocument.getMainDocument().getPLCSection();
          let mut plcSection: &PLCSection = textDocument.getMainDocument().and_then(|d| plc_as_mut::<PLCSection>(d.getPLCSection())).expect("getPLCSection");
          // java int size = plcSection.size();
          let mut size: i32 = plcSection.size();
          let mut i: i32 = 0;
          // java for(int i = 0; i < size; ++i)
          while i < size {
            // java PLCSection.SectionNode sectionNode = (PLCSection.SectionNode)plcSection.get(i);
            let mut sectionNode: crate::cn::wps::moffice::writer::data::PLCSection::SectionNode = as_section(plcSection.get(i));
            // java long range = HelperHeaderFooter.getRangeHeader(document, sectionNode, KHeaderFooterIndex.HeaderFooterPrimary).getRange();
            let mut range: i64 = HelperHeaderFooter::getRangeHeader(document, &sectionNode, KHeaderFooterIndex::HeaderFooterPrimary).expect("getRangeHeader").getRange();
            // java if (RANGEFactory.contains(range, cp))
            if RANGEFactory::contains(range, cp) {
              // java return true;
              return true;
            }
            // java range = HelperHeaderFooter.getRangeHeader(document, sectionNode, KHeaderFooterIndex.HeaderFooterFirstPage).getRange();
            range = HelperHeaderFooter::getRangeHeader(document, &sectionNode, KHeaderFooterIndex::HeaderFooterFirstPage).expect("getRangeHeader").getRange();
            // java if (RANGEFactory.contains(range, cp))
            if RANGEFactory::contains(range, cp) {
              // java return true;
              return true;
            }
            // java range = HelperHeaderFooter.getRangeHeader(document, sectionNode, KHeaderFooterIndex.HeaderFooterEvenPages).getRange();
            range = HelperHeaderFooter::getRangeHeader(document, &sectionNode, KHeaderFooterIndex::HeaderFooterEvenPages).expect("getRangeHeader").getRange();
            // java if (RANGEFactory.contains(range, cp))
            if RANGEFactory::contains(range, cp) {
              // java return true;
              return true;
            }
            // java range = HelperHeaderFooter.getRangeFooter(document, sectionNode, KHeaderFooterIndex.HeaderFooterPrimary).getRange();
            range = HelperHeaderFooter::getRangeFooter(document, &sectionNode, KHeaderFooterIndex::HeaderFooterPrimary).expect("getRangeFooter").getRange();
            // java if (RANGEFactory.contains(range, cp))
            if RANGEFactory::contains(range, cp) {
              // java return false;
              return false;
            }
            // java range = HelperHeaderFooter.getRangeFooter(document, sectionNode, KHeaderFooterIndex.HeaderFooterFirstPage).getRange();
            range = HelperHeaderFooter::getRangeFooter(document, &sectionNode, KHeaderFooterIndex::HeaderFooterFirstPage).expect("getRangeFooter").getRange();
            // java if (RANGEFactory.contains(range, cp))
            if RANGEFactory::contains(range, cp) {
              // java return false;
              return false;
            }
            // java range = HelperHeaderFooter.getRangeFooter(document, sectionNode, KHeaderFooterIndex.HeaderFooterEvenPages).getRange();
            range = HelperHeaderFooter::getRangeFooter(document, &sectionNode, KHeaderFooterIndex::HeaderFooterEvenPages).expect("getRangeFooter").getRange();
            // java if (RANGEFactory.contains(range, cp))
            if RANGEFactory::contains(range, cp) {
              // java return false;
              return false;
            }
            i += 1;
          }
        }
        // java Assert.fail("It should not reach here!");
        Assert::fail__string(&"It should not reach here!");
        // java return false;
        return false;
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java public static int getVisitableCpStart(Document document, long range)
    pub fn getVisitableCpStart(document: &dyn Document, range: i64) -> i32 {
      // java int start = RANGEFactory.getStart(range);
      let mut start: i32 = RANGEFactory::getStart(range);
      // java int end = RANGEFactory.getEnd(range);
      let mut end: i32 = RANGEFactory::getEnd(range);
      // java IChpxTable chpxTable = document.getChpxTable();
      // rust: 门面返回擦除指针 `Option<*mut dyn IChpxTable>` ⇒ 保持裸指针并解引用。
      let mut chpxTable: *mut dyn IChpxTable = document.getChpxTable().expect("getChpxTable");
      // java int visitableCp;
      let mut visitableCp: i32;
      // java IChpxTable.Entry e;
      let mut e: Option<Box<dyn ChpxEntry>> = None;
      visitableCp = start;
      // java for(visitableCp = start start < end start = e.endCp())
      while start < end {
        // java e = IChpxTable::seek(unsafe { &*chpxTable }, start).expect("e");
        e = IChpxTable::seek(unsafe { &*chpxTable }, start);
        visitableCp = start;
        if e.as_ref().map(|ee| ITableEntry::isHead(&**ee) || !Self::is_revision_delete(&**ee)).unwrap_or(true) {
          // java break;
          break;
        }
      }
      // java return visitableCp;
      return visitableCp;
    }

    // java private static boolean is_revision_delete(IChpxTable.Entry run)
    pub fn is_revision_delete(run: &dyn ChpxEntry) -> bool {
      // java Revision revision = (Revision)run.getProperty().getValue(45);
      // rust: 后接 `// java (null != revision)` ⇒ 保持 `Option`。
      let mut revision: Option<Revision> = ChpxEntry::getProperty(run).and_then(|ps| ps.getValue(45)).and_then(|v| v.as_ref().as_any().downcast_ref::<Revision>().cloned());
      if revision.is_some() {
        // java Assert.assertTrue("revision.getMark() == Revision.DELETE should be true!", revision.getMark() == 1);
        Assert::assertTrue__string_boolean(&"revision.getMark() == Revision::DELETE should be true!", revision.as_ref().map(|r| r.getMark() == 1).unwrap_or(false));
        // java return true;
        return true;
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java public static boolean isMatchedLowSurrogate(Document doc, int cp)
    pub fn isMatchedLowSurrogate(doc: &dyn Document, cp: i32) -> bool {
      // java ITextRope textRope = doc.getTextRope();
      let mut textRope: &mut dyn ITextRope = unsafe { &mut *doc.getTextRope().expect("getTextRope") };
      // java int prevCP = cp - 1;
      let mut prevCP: i32 = cp - 1;
      // java int docLen = textRope.length();
      let mut docLen: i32 = textRope.length();
      // java if (cp >= 0 && cp < docLen)
      if cp >= 0 && cp < docLen {
        // java if (prevCP >= 0 && prevCP < docLen)
        if prevCP >= 0 && prevCP < docLen {
          // java if (!Character.isLowSurrogate(textRope.charAt(cp)))
          if !Character::isLowSurrogate(textRope.charAt(cp)) {
            // java return false;
            return false;
          }
          // java else if (!Character.isHighSurrogate(textRope.charAt(prevCP)))
          else if !Character::isHighSurrogate(textRope.charAt(prevCP)) {
            // java return false;
            return false;
          }
          // java else
          else {
            // java IChpxTable.Entry runEntry = doc.getChpxTable().seek(prevCP);
            let mut runEntry: Box<dyn ChpxEntry> = chpx_seek(doc.getChpxTable().expect("getChpxTable"), prevCP).expect("runEntry");
            // java return prevCP + 1 < runEntry.endCp();
            return prevCP + 1 < runEntry.endCp();
          }
        }
        // java else
        else {
          // java return false;
          return false;
        }
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java public static boolean isParagraphBreak(Document document, IChpxTable.Entry run)
    pub fn isParagraphBreak(document: &mut dyn Document, run: &mut dyn ChpxEntry) -> bool {
      // java if (RANGEFactory.getLength(run.range()) == 1)
      if RANGEFactory::getLength(run.range()) == 1 {
        // java char c = document.charAt(run.position());
        let mut c: char = document.charAt(run.position());
        // java return ControlChar.isParagraphBreak(c);
        return ControlChar::isParagraphBreak(c);
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java public static boolean isRevisionDelete(PropertySet data)
    pub fn isRevisionDelete__propertyset(data: &PropertySet) -> bool {
      // java Object revision = data.getValue(45);
      // rust: 后接 `// java (null != revision)` ⇒ 保持 `Option`。
      let mut revision = data.getValue(45);
      let mut fRevisionDelete: bool = data.getBoolean__int_boolean(54, false);
      // java return null != revision && fRevisionDelete;
      return revision.is_some() && fRevisionDelete;
    }

    // java public static boolean isRevisionDelete(Document document, int startCP, int endCP)
    pub fn isRevisionDelete__document_int_int(document: &dyn Document, startCP: i32, endCP: i32) -> bool {
      // java IChpxTable chpxTable = document.getChpxTable();
      // rust: 门面返回擦除指针 `Option<*mut dyn IChpxTable>` ⇒ 保持裸指针并解引用。
      let mut chpxTable: *mut dyn IChpxTable = document.getChpxTable().expect("getChpxTable");
      // java IChpxTable.Entry start = chpxTable.seek(startCP);
      let mut start: Box<dyn ChpxEntry> = chpx_seek(chpxTable, startCP).expect("start");
      // java IChpxTable.Entry end = chpxTable.seek(endCP);
      let mut end: Box<dyn ChpxEntry> = IChpxTable::seek(unsafe { &*chpxTable }, endCP).expect("end");
      // java if (start == end)
      if chpx_eq(&*start, &*end) {
        // java return isRunDelete(start);
        return Self::isRunDelete__entry(&*start);
      }
      // java else
      else {
        // java if (!end.isEnd() && end.position() != endCP)
        if !end.isEnd() && end.position() != endCP {
          // java end = end.getNext();
          end = ChpxEntry::getNext(&*end).expect("getNext");
        }
        // java boolean runDelete;
        let mut runDelete: bool;
        runDelete = false;
        // java for(runDelete = false start != end runDelete = true)
        while !chpx_eq(&*start, &*end) {
          // java if (!isRunDelete(start))
          if !Self::isRunDelete__entry(&*start) {
            // java runDelete = false;
            runDelete = false;
            // java break;
            break;
          }
          // java start = start.getNext();
          start = ChpxEntry::getNext(&*start).expect("getNext");
        }
        // java return runDelete;
        return runDelete;
      }
    }

    // java public static int getShapeFieldTripeStartCP(Document doc, int cp)
    pub fn getShapeFieldTripeStartCP(doc: &dyn Document, cp: i32) -> i32 {
      // java PLCField.FieldTriple triple = doc.getPLCField().expect("getPLCField").getContainingField(cp).expect("field");
      let mut triple = field_mut(doc).and_then(|f| f.getContainingField(cp));
      // java return null != triple ? triple.getStartCP() : cp;
      return if let Some(ref t) = triple { t.getStartCP() } else { cp };
    }

    // java public static void dealWithShapeAnchor(Document document, RANGE range, boolean enable)
    pub fn dealWithShapeAnchor(document: &mut dyn Document, range: &mut RANGE, enable: bool) {
      // java int startCp = range.start;
      let mut startCp: i32 = range.start;
      // java IPapxTable papxTable = document.getPapxTable();
      // rust: 门面返回擦除指针 `Option<*mut dyn IPapxTable>` ⇒ 保持裸指针并解引用。
      let mut papxTable: *mut dyn IPapxTable = document.getPapxTable().expect("getPapxTable");
      // java IPapxTable.Entry start = papxTable.seek(range.start);
      let mut start: Box<dyn PapxEntry> = IPapxTable::seek(unsafe { &*papxTable }, range.start).expect("start");
      // java IPapxTable.Entry end = papxTable.seek(range.end);
      let mut end: Box<dyn PapxEntry> = papx_seek(papxTable, range.end).expect("end");
      // java if (enable)
      if enable {
        // java int minCp = start.position();
        let mut minCp: i32 = start.position();
        // java if (start == end || startCp == minCp)
        if papx_eq(&*start, &*end) || startCp == minCp {
          // java return;
          return;
        }
        // java do (minCp < startCp)
        loop {
            // java --startCp;
            { let _tmp = startCp; startCp -= 1; _tmp };
            // java char c = document.charAt(startCp);
            let mut c: char = document.charAt(startCp);
            // java if (c != '\b')
            if c != '\u{0008}' {
              // java break;
              break;
            }
            // java --range.start;
            { let _tmp = range.start; range.start -= 1; _tmp };
        }
      }
      // java else
      else {
        // java if (startCp != start.position())
        if startCp != start.position() {
          // java return;
          return;
        }
        // java int maxCp = Math.min(start.endCp(), range.end);
        let mut maxCp: i32 = Math::min(start.endCp(), range.end);
        // java do (startCp < maxCp)
        loop {
            // java char c = document.charAt(startCp);
            let mut c: char = document.charAt(startCp);
            // java if (c != '\b')
            if c != '\u{0008}' {
              // java break;
              break;
            }
            // java ++startCp;
            { let _tmp = startCp; startCp += 1; _tmp };
            // java ++range.start;
            { let _tmp = range.start; range.start += 1; _tmp };
        }
      }
    }

    // java public static boolean isEQField(Document document, int start)
    pub fn isEQField(document: &dyn Document, start: i32) -> bool {
      // java PLCField.FieldTriple field = document.getPLCField().expect("getPLCField").getContainingField(start).expect("field");
      let mut field = field_mut(document).and_then(|f| f.getContainingField(start));
      // java return field != null && KFieldType.FieldEQ.getFlt() == field.getFieldType();
      return field.as_ref().map(|f| KFieldType::FieldEQ.getFlt() == f.getFieldType()).unwrap_or(false);
    }

    // java public static boolean splitDeleteWhenCrossParas(Document document, int start, int end)
    pub fn splitDeleteWhenCrossParas(document: &dyn Document, start: i32, end: i32) -> bool {
      // java IPapxTable papxTable = document.getPapxTable();
      // rust: 门面返回擦除指针 `Option<*mut dyn IPapxTable>` ⇒ 保持裸指针并解引用。
      let mut papxTable: *mut dyn IPapxTable = document.getPapxTable().expect("getPapxTable");
      // java IPapxTable.Entry s = papxTable.seek(start);
      let mut s: Box<dyn PapxEntry> = IPapxTable::seek(unsafe { &*papxTable }, start).expect("s");
      // java IPapxTable.Entry e = papxTable.seek(end);
      let mut e: Box<dyn PapxEntry> = papx_seek(papxTable, end).expect("e");
      // java if (e.isEnd())
      if e.isEnd() {
        // java e = e.getPrev();
        e = PapxEntry::getPrev(&*e).expect("getPrev");
      }
      // java Assert.assertTrue("s should not be the end of the papxTable", !s.isEnd());
      Assert::assertTrue__string_boolean(&"s should not be the end of the papxTable", !s.isEnd());
      // java if (s != e)
      if !papx_eq(&*s, &*e) {
        // java int endCp = s.endCp();
        let mut endCp: i32 = s.endCp();
        // java if (start > s.position() && start < endCp - 1)
        if start > s.position() && start < endCp - 1 {
          // java if (end == endCp)
          if end == endCp {
            // java return false;
            return false;
          }
          // java return true;
          return true;
        }
      }
      // java return false;
      return false;
    }

    // java public static int getTargetHightLightColor(Document document, int cp)
    pub fn getTargetHightLightColor(document: &dyn Document, cp: i32) -> i32 {
      // java int targetColor = 0;
      let mut targetColor: i32 = 0;
      // java IChpxTable chpxTable = document.getChpxTable();
      // rust: 门面返回擦除指针 `Option<*mut dyn IChpxTable>` ⇒ 保持裸指针并解引用。
      let mut chpxTable: *mut dyn IChpxTable = document.getChpxTable().expect("getChpxTable");
      // java IPropertyCalculate tool = PropertyCalculateFactory.create(document);
      // rust: Factory/Impl 未登记；`getChp(chpx, 15, 1)` 取值键为 pid 15。
      let mut chpx: Box<dyn ChpxEntry> = chpx_seek(chpxTable, cp).expect("chpx");
      if !chpx.isEnd() {
        // java Object prop = tool.getChp(chpx, 15, 1);
        let mut prop = ChpxEntry::getProperty(&*chpx).and_then(|ps| ps.getValue(15)).and_then(|v| v.as_ref().as_any().downcast_ref::<i32>().copied());
        if let Some(c) = prop {
          targetColor = c;
        }
      }
      // java return targetColor;
      return targetColor;
    }

    // java public static boolean isVerticalTextForMainDocCP(Document document, int mainCP)
    pub fn isVerticalTextForMainDocCP__document_int(document: &dyn Document, mainCP: i32) -> bool {
      // java if (document.getType() != 0)
      if document.getType() != 0 {
        // java return false;
        return false;
      }
      // java else
      else {
        // java PLCSection sections = document.getPLCSection();
        let mut sections: &PLCSection = plc_as_mut::<PLCSection>(document.getPLCSection()).expect("getPLCSection");
        // java PLCSection.SectionNode sectionNode = sections.getSection(mainCP);
        // rust: 后接 `// java (null != sectionNode)` ⇒ 保持 `Option`。
        let mut sectionNode: Option<crate::cn::wps::moffice::writer::data::PLCSection::SectionNode> = sections.getSection(mainCP);
        return if let Some(ref sn) = sectionNode { Self::isVerticalTextForMainDocCP__sectionnode(Some(sn)) } else { false };
      }
    }

    // java public static boolean isVerticalTextForMainDocCP(PLCSection.SectionNode sectionNode)
    pub fn isVerticalTextForMainDocCP__sectionnode(sectionNode: Option<&crate::cn::wps::moffice::writer::data::PLCSection::SectionNode>) -> bool {
      // java if (null != sectionNode)
      if let Some(sectionNode) = sectionNode {
        // java int textFlow = sectionNode.getPropertySet().getInt(714, DefaultValue.section_TextFlow);
        let mut textFlow: i32 = sectionNode.getPropertySet().getInt__int_int(714, section_TextFlow.load(std::sync::atomic::Ordering::SeqCst));
        // java if (0 != textFlow && 4 != textFlow)
        if 0 != textFlow && 4 != textFlow {
          // java return true;
          return true;
        }
      }
      // java return false;
      return false;
    }

    // java public static PLCSection.SectionNode getSectionNode(Document document, int mainCP)
    pub fn getSectionNode(document: &dyn Document, mainCP: i32) -> Option<crate::cn::wps::moffice::writer::data::PLCSection::SectionNode> {
      // java if (document.getType() != 0)
      if document.getType() != 0 {
        // java return null;
        return None;
      }
      // java else
      else {
        // java PLCSection sections = document.getPLCSection();
        let mut sections: &PLCSection = plc_as_mut::<PLCSection>(document.getPLCSection()).expect("getPLCSection");
        // java return sections.getSection(mainCP);
        return sections.getSection(mainCP);
      }
    }

    // java public static int expandToSelectField(Document document, int cp, boolean isStart)
    pub fn expandToSelectField__document_int_boolean(document: &dyn Document, cp: i32, isStart: bool) -> i32 {
      // java int checkCp = isStart ? cp : Math.max(cp - 1, 0);
      let mut checkCp: i32 = (if isStart { cp } else { Math::max(cp - 1, 0) });
      // java PLCField.FieldTriple field = document.getPLCField().getMaxMatchCpField(checkCp);
      // rust: 后接 `// java (null != field)` ⇒ 保持 `Option`。
      let mut field = field_mut(document).and_then(|f| f.getMaxMatchCpField(checkCp));
      if field.is_some() {
        // java return isStart ? field.getStartCP() : field.getEnd();
        return if isStart { field.as_ref().unwrap().getStartCP() } else { field.as_ref().unwrap().getEnd() };
      }
      // java else
      else {
        // java return cp;
        return cp;
      }
    }

}














