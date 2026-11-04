// Java: import cn.wps.collections.Node
use util::collections::Node::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.core.KStyles
use crate::cn::wps::moffice::writer::core::KStyles::*;
// Java: import cn.wps.moffice.writer.data.PropertyUtil
use crate::cn::wps::moffice::writer::data::PropertyUtil::*;
// Java: import cn.wps.moffice.writer.data.table.CellInfo
use crate::cn::wps::moffice::writer::data::table::CellInfo::*;
// Java: import cn.wps.moffice.writer.data.table.Data
use crate::cn::wps::moffice::writer::data::table::Data::*;
// Java: import cn.wps.moffice.writer.data.table.ITableManager
use crate::cn::wps::moffice::writer::data::table::ITableManager::*;
// Java: import cn.wps.moffice.writer.data.table.RowInfo
use crate::cn::wps::moffice::writer::data::table::RowInfo::*;
// Java: import cn.wps.moffice.writer.data.table.TableInfo
use crate::cn::wps::moffice::writer::data::table::TableInfo::*;
// Java: import cn.wps.moffice.writer.data.table.TableTool
use crate::cn::wps::moffice::writer::data::table::TableTool::*;
// Java: import cn.wps.moffice.writer.data.table.tree.TableNode
use crate::cn::wps::moffice::writer::data::table::tree::TableNode::*;
// Java: import cn.wps.moffice.writer.data.table.tree.TableInfoNode
use crate::cn::wps::moffice::writer::data::table::tree::TableInfoNode::*;
// Java: import cn.wps.moffice.writer.data.table.tree.CellInfoNode
use crate::cn::wps::moffice::writer::data::table::tree::CellInfoNode::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;

// java public class RowInfoNode extends TableNode implements RowInfo
#[derive(Default)]
pub struct RowInfoNode {
    // java TableNode super_
    pub(crate) super_: TableNode,
    // java private TableInfoNode mParent;
    mParent: Option<TableInfoNode>,
    // java private Data.RowData mData;
    mData: Option<crate::cn::wps::moffice::writer::data::table::Data::RowData>,
}

// java public interface RowInfo
impl RowInfo for RowInfoNode {
    // java public TableInfo getParent()
    fn getParent(&self) -> &dyn TableInfo {
      // java return this.getParentNode();
      return self.getParentNode();
    }

    // java  int getIndex()
    fn getIndex(&self) -> i32 {
    }

    // java  long getRange()
    fn getRange(&self) -> i64 {
    }

    // java  int getStartCP()
    fn getStartCP(&self) -> i32 {
    }

    // java  int getEndCP()
    fn getEndCP(&self) -> i32 {
    }

    // java public RowInfo getNextRow()
    fn getNextRow(&self) -> RowInfo {
      // java return (RowInfo)this.getNextTableNode();
      return (self.super_.getNextTableNode()).downcast_ref::<RowInfo>().unwrap();
    }

    // java public RowInfo getPrevRow()
    fn getPrevRow(&self) -> RowInfo {
      // java return (RowInfo)this.getPrevTableNode();
      return (self.super_.getPrevTableNode()).downcast_ref::<RowInfo>().unwrap();
    }

    // java public int getRowHeight()
    fn getRowHeight(&mut self) -> i32 {
      // java this.valid();
      self.valid();
      // java return this.getData().getRowHeight();
      return self.getData().getRowHeight();
    }

    // java public int getRgdxaCount()
    fn getRgdxaCount(&mut self) -> i32 {
      // java this.valid();
      self.valid();
      // java return this.getData().getRgdxaCount();
      return self.getData().getRgdxaCount();
    }

    // java public int getRgdxa(int index)
    fn getRgdxa(&mut self, index: i32) -> i32 {
      // java this.valid();
      self.valid();
      // java return this.getData().getRgdxa(index);
      return self.getData().getRgdxa(index);
    }

    // java  int size()
    fn size(&self) -> i32 {
    }

    // java public int getCellWidth(int index)
    fn getCellWidth(&mut self, index: i32) -> i32 {
      // java this.valid();
      self.valid();
      // java this.delayInitCellWidths();
      self.delayInitCellWidths();
      // java return this.getData().getCellWidth(index);
      return self.getData().getCellWidth(index);
    }

    // java public CellInfo getCellByIndex(int index)
    fn getCellByIndex(&self, index: i32) -> CellInfo {
      // java return (CellInfo)this.mTree.get(index);
      return (self.mTree.get(index)).downcast_ref::<CellInfo>().unwrap();
    }

    // java public CellInfo getCellByCP(int cp)
    fn getCellByCP(&self, cp: i32) -> CellInfo {
      // java return (CellInfo)this.getNodeByCP(cp);
      return (self.super_.getNodeByCP(cp)).downcast_ref::<CellInfo>().unwrap();
    }

    // java public boolean cantSplit()
    fn cantSplit(&mut self) -> bool {
      // java this.valid();
      self.valid();
      // java return this.getData().cantSplit();
      return self.getData().cantSplit();
    }

    // java public PropertySet getRowProperty()
    fn getRowProperty(&self) -> PropertySet {
      // java return this.getOriginalProperty();
      return self.getOriginalProperty();
    }

    // java public boolean isRevisionDelete()
    fn isRevisionDelete(&mut self) -> bool {
      // java this.valid();
      self.valid();
      // java return this.getData().isRevisionDelete();
      return self.getData().isRevisionDelete();
    }

    // java public boolean isRevisionInsert()
    fn isRevisionInsert(&mut self) -> bool {
      // java this.valid();
      self.valid();
      // java return this.getData().isRevisionInsert();
      return self.getData().isRevisionInsert();
    }

    // java public boolean isFormatChange()
    fn isFormatChange(&mut self) -> bool {
      // java this.valid();
      self.valid();
      // java return this.getData().isFormatChange();
      return self.getData().isFormatChange();
    }

    // java public PropertySet getOriginalProperty()
    fn getOriginalProperty(&self) -> PropertySet {
      // java return this.getDocument().getPapxTable().seek(this.getEndCP() - 1).getProperty();
      return self.super_.getDocument().getPapxTable().seek(self.super_.getEndCP() - 1).getProperty();
    }

    // java public PropertySet getFinalProperty()
    fn getFinalProperty(&self) -> PropertySet {
      // java PropertySet prop = this.getOriginalProperty();
      let mut prop: PropertySet = self.getOriginalProperty();
      // java Object value = prop.getValue(363);
      let mut value: Box<dyn Object> = prop.getValue(363);
      // java if (null != value)
      if value.is_some() {
        // java KStyles styles = this.getDocument().getTextDocument().getStyles();
        let mut styles: KStyles = self.super_.getDocument().getTextDocument().getStyles();
        // java return PropertyUtil.getFinalTableFormat(styles, prop);
        return PropertyUtil::getFinalTableFormat(&styles, &prop);
      }
      // java else
      else {
        // java return prop;
        return prop;
      }
    }

    // java public Data.RowData getData()
    fn getData(&mut self) -> &mut crate::cn::wps::moffice::writer::data::table::Data::RowData {
      // java if (this.mData == null)
      if self.mData.is_none() {
        // java this.mData = new Data.RowData();
        self.mData = RowData::new();
      }
      // java return this.mData;
      return self.mData.as_mut().unwrap();
    }

}

// java public class RowInfoNode extends TableNode implements RowInfo
impl RowInfoNode {
    // java   RowInfoNode()
    pub fn RowInfoNode() -> Self {
      // java super(true);
      Self { super_: TableNode::new(true), mParent: Default::default(), mData: Default::default() }
    }

    // java  TableNode newSubNode()
    pub fn newSubNode(&mut self) -> TableNode {
      // java return new CellInfoNode(false);
      return CellInfoNode::new(false);
    }

    // java  TableInfoNode getParentNode()
    pub fn getParentNode(&self) -> TableInfoNode {
      // java return this.mParent;
      return self.mParent;
    }

    // java  void setParentNode(TableNode parentNode)
    pub fn setParentNode(&mut self, parentNode: &TableNode) {
      // java this.mParent = (TableInfoNode)parentNode;
      self.mParent = (parentNode).downcast_ref::<TableInfoNode>().unwrap();
    }

    // java  void init(ITableManager.IRowEntry rowEntry)
    pub fn init(&mut self, rowEntry: &ITableManager::IRowEntry) {
      // java this.mLength = rowEntry.getEnd() - rowEntry.getStart();
      self.mLength = rowEntry.getEnd() - rowEntry.getStart();
      // java Node lastCell = this.mTree.getHead();
      let mut lastCell: Box<dyn Node> = self.mTree.getHead();
      // java int lastCellStart = rowEntry.getStart();
      let mut lastCellStart: i32 = rowEntry.getStart();
      let mut cellEntry: ITableManager::ICellEntry = rowEntry.getFirstChildCell();
      // java for(ITableManager.ICellEntry cellEntry = rowEntry.getFirstChildCell(); cellEntry.isValid() cellEntry = cellEntry.getNext())
      while cellEntry.isValid() {
        // java int cellStart = cellEntry.getStart();
        let mut cellStart: i32 = cellEntry.getStart();
        // java CellInfoNode newCell = (CellInfoNode)this.createSubNode();
        let mut newCell: CellInfoNode = (self.super_.createSubNode()).downcast_ref::<CellInfoNode>().unwrap();
        // java newCell.setOffset(cellStart - lastCellStart);
        newCell.setOffset(cellStart - lastCellStart);
        // java newCell.init(cellEntry);
        newCell.init(cellEntry);
        // java this.mTree.insertRight(lastCell, newCell);
        self.mTree.insertRight(lastCell, newCell);
        // java lastCell = newCell;
        lastCell = newCell;
        // java lastCellStart = cellStart;
        lastCellStart = cellStart;
      }
    }

    // java  void valid()
    pub fn valid(&self) {
      // java TableInfoNode tableInfo = this.getParentNode();
      let mut tableInfo: TableInfoNode = self.getParentNode();
      // java if (tableInfo != null)
      if tableInfo.is_some() {
        // java tableInfo.validRow(this);
        tableInfo.validRow(self);
      }
    }

    // java  void validInTableLock()
    pub fn validInTableLock(&mut self) {
      // java if (this.mGeneration < this.mParent.mGeneration)
      if self.mGeneration < self.mParent.mGeneration {
        // java this.mGeneration = this.mParent.mGeneration;
        self.mGeneration = self.mParent.mGeneration;
        // java TableTool tableTool = this.getTableTool();
        let mut tableTool: TableTool = self.super_.getTableTool();
        // java tableTool.calcRowStyle(this);
        tableTool.calcRowStyle(self);
      }
    }

    // java private void delayInitCellWidths()
    pub fn delayInitCellWidths(&self) {
      // java Data.RowData data = this.getData();
      let mut data: &mut crate::cn::wps::moffice::writer::data::table::Data::RowData = self.getData();
      // java if (data.mCellWidths == null)
      if data.mCellWidths.is_none() {
        // java data.mCellWidths = TableTool.calcCellWidths(this, this.getFinalProperty(), this.getParent().getJC(), false);
        data.mCellWidths = TableTool::calcCellWidths(&self, &self.getFinalProperty(), &self.getParent().getJC(), false);
      }
    }

    // java  void valid(int start, int end)
    pub fn valid__int_int(&mut self, start: i32, end: i32) {
      // java throw new UnsupportedOperationException();
      panic!("{}", UnsupportedOperationException::new());
    }

    // java protected void close()
    pub fn close(&mut self) {
      // java if (this.nodeCount() > 0)
      if self.super_.nodeCount() > 0 {
        let mut subNode: TableNode = (self.mTree.leftmost()).downcast_ref::<TableNode>().unwrap();
        // java for(TableNode subNode = (TableNode)this.mTree.leftmost(); !subNode.isHead() subNode = (TableNode)subNode.getNext())
        while !subNode.isHead() {
          // java subNode.close();
          subNode.close();
        }
      }
    }

}

impl std::ops::Deref for RowInfoNode {
    type Target = TableNode;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for RowInfoNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
