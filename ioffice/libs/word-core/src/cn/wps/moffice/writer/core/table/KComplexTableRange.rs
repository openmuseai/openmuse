// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.KRange
use crate::cn::wps::moffice::writer::core::KRange::*;
// Java: import cn.wps.moffice.writer.core.RangeTool
use crate::cn::wps::moffice::writer::core::RangeTool::*;
// Java: import cn.wps.moffice.writer.core.shape.InkFilter
// CARGO_BLOCKER: InkFilter screened pending spanset/IFilter repair
// use crate::cn::wps::moffice::writer::core::shape::InkFilter::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::{
    Entry as IPapxTableEntry, IPapxTable,
};
// Java: import cn.wps.moffice.writer.data.table.KTableManager
use crate::cn::wps::moffice::writer::data::table::KTableManager::*;
// Java: import cn.wps.moffice.writer.data.table.TableInfo
use crate::cn::wps::moffice::writer::data::table::TableInfo::*;
// Java: import java.util.ArrayList
// Java: import cn.wps.moffice.writer.core.table.KTableRangeBase
use crate::cn::wps::moffice::writer::core::table::KTableRangeBase::*;
// Java: import cn.wps.moffice.writer.core.table.RowRevisionFixer
use crate::cn::wps::moffice::writer::core::table::RowRevisionFixer::*;
// Java: import org.apache.poi.poifs.filesystem.Entry
use io_poi_compact::org::apache::poi::poifs::filesystem::Entry::*;

// java public class KComplexTableRange extends KTableRangeBase
#[derive(Default)]
pub struct KComplexTableRange {
    // java KTableRangeBase super_
    pub(crate) super_: KTableRangeBase,
    // java private ArrayList<KTableRangeBase> mRanges = new ArrayList();
    mRanges: Option<Vec<KTableRangeBase>>,
    // java boolean fNormalBegin = false;
    pub(crate) fNormalBegin: bool,
    // java boolean fPartTableEnd = false;
    pub(crate) fPartTableEnd: bool,
}

// java static class NormalRange extends KTableRangeBase
#[derive(Default)]
pub struct NormalRange {
    // java KTableRangeBase super_
    pub(crate) super_: KTableRangeBase,
    // java private int mSubTableLevel;
    mSubTableLevel: i32,
    // java private KRange mRange;
    mRange: Option<KRange>,
}

// java public class KComplexTableRange extends KTableRangeBase
impl KComplexTableRange {
    // java public  KComplexTableRange(Document document)
    pub fn new__document(document: &dyn Document) -> Self {
      // java this.document = document;
      Self { super_: KTableRangeBase::new(), mRanges: Default::default(), fNormalBegin: Default::default(), fPartTableEnd: Default::default() }
    }

    // java public  KComplexTableRange(Document document, int start, int end, int level)
    pub fn new__document_int_int_int(document: &dyn Document, start: i32, end: i32, level: i32) -> Self {
      // java this.document = document;
      let mut this = Self { super_: KTableRangeBase::new(), mRanges: Default::default(), fNormalBegin: Default::default(), fPartTableEnd: Default::default() };
      // java this.add(start, end, level);
      this.add(start, end, level);
      this
    }

    // java public void add(int start, int end, int level)
    pub fn add(&mut self, start: i32, end: i32, level: i32) {
      // java if (RangeTool.isEndOfSubDocumentsPLCNode(this.document, end))
      if RangeTool::isEndOfSubDocumentsPLCNode(&self.document, end) {
        // java --end;
        { let _tmp = end; end -= 1; _tmp };
      }
      // java KTableManager tableManager = this.document.getTableInfos().getTableManager();
      let mut tableManager: KTableManager = self.document.getTableInfos().getTableManager();
      // java int tableEnd;
      let mut tableEnd: i32;
      let mut cpStart: i32 = start;
      // java for(int cpStart = start; start < end start = tableEnd)
      while start < end {
        // java int tableStart = tableManager.findNextRowStart(start, end, level);
        let mut tableStart: i32 = tableManager.findNextRowStart(start, end, level);
        // java if (start < tableStart)
        if start < tableStart {
          // java this.mRanges.add(new NormalRange(this.document, start, tableStart, level));
          self.mRanges.add(NormalRange::new(self.document, start, tableStart, level));
          // java if (start == cpStart)
          if start == cpStart {
            // java this.fNormalBegin = this.document.getPapxTable().seek(start).position() < cpStart;
            self.fNormalBegin = self.document.getPapxTable().seek(start).position() < cpStart;
          }
        }
        // java if (tableStart >= end)
        if tableStart >= end {
          // java this.fPartTableEnd = tableManager.isInTable(end, level);
          self.fPartTableEnd = tableManager.isInTable(end, level);
          // java break;
          break;
        }
        // java TableInfo tableInfo = this.document.getTableInfos().getTableInfoByLevel(tableStart, level);
        let mut tableInfo: TableInfo = self.document.getTableInfos().getTableInfoByLevel(tableStart, level);
        // java this.fPartTableEnd = tableInfo.getEndCP() != end;
        self.fPartTableEnd = tableInfo.getEndCP() != end;
        // java tableEnd = Math.min(tableInfo.getEndCP(), end);
        tableEnd = Math::min(tableInfo.getEndCP(), end);
        // java this.mRanges.add(KRowRange.createRowRange(this.document, tableStart, tableEnd));
        self.mRanges.add(KRowRange::createRowRange(&self.document, tableStart, tableEnd));
      }
    }

    // java protected void beforeRemoveRowInRevision(RowRevisionFixer fixer)
    pub fn beforeRemoveRowInRevision(&self, fixer: &RowRevisionFixer) {
      let mut i: i32 = (self.mRanges.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) - 1;
      // java for(int i = this.mRanges.size() - 1; i >= 0 --i)
      while i >= 0 {
        // java ((KTableRangeBase)this.mRanges.get(i)).beforeRemoveRowInRevision(fixer);
        (self.mRanges.get(i)).downcast_ref::<KTableRangeBase>().unwrap().beforeRemoveRowInRevision(fixer);
      }
    }

    // java public int delete(KRange.DeleteRangeReason reason)
    pub fn delete(&self, reason: &crate::cn::wps::moffice::writer::core::KRange::DeleteRangeReason) -> i32 {
      // java if (this.fNormalBegin && this.fPartTableEnd && this.document.getTextDocument().getTrackChange() && this.document.getTextStream().getRevisionFileter())
      if self.fNormalBegin && self.fPartTableEnd && self.document.getTextDocument().getTrackChange() && self.document.getTextStream().getRevisionFileter() {
        // java return -1;
        return -1;
      }
      // java else
      else {
        // java int removed = 0;
        let mut removed: i32 = 0;
        let mut i: i32 = (self.mRanges.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) - 1;
        // java for(int i = this.mRanges.size() - 1; i >= 0 --i)
        while i >= 0 {
          // java removed += ((KTableRangeBase)this.mRanges.get(i)).delete(KRange.DeleteRangeReason.backspace_key);
          removed = removed + (self.mRanges.get(i)).downcast_ref::<KTableRangeBase>().unwrap().delete(KRange::DeleteRangeReason.backspace_key);
        }
        // java return removed;
        return removed;
      }
    }

}

impl std::ops::Deref for KComplexTableRange {
    type Target = KTableRangeBase;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for KComplexTableRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java static class NormalRange extends KTableRangeBase
impl NormalRange {
    // java public  NormalRange(Document document, int start, int end, int subTableLevel)
    pub fn NormalRange(document: &dyn Document, start: i32, end: i32, subTableLevel: i32) -> Self {
      // java this.mRange = document.getRange(start, end);
      Self { super_: KTableRangeBase::new(), mSubTableLevel: subTableLevel, mRange: document.getRange__int_int(start, end) }
      // java this.mSubTableLevel = subTableLevel;
    }

    // java public int delete(KRange.DeleteRangeReason reason)
    pub fn delete(&mut self, reason: &crate::cn::wps::moffice::writer::core::KRange::DeleteRangeReason) -> i32 {
      // java Document document = this.mRange.getSubDocument();
      let mut document: Box<dyn Document> = self.mRange.getSubDocument();
      // java PropertySet endProp = document.getPapxTable().seek(this.mRange.getEnd()).getProperty();
      let mut endProp: PropertySet = document.getPapxTable().seek(self.mRange.getEnd()).getProperty();
      // java int endLevel = endProp.getInt(224, 0);
      let mut endLevel: i32 = endProp.getInt(224, 0);
      // java int removed = this.mRange.removeText(new InkFilter());
      // CARGO_BLOCKER: InkFilter screened
      let mut removed: i32 = 0;
      // java if (endLevel >= this.mSubTableLevel)
      if endLevel >= self.mSubTableLevel {
        // java IPapxTable.Entry cellStart = document.getPapxTable().seek(this.mRange.getEnd());
        let mut cellStart: Box<dyn IPapxTableEntry> = document.getPapxTable().seek(self.mRange.getEnd());
        // java int cellStartCp = cellStart.position();
        let mut cellStartCp: i32 = cellStart.position();
        // java if (cellStartCp < this.mRange.getEnd())
        if cellStartCp < self.mRange.getEnd() {
          // java cellStart.setProperty(endProp);
          cellStart.setProperty(endProp);
        }
      }
      // java return removed;
      return removed;
    }

}

impl std::ops::Deref for NormalRange {
    type Target = KTableRangeBase;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for NormalRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
