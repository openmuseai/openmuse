// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.collections.textrope.ITextRope
use crate::cn::wps::collections::textrope::ITextRope::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.transaction.Transaction
use util::moffice::transaction::Transaction::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KHeaderFooterIndex
use crate::cn::wps::moffice::writer::core::KHeaderFooterIndex::*;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.SubDocument
use crate::cn::wps::moffice::writer::core::SubDocument::*;
// Java: import cn.wps.moffice.writer.core.TextDocument
use crate::cn::wps::moffice::writer::core::TextDocument::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::{IChpxTable, Entry as IChpxEntry};
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::{IPapxTable, Entry as IPapxEntry};
use crate::cn::wps::moffice::writer::data::ITable::Entry as ITableEntry;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::*;
// Java: import cn.wps.moffice.writer.data.PLCHeader
use crate::cn::wps::moffice::writer::data::PLCHeader::*;
// Java: import cn.wps.moffice.writer.data.PLCSection
use crate::cn::wps::moffice::writer::data::PLCSection::*;
// Java: import cn.wps.util.concurrent.locks.RangeWriteLock
use crate::cn::wps::util::concurrent::locks::RangeWriteLock::*;
// Java: import cn.wps.io.dom.Node
// generator defect: not in HelperHeaderFooter.java — skipped so PLC::Node is not shadowed
// Java: import org.apache.poi.poifs.filesystem.Entry
// generator defect: not in HelperHeaderFooter.java — skipped so ITable::Entry is not shadowed

// java public final class HelperHeaderFooter
#[derive(Default)]
pub struct HelperHeaderFooter {
}

// java public static class RangeHeaderFooter extends KRange
#[derive(Default)]
pub struct RangeHeaderFooter {
    // java KRange super_
    pub(crate) super_: KRange,
    // java public PLC.Node mStart;
    pub(crate) mStart: Node,
}

// java public final class HelperHeaderFooter
impl HelperHeaderFooter {
    // java public static void appendHeaderFooterNode(PLCSection.SectionNode sectionNode, PLCHeader plcHeader)
    pub fn appendHeaderFooterNode__sectionnode_plcheader(sectionNode: &mut SectionNode, plcHeader: &mut PLCHeader) {
      // java Assert.assertNotNull("sectionNode should not be null!", sectionNode);
      Assert::assertNotNull_msg("sectionNode should not be null!", &*sectionNode);
      // java Assert.assertNotNull("plcHeader should not be null!", plcHeader);
      Assert::assertNotNull_msg("plcHeader should not be null!", &*plcHeader);
      // java Document documentHeaderFooter = plcHeader.getDocument();
      let documentHeaderFooter: *mut dyn Document = plcHeader
        .getDocument_mut()
        .expect("documentHeaderFooter") as *mut dyn Document;
      let documentHeaderFooter_ref = unsafe { &*documentHeaderFooter };
      // java Assert.assertNotNull("documentHeaderFooter should not be null!", documentHeaderFooter);
      Assert::assertNotNull_msg("documentHeaderFooter should not be null!", documentHeaderFooter_ref);
      // java Assert.assertTrue("Document.HEADER_DOCUMENT == documentHeaderFooter.getType() should be true!", 2 == documentHeaderFooter.getType());
      Assert::assertTrue_msg("Document.HEADER_DOCUMENT == documentHeaderFooter.getType() should be true!", 2 == documentHeaderFooter_ref.getType());
      // java debugCheck_appendHeaderFooterNodeSectionNode(sectionNode);
      Self::debugCheck_appendHeaderFooterNodeSectionNode(&sectionNode);
      // java PLC.Node evenHeader = sectionNode.getEvenHeader();
      let mut evenHeader: Node = sectionNode.getEvenHeader();
      // java if (null == evenHeader)
      if evenHeader.ptr.is_null() {
        // java TextDocument textDocument = documentHeaderFooter.getTextDocument();
        // java RangeWriteLock wrietLock = textDocument.getMainDocument().writeLock();
        // rust: `Document::writeLock` 当前默认实现即空锁；从已存储的 `*mut Document`
        //   调用，避免 `&T as &mut T`。Java 主文档锁与页眉文档锁在默认实现下等价。
        let mut wrietLock: RangeWriteLock = unsafe { (*documentHeaderFooter).writeLock() };
            // java int len = documentHeaderFooter.getLength();
            let mut len: i32 = unsafe { (*documentHeaderFooter).getLength() };
            // java appendHeaderFooterNode(sectionNode, plcHeader, len);
            Self::appendHeaderFooterNode__sectionnode_plcheader_int(sectionNode, plcHeader, len);
              // java wrietLock.unlock();
              wrietLock.unlock();
      }
    }

    // java public static void appendHeaderFooterNodeSection(PLCSection.SectionNode sectionNode, PLCHeader plcHeader, int len)
    pub fn appendHeaderFooterNodeSection(sectionNode: &mut SectionNode, plcHeader: &mut PLCHeader, len: i32) {
      // java Assert.assertNotNull("sectionNode should not be null!", sectionNode);
      Assert::assertNotNull_msg("sectionNode should not be null!", &*sectionNode);
      // java Assert.assertNotNull("plcHeader should not be null!", plcHeader);
      Assert::assertNotNull_msg("plcHeader should not be null!", &*plcHeader);
      // java debugCheck_appendHeaderFooterNodeSectionNode(sectionNode);
      Self::debugCheck_appendHeaderFooterNodeSectionNode(sectionNode);
      // java appendHeaderFooterNode(sectionNode, plcHeader, len);
      Self::appendHeaderFooterNode__sectionnode_plcheader_int(sectionNode, plcHeader, len);
    }

    // java private static void appendHeaderFooterNode(PLCSection.SectionNode sectionNode, PLCHeader plcHeader, int len)
    pub fn appendHeaderFooterNode__sectionnode_plcheader_int(sectionNode: &mut SectionNode, plcHeader: &mut PLCHeader, len: i32) {
      // java appendEvenHeader(sectionNode, plcHeader, len);
      Self::appendEvenHeader(sectionNode, plcHeader, len);
      // java appendOddHeader(sectionNode, plcHeader, len);
      Self::appendOddHeader(sectionNode, plcHeader, len);
      // java appendEvenFooter(sectionNode, plcHeader, len);
      Self::appendEvenFooter(sectionNode, plcHeader, len);
      // java appendOddFooter(sectionNode, plcHeader, len);
      Self::appendOddFooter(sectionNode, plcHeader, len);
      // java appendFirstHeader(sectionNode, plcHeader, len);
      Self::appendFirstHeader(sectionNode, plcHeader, len);
      // java appendFirstFooter(sectionNode, plcHeader, len);
      Self::appendFirstFooter(sectionNode, plcHeader, len);
    }

    // java public static boolean hasHeaderEven(PLCSection.SectionNode sectionNode, Document document)
    pub fn hasHeaderEven(sectionNode: &SectionNode, document: &dyn Document) -> bool {
      // java RangeHeaderFooter rangeEvenHeader = getRangeHeader(document, sectionNode, KHeaderFooterIndex.HeaderFooterEvenPages);
      let mut rangeEvenHeader: RangeHeaderFooter = Self::getRangeHeader(document, sectionNode, KHeaderFooterIndex::HeaderFooterEvenPages).unwrap();
      // java Assert.assertNotNull("rangeEvenHeader should not be null!", rangeEvenHeader);
      Assert::assertNotNull_msg("rangeEvenHeader should not be null!", &rangeEvenHeader);
      // java boolean var3;
      let mut var3: bool;
          // java if (!hasContentHeaderFooter(rangeEvenHeader))
          if !Self::hasContentHeaderFooter(&rangeEvenHeader) {
            // java var3 = false;
            var3 = false;
            // java return var3;
            return var3;
          }
          // java var3 = true;
          var3 = true;
            // java rangeEvenHeader.dispose();
            rangeEvenHeader.dispose();
      // java return var3;
      return var3;
    }

    // java public static boolean hasFooterEven(PLCSection.SectionNode sectionNode, Document document)
    pub fn hasFooterEven(sectionNode: &SectionNode, document: &dyn Document) -> bool {
      // java RangeHeaderFooter rangeEvenFooter = getRangeFooter(document, sectionNode, KHeaderFooterIndex.HeaderFooterEvenPages);
      let mut rangeEvenFooter: RangeHeaderFooter = Self::getRangeFooter(document, sectionNode, KHeaderFooterIndex::HeaderFooterEvenPages).unwrap();
      // java Assert.assertNotNull("rangeEvenFooter should not be null!", rangeEvenFooter);
      Assert::assertNotNull_msg("rangeEvenFooter should not be null!", &rangeEvenFooter);
      // java boolean var3;
      let mut var3: bool;
          // java if (!hasContentHeaderFooter(rangeEvenFooter))
          if !Self::hasContentHeaderFooter(&rangeEvenFooter) {
            // java var3 = false;
            var3 = false;
            // java return var3;
            return var3;
          }
          // java var3 = true;
          var3 = true;
            // java rangeEvenFooter.dispose();
            rangeEvenFooter.dispose();
      // java return var3;
      return var3;
    }

    // java public static boolean hasHeaderFirst(PLCSection.SectionNode sectionNode, Document document)
    pub fn hasHeaderFirst(sectionNode: &SectionNode, document: &dyn Document) -> bool {
      // java RangeHeaderFooter rangeFirstHeader = getRangeHeader(document, sectionNode, KHeaderFooterIndex.HeaderFooterFirstPage);
      let mut rangeFirstHeader: RangeHeaderFooter = Self::getRangeHeader(document, sectionNode, KHeaderFooterIndex::HeaderFooterFirstPage).unwrap();
      // java Assert.assertNotNull("rangeFirstHeader should not be null!", rangeFirstHeader);
      Assert::assertNotNull_msg("rangeFirstHeader should not be null!", &rangeFirstHeader);
      // java boolean var3;
      let mut var3: bool;
          // java if (!hasContentHeaderFooter(rangeFirstHeader))
          if !Self::hasContentHeaderFooter(&rangeFirstHeader) {
            // java var3 = false;
            var3 = false;
            // java return var3;
            return var3;
          }
          // java var3 = true;
          var3 = true;
            // java rangeFirstHeader.dispose();
            rangeFirstHeader.dispose();
      // java return var3;
      return var3;
    }

    // java public static boolean hasFooterFirst(PLCSection.SectionNode sectionNode, Document document)
    pub fn hasFooterFirst(sectionNode: &SectionNode, document: &dyn Document) -> bool {
      // java RangeHeaderFooter rangeFirstFooter = getRangeFooter(document, sectionNode, KHeaderFooterIndex.HeaderFooterFirstPage);
      let mut rangeFirstFooter: RangeHeaderFooter = Self::getRangeFooter(document, sectionNode, KHeaderFooterIndex::HeaderFooterFirstPage).unwrap();
      // java Assert.assertNotNull("rangeFirstFooter should not be null!", rangeFirstFooter);
      Assert::assertNotNull_msg("rangeFirstFooter should not be null!", &rangeFirstFooter);
      // java boolean var3;
      let mut var3: bool;
          // java if (!hasContentHeaderFooter(rangeFirstFooter))
          if !Self::hasContentHeaderFooter(&rangeFirstFooter) {
            // java var3 = false;
            var3 = false;
            // java return var3;
            return var3;
          }
          // java var3 = true;
          var3 = true;
            // java rangeFirstFooter.dispose();
            rangeFirstFooter.dispose();
      // java return var3;
      return var3;
    }

    // java public static boolean hasHeaderOdd(PLCSection.SectionNode sectionNode, Document document)
    pub fn hasHeaderOdd(sectionNode: &SectionNode, document: &dyn Document) -> bool {
      // java RangeHeaderFooter rangeOddHeader = getRangeHeader(document, sectionNode, KHeaderFooterIndex.HeaderFooterPrimary);
      let mut rangeOddHeader: RangeHeaderFooter = Self::getRangeHeader(document, sectionNode, KHeaderFooterIndex::HeaderFooterPrimary).unwrap();
      // java Assert.assertNotNull("rangeOddHeader should not be null!", rangeOddHeader);
      Assert::assertNotNull_msg("rangeOddHeader should not be null!", &rangeOddHeader);
      // java boolean var3;
      let mut var3: bool;
          // java if (!hasContentHeaderFooter(rangeOddHeader))
          if !Self::hasContentHeaderFooter(&rangeOddHeader) {
            // java var3 = false;
            var3 = false;
            // java return var3;
            return var3;
          }
          // java var3 = true;
          var3 = true;
            // java rangeOddHeader.dispose();
            rangeOddHeader.dispose();
      // java return var3;
      return var3;
    }

    // java public static boolean hasFooterOdd(PLCSection.SectionNode sectionNode, Document document)
    pub fn hasFooterOdd(sectionNode: &SectionNode, document: &dyn Document) -> bool {
      // java RangeHeaderFooter rangeOddFooter = getRangeFooter(document, sectionNode, KHeaderFooterIndex.HeaderFooterPrimary);
      let mut rangeOddFooter: RangeHeaderFooter = Self::getRangeFooter(document, sectionNode, KHeaderFooterIndex::HeaderFooterPrimary).unwrap();
      // java Assert.assertNotNull("rangeOddFooter should not be null!", rangeOddFooter);
      Assert::assertNotNull_msg("rangeOddFooter should not be null!", &rangeOddFooter);
      // java boolean var3;
      let mut var3: bool;
          // java if (!hasContentHeaderFooter(rangeOddFooter))
          if !Self::hasContentHeaderFooter(&rangeOddFooter) {
            // java var3 = false;
            var3 = false;
            // java return var3;
            return var3;
          }
          // java var3 = true;
          var3 = true;
            // java rangeOddFooter.dispose();
            rangeOddFooter.dispose();
      // java return var3;
      return var3;
    }

    // java public static boolean hasHeaderFooter(PLCSection plcSection, Document document)
    pub fn hasHeaderFooter__plcsection_document(plcSection: &PLCSection, document: &dyn Document) -> bool {
      // java Assert.assertNotNull("plcSection should not be null!", plcSection);
      Assert::assertNotNull_msg("plcSection should not be null!", plcSection);
      // java int size = plcSection.size();
      let mut size: i32 = plcSection.size();
      let mut i: i32 = 0;
      // java for(int i = 0; i < size; ++i)
      while i < size {
        // java if (hasHeaderFooter((PLCSection.SectionNode)plcSection.get(i), document))
        if Self::hasHeaderFooter__sectionnode_document(&as_section(plcSection.get(i)), document) {
          // java return true;
          return true;
        }
        i += 1;
      }
      // java return false;
      return false;
    }

    // java private static boolean hasHeaderFooter(PLCSection.SectionNode sectionNode, Document document)
    pub fn hasHeaderFooter__sectionnode_document(sectionNode: &SectionNode, document: &dyn Document) -> bool {
      // java Assert.assertNotNull("firstSectionNode should not be null!", sectionNode);
      Assert::assertNotNull_msg("firstSectionNode should not be null!", sectionNode);
      // java debugCheck_appendHeaderFooterNodeSectionNode(sectionNode);
      Self::debugCheck_appendHeaderFooterNodeSectionNode(&sectionNode);
      // java RangeHeaderFooter rangeEvenHeader = getRangeHeader(document, sectionNode, KHeaderFooterIndex.HeaderFooterEvenPages);
      let mut rangeEvenHeader: RangeHeaderFooter = Self::getRangeHeader(document, &sectionNode, KHeaderFooterIndex::HeaderFooterEvenPages).unwrap();
      // java Assert.assertNotNull("rangeEvenHeader should not be null!", rangeEvenHeader);
      Assert::assertNotNull_msg("rangeEvenHeader should not be null!", &rangeEvenHeader);
      // java RangeHeaderFooter rangeOddHeader = getRangeHeader(document, sectionNode, KHeaderFooterIndex.HeaderFooterPrimary);
      let mut rangeOddHeader: RangeHeaderFooter = Self::getRangeHeader(document, &sectionNode, KHeaderFooterIndex::HeaderFooterPrimary).unwrap();
      // java Assert.assertNotNull("rangeOddHeader should not be null!", rangeOddHeader);
      Assert::assertNotNull_msg("rangeOddHeader should not be null!", &rangeOddHeader);
      // java RangeHeaderFooter rangeEvenFooter = getRangeFooter(document, sectionNode, KHeaderFooterIndex.HeaderFooterEvenPages);
      let mut rangeEvenFooter: RangeHeaderFooter = Self::getRangeFooter(document, &sectionNode, KHeaderFooterIndex::HeaderFooterEvenPages).unwrap();
      // java Assert.assertNotNull("rangeEvenFooter should not be null!", rangeEvenFooter);
      Assert::assertNotNull_msg("rangeEvenFooter should not be null!", &rangeEvenFooter);
      // java RangeHeaderFooter rangeOddFooter = getRangeFooter(document, sectionNode, KHeaderFooterIndex.HeaderFooterPrimary);
      let mut rangeOddFooter: RangeHeaderFooter = Self::getRangeFooter(document, &sectionNode, KHeaderFooterIndex::HeaderFooterPrimary).unwrap();
      // java Assert.assertNotNull("rangeOddFooter should not be null!", rangeOddFooter);
      Assert::assertNotNull_msg("rangeOddFooter should not be null!", &rangeOddFooter);
      // java RangeHeaderFooter rangeFirstHeader = getRangeHeader(document, sectionNode, KHeaderFooterIndex.HeaderFooterFirstPage);
      let mut rangeFirstHeader: RangeHeaderFooter = Self::getRangeHeader(document, &sectionNode, KHeaderFooterIndex::HeaderFooterFirstPage).unwrap();
      // java Assert.assertNotNull("rangeFirstHeader should not be null!", rangeFirstHeader);
      Assert::assertNotNull_msg("rangeFirstHeader should not be null!", &rangeFirstHeader);
      // java RangeHeaderFooter rangeFirstFooter = getRangeFooter(document, sectionNode, KHeaderFooterIndex.HeaderFooterFirstPage);
      let mut rangeFirstFooter: RangeHeaderFooter = Self::getRangeFooter(document, &sectionNode, KHeaderFooterIndex::HeaderFooterFirstPage).unwrap();
      // java Assert.assertNotNull("rangeFirstFooter should not be null!", rangeFirstFooter);
      Assert::assertNotNull_msg("rangeFirstFooter should not be null!", &rangeFirstFooter);
      // java boolean var8;
      let mut var8: bool;
          // java if (!hasContentHeaderFooter(rangeEvenHeader) && !hasContentHeaderFooter(rangeOddHeader) && !hasContentHeaderFooter(rangeEvenFooter) && !hasContentHeaderFooter(rangeOddFooter) && !hasContentHeaderFooter(rangeFirstHeader) && !hasContentHeaderFooter(rangeFirstFooter))
          if !Self::hasContentHeaderFooter(&rangeEvenHeader) && !Self::hasContentHeaderFooter(&rangeOddHeader) && !Self::hasContentHeaderFooter(&rangeEvenFooter) && !Self::hasContentHeaderFooter(&rangeOddFooter) && !Self::hasContentHeaderFooter(&rangeFirstHeader) && !Self::hasContentHeaderFooter(&rangeFirstFooter) {
            // java var8 = false;
            var8 = false;
            // java return var8;
            return var8;
          }
          // java var8 = true;
          var8 = true;
            // java rangeEvenHeader.dispose();
            rangeEvenHeader.dispose();
            // java rangeOddHeader.dispose();
            rangeOddHeader.dispose();
            // java rangeEvenFooter.dispose();
            rangeEvenFooter.dispose();
            // java rangeOddFooter.dispose();
            rangeOddFooter.dispose();
            // java rangeFirstHeader.dispose();
            rangeFirstHeader.dispose();
            // java rangeFirstFooter.dispose();
            rangeFirstFooter.dispose();
      // java return var8;
      return var8;
    }

    // java private static boolean hasContentHeaderFooter(RangeHeaderFooter range)
    pub fn hasContentHeaderFooter(range: &RangeHeaderFooter) -> bool {
      // java Assert.assertNotNull("range should not be null!", range);
      Assert::assertNotNull_msg("range should not be null!", &*range);
      // java return range.getEnd() - range.getStart() >= 2;
      return range.getEnd() - range.getStart() >= 2;
    }

    // java public static void establishHeaderFooterWithContent(Document document, RangeHeaderFooter range)
    pub fn establishHeaderFooterWithContent(document: &mut dyn Document, range: &mut RangeHeaderFooter) {
      // java TextDocument textDocument = document.getTextDocument();
      let mut textDocument: &dyn SoftTextDocument = document.getTextDocument().expect("getTextDocument");
      // java Assert.assertNotNull("textDocument should not be null!", textDocument);
      Assert::assertNotNull_msg("textDocument should not be null!", &textDocument);
      // java Transaction transaction = textDocument.getTransaction();
      // rust: Java 是引用语义（`Transaction` 无 `Clone`）⇒ 保持 `&Transaction`。
      let mut transaction: &Transaction = textDocument.getTransaction().expect("getTransaction");
      // java Assert.assertNotNull("transaction should not be null!", transaction);
      Assert::assertNotNull_msg("transaction should not be null!", &transaction);
      // java establishHeaderFooter(range.mStart, document.getPLCHeader());
      Self::establishHeaderFooter(
        range.mStart,
        document.getPLCHeader_mut().expect("getPLCHeader"),
      );
      // java int endCp = range.getEnd();
      let mut endCp: i32 = range.getEnd();
      // java range.setRange(endCp, endCp);
      range.setRange(endCp, endCp);
      // java range.setParagraphProperty(createParagraphPropertyHeaderFooter(), 2);
      range.setParagraphProperty(Self::createParagraphPropertyHeaderFooter(), 2);
      // java range.insertParagraph();
      range.insertParagraph();
    }

    // java private static PropertySet createParagraphPropertyHeaderFooter()
    pub fn createParagraphPropertyHeaderFooter() -> PropertySet {
      // java MutablePropertySet mutablePropertySet = new MutablePropertySet();
      let mut mutablePropertySet: MutablePropertySet = MutablePropertySet::new();
      // java mutablePropertySet.setInt(193, 1);
      mutablePropertySet.setInt(193, 1);
      // java return mutablePropertySet.freeze();
      return mutablePropertySet.freeze();
    }

    // java public static void establishHeaderFooter(PLC.Node start, PLCHeader plcHeader)
    pub fn establishHeaderFooter(start: Node, plcHeader: &mut PLCHeader) {
      // java Assert.assertNotNull("start should not be null!", start);
      Assert::assertNotNull_msg("start should not be null!", &start);
      // java Assert.assertNotNull("plcHeader should not be null!", plcHeader);
      Assert::assertNotNull_msg("plcHeader should not be null!", &*plcHeader);
      // java PLC.Node next = start.getNext();
      let mut next: Node = start.getNext();
      // java Assert.assertNotNull("next should not be null!", next);
      Assert::assertNotNull_msg("next should not be null!", &next);
      // java Document documentHeaderFooter = plcHeader.getDocument();
      let documentHeaderFooter: *mut dyn Document = plcHeader
        .getDocument_mut()
        .expect("documentHeaderFooter") as *mut dyn Document;
      // java int cpStart = start.getCP();
      let mut cpStart: i32 = start.getCP();
      // java int cpEnd = next.getCP();
      let mut cpEnd: i32 = next.getCP();
      // java TextDocument textDocument = documentHeaderFooter.getTextDocument();
      let textDocument: *mut dyn SoftTextDocument = unsafe {
        (*documentHeaderFooter)
          .asAny_mut()
          .downcast_mut::<SubDocument>()
          .expect("SubDocument")
          .getTextDocument() as *mut dyn SoftTextDocument
      };
      {
        let textDocument = unsafe { &mut *textDocument };
        Assert::assertNotNull_msg("textDocument should not be null!", &textDocument);
        let mut transaction: &Transaction = textDocument.getTransaction().expect("getTransaction");
        Assert::assertNotNull_msg("transaction should not be null!", &transaction);
        // java textDocument.start();
        textDocument.start();
      }
          // java insertParagraph((SubDocument)documentHeaderFooter, '\r', cpEnd, PropertySet.empty);
          Self::insertParagraph(
            unsafe { &mut *documentHeaderFooter }
              .asAny_mut()
              .downcast_mut::<SubDocument>()
              .expect("SubDocument"),
            '\r',
            cpEnd,
            &*empty,
          );
          let mut node: Node = start.getNext();
          // java for(PLC.Node node = start.getNext(); !node.isHead() && cpStart == node.getCP() node = node.getNext())
          while !node.isHead() && cpStart == node.getCP() {
            // java plcHeader.moveRight(node, 1);
            plcHeader.moveRight(node, 1);
            node = node.getNext();
          }
            // java textDocument.commit("establishHeaderFooter");
            unsafe { (*textDocument).commit("establishHeaderFooter"); }
    }

    // java public static RangeHeaderFooter getRangeHeader(Document document, PLCSection.SectionNode sectionNode, KHeaderFooterIndex index)
    pub fn getRangeHeader(document: &dyn Document, sectionNode: &SectionNode, index: KHeaderFooterIndex) -> Option<RangeHeaderFooter> {
      // java PLC.Node start = null;
      let mut start: Node = Node::Node();
      // java PLC.Node end = null;
      let mut end: Node = Node::Node();
      // java switch (index)
      match index {
        // java case HeaderFooterPrimary:
        KHeaderFooterIndex::HeaderFooterPrimary => {
          // java start = sectionNode.getOddHeader();
          start = sectionNode.getOddHeader();
          // java end = sectionNode.getEvenFooter();
          end = sectionNode.getEvenFooter();
        }
        // java case HeaderFooterEvenPages:
        KHeaderFooterIndex::HeaderFooterEvenPages => {
          // java start = sectionNode.getEvenHeader();
          start = sectionNode.getEvenHeader();
          // java end = sectionNode.getOddHeader();
          end = sectionNode.getOddHeader();
        }
        // java case HeaderFooterFirstPage:
        KHeaderFooterIndex::HeaderFooterFirstPage => {
          // java start = sectionNode.getFirstHeader();
          start = sectionNode.getFirstHeader();
          // java end = sectionNode.getFirstFooter();
          end = sectionNode.getFirstFooter();
        }
        // java default:
        _ => {
          // java Assert.fail("It should not reach here!");
          Assert::fail_msg("It should not reach here!");
          // java return null;
          return None;
        }
      }
      // java Assert.assertNotNull("start should not be null!", start);
      Assert::assertNotNull_msg("start should not be null!", &start);
      // java Assert.assertNotNull("end should not be null!", end);
      Assert::assertNotNull_msg("end should not be null!", &end);
      // java return new RangeHeaderFooter(document, start, end);
      return Some(RangeHeaderFooter::RangeHeaderFooter(document, start, end));
    }

    // java public static RangeHeaderFooter getRangeFooter(Document document, PLCSection.SectionNode sectionNode, KHeaderFooterIndex index)
    pub fn getRangeFooter(document: &dyn Document, sectionNode: &SectionNode, index: KHeaderFooterIndex) -> Option<RangeHeaderFooter> {
      // java PLC.Node next = sectionNode.getNext();
      let mut next: Node = sectionNode.getNext();
      // java PLCSection.SectionNode nextSectionNode = next instanceof PLCSection.SectionNode ? (PLCSection.SectionNode)next : null;
      let mut nextSectionNode: SectionNode = (if next.isHead() || next.ptr.is_null() { SectionNode::default() } else { as_section(next) });
      // java PLC.Node start = null;
      let mut start: Node = Node::Node();
      // java PLC.Node end = null;
      let mut end: Node = Node::Node();
      // java switch (index)
      match index {
        // java case HeaderFooterPrimary:
        KHeaderFooterIndex::HeaderFooterPrimary => {
          // java start = sectionNode.getOddFooter();
          start = sectionNode.getOddFooter();
          // java end = sectionNode.getFirstHeader();
          end = sectionNode.getFirstHeader();
        }
        // java case HeaderFooterEvenPages:
        KHeaderFooterIndex::HeaderFooterEvenPages => {
          // java start = sectionNode.getEvenFooter();
          start = sectionNode.getEvenFooter();
          // java end = sectionNode.getOddFooter();
          end = sectionNode.getOddFooter();
        }
        // java case HeaderFooterFirstPage:
        KHeaderFooterIndex::HeaderFooterFirstPage => {
          // java start = sectionNode.getFirstFooter();
          start = sectionNode.getFirstFooter();
          // java end = null == nextSectionNode ? null : nextSectionNode.getEvenHeader();
          end = (if nextSectionNode.ptr.is_null() { Node::Node() } else { nextSectionNode.getEvenHeader() });
        }
        // java default:
        _ => {
          // java Assert.fail("It should not reach here!");
          Assert::fail_msg("It should not reach here!");
          // java return null;
          return None;
        }
      }
      // java Assert.assertNotNull("start should not be null!", start);
      Assert::assertNotNull_msg("start should not be null!", &start);
      // java return new RangeHeaderFooter(document, start, end);
      return Some(RangeHeaderFooter::RangeHeaderFooter(document, start, end));
    }

    // java private static IPapxTable.Entry insertParagraph(SubDocument document, char ch, int cp, PropertySet papx)
    pub fn insertParagraph(document: &mut SubDocument, ch: char, cp: i32, papx: &PropertySet) -> Option<Box<dyn IPapxEntry>> {
      // java Assert.assertNotNull("document should not be null", document);
      Assert::assertNotNull_msg("document should not be null", &*document);
      // java Assert.assertNotNull("papx should not be null", papx);
      Assert::assertNotNull_msg("papx should not be null", papx);
      // java IPapxTable papxTable = document.getPapxTable();
      // rust: 门面返回擦除指针 `Option<*mut dyn IPapxTable>`（`Document.rs:84`）
      //   ⇒ 与 `TextLayouter.rs:494` 一致：解引用裸指针，**不可** `Box::from_raw`
      //   （那会夺取所有权并在作用域结束时释放非本函数持有的对象）。
      // rust: ★T7 —— 必须 UFCS 走 `Document` trait（返回 `*mut dyn`，对应 Java 可写引用）；
      //   直接 `document.getPapxTable()` 会经 `Deref` 命中 `SubDocument` 的固有
      //   只读 `&dyn` 版本 ⇒ 后续 `insert` 无 `&mut` 可用。
      let mut papxTable: *mut dyn IPapxTable =
        <dyn Document as Document>::getPapxTable(document).expect("getPapxTable");
      // java Assert.assertNotNull("papxTable should not be null!", papxTable);
      Assert::assertNotNull_msg("papxTable should not be null!", &papxTable);
      // java insertText(document, ch, cp, PropertySet.empty);
      Self::insertText__subdocument_char_int_propertyset(document, ch, cp, &*empty);
      // java IPapxTable.Entry entry = papxTable.seek(cp);
      // rust: `IPapxTable::seek` 门面返回 `Option<Box<dyn Entry>>`；Java 侧恒非 null
      //   （紧随其后的 `assertNotNull` 亦如此断言）⇒ 解包为具体句柄。
      let mut entry: Box<dyn IPapxEntry> =
        IPapxTable::seek(unsafe { &*papxTable }, cp).expect("entry");
      // java Assert.assertNotNull("entry should not be null", entry);
      Assert::assertNotNull_msg("entry should not be null", &entry);
      // java if (cp != entry.position())
      if cp != ITableEntry::position(&*entry) {
        // java return papxTable.insert(cp, papx);
        return IPapxTable::insert(unsafe { &mut *papxTable }, cp, Some(papx));
      }
      // java else
      else {
        // java Assert.assertFalse("entry.isHead() should be false!", entry.isHead());
        Assert::assertFalse_msg("entry.isHead() should be false!", ITableEntry::isHead(&*entry));
        // java PropertySet preProps = entry.getProperty();
        // rust: 门面返回 `Option<&PropertySet>`；Java 侧紧随的 assertNotNull 保证非 null。
        let mut preProps: PropertySet = <dyn IPapxEntry as IPapxEntry>::getProperty(&*entry).expect("preProps").clone();
        // java Assert.assertNotNull("preProps should not be null", preProps);
        Assert::assertNotNull_msg("preProps should not be null", &preProps);
        // java entry.setProperty(papx);
        <dyn IPapxEntry as IPapxEntry>::setProperty(&mut *entry, Some(papx));
        // java movePrePropsTo(cp + 1, preProps, papxTable);
        Self::movePrePropsTo__int_propertyset_ipapxtable(cp + 1, &preProps, unsafe { &mut *papxTable });
        // java return entry;
        return Some(entry);
      }
    }

    // java private static void movePrePropsTo(int to, PropertySet preProps, IPapxTable papxTable)
    pub fn movePrePropsTo__int_propertyset_ipapxtable(to: i32, preProps: &PropertySet, papxTable: &mut dyn IPapxTable) {
      // java Assert.assertTrue("to >= 0 should be true!", to >= 0);
      Assert::assertTrue_msg("to >= 0 should be true!", to >= 0);
      // java Assert.assertNotNull("preProps should not be null", preProps);
      Assert::assertNotNull_msg("preProps should not be null", &preProps);
      // java Assert.assertNotNull("papxTable should not be null", papxTable);
      Assert::assertNotNull_msg("papxTable should not be null", &*papxTable);
      // java IPapxTable.Entry entry = papxTable.seek(to);
      // rust: 同 `insertTextAt` —— `seek` 门面返回 `Option`，Java 侧恒非 null。
      let mut entry: Box<dyn IPapxEntry> =
        IPapxTable::seek(unsafe { &*papxTable }, to).expect("entry");
      // java if (to != entry.position())
      if to != ITableEntry::position(&*entry) {
        // java papxTable.insert(to, preProps);
        IPapxTable::insert(unsafe { &mut *papxTable }, to, Some(preProps));
      }
      // java else if (!entry.isHead())
      else if !ITableEntry::isHead(&*entry) {
        // java entry.setProperty(preProps);
        <dyn IPapxEntry as IPapxEntry>::setProperty(&mut *entry, Some(preProps));
      }
    }

    // java private static void insertText(SubDocument document, char ch, int cp, PropertySet props)
    pub fn insertText__subdocument_char_int_propertyset(document: &mut SubDocument, ch: char, cp: i32, props: &PropertySet) {
      // java insertText(document, ch + "", cp, props);
      Self::insertText__subdocument_string_int_propertyset(document, &ch.to_string(), cp, props);
    }

    // java private static void insertText(SubDocument document, String text, int cp, PropertySet props)
    pub fn insertText__subdocument_string_int_propertyset(document: &mut SubDocument, text: &str, cp: i32, props: &PropertySet) {
      // java Assert.assertNotNull("document should not be null", document);
      Assert::assertNotNull_msg("document should not be null", &*document);
      // java Assert.assertNotNull("text should not be null", text);
      Assert::assertNotNull_msg("text should not be null", text);
      // java Assert.assertTrue("cp >= 0 should be true!", cp >= 0);
      Assert::assertTrue_msg("cp >= 0 should be true!", cp >= 0);
      // java Assert.assertNotNull("props should not be null", props);
      Assert::assertNotNull_msg("props should not be null", props);
      // java int lengthText = text.length();
      let mut lengthText: i32 = (text.len() as i32);
      // java if (0 != lengthText)
      if 0 != lengthText {
        // java ITextRope textRope = document.getTextRope();
        // rust: 门面返回擦除指针 `Option<*mut dyn ITextRope>`（`Document.rs:74`）
        //   ⇒ 保持裸指针并解引用（同 `TextLayouter` 处理 `getPapxTable` 的方式）。
        // rust: ★T7 —— 同上，UFCS 走 `Document` trait 取可写指针。
        let mut textRope: *mut dyn ITextRope =
          <dyn Document as Document>::getTextRope(document).expect("getTextRope");
        // java Assert.assertNotNull("textRope should not be null", textRope);
        Assert::assertNotNull_msg("textRope should not be null", &textRope);
        // java IChpxTable chpxTable = document.getChpxTable();
        // rust: 门面返回擦除指针 `Option<*mut dyn IChpxTable>`（`Document.rs:79`）
        //   ⇒ 保持裸指针并解引用。
        // rust: ★T7 —— 同上，UFCS 走 `Document` trait 取可写指针。
        let mut chpxTable: *mut dyn IChpxTable =
          <dyn Document as Document>::getChpxTable(document).expect("getChpxTable");
        // java Assert.assertNotNull("chpxTable should not be null", chpxTable);
        Assert::assertNotNull_msg("chpxTable should not be null", &chpxTable);
        // java textRope.insert(cp, (String)text, 0, lengthText);
        unsafe { &mut *textRope }.insert__int_string_int_int(cp, &text.to_string(), 0, lengthText);
        // java IChpxTable.Entry entry = chpxTable.seek(cp);
        // rust: 同 Papx 分支 —— `seek` 门面返回 `Option`，Java 侧恒非 null（紧随断言）。
        let mut entry: Box<dyn IChpxEntry> =
          IChpxTable::seek(unsafe { &*chpxTable }, cp).expect("entry");
        // java Assert.assertNotNull("entry should not be null", entry);
        Assert::assertNotNull_msg("entry should not be null", &entry);
        // java if (cp != entry.position())
        if cp != ITableEntry::position(&*entry) {
          // java chpxTable.insert(cp, props);
          IChpxTable::insert(unsafe { &mut *chpxTable }, cp, Some(props));
        }
        // java else
        else {
          // java Assert.assertFalse("entry.isHead() should be false!", entry.isHead());
          Assert::assertFalse_msg(
            "entry.isHead() should be false!",
            ITableEntry::isHead(&*entry),
          );
          // java PropertySet preProps = entry.getProperty();
          // rust: 门面返回 `Option<&PropertySet>`；Java 侧紧随的 assertNotNull 保证非 null。
          let mut preProps: PropertySet =
            <dyn IChpxEntry as IChpxEntry>::getProperty(&*entry).expect("preProps").clone();
          // java Assert.assertNotNull("preProps should not be null", preProps);
          Assert::assertNotNull_msg("preProps should not be null", &preProps);
          // java entry.setProperty(props);
          <dyn IChpxEntry as IChpxEntry>::setProperty(&mut *entry, Some(props));
          // java movePrePropsTo(cp + lengthText, preProps, chpxTable);
          Self::movePrePropsTo__int_propertyset_ichpxtable(cp + lengthText, &preProps, unsafe { &mut *chpxTable });
        }
      }
    }

    // java private static void movePrePropsTo(int to, PropertySet preProps, IChpxTable chpxTable)
    pub fn movePrePropsTo__int_propertyset_ichpxtable(to: i32, preProps: &PropertySet, chpxTable: &mut dyn IChpxTable) {
      // java Assert.assertTrue("to >= 0 should be true!", to >= 0);
      Assert::assertTrue_msg("to >= 0 should be true!", to >= 0);
      // java Assert.assertNotNull("preProps should not be null", preProps);
      Assert::assertNotNull_msg("preProps should not be null", &preProps);
      // java Assert.assertNotNull("chpxTable should not be null", chpxTable);
      Assert::assertNotNull_msg("chpxTable should not be null", &chpxTable);
      // java IChpxTable.Entry entry = chpxTable.seek(to);
      // rust: 同 Papx 版 `movePrePropsTo` —— `seek` 门面返回 `Option`，Java 侧恒非 null。
      let mut entry: Box<dyn IChpxEntry> =
        IChpxTable::seek(unsafe { &*chpxTable }, to).expect("entry");
      // java if (to != entry.position())
      if to != ITableEntry::position(&*entry) {
        // java chpxTable.insert(to, preProps);
        IChpxTable::insert(unsafe { &mut *chpxTable }, to, Some(preProps));
      }
      // java else if (!entry.isHead())
      else if !ITableEntry::isHead(&*entry) {
        // java entry.setProperty(preProps);
        <dyn IChpxEntry as IChpxEntry>::setProperty(&mut *entry, Some(preProps));
      }
    }

    // java private static void appendEvenHeader(PLCSection.SectionNode sectionNode, PLCHeader plcHeader, int length)
    pub fn appendEvenHeader(sectionNode: &mut SectionNode, plcHeader: &mut PLCHeader, length: i32) {
      // java Assert.assertNotNull("sectionNode should not be null!", sectionNode);
      Assert::assertNotNull_msg("sectionNode should not be null!", &*sectionNode);
      // java Assert.assertNotNull("plcHeader should not be null!", plcHeader);
      Assert::assertNotNull_msg("plcHeader should not be null!", &*plcHeader);
      // java sectionNode.setEvenHeader(plcHeader.addNodeEvenHeader(length));
      sectionNode.setEvenHeader(plcHeader.addNodeEvenHeader(length));
    }

    // java private static void appendOddHeader(PLCSection.SectionNode sectionNode, PLCHeader plcHeader, int length)
    pub fn appendOddHeader(sectionNode: &mut SectionNode, plcHeader: &mut PLCHeader, length: i32) {
      // java Assert.assertNotNull("sectionNode should not be null!", sectionNode);
      Assert::assertNotNull_msg("sectionNode should not be null!", &*sectionNode);
      // java Assert.assertNotNull("plcHeader should not be null!", plcHeader);
      Assert::assertNotNull_msg("plcHeader should not be null!", &*plcHeader);
      // java sectionNode.setOddHeader(plcHeader.addNodeOddHeader(length));
      sectionNode.setOddHeader(plcHeader.addNodeOddHeader(length));
    }

    // java private static void appendEvenFooter(PLCSection.SectionNode sectionNode, PLCHeader plcHeader, int length)
    pub fn appendEvenFooter(sectionNode: &mut SectionNode, plcHeader: &mut PLCHeader, length: i32) {
      // java Assert.assertNotNull("sectionNode should not be null!", sectionNode);
      Assert::assertNotNull_msg("sectionNode should not be null!", &*sectionNode);
      // java Assert.assertNotNull("plcHeader should not be null!", plcHeader);
      Assert::assertNotNull_msg("plcHeader should not be null!", &*plcHeader);
      // java sectionNode.setEvenFooter(plcHeader.addNodeEvenFooter(length));
      sectionNode.setEvenFooter(plcHeader.addNodeEvenFooter(length));
    }

    // java private static void appendOddFooter(PLCSection.SectionNode sectionNode, PLCHeader plcHeader, int length)
    pub fn appendOddFooter(sectionNode: &mut SectionNode, plcHeader: &mut PLCHeader, length: i32) {
      // java Assert.assertNotNull("sectionNode should not be null!", sectionNode);
      Assert::assertNotNull_msg("sectionNode should not be null!", &*sectionNode);
      // java Assert.assertNotNull("plcHeader should not be null!", plcHeader);
      Assert::assertNotNull_msg("plcHeader should not be null!", &*plcHeader);
      // java sectionNode.setOddFooter(plcHeader.addNodeOddFooter(length));
      sectionNode.setOddFooter(plcHeader.addNodeOddFooter(length));
    }

    // java private static void appendFirstHeader(PLCSection.SectionNode sectionNode, PLCHeader plcHeader, int length)
    pub fn appendFirstHeader(sectionNode: &mut SectionNode, plcHeader: &mut PLCHeader, length: i32) {
      // java Assert.assertNotNull("sectionNode should not be null!", sectionNode);
      Assert::assertNotNull_msg("sectionNode should not be null!", &*sectionNode);
      // java Assert.assertNotNull("plcHeader should not be null!", plcHeader);
      Assert::assertNotNull_msg("plcHeader should not be null!", &*plcHeader);
      // java sectionNode.setFirstHeader(plcHeader.addNodeFirstHeader(length));
      sectionNode.setFirstHeader(plcHeader.addNodeFirstHeader(length));
    }

    // java private static void appendFirstFooter(PLCSection.SectionNode sectionNode, PLCHeader plcHeader, int length)
    pub fn appendFirstFooter(sectionNode: &mut SectionNode, plcHeader: &mut PLCHeader, length: i32) {
      // java Assert.assertNotNull("sectionNode should not be null!", sectionNode);
      Assert::assertNotNull_msg("sectionNode should not be null!", &*sectionNode);
      // java Assert.assertNotNull("plcHeader should not be null!", plcHeader);
      Assert::assertNotNull_msg("plcHeader should not be null!", &*plcHeader);
      // java sectionNode.setFirstFooter(plcHeader.addNodeFirstFooter(length));
      sectionNode.setFirstFooter(plcHeader.addNodeFirstFooter(length));
    }

    // java private static void debugCheck_appendHeaderFooterNodeSectionNode(PLCSection.SectionNode sectionNode)
    pub fn debugCheck_appendHeaderFooterNodeSectionNode(sectionNode: &SectionNode) {
    }

    // java public static boolean fSectionNodeHasContent(Document document, PLCSection.SectionNode sectionNode)
    pub fn fSectionNodeHasContent(document: &dyn Document, sectionNode: &SectionNode) -> bool {
      // java if (hasFooterEven(sectionNode, document))
      if Self::hasFooterEven(sectionNode, document) {
        // java return true;
        return true;
      }
      // java else if (hasFooterOdd(sectionNode, document))
      else if Self::hasFooterOdd(sectionNode, document) {
        // java return true;
        return true;
      }
      // java else if (hasHeaderEven(sectionNode, document))
      else if Self::hasHeaderEven(sectionNode, document) {
        // java return true;
        return true;
      }
      // java else if (hasHeaderOdd(sectionNode, document))
      else if Self::hasHeaderOdd(sectionNode, document) {
        // java return true;
        return true;
      }
      // java else if (hasFooterFirst(sectionNode, document))
      else if Self::hasFooterFirst(sectionNode, document) {
        // java return true;
        return true;
      }
      // java else
      else {
        // java return hasHeaderFirst(sectionNode, document);
        return Self::hasHeaderFirst(sectionNode, document);
      }
    }

    // java public static void addHeaderFooterNode(PLCSection.SectionNode sectionNode, PLCHeader plcHeader)
    pub fn addHeaderFooterNode(sectionNode: &mut SectionNode, plcHeader: &mut PLCHeader) {
      // java Assert.assertNotNull("sectionNode should not be null!", sectionNode);
      Assert::assertNotNull_msg("sectionNode should not be null!", &*sectionNode);
      // java Assert.assertNotNull("plcHeader should not be null!", plcHeader);
      Assert::assertNotNull_msg("plcHeader should not be null!", &*plcHeader);
      // java Document documentHeaderFooter = plcHeader.getDocument();
      let documentHeaderFooter: *mut dyn Document = plcHeader
        .getDocument_mut()
        .expect("documentHeaderFooter") as *mut dyn Document;
      let documentHeaderFooter_ref = unsafe { &*documentHeaderFooter };
      // java Assert.assertNotNull("documentHeaderFooter should not be null!", documentHeaderFooter);
      Assert::assertNotNull_msg("documentHeaderFooter should not be null!", documentHeaderFooter_ref);
      // java Assert.assertTrue("Document.HEADER_DOCUMENT == documentHeaderFooter.getType() should be true!", 2 == documentHeaderFooter.getType());
      Assert::assertTrue_msg("Document.HEADER_DOCUMENT == documentHeaderFooter.getType() should be true!", 2 == documentHeaderFooter_ref.getType());
      // java debugCheck_appendHeaderFooterNodeSectionNode(sectionNode);
      Self::debugCheck_appendHeaderFooterNodeSectionNode(sectionNode);
      // java PLC.Node evenHeader = sectionNode.getEvenHeader();
      let mut evenHeader: Node = sectionNode.getEvenHeader();
      // java if (null == evenHeader)
      if evenHeader.ptr.is_null() {
        // java TextDocument textDocument = documentHeaderFooter.getTextDocument();
        // java RangeWriteLock wrietLock = textDocument.getMainDocument().writeLock();
        let mut wrietLock: RangeWriteLock = unsafe { (*documentHeaderFooter).writeLock() };
            // java int index = textDocument.getMainDocument().getPLCSection().getIndex(sectionNode) * 6;
            let mut index: i32 = unsafe {
              (*documentHeaderFooter)
                .asAny_mut()
                .downcast_mut::<SubDocument>()
                .expect("SubDocument")
                .getPLCSection()
                .expect("getPLCSection")
                .getIndex(sectionNode)
                * 6
            };
            // java int lastIndex = index > 0 ? index - 1 : index;
            let mut lastIndex: i32 = (if index > 0 { index - 1 } else { index });
            // java int cp = plcHeader.get(lastIndex).position();
            let mut cp: i32 = plcHeader.get(lastIndex).position();
            // java sectionNode.setEvenHeader(plcHeader.insertNodeEvenHeader(cp, index));
            sectionNode.setEvenHeader(plcHeader.insertNodeEvenHeader(cp, index));
            // java sectionNode.setOddHeader(plcHeader.insertNodeOddHeader(cp, index + 1));
            sectionNode.setOddHeader(plcHeader.insertNodeOddHeader(cp, index + 1));
            // java sectionNode.setEvenFooter(plcHeader.insertNodeOddHeader(cp, index + 2));
            sectionNode.setEvenFooter(plcHeader.insertNodeOddHeader(cp, index + 2));
            // java sectionNode.setOddFooter(plcHeader.insertNodeOddHeader(cp, index + 3));
            sectionNode.setOddFooter(plcHeader.insertNodeOddHeader(cp, index + 3));
            // java sectionNode.setFirstHeader(plcHeader.insertNodeOddHeader(cp, index + 4));
            sectionNode.setFirstHeader(plcHeader.insertNodeOddHeader(cp, index + 4));
            // java sectionNode.setFirstFooter(plcHeader.insertNodeOddHeader(cp, index + 5));
            sectionNode.setFirstFooter(plcHeader.insertNodeOddHeader(cp, index + 5));
              // java wrietLock.unlock();
              wrietLock.unlock();
      }
    }

}

// java public static class RangeHeaderFooter extends KRange
impl RangeHeaderFooter {
    // java public  RangeHeaderFooter(Document document, PLC.Node start, PLC.Node end)
    pub fn RangeHeaderFooter(document: &dyn Document, start: Node, end: Node) -> Self {
      // java super(document, start.getCP(), end == null ? document.getLength() : end.getCP());
      // java this.mStart = start;
      Self { super_: KRange::new__document_int_int(document, start.getCP(), (if end.ptr.is_null() { document.getLength() } else { end.getCP() })), mStart: start }
    }

}

impl std::ops::Deref for RangeHeaderFooter {
    type Target = KRange;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for RangeHeaderFooter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
