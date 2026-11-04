// Java: import cn.wps.moffice.property.MutablePropertySet
use util::moffice::property::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::*;
// Java: import cn.wps.moffice.writer.data.PropertyUtil
use crate::cn::wps::moffice::writer::data::PropertyUtil::*;
// Java: import cn.wps.moffice.writer.data.table.RowInfo
use crate::cn::wps::moffice::writer::data::table::RowInfo::*;
// Java: import cn.wps.moffice.writer.data.table.TableTool
use crate::cn::wps::moffice::writer::data::table::TableTool::*;
// Java: import cn.wps.moffice.writer.data.value.DefTable
use crate::cn::wps::moffice::writer::data::value::DefTable::*;
// Java: import cn.wps.moffice.writer.data.value.DefTableBuilder
use crate::cn::wps::moffice::writer::data::value::DefTableBuilder::*;
// Java: import cn.wps.moffice.writer.data.value.TableCellBuilder
use crate::cn::wps::moffice::writer::data::value::TableCellBuilder::*;
// Java: import org.apache.poi.poifs.filesystem.Entry
use io_poi_compact::org::apache::poi::poifs::filesystem::Entry::*;

// java private static DefTableTransport sDefaultTransport = new DefTableTransport() { public void setDefTable(Document document, int rowEndCp, DefTable defTable) { IPapxTable.Entry entry = document.getPapxTable().seek(rowEndCp); PropertySet finalProp = PropertyUtil.getFinalTableFormat(document.getStyles(), entry.getProperty()); MutablePropertySet propertySet = new MutablePropertySet(finalProp); propertySet.setValue(306, defTable); entry.setProperty(propertySet.freeze()); } public DefTable getDefTable(Document document, int rowEndCp) { return (DefTable)Tools.getPropFromRowProp(document.getPapxTable().seek(rowEndCp).getProperty(), 306); } };
static sDefaultTransport: std::sync::LazyLock<std::sync::Arc<Option<DefTableTransport>>> =
                    std::sync::LazyLock::new(|| std::sync::Arc::new(DefTableTransport::new()));

// java public class DefTableFixer
#[derive(Default)]
pub struct DefTableFixer {
    // java private Document mDocument;
    mDocument: Option<Box<dyn Document>>,
    // java private DefTableTransport mTransport;
    mTransport: Option<DefTableTransport>,
    // java private DefTableBuilder mSurvivor;
    mSurvivor: Option<DefTableBuilder>,
    // java private int mSurvivorFc;
    mSurvivorFc: i32,
    // java private DefTableBuilder mLastSurvivor;
    mLastSurvivor: Option<DefTableBuilder>,
    // java private int mLastSurvivorFc;
    mLastSurvivorFc: i32,
    // java private DefTable mAfterLastSurvivor;
    mAfterLastSurvivor: Option<DefTable>,
}

// java public class DefTableFixer
impl DefTableFixer {
    // java public  DefTableFixer(Document document)
    pub fn new__document(document: &dyn Document) -> Self {
      // java this(document, sDefaultTransport);
      let mut this = Self::new__document_deftabletransport(&document, &sDefaultTransport);
      this
    }

    // java public  DefTableFixer(Document document, DefTableTransport transport)
    pub fn new__document_deftabletransport(document: &dyn Document, transport: &DefTableTransport) -> Self {
      // java this.mDocument = document;
      Self { mDocument: document, mTransport: transport, mSurvivor: Default::default(), mSurvivorFc: Default::default(), mLastSurvivor: Default::default(), mLastSurvivorFc: Default::default(), mAfterLastSurvivor: Default::default() }
      // java this.mTransport = transport;
    }

    // java public void init(RowInfo afterRow)
    pub fn init(&mut self, afterRow: Option<&dyn RowInfo>) {
      // java if (afterRow != null)
      if let Some(afterRow) = afterRow {
        // java this.mSurvivor = DefTableBuilder.obtain((DefTable)Tools.getPropFromRowProp(afterRow.getOriginalProperty(), 306));
        self.mSurvivor = DefTableBuilder::obtain__deftable(&(Tools::getPropFromRowProp(afterRow.getOriginalProperty(), 306)).downcast_ref::<DefTable>().unwrap());
        // java this.mSurvivorFc = this.mDocument.getTextRope().cp2fc(afterRow.getEndCP() - 1);
        self.mSurvivorFc = self.mDocument.getTextRope().cp2fc(afterRow.getEndCP() - 1);
        // java RowInfo afterAfterRow = afterRow.getNextRow();
        let mut afterAfterRow: RowInfo = afterRow.getNextRow();
        // java if (afterAfterRow != null)
        if afterAfterRow.is_some() {
          // java this.mLastSurvivor = DefTableBuilder.obtain((DefTable)Tools.getPropFromRowProp(afterAfterRow.getOriginalProperty(), 306));
          self.mLastSurvivor = DefTableBuilder::obtain__deftable(&(Tools::getPropFromRowProp(afterAfterRow.getOriginalProperty(), 306)).downcast_ref::<DefTable>().unwrap());
        }
      }
    }

    // java public void addSurvivorRow(int newSurvivorFc)
    pub fn addSurvivorRow(&mut self, newSurvivorFc: i32) {
      // java int cp = this.mDocument.getTextRope().fc2cp(newSurvivorFc);
      let mut cp: i32 = self.mDocument.getTextRope().fc2cp(newSurvivorFc);
      // java DefTableBuilder newDefTable = DefTableBuilder.obtain(this.mTransport.getDefTable(this.mDocument, cp));
      let mut newDefTable: DefTableBuilder = DefTableBuilder::obtain__deftable(&self.mTransport.getDefTable(self.mDocument, cp));
      // java this.push(newDefTable, newSurvivorFc);
      self.push(&newDefTable, newSurvivorFc);
      // java if (this.mLastSurvivor != null && this.fixVertMergeAfter(this.mLastSurvivor, this.mSurvivor, this.mAfterLastSurvivor))
      if let Some(mLastSurvivor) = self.mLastSurvivor.as_mut() {
        if self.fixVertMergeAfter(&self.mLastSurvivor, &self.mSurvivor, &self.mAfterLastSurvivor) {
          // java this.doFixLastSurvivor();
          self.doFixLastSurvivor();
        }
      }
    }

    // java public void addBeforeRow(int beforeRowFc)
    pub fn addBeforeRow(&mut self, beforeRowFc: i32) {
      // java this.addSurvivorRow(beforeRowFc);
      self.addSurvivorRow(beforeRowFc);
      // java this.push((DefTableBuilder)null, -1);
      self.push(Default::default(), -1);
      // java if (this.mLastSurvivor != null && this.fixVertMergeBefore(this.mLastSurvivor, this.mAfterLastSurvivor))
      if let Some(mLastSurvivor) = self.mLastSurvivor.as_mut() {
        if self.fixVertMergeBefore(&self.mLastSurvivor, &self.mAfterLastSurvivor) {
          // java this.doFixLastSurvivor();
          self.doFixLastSurvivor();
        }
      }
    }

    // java public void noBeforeRow()
    pub fn noBeforeRow(&mut self) {
      // java this.push((DefTableBuilder)null, -1);
      self.push(Default::default(), -1);
      // java if (this.mLastSurvivor != null && this.fixVertMergeAfter(this.mLastSurvivor, (DefTable)null, this.mAfterLastSurvivor))
      if let Some(mLastSurvivor) = self.mLastSurvivor.as_mut() {
        if self.fixVertMergeAfter(&self.mLastSurvivor, Default::default(), &self.mAfterLastSurvivor) {
          // java this.doFixLastSurvivor();
          self.doFixLastSurvivor();
        }
      }
    }

    // java private void push(DefTableBuilder newSurvivor, int fc)
    pub fn push(&mut self, newSurvivor: &DefTableBuilder, fc: i32) {
      // java this.mAfterLastSurvivor = this.mLastSurvivor;
      self.mAfterLastSurvivor = self.mLastSurvivor;
      // java this.mLastSurvivor = this.mSurvivor;
      self.mLastSurvivor = self.mSurvivor;
      // java this.mSurvivor = newSurvivor;
      self.mSurvivor = newSurvivor;
      // java this.mLastSurvivorFc = this.mSurvivorFc;
      self.mLastSurvivorFc = self.mSurvivorFc;
      // java this.mSurvivorFc = fc;
      self.mSurvivorFc = fc;
    }

    // java private void doFixLastSurvivor()
    pub fn doFixLastSurvivor(&mut self) {
      // java int rowEndCp = this.mDocument.getTextRope().fc2cp(this.mLastSurvivorFc);
      let mut rowEndCp: i32 = self.mDocument.getTextRope().fc2cp(self.mLastSurvivorFc);
      // java this.mTransport.setDefTable(this.mDocument, rowEndCp, this.mLastSurvivor);
      self.mTransport.setDefTable(self.mDocument, rowEndCp, self.mLastSurvivor);
    }

    // java private boolean fixVertMergeBefore(DefTableBuilder defTableBefore, DefTable defTableAfter)
    pub fn fixVertMergeBefore(&mut self, defTableBefore: &DefTableBuilder, defTableAfter: &DefTable) -> bool {
      // java boolean change = false;
      let mut change: bool = false;
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = defTableBefore.getCellCount();
      // java for(int size = defTableBefore.getCellCount(); i < size; ++i)
      while i < size {
        // java TableCellBuilder tc = defTableBefore.getTableCellBuilder(i);
        let mut tc: TableCellBuilder = defTableBefore.getTableCellBuilder(i);
        // java if (3 == tc.getVertMerge() && null == TableTool.getNextMergeCell(defTableBefore, i, defTableAfter))
        if 3 == tc.getVertMerge() && TableTool::getNextMergeCell__deftable_int_deftable(&defTableBefore, i, Some(&defTableAfter)).is_none() {
          // java tc.setVertMerge(0);
          tc.setVertMerge(0);
          // java change = true;
          change = true;
        }
      }
      // java return change;
      return change;
    }

    // java private boolean fixVertMergeAfter(DefTableBuilder defTableAfter, DefTable defTableBefore, DefTable defTableAfterAfter)
    pub fn fixVertMergeAfter(&mut self, defTableAfter: &DefTableBuilder, defTableBefore: &DefTable, defTableAfterAfter: &DefTable) -> bool {
      // java boolean change = false;
      let mut change: bool = false;
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = defTableAfter.getCellCount();
      // java for(int size = defTableAfter.getCellCount(); i < size; ++i)
      while i < size {
        // java TableCellBuilder tc = defTableAfter.getTableCellBuilder(i);
        let mut tc: TableCellBuilder = defTableAfter.getTableCellBuilder(i);
        // java if (1 == tc.getVertMerge())
        if 1 == tc.getVertMerge() {
          // java if (null == TableTool.getPrevMergeCell(defTableAfter, i, defTableBefore))
          if TableTool::getPrevMergeCell__deftable_int_deftable(&defTableAfter, i, Some(&defTableBefore)).is_none() {
            // java if (null == TableTool.getNextMergeCell(defTableAfter, i, defTableAfterAfter))
            if TableTool::getNextMergeCell__deftable_int_deftable(&defTableAfter, i, Some(&defTableAfterAfter)).is_none() {
              // java tc.setVertMerge(0);
              tc.setVertMerge(0);
            }
            // java else
            else {
              // java tc.setVertMerge(3);
              tc.setVertMerge(3);
            }
            // java change = true;
            change = true;
          }
        }
        // java else if (3 == tc.getVertMerge() && null == TableTool.getNextMergeCell(defTableAfter, i, defTableAfterAfter))
        else if 3 == tc.getVertMerge() && TableTool::getNextMergeCell__deftable_int_deftable(&defTableAfter, i, Some(&defTableAfterAfter)).is_none() {
          // java tc.setVertMerge(0);
          tc.setVertMerge(0);
          // java change = true;
          change = true;
        }
      }
      // java return change;
      return change;
    }

}
