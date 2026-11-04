// Java: import cn.wps.base.io.ControlChar
use util::base::io::ControlChar::*;
// Java: import cn.wps.collections.textrope.ITextRope
use crate::cn::wps::collections::textrope::ITextRope::*;
// Java: import cn.wps.moffice.drawing.GroupShape
use drawing::cn::wps::moffice::drawing::GroupShape::*;
// Java: import cn.wps.moffice.drawing.IDiagram
use drawing::cn::wps::moffice::drawing::IDiagram::*;
// Java: import cn.wps.moffice.drawing.IDrawingContainer
use drawing::cn::wps::moffice::drawing::IDrawingContainer::*;
// Java: import cn.wps.moffice.drawing.OLE
use drawing::cn::wps::moffice::drawing::OLE::*;
// Java: import cn.wps.moffice.drawing.Shape
use drawing::cn::wps::moffice::drawing::Shape::Shape;
// Java: import cn.wps.moffice.drawing.chart.IChart
use drawing::cn::wps::moffice::drawing::chart::IChart::*;
// Java: import cn.wps.moffice.drawing.graphics.BlipFill
use drawing::cn::wps::moffice::drawing::graphics::BlipFill::*;
// Java: import cn.wps.moffice.drawing.graphics.Picture
use drawing::cn::wps::moffice::drawing::graphics::Picture::*;
// Java: import cn.wps.moffice.drawing.media.IMediaLib
use drawing::cn::wps::moffice::drawing::media::IMediaLib::*;
// Java: import cn.wps.moffice.drawing.media.MediaItem
use drawing::cn::wps::moffice::drawing::media::MediaItem::*;
// Java: import cn.wps.moffice.drawing.media.MediaTypeEnum
use drawing::cn::wps::moffice::drawing::media::MediaTypeEnum::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KStyle
use crate::cn::wps::moffice::writer::core::KStyle::*;
// Java: import cn.wps.moffice.writer.core.KStyles
use crate::cn::wps::moffice::writer::core::KStyles::*;
// Java: import cn.wps.moffice.writer.core.SubDocument
use crate::cn::wps::moffice::writer::core::SubDocument::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::*;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::*;
// Java: import cn.wps.moffice.writer.data.PLCBookmarkEnd
use crate::cn::wps::moffice::writer::data::PLCBookmarkEnd::*;
// Java: import cn.wps.moffice.writer.data.PLCBookmarkStart
use crate::cn::wps::moffice::writer::data::PLCBookmarkStart::*;
// Java: import cn.wps.moffice.writer.data.PLCCommentRef
use crate::cn::wps::moffice::writer::data::PLCCommentRef::*;
// Java: import cn.wps.moffice.writer.data.PLCCommentTxt
use crate::cn::wps::moffice::writer::data::PLCCommentTxt::*;
// Java: import cn.wps.moffice.writer.data.PLCField
use crate::cn::wps::moffice::writer::data::PLCField::*;
// Java: import cn.wps.moffice.writer.data.PLCFootnoteEndnoteRef
use crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteRef::*;
// Java: import cn.wps.moffice.writer.data.PLCFootnoteEndnoteTxt
use crate::cn::wps::moffice::writer::data::PLCFootnoteEndnoteTxt::*;
// Java: import cn.wps.moffice.writer.data.PLCMathEnd
use crate::cn::wps::moffice::writer::data::PLCMathEnd::*;
// Java: import cn.wps.moffice.writer.data.PLCMathFunction
use crate::cn::wps::moffice::writer::data::PLCMathFunction::*;
// Java: import cn.wps.moffice.writer.data.PLCMathStart
use crate::cn::wps::moffice::writer::data::PLCMathStart::*;
// Java: import cn.wps.moffice.writer.data.PLCShapeAnchor
use crate::cn::wps::moffice::writer::data::PLCShapeAnchor::*;
// Java: import cn.wps.moffice.writer.data.PropertyId
use crate::cn::wps::moffice::writer::data::PropertyId::*;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::*;
// Java: import cn.wps.moffice.writer.data.TableTool
use crate::cn::wps::moffice::writer::data::TableTool::*;
// Java: import cn.wps.moffice.writer.data.autonum.AutoNumTable
use crate::cn::wps::moffice::writer::data::autonum::AutoNumTable::*;
// Java: import cn.wps.moffice.writer.data.autonum.LfoData
use crate::cn::wps::moffice::writer::data::autonum::LfoData::*;
// Java: import cn.wps.moffice.writer.data.autonum.LfoDataTable
use crate::cn::wps::moffice::writer::data::autonum::LfoDataTable::*;
// Java: import cn.wps.moffice.writer.data.autonum.LstData
use crate::cn::wps::moffice::writer::data::autonum::LstData::*;
// Java: import cn.wps.moffice.writer.data.autonum.LstDataTable
use crate::cn::wps::moffice::writer::data::autonum::LstDataTable::*;
// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.List
// Java: import java.util.Vector
use stubs::java::util::Vector::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
// Java: import cn.wps.moffice.writer.core.CloneNotSupportedException
use stubs::java::lang::CloneNotSupportedException::*;

// java public abstract class CopyDataBase
#[derive(Default)]
pub struct CopyDataBase {
}

// java public abstract class CopyDataBase
impl CopyDataBase {
    // java protected void copyTextRope(Document srcDocument, Document desDocument, int destCp, long srcRange)
    pub fn copyTextRope(&mut self, srcDocument: &dyn Document, desDocument: &dyn Document, destCp: i32, srcRange: i64) {
      // java int start = RANGEFactory.getStart(srcRange);
      let mut start: i32 = RANGEFactory::getStart(srcRange);
      // java int end = RANGEFactory.getEnd(srcRange);
      let mut end: i32 = RANGEFactory::getEnd(srcRange);
      // java int len = end - start;
      let mut len: i32 = end - start;
      // java ITextRope srcTextRope = srcDocument.getTextRope();
      let mut srcTextRope: Box<dyn ITextRope> = srcDocument.getTextRope();
      // java ITextRope destTextRope = desDocument.getTextRope();
      let mut destTextRope: Box<dyn ITextRope> = desDocument.getTextRope();
      // java char[] buffer = new char[len];
      let mut buffer: Vec<char> = vec![Default::default(); (len) as usize];
      // java srcTextRope.getText(start, start + len, buffer, 0);
      srcTextRope.getText(start, start + len, buffer, 0);
      // java destTextRope.insert(destCp, (char[])buffer, 0, len);
      destTextRope.insert(destCp, todo!("cast → Vec<char>"), 0, len);
    }

    // java public void copyCPTable(Document srcDocument, Document desDocument, int destCP, long srcRange)
    pub fn copyCPTable(&mut self, srcDocument: &dyn Document, desDocument: &dyn Document, destCP: i32, srcRange: i64) {
      // java int startCP = RANGEFactory.getStart(srcRange);
      let mut startCP: i32 = RANGEFactory::getStart(srcRange);
      // java int endCP = RANGEFactory.getEnd(srcRange);
      let mut endCP: i32 = RANGEFactory::getEnd(srcRange);
      // java IChpxTable srcChpx = srcDocument.getChpxTable();
      let mut srcChpx: Box<dyn IChpxTable> = srcDocument.getChpxTable();
      // java IChpxTable destChpx = desDocument.getChpxTable();
      let mut destChpx: Box<dyn IChpxTable> = desDocument.getChpxTable();
      // java IPapxTable srcPapx = srcDocument.getPapxTable();
      let mut srcPapx: Box<dyn IPapxTable> = srcDocument.getPapxTable();
      // java IPapxTable destPapx = desDocument.getPapxTable();
      let mut destPapx: Box<dyn IPapxTable> = desDocument.getPapxTable();
      // java int len = endCP - startCP;
      let mut len: i32 = endCP - startCP;
      // java boolean fAppendParagraph = this.needAppendParagraph(srcDocument, endCP);
      let mut fAppendParagraph: bool = self.needAppendParagraph(&mut srcDocument, endCP);
      // java int insertCP = fAppendParagraph ? destCP + len + 1 : destCP + len;
      let mut insertCP: i32 = (if fAppendParagraph { destCP + len + 1 } else { destCP + len });
      // java IChpxTable.Entry chEntry = destChpx.seek(destCP);
      let mut chEntry: Box<dyn Entry> = destChpx.seek(destCP);
      // java if (chEntry.position() == destCP)
      if chEntry.position() == destCP {
        // java PropertySet property = chEntry.getProperty();
        let mut property: PropertySet = chEntry.getProperty();
        // java destChpx.remove(chEntry);
        destChpx.remove(chEntry);
        // java destChpx.insert(insertCP, property);
        destChpx.insert(insertCP, property);
      }
      // java IPapxTable.Entry paEntry = destPapx.seek(destCP);
      let mut paEntry: Box<dyn Entry> = destPapx.seek(destCP);
      // java int destLevel = paEntry.isEnd() ? 0 : TableTool.getTableLevel(desDocument, paEntry);
      let mut destLevel: i32 = (if paEntry.isEnd() { 0 } else { TableTool::getTableLevel(&desDocument, &paEntry) });
      // java if (paEntry.position() == destCP)
      if paEntry.position() == destCP {
        // java PropertySet property = paEntry.getProperty();
        let mut property: PropertySet = paEntry.getProperty();
        // java destPapx.remove(paEntry);
        destPapx.remove(paEntry);
        // java destPapx.insert(insertCP, property);
        destPapx.insert(insertCP, property);
      }
      // java if (fAppendParagraph)
      if fAppendParagraph {
        // java IPapxTable.Entry paraEnd = srcPapx.seek(endCP);
        let mut paraEnd: Box<dyn Entry> = srcPapx.seek(endCP);
        // java char anchor = this.getParaEndChar(srcDocument, paraEnd);
        let mut anchor: char = self.getParaEndChar(&mut srcDocument, &mut paraEnd);
        // java char[] buffer = new char[]{anchor};
        let mut buffer: Vec<char> = vec![anchor];
        // java desDocument.getTextRope().insert(destCP + len + 1, (char[])buffer, 0, 1);
        desDocument.getTextRope().insert(destCP + len + 1, todo!("cast → Vec<char>"), 0, 1);
        // java IChpxTable.Entry runEntry = srcChpx.seek(endCP);
        let mut runEntry: Box<dyn Entry> = srcChpx.seek(endCP);
        // java PropertySet runProp = this.correctRunStyle(runEntry.getProperty());
        let mut runProp: PropertySet = self.correctRunStyle(&runEntry.getProperty());
        // java destChpx.insert(destCP + len, runProp);
        destChpx.insert(destCP + len, runProp);
      }
      // java IChpxTable.Entry runEntry = srcChpx.seek(startCP);
      let mut runEntry: Box<dyn Entry> = srcChpx.seek(startCP);
      // java IChpxTable.Entry runEnd = srcChpx.seek(endCP);
      let mut runEnd: Box<dyn Entry> = srcChpx.seek(endCP);
      // java if (runEntry == runEnd)
      if runEntry == runEnd {
        // java runEnd = runEnd.getNext();
        runEnd = runEnd.getNext();
      }
      // java insertCP = destCP;
      insertCP = destCP;
      let mut isFirst: bool = true;
      // java for(boolean isFirst = true; runEntry != runEnd isFirst = false)
      while runEntry != runEnd {
        // java PropertySet property = this.correctRunStyle(runEntry.getProperty());
        let mut property: PropertySet = self.correctRunStyle(&runEntry.getProperty());
        // java destChpx.insert(insertCP, property);
        destChpx.insert(insertCP, property);
        // java int distance = isFirst ? runEntry.endCp() - startCP : runEntry.length();
        let mut distance: i32 = (if isFirst { runEntry.endCp() - startCP } else { (runEntry.len() as i32) });
        // java insertCP += distance;
        insertCP = insertCP + distance;
        // java runEntry = runEntry.getNext();
        runEntry = runEntry.getNext();
      }
      // java IPapxTable.Entry paraEntry = srcPapx.seek(startCP);
      let mut paraEntry: Box<dyn Entry> = srcPapx.seek(startCP);
      // java IPapxTable.Entry paraEnd = srcPapx.seek(endCP);
      let mut paraEnd: Box<dyn Entry> = srcPapx.seek(endCP);
      // java if (fAppendParagraph)
      if fAppendParagraph {
        // java paraEnd = paraEnd.getNext();
        paraEnd = paraEnd.getNext();
      }
      // java insertCP = destCP;
      insertCP = destCP;
      let mut var36: bool = true;
      // java for(boolean var36 = true; paraEntry != paraEnd var36 = false)
      while paraEntry != paraEnd {
        // java PropertySet property = this.correctParagraphStyle(paraEntry.getProperty());
        let mut property: PropertySet = self.correctParagraphStyle(&paraEntry.getProperty());
        // java destPapx.insert(insertCP, property);
        destPapx.insert(insertCP, property);
        // java int distance = var36 ? paraEntry.endCp() - startCP : paraEntry.length();
        let mut distance: i32 = (if var36 { paraEntry.endCp() - startCP } else { (paraEntry.len() as i32) });
        // java insertCP += distance;
        insertCP = insertCP + distance;
        // java paraEntry = paraEntry.getNext();
        paraEntry = paraEntry.getNext();
      }
      // java boolean fCopyTable = TableTool.isInTable(srcDocument, startCP);
      let mut fCopyTable: bool = TableTool::isInTable__propertyset_int(&srcDocument, startCP);
      // java if (fCopyTable)
      if fCopyTable {
        // java this.correctTableInfo(srcDocument, desDocument, destCP, destLevel, srcRange);
        self.correctTableInfo(&srcDocument, &desDocument, destCP, destLevel, srcRange);
      }
    }

    // java private void correctTableInfo(Document srcDocument, Document desDocument, int destCP, int desLevel, long srcRange)
    pub fn correctTableInfo(&mut self, srcDocument: &dyn Document, desDocument: &dyn Document, destCP: i32, desLevel: i32, srcRange: i64) {
      // java int startCP = RANGEFactory.getStart(srcRange);
      let mut startCP: i32 = RANGEFactory::getStart(srcRange);
      // java int endCP = RANGEFactory.getEnd(srcRange);
      let mut endCP: i32 = RANGEFactory::getEnd(srcRange);
      // java IPapxTable srcPapx = srcDocument.getPapxTable();
      let mut srcPapx: Box<dyn IPapxTable> = srcDocument.getPapxTable();
      // java IPapxTable.Entry srcParaEntry = srcPapx.seek(startCP);
      let mut srcParaEntry: Box<dyn Entry> = srcPapx.seek(startCP);
      // java IPapxTable.Entry srcParaEndEntry = srcPapx.seek(endCP - 1);
      let mut srcParaEndEntry: Box<dyn Entry> = srcPapx.seek(endCP - 1);
      // java int srcStartLevel = srcParaEntry.isEnd() ? 0 : TableTool.getTableLevel(srcDocument, srcParaEntry);
      let mut srcStartLevel: i32 = (if srcParaEntry.isEnd() { 0 } else { TableTool::getTableLevel(&srcDocument, &srcParaEntry) });
      // java int srcEndLevel = srcParaEndEntry.isEnd() ? 0 : TableTool.getTableLevel(srcDocument, srcParaEndEntry);
      let mut srcEndLevel: i32 = (if srcParaEndEntry.isEnd() { 0 } else { TableTool::getTableLevel(&srcDocument, &srcParaEndEntry) });
      // java boolean fInSameLevel = srcStartLevel == srcEndLevel;
      let mut fInSameLevel: bool = srcStartLevel == srcEndLevel;
      // java IPapxTable destPapx = desDocument.getPapxTable();
      let mut destPapx: Box<dyn IPapxTable> = desDocument.getPapxTable();
      // java IPapxTable.Entry desParaEntry = destPapx.seek(destCP);
      let mut desParaEntry: Box<dyn Entry> = destPapx.seek(destCP);
      // java IPapxTable.Entry desParaEnd = destPapx.seek(destCP + endCP - startCP);
      let mut desParaEnd: Box<dyn Entry> = destPapx.seek(destCP + endCP - startCP);
      // java if (this.needAppendParagraph(srcDocument, endCP))
      if self.needAppendParagraph(&mut srcDocument, endCP) {
        // java desParaEnd = desParaEnd.getNext();
        desParaEnd = desParaEnd.getNext();
      }
      // java desDocument.getTextDocument().setInIOState(true);
      desDocument.getTextDocument().setInIOState(true);
      // java if (fInSameLevel)
      if fInSameLevel {
        // java boolean isRowEnd = TableTool.isRowEnd(srcDocument, endCP - 1);
        let mut isRowEnd: bool = TableTool::isRowEnd__document_int(&mut srcDocument, endCP - 1);
        // java int newLevel = 0;
        let mut newLevel: i32 = 0;
        let mut startLevel: i32 = (if isRowEnd { 1 } else { 0 });
        // java for(int startLevel = isRowEnd ? 1 : 0; desParaEntry != desParaEnd desParaEntry = desParaEntry.getNext())
        while desParaEntry != desParaEnd {
          // java PropertySet property = desParaEntry.getProperty();
          let mut property: PropertySet = desParaEntry.getProperty();
          // java int curSrcLevel = property.getInt(224, 0);
          let mut curSrcLevel: i32 = property.getInt(224, 0);
          // java if (curSrcLevel == srcStartLevel)
          if curSrcLevel == srcStartLevel {
            // java newLevel = startLevel;
            newLevel = startLevel;
          }
          // java else
          else {
            // java newLevel = startLevel + Math.abs(curSrcLevel - srcStartLevel);
            newLevel = startLevel + Math::abs(curSrcLevel - srcStartLevel);
          }
          // java this.correctTableProp(desParaEntry, newLevel + desLevel);
          self.correctTableProp(&mut desParaEntry, newLevel + desLevel);
          // java this.correctTableTextRope(desDocument, desParaEntry);
          self.correctTableTextRope(&mut desDocument, &mut desParaEntry);
        }
      }
      // java else
      else {
        // java int newLevel = 0;
        let mut newLevel: i32 = 0;
        let mut minLevel: i32 = Math::min(srcStartLevel, srcEndLevel);
        // java for(int minLevel = Math.min(srcStartLevel, srcEndLevel); desParaEntry != desParaEnd desParaEntry = desParaEntry.getNext())
        while desParaEntry != desParaEnd {
          // java PropertySet property = desParaEntry.getProperty();
          let mut property: PropertySet = desParaEntry.getProperty();
          // java int curSrcLevel = property.getInt(224, 0);
          let mut curSrcLevel: i32 = property.getInt(224, 0);
          // java newLevel = curSrcLevel - minLevel;
          newLevel = curSrcLevel - minLevel;
          // java this.correctTableProp(desParaEntry, newLevel + desLevel);
          self.correctTableProp(&mut desParaEntry, newLevel + desLevel);
          // java this.correctTableTextRope(desDocument, desParaEntry);
          self.correctTableTextRope(&mut desDocument, &mut desParaEntry);
        }
      }
      // java desDocument.getTextDocument().setInIOState(false);
      desDocument.getTextDocument().setInIOState(false);
    }

    // java protected void correctTableProp(IPapxTable.Entry paraEntry, int newLevel)
    pub fn correctTableProp(&mut self, paraEntry: &mut dyn Entry, newLevel: i32) {
      // java PropertySet property = paraEntry.getProperty();
      let mut property: PropertySet = paraEntry.getProperty();
      // java PropertySet newProp = this.addOrRemoveTableInfo(property, newLevel);
      let mut newProp: PropertySet = self.addOrRemoveTableInfo(&property, newLevel);
      // java paraEntry.setProperty(newProp);
      paraEntry.setProperty(newProp);
    }

    // java private void correctTableTextRope(Document document, IPapxTable.Entry paraEntry)
    pub fn correctTableTextRope(&mut self, document: &mut dyn Document, paraEntry: &mut dyn Entry) {
      // java PropertySet property = paraEntry.getProperty();
      let mut property: PropertySet = paraEntry.getProperty();
      // java int newLevel = property.getInt(224, 0);
      let mut newLevel: i32 = property.getInt(224, 0);
      // java if (newLevel == 1)
      if newLevel == 1 {
        // java ITextRope textRope = document.getTextRope();
        let mut textRope: Box<dyn ITextRope> = document.getTextRope();
        // java int curCp = paraEntry.endCp() - 1;
        let mut curCp: i32 = paraEntry.endCp() - 1;
        // java char curChar = document.charAt(curCp);
        let mut curChar: char = document.charAt(curCp);
        // java char newChar = curChar;
        let mut newChar: char = curChar;
        // java if (property.getBoolean(226, false) && 7 != curChar)
        if property.getBoolean(226, false) && 7 != curChar {
          // java newChar = 7;
          newChar = 7;
        }
        // java else if (property.getBoolean(225, false) && 7 != curChar)
        else if property.getBoolean(225, false) && 7 != curChar {
          // java newChar = 7;
          newChar = 7;
        }
        // java if (newChar != curChar)
        if newChar != curChar {
          // java textRope.insert(curCp, newChar);
          textRope.insert(curCp, newChar);
          // java textRope.erase(curCp + 1, curCp + 2);
          textRope.erase(curCp + 1, curCp + 2);
        }
      }
    }

    // java private char getParaEndChar(Document srcDocument, IPapxTable.Entry paraEnd)
    pub fn getParaEndChar(&self, srcDocument: &mut dyn Document, paraEnd: &mut dyn Entry) -> char {
      // java return TableTool.isCellEndOrRowEnd(srcDocument, paraEnd.endCp() - 1) ? '\r' : srcDocument.charAt(paraEnd.endCp() - 1);
      return (if TableTool::isCellEndOrRowEnd(&mut srcDocument, paraEnd.endCp() - 1) { '\r' } else { srcDocument.charAt(paraEnd.endCp() - 1) });
    }

    // java protected PropertySet addOrRemoveTableInfo(PropertySet property, int newLevel)
    pub fn addOrRemoveTableInfo(&mut self, property: &PropertySet, newLevel: i32) -> PropertySet {
      // java MutablePropertySet prop = new MutablePropertySet(property);
      let mut prop: MutablePropertySet = MutablePropertySet::new(property);
      // java if (newLevel >= 1)
      if newLevel >= 1 {
        // java prop.setBoolean(223, true);
        prop.setBoolean(223, true);
        // java prop.setInt(224, newLevel);
        prop.setInt(224, newLevel);
      }
      // java else
      else {
        // java for(int id : PropertyId.special_table_pid)
        for id in PropertyId::special_table_pid.iter() {
          // java Object value = property.getValue(id);
          let mut value: Box<dyn Object> = property.getValue(id);
          // java if (null != value)
          if value.is_some() {
            // java prop.remove(id);
            prop.remove(id);
          }
        }
      }
      // java return prop.freeze();
      return prop.freeze();
    }

    // java private boolean needAppendParagraph(Document srcDocument, int endCP)
    pub fn needAppendParagraph(&mut self, srcDocument: &mut dyn Document, endCP: i32) -> bool {
      // java int paraBreak = Math.max(0, endCP - 1);
      let mut paraBreak: i32 = Math::max(0, endCP - 1);
      // java char c = srcDocument.charAt(paraBreak);
      let mut c: char = srcDocument.charAt(paraBreak);
      // java return !ControlChar.isParagraphBreak(c);
      return !ControlChar::isParagraphBreak(c);
    }

    // java protected void copyField(Document srcDocument, Document desDocument, int destCP, long srcRange)
    pub fn copyField(&mut self, srcDocument: &dyn Document, desDocument: &dyn Document, destCP: i32, srcRange: i64) {
      // java int startCP = RANGEFactory.getStart(srcRange);
      let mut startCP: i32 = RANGEFactory::getStart(srcRange);
      // java int endCP = RANGEFactory.getEnd(srcRange);
      let mut endCP: i32 = RANGEFactory::getEnd(srcRange);
      // java PLCField srcField = srcDocument.getPLCField();
      let mut srcField: PLCField = srcDocument.getPLCField();
      // java PLCField destField = desDocument.getPLCField();
      let mut destField: PLCField = desDocument.getPLCField();
      let mut node: Box<dyn Node> = srcField.lowerBound(startCP);
      // java for(PLC.Node node = srcField.lowerBound(startCP); !node.isEnd() && node.getCP() < endCP node = node.getNext())
      while !node.isEnd() && node.getCP() < endCP {
        // java PLCField.FieldNode srcNode = (PLCField.FieldNode)node;
        let mut srcNode: PLCField::FieldNode = (node).downcast_ref::<PLCField::FieldNode>().unwrap();
        // java int distance = srcNode.getCP() - startCP;
        let mut distance: i32 = srcNode.getCP() - startCP;
        // java PLCField.FieldNode newNode = srcNode.clone();
        let mut newNode: PLCField::FieldNode = srcNode.clone();
        // java destField.insert(destCP + distance, newNode);
        destField.insert(destCP + distance, newNode);
      }
    }

    // java protected void copyBookmarks(Document srcDocument, Document desDocument, int destCP, long srcRange)
    pub fn copyBookmarks(&mut self, srcDocument: &dyn Document, desDocument: &dyn Document, destCP: i32, srcRange: i64) {
      // java int startCP = RANGEFactory.getStart(srcRange);
      let mut startCP: i32 = RANGEFactory::getStart(srcRange);
      // java int endCP = RANGEFactory.getEnd(srcRange);
      let mut endCP: i32 = RANGEFactory::getEnd(srcRange);
      // java PLCBookmarkStart destBookmarkStart = desDocument.getBookmarkStarts();
      let mut destBookmarkStart: PLCBookmarkStart = desDocument.getBookmarkStarts();
      // java PLCBookmarkEnd destBookmarkEnd = desDocument.getBookmarkEnds();
      let mut destBookmarkEnd: PLCBookmarkEnd = desDocument.getBookmarkEnds();
      // java PLCBookmarkStart srcBookmarkStart = srcDocument.getBookmarkStarts();
      let mut srcBookmarkStart: PLCBookmarkStart = srcDocument.getBookmarkStarts();
      let mut node: Box<dyn Node> = srcBookmarkStart.lowerBound(startCP);
      // java for(PLC.Node node = srcBookmarkStart.lowerBound(startCP); !node.isEnd() && node.getCP() < endCP node = node.getNext())
      while !node.isEnd() && node.getCP() < endCP {
        // java PLCBookmarkStart.BookmarkStartNode startNode = (PLCBookmarkStart.BookmarkStartNode)node;
        let mut startNode: PLCBookmarkStart::BookmarkStartNode = (node).downcast_ref::<PLCBookmarkStart::BookmarkStartNode>().unwrap();
        // java PLCBookmarkEnd.BookmarkEndNode endNode = startNode.getEnd();
        let mut endNode: PLCBookmarkEnd::BookmarkEndNode = startNode.getEnd();
        // java if (endCP < endNode.getCP())
        if endCP < endNode.getCP() {
          // java return;
          return;
        }
        // java int distance = node.getCP() - startCP;
        let mut distance: i32 = node.getCP() - startCP;
        // java PLCBookmarkStart.BookmarkStartNode newStartNode = destBookmarkStart.addBookmarkStart(destCP + distance);
        let mut newStartNode: PLCBookmarkStart::BookmarkStartNode = destBookmarkStart.addBookmarkStart(destCP + distance);
        // java distance = endNode.getCP() - startCP;
        distance = endNode.getCP() - startCP;
        // java PLCBookmarkEnd.BookmarkEndNode newEndNode = destBookmarkEnd.addBookmarkEnd(destCP + distance);
        let mut newEndNode: PLCBookmarkEnd::BookmarkEndNode = destBookmarkEnd.addBookmarkEnd(destCP + distance);
        // java newStartNode.setName(startNode.getName());
        newStartNode.setName(startNode.getName());
        // java newStartNode.setEnd(newEndNode);
        newStartNode.setEnd(newEndNode);
        // java newEndNode.setStart(newStartNode);
        newEndNode.setStart(newStartNode);
      }
    }

    // java protected void copyShapes(Document srcDocument, Document desDocument, int destCp, long srcRange)
    pub fn copyShapes(&mut self, srcDocument: &dyn Document, desDocument: &dyn Document, destCp: i32, srcRange: i64) {
      // java int startCP = RANGEFactory.getStart(srcRange);
      let mut startCP: i32 = RANGEFactory::getStart(srcRange);
      // java int endCP = RANGEFactory.getEnd(srcRange);
      let mut endCP: i32 = RANGEFactory::getEnd(srcRange);
      // java PLCShapeAnchor destPlc = desDocument.getShapeAnchors();
      let mut destPlc: PLCShapeAnchor = desDocument.getShapeAnchors();
      // java PLCShapeAnchor srcPlc = srcDocument.getShapeAnchors();
      let mut srcPlc: PLCShapeAnchor = srcDocument.getShapeAnchors();
      // java IDrawingContainer srcContainer = srcDocument.getDrawingContainer();
      let mut srcContainer: Box<dyn IDrawingContainer> = srcDocument.getDrawingContainer();
      let mut shapeNode: Box<dyn Node> = srcPlc.lowerBound(startCP);
      // java for(PLC.Node shapeNode = srcPlc.lowerBound(startCP); !shapeNode.isEnd() && shapeNode.getCP() < endCP shapeNode = shapeNode.getNext())
      while !shapeNode.isEnd() && shapeNode.getCP() < endCP {
        // java PLCShapeAnchor.ShapeAnchor shapeAnchor = (PLCShapeAnchor.ShapeAnchor)shapeNode;
        let mut shapeAnchor: PLCShapeAnchor::ShapeAnchor = (shapeNode).downcast_ref::<PLCShapeAnchor::ShapeAnchor>().unwrap();
        // java int shapeId = shapeAnchor.getShapeId();
        let mut shapeId: i32 = shapeAnchor.getShapeId();
        // java Shape srcShape = srcContainer.findShapeByID(shapeId);
        let mut srcShape: Shape = srcContainer.findShapeByID(shapeId);
        // java Shape newShape = this.cloneShape(srcShape, desDocument);
        let mut newShape: Shape = self.cloneShape(&mut srcShape, &desDocument);
        // java int distance = shapeNode.getCP() - startCP;
        let mut distance: i32 = shapeNode.getCP() - startCP;
        // java destPlc.addShapeAnchor(destCp + distance, newShape.getShapeID());
        destPlc.addShapeAnchor(destCp + distance, newShape.getShapeID());
      }
    }

    // java protected Shape cloneShape(Shape srcShape, Document destDocument)
    pub fn cloneShape(&mut self, srcShape: &mut Shape, destDocument: &dyn Document) -> Option<Shape> {
          // java IDrawingContainer destContainer = destDocument.getDrawingContainer();
          let mut destContainer: Box<dyn IDrawingContainer> = destDocument.getDrawingContainer();
          // java Shape newShape = srcShape.cloneAll(destContainer);
          let mut newShape: Shape = srcShape.cloneAll__idrawingcontainer(&mut destContainer);
          // java int newShapeId = newShape.getShapeID();
          let mut newShapeId: i32 = newShape.getShapeID();
          // java newShape.setDrawingContainer(destContainer);
          newShape.setDrawingContainer(destContainer);
          // java int shapeId = srcShape.getShapeID();
          let mut shapeId: i32 = srcShape.getShapeID();
          // java Document srcDocument = (Document)srcShape.getSubDocument();
          let mut srcDocument: Box<dyn Document> = todo!("cast → Box<dyn Document>");
          // java IDrawingContainer srcContainer = srcShape.getDrawingContainer();
          let mut srcContainer: Box<dyn IDrawingContainer> = srcShape.getDrawingContainer();
          // java IDiagram diagramDrawing = srcShape.getDiagramDrawingML();
          let mut diagramDrawing: Box<dyn IDiagram> = srcShape.getDiagramDrawingML();
          // java TextDocument textDocument = srcDocument.getTextDocument();
          let mut textDocument: TextDocument = srcDocument.getTextDocument();
          // java String drawingRid = (String)textDocument.getShapeId2DrawingRID().get(shapeId);
          let mut drawingRid: String = textDocument.getShapeId2DrawingRID().get(shapeId).to_string();
          // java if (null != drawingRid)
          if drawingRid.is_some() {
            // java textDocument.getShapeId2DrawingRID().put(newShapeId, drawingRid);
            textDocument.getShapeId2DrawingRID().put(newShapeId, drawingRid);
          }
          // java ArrayList<Integer> arrayList = textDocument.getDiagramShapeId(srcDocument.getType());
          let mut arrayList: Vec<i32> = textDocument.getDiagramShapeId(srcDocument.getType());
          // java if (arrayList.contains(shapeId))
          if arrayList.contains(shapeId) {
            // java textDocument.getDiagramShapeId(destDocument.getType()).add(newShapeId);
            textDocument.getDiagramShapeId(destDocument.getType()).add(newShapeId);
          }
          // java List<Shape> chartShapes = textDocument.getChartShapes();
          let mut chartShapes: Vec<Shape> = textDocument.getChartShapes();
          // java if (null != chartShapes && chartShapes.contains(srcShape))
          if chartShapes.is_some() && chartShapes.contains(srcShape) {
            // java chartShapes.add(newShape);
            chartShapes.add(newShape);
          }
          // java if (diagramDrawing != null && null != destContainer)
          if diagramDrawing.is_some() && destContainer.is_some() {
            // java diagramDrawing.setShapeIdx(newShapeId);
            diagramDrawing.setShapeIdx(newShapeId);
            // java destContainer.addDiagram(newShape, diagramDrawing);
            destContainer.addDiagram(newShape, diagramDrawing);
          }
          // java Shape chartShape = srcContainer.getChartShape(srcShape.getBaseShape());
          let mut chartShape: Shape = srcContainer.getChartShape(srcShape.getBaseShape());
          // java if (chartShape != null && destContainer != null)
          if chartShape.is_some() && destContainer.is_some() {
            // java destContainer.addChartShape(newShape, this.cloneShape(chartShape, destDocument));
            destContainer.addChartShape(newShape, self.cloneShape(&mut chartShape, &destDocument));
          }
          // java IChart chart = srcContainer.getChart(srcShape.getBaseShape());
          let mut chart: Box<dyn IChart> = srcContainer.getChart(srcShape.getBaseShape());
          // java if (chart != null && destContainer != null)
          if chart.is_some() && destContainer.is_some() {
            // java destContainer.addChart(newShape, chart);
            destContainer.addChart(newShape, chart);
          }
          // java destContainer.addShape(newShape);
          destContainer.addShape(newShape);
          // java this.copyMediaLib(srcShape, newShape, srcContainer, destContainer);
          self.copyMediaLib(&srcShape, &newShape, &srcContainer, &destContainer);
          // java return newShape;
          return newShape;
          // java CloneNotSupportedException var16
          todo!();
            // java return null;
            return None;
    }

    // java protected void copyMediaLib(Shape srcShape, Shape desShape, IDrawingContainer srcContainer, IDrawingContainer destContainer)
    pub fn copyMediaLib(&mut self, srcShape: &Shape, desShape: &Shape, srcContainer: &dyn IDrawingContainer, destContainer: &dyn IDrawingContainer) {
      // java if (srcContainer != destContainer)
      if srcContainer != destContainer {
        // java IMediaLib srcMediaLib = srcContainer.getMediaLib();
        let mut srcMediaLib: Box<dyn IMediaLib> = srcContainer.getMediaLib();
        // java IMediaLib desMediaLib = destContainer.getMediaLib();
        let mut desMediaLib: Box<dyn IMediaLib> = destContainer.getMediaLib();
        // java Picture srcPicture = srcShape.getPicture();
        let mut srcPicture: Picture = srcShape.getPicture();
        // java Picture desPicture = desShape.getPicture();
        let mut desPicture: Picture = desShape.getPicture();
        // java if (srcPicture != null)
        if srcPicture.is_some() {
          // java desPicture.setMediaID(this.genMediaItem(srcPicture.getMediaID(), MediaTypeEnum.PICTURE, srcMediaLib, desMediaLib));
          desPicture.setMediaID(self.genMediaItem(srcPicture.getMediaID(), &MediaTypeEnum::PICTURE, &srcMediaLib, &mut desMediaLib));
        }
        // java BlipFill srcBlipFill = srcShape.getBlipFill();
        let mut srcBlipFill: BlipFill = srcShape.getBlipFill();
        // java BlipFill desBlipFill = desShape.getBlipFill();
        let mut desBlipFill: BlipFill = desShape.getBlipFill();
        // java if (srcBlipFill != null)
        if srcBlipFill.is_some() {
          // java desBlipFill.setMediaID(this.genMediaItem(srcBlipFill.getMediaID(), MediaTypeEnum.PICTURE, srcMediaLib, desMediaLib));
          desBlipFill.setMediaID(self.genMediaItem(srcBlipFill.getMediaID(), &MediaTypeEnum::PICTURE, &srcMediaLib, &mut desMediaLib));
        }
        // java OLE srcOle = srcShape.getOLE();
        let mut srcOle: OLE = srcShape.getOLE();
        // java OLE desOle = desShape.getOLE();
        let mut desOle: OLE = desShape.getOLE();
        // java if (srcOle != null)
        if srcOle.is_some() {
          // java desOle.setMediaID(this.genMediaItem(srcOle.getMediaID(), MediaTypeEnum.OLE, srcMediaLib, desMediaLib));
          desOle.setMediaID(self.genMediaItem(srcOle.getMediaID(), &MediaTypeEnum::OLE, &srcMediaLib, &mut desMediaLib));
        }
        // java if (srcShape instanceof GroupShape)
        if srcShape.downcast_ref::<GroupShape>().is_some() {
          // java int srcChildCount = ((GroupShape)srcShape).getChildCount();
          let mut srcChildCount: i32 = (srcShape).downcast_ref::<GroupShape>().unwrap().getChildCount();
          // java int desChildCount = ((GroupShape)desShape).getChildCount();
          let mut desChildCount: i32 = (desShape).downcast_ref::<GroupShape>().unwrap().getChildCount();
          // java if (srcChildCount != desChildCount)
          if srcChildCount != desChildCount {
            // java return;
            return;
          }
          let mut i: i32 = 0;
          // java for(int i = 0; i < srcChildCount; ++i)
          while i < srcChildCount {
            // java Shape srcChildShape = ((GroupShape)srcShape).getChildShape(i);
            let mut srcChildShape: Shape = (srcShape).downcast_ref::<GroupShape>().unwrap().getChildShape(i);
            // java Shape desChildShape = ((GroupShape)desShape).getChildShape(i);
            let mut desChildShape: Shape = (desShape).downcast_ref::<GroupShape>().unwrap().getChildShape(i);
            // java this.copyMediaLib(srcChildShape, desChildShape, srcChildShape.getDrawingContainer(), desChildShape.getDrawingContainer());
            self.copyMediaLib(&srcChildShape, &desChildShape, &srcChildShape.getDrawingContainer(), &desChildShape.getDrawingContainer());
          }
        }
      }
    }

    // java private int genMediaItem(int srcMediaId, MediaTypeEnum mediaType, IMediaLib srcMediaLib, IMediaLib desMediaLib)
    pub fn genMediaItem(&mut self, srcMediaId: i32, mediaType: &MediaTypeEnum, srcMediaLib: &dyn IMediaLib, desMediaLib: &mut dyn IMediaLib) -> i32 {
      // java String path = srcMediaLib.getMediaByID(srcMediaId, mediaType);
      let mut path: String = srcMediaLib.getMediaByID(srcMediaId, &mediaType);
      // java int mediaId = desMediaLib.getMediaIDByName(path, mediaType);
      let mut mediaId: i32 = desMediaLib.getMediaIDByName(&path, &mediaType);
      // java if (mediaId == -1)
      if mediaId == -1 {
        // java mediaId = desMediaLib.addMedia(path, path, mediaType);
        mediaId = desMediaLib.addMedia__string_string_mediatypeenum(&path, &path, &mediaType);
      }
      // java MediaItem item = desMediaLib.getMediaByID2(mediaId, mediaType);
      let mut item: MediaItem = desMediaLib.getMediaByID2(mediaId, &mediaType);
      // java item.setShare(true);
      item.setShare(true);
      // java return mediaId;
      return mediaId;
    }

    // java protected void copyBKShape(Document srcDocument, Document desDocument)
    pub fn copyBKShape(&mut self, srcDocument: &dyn Document, desDocument: &dyn Document) {
      // java if (srcDocument != desDocument)
      if srcDocument != desDocument {
        // java IDrawingContainer srcContainer = srcDocument.getDrawingContainer();
        let mut srcContainer: Box<dyn IDrawingContainer> = srcDocument.getDrawingContainer();
        // java IDrawingContainer desContainer = desDocument.getDrawingContainer();
        let mut desContainer: Box<dyn IDrawingContainer> = desDocument.getDrawingContainer();
        // java if (srcContainer != null && desContainer != null)
        if srcContainer.is_some() && desContainer.is_some() {
          // java Vector<Shape> bkShapes = srcContainer.getBKShapes();
          let mut bkShapes: Vec<Shape> = srcContainer.getBKShapes();
          // java int i = 0;
          let mut i: i32 = 0;
          let mut count: i32 = (bkShapes.len() as i32);
          // java for(int count = bkShapes.size(); i < count; ++i)
          while i < count {
            // java Shape shape = (Shape)bkShapes.elementAt(i);
            let mut shape: Shape = (bkShapes.elementAt(i)).downcast_ref::<Shape>().unwrap();
            // java this.cloneShape(shape, desDocument);
            self.cloneShape(&mut shape, &desDocument);
          }
        }
      }
    }

    // java protected void copyMath(Document srcDocument, Document desDocument, int destCp, long srcRange)
    pub fn copyMath(&mut self, srcDocument: &dyn Document, desDocument: &mut dyn Document, destCp: i32, srcRange: i64) {
      // java int startCP = RANGEFactory.getStart(srcRange);
      let mut startCP: i32 = RANGEFactory::getStart(srcRange);
      // java int endCP = RANGEFactory.getEnd(srcRange);
      let mut endCP: i32 = RANGEFactory::getEnd(srcRange);
      // java PLCMathStart destPLCMathStart = desDocument.getOMathStarts();
      let mut destPLCMathStart: PLCMathStart = desDocument.getOMathStarts();
      // java PLCMathEnd destPLCMathEnd = desDocument.getOMathEnds();
      let mut destPLCMathEnd: PLCMathEnd = desDocument.getOMathEnds();
      // java PLCMathFunction destPLCMathFunction = desDocument.getPLCMathFunction();
      let mut destPLCMathFunction: PLCMathFunction = desDocument.getPLCMathFunction();
      // java PLCMathStart srcPLCMathStart = srcDocument.getOMathStarts();
      let mut srcPLCMathStart: PLCMathStart = srcDocument.getOMathStarts();
      // java if (null != srcPLCMathStart && !srcPLCMathStart.isEmpty())
      if srcPLCMathStart.is_some() && !srcPLCMathStart.is_empty() {
        // java if (null == destPLCMathStart)
        if destPLCMathStart.is_none() {
          // java desDocument.createPLCOMath();
          desDocument.createPLCOMath();
        }
        let mut node: Box<dyn Node> = srcPLCMathStart.lowerBound(startCP);
        // java for(PLC.Node node = srcPLCMathStart.lowerBound(startCP); !node.isEnd() && node.getCP() < endCP node = node.getNext())
        while !node.isEnd() && node.getCP() < endCP {
          // java PLCMathStart.OMathStartNode startNode = (PLCMathStart.OMathStartNode)node;
          let mut startNode: PLCMathStart::OMathStartNode = (node).downcast_ref::<PLCMathStart::OMathStartNode>().unwrap();
          // java int distance = startNode.getCP() - startCP;
          let mut distance: i32 = startNode.getCP() - startCP;
          // java PLCMathStart.OMathStartNode newStartNode = destPLCMathStart.addOMathStart(destCp + distance);
          let mut newStartNode: PLCMathStart::OMathStartNode = destPLCMathStart.addOMathStart(destCp + distance);
          // java PLCMathEnd.OMathEndNode endNode = startNode.getEnd();
          let mut endNode: PLCMathEnd::OMathEndNode = startNode.getEnd();
          // java distance = endNode.getCP() - startCP;
          distance = endNode.getCP() - startCP;
          // java PLCMathEnd.OMathEndNode newEndNode = destPLCMathEnd.addOMathEnd(destCp + distance);
          let mut newEndNode: PLCMathEnd::OMathEndNode = destPLCMathEnd.addOMathEnd(destCp + distance);
          // java newStartNode.setEnd(newEndNode);
          newStartNode.setEnd(newEndNode);
          // java newEndNode.setStart(newStartNode);
          newEndNode.setStart(newStartNode);
        }
      }
      // java PLCMathFunction srcPLCMathFunction = srcDocument.getPLCMathFunction();
      let mut srcPLCMathFunction: PLCMathFunction = srcDocument.getPLCMathFunction();
      // java if (null != srcPLCMathFunction && !srcPLCMathFunction.isEmpty())
      if srcPLCMathFunction.is_some() && !srcPLCMathFunction.is_empty() {
        // java if (null == destPLCMathFunction)
        if destPLCMathFunction.is_none() {
          // java desDocument.createPLCMath();
          desDocument.createPLCMath();
        }
        let mut node: Box<dyn Node> = srcPLCMathFunction.lowerBound(startCP);
        // java for(PLC.Node node = srcPLCMathFunction.lowerBound(startCP); !node.isEnd() && node.getCP() < endCP node = node.getNext())
        while !node.isEnd() && node.getCP() < endCP {
          // java PLCMathFunction.MathNode mathNode = (PLCMathFunction.MathNode)node;
          let mut mathNode: PLCMathFunction::MathNode = (node).downcast_ref::<PLCMathFunction::MathNode>().unwrap();
          // java if (mathNode.isBeginNode())
          if mathNode.isBeginNode() {
            // java PLCMathFunction.MathStartNode startNode = (PLCMathFunction.MathStartNode)mathNode;
            let mut startNode: PLCMathFunction::MathStartNode = (mathNode).downcast_ref::<PLCMathFunction::MathStartNode>().unwrap();
            // java int distance = startNode.getCP() - startCP;
            let mut distance: i32 = startNode.getCP() - startCP;
            // java PLCMathFunction.MathStartNode newStartNode = destPLCMathFunction.addMathStartNode(destCp + distance);
            let mut newStartNode: PLCMathFunction::MathStartNode = destPLCMathFunction.addMathStartNode(destCp + distance);
            // java newStartNode.setType(startNode.getType());
            newStartNode.setType(startNode.getType());
            // java newStartNode.setPropertySet(startNode.getPropertySet());
            newStartNode.setPropertySet(startNode.getPropertySet());
          }
          // java else
          else {
            // java int distance = mathNode.getCP() - startCP;
            let mut distance: i32 = mathNode.getCP() - startCP;
            // java destPLCMathFunction.addMathEndNode(destCp + distance);
            destPLCMathFunction.addMathEndNode(destCp + distance);
          }
        }
      }
    }

    // java protected void copyStyles(Document srcDocument, Document desDocument)
    pub fn copyStyles(&mut self, srcDocument: &dyn Document, desDocument: &dyn Document) {
      // java if (srcDocument != desDocument)
      if srcDocument != desDocument {
        // java KStyles srcStyles = srcDocument.getTextDocument().getStyles();
        let mut srcStyles: KStyles = srcDocument.getTextDocument().getStyles();
        // java KStyles destStyles = desDocument.getTextDocument().getStyles();
        let mut destStyles: KStyles = desDocument.getTextDocument().getStyles();
        // java for(KStyle style : srcStyles.getStyles())
        for style in srcStyles.getStyles().iter() {
          // java destStyles.add((KStyle)style.clone());
          destStyles.add((style.clone()).downcast_ref::<KStyle>().unwrap());
        }
      }
    }

    // java protected void copyNumbering(Document srcDocument, Document desDocument)
    pub fn copyNumbering(&mut self, srcDocument: &dyn Document, desDocument: &dyn Document) {
      // java if (srcDocument != desDocument)
      if srcDocument != desDocument {
        // java AutoNumTable srcAutoNumTable = srcDocument.getTextDocument().getAutoNumTable();
        let mut srcAutoNumTable: AutoNumTable = srcDocument.getTextDocument().getAutoNumTable();
        // java AutoNumTable destAutoNumTable = desDocument.getTextDocument().getAutoNumTable();
        let mut destAutoNumTable: AutoNumTable = desDocument.getTextDocument().getAutoNumTable();
        // java LstDataTable srcLstDataTable = srcAutoNumTable.getLstDataTable();
        let mut srcLstDataTable: LstDataTable = srcAutoNumTable.getLstDataTable();
        // java LstDataTable destLstDataTable = destAutoNumTable.getLstDataTable();
        let mut destLstDataTable: LstDataTable = destAutoNumTable.getLstDataTable();
        // java HashMap<Integer, LstData> srcLstMap = srcLstDataTable.getLstMap();
        let mut srcLstMap: HashMap<i32, LstData> = srcLstDataTable.getLstMap();
            // java for(Integer lsid : srcLstMap.keySet())
            for lsid in srcLstMap.keySet().iter() {
              // java destLstDataTable.add(((LstData)srcLstMap.get(lsid)).clone());
              destLstDataTable.add((srcLstMap.get(lsid)).downcast_ref::<LstData>().unwrap().clone());
            }
            // java CloneNotSupportedException e
            todo!();
              // java e.printStackTrace();
              e.printStackTrace();
        // java LfoDataTable srcLfoDataTable = srcAutoNumTable.getLfoDataTable();
        let mut srcLfoDataTable: LfoDataTable = srcAutoNumTable.getLfoDataTable();
        // java LfoDataTable destLfoDataTable = destAutoNumTable.getLfoDataTable();
        let mut destLfoDataTable: LfoDataTable = destAutoNumTable.getLfoDataTable();
        // java HashMap<Integer, LfoData> srcLfoMap = srcLfoDataTable.getLfoDataMap();
        let mut srcLfoMap: HashMap<i32, LfoData> = srcLfoDataTable.getLfoDataMap();
            // java for(Integer lfo : srcLfoMap.keySet())
            for lfo in srcLfoMap.keySet().iter() {
              // java destLfoDataTable.add(lfo, ((LfoData)srcLfoMap.get(lfo)).clone());
              destLfoDataTable.add(lfo, (srcLfoMap.get(lfo)).downcast_ref::<LfoData>().unwrap().clone());
            }
            // java CloneNotSupportedException e
            todo!();
              // java e.printStackTrace();
              e.printStackTrace();
      }
    }

    // java protected void copyDocumentProperty(Document srcDocument, Document desDocument)
    pub fn copyDocumentProperty(&mut self, srcDocument: &dyn Document, desDocument: &dyn Document) {
      // java if (srcDocument != desDocument)
      if srcDocument != desDocument {
        // java TextDocument srcTextDocument = srcDocument.getTextDocument();
        let mut srcTextDocument: TextDocument = srcDocument.getTextDocument();
        // java TextDocument desTextDocument = desDocument.getTextDocument();
        let mut desTextDocument: TextDocument = desDocument.getTextDocument();
        // java desTextDocument.setProperty(srcTextDocument.getProperty());
        desTextDocument.setProperty(srcTextDocument.getProperty());
      }
    }

    // java protected void copyComments(Document srcDocument, Document desDocument, int destCP, long rg)
    pub fn copyComments(&mut self, srcDocument: &dyn Document, desDocument: &dyn Document, destCP: i32, rg: i64) {
      // java if (srcDocument.getType() == 0)
      if srcDocument.getType() == 0 {
        // java Document srcCommentDocument = (SubDocument)srcDocument.getTextDocument().getSubdocument(3);
        let mut srcCommentDocument: Box<dyn Document> = (srcDocument.getTextDocument().getSubdocument(3)).downcast_ref::<SubDocument>().unwrap();
        // java Document desCommentDocument = (SubDocument)desDocument.getTextDocument().getSubdocument(3);
        let mut desCommentDocument: Box<dyn Document> = (desDocument.getTextDocument().getSubdocument(3)).downcast_ref::<SubDocument>().unwrap();
        // java PLCCommentRef srcCommentRefs = srcDocument.getCommentRef();
        let mut srcCommentRefs: PLCCommentRef = srcDocument.getCommentRef();
        // java PLCCommentRef desCommentRefs = desDocument.getCommentRef();
        let mut desCommentRefs: PLCCommentRef = desDocument.getCommentRef();
        // java PLCBookmarkStart desCommentStarts = desDocument.getCommentStarts();
        let mut desCommentStarts: PLCBookmarkStart = desDocument.getCommentStarts();
        // java PLCBookmarkEnd desCommentEnds = desDocument.getCommentEnds();
        let mut desCommentEnds: PLCBookmarkEnd = desDocument.getCommentEnds();
        // java PLCCommentTxt srcCommentTxt = srcCommentDocument.getCommentTxt();
        let mut srcCommentTxt: PLCCommentTxt = srcCommentDocument.getCommentTxt();
        // java PLCCommentTxt desCommentTxt = desCommentDocument.getCommentTxt();
        let mut desCommentTxt: PLCCommentTxt = desCommentDocument.getCommentTxt();
        // java int startCP = RANGEFactory.getStart(rg);
        let mut startCP: i32 = RANGEFactory::getStart(rg);
        // java int endCP = RANGEFactory.getEnd(rg);
        let mut endCP: i32 = RANGEFactory::getEnd(rg);
        let mut node: Box<dyn Node> = srcCommentRefs.lowerBound(startCP);
        // java for(PLC.Node node = srcCommentRefs.lowerBound(startCP); !srcCommentRefs.isEnd(node) && node.getCP() < endCP node = node.getNext())
        while !srcCommentRefs.isEnd(node) && node.getCP() < endCP {
          // java PLCCommentRef.CommentRefNode srcRefNode = (PLCCommentRef.CommentRefNode)node;
          let mut srcRefNode: PLCCommentRef::CommentRefNode = (node).downcast_ref::<PLCCommentRef::CommentRefNode>().unwrap();
          // java PLCCommentTxt.CommentTxtNode srcTxtNode = srcRefNode.getCommentTxt();
          let mut srcTxtNode: PLCCommentTxt::CommentTxtNode = srcRefNode.getCommentTxt();
          // java long commentRange = srcCommentTxt.getCommentContentRange(srcTxtNode);
          let mut commentRange: i64 = srcCommentTxt.getCommentContentRange(srcTxtNode);
          // java int oldLength = desCommentDocument.getLength();
          let mut oldLength: i32 = desCommentDocument.getLength();
          // java this.copyData(srcCommentDocument, desCommentDocument, oldLength, commentRange);
          self.copyData(&srcCommentDocument, &desCommentDocument, oldLength, commentRange);
          // java PLCCommentTxt.CommentTxtNode newTxtNode = desCommentTxt.addCommentTxt(oldLength);
          let mut newTxtNode: PLCCommentTxt::CommentTxtNode = desCommentTxt.addCommentTxt(oldLength);
          // java int distance = srcRefNode.getCommentStart().getCP() - startCP;
          let mut distance: i32 = srcRefNode.getCommentStart().getCP() - startCP;
          // java PLCBookmarkStart.BookmarkStartNode newStart = desCommentStarts.addBookmarkStart(distance + destCP);
          let mut newStart: PLCBookmarkStart::BookmarkStartNode = desCommentStarts.addBookmarkStart(distance + destCP);
          // java distance = srcRefNode.getCommentEnd().getCP() - startCP;
          distance = srcRefNode.getCommentEnd().getCP() - startCP;
          // java PLCBookmarkEnd.BookmarkEndNode newEnd = desCommentEnds.addBookmarkEnd(destCP + distance);
          let mut newEnd: PLCBookmarkEnd::BookmarkEndNode = desCommentEnds.addBookmarkEnd(destCP + distance);
          // java newStart.setEnd(newEnd);
          newStart.setEnd(newEnd);
          // java newEnd.setStart(newStart);
          newEnd.setStart(newStart);
          // java newStart.setName(srcRefNode.getCommentStart().getName());
          newStart.setName(srcRefNode.getCommentStart().getName());
          // java PLCCommentRef.CommentRefNode newRefNode = srcRefNode.clone();
          let mut newRefNode: PLCCommentRef::CommentRefNode = srcRefNode.clone();
          // java newTxtNode.setCommentRef(newRefNode);
          newTxtNode.setCommentRef(newRefNode);
          // java newStart.setCommentRef(newRefNode);
          newStart.setCommentRef(newRefNode);
          // java newEnd.setCommentRef(newRefNode);
          newEnd.setCommentRef(newRefNode);
          // java newRefNode.setCommentTxt(newTxtNode);
          newRefNode.setCommentTxt(newTxtNode);
          // java newRefNode.setCommentStart(newStart);
          newRefNode.setCommentStart(newStart);
          // java newRefNode.setCommentEnd(newEnd);
          newRefNode.setCommentEnd(newEnd);
          // java distance = srcRefNode.getCP() - startCP;
          distance = srcRefNode.getCP() - startCP;
          // java desCommentRefs.insert(destCP + distance, newRefNode);
          desCommentRefs.insert(destCP + distance, newRefNode);
        }
      }
    }

    // java protected void copyFootnoteEndnote(Document srcDocument, Document desDocument, int destCP, long rg)
    pub fn copyFootnoteEndnote(&mut self, srcDocument: &dyn Document, desDocument: &dyn Document, destCP: i32, rg: i64) {
      // java if (srcDocument.getType() == 0)
      if srcDocument.getType() == 0 {
        // java Document srcEndNoteDocument = srcDocument.getTextDocument().getSubdocument(4);
        let mut srcEndNoteDocument: Box<dyn Document> = srcDocument.getTextDocument().getSubdocument(4);
        // java Document desEndNoteDocument = desDocument.getTextDocument().getSubdocument(4);
        let mut desEndNoteDocument: Box<dyn Document> = desDocument.getTextDocument().getSubdocument(4);
        // java this.copyNotes(srcDocument.getEndnoteRef(), srcEndNoteDocument.getEndnoteTxt(), desDocument.getEndnoteRef(), desEndNoteDocument.getEndnoteTxt(), destCP, rg);
        self.copyNotes(&mut srcDocument.getEndnoteRef(), &srcEndNoteDocument.getEndnoteTxt(), &mut desDocument.getEndnoteRef(), &mut desEndNoteDocument.getEndnoteTxt(), destCP, rg);
        // java Document srcFootNoteDocument = srcDocument.getTextDocument().getSubdocument(1);
        let mut srcFootNoteDocument: Box<dyn Document> = srcDocument.getTextDocument().getSubdocument(1);
        // java Document desFootNoteDocument = desDocument.getTextDocument().getSubdocument(1);
        let mut desFootNoteDocument: Box<dyn Document> = desDocument.getTextDocument().getSubdocument(1);
        // java this.copyNotes(srcDocument.getFootnoteRef(), srcFootNoteDocument.getFootnoteTxt(), desDocument.getFootnoteRef(), desFootNoteDocument.getFootnoteTxt(), destCP, rg);
        self.copyNotes(&mut srcDocument.getFootnoteRef(), &srcFootNoteDocument.getFootnoteTxt(), &mut desDocument.getFootnoteRef(), &mut desFootNoteDocument.getFootnoteTxt(), destCP, rg);
      }
    }

    // java private void copyNotes(PLCFootnoteEndnoteRef srcRefPLC, PLCFootnoteEndnoteTxt srcTxtPLC, PLCFootnoteEndnoteRef desRefPLC, PLCFootnoteEndnoteTxt desTxtPLC, int destCP, long rg)
    pub fn copyNotes(&mut self, srcRefPLC: &mut PLCFootnoteEndnoteRef, srcTxtPLC: &PLCFootnoteEndnoteTxt, desRefPLC: &mut PLCFootnoteEndnoteRef, desTxtPLC: &mut PLCFootnoteEndnoteTxt, destCP: i32, rg: i64) {
      // java Document srcNoteDocument = srcTxtPLC.getDocument();
      let mut srcNoteDocument: Box<dyn Document> = srcTxtPLC.getDocument();
      // java Document desNoteDocument = desTxtPLC.getDocument();
      let mut desNoteDocument: Box<dyn Document> = desTxtPLC.getDocument();
      // java int startCP = RANGEFactory.getStart(rg);
      let mut startCP: i32 = RANGEFactory::getStart(rg);
      // java int endCP = RANGEFactory.getEnd(rg);
      let mut endCP: i32 = RANGEFactory::getEnd(rg);
      let mut node: Box<dyn Node> = srcRefPLC.lowerBound(startCP);
      // java for(PLC.Node node = srcRefPLC.lowerBound(startCP); !srcRefPLC.isHead(node) && node.getCP() < endCP node = node.getNext())
      while !srcRefPLC.isHead(node) && node.getCP() < endCP {
        // java PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode refNode = (PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode)node;
        let mut refNode: PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode = (node).downcast_ref::<PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode>().unwrap();
        // java PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode txtNode = refNode.getFootnoteEndnoteTxtNode();
        let mut txtNode: PLCFootnoteEndnoteTxt::FootnoteEndnoteTxtNode = refNode.getFootnoteEndnoteTxtNode();
        // java long srcNoteTxtRange = srcTxtPLC.getFootnoteEndnoteRange(txtNode);
        let mut srcNoteTxtRange: i64 = srcTxtPLC.getFootnoteEndnoteRange(&mut txtNode);
        // java int oldLength = desNoteDocument.getLength();
        let mut oldLength: i32 = desNoteDocument.getLength();
        // java this.copyData(srcNoteDocument, desNoteDocument, oldLength, srcNoteTxtRange);
        self.copyData(&srcNoteDocument, &desNoteDocument, oldLength, srcNoteTxtRange);
        // java PLCFootnoteEndnoteTxt.FootnoteEndnoteTxtNode newTxtNode = desTxtPLC.addFootnoteEndnoteTxt(oldLength);
        let mut newTxtNode: PLCFootnoteEndnoteTxt::FootnoteEndnoteTxtNode = desTxtPLC.addFootnoteEndnoteTxt(oldLength);
        // java int distance = refNode.getCP() - startCP;
        let mut distance: i32 = refNode.getCP() - startCP;
        // java PLCFootnoteEndnoteRef.FootnoteEndnoteRefNode newRefNode = desRefPLC.addFootnoteEndnoteRef(distance + destCP);
        let mut newRefNode: PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode = desRefPLC.addFootnoteEndnoteRef(distance + destCP);
        // java newRefNode.setNumberStart(refNode.getNumStart());
        newRefNode.setNumberStart(refNode.getNumStart());
        // java newRefNode.setFootnoteEndnoteTxtNode(newTxtNode);
        newRefNode.setFootnoteEndnoteTxtNode(newTxtNode);
        // java newTxtNode.setFootnoteEndnoteRef(newRefNode);
        newTxtNode.setFootnoteEndnoteRef(newRefNode);
      }
    }

    // java protected void copyHeaderFooter(Document srcDocument, Document desDocument, int destCP, long rg)
    pub fn copyHeaderFooter(&mut self, srcDocument: &dyn Document, desDocument: &dyn Document, destCP: i32, rg: i64) {
      // java if (srcDocument.getType() == 0)
      if srcDocument.getType() == 0 {
      }
    }

    // java protected void copySection(Document srcDocument, Document desDocument, long rg)
    pub fn copySection(&mut self, srcDocument: &dyn Document, desDocument: &dyn Document, rg: i64) {
      // java if (srcDocument.getType() == 0)
      if srcDocument.getType() == 0 {
      }
    }

    // java protected PropertySet correctRunStyle(PropertySet property)
    pub fn correctRunStyle(&mut self, property: &PropertySet) -> PropertySet {
      // java return property;
      return property;
    }

    // java protected PropertySet correctParagraphStyle(PropertySet property)
    pub fn correctParagraphStyle(&mut self, property: &PropertySet) -> PropertySet {
      // java return property;
      return property;
    }

    // java protected abstract void copyData(Document var1, Document var2, int var3, long var4)
    pub fn copyData(&mut self, var1: &dyn Document, var2: &dyn Document, var3: i32, var4: i64) {
      panic!("CopyDataBase::copyData not implemented");
    }

}
