// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::{
    Entry as IPapxTableEntry, IPapxTable,
};
// Java: import cn.wps.moffice.writer.data.PropertyUtil
use crate::cn::wps::moffice::writer::data::PropertyUtil::*;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::*;
// Java: import org.apache.poi.poifs.filesystem.Entry
use io_poi_compact::org::apache::poi::poifs::filesystem::Entry::*;

// java static final int INVALID_CELL_INDEX = -1;
pub(crate) const INVALID_CELL_INDEX: i32 = -1;

// java public class CellsItem
#[derive(Default)]
pub struct CellsItem {
    // java public Document document = null;
    pub(crate) document: Option<Box<dyn Document>>,
    // java public int rowEndIndex = -1;
    pub(crate) rowEndIndex: i32,
    // java public long rowRange;
    pub(crate) rowRange: i64,
    // java public int startCellIndex;
    pub(crate) startCellIndex: i32,
    // java public int endCellIndex;
    pub(crate) endCellIndex: i32,
    // java public long cellsRange;
    pub(crate) cellsRange: i64,
}

// java public class CellsItem
impl CellsItem {
    // java public  CellsItem(Document document, Long rowRange)
    pub fn CellsItem(document: &dyn Document, rowRange: i64) -> Self {
      // java this.document = document;
      let mut this = Self { document: document, rowEndIndex: Default::default(), rowRange: rowRange, startCellIndex: -1, endCellIndex: -1, cellsRange: Default::default() };
      // java this.rowEndIndex = document.getTextRope().cp2fc(RANGEFactory.getEnd(rowRange) - 1); this.rowRange = rowRange;
      this.rowEndIndex = document.getTextRope().cp2fc(RANGEFactory::getEnd(rowRange) - 1);
      // java this.startCellIndex = -1;
      // java this.endCellIndex = -1;
      this
    }

    // java public PropertySet getRowProperty()
    pub fn getRowProperty(&self) -> Option<PropertySet> {
      // java if (null != this.document && -1 != this.rowEndIndex)
      if let Some(document) = self.document.as_ref() {
        if -1 != self.rowEndIndex {
          // java IPapxTable.Entry entry = this.document.getPapxTable().seek(this.document.getTextRope().fc2cp(this.rowEndIndex));
          let mut entry: Box<dyn IPapxTableEntry> = document.getPapxTable().seek(document.getTextRope().fc2cp(self.rowEndIndex));
          // java return null == entry ? null : entry.getProperty();
          return (if entry.is_none() { None } else { Some(entry.getProperty()) });
        }
      }
      // java else
      else {
        // java return null;
        return None;
      }
    }

    // java public PropertySet getFinalRowProperty()
    pub fn getFinalRowProperty(&self) -> Option<PropertySet> {
      // java if (null != this.document && -1 != this.rowEndIndex)
      if let Some(document) = self.document.as_ref() {
        if -1 != self.rowEndIndex {
          // java IPapxTable.Entry entry = this.document.getPapxTable().seek(this.document.getTextRope().fc2cp(this.rowEndIndex));
          let mut entry: Box<dyn IPapxTableEntry> = document.getPapxTable().seek(document.getTextRope().fc2cp(self.rowEndIndex));
          // java if (null == entry)
          if entry.is_none() {
            // java return null;
            return None;
          }
          // java else
          else {
            // java PropertySet rowProp = entry.getProperty();
            let mut rowProp: PropertySet = entry.getProperty();
            // java return PropertyUtil.getFinalTableFormat(this.document.getStyles(), rowProp);
            return PropertyUtil::getFinalTableFormat(&document.getStyles(), &rowProp);
          }
        }
      }
      // java else
      else {
        // java return null;
        return None;
      }
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java return "[" + RANGEFactory.getStart(this.rowRange) + "[" + RANGEFactory.getStart(this.cellsRange) + "," + RANGEFactory.getEnd(this.cellsRange) + ")" + RANGEFactory.getEnd(this.rowRange) + ") " + this.startCellIndex + "-" + this.endCellIndex;
      return "[" + RANGEFactory::getStart(self.rowRange) + "[" + RANGEFactory::getStart(self.cellsRange) + "," + RANGEFactory::getEnd(self.cellsRange) + ")" + RANGEFactory::getEnd(self.rowRange) + ") " + self.startCellIndex + "-" + self.endCellIndex;
    }

}
