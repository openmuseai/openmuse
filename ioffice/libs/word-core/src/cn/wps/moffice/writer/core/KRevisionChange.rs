// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.table.KTableRangeBase
use crate::cn::wps::moffice::writer::core::table::KTableRangeBase::*;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::{
    Entry as IChpxTableEntry, IChpxTable,
};
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::{
    Entry as IPapxTableEntry, IPapxTable,
};
// Java: import cn.wps.moffice.writer.data.ITable
use crate::cn::wps::moffice::writer::data::ITable::{
    Entry as ITableEntry, ITable,
};
// Java: import cn.wps.moffice.writer.data.ITextStream
use crate::cn::wps::moffice::writer::data::ITextStream::*;
// Java: import cn.wps.moffice.writer.data.PLC
use crate::cn::wps::moffice::writer::data::PLC::{Node as PLCNode, PLC};
// Java: import cn.wps.moffice.writer.data.PLCField
use crate::cn::wps::moffice::writer::data::PLCField::*;
// Java: import cn.wps.moffice.writer.data.PLCMathEnd
use crate::cn::wps::moffice::writer::data::PLCMathEnd::*;
// Java: import cn.wps.moffice.writer.data.PLCMathStart
use crate::cn::wps::moffice::writer::data::PLCMathStart::*;
// Java: import cn.wps.moffice.writer.data.PropertyAction
use crate::cn::wps::moffice::writer::data::PropertyAction::*;
// Java: import cn.wps.moffice.writer.data.PropertyOperations
use crate::cn::wps::moffice::writer::data::PropertyOperations::*;
// Java: import cn.wps.moffice.writer.data.PropertyUtil
use crate::cn::wps::moffice::writer::data::PropertyUtil::*;
// Java: import cn.wps.moffice.writer.data.RANGE
use crate::cn::wps::moffice::writer::data::RANGE::*;
// Java: import cn.wps.moffice.writer.data.table.RowInfo
use crate::cn::wps::moffice::writer::data::table::RowInfo::*;
// Java: import cn.wps.moffice.writer.data.table.TableInfo
use crate::cn::wps::moffice::writer::data::table::TableInfo::*;
// Java: import cn.wps.moffice.writer.data.value.Revision
use crate::cn::wps::moffice::writer::data::value::Revision::*;
// Java: import java.util.Collections
// Java: import java.util.Map
// Java: import java.util.TreeMap
use stubs::java::util::TreeMap::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.SubDocument
use crate::cn::wps::moffice::writer::core::SubDocument::*;
// Java: import cn.wps.moffice.writer.core.KTableRevisionChange
use crate::cn::wps::moffice::writer::core::KTableRevisionChange::*;
// Java: import cn.wps.moffice.writer.core.KSectionRevision
use crate::cn::wps::moffice::writer::core::KSectionRevision::*;
// Java: import org.apache.poi.poifs.filesystem.Entry
// (parked — ambiguous with ITable/IChpxTable/IPapxTable::Entry)
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
// Java: import cn.wps.io.dom.Node
// (parked — PLC.Node used via PLCField)
use std::collections::{BTreeMap, HashMap};

// java public class KRevisionChange
pub struct KRevisionChange {
    // java private Document mDocument;
    mDocument: *mut dyn Document,
    // java private KRange mKRange;
    mKRange: Option<*mut KRange>,
}

// java public static enum RevisionChange
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RevisionChange {
    ACCEPT,
    REJECT,
}

// java public static enum RevisionType
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RevisionType {
    DELETE,
    INSERT,
    RUN_FORMAT,
    PARAGRAPH_FORMAT,
    TABLE_FORMAT,
    SECTION_FORMAT,
}

// java public class KRevisionChange
impl KRevisionChange {
    fn doc(&self) -> &dyn Document {
        unsafe { &*self.mDocument }
    }
    fn doc_mut(&mut self) -> &mut dyn Document {
        unsafe { &mut *self.mDocument }
    }

    // java public  KRevisionChange(Document document)
    pub fn new__document(document: &mut dyn Document) -> Self {
      // java this.mDocument = document;
      Self {
        mDocument: document as *const dyn Document as *mut dyn Document,
        mKRange: None,
      }
    }

    // java public  KRevisionChange(Document document, KRange kRange)
    pub fn new__document_krange(document: &mut dyn Document, kRange: &mut KRange) -> Self {
      // java this(document);
      let mut this = Self::new__document(document);
      // java this.mKRange = kRange;
      this.mKRange = Some(kRange as *const KRange as *mut KRange);
      this
    }

    // java public void changeRevision(int startCp, int endCp, RevisionType revisionType, RevisionChange change)
    pub fn changeRevision(&mut self, startCp: i32, endCp: i32, revisionType: RevisionType, change: RevisionChange) {
      // java if (startCp < endCp)
      if startCp < endCp {
        // java this.mDocument.getTextDocument().start();
        self.doc().getTextDocument().start();
            // java SubDocument d = (SubDocument)this.mDocument;
            let d = self.doc();
            // java d.fireBeforeChange(startCp, endCp, revisionType, change);
            d.fireBeforeChange(startCp, endCp, revisionType, change);
            // java ITextStream textStream = this.mDocument.getTextStream();
            let mut textStream: Box<dyn ITextStream> = self.doc().getTextStream();
            // java textStream.disableRevisionFileter();
            textStream.disableRevisionFileter();
            // java switch (revisionType)
            match revisionType {
              // java case DELETE:
              RevisionType::DELETE => {
                // java this.changeDeleteRevision(startCp, endCp, textStream, change);
                self.changeDeleteRevision(startCp, endCp, textStream.as_mut(), change);
              }
              // java case INSERT:
              RevisionType::INSERT => {
                // java this.changeInsertRevision(startCp, endCp, textStream, change);
                self.changeInsertRevision(startCp, endCp, textStream.as_mut(), change);
              }
              // java case RUN_FORMAT:
              RevisionType::RUN_FORMAT => {
                // java this.changeRunFormatRevision(startCp, endCp, textStream, change);
                self.changeRunFormatRevision(startCp, endCp, textStream.as_mut(), change);
              }
              // java case PARAGRAPH_FORMAT:
              RevisionType::PARAGRAPH_FORMAT => {
                // java this.changeParagraphFormatRevision(startCp, endCp, textStream, change);
                self.changeParagraphFormatRevision(startCp, endCp, textStream.as_mut(), change);
              }
              // java case TABLE_FORMAT:
              RevisionType::TABLE_FORMAT => {
                // java KTableRevisionChange kTableRevisionChange = new KTableRevisionChange(this.mDocument);
                let mut kTableRevisionChange: KTableRevisionChange = KTableRevisionChange::KTableRevisionChange(self.doc());
                // java kTableRevisionChange.changeTableFormatRevision(startCp, endCp, textStream.as_ref(), change);
                kTableRevisionChange.changeTableFormatRevision(startCp, endCp, textStream.as_ref(), change);
              }
              // java case SECTION_FORMAT:
              RevisionType::SECTION_FORMAT => {
                // java KSectionRevision sectionRevision = new KSectionRevision(this.mDocument.getTextDocument());
                let mut sectionRevision: KSectionRevision = KSectionRevision::new();
                // java sectionRevision.changeSectionFormatRevision(startCp, endCp, change);
                sectionRevision.changeSectionFormatRevision(startCp, endCp, change);
              }
              // java default:
              _ => {
                // java Assert.fail("It should not reach here!");
                Assert::fail__string("It should not reach here!");
              }
            }
            // java textStream.enableRevisionFileter();
            textStream.enableRevisionFileter();
            // java d.fireAfterChange(startCp, endCp, revisionType, change);
            d.fireAfterChange(startCp, endCp, revisionType, change);
              // java this.mDocument.getTextDocument().commit("change Run Revision");
              self.doc().getTextDocument().commit("change Run Revision");
      }
    }

    // java static boolean checkSubDocumentRevision(Document document)
    pub fn checkSubDocumentRevision(document: &dyn Document) -> bool {
      // java Assert.assertNotNull("document should not be null !", document);
      Assert::assertNotNull__string_object("document should not be null !", document);
      // java return checkParagraphRevision(document) || checkRunRevision(document) || KTableRevisionChange.checkTableRevision(document);
      return Self::checkParagraphRevision(document) || Self::checkRunRevision(document) || KTableRevisionChange::checkTableRevision(document);
    }

    // java  void changeAll(RevisionChange change)
    pub fn changeAll(&mut self, change: RevisionChange) {
      // java ITextStream textStream = this.mDocument.getTextStream();
      let mut textStream: Box<dyn ITextStream> = self.doc().getTextStream();
      // java KTableRevisionChange kTableRevisionChange = new KTableRevisionChange(this.mDocument);
      let mut kTableRevisionChange: KTableRevisionChange = KTableRevisionChange::KTableRevisionChange(self.doc());
      // java kTableRevisionChange.changeAllTableFormatRevision(textStream, change);
      kTableRevisionChange.changeAllTableFormatRevision(textStream, change);
      // java this.changeAllRunRevision(textStream, change);
      self.changeAllRunRevision(textStream.as_mut(), change);
      // java this.changeAllParagraphRevision(textStream, change);
      self.changeAllParagraphRevision(textStream.as_mut(), change);
      // java this.changeAllSectionRevision(change);
      self.changeAllSectionRevision(change);
    }

    // java private void changeAllRunRevision(ITextStream textStream, RevisionChange change)
    pub fn changeAllRunRevision(&mut self, textStream: &mut dyn ITextStream, change: RevisionChange) {
      // java Assert.assertNotNull("textStream should not be null!", textStream);
      Assert::assertNotNull__string_object("textStream should not be null!", textStream);
      // java Assert.assertNotNull("change should not be null!", change);
      Assert::assertNotNull__string_object("change should not be null!", change);
      // java this.changeAllDeleteRunRevision(textStream, change);
      self.changeAllDeleteRunRevision(textStream, change);
      // java this.changeAllInsertRevision(textStream, change);
      self.changeAllInsertRevision(textStream, change);
      // java this.changeAllRunForamtRevision(textStream, change);
      self.changeAllRunForamtRevision(textStream, change);
    }

    // java private void changeAllDeleteRunRevision(ITextStream textStream, RevisionChange change)
    pub fn changeAllDeleteRunRevision(&mut self, textStream: &mut dyn ITextStream, change: RevisionChange) {
      // java textStream.disableRevisionFileter();
      textStream.disableRevisionFileter();
      // java this.changeDeleteRevision(0, this.mDocument.getLength(), textStream, change);
      self.changeDeleteRevision(0, self.doc().getLength(), textStream, change);
      // java textStream.enableRevisionFileter();
      textStream.enableRevisionFileter();
    }

    // java private void changeAllInsertRevision(ITextStream textStream, RevisionChange change)
    pub fn changeAllInsertRevision(&mut self, textStream: &mut dyn ITextStream, change: RevisionChange) {
      // java textStream.disableRevisionFileter();
      textStream.disableRevisionFileter();
      // java this.changeInsertRevision(0, this.mDocument.getLength(), textStream, change);
      self.changeInsertRevision(0, self.doc().getLength(), textStream, change);
      // java textStream.enableRevisionFileter();
      textStream.enableRevisionFileter();
    }

    // java private void changeAllRunForamtRevision(ITextStream textStream, RevisionChange change)
    pub fn changeAllRunForamtRevision(&mut self, textStream: &mut dyn ITextStream, change: RevisionChange) {
      // java textStream.disableRevisionFileter();
      textStream.disableRevisionFileter();
      // java this.changeRunFormatRevision(0, this.mDocument.getLength(), textStream, change);
      self.changeRunFormatRevision(0, self.doc().getLength(), textStream, change);
      // java textStream.enableRevisionFileter();
      textStream.enableRevisionFileter();
    }

    // java private void changeAllParagraphRevision(ITextStream textStream, RevisionChange change)
    pub fn changeAllParagraphRevision(&mut self, textStream: &mut dyn ITextStream, change: RevisionChange) {
      // java ITextStream iTextStream = this.mDocument.getTextStream();
      let mut iTextStream: Box<dyn ITextStream> = self.doc().getTextStream();
      // java iTextStream.disableRevisionFileter();
      iTextStream.disableRevisionFileter();
      // java this.changeParagraphFormatRevision(0, this.mDocument.getLength(), iTextStream, change);
      self.changeParagraphFormatRevision(0, self.doc().getLength(), textStream.as_mut(), change);
      // java iTextStream.enableRevisionFileter();
      iTextStream.enableRevisionFileter();
    }

    // java private void changeAllSectionRevision(RevisionChange change)
    pub fn changeAllSectionRevision(&mut self, change: RevisionChange) {
      // java KSectionRevision sectionRevision = new KSectionRevision(this.mDocument.getTextDocument());
      let mut sectionRevision: KSectionRevision = KSectionRevision::new();
      // java sectionRevision.changeSectionFormatRevision(0, this.mDocument.getMainDocument().getLength(), change);
      sectionRevision.changeSectionFormatRevision(0, self.doc().getMainDocument().getLength(), change);
    }

    // java private static void rejectSingleRunFormatRevision(int startCp, int endCp, IChpxTable.Entry entry, ITextStream textStream)
    pub fn rejectSingleRunFormatRevision(startCp: i32, endCp: i32, entry: &dyn IChpxTableEntry, textStream: &mut dyn ITextStream) {
      // java final MutablePropertySet mutaProp = new MutablePropertySet();
      let mut mutaProp: MutablePropertySet = MutablePropertySet::new();
      // java PropertySet prop = entry.getProperty();
      let mut prop: PropertySet = entry.getProperty();
      // java Revision revision = (Revision)prop.getValue(48);
      let mut revision: Revision = (prop.getValue(48)).downcast_ref::<Revision>().unwrap();
      // java Assert.assertNotNull("revision should not be null!", revision);
      Assert::assertNotNull__string_object("revision should not be null!", revision);
      // java mutaProp.setValue(48, revision);
      mutaProp.setValue(48, revision);
      // java mutaProp.setBoolean(47, true);
      mutaProp.setBoolean(47, true);
      // java Object id = prop.getValue(51);
      let mut id: Box<dyn Object> = prop.getValue(51);
      // java if (null != id)
      if true {
        // java mutaProp.setInt(51, (Integer)id);
        mutaProp.setInt(51, ((id) as i32));
      }
      // java textStream.modifyChpx(startCp, entry.getNext().position(), new ITextStream.IPropertyModify() { public void compose(ITable.Entry entry, PropertyAction action) { action.addOperation(PropertyOperations.create(3, mutaProp.freeze())); } });
      textStream.modifyChpx(startCp, entry.getNext().position(), &EmptyPropertyModify::new());
    }

    // java private void changeParagraphFormatRevision(int startCp, int endCp, ITextStream textStream, RevisionChange change)
    pub fn changeParagraphFormatRevision(&mut self, startCp: i32, endCp: i32, textStream: &mut dyn ITextStream, change: RevisionChange) {
      // java IPapxTable papx = this.mDocument.getPapxTable();
      let mut papx: Box<dyn IPapxTable> = self.doc().getPapxTable();
      // java IPapxTable.Entry startEntry = papx.seek(startCp);
      let mut startEntry: Box<dyn IPapxTableEntry> = papx.seek(startCp);
      // java IPapxTable.Entry endEntry = papx.seek(endCp);
      let mut endEntry: Box<dyn IPapxTableEntry> = papx.seek(endCp);
      let mut e: Box<dyn IPapxTableEntry> = startEntry;
      // java for(IPapxTable.Entry e = startEntry; e != endEntry e = e.getNext())
      while e != endEntry {
        // java int start = e.position();
        let mut start: i32 = e.position();
        // java PropertySet prop = e.getProperty();
        let mut prop: PropertySet = e.getProperty();
        // java if (null != prop.getValue(237))
        if prop.getValue(237).is_some() {
          // java changeSingleParagraphFormatRevision(start, e, textStream, change);
          Self::changeSingleParagraphFormatRevision(start, e.as_ref(), textStream, change);
        }
      }
    }

    // java private static void changeSingleParagraphFormatRevision(int startCp, IPapxTable.Entry entry, ITextStream textStream, RevisionChange change)
    pub fn changeSingleParagraphFormatRevision(startCp: i32, entry: &dyn IPapxTableEntry, textStream: &mut dyn ITextStream, change: RevisionChange) {
      // java Assert.assertNotNull("entry should not be null!", entry);
      Assert::assertNotNull__string_object("entry should not be null!", entry);
      // java Assert.assertNotNull("textStream should not be null!", textStream);
      Assert::assertNotNull__string_object("textStream should not be null!", textStream);
      // java Assert.assertNotNull("change should not be null!", change);
      Assert::assertNotNull__string_object("change should not be null!", change);
      // java switch (change)
      match change {
        // java case ACCEPT:
        ACCEPT => {
          // java acceptSingleParagraphRevision(startCp, entry, textStream);
          Self::acceptSingleParagraphRevision(startCp, entry, textStream);
        }
        // java case REJECT:
        REJECT => {
          // java rejectSingleParagraphRevision(startCp, entry, textStream);
          Self::rejectSingleParagraphRevision(startCp, entry, textStream);
        }
        // java default:
        _ => {
          // java Assert.fail("It should not reach here!");
          Assert::fail__string("It should not reach here!");
        }
      }
    }

    // java private static void acceptSingleParagraphRevision(int startCp, IPapxTable.Entry entry, ITextStream textStream)
    pub fn acceptSingleParagraphRevision(startCp: i32, entry: &dyn IPapxTableEntry, textStream: &mut dyn ITextStream) {
      // java Assert.assertNotNull("entry should not be null!", entry);
      Assert::assertNotNull__string_object("entry should not be null!", entry);
      // java Assert.assertNotNull("textStream should not be null!", textStream);
      Assert::assertNotNull__string_object("textStream should not be null!", textStream);
      // java final PropertySet finalProp = PropertyUtil.getFinalParagraphFormat(entry.getProperty());
      let mut finalProp: PropertySet = PropertyUtil::getFinalParagraphFormat(entry.getProperty());
      // java textStream.modifyPapx(startCp, entry.getNext().position(), new ITextStream.IPropertyModify() { public void compose(ITable.Entry entry, PropertyAction action) { action.addOperation(PropertyOperations.create(1, finalProp)); } });
      textStream.modifyPapx(startCp, entry.getNext().position(), &EmptyPropertyModify::new());
    }

    // java private static void rejectSingleParagraphRevision(int startCp, IPapxTable.Entry entry, ITextStream textStream)
    pub fn rejectSingleParagraphRevision(startCp: i32, entry: &dyn IPapxTableEntry, textStream: &mut dyn ITextStream) {
      // java Assert.assertNotNull("entry should not be null!", entry);
      Assert::assertNotNull__string_object("entry should not be null!", entry);
      // java Assert.assertNotNull("textStream should not be null!", textStream);
      Assert::assertNotNull__string_object("textStream should not be null!", textStream);
      // java final PropertySet prop = getParaRevison(entry.getProperty());
      let mut prop: PropertySet = Self::getParaRevison(&entry.getProperty());
      // java textStream.modifyPapx(startCp, entry.getNext().position(), new ITextStream.IPropertyModify() { public void compose(ITable.Entry entry, PropertyAction action) { action.addOperation(PropertyOperations.create(3, prop)); } });
      textStream.modifyPapx(startCp, entry.getNext().position(), &EmptyPropertyModify::new());
    }

    // java private static PropertySet getParaRevison(PropertySet prop)
    pub fn getParaRevison(prop: &PropertySet) -> PropertySet {
      // java MutablePropertySet mutaProp = new MutablePropertySet();
      let mut mutaProp: MutablePropertySet = MutablePropertySet::new();
      // java Revision revision = (Revision)prop.getValue(237);
      let mut revision: Revision = (prop.getValue(237)).downcast_ref::<Revision>().unwrap();
      // java Assert.assertNotNull("revision should not be null!", revision);
      Assert::assertNotNull__string_object("revision should not be null!", revision);
      // java mutaProp.setValue(237, revision);
      mutaProp.setValue(237, revision);
      // java mutaProp.setBoolean(238, true);
      mutaProp.setBoolean(238, true);
      // java mutaProp.setInt(239, (Integer)prop.getValue(239));
      mutaProp.setInt(239, ((prop.getValue(239)) as i32));
      // java return mutaProp.freeze();
      return mutaProp.freeze();
    }

    // java private void changeRunFormatRevision(int startCp, int endCp, ITextStream textStream, RevisionChange change)
    pub fn changeRunFormatRevision(&mut self, startCp: i32, endCp: i32, textStream: &mut dyn ITextStream, change: RevisionChange) {
      // java IChpxTable chpxTable = this.mDocument.getChpxTable();
      let mut chpxTable: Box<dyn IChpxTable> = self.doc().getChpxTable();
      // java IChpxTable.Entry startEntry = chpxTable.seek(startCp);
      let mut startEntry: Box<dyn IPapxTableEntry> = chpxTable.seek(startCp);
      // java IChpxTable.Entry endEntry = chpxTable.seek(endCp);
      let mut endEntry: Box<dyn IPapxTableEntry> = chpxTable.seek(endCp);
      // java IChpxTable.Entry e = startEntry;
      let mut e: Box<dyn IPapxTableEntry> = startEntry;
      let mut start: i32 = startCp;
      // java for(int start = startCp; e != endEntry start = e.position())
      while e != endEntry {
        // java IChpxTable.Entry thisEntry = e;
        let mut thisEntry: Box<dyn IChpxTableEntry> = e;
        // java e = e.getNext();
        e = e.getNext();
        // java PropertySet prop = thisEntry.getProperty();
        let mut prop: PropertySet = thisEntry.getProperty();
        // java Revision revision = (Revision)prop.getValue(48);
        let mut revision: Revision = (prop.getValue(48)).downcast_ref::<Revision>().unwrap();
        // java if (null != revision)
        if true {
          // java int rangeStartCp = Math.max(start, startCp);
          let mut rangeStartCp: i32 = Math::max(start, startCp);
          // java int rangeEndCp = Math.min(thisEntry.getNext().position(), endCp);
          let mut rangeEndCp: i32 = Math::min(thisEntry.getNext().position(), endCp);
          // java changeSingleRunFormatRevision(rangeStartCp, rangeEndCp, thisEntry, textStream, change);
          Self::changeSingleRunFormatRevision(rangeStartCp, rangeEndCp, thisEntry.as_ref(), textStream, change);
        }
        // java if (e.isEnd())
        if e.isEnd() {
          // java break;
          break;
        }
      }
    }

    // java private static void changeSingleRunFormatRevision(int startCp, int endCp, IChpxTable.Entry entry, ITextStream textStream, RevisionChange change)
    pub fn changeSingleRunFormatRevision(startCp: i32, endCp: i32, entry: &dyn IChpxTableEntry, textStream: &mut dyn ITextStream, change: RevisionChange) {
      // java switch (change)
      match change {
        // java case ACCEPT:
        ACCEPT => {
          // java acceptSingleRunFormatRevision(startCp, endCp, entry, textStream);
          Self::acceptSingleRunFormatRevision(startCp, endCp, entry, textStream);
        }
        // java case REJECT:
        REJECT => {
          // java rejectSingleRunFormatRevision(startCp, endCp, entry, textStream);
          Self::rejectSingleRunFormatRevision(startCp, endCp, entry, textStream);
        }
        // java default:
        _ => {
          // java Assert.fail("It should not reach here!");
          Assert::fail__string("It should not reach here!");
        }
      }
    }

    // java private static void acceptSingleRunFormatRevision(int startCp, int endCp, IChpxTable.Entry entry, ITextStream textStream)
    pub fn acceptSingleRunFormatRevision(startCp: i32, endCp: i32, entry: &dyn IChpxTableEntry, textStream: &mut dyn ITextStream) {
      // java final PropertySet finalProp = PropertyUtil.getFinalNormalRunFormat(entry.getProperty());
      let mut finalProp: PropertySet = PropertyUtil::getFinalNormalRunFormat(entry.getProperty());
      // java textStream.modifyChpx(startCp, entry.getNext().position(), new ITextStream.IPropertyModify() { public void compose(ITable.Entry entry, PropertyAction action) { action.addOperation(PropertyOperations.create(1, finalProp)); } });
      textStream.modifyChpx(startCp, entry.getNext().position(), &EmptyPropertyModify::new());
    }

    // java private void removeRunRange(int start, int end, ITextStream textStream)
    pub fn removeRunRange(&mut self, start: i32, end: i32, textStream: &mut dyn ITextStream) {
      // java Assert.assertTrue(start < end);
      Assert::assertTrue__boolean(start < end);
      // java RANGE range = RANGE.obtain(start, end);
      let mut range: RANGE = RANGE::obtain__int_int(start, end);
      // java KTableRangeBase.TableRangeContext context = KTableRangeBase.normalize(this.mDocument, range);
      let mut context: KTableRangeBase::TableRangeContext = KTableRangeBase::normalize(self.doc(), range);
      // java range.recycle();
      range.recycle();
      // java int rowEnd;
      let mut rowEnd: i32;
      // java if (context.selType == SelectionType.TABLEROW)
      if context.selType == SelectionType::TABLEROW {
        // java rowEnd = end;
        rowEnd = end;
      }
      // java else if (context.selType == SelectionType.TABLECOLUMN)
      else if context.selType == SelectionType::TABLECOLUMN {
        // java TableInfo tableInfo = this.mDocument.getTableInfos().getTableInfoByLevel(end - 1, context.rootLevel);
        let mut tableInfo: TableInfo = self.doc().getTableInfos().getTableInfoByLevel(end - 1, context.rootLevel);
        // java RowInfo rowInfo = tableInfo.getRowByCP(end - 1);
        let mut rowInfo: RowInfo = tableInfo.getRowByCP(end - 1);
        // java if (start < rowInfo.getStartCP())
        if start < rowInfo.getStartCP() {
          // java rowEnd = rowInfo.getStartCP();
          rowEnd = rowInfo.getStartCP();
        }
        // java else
        else {
          // java rowEnd = end;
          rowEnd = end;
        }
      }
      // java else if (context.endInTable)
      else if context.endInTable {
        // java TableInfo tableInfo = this.mDocument.getTableInfos().getTableInfoByLevel(end - 1, context.rootLevel + 1);
        let mut tableInfo: TableInfo = self.doc().getTableInfos().getTableInfoByLevel(end - 1, context.rootLevel + 1);
        // java RowInfo rowInfo = tableInfo.getRowByCP(end - 1);
        let mut rowInfo: RowInfo = tableInfo.getRowByCP(end - 1);
        // java if (end == rowInfo.getEndCP())
        if end == rowInfo.getEndCP() {
          // java rowEnd = end;
          rowEnd = end;
        }
        // java else
        else {
          // java rowEnd = rowInfo.getStartCP();
          rowEnd = rowInfo.getStartCP();
        }
      }
      // java else
      else {
        // java rowEnd = context.hasTable ? end : start;
        rowEnd = (if context.hasTable { end } else { start });
      }
      // java int nRemoved = 0;
      let mut nRemoved: i32 = 0;
      // java if (rowEnd < end)
      if rowEnd < end {
        // java if (rowEnd == start)
        if rowEnd == start {
          // java IPapxTable.Entry startPara = this.mDocument.getPapxTable().seek(start);
          let mut startPara: Box<dyn Entry> = self.doc().getPapxTable().seek(start);
          // java IPapxTable.Entry endPara = this.mDocument.getPapxTable().seek(end);
          let mut endPara: Box<dyn Entry> = self.doc().getPapxTable().seek(end);
          // java boolean resetToEndProp = startPara != endPara && endPara.position() < end;
          let mut resetToEndProp: bool = startPara != endPara && endPara.position() < end;
          // java nRemoved = textStream.removeText(rowEnd, end);
          nRemoved = textStream.removeText(rowEnd, end);
          // java if (resetToEndProp)
          if resetToEndProp {
            // java startPara.setProperty(endPara.getProperty());
            startPara.setProperty(endPara.getProperty());
          }
        }
        // java else
        else {
          // java nRemoved = textStream.removeText(rowEnd, end);
          nRemoved = textStream.removeText(rowEnd, end);
        }
      }
      // java if (start < rowEnd)
      if start < rowEnd {
        // java KTableRangeBase tableRange = KTableRangeBase.createTableRange(this.mDocument, start, rowEnd);
        let mut tableRange: KTableRangeBase = KTableRangeBase::createTableRange(self.doc(), start, rowEnd);
        // java if (tableRange != null)
        if true {
          // java nRemoved = tableRange.delete(KRange.DeleteRangeReason.backspace_key);
          nRemoved = tableRange.delete(KRange::DeleteRangeReason.backspace_key);
        }
        // java else
        else {
          // java nRemoved = textStream.removeText(start, rowEnd);
          nRemoved = textStream.removeText(start, rowEnd);
        }
      }
      // java this.changeKRange(start, end, nRemoved);
      self.changeKRange(start, end, nRemoved);
    }

    // java private void changeKRange(int start, int end, int len)
    pub fn changeKRange(&mut self, start: i32, end: i32, len: i32) {
      // java Assert.assertTrue(start < end);
      Assert::assertTrue__boolean(start < end);
      // java if (null != this.mKRange && len != 0)
      if let Some(mKRange) = self.mKRange.as_mut() {
        if len != 0 {
          // java int selectStart = this.mKRange.getStart();
          let mut selectStart: i32 = mKRange.getStart();
          // java int selectEnd = this.mKRange.getEnd();
          let mut selectEnd: i32 = mKRange.getEnd();
          // java if (this.mKRange.getStart() > start)
          if mKRange.getStart() > start {
            // java if (this.mKRange.getStart() >= end)
            if mKRange.getStart() >= end {
              // java selectStart -= len;
              selectStart = selectStart - len;
            }
            // java else
            else {
              // java selectStart = start;
              selectStart = start;
            }
          }
          // java if (this.mKRange.getEnd() > start)
          if mKRange.getEnd() > start {
            // java if (this.mKRange.getEnd() >= end)
            if mKRange.getEnd() >= end {
              // java selectEnd -= len;
              selectEnd = selectEnd - len;
            }
            // java else
            else {
              // java selectEnd = start;
              selectEnd = start;
            }
          }
          // java if (selectEnd == selectStart)
          if selectEnd == selectStart {
            // java int var6 = Math.min(this.mDocument.getLength() - 1, selectStart);
            let mut var6: i32 = Math::min(self.doc().getLength() - 1, selectStart);
            // java selectStart = Math.max(0, var6);
            selectStart = Math::max(0, var6);
            // java selectEnd = selectStart;
            selectEnd = selectStart;
          }
          // java this.mKRange.setRange(selectStart, selectEnd);
          mKRange.setRange(selectStart, selectEnd);
        }
      }
    }

    // java private void changeInsertRevision(int startCp, int endCp, ITextStream textStream, RevisionChange change)
    pub fn changeInsertRevision(&mut self, startCp: i32, endCp: i32, textStream: &mut dyn ITextStream, change: RevisionChange) {
      // java switch (change)
      match change {
        // java case ACCEPT:
        ACCEPT => {
          // java this.acceptInsertRevision(startCp, endCp, textStream);
          self.acceptInsertRevision(startCp, endCp, textStream);
        }
        // java case REJECT:
        REJECT => {
          // java this.rejectInsertRevision(startCp, endCp, textStream);
          self.rejectInsertRevision(startCp, endCp, textStream);
        }
        // java default:
        _ => {
          // java Assert.fail("It should not reach here!");
          Assert::fail__string("It should not reach here!");
        }
      }
    }

    // java private void rejectInsertRevision(int startCp, int endCp, ITextStream textStream)
    pub fn rejectInsertRevision(&mut self, startCp: i32, endCp: i32, textStream: &mut dyn ITextStream) {
      // java int propertyId = 46;
      let mut propertyId: i32 = 46;
      // java this.acceptDeleteRejectInsertRevision(startCp, endCp, textStream, 46);
      self.acceptDeleteRejectInsertRevision(startCp, endCp, textStream, 46);
    }

    // java private TreeMap<Integer, RANGE> getRemoveRangeMap(int startCp, IChpxTable.Entry startEntry, int endCp, IChpxTable.Entry endEntry, TreeMap<Integer, IChpxTable.Entry> changeEntryMap, int propertyId)
    pub fn getRemoveRangeMap(&self, startCp: i32, startEntry: &dyn IChpxTableEntry, endCp: i32, endEntry: &dyn IChpxTableEntry, changeEntryMap: &mut BTreeMap<i32, Box<dyn IChpxTableEntry>>, propertyId: i32) -> BTreeMap<i32, RANGE> {
      // java TreeMap<Integer, RANGE> rangeMap = new TreeMap(Collections.reverseOrder());
      let mut rangeMap: BTreeMap<i32, RANGE> = BTreeMap::new();
      // java PLCMathStart plcMathStart = this.mDocument.getOMathStarts();
      let mut plcMathStart: PLCMathStart = self.doc().getOMathStarts();
      // java if (this.isAdd(startEntry, changeEntryMap, propertyId, plcMathStart) && startCp == startEntry.position())
      if self.isAdd(startEntry, changeEntryMap, propertyId, Some(&plcMathStart)) && startCp == startEntry.position() {
        // java changeEntryMap.put(startEntry.position(), startEntry);
        changeEntryMap.put(startEntry.position(), startEntry);
      }
      // java if (this.isAdd(endEntry, changeEntryMap, propertyId, plcMathStart) && endCp == endEntry.endCp())
      if self.isAdd(endEntry, changeEntryMap, propertyId, Some(&plcMathStart)) && endCp == endEntry.endCp() {
        // java changeEntryMap.put(endEntry.position(), endEntry);
        changeEntryMap.put(endEntry.position(), endEntry);
      }
      // java this.filterAndConsumeSpecialEntry(changeEntryMap, propertyId);
      self.filterAndConsumeSpecialEntry(changeEntryMap, propertyId);
      // java convert2MergeRangeMap(changeEntryMap, rangeMap);
      Self::convert2MergeRangeMap(changeEntryMap, &mut rangeMap);
      // java if (this.isAdd(startEntry, changeEntryMap, propertyId, plcMathStart))
      if self.isAdd(startEntry, changeEntryMap, propertyId, Some(&plcMathStart)) {
        // java RANGE startRange = RANGE.obtain(startEntry.position(), startEntry.endCp());
        let mut startRange: RANGE = RANGE::obtain__int_int(startEntry.position(), startEntry.endCp());
        // java RANGE range = (RANGE)rangeMap.get(startRange.end);
        let mut range: RANGE = (rangeMap.get(startRange.end)).downcast_ref::<RANGE>().unwrap();
        // java if (null != range)
        if true {
          // java range.start = startCp;
          range.start = startCp;
        }
        // java else
        else {
          // java startRange.start = startCp;
          startRange.start = startCp;
          // java rangeMap.put(startCp, startRange);
          rangeMap.put(startCp, startRange);
        }
      }
      // java int endRemoveOffset = endCp - endEntry.position();
      let mut endRemoveOffset: i32 = endCp - endEntry.position();
      // java if (this.isAdd(endEntry, changeEntryMap, propertyId, plcMathStart) && endRemoveOffset > 0)
      if self.isAdd(endEntry, changeEntryMap, propertyId, Some(&plcMathStart)) && endRemoveOffset > 0 {
        // java int endPos = endEntry.position();
        let mut endPos: i32 = endEntry.position();
        // java rangeMap.put(endPos, RANGE.obtain(endPos, endPos + endRemoveOffset));
        rangeMap.put(endPos, RANGE::obtain__int_int(endPos, endPos + endRemoveOffset));
      }
      // java return rangeMap;
      return rangeMap;
    }

    // java private boolean isAdd(IChpxTable.Entry runEntry, TreeMap<Integer, IChpxTable.Entry> changeEntryMap, int propertyId, PLCMathStart plcMathStart)
    pub fn isAdd(&self, runEntry: &dyn IChpxTableEntry, changeEntryMap: &BTreeMap<i32, Box<dyn IChpxTableEntry>>, propertyId: i32, plcMathStart: Option<&PLCMathStart>) -> bool {
      // java return !runEntry.isEnd() && !isMathRevision(plcMathStart, runEntry) && !changeEntryMap.containsKey(runEntry.position()) && runEntry.getProperty().contains(propertyId);
      return !runEntry.isEnd() && !Self::isMathRevision(plcMathStart, runEntry) && !changeEntryMap.containsKey(runEntry.position()) && runEntry.getProperty().contains(propertyId);
    }

    // java private void removeRangeMapFromBack2Front(TreeMap<Integer, RANGE> rangeMap, ITextStream textStream)
    pub fn removeRangeMapFromBack2Front(&mut self, rangeMap: BTreeMap<i32, RANGE>, textStream: &mut dyn ITextStream) {
      // java for(Map.Entry<Integer, RANGE> m : rangeMap.entrySet())
      for m in rangeMap.entrySet().iter() {
        // java RANGE range = (RANGE)m.getValue();
        let mut range: RANGE = (m.getValue()).downcast_ref::<RANGE>().unwrap();
        // java this.removeRunRange(range.start, range.end, textStream);
        self.removeRunRange(range.start, range.end, textStream);
        // java range.recycle();
        range.recycle();
      }
    }

    // java private static void convert2MergeRangeMap(TreeMap<Integer, IChpxTable.Entry> changeEntryMap, TreeMap<Integer, RANGE> rangeMap)
    pub fn convert2MergeRangeMap(changeEntryMap: &BTreeMap<i32, Box<dyn IChpxTableEntry>>, rangeMap: &mut BTreeMap<i32, RANGE>) {
      // java Assert.assertNotNull("changeEntryMap should not be null!", changeEntryMap);
      Assert::assertNotNull__string_object("changeEntryMap should not be null!", changeEntryMap);
      // java Assert.assertNotNull("rangeMap should not be null!", rangeMap);
      Assert::assertNotNull__string_object("rangeMap should not be null!", rangeMap);
      // java if (changeEntryMap.size() > 0)
      if (changeEntryMap.len() as i32) > 0 {
        // java RANGE range = null;
        let mut range: RANGE = Default::default();
        // java for(Map.Entry<Integer, IChpxTable.Entry> m : changeEntryMap.entrySet())
        for m in changeEntryMap.entrySet().iter() {
          // java IChpxTable.Entry thisEntry = (IChpxTable.Entry)m.getValue();
          let thisEntry = m.getValue();
          // java RANGE entryRange = RANGE.obtain(thisEntry.position(), thisEntry.endCp());
          let mut entryRange: RANGE = RANGE::obtain__int_int(thisEntry.position(), thisEntry.endCp());
          // java if (null == range)
          if false {
            // java range = entryRange;
            range = entryRange;
            // java rangeMap.put(thisEntry.position(), entryRange);
            rangeMap.put(thisEntry.position(), entryRange);
          }
          // java else if (entryRange.start <= range.end && entryRange.end >= range.end)
          else if entryRange.start <= range.end && entryRange.end >= range.end {
            // java range.end = entryRange.end;
            range.end = entryRange.end;
          }
          // java else
          else {
            // java range = entryRange;
            range = entryRange;
            // java rangeMap.put(thisEntry.position(), entryRange);
            rangeMap.put(thisEntry.position(), entryRange);
          }
        }
      }
    }

    // java private boolean checkAndGetSpecialField(IChpxTable.Entry startEntry, IChpxTable cpxTable, PLCField fields, Map<Integer, IChpxTable.Entry> changeEntryMap)
    pub fn checkAndGetSpecialField(&mut self, startEntry: &dyn IChpxTableEntry, cpxTable: &dyn IChpxTable, fields: &PLCField, changeEntryMap: &mut HashMap<i32, Box<dyn IChpxTableEntry>>) -> bool {
      // java if (this.checkSpecialRange(startEntry, fields))
      if self.checkSpecialRange(startEntry, fields) {
        // java this.getFieldRun(startEntry, changeEntryMap, cpxTable, fields);
        self.getFieldRun(startEntry, changeEntryMap, cpxTable, fields);
        // java return true;
        return true;
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java private void rejectInsert_acceptDelete4SingleRevision(int startCp, int endCp, IChpxTable.Entry thisEntry, ITextStream textStream, IChpxTable cpxTable, PLCField fields, int propertyId)
    pub fn rejectInsert_acceptDelete4SingleRevision(&mut self, startCp: i32, endCp: i32, thisEntry: &dyn IChpxTableEntry, textStream: &mut dyn ITextStream, cpxTable: &dyn IChpxTable, fields: &PLCField, propertyId: i32) {
      // java TreeMap<Integer, IChpxTable.Entry> changeEntryMap = new TreeMap();
      let mut changeEntryMap: BTreeMap<i32, Box<dyn IChpxTableEntry>> = BTreeMap::new();
      // java if (thisEntry.getProperty().contains(propertyId))
      if thisEntry.getProperty().contains(propertyId) {
        // java if (this.checkSpecialRange(thisEntry, fields))
        if self.checkSpecialRange(thisEntry, fields) {
          // java this.getFieldRun(thisEntry, changeEntryMap, cpxTable, fields);
          self.getFieldRun(thisEntry, changeEntryMap, cpxTable, fields);
        }
        // java else
        else {
          // java this.removeRunRange(startCp, endCp, textStream);
          self.removeRunRange(startCp, endCp, textStream);
        }
        // java this.removeMapRange(changeEntryMap, textStream);
        self.removeMapRange(changeEntryMap, textStream);
      }
    }

    // java private void removeMapRange(TreeMap<Integer, IChpxTable.Entry> changeEntryMap, ITextStream textStream)
    pub fn removeMapRange(&mut self, changeEntryMap: &BTreeMap<i32, Box<dyn IChpxTableEntry>>, textStream: &mut dyn ITextStream) {
      // java if (changeEntryMap.size() > 0)
      if (changeEntryMap.len() as i32) > 0 {
        // java RANGE range = mergeRange(changeEntryMap);
        let mut range: RANGE = Self::mergeRange(changeEntryMap).unwrap();
        // java if (null != range)
        if true {
          // java this.removeRunRange(range.start, range.end, textStream);
          self.removeRunRange(range.start, range.end, textStream);
        }
        // java else
        else {
          // java for(Map.Entry<Integer, IChpxTable.Entry> m : changeEntryMap.entrySet())
          for m in changeEntryMap.entrySet().iter() {
            // java IChpxTable.Entry thisEntry = (IChpxTable.Entry)m.getValue();
            let thisEntry = m.getValue();
            // java this.removeRunRange(thisEntry.position(), thisEntry.getNext().position(), textStream);
            self.removeRunRange(thisEntry.position(), thisEntry.getNext().position(), textStream);
          }
        }
      }
    }

    // java private static RANGE mergeRange(TreeMap<Integer, IChpxTable.Entry> changeEntryMap)
    pub fn mergeRange(changeEntryMap: &BTreeMap<i32, Box<dyn IChpxTableEntry>>) -> Option<RANGE> {
      // java RANGE range = null;
      let mut range: RANGE = Default::default();
      // java for(Map.Entry<Integer, IChpxTable.Entry> m : changeEntryMap.entrySet())
      for m in changeEntryMap.entrySet().iter() {
        // java IChpxTable.Entry thisEntry = (IChpxTable.Entry)m.getValue();
        let thisEntry = m.getValue();
        // java RANGE entryRange = RANGE.obtain(thisEntry.position(), thisEntry.endCp());
        let mut entryRange: RANGE = RANGE::obtain__int_int(thisEntry.position(), thisEntry.endCp());
        // java if (null != range)
        if true {
          // java if (entryRange.start > range.end || entryRange.end < range.end)
          if entryRange.start > range.end || entryRange.end < range.end {
            // java return null;
            return None;
          }
          // java range.end = entryRange.end;
          range.end = entryRange.end;
        }
        // java else
        else {
          // java range = RANGE.obtain(entryRange);
          range = RANGE::obtain__range(entryRange);
        }
      }
      // java return range;
      return Some(range);
    }

    // java private boolean checkSpecialRange(IChpxTable.Entry runEntry, PLCField fields)
    pub fn checkSpecialRange(&self, runEntry: &dyn IChpxTableEntry, fields: &PLCField) -> bool {
      // java PLCNode fieldNode = fields.find(runEntry.position());
      let mut fieldNode: Box<dyn Node> = fields.find(runEntry.position());
      // java return fieldNode instanceof PLCField.FieldNode;
      return fieldNode.downcast_ref::<PLCField::FieldNode>().is_some();
    }

    // java private void getFieldRun(IChpxTable.Entry startEntry, Map<Integer, IChpxTable.Entry> changeEntryMap, IChpxTable cpxTable, PLCField fields)
    pub fn getFieldRun(&mut self, startEntry: &dyn IChpxTableEntry, changeEntryMap: &mut HashMap<i32, Box<dyn IChpxTableEntry>>, cpxTable: &dyn IChpxTable, fields: &PLCField) {
      // java Assert.assertNotNull("startEntry should not be null!", startEntry);
      Assert::assertNotNull__string_object("startEntry should not be null!", startEntry);
      // java Assert.assertNotNull("changeEntryMap should not be null!", changeEntryMap);
      Assert::assertNotNull__string_object("changeEntryMap should not be null!", changeEntryMap);
      // java Assert.assertNotNull("cpxTable should not be null!", cpxTable);
      Assert::assertNotNull__string_object("cpxTable should not be null!", cpxTable);
      // java Assert.assertNotNull("fields should not be null!", fields);
      Assert::assertNotNull__string_object("fields should not be null!", fields);
      // java int cp = startEntry.position();
      let mut cp: i32 = startEntry.position();
      // java PLCField.FieldTriple fieldTriple = fields.getContainingField(cp);
      let mut fieldTriple: PLCField::FieldTriple = fields.getContainingField(cp);
      // java if (null != fieldTriple)
      if true {
        let mut e: Box<dyn Entry> = cpxTable.seek(fieldTriple.getStartCP());
        // java for(IChpxTable.Entry e = cpxTable.seek(fieldTriple.getStartCP()); e.position() <= fieldTriple.getEndCP() e = e.getNext())
        while e.position() <= fieldTriple.getEndCP() {
          // java changeEntryMap.put(e.position(), e);
          changeEntryMap.put(e.position(), e);
        }
      }
    }

    // java  void acceptInsertRevision(int startCp, int endCp, ITextStream textStream)
    pub fn acceptInsertRevision(&mut self, startCp: i32, endCp: i32, textStream: &mut dyn ITextStream) {
      // java IChpxTable cpxTable = this.mDocument.getChpxTable();
      let mut cpxTable: Box<dyn IChpxTable> = self.doc().getChpxTable();
      // java IChpxTable.Entry startEntry = cpxTable.seek(startCp);
      let mut startEntry: Box<dyn IChpxTableEntry> = cpxTable.seek(startCp);
      // java PLCMathStart plcMathStart = this.mDocument.getOMathStarts();
      let mut plcMathStart: PLCMathStart = self.doc().getOMathStarts();
      // java IChpxTable.Entry endEntry = cpxTable.seek(endCp);
      let mut endEntry: Box<dyn IChpxTableEntry> = cpxTable.seek(endCp);
      // java if (startEntry == endEntry)
      if startEntry == endEntry {
        // java if (!isMathRevision(plcMathStart, startEntry))
        if !Self::isMathRevision(Some(&plcMathStart), startEntry.as_ref()) {
          // java acceptSingleInsertRunRevision(startCp, endCp, startEntry, textStream);
          Self::acceptSingleInsertRunRevision(startCp, endCp, startEntry.as_ref(), textStream);
        }
      }
      // java else
      else {
        // java IChpxTable.Entry e = startEntry;
        let mut e: Box<dyn IPapxTableEntry> = startEntry;
        // java while (e != endEntry)
        while e != endEntry {
          // java IChpxTable.Entry thisEntry = e;
          let mut thisEntry: Box<dyn IChpxTableEntry> = e;
          // java e = e.getNext();
          e = e.getNext();
          // java if (!isMathRevision(plcMathStart, thisEntry))
          if !Self::isMathRevision(Some(&plcMathStart), thisEntry.as_ref()) {
            // java PropertySet prop = thisEntry.getProperty();
            let mut prop: PropertySet = thisEntry.getProperty();
            // java Revision revision = (Revision)prop.getValue(46);
            let mut revision: Revision = (prop.getValue(46)).downcast_ref::<Revision>().unwrap();
            // java if (null != revision)
            if true {
              // java int rangeStartCp = Math.max(thisEntry.position(), startCp);
              let mut rangeStartCp: i32 = Math::max(thisEntry.position(), startCp);
              // java int rangeEndCp = Math.min(thisEntry.getNext().position(), endCp);
              let mut rangeEndCp: i32 = Math::min(thisEntry.getNext().position(), endCp);
              // java if (rangeStartCp != rangeEndCp)
              if rangeStartCp != rangeEndCp {
                // java acceptSingleInsertRunRevision(rangeStartCp, rangeEndCp, thisEntry, textStream);
                Self::acceptSingleInsertRunRevision(rangeStartCp, rangeEndCp, thisEntry.as_ref(), textStream);
              }
            }
          }
        }
      }
    }

    // java private static void acceptSingleInsertRunRevision(int startCp, int endCp, IChpxTable.Entry entry, ITextStream textStream)
    pub fn acceptSingleInsertRunRevision(startCp: i32, endCp: i32, entry: &dyn IChpxTableEntry, textStream: &mut dyn ITextStream) {
      // java final MutablePropertySet mutaProp = new MutablePropertySet();
      let mut mutaProp: MutablePropertySet = MutablePropertySet::new();
      // java PropertySet prop = entry.getProperty();
      let mut prop: PropertySet = entry.getProperty();
      // java Revision revision = (Revision)prop.getValue(46);
      let mut revision: Revision = (prop.getValue(46)).downcast_ref::<Revision>().unwrap();
      // java Assert.assertNotNull("revision should not be null!", revision);
      Assert::assertNotNull__string_object("revision should not be null!", revision);
      // java mutaProp.setValue(46, revision);
      mutaProp.setValue(46, revision);
      // java mutaProp.setBoolean(55, true);
      mutaProp.setBoolean(55, true);
      // java textStream.modifyChpx(startCp, endCp, new ITextStream.IPropertyModify() { public void compose(ITable.Entry entry, PropertyAction action) { action.addOperation(PropertyOperations.create(3, mutaProp.freeze())); } });
      textStream.modifyChpx(startCp, endCp, &EmptyPropertyModify::new());
    }

    // java private void changeDeleteRevision(int startCp, int endCp, ITextStream textStream, RevisionChange change)
    pub fn changeDeleteRevision(&mut self, startCp: i32, endCp: i32, textStream: &mut dyn ITextStream, change: RevisionChange) {
      // java switch (change)
      match change {
        // java case ACCEPT:
        ACCEPT => {
          // java this.acceptDeleteRevision(startCp, endCp, textStream);
          self.acceptDeleteRevision(startCp, endCp, textStream);
        }
        // java case REJECT:
        REJECT => {
          // java this.rejectDeleteRevision(startCp, endCp, textStream);
          self.rejectDeleteRevision(startCp, endCp, textStream);
        }
        // java default:
        _ => {
          // java Assert.fail("It should not reach here!");
          Assert::fail__string("It should not reach here!");
        }
      }
    }

    // java private void acceptDeleteRevision(int startCp, int endCp, ITextStream textStream)
    pub fn acceptDeleteRevision(&mut self, startCp: i32, endCp: i32, textStream: &mut dyn ITextStream) {
      // java int propertyId = 45;
      let mut propertyId: i32 = 45;
      // java this.acceptDeleteRejectInsertRevision(startCp, endCp, textStream, 45);
      self.acceptDeleteRejectInsertRevision(startCp, endCp, textStream, 45);
    }

    // java private void acceptDeleteRejectInsertRevision(int startCp, int endCp, ITextStream textStream, int propertyId)
    pub fn acceptDeleteRejectInsertRevision(&mut self, startCp: i32, endCp: i32, textStream: &mut dyn ITextStream, propertyId: i32) {
      // java IChpxTable cpxTable = this.mDocument.getChpxTable();
      let mut cpxTable: Box<dyn IChpxTable> = self.doc().getChpxTable();
      // java IPapxTable papxTable = this.mDocument.getPapxTable();
      let mut papxTable: Box<dyn IPapxTable> = self.doc().getPapxTable();
      // java PLCField fields = this.mDocument.getPLCField();
      let mut fields: PLCField = self.doc().getPLCField();
      // java PLCMathStart plcMathStart = this.mDocument.getOMathStarts();
      let mut plcMathStart: PLCMathStart = self.doc().getOMathStarts();
      // java IChpxTable.Entry startEntry = cpxTable.seek(startCp);
      let mut startEntry: Box<dyn IChpxTableEntry> = cpxTable.seek(startCp);
      // java IChpxTable.Entry endEntry = cpxTable.seek(endCp);
      let mut endEntry: Box<dyn IChpxTableEntry> = cpxTable.seek(endCp);
      // java TreeMap<Integer, IChpxTable.Entry> changeEntryMap = new TreeMap();
      let mut changeEntryMap: BTreeMap<i32, Box<dyn IChpxTableEntry>> = BTreeMap::new();
      // java if (startEntry == endEntry)
      if startEntry == endEntry {
        // java if (!isMathRevision(plcMathStart, startEntry))
        if !Self::isMathRevision(Some(&plcMathStart), startEntry.as_ref()) {
          // java this.rejectInsert_acceptDelete4SingleRevision(startCp, endCp, startEntry, textStream, cpxTable, fields, propertyId);
          self.rejectInsert_acceptDelete4SingleRevision(startCp, endCp, startEntry.as_ref(), textStream, cpxTable.as_ref(), &fields, propertyId);
        }
      }
      // java else
      else {
        // java IChpxTable.Entry e = startEntry.getNext();
        let mut e: Box<dyn IChpxTableEntry> = startEntry.getNext();
        // java while (e != endEntry)
        while e != endEntry {
          // java if (isMathRevision(plcMathStart, e))
          if Self::isMathRevision(Some(&plcMathStart), e.as_ref()) {
            // java e = e.getNext();
            e = e.getNext();
          }
          // java else
          else {
            // java PropertySet prop = e.getProperty();
            let mut prop: PropertySet = e.getProperty();
            // java Revision revision = (Revision)prop.getValue(propertyId);
            let mut revision: Revision = (prop.getValue(propertyId)).downcast_ref::<Revision>().unwrap();
            // java if (null != revision)
            if true {
              // java this.getRunEntrys(cpxTable, papxTable, fields, changeEntryMap, e);
              self.getRunEntrys(cpxTable.as_ref(), papxTable.as_ref(), &fields, changeEntryMap, e.as_ref());
            }
            // java e = e.getNext();
            e = e.getNext();
          }
        }
        // java if (!isMathRevision(plcMathStart, startEntry) && startEntry.getProperty().contains(propertyId))
        if !Self::isMathRevision(Some(&plcMathStart), startEntry.as_ref()) && startEntry.getProperty().contains(propertyId) {
          // java this.checkAndGetSpecialField(startEntry, cpxTable, fields, changeEntryMap);
          self.checkAndGetSpecialField(startEntry.as_ref(), cpxTable.as_ref(), &fields, changeEntryMap);
        }
        // java TreeMap<Integer, RANGE> rangeMap = this.getRemoveRangeMap(startCp, startEntry, endCp, endEntry, changeEntryMap, propertyId);
        let rangeMap = self.getRemoveRangeMap(startCp, startEntry.as_ref(), endCp, endEntry.as_ref(), &mut changeEntryMap, propertyId);
        // java this.removeRangeMapFromBack2Front(rangeMap, textStream);
        self.removeRangeMapFromBack2Front(rangeMap, textStream);
      }
    }

    // java private void filterAndConsumeSpecialEntry(TreeMap<Integer, IChpxTable.Entry> changeEntryMap, int propertyId)
    pub fn filterAndConsumeSpecialEntry(&mut self, changeEntryMap: &mut BTreeMap<i32, Box<dyn IChpxTableEntry>>, propertyId: i32) {
      // java if (propertyId == 45 && !changeEntryMap.isEmpty())
      if propertyId == 45 && !changeEntryMap.is_empty() {
        // java IChpxTable.Entry lastRun = this.mDocument.getChpxTable().last();
        let mut lastRun: Box<dyn IChpxTableEntry> = self.doc().getChpxTable().last();
        // java if (RangeTool.isParagraphBreak(this.mDocument, lastRun) && changeEntryMap.containsValue(lastRun))
        if RangeTool::isParagraphBreak(self.doc(), lastRun.as_ref()) && changeEntryMap.containsValue(lastRun) {
          // java rejectSingleDeleteRunRevision(lastRun.position(), lastRun.endCp(), lastRun, this.mDocument.getTextStream(), this.mDocument);
          Self::rejectSingleDeleteRunRevision(lastRun.position(), lastRun.endCp(), lastRun.as_ref(), &mut self.doc().getTextStream(), self.doc());
          // java changeEntryMap.remove(lastRun.position(), lastRun);
          changeEntryMap.remove(lastRun.position(), lastRun);
        }
      }
    }

    // java private static boolean isMathRevision(PLCMathStart plcMathStart, IChpxTable.Entry e)
    pub fn isMathRevision(plcMathStart: Option<&PLCMathStart>, e: &dyn IChpxTableEntry) -> bool {
      // java if (null != plcMathStart && !plcMathStart.isEmpty())
      if let Some(plcMathStart) = plcMathStart {
        if !plcMathStart.isEmpty() {
          // java int cp = e.position();
          let mut cp: i32 = e.position();
          // java PLCMathStart.OMathStartNode startNode = plcMathStart.findClose(cp);
          let mut startNode: PLCMathStart::OMathStartNode = plcMathStart.findClose(cp);
          // java if (null == startNode)
          if false {
            // java return false;
            return false;
          }
          // java else
          else {
            // java Assert.assertTrue("startNode instanceof OMathStartNode should be true", startNode instanceof PLCMathStart.OMathStartNode);
            Assert::assertTrue__string_boolean("startNode instanceof OMathStartNode should be true", startNode.downcast_ref::<PLCMathStart::OMathStartNode>().is_some());
            // java PLCMathEnd.OMathEndNode endNode = startNode.getEnd();
            let mut endNode: PLCMathEnd::OMathEndNode = startNode.getEnd();
            // java return startNode.getCP() <= cp && endNode.getCP() > cp;
            return startNode.getCP() <= cp && endNode.getCP() > cp;
          }
        }
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java private void getRunEntrys(IChpxTable cpxTable, IPapxTable papxTable, PLCField fields, TreeMap<Integer, IChpxTable.Entry> changeEntryMap, IChpxTable.Entry e)
    pub fn getRunEntrys(&mut self, cpxTable: &dyn IChpxTable, papxTable: &dyn IPapxTable, fields: &PLCField, changeEntryMap: &mut BTreeMap<i32, Box<dyn IChpxTableEntry>>, e: &dyn IChpxTableEntry) {
      // java Assert.assertNotNull("cpxTable should not be null!", cpxTable);
      Assert::assertNotNull__string_object("cpxTable should not be null!", cpxTable);
      // java Assert.assertNotNull("papxTable should not be null!", papxTable);
      Assert::assertNotNull__string_object("papxTable should not be null!", papxTable);
      // java Assert.assertNotNull("fields should not be null!", fields);
      Assert::assertNotNull__string_object("fields should not be null!", fields);
      // java Assert.assertNotNull("changeEntryMap should not be null!", changeEntryMap);
      Assert::assertNotNull__string_object("changeEntryMap should not be null!", changeEntryMap);
      // java Assert.assertNotNull("e should not be null!", e);
      Assert::assertNotNull__string_object("e should not be null!", e);
      // java IPapxTable.Entry papxEntry = papxTable.seek(e.position());
      let mut papxEntry: Box<dyn IPapxTableEntry> = papxTable.seek(e.position());
      // java int level = papxEntry.getProperty().getInt(224, 1);
      let mut level: i32 = papxEntry.getProperty().getInt(224, 1);
      // java if (KTableRevisionChange.isRowEnd(this.mDocument, papxEntry, level))
      if KTableRevisionChange::isRowEnd(self.doc(), papxEntry, level) {
        // java this.getSpecialRunEntryInTable(cpxTable, changeEntryMap, papxEntry, level);
        self.getSpecialRunEntryInTable(cpxTable, changeEntryMap, papxEntry.as_ref(), level);
      }
      // java if (this.checkSpecialRange(e, fields))
      if self.checkSpecialRange(e, fields) {
        // java this.getFieldRun(e, changeEntryMap, cpxTable, fields);
        self.getFieldRun(e, changeEntryMap, cpxTable, fields);
      }
      // java else
      else {
        // java changeEntryMap.put(e.position(), e);
        changeEntryMap.put(e.position(), e);
      }
    }

    // java private void getSpecialRunEntryInTable(IChpxTable cpxTable, TreeMap<Integer, IChpxTable.Entry> changeEntryMap, IPapxTable.Entry papxEntry, int level)
    pub fn getSpecialRunEntryInTable(&mut self, cpxTable: &dyn IChpxTable, changeEntryMap: &mut BTreeMap<i32, Box<dyn IChpxTableEntry>>, papxEntry: &dyn IPapxTableEntry, level: i32) {
      // java RowInfo rowInfo = this.mDocument.getTableInfos().getRowInfoByLevel(papxEntry.position(), level);
      let mut rowInfo: RowInfo = self.doc().getTableInfos().getRowInfoByLevel(papxEntry.position(), level);
      // java int rowStart = rowInfo.getStartCP();
      let mut rowStart: i32 = rowInfo.getStartCP();
      // java int rowEnd = rowInfo.getEndCP();
      let mut rowEnd: i32 = rowInfo.getEndCP();
      // java IChpxTable.Entry startRowEntry = cpxTable.seek(rowStart);
      let mut startRowEntry: Box<dyn IChpxTableEntry> = cpxTable.seek(rowStart);
      // java IChpxTable.Entry endRowEntry = cpxTable.seek(rowEnd);
      let mut endRowEntry: Box<dyn IChpxTableEntry> = cpxTable.seek(rowEnd);
      let mut e1: Box<dyn Entry> = startRowEntry;
      // java for(IChpxTable.Entry e1 = startRowEntry; e1 != endRowEntry e1 = e1.getNext())
      while e1 != endRowEntry {
        // java if (!changeEntryMap.containsKey(e1.position()))
        if !changeEntryMap.containsKey(e1.position()) {
          // java changeEntryMap.put(e1.position(), e1);
          changeEntryMap.put(e1.position(), e1);
        }
      }
    }

    // java private void rejectDeleteRevision(int startCp, int endCp, ITextStream textStream)
    pub fn rejectDeleteRevision(&mut self, startCp: i32, endCp: i32, textStream: &mut dyn ITextStream) {
      // java IChpxTable cpxTable = this.mDocument.getChpxTable();
      let mut cpxTable: Box<dyn IChpxTable> = self.doc().getChpxTable();
      // java IChpxTable.Entry startEntry = cpxTable.seek(startCp);
      let mut startEntry: Box<dyn IChpxTableEntry> = cpxTable.seek(startCp);
      // java PLCMathStart plcMathStart = this.mDocument.getOMathStarts();
      let mut plcMathStart: PLCMathStart = self.doc().getOMathStarts();
      // java IChpxTable.Entry endEntry = cpxTable.seek(endCp);
      let mut endEntry: Box<dyn IChpxTableEntry> = cpxTable.seek(endCp);
      // java if (startEntry == endEntry)
      if startEntry == endEntry {
        // java if (!isMathRevision(plcMathStart, startEntry))
        if !Self::isMathRevision(Some(&plcMathStart), startEntry.as_ref()) {
          // java rejectSingleDeleteRunRevision(startCp, endCp, startEntry, textStream, this.mDocument);
          Self::rejectSingleDeleteRunRevision(startCp, endCp, startEntry.as_ref(), textStream, self.doc());
        }
      }
      // java else
      else {
        // java IChpxTable.Entry e = startEntry;
        let mut e: Box<dyn IPapxTableEntry> = startEntry;
        // java while (e != endEntry)
        while e != endEntry {
          // java IChpxTable.Entry thisEntry = e;
          let mut thisEntry: Box<dyn IChpxTableEntry> = e;
          // java e = e.getNext();
          e = e.getNext();
          // java if (!isMathRevision(plcMathStart, thisEntry))
          if !Self::isMathRevision(Some(&plcMathStart), thisEntry.as_ref()) {
            // java PropertySet prop = thisEntry.getProperty();
            let mut prop: PropertySet = thisEntry.getProperty();
            // java Revision revision = (Revision)prop.getValue(45);
            let mut revision: Revision = (prop.getValue(45)).downcast_ref::<Revision>().unwrap();
            // java if (null != revision)
            if true {
              // java int rangeStartCp = Math.max(thisEntry.position(), startCp);
              let mut rangeStartCp: i32 = Math::max(thisEntry.position(), startCp);
              // java int rangeEndCp = Math.min(thisEntry.getNext().position(), endCp);
              let mut rangeEndCp: i32 = Math::min(thisEntry.getNext().position(), endCp);
              // java if (rangeStartCp != rangeEndCp)
              if rangeStartCp != rangeEndCp {
                // java rejectSingleDeleteRunRevision(rangeStartCp, rangeEndCp, thisEntry, textStream, this.mDocument);
                Self::rejectSingleDeleteRunRevision(rangeStartCp, rangeEndCp, thisEntry.as_ref(), textStream, self.doc());
              }
            }
          }
        }
      }
    }

    // java private static void rejectSingleDeleteRunRevision(int startCp, int endCp, IChpxTable.Entry entry, ITextStream textStream, Document document)
    pub fn rejectSingleDeleteRunRevision(startCp: i32, endCp: i32, entry: &dyn IChpxTableEntry, textStream: &mut dyn ITextStream, document: &dyn Document) {
      // java final MutablePropertySet mutaProp = new MutablePropertySet();
      let mut mutaProp: MutablePropertySet = MutablePropertySet::new();
      // java PropertySet prop = entry.getProperty();
      let mut prop: PropertySet = entry.getProperty();
      // java Revision revision = (Revision)prop.getValue(45);
      let mut revision: Revision = (prop.getValue(45)).downcast_ref::<Revision>().unwrap();
      // java Assert.assertNotNull("revision should not be null!", revision);
      Assert::assertNotNull__string_object("revision should not be null!", revision);
      // java mutaProp.setValue(45, revision);
      mutaProp.setValue(45, revision);
      // java mutaProp.setBoolean(54, true);
      mutaProp.setBoolean(54, true);
      // java mutaProp.setInt(49, (Integer)prop.getValue(49));
      mutaProp.setInt(49, ((prop.getValue(49)) as i32));
      // java textStream.modifyChpx(startCp, endCp, new ITextStream.IPropertyModify() { public void compose(ITable.Entry entry, PropertyAction action) { action.addOperation(PropertyOperations.create(3, mutaProp.freeze())); } });
      textStream.modifyChpx(startCp, endCp, &EmptyPropertyModify::new());
    }

    // java private static boolean checkRunRevision(Document document)
    pub fn checkRunRevision(document: &dyn Document) -> bool {
      // java IChpxTable chpxTable = document.getChpxTable();
      let mut chpxTable: Box<dyn IChpxTable> = document.getChpxTable();
      let mut entry: Box<dyn Entry> = chpxTable.seek(0);
      // java for(IChpxTable.Entry entry = chpxTable.seek(0); !entry.isEnd() entry = entry.getNext())
      while !entry.isEnd() {
        // java PropertySet prop = entry.getProperty();
        let mut prop: PropertySet = entry.getProperty();
        // java if (prop.contains(45))
        if prop.contains(45) {
          // java return true;
          return true;
        }
        // java if (prop.contains(46))
        if prop.contains(46) {
          // java return true;
          return true;
        }
        // java if (prop.contains(48))
        if prop.contains(48) {
          // java return true;
          return true;
        }
      }
      // java return false;
      return false;
    }

    // java private static boolean checkParagraphRevision(Document document)
    pub fn checkParagraphRevision(document: &dyn Document) -> bool {
      let mut entry: Box<dyn Entry> = document.getPapxTable().seek(0);
      // java for(IPapxTable.Entry entry = document.getPapxTable().seek(0); !entry.isEnd() entry = entry.getNext())
      while !entry.isEnd() {
        // java PropertySet prop = entry.getProperty();
        let mut prop: PropertySet = entry.getProperty();
        // java if (prop.contains(237))
        if prop.contains(237) {
          // java return true;
          return true;
        }
        // java if (prop.contains(363) && prop.contains(364))
        if prop.contains(363) && prop.contains(364) {
          // java return true;
          return true;
        }
      }
      // java return false;
      return false;
    }

}

// java public static enum RevisionChange
impl RevisionChange {
}

// java public static enum RevisionType
impl RevisionType {
}
