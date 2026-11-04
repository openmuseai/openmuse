// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
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
// Java: import cn.wps.moffice.writer.data.PropertyAction
use crate::cn::wps::moffice::writer::data::PropertyAction::*;
// Java: import cn.wps.moffice.writer.data.PropertyOperations
use crate::cn::wps::moffice::writer::data::PropertyOperations::*;
// Java: import cn.wps.moffice.writer.data.PropertyUtil
use crate::cn::wps::moffice::writer::data::PropertyUtil::*;
// Java: import cn.wps.moffice.writer.data.value.CellMargin
use crate::cn::wps::moffice::writer::data::value::CellMargin::*;
// Java: import cn.wps.moffice.writer.data.value.Revision
use crate::cn::wps::moffice::writer::data::value::Revision::*;
// Java: import java.util.ArrayList
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KRevisionChange
use crate::cn::wps::moffice::writer::core::KRevisionChange::*;
// Java: import org.apache.poi.poifs.filesystem.Entry
use io_poi_compact::org::apache::poi::poifs::filesystem::Entry::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
// Java: import cn.wps.moffice.writer.core.KStyles
use crate::cn::wps::moffice::writer::core::KStyles::*;

// java private static final int defaultTableLevel = 1;
const defaultTableLevel: i32 = 1;

// java  class KTableRevisionChange
#[derive(Default)]
pub struct KTableRevisionChange {
    // java private Document mDocument = null;
    mDocument: *mut dyn Document,
}

// java  class KTableRevisionChange
impl KTableRevisionChange {
    // java public  KTableRevisionChange(Document document)
    pub fn KTableRevisionChange(document: &dyn Document) -> Self {
      // java Assert.assertNotNull("document should not be null !", document);
      Assert::assertNotNull__string_object(&"document should not be null !", Some(&document));
      // java Assert.assertTrue("document.getLength() > 0 should be true!", document.getLength() > 0);
      Assert::assertTrue__string_boolean(&"document.getLength() > 0 should be true!", document.getLength() > 0);
      // java Assert.assertTrue("the document type invalid!", document.getType() < 7);
      Assert::assertTrue__string_boolean(&"the document type invalid!", document.getType() < 7);
      // java Assert.assertTrue("comment document should not support revision!", document.getType() != 3);
      Assert::assertTrue__string_boolean(&"comment document should not support revision!", document.getType() != 3);
      // java this.mDocument = document;
      let mut this = Self { mDocument: document };
      this
    }

    // java public void changeAllTableFormatRevision(ITextStream textStream, KRevisionChange.RevisionChange change)
    pub fn changeAllTableFormatRevision__itextstream_revisionchange(&mut self, textStream: &dyn ITextStream, change: &KRevisionChange::RevisionChange) {
      // java Assert.assertNotNull("textStream should not be null!", textStream);
      Assert::assertNotNull__string_object(&"textStream should not be null!", Some(&textStream));
      // java Assert.assertNotNull("change should not be null!", change);
      Assert::assertNotNull__string_object(&"change should not be null!", Some(&change));
      // java ITextStream iTextStream = this.mDocument.getTextStream();
      let mut iTextStream: Box<dyn ITextStream> = self.mDocument.getTextStream();
      // java iTextStream.disableRevisionFileter();
      iTextStream.disableRevisionFileter();
      // java this.changeAllTableFormatRevision(0, this.mDocument.getLength(), iTextStream, change);
      self.changeAllTableFormatRevision__int_int_itextstream_revisionchange(0, self.mDocument.getLength(), &iTextStream, &change);
      // java iTextStream.enableRevisionFileter();
      iTextStream.enableRevisionFileter();
    }

    // java private void changeAllTableFormatRevision(int startCp, int endCp, ITextStream textStream, KRevisionChange.RevisionChange change)
    pub fn changeAllTableFormatRevision__int_int_itextstream_revisionchange(&mut self, startCp: i32, endCp: i32, textStream: &dyn ITextStream, change: &KRevisionChange::RevisionChange) {
      // java Assert.assertTrue("startCp < endCp should be true!", startCp < endCp);
      Assert::assertTrue__string_boolean(&"startCp < endCp should be true!", startCp < endCp);
      // java Assert.assertNotNull("textStream should not be null!", textStream);
      Assert::assertNotNull__string_object(&"textStream should not be null!", Some(&textStream));
      // java Assert.assertNotNull("change should not be null!", change);
      Assert::assertNotNull__string_object(&"change should not be null!", Some(&change));
      // java if (endCp > startCp)
      if endCp > startCp {
        // java IPapxTable papxTable = this.mDocument.getPapxTable();
        let mut papxTable: Box<dyn IPapxTable> = self.mDocument.getPapxTable();
        // java IPapxTable.Entry startRowEntry = getTableRowEndEntry(papxTable, startCp);
        let mut startRowEntry: Box<dyn IPapxTableEntry> = Self::getTableRowEndEntry__ipapxtable_int(&mut papxTable, startCp);
        // java IPapxTable.Entry endRowEntry = getEndCpTableRowEndEntry(papxTable, endCp);
        let mut endRowEntry: Box<dyn IPapxTableEntry> = Self::getEndCpTableRowEndEntry(&mut papxTable, endCp);
        // java if (this.checkRevisionRangeValid(startRowEntry, endRowEntry, papxTable))
        if self.checkRevisionRangeValid(&mut startRowEntry, &mut endRowEntry, &papxTable) {
          // java int endRowEndCp = endRowEntry.isEnd() ? this.mDocument.getLength() : endRowEntry.position();
          let mut endRowEndCp: i32 = (if endRowEntry.isEnd() { self.mDocument.getLength() } else { endRowEntry.position() });
          // java IPapxTable.Entry e = startRowEntry;
          let mut e: Box<dyn IPapxTableEntry> = startRowEntry;
          // java while (!e.isEnd())
          while !e.isEnd() {
            // java if (e.getProperty().contains(363))
            if e.getProperty().contains(363) {
              // java this.changeSingleTableFormatRevision(e.position(), e, textStream, change);
              self.changeSingleTableFormatRevision(e.position(), &e, &textStream, &change);
            }
            // java if (e.position() >= endRowEndCp)
            if e.position() >= endRowEndCp {
              // java break;
              break;
            }
            // java e = getTableRowEndEntry(papxTable, e.getNext().position());
            e = Self::getTableRowEndEntry__ipapxtable_int(&mut papxTable, e.getNext().position());
            // java if (e.position() > endRowEndCp)
            if e.position() > endRowEndCp {
              // java break;
              break;
            }
          }
        }
      }
    }

    // java public void changeTableFormatRevision(int startCp, int endCp, ITextStream textStream, KRevisionChange.RevisionChange change)
    pub fn changeTableFormatRevision(&mut self, startCp: i32, endCp: i32, textStream: &dyn ITextStream, change: &KRevisionChange::RevisionChange) {
      // java Assert.assertTrue("startCp < endCp should be true!", startCp < endCp);
      Assert::assertTrue__string_boolean(&"startCp < endCp should be true!", startCp < endCp);
      // java Assert.assertNotNull("textStream should not be null!", textStream);
      Assert::assertNotNull__string_object(&"textStream should not be null!", Some(&textStream));
      // java Assert.assertNotNull("change should not be null!", change);
      Assert::assertNotNull__string_object(&"change should not be null!", Some(&change));
      // java int validEndCp = endCp - 1;
      let mut validEndCp: i32 = endCp - 1;
      // java if (validEndCp > startCp)
      if validEndCp > startCp {
        // java ITextStream iTextStream = this.mDocument.getTextStream();
        let mut iTextStream: Box<dyn ITextStream> = self.mDocument.getTextStream();
        // java iTextStream.disableRevisionFileter();
        iTextStream.disableRevisionFileter();
        // java IPapxTable papxTable = this.mDocument.getPapxTable();
        let mut papxTable: Box<dyn IPapxTable> = self.mDocument.getPapxTable();
        // java IPapxTable.Entry endRowEntry = getEndCpTableRowEndEntry(papxTable, validEndCp);
        let mut endRowEntry: Box<dyn IPapxTableEntry> = Self::getEndCpTableRowEndEntry(&mut papxTable, validEndCp);
        // java int tableLevel = endRowEntry.isEnd() ? 1 : endRowEntry.getProperty().getInt(224, 1);
        let mut tableLevel: i32 = (if endRowEntry.isEnd() { 1 } else { endRowEntry.getProperty().getInt(224, 1) });
        // java IPapxTable.Entry startRowEntry = getTableRowEndEntry(this.mDocument, papxTable, startCp, tableLevel);
        let mut startRowEntry: Box<dyn IPapxTableEntry> = Self::getTableRowEndEntry__document_ipapxtable_int_int(&self.mDocument, &mut papxTable, startCp, tableLevel);
        // java if (this.checkRevisionRangeValid(startRowEntry, endRowEntry, papxTable))
        if self.checkRevisionRangeValid(&mut startRowEntry, &mut endRowEntry, &papxTable) {
          // java int endRowEndCp = endRowEntry.isEnd() ? this.mDocument.getLength() : endRowEntry.position();
          let mut endRowEndCp: i32 = (if endRowEntry.isEnd() { self.mDocument.getLength() } else { endRowEntry.position() });
          // java IPapxTable.Entry e = startRowEntry;
          let mut e: Box<dyn IPapxTableEntry> = startRowEntry;
          // java while (!e.isEnd())
          while !e.isEnd() {
            // java if (e.getProperty().contains(363))
            if e.getProperty().contains(363) {
              // java this.changeSingleTableFormatRevision(e.position(), e, textStream, change);
              self.changeSingleTableFormatRevision(e.position(), &e, &textStream, &change);
            }
            // java if (e.position() >= endRowEndCp)
            if e.position() >= endRowEndCp {
              // java break;
              break;
            }
            // java e = getTableRowEndEntry(this.mDocument, papxTable, e.getNext().position(), tableLevel);
            e = Self::getTableRowEndEntry__document_ipapxtable_int_int(&self.mDocument, &mut papxTable, e.getNext().position(), tableLevel);
            // java if (e.position() > endRowEndCp)
            if e.position() > endRowEndCp {
              // java break;
              break;
            }
          }
          // java iTextStream.enableRevisionFileter();
          iTextStream.enableRevisionFileter();
        }
      }
    }

    // java private boolean checkRevisionRangeValid(IPapxTable.Entry startRowEntry, IPapxTable.Entry endRowEntry, IPapxTable papxTable)
    pub fn checkRevisionRangeValid(&mut self, startRowEntry: &mut dyn IPapxTableEntry, endRowEntry: &mut dyn IPapxTableEntry, papxTable: &dyn IPapxTable) -> bool {
      // java Assert.assertNotNull("startRowEntry should not be null!", startRowEntry);
      Assert::assertNotNull__string_object(&"startRowEntry should not be null!", Some(&startRowEntry));
      // java Assert.assertNotNull("endRowEntry should not be null!", endRowEntry);
      Assert::assertNotNull__string_object(&"endRowEntry should not be null!", Some(&endRowEntry));
      // java Assert.assertNotNull("papxTable should not be null!", papxTable);
      Assert::assertNotNull__string_object(&"papxTable should not be null!", Some(&papxTable));
      // java if (startRowEntry.isEnd())
      if startRowEntry.isEnd() {
        // java return false;
        return false;
      }
      // java else if (endRowEntry.isEnd())
      else if endRowEntry.isEnd() {
        // java return true;
        return true;
      }
      // java else
      else {
        // java int startRowPos = startRowEntry.position();
        let mut startRowPos: i32 = startRowEntry.position();
        // java int endRowPos = endRowEntry.position();
        let mut endRowPos: i32 = endRowEntry.position();
        // java if (startRowPos > endRowPos)
        if startRowPos > endRowPos {
          // java return false;
          return false;
        }
        // java else if (startRowPos < endRowPos)
        else if startRowPos < endRowPos {
          // java return true;
          return true;
        }
        // java else
        else {
          // java if (startRowPos == endRowPos)
          if startRowPos == endRowPos {
            // java PropertySet startProp = startRowEntry.getProperty();
            let mut startProp: PropertySet = startRowEntry.getProperty();
            // java if (!startProp.getBoolean(223, false))
            if !startProp.getBoolean(223, false) {
              // java return false;
              return false;
            }
          }
          // java return true;
          return true;
        }
      }
    }

    // java private void acceptSingleTableRevision(int startCp, IPapxTable.Entry entry, ITextStream textStream)
    pub fn acceptSingleTableRevision(&self, startCp: i32, entry: &dyn IPapxTableEntry, textStream: &mut dyn ITextStream) {
      // java Assert.assertNotNull("entry should not be null!", entry);
      Assert::assertNotNull__string_object(&"entry should not be null!", Some(&entry));
      // java Assert.assertNotNull("textStream should not be null!", textStream);
      Assert::assertNotNull__string_object(&"textStream should not be null!", Some(&textStream));
      // java final PropertySet finalProp = acceptTableFormat(this.mDocument.getStyles(), entry.getProperty());
      let mut finalProp: PropertySet = Self::acceptTableFormat(&self.mDocument.getStyles(), &entry.getProperty());
      // java textStream.modifyPapx(startCp, entry.getNext().position(), new ITextStream.IPropertyModify() { public void compose(ITable.Entry entry, PropertyAction action) { action.addOperation(PropertyOperations.create(1, finalProp)); } });
      textStream.modifyPapx(startCp, entry.getNext().position(), &IPropertyModify::new());
    }

    // java private void rejectSingleTableRevision(int startCp, IPapxTable.Entry entry, ITextStream textStream)
    pub fn rejectSingleTableRevision(&mut self, startCp: i32, entry: &dyn IPapxTableEntry, textStream: &mut dyn ITextStream) {
      // java Assert.assertNotNull("entry should not be null!", entry);
      Assert::assertNotNull__string_object(&"entry should not be null!", Some(&entry));
      // java Assert.assertNotNull("textStream should not be null!", textStream);
      Assert::assertNotNull__string_object(&"textStream should not be null!", Some(&textStream));
      // java final PropertySet prop = getTableRevison(entry.getProperty());
      let mut prop: PropertySet = Self::getTableRevison(&entry.getProperty());
      // java textStream.modifyPapx(startCp, entry.getNext().position(), new ITextStream.IPropertyModify() { public void compose(ITable.Entry entry, PropertyAction action) { action.addOperation(PropertyOperations.create(3, prop)); } });
      textStream.modifyPapx(startCp, entry.getNext().position(), &IPropertyModify::new());
    }

    // java private static PropertySet getTableRevison(PropertySet prop)
    pub fn getTableRevison(prop: &PropertySet) -> PropertySet {
      // java Assert.assertNotNull("prop should not be null!", prop);
      Assert::assertNotNull__string_object(&"prop should not be null!", Some(&prop));
      // java MutablePropertySet mutaProp = new MutablePropertySet();
      let mut mutaProp: MutablePropertySet = MutablePropertySet::new();
      // java Object value = prop.getValue(363);
      let mut value: Box<dyn Object> = prop.getValue(363);
      // java Assert.assertNotNull("value should not be null!", value);
      Assert::assertNotNull__string_object(&"value should not be null!", Some(&value));
      // java mutaProp.setValue(363, (Revision)value);
      mutaProp.setValue(363, (value).downcast_ref::<Revision>().unwrap());
      // java value = prop.getValue(364);
      value = prop.getValue(364);
      // java if (null != value)
      if value.is_some() {
        // java mutaProp.setValue(364, value);
        mutaProp.setValue(364, value);
      }
      // java value = prop.getValue(365);
      value = prop.getValue(365);
      // java if (null != value)
      if value.is_some() {
        // java mutaProp.setValue(365, value);
        mutaProp.setValue(365, value);
      }
      // java return mutaProp.freeze();
      return mutaProp.freeze();
    }

    // java private void changeSingleTableFormatRevision(int startCp, IPapxTable.Entry entry, ITextStream textStream, KRevisionChange.RevisionChange change)
    pub fn changeSingleTableFormatRevision(&mut self, startCp: i32, entry: &dyn IPapxTableEntry, textStream: &dyn ITextStream, change: &KRevisionChange::RevisionChange) {
      // java Assert.assertNotNull("entry should not be null!", entry);
      Assert::assertNotNull__string_object(&"entry should not be null!", Some(&entry));
      // java Assert.assertNotNull("textStream should not be null!", textStream);
      Assert::assertNotNull__string_object(&"textStream should not be null!", Some(&textStream));
      // java Assert.assertNotNull("change should not be null!", change);
      Assert::assertNotNull__string_object(&"change should not be null!", Some(&change));
      // java switch (change)
      match change {
        // java case ACCEPT:
        ACCEPT => {
          // java this.acceptSingleTableRevision(startCp, entry, textStream);
          self.acceptSingleTableRevision(startCp, &entry, &mut textStream);
        }
        // java case REJECT:
        REJECT => {
          // java this.rejectSingleTableRevision(startCp, entry, textStream);
          self.rejectSingleTableRevision(startCp, &entry, &mut textStream);
        }
        // java default:
        _ => {
          // java Assert.fail("It should not reach here!");
          Assert::fail__string(&"It should not reach here!");
        }
      }
    }

    // java private static IPapxTable.Entry getTableRowEndEntry(Document document, IPapxTable papxTable, int cp, int level)
    pub fn getTableRowEndEntry__document_ipapxtable_int_int(document: &dyn Document, papxTable: &mut dyn IPapxTable, cp: i32, level: i32) -> Box<dyn IPapxTableEntry> {
      // java Assert.assertNotNull("papxTable should not be null!", papxTable);
      Assert::assertNotNull__string_object(&"papxTable should not be null!", Some(&papxTable));
      // java IPapxTable.Entry rowEntry = papxTable.seek(cp);
      let mut rowEntry: Box<dyn IPapxTableEntry> = papxTable.seek(cp);
      // java return rowEntry.isEnd() ? rowEntry : getTableRowEndEntry(document, rowEntry, level);
      return (if rowEntry.isEnd() { rowEntry } else { Self::getTableRowEndEntry__document_entry_int(&document, &rowEntry, level) });
    }

    // java private static IPapxTable.Entry getTableRowEndEntry(Document document, IPapxTable.Entry rowEntry, int level)
    pub fn getTableRowEndEntry__document_entry_int(document: &dyn Document, rowEntry: &dyn IPapxTableEntry, level: i32) -> Box<dyn IPapxTableEntry> {
      // java Assert.assertNotNull("rowEntry should not be null!", rowEntry);
      Assert::assertNotNull__string_object(&"rowEntry should not be null!", Some(&rowEntry));
      // java IPapxTable.Entry e;
      let mut e: Box<dyn IPapxTableEntry> = Box::new(()) as Box<dyn IPapxTableEntry>;
      e = rowEntry;
      // java for(e = rowEntry !e.isEnd() && !isRowEnd(document, e, level) e = e.getNext())
      while !e.isEnd() && !Self::isRowEnd(&mut document, &mut e, level) {
      }
      // java return e;
      return e;
    }

    // java private static IPapxTable.Entry getTableRowEndEntry(IPapxTable papxTable, int cp)
    pub fn getTableRowEndEntry__ipapxtable_int(papxTable: &mut dyn IPapxTable, cp: i32) -> Box<dyn IPapxTableEntry> {
      // java Assert.assertNotNull("papxTable should not be null!", papxTable);
      Assert::assertNotNull__string_object(&"papxTable should not be null!", Some(&papxTable));
      // java IPapxTable.Entry rowEntry = papxTable.seek(cp);
      let mut rowEntry: Box<dyn IPapxTableEntry> = papxTable.seek(cp);
      // java return rowEntry.isEnd() ? rowEntry : getTableRowEndEntry(rowEntry);
      return (if rowEntry.isEnd() { rowEntry } else { Self::getTableRowEndEntry__entry(&rowEntry) });
    }

    // java private static IPapxTable.Entry getEndCpTableRowEndEntry(IPapxTable papxTable, int cp)
    pub fn getEndCpTableRowEndEntry(papxTable: &mut dyn IPapxTable, cp: i32) -> Box<dyn IPapxTableEntry> {
      // java Assert.assertNotNull("papxTable should not be null!", papxTable);
      Assert::assertNotNull__string_object(&"papxTable should not be null!", Some(&papxTable));
      // java IPapxTable.Entry rowEntry = papxTable.seek(cp);
      let mut rowEntry: Box<dyn IPapxTableEntry> = papxTable.seek(cp);
      // java if (rowEntry.isEnd())
      if rowEntry.isEnd() {
        // java return rowEntry;
        return rowEntry;
      }
      // java else
      else {
        // java PropertySet prop = rowEntry.getProperty();
        let mut prop: PropertySet = rowEntry.getProperty();
        // java return !prop.getBoolean(223, false) ? rowEntry : getTableRowEndEntry(rowEntry);
        return (if !prop.getBoolean(223, false) { rowEntry } else { Self::getTableRowEndEntry__entry(&rowEntry) });
      }
    }

    // java private static IPapxTable.Entry getTableRowEndEntry(IPapxTable.Entry rowEntry)
    pub fn getTableRowEndEntry__entry(rowEntry: &dyn IPapxTableEntry) -> Box<dyn IPapxTableEntry> {
      // java Assert.assertNotNull("rowEntry should not be null!", rowEntry);
      Assert::assertNotNull__string_object(&"rowEntry should not be null!", Some(&rowEntry));
      // java IPapxTable.Entry e;
      let mut e: Box<dyn IPapxTableEntry> = Box::new(()) as Box<dyn IPapxTableEntry>;
      e = rowEntry;
      // java for(e = rowEntry !e.isEnd() e = e.getNext())
      while !e.isEnd() {
        // java PropertySet prop = e.getProperty();
        let mut prop: PropertySet = e.getProperty();
        // java if (prop.getBoolean(225, false))
        if prop.getBoolean(225, false) {
          // java break;
          break;
        }
      }
      // java return e;
      return e;
    }

    // java private void acceptCellsInRow(IPapxTable.Entry rowEntry)
    pub fn acceptCellsInRow(&self, rowEntry: &dyn IPapxTableEntry) {
      // java Assert.assertNotNull("rowEntry should not be null!", rowEntry);
      Assert::assertNotNull__string_object(&"rowEntry should not be null!", Some(&rowEntry));
      // java ArrayList<IChpxTable.Entry> insertCellList = this.getInsertCellEntry(rowEntry);
      let mut insertCellList: Vec<IChpxTable::Entry> = self.getInsertCellEntry(&rowEntry);
      // java if (null != insertCellList)
      if insertCellList.is_some() {
        // java KRevisionChange kRevisionChange = new KRevisionChange(this.mDocument);
        let mut kRevisionChange: KRevisionChange = KRevisionChange::new(self.mDocument);
        // java int len = insertCellList.size();
        let mut len: i32 = (insertCellList.len() as i32);
        let mut i: i32 = 0;
        // java for(int i = 0; i < len; ++i)
        while i < len {
          // java IChpxTable.Entry cellRunEntry = (IChpxTable.Entry)insertCellList.get(i);
          let mut cellRunEntry: Box<dyn IChpxTableEntry> = todo!("cast → Box<dyn IChpxTableEntry>");
          // java kRevisionChange.acceptInsertRevision(cellRunEntry.position(), cellRunEntry.endCp(), this.mDocument.getTextStream());
          kRevisionChange.acceptInsertRevision(cellRunEntry.position(), cellRunEntry.endCp(), self.mDocument.getTextStream());
        }
      }
    }

    // java private ArrayList<IChpxTable.Entry> getInsertCellEntry(IPapxTable.Entry rowEntry)
    pub fn getInsertCellEntry(&self, rowEntry: &dyn IPapxTableEntry) -> Option<Vec<IChpxTable::Entry>> {
      // java Assert.assertNotNull("rowEntry should not be null!", rowEntry);
      Assert::assertNotNull__string_object(&"rowEntry should not be null!", Some(&rowEntry));
      // java ArrayList<IPapxTable.Entry> cellEntryList = this.getCellEndList(rowEntry);
      let mut cellEntryList: Vec<IPapxTable::Entry> = self.getCellEndList(&rowEntry);
      // java int cellSize = cellEntryList.size();
      let mut cellSize: i32 = (cellEntryList.len() as i32);
      // java if (cellSize <= 0)
      if cellSize <= 0 {
        // java return null;
        return None;
      }
      // java else
      else {
        // java IChpxTable chpxTable = this.mDocument.getChpxTable();
        let mut chpxTable: Box<dyn IChpxTable> = self.mDocument.getChpxTable();
        // java ArrayList<IChpxTable.Entry> cellRunEntryList = new ArrayList();
        let mut cellRunEntryList: Vec<IChpxTable::Entry> = Vec::new();
        let mut i: i32 = 0;
        // java for(int i = 0; i < cellSize; ++i)
        while i < cellSize {
          // java IPapxTable.Entry cellEntry = (IPapxTable.Entry)cellEntryList.get(i);
          let mut cellEntry: Box<dyn IPapxTableEntry> = todo!("cast → Box<dyn IPapxTableEntry>");
          // java IChpxTable.Entry cellRunEntry = chpxTable.seek(cellEntry.position());
          let mut cellRunEntry: Box<dyn IChpxTableEntry> = chpxTable.seek(cellEntry.position());
          // java PropertySet prop = cellRunEntry.getProperty();
          let mut prop: PropertySet = cellRunEntry.getProperty();
          // java if (prop.contains(46))
          if prop.contains(46) {
            // java cellRunEntryList.add(cellRunEntry);
            cellRunEntryList.add(cellRunEntry);
          }
        }
        // java int revisionCellSize = cellRunEntryList.size();
        let mut revisionCellSize: i32 = (cellRunEntryList.len() as i32);
        // java if (revisionCellSize == cellSize)
        if revisionCellSize == cellSize {
          // java return null;
          return None;
        }
        // java else if (revisionCellSize <= 0)
        else if revisionCellSize <= 0 {
          // java return null;
          return None;
        }
        // java else
        else {
          // java return cellRunEntryList;
          return cellRunEntryList;
        }
      }
    }

    // java private ArrayList<IPapxTable.Entry> getCellEndList(IPapxTable.Entry rowEndEntry)
    pub fn getCellEndList(&self, rowEndEntry: &dyn IPapxTableEntry) -> Vec<IPapxTable::Entry> {
      // java Assert.assertNotNull("rowEndEntry should not be null!", rowEndEntry);
      Assert::assertNotNull__string_object(&"rowEndEntry should not be null!", Some(&rowEndEntry));
      // java ArrayList<IPapxTable.Entry> cellEntryList = new ArrayList();
      let mut cellEntryList: Vec<IPapxTable::Entry> = Vec::new();
      let mut e: Box<dyn IPapxTableEntry> = rowEndEntry.getPrev();
      // java for(IPapxTable.Entry e = rowEndEntry.getPrev(); !e.isHead() e = e.getPrev())
      while !e.isHead() {
        // java PropertySet prop = e.getProperty();
        let mut prop: PropertySet = e.getProperty();
        // java int level = prop.getInt(224, 1);
        let mut level: i32 = prop.getInt(224, 1);
        // java if (isRowEnd(this.mDocument, e, level) || !prop.getBoolean(223, false))
        if Self::isRowEnd(&mut self.mDocument, &mut e, level) || !prop.getBoolean(223, false) {
          // java break;
          break;
        }
        // java if (isCellEnd(this.mDocument, e, level))
        if Self::isCellEnd(&mut self.mDocument, &mut e, level) {
          // java cellEntryList.add(e);
          cellEntryList.add(e);
        }
      }
      // java return cellEntryList;
      return cellEntryList;
    }

    // java private static boolean isCellEnd(Document document, IPapxTable.Entry paraEntry, int level)
    pub fn isCellEnd(document: &mut dyn Document, paraEntry: &mut dyn IPapxTableEntry, level: i32) -> bool {
      // java Assert.assertNotNull("document should not be null!", document);
      Assert::assertNotNull__string_object(&"document should not be null!", Some(&document));
      // java Assert.assertNotNull("paraEntry should not be null!", paraEntry);
      Assert::assertNotNull__string_object(&"paraEntry should not be null!", Some(&paraEntry));
      // java PropertySet prop = paraEntry.getProperty();
      let mut prop: PropertySet = paraEntry.getProperty();
      // java if (prop == null)
      if prop.is_none() {
        // java prop = PropertySet.empty;
        prop = PropertySet::empty();
      }
      // java if (prop.getInt(224, 1) != level)
      if prop.getInt(224, 1) != level {
        // java return false;
        return false;
      }
      // java else
      else {
        // java char c = document.charAt(paraEntry.endCp() - 1);
        let mut c: char = document.charAt(paraEntry.endCp() - 1);
        // java if (level == 1)
        if level == 1 {
          // java return c == 7;
          return c == 7;
        }
        // java else
        else {
          // java return c == '\r' && prop.getBoolean(226, false);
          return c == '\r' && prop.getBoolean(226, false);
        }
      }
    }

    // java static boolean isRowEnd(Document document, IPapxTable.Entry paraEntry, int level)
    pub fn isRowEnd(document: &mut dyn Document, paraEntry: &mut dyn IPapxTableEntry, level: i32) -> bool {
      // java Assert.assertNotNull("document should not be null!", document);
      Assert::assertNotNull__string_object(&"document should not be null!", Some(&document));
      // java Assert.assertNotNull("paraEntry should not be null!", paraEntry);
      Assert::assertNotNull__string_object(&"paraEntry should not be null!", Some(&paraEntry));
      // java PropertySet prop = paraEntry.getProperty();
      let mut prop: PropertySet = paraEntry.getProperty();
      // java if (prop == null)
      if prop.is_none() {
        // java prop = PropertySet.empty;
        prop = PropertySet::empty();
      }
      // java if (prop.getInt(224, 1) != level)
      if prop.getInt(224, 1) != level {
        // java return false;
        return false;
      }
      // java else if (!prop.getBoolean(225, false))
      else if !prop.getBoolean(225, false) {
        // java return false;
        return false;
      }
      // java else
      else {
        // java char c = document.charAt(paraEntry.endCp() - 1);
        let mut c: char = document.charAt(paraEntry.endCp() - 1);
        // java if (level == 1)
        if level == 1 {
          // java return c == 7;
          return c == 7;
        }
        // java else
        else {
          // java return c == '\r';
          return c == '\r';
        }
      }
    }

    // java static boolean checkTableRevision(Document document)
    pub fn checkTableRevision(document: &dyn Document) -> bool {
      // java Assert.assertNotNull("document should not be null !", document);
      Assert::assertNotNull__string_object(&"document should not be null !", Some(&document));
      // java Assert.assertTrue("document.getLength() > 0 should be true!", document.getLength() > 0);
      Assert::assertTrue__string_boolean(&"document.getLength() > 0 should be true!", document.getLength() > 0);
      let mut entry: Box<dyn IPapxTableEntry> = document.getPapxTable().seek(0);
      // java for(IPapxTable.Entry entry = document.getPapxTable().seek(0); !entry.isEnd() entry = entry.getNext())
      while !entry.isEnd() {
        // java PropertySet prop = entry.getProperty();
        let mut prop: PropertySet = entry.getProperty();
        // java if (prop.contains(363))
        if prop.contains(363) {
          // java return true;
          return true;
        }
      }
      // java return false;
      return false;
    }

    // java private static PropertySet acceptTableFormat(KStyles styles, PropertySet prop)
    pub fn acceptTableFormat(styles: &KStyles, prop: &PropertySet) -> PropertySet {
      // java Assert.assertNotNull("styles should not be null !", styles);
      Assert::assertNotNull__string_object(&"styles should not be null !", Some(&styles));
      // java Assert.assertNotNull("prop should not be null !", prop);
      Assert::assertNotNull__string_object(&"prop should not be null !", Some(&prop));
      // java PropertySet finalProp = PropertyUtil.getFinalTableFormat(styles, prop);
      let mut finalProp: PropertySet = PropertyUtil::getFinalTableFormat(&styles, &prop);
      // java Revision revision = (Revision)prop.getValue(363);
      let mut revision: Revision = (prop.getValue(363)).downcast_ref::<Revision>().unwrap();
      // java if (null != revision)
      if revision.is_some() {
        // java CellMargin[] resultMargin = mergeDefCellMarginProp(prop, revision.getProperty());
        let mut resultMargin: Vec<CellMargin> = Self::mergeDefCellMarginProp(&prop, &revision.getProperty());
        // java if (null != resultMargin)
        if resultMargin.is_some() {
          // java MutablePropertySet mPropertySet = new MutablePropertySet(finalProp);
          let mut mPropertySet: MutablePropertySet = MutablePropertySet::new(finalProp);
          // java mPropertySet.setValue(301, resultMargin);
          mPropertySet.setValue(301, resultMargin);
          // java return mPropertySet.freeze();
          return mPropertySet.freeze();
        }
      }
      // java return finalProp;
      return finalProp;
    }

    // java private static CellMargin[] mergeDefCellMarginProp(PropertySet originalProp, PropertySet revisionProp)
    pub fn mergeDefCellMarginProp(originalProp: &PropertySet, revisionProp: &PropertySet) -> Option<Vec<CellMargin>> {
      // java Assert.assertNotNull("originalProp should not be null !", originalProp);
      Assert::assertNotNull__string_object(&"originalProp should not be null !", Some(&originalProp));
      // java Assert.assertNotNull("revisionProp should not be null !", revisionProp);
      Assert::assertNotNull__string_object(&"revisionProp should not be null !", Some(&revisionProp));
      // java Object originalValue = originalProp.getValue(301);
      let mut originalValue: Box<dyn Object> = originalProp.getValue(301);
      // java Object value = revisionProp.getValue(301);
      let mut value: Box<dyn Object> = revisionProp.getValue(301);
      // java return null != originalValue && null != value ? getResultDefCellMargin((CellMargin[])originalValue, (CellMargin[])value) : null;
      return (if originalValue.is_some() && value.is_some() { Some(Self::getResultDefCellMargin(&todo!("cast → Vec<CellMargin>"), &todo!("cast → Vec<CellMargin>"))) } else { None });
    }

    // java private static CellMargin[] getResultDefCellMargin(CellMargin[] originalMargins, CellMargin[] margins)
    pub fn getResultDefCellMargin(originalMargins: &Vec<CellMargin>, margins: &Vec<CellMargin>) -> Option<Vec<CellMargin>> {
      // java Assert.assertNotNull("originalMargins should not be null !", originalMargins);
      Assert::assertNotNull__string_object(&"originalMargins should not be null !", Some(&originalMargins));
      // java Assert.assertNotNull("margins should not be null !", margins);
      Assert::assertNotNull__string_object(&"margins should not be null !", Some(&margins));
      // java ArrayList<CellMargin> resultMargin = new ArrayList();
      let mut resultMargin: Vec<CellMargin> = Vec::new();
      // java int originalLen = originalMargins.length;
      let mut originalLen: i32 = (originalMargins.len() as i32);
      // java for(CellMargin m : margins)
      for m in margins.iter() {
        // java if (null != m)
        if m.is_some() {
          // java int bits = m.getGrbitChg();
          let mut bits: i32 = m.getGrbitChg();
          let mut j: i32 = 0;
          // java for(int j = 0; j < originalLen ++j)
          while j < originalLen {
            // java CellMargin originalM = originalMargins[j];
            let mut originalM: CellMargin = originalMargins[(j) as usize];
            // java if (null != originalM)
            if originalM.is_some() {
              // java int originalBits = originalM.getGrbitChg();
              let mut originalBits: i32 = originalM.getGrbitChg();
              // java int resultBits = originalBits ^ originalBits & bits;
              let mut resultBits: i32 = originalBits ^ originalBits & bits;
              // java if (0 != resultBits)
              if 0 != resultBits {
                // java CellMargin mergeMargin = CellMargin.obtain(originalM, originalBits);
                let mut mergeMargin: CellMargin = CellMargin::obtain__cellmargin_int(&originalM, originalBits);
                // java if (!resultMargin.contains(mergeMargin))
                if !resultMargin.contains(mergeMargin) {
                  // java resultMargin.add(mergeMargin);
                  resultMargin.add(mergeMargin);
                }
              }
            }
          }
        }
      }
      // java addToArrayList(resultMargin, margins);
      Self::addToArrayList(&mut resultMargin, &margins);
      // java if (resultMargin.size() > 0)
      if (resultMargin.len() as i32) > 0 {
        // java return (CellMargin[])resultMargin.toArray(new CellMargin[resultMargin.size()]);
        return todo!("cast → Vec<CellMargin>");
      }
      // java else
      else {
        // java return null;
        return None;
      }
    }

    // java private static void addToArrayList(ArrayList<CellMargin> resultMargin, CellMargin[] originalMargins)
    pub fn addToArrayList(resultMargin: &mut Vec<CellMargin>, originalMargins: &Vec<CellMargin>) {
      // java Assert.assertNotNull("resultMargin should not be null !", resultMargin);
      Assert::assertNotNull__string_object(&"resultMargin should not be null !", Some(&resultMargin));
      // java Assert.assertNotNull("originalMargins should not be null !", originalMargins);
      Assert::assertNotNull__string_object(&"originalMargins should not be null !", Some(&originalMargins));
      // java for(CellMargin cellMargin : originalMargins)
      for cellMargin in originalMargins.iter() {
        // java if (null != cellMargin)
        if cellMargin.is_some() {
          // java CellMargin mergeMargin = CellMargin.obtain(cellMargin, cellMargin.getGrbitChg());
          let mut mergeMargin: CellMargin = CellMargin::obtain__cellmargin_int(&cellMargin, cellMargin.getGrbitChg());
          // java if (!resultMargin.contains(mergeMargin))
          if !resultMargin.contains(mergeMargin) {
            // java resultMargin.add(mergeMargin);
            resultMargin.add(mergeMargin);
          }
        }
      }
    }

}
