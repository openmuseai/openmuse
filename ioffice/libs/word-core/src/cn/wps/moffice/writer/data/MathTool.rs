// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.kfc.impl.arrayList.WIntArrayList
use util::kfc::r#impl::arrayList::WIntArrayList::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.data.PLCMathStart
use crate::cn::wps::moffice::writer::data::PLCMathStart::*;
// Java: import cn.wps.moffice.writer.data.PLCMathEnd
use crate::cn::wps::moffice::writer::data::PLCMathEnd::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::*;
// Java: import org.apache.poi.poifs.filesystem.Entry
// (generator mis-import; IChpxTable::Entry)
use crate::cn::wps::moffice::writer::data::IChpxTable::Entry as ChpxEntry;
// Java: import cn.wps.moffice.writer.data.PLCMathFunction
use crate::cn::wps::moffice::writer::data::PLCMathFunction::*;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::*;

fn assert_doc(_msg: &str, _document: &dyn Document) {
    // Assert API is Option-oriented; Document ref is non-null at call sites
}

fn doc_char_at(document: &dyn Document, cp: i32) -> char {
    let p = document as *const dyn Document as *mut dyn Document;
    unsafe { (*p).charAt(cp) }
}

fn doc_chpx(document: &dyn Document) -> Option<&'static mut dyn IChpxTable> {
    document.getChpxTable().map(|p| unsafe { &mut *p })
}

fn doc_omath_starts(document: &dyn Document) -> Option<&'static mut PLCMathStart> {
    document.getOMathStarts().map(|p| unsafe { &mut *p })
}

fn doc_omath_ends(document: &dyn Document) -> Option<&'static mut PLCMathEnd> {
    document.getOMathEnds().map(|p| unsafe { &mut *p })
}

fn doc_plc_math(document: &dyn Document) -> Option<&'static mut PLCMathFunction> {
    document.getPLCMathFunction().map(|r| {
        let p = r as *const PLCMathFunction as *mut PLCMathFunction;
        unsafe { &mut *p }
    })
}

// java public class MathTool
#[derive(Default)]
pub struct MathTool {
}

// java public class MathTool
impl MathTool {
    // java public static boolean isInMath(Document document, int cp)
    pub fn isInMath(document: &dyn Document, cp: i32) -> bool {
      // java PLCMathStart plcMathStart = document.getOMathStarts();
      let mut plcMathStart = doc_omath_starts(document);
      // java PLCMathEnd plcMathEnd = document.getOMathEnds();
      let mut plcMathEnd = doc_omath_ends(document);
      // java if (null != plcMathStart && null != plcMathEnd)
      if plcMathStart.is_some() && plcMathEnd.is_some() {
        // java PLCMathStart.OMathStartNode oMathStartNode = plcMathStart.findClose(cp);
        let mut oMathStartNode = plcMathStart.as_mut().unwrap().findClose(cp);
        // java if (null == oMathStartNode)
        if oMathStartNode.is_none() {
          // java return false;
          return false;
        }
        // java else
        else {
          // java PLCMathEnd.OMathEndNode oMathEndNode = oMathStartNode.getEnd();
          let mut oMathEndNode = oMathStartNode.as_ref().unwrap().getEnd();
          // java return null != oMathEndNode && cp < oMathEndNode.getCP();
          return oMathEndNode.is_some() && cp < oMathEndNode.unwrap().getCP();
        }
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java public static int getMathSepaCP(Document document, int startCp)
    pub fn getMathSepaCP(document: &dyn Document, startCp: i32) -> i32 {
      // java int curCp = startCp;
      let mut curCp: i32 = startCp;
      // java IChpxTable chpxTable = document.getChpxTable();
      let mut chpxTable = doc_chpx(document).expect("chpxTable");
      // java IChpxTable.Entry curEntry = chpxTable.seek(startCp);
      let mut curEntry = IChpxTable::seek(chpxTable, startCp).expect("seek");
      // java if (isFMathSpecEntry(curEntry))
      if Self::isFMathSpecEntry(curEntry.as_ref()) {
        // java if (isMathEndPlaceholder(document, startCp))
        if Self::isMathEndPlaceholder(document, startCp) {
          // java return -1;
          return -1;
        }
        // java curCp = startCp + 1;
        curCp = startCp + 1;
        // java if (curCp >= curEntry.endCp() && !curEntry.isEnd())
        if curCp >= curEntry.endCp() && !curEntry.isEnd() {
          // java curEntry = curEntry.getNext();
          curEntry = ChpxEntry::getNext(&*curEntry).expect("getNext");
        }
      }
      // java while(!curEntry.isEnd())
      while !curEntry.isEnd() {
        // java if (isFMathSpecEntry(curEntry))
        if Self::isFMathSpecEntry(curEntry.as_ref()) {
          // java if (!isMathBeginPlaceholder(document, curCp))
          if !Self::isMathBeginPlaceholder(document, curCp) {
            // java return curCp;
            return curCp;
          }
          // java PLCMathFunction.MathNode startNode = getMathStartNodeByCp(document, curCp);
          let mut startNode = Self::getMathStartNodeByCp(document, curCp).expect("startNode");
          // java PLCMathFunction.MathNode endNode = startNode.getEndNode();
          let mut endNode = startNode.getEndNode().expect("endNode");
          // java curCp = endNode.getCP() + 1;
          curCp = endNode.getCP() + 1;
          // java curEntry = chpxTable.seek(curCp);
          curEntry = IChpxTable::seek(chpxTable, curCp).expect("seek");
        }
        // java else
        else {
          // java curEntry = curEntry.getNext();
          curEntry = ChpxEntry::getNext(&*curEntry).expect("getNext");
          // java curCp = curEntry.position();
          curCp = curEntry.position();
        }
      }
      // java return -1;
      return -1;
    }

    // java public static boolean hasSubMath(Document document, int startCP, int endCP)
    pub fn hasSubMath(document: &dyn Document, startCP: i32, endCP: i32) -> bool {
      // java Assert.assertNotNull("document should not be null", document);
      assert_doc("document should not be null", document);
      // java IChpxTable chpxTable = document.getChpxTable();
      let mut chpxTable = doc_chpx(document);
      // java Assert.assertNotNull("chpxTable should not be null", chpxTable);
      Assert::assertNotNull_msg("chpxTable should not be null", &chpxTable);
      let mut chpxTable = chpxTable.expect("chpxTable");
      // java IChpxTable.Entry frag = chpxTable.seek(startCP);
      let mut frag = IChpxTable::seek(chpxTable, startCP);
      // java Assert.assertNotNull("frag should not be null", frag);
      Assert::assertNotNull_msg("frag should not be null", &frag);
      let mut frag = frag.expect("frag");
      let mut cp: i32 = frag.position();
      // java for(int cp = frag.position(); cp < endCP; cp = frag.position())
      while cp < endCP {
        // java if (isMathBeginPlaceholder(document, cp))
        if Self::isMathBeginPlaceholder(document, cp) {
          // java return true;
          return true;
        }
        // java frag = frag.getNext();
        frag = ChpxEntry::getNext(&*frag).expect("getNext");
        cp = frag.position();
      }
      // java return false;
      return false;
    }

    // java public static int getMatrixRowEntry(Document document, int startCp)
    pub fn getMatrixRowEntry(document: &dyn Document, startCp: i32) -> i32 {
      // java int curCp = startCp;
      let mut curCp: i32 = startCp;
      // java IChpxTable chpxTable = document.getChpxTable();
      let mut chpxTable = doc_chpx(document).expect("chpxTable");
      // java IChpxTable.Entry curEntry = chpxTable.seek(startCp);
      let mut curEntry = IChpxTable::seek(chpxTable, startCp).expect("seek");
      // java if (isFMathSpecEntry(curEntry))
      if Self::isFMathSpecEntry(curEntry.as_ref()) {
        // java if (isMathEndPlaceholder(document, startCp))
        if Self::isMathEndPlaceholder(document, startCp) {
          // java return -1;
          return -1;
        }
        // java curCp = startCp + 1;
        curCp = startCp + 1;
        // java if (curCp >= curEntry.endCp() && !curEntry.isEnd())
        if curCp >= curEntry.endCp() && !curEntry.isEnd() {
          // java curEntry = curEntry.getNext();
          curEntry = ChpxEntry::getNext(&*curEntry).expect("getNext");
        }
      }
      // java while(!curEntry.isEnd())
      while !curEntry.isEnd() {
        // java if (isFMathSpecEntry(curEntry))
        if Self::isFMathSpecEntry(curEntry.as_ref()) {
          // java if (isMathBeginPlaceholder(document, curCp))
          if Self::isMathBeginPlaceholder(document, curCp) {
            // java PLCMathFunction.MathNode startNode = getMathStartNodeByCp(document, curCp);
            let mut startNode = Self::getMathStartNodeByCp(document, curCp).expect("startNode");
            // java PLCMathFunction.MathNode endNode = startNode.getEndNode();
            let mut endNode = startNode.getEndNode().expect("endNode");
            // java curCp = endNode.getCP() + 1;
            curCp = endNode.getCP() + 1;
            // java curEntry = chpxTable.seek(curCp);
            curEntry = IChpxTable::seek(chpxTable, curCp).expect("seek");
          }
          // java else
          else {
            // java if (isMathEndPlaceholder(document, curCp) || isMatrixRowPlaceholder(document, curCp))
            if Self::isMathEndPlaceholder(document, curCp) || Self::isMatrixRowPlaceholder(document, curCp) {
              // java break;
              break;
            }
            // java ++curCp;
            curCp += 1;
            // java curEntry = getChpxEntryByCp(document, curCp);
            curEntry = Self::getChpxEntryByCp(document, curCp).expect("entry");
          }
        }
        // java else
        else {
          // java curEntry = curEntry.getNext();
          curEntry = ChpxEntry::getNext(&*curEntry).expect("getNext");
          // java curCp = curEntry.position();
          curCp = curEntry.position();
        }
      }
      // java return curCp;
      return curCp;
    }

    // java public static boolean isMathBeginPlaceholder(Document document, int cp)
    pub fn isMathBeginPlaceholder(document: &dyn Document, cp: i32) -> bool {
      // java Assert.assertNotNull("document should not be null", document);
      assert_doc("document should not be null", document);
      // java char placeholder = document.charAt(cp);
      let mut placeholder: char = doc_char_at(document, cp);
      // java return 28 == placeholder;
      return (placeholder as u32) == 28;
    }

    // java public static boolean isMathEndPlaceholder(Document document, int cp)
    pub fn isMathEndPlaceholder(document: &dyn Document, cp: i32) -> bool {
      // java Assert.assertNotNull("document should not be null", document);
      assert_doc("document should not be null", document);
      // java char placeholder = document.charAt(cp);
      let mut placeholder: char = doc_char_at(document, cp);
      // java return 27 == placeholder;
      return (placeholder as u32) == 27;
    }

    // java private static boolean isMatrixRowPlaceholder(Document document, int cp)
    pub fn isMatrixRowPlaceholder(document: &dyn Document, cp: i32) -> bool {
      // java Assert.assertNotNull("document should not be null", document);
      assert_doc("document should not be null", document);
      // java char placeholder = document.charAt(cp);
      let mut placeholder: char = doc_char_at(document, cp);
      // java return '@' == placeholder;
      return '@' == placeholder;
    }

    // java public static IChpxTable.Entry getEndEntryByStartCp(Document document, int startCp)
    pub fn getEndEntryByStartCp(document: &dyn Document, startCp: i32) -> Option<Box<dyn ChpxEntry>> {
      // java Assert.assertNotNull("document should not be null", document);
      assert_doc("document should not be null", document);
      // java PLCMathFunction.MathNode startNode = getMathStartNodeByCp(document, startCp);
      let mut startNode = Self::getMathStartNodeByCp(document, startCp)?;
      // java PLCMathFunction.MathNode endNode = startNode.getEndNode();
      let mut endNode = startNode.getEndNode()?;
      // java int endCp = endNode.getCP();
      let mut endCp: i32 = endNode.getCP();
      // java return document.getChpxTable().seek(endCp);
      return doc_chpx(document).and_then(|t| IChpxTable::seek(&*t, endCp));
    }

    // java public static boolean isFMathSpecEntry(IChpxTable.Entry entry)
    pub fn isFMathSpecEntry(entry: &dyn ChpxEntry) -> bool {
      // java Assert.assertNotNull("entry should not be null", entry);
      let _ = entry;
      // java if (entry.isEnd())
      if entry.isEnd() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java PropertySet propSet = entry.getProperty();
        let mut propSet = ChpxEntry::getProperty(entry);
        // java return propSet.getBoolean(104, false);
        return propSet.map(|p| p.getBoolean__int_boolean(104, false)).unwrap_or(false);
      }
    }

    // java public static PLCMathFunction.MathStartNode getMathStartNodeByCp(Document document, int cp)
    pub fn getMathStartNodeByCp(document: &dyn Document, cp: i32) -> Option<MathStartNode> {
      // java Assert.assertNotNull("document should not be null", document);
      assert_doc("document should not be null", document);
      // java PLCMathFunction plcMath = document.getPLCMathFunction();
      let mut plcMath = doc_plc_math(document);
      // java Assert.assertNotNull("plcMath should not be null", plcMath);
      Assert::assertNotNull_msg("plcMath should not be null", &plcMath);
      // java return plcMath.findMathStartNodeByCP(cp);
      return plcMath.map(|p| p.findMathStartNodeByCP(cp)).flatten();
    }

    // java public static PLCMathFunction.MathNode getMathEndNodeByCp(Document document, int cp)
    pub fn getMathEndNodeByCp(document: &dyn Document, cp: i32) -> Option<MathNode> {
      // java Assert.assertNotNull("document should not be null", document);
      assert_doc("document should not be null", document);
      // java PLCMathFunction plcMath = document.getPLCMathFunction();
      let mut plcMath = doc_plc_math(document);
      // java Assert.assertNotNull("plcMath should not be null", plcMath);
      Assert::assertNotNull_msg("plcMath should not be null", &plcMath);
      // java return plcMath.findMathEndNodeByCP(cp);
      return plcMath.map(|p| p.findMathEndNodeByCP(cp)).flatten();
    }

    // java public static boolean isOMathStart(Document document, int cp)
    pub fn isOMathStart(document: &dyn Document, cp: i32) -> bool {
      // java Assert.assertNotNull("document should not be null", document);
      assert_doc("document should not be null", document);
      // java return null != getOMathStartNode(document, cp);
      return Self::getOMathStartNode(document, cp).is_some();
    }

    // java public static boolean isOMathEnd(Document document, int cp)
    pub fn isOMathEnd(document: &dyn Document, cp: i32) -> bool {
      // java Assert.assertNotNull("document should not be null", document);
      assert_doc("document should not be null", document);
      // java return null != getOMathEndNode(document, cp);
      return Self::getOMathEndNode(document, cp).is_some();
    }

    // java public static PLCMathStart.OMathStartNode getOMathStartNode(Document document, int cp)
    pub fn getOMathStartNode(document: &dyn Document, cp: i32) -> Option<OMathStartNode> {
      // java Assert.assertNotNull("document should not be null", document);
      assert_doc("document should not be null", document);
      // java PLCMathStart plcOMathStart = document.getOMathStarts();
      let mut plcOMathStart = doc_omath_starts(document);
      // java return null == plcOMathStart ? null : plcOMathStart.findStartByCP(cp);
      return plcOMathStart.and_then(|p| p.findStartByCP(cp));
    }

    // java public static PLCMathEnd.OMathEndNode getOMathEndNode(Document document, int cp)
    pub fn getOMathEndNode(document: &dyn Document, cp: i32) -> Option<OMathEndNode> {
      // java Assert.assertNotNull("document should not be null", document);
      assert_doc("document should not be null", document);
      // java PLCMathEnd plcOMathEnd = document.getOMathEnds();
      let mut plcOMathEnd = doc_omath_ends(document);
      // java return null == plcOMathEnd ? null : plcOMathEnd.findEndByCP(cp);
      return plcOMathEnd.and_then(|p| p.findEndByCP(cp));
    }

    // java public static IChpxTable.Entry getChpxEntryByCp(Document document, int cp)
    pub fn getChpxEntryByCp(document: &dyn Document, cp: i32) -> Option<Box<dyn ChpxEntry>> {
      // java Assert.assertNotNull("document should not be null", document);
      assert_doc("document should not be null", document);
      // java IChpxTable chpxTable = document.getChpxTable();
      let mut chpxTable = doc_chpx(document)?;
      // java return chpxTable.seek(cp);
      return IChpxTable::seek(chpxTable, cp);
    }

    // java public static String getMathXml(Document document, PLCMathFunction.MathNode node)
    pub fn getMathXml(_document: &dyn Document, _node: &MathNode) -> Option<String> {
      // java return null;
      return None;
    }

    // java public static boolean convertMathXml(Document document, int cp, String mathXml)
    pub fn convertMathXml(_document: &dyn Document, _cp: i32, _mathXml: &str) -> bool {
      // java return false;
      return false;
    }

    // java public static boolean hasMath(Document document)
    pub fn hasMath(document: &dyn Document) -> bool {
      // java Assert.assertNotNull("document should not be null", document);
      assert_doc("document should not be null", document);
      // java PLCMathStart plcOMathStart = document.getOMathStarts();
      let mut plcOMathStart = doc_omath_starts(document);
      // java PLCMathEnd plcOMathEnd = document.getOMathEnds();
      let mut plcOMathEnd = doc_omath_ends(document);
      // java return null != plcOMathStart && null != plcOMathEnd;
      return plcOMathStart.is_some() && plcOMathEnd.is_some();
    }

    // java public static WIntArrayList getOMathCpList(Document document)
    pub fn getOMathCpList(document: &dyn Document) -> Option<WIntArrayList> {
      // java Assert.assertNotNull("document should not be null", document);
      assert_doc("document should not be null", document);
      // java PLCMathStart plcOMathStart = document.getOMathStarts();
      let mut plcOMathStart = doc_omath_starts(document);
      // java PLCMathEnd plcOMathEnd = document.getOMathEnds();
      let mut plcOMathEnd = doc_omath_ends(document);
      // java if (null != plcOMathStart && null != plcOMathEnd)
      if plcOMathStart.is_some() && plcOMathEnd.is_some() {
        // java WIntArrayList intList = new WIntArrayList();
        let mut intList: WIntArrayList = WIntArrayList::new();
        // java PLC.Iterator oMathIter = plcOMathStart.createIterator();
        let mut oMathIter = plcOMathStart.as_mut().unwrap().createIterator();
        // java while(oMathIter.hasPrev())
        while oMathIter.hasPrev() {
          // java PLCMathStart.OMathStartNode oMathStartNode = (PLCMathStart.OMathStartNode)oMathIter.prev();
          let prev_node = oMathIter.prev();
          let mut oMathStartNode = OMathStartNode { super_: prev_node, end: None, mOMathPara: None };
          // java PLCMathEnd.OMathEndNode oMathEndNode = oMathStartNode.getEnd();
          let mut oMathEndNode = oMathStartNode.getEnd();
          // java intList.add(oMathIter.getStart());
          intList.add(oMathIter.getStart());
          // java intList.add(oMathEndNode.getCP());
          if let Some(e) = oMathEndNode {
            intList.add(e.getCP());
          }
        }
        // java intList.sort();
        intList.sort();
        // java return intList;
        return Some(intList);
      }
      // java else
      else {
        // java return null;
        return None;
      }
    }

    // java public static WIntArrayList getOMathCps4Chpx(Document document, int start, int end)
    pub fn getOMathCps4Chpx(document: &dyn Document, start: i32, end: i32) -> Option<WIntArrayList> {
      // java Assert.assertNotNull("document should not be null", document);
      assert_doc("document should not be null", document);
      // java if (!hasMath(document))
      if !Self::hasMath(document) {
        // java return null;
        return None;
      }
      // java else
      else {
        // java WIntArrayList intList = getOMathCpList(document);
        let mut intList: WIntArrayList = Self::getOMathCpList(document)?;
        // java WIntArrayList curCpList = new WIntArrayList();
        let mut curCpList: WIntArrayList = WIntArrayList::new();
        // java int length = intList.size();
        let mut length: i32 = intList.size();
        let mut i: i32 = 0;
        // java for(int i = 0; i < length; ++i)
        while i < length {
          // java int oMathCp = intList.get(i);
          let mut oMathCp: i32 = intList.get(i);
          // java if (start <= oMathCp && oMathCp < end)
          if start <= oMathCp && oMathCp < end {
            // java curCpList.add(oMathCp);
            curCpList.add(oMathCp);
          }
          // java if (oMathCp > end)
          if oMathCp > end {
            // java break;
            break;
          }
          i += 1;
        }
        // java return curCpList;
        return Some(curCpList);
      }
    }

    // java public static boolean hasMathInTextDocument(TextDocument textDocument)
    pub fn hasMathInTextDocument(textDocument: Option<&TextDocument>) -> bool {
      // java if (textDocument == null)
      if textDocument.is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        let textDocument = textDocument.unwrap();
        // java int size = 7;
        let mut size: i32 = 7;
        let mut i: i32 = 0;
        // java for(int i = 0; i < size; ++i)
        while i < size {
          // java if (hasMathInSubDocument(textDocument.getSubdocument(i)))
          let sub = textDocument.getSubdocument(i);
          if Self::hasMathInSubDocument(sub.map(|s| s as &dyn Document)) {
            // java return true;
            return true;
          }
          i += 1;
        }
        // java return false;
        return false;
      }
    }

    // java private static boolean hasMathInSubDocument(Document document)
    pub fn hasMathInSubDocument(document: Option<&dyn Document>) -> bool {
      // java return null != document && document.getLength() != 0 ? document.isContainMathEquations() : false;
      return match document {
        Some(d) if d.getLength() != 0 => d.isContainMathEquations(),
        _ => false,
      };
    }

}
