// Java: import cn.wps.collections.Node
use util::collections::Node::*;
// Java: import cn.wps.collections.PositionTreeExtension
use util::collections::PositionTreeExtension::*;
// Java: import cn.wps.moffice.drawing.IPos
use drawing::cn::wps::moffice::drawing::IPos::*;
// Java: import cn.wps.moffice.drawing.graphics.SHD
use drawing::cn::wps::moffice::drawing::graphics::SHD::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.writer.data.DefaultValue
use crate::cn::wps::moffice::writer::data::DefaultValue::*;
// Java: import cn.wps.moffice.writer.data.table.CellInfo
use crate::cn::wps::moffice::writer::data::table::CellInfo::*;
// Java: import cn.wps.moffice.writer.data.table.Data
use crate::cn::wps::moffice::writer::data::table::Data::*;
// Java: import cn.wps.moffice.writer.data.table.ITableManager
use crate::cn::wps::moffice::writer::data::table::ITableManager::*;
// Java: import cn.wps.moffice.writer.data.table.KTableManager
use crate::cn::wps::moffice::writer::data::table::KTableManager::*;
// Java: import cn.wps.moffice.writer.data.table.RowInfo
use crate::cn::wps::moffice::writer::data::table::RowInfo::*;
// Java: import cn.wps.moffice.writer.data.table.TableInfo
use crate::cn::wps::moffice::writer::data::table::TableInfo::*;
// Java: import cn.wps.moffice.writer.data.table.TableJCData
use crate::cn::wps::moffice::writer::data::table::TableJCData::*;
// Java: import cn.wps.moffice.writer.data.table.TableTool
use crate::cn::wps::moffice::writer::data::table::TableTool::*;
// Java: import cn.wps.moffice.writer.data.table.grid.GridCols
use crate::cn::wps::moffice::writer::data::table::grid::GridCols::*;
// Java: import cn.wps.moffice.writer.data.table.grid.GridColsImpl
use crate::cn::wps::moffice::writer::data::table::grid::GridColsImpl::*;
// Java: import cn.wps.moffice.writer.data.table.grid.GridResizeRequest
use crate::cn::wps::moffice::writer::data::table::grid::GridResizeRequest::*;
// Java: import cn.wps.moffice.writer.data.value.BRC
use crate::cn::wps::moffice::writer::data::value::BRC::*;
// Java: import cn.wps.moffice.writer.data.value.TableBorders
use crate::cn::wps::moffice::writer::data::value::TableBorders::*;
// Java: import cn.wps.moffice.writer.data.value.TableLook
use crate::cn::wps::moffice::writer::data::value::TableLook::*;
// Java: import cn.wps.moffice.writer.data.table.tree.TableNode
use crate::cn::wps::moffice::writer::data::table::tree::TableNode::*;
// Java: import cn.wps.moffice.writer.data.table.tree.CellInfoNode
use crate::cn::wps::moffice::writer::data::table::tree::CellInfoNode::*;
// Java: import cn.wps.moffice.writer.data.table.tree.RowInfoNode
use crate::cn::wps::moffice::writer::data::table::tree::RowInfoNode::*;

// java public class TableInfoNode extends TableNode implements TableInfo
#[derive(Default)]
pub struct TableInfoNode {
    // java TableNode super_
    pub(crate) super_: TableNode,
    // java private CellInfoNode mParent;
    mParent: Option<CellInfoNode>,
    // java private int mLevel;
    mLevel: i32,
    // java private Data.TableData mData;
    mData: Option<crate::cn::wps::moffice::writer::data::table::Data::TableData>,
    // java private boolean mValid;
    mValid: bool,
    // java private boolean mIsComplete = true;
    mIsComplete: bool,
}

// java public interface TableInfo
impl TableInfo for TableInfoNode {
    // java public int getLevel()
    fn getLevel(&self) -> i32 {
      // java return this.mLevel;
      return self.mLevel;
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

    // java  int size()
    fn size(&self) -> i32 {
    }

    // java public RowInfo getRowByIndex(int index)
    fn getRowByIndex(&self, index: i32) -> RowInfo {
      // java return (RowInfo)this.mTree.get(index);
      return (self.mTree.get(index)).downcast_ref::<RowInfo>().unwrap();
    }

    // java public RowInfo getRowByCP(int cp)
    fn getRowByCP(&self, cp: i32) -> RowInfo {
      // java return (RowInfo)this.getNodeByCP(cp);
      return (self.super_.getNodeByCP(cp)).downcast_ref::<RowInfo>().unwrap();
    }

    // java public CellInfo getParentCell()
    fn getParentCell(&self) -> CellInfo {
      // java return this.getParentNode();
      return self.getParentNode();
    }

    // java public synchronized TableJCData getJC()
    fn getJC(&self) -> TableJCData {
      // java Data.TableData data = this.getData();
      let mut data: crate::cn::wps::moffice::writer::data::table::Data::TableData = self.getData();
      // java if (data.mTableJCData == null)
      if data.mTableJCData.is_none() {
        // java data.mTableJCData = this.getTableTool().calcJC(this);
        data.mTableJCData = self.super_.getTableTool().calcJC(self);
      }
      // java return data.mTableJCData;
      return data.mTableJCData;
    }

    // java public GridCols getGrid()
    fn getGrid(&self) -> Box<dyn GridCols> {
      // java Data.TableData data = this.getData();
      let mut data: crate::cn::wps::moffice::writer::data::table::Data::TableData = self.getData();
      // java if (data.mGridCols == null)
      if data.mGridCols.is_none() {
        // java GridColsImpl gridCols = new GridColsImpl(this.mCache, this);
        let mut gridCols: GridColsImpl = GridColsImpl::new(self.mCache, self);
        // java gridCols.valid();
        gridCols.valid();
        // java data.mGridCols = gridCols;
        data.mGridCols = gridCols;
      }
      // java return data.mGridCols;
      return data.mGridCols;
    }

    // java public void requestAutoFitDelay()
    fn requestAutoFitDelay(&self) {
      // java GridResizeRequest request = this.mCache.getGridResizeRequest();
      let mut request: Box<dyn GridResizeRequest> = self.mCache.getGridResizeRequest();
      // java if (request != null)
      if request.is_some() {
        // java request.requestWithoutReTypo(this.getRange(), this.getDocument());
        request.requestWithoutReTypo(self.super_.getRange(), self.super_.getDocument());
      }
    }

    // java public IPos getPos()
    fn getPos(&mut self) -> Box<dyn IPos> {
      // java this.valid();
      self.valid();
      // java return this.getData().getPos();
      return self.getData().getPos();
    }

    // java public SHD getSHD()
    fn getSHD(&mut self) -> SHD {
      // java this.valid();
      self.valid();
      // java return this.getData().getSHD();
      return self.getData().getSHD();
    }

    // java public TableLook getTableLook()
    fn getTableLook(&mut self) -> TableLook {
      // java this.valid();
      self.valid();
      // java return this.getData().getTableLook();
      return self.getData().getTableLook();
    }

    // java public TableBorders getTableBorders()
    fn getTableBorders(&mut self) -> TableBorders {
      // java this.valid();
      self.valid();
      // java return this.getData().getTableBorders();
      return self.getData().getTableBorders();
    }

    // java public int getStyleId()
    fn getStyleId(&mut self) -> i32 {
      // java this.valid();
      self.valid();
      // java return this.getData().getStyleId();
      return self.getData().getStyleId();
    }

    // java public BRC getBrcTop()
    fn getBrcTop(&mut self) -> BRC {
      // java this.valid();
      self.valid();
      // java return this.getData().getBrcTop();
      return self.getData().getBrcTop();
    }

    // java public BRC getBrcLeft()
    fn getBrcLeft(&mut self) -> BRC {
      // java this.valid();
      self.valid();
      // java return this.getData().getBrcLeft();
      return self.getData().getBrcLeft();
    }

    // java public BRC getBrcBottom()
    fn getBrcBottom(&mut self) -> BRC {
      // java this.valid();
      self.valid();
      // java return this.getData().getBrcBottom();
      return self.getData().getBrcBottom();
    }

    // java public BRC getBrcRight()
    fn getBrcRight(&mut self) -> BRC {
      // java this.valid();
      self.valid();
      // java return this.getData().getBrcRight();
      return self.getData().getBrcRight();
    }

    // java public boolean isRTL()
    fn isRTL(&mut self) -> bool {
      // java this.valid();
      self.valid();
      // java return this.getData().isRTL();
      return self.getData().isRTL();
    }

    // java public boolean isComplete()
    fn isComplete(&self) -> bool {
      // java return this.mIsComplete;
      return self.mIsComplete;
    }

    // java public boolean isTableFit()
    fn isTableFit(&self) -> bool {
      // java TableInfo rootTable = this.mCache.getRootTable(this.getStartCP());
      let mut rootTable: TableInfo = self.mCache.getRootTable(self.super_.getStartCP());
      // java return rootTable.getTableProperty().getBoolean(302, DefaultValue.table_fit);
      return rootTable.getTableProperty().getBoolean(302, DefaultValue::table_fit);
    }

    // java public PropertySet getTableProperty()
    fn getTableProperty(&self) -> PropertySet {
      // java return ((RowInfo)this.getFirstChild()).getFinalProperty();
      return (self.super_.getFirstChild()).downcast_ref::<RowInfo>().unwrap().getFinalProperty();
    }

    // java public Data.TableData getData()
    fn getData(&mut self) -> Data::TableData {
      // java if (this.mData == null)
      if self.mData.is_none() {
        // java this.mData = new Data.TableData();
        self.mData = TableData::new();
      }
      // java return this.mData;
      return self.mData;
    }

    // java public void invalid()
    fn invalid(&mut self) {
      // java this.invalid(true);
      self.invalid__boolean(true);
    }

}

// java public class TableInfoNode extends TableNode implements TableInfo
impl TableInfoNode {
    // java   TableInfoNode()
    pub fn TableInfoNode() -> Self {
      // java super(true);
      // java this.mGeneration = 1;
      Self { super_: TableNode::new(true), mParent: Default::default(), mLevel: Default::default(), mData: Default::default(), mValid: Default::default(), mIsComplete: Default::default() }
    }

    // java  TableNode newSubNode()
    pub fn newSubNode(&mut self) -> TableNode {
      // java return new RowInfoNode();
      return RowInfoNode::new();
    }

    // java  CellInfoNode getParentNode()
    pub fn getParentNode(&self) -> CellInfoNode {
      // java return this.mParent;
      return self.mParent;
    }

    // java  void setParentNode(TableNode parentNode)
    pub fn setParentNode(&mut self, parentNode: &TableNode) {
      // java this.mParent = (CellInfoNode)parentNode;
      self.mParent = (parentNode).downcast_ref::<CellInfoNode>().unwrap();
    }

    // java  ITableManager.IRowEntry init(ITableManager.IRowEntry rowEntry, int level, boolean scanNext, int maxCP)
    pub fn init(&mut self, rowEntry: &mut ITableManager::IRowEntry, level: i32, scanNext: bool, maxCP: i32) -> ITableManager::IRowEntry {
      // java this.mLevel = level;
      self.mLevel = level;
      // java int oldTableEnd = rowEntry.getStart();
      let mut oldTableEnd: i32 = rowEntry.getStart();
      // java int tableEnd = oldTableEnd;
      let mut tableEnd: i32 = oldTableEnd;
      // java ITableManager.IRowEntry lastRowEntry = null;
      let mut lastRowEntry: ITableManager::IRowEntry = ITableManager::IRowEntry::default();
      // java RowInfoNode lastRow = (RowInfoNode)this.mTree.rightmost();
      let mut lastRow: RowInfoNode = (self.mTree.rightmost()).downcast_ref::<RowInfoNode>().unwrap();
      // java int lastRowStart = lastRow.isHead() ? oldTableEnd : lastRow.getStartCP();
      let mut lastRowStart: i32 = (if lastRow.isHead() { oldTableEnd } else { lastRow.getStartCP() });
      // java do (rowEntry.isValid())
      loop {
          // java int start = rowEntry.getStart();
          let mut start: i32 = rowEntry.getStart();
          // java int end = rowEntry.getEnd();
          let mut end: i32 = rowEntry.getEnd();
          // java if (!lastRow.isHead() && (start != tableEnd || !TableTool.isInTheSameTable(this.getDocument(), lastRowStart, tableEnd, start, end)))
          if !lastRow.isHead() && start != tableEnd || !TableTool::isInTheSameTable(&self.super_.getDocument(), lastRowStart, tableEnd, start, end) {
            // java break;
            break;
          }
          // java RowInfoNode newRow = (RowInfoNode)this.createSubNode();
          let mut newRow: RowInfoNode = (self.super_.createSubNode()).downcast_ref::<RowInfoNode>().unwrap();
          // java newRow.setOffset(start - lastRowStart);
          newRow.setOffset(start - lastRowStart);
          // java newRow.init(rowEntry);
          newRow.init(rowEntry);
          // java this.mTree.insertRight(lastRow, newRow);
          self.mTree.insertRight(lastRow, newRow);
          // java tableEnd = end;
          tableEnd = end;
          // java lastRow = newRow;
          lastRow = newRow;
          // java lastRowStart = start;
          lastRowStart = start;
          // java if (scanNext && lastRowEntry != null && lastRowEntry != rowEntry)
          if scanNext && lastRowEntry.is_some() && lastRowEntry != rowEntry {
            // java lastRowEntry.recycle();
            lastRowEntry.recycle();
          }
          // java lastRowEntry = rowEntry;
          lastRowEntry = rowEntry;
          // java if (!scanNext)
          if !scanNext {
            // java rowEntry = rowEntry.getNext();
            rowEntry = rowEntry.getNext();
          }
          // java else
          else {
            // java if (end >= maxCP || !this.getTableManager().isInTable(end, level))
            if end >= maxCP || !self.super_.getTableManager().isInTable(end, level) {
              // java break;
              break;
            }
            // java rowEntry = this.getTableManager().createRow(end, level);
            rowEntry = self.super_.getTableManager().createRow(end, level);
          }
      }
      // java if (this.mLength != 0 && lastRowEntry != null)
      if self.mLength != 0 && lastRowEntry.is_some() {
        // java this.invalid();
        self.invalid();
      }
      // java this.mLength += tableEnd - oldTableEnd;
      self.mLength = self.mLength + tableEnd - oldTableEnd;
      // java return lastRowEntry;
      return lastRowEntry;
    }

    // java public void invalid(boolean invalidGrid)
    pub fn invalid__boolean(&mut self, invalidGrid: bool) {
      // java this.mValid = false;
      self.mValid = false;
      // java this.getData().mTableJCData = null;
      self.getData().mTableJCData = None;
      // java if (invalidGrid && this.getData().mGridCols != null)
      if invalidGrid && self.getData().mGridCols.is_some() {
        // java this.getData().mGridCols.invalid();
        self.getData().mGridCols.invalid();
      }
      // java ++this.mGeneration;
      { let _tmp = self.mGeneration; self.mGeneration += 1; _tmp };
      // java if (this.mGeneration < 0)
      if self.mGeneration < 0 {
        // java this.mGeneration = 1;
        self.mGeneration = 1;
        let mut row: TableNode = (self.mTree.leftmost()).downcast_ref::<TableNode>().unwrap();
        // java for(TableNode row = (TableNode)this.mTree.leftmost(); !row.isHead() row = (TableNode)row.getNext())
        while !row.isHead() {
          // java row.mGeneration = 0;
          row.mGeneration = 0;
          let mut cell: TableNode = (row.mTree.leftmost()).downcast_ref::<TableNode>().unwrap();
          // java for(TableNode cell = (TableNode)row.mTree.leftmost(); !cell.isHead() cell = (TableNode)cell.getNext())
          while !cell.isHead() {
            // java cell.mGeneration = 0;
            cell.mGeneration = 0;
          }
        }
      }
    }

    // java synchronized void valid()
    pub fn valid(&mut self) {
      // java if (!this.mValid)
      if !self.mValid {
        // java this.mValid = true;
        self.mValid = true;
        // java TableTool tableTool = this.getTableTool();
        let mut tableTool: TableTool = self.super_.getTableTool();
        // java tableTool.calcTableStyles(this);
        tableTool.calcTableStyles(self);
      }
    }

    // java synchronized void validRow(RowInfoNode rowInfo)
    pub fn validRow(&mut self, rowInfo: &mut RowInfoNode) {
      // java rowInfo.validInTableLock();
      rowInfo.validInTableLock();
    }

    // java synchronized void validCell(CellInfoNode cellInfo)
    pub fn validCell(&mut self, cellInfo: &mut CellInfoNode) {
      // java cellInfo.validInTableLock();
      cellInfo.validInTableLock();
    }

    // java  void setComplete(boolean isComplete)
    pub fn setComplete(&mut self, isComplete: bool) {
      // java if (isComplete && !this.mIsComplete)
      if isComplete && !self.mIsComplete {
      }
      // java this.mIsComplete = isComplete;
      self.mIsComplete = isComplete;
    }

    // java  boolean removeNodeInRange(int start, int end, int[] offsetBuf)
    pub fn removeNodeInRange(&mut self, start: i32, end: i32, offsetBuf: &Vec<i32>) -> bool {
      // java boolean nodeChanged = super.removeNodeInRange(start, end, offsetBuf);
      let mut nodeChanged: bool = self.super_.removeNodeInRange(start, end, &offsetBuf);
      // java if (nodeChanged)
      if nodeChanged {
        // java this.invalid();
        self.invalid();
      }
      // java return nodeChanged;
      return nodeChanged;
    }

    // java  void valid(int start, int end)
    pub fn valid__int_int(&mut self, start: i32, end: i32) {
      // java int position = this.getStartCP();
      let mut position: i32 = self.super_.getStartCP();
      // java TableNode lastNode = this.getNodeByOffset(start - 1, (int[])null);
      let mut lastNode: TableNode = self.super_.getNodeByOffset(start - 1, Default::default());
      // java int lastOffset = start - lastNode.mLength;
      let mut lastOffset: i32 = start - lastNode.mLength;
      // java do (start < end)
      loop {
          // java ITableManager.IRowEntry rowEntry = this.getTableManager().createRow(position + start, this.mLevel);
          let mut rowEntry: ITableManager::IRowEntry = self.super_.getTableManager().createRow(position + start, self.mLevel);
          // java RowInfoNode newNode = (RowInfoNode)this.createSubNode();
          let mut newNode: RowInfoNode = (self.super_.createSubNode()).downcast_ref::<RowInfoNode>().unwrap();
          // java newNode.setOffset(start - lastOffset);
          newNode.setOffset(start - lastOffset);
          // java newNode.init(rowEntry);
          newNode.init(rowEntry);
          // java this.mTree.insertRight(lastNode, newNode);
          self.mTree.insertRight(lastNode, newNode);
          // java rowEntry.recycle();
          rowEntry.recycle();
          // java lastNode = newNode;
          lastNode = newNode;
          // java lastOffset = start;
          lastOffset = start;
          // java start += newNode.mLength;
          start = start + newNode.mLength;
      }
    }

    // java  TableInfoNode tryAppend()
    pub fn tryAppend(&mut self) -> TableInfoNode {
      // java KTableManager tableManager = this.getTableManager();
      let mut tableManager: KTableManager = self.super_.getTableManager();
      // java int tableEnd = this.getEndCP();
      let mut tableEnd: i32 = self.super_.getEndCP();
      // java if (tableManager.isInTable(tableEnd, this.mLevel))
      if tableManager.isInTable(tableEnd, self.mLevel) {
        // java TableInfoNode next = (TableInfoNode)this.getNext();
        let mut next: TableInfoNode = (self.getNext()).downcast_ref::<TableInfoNode>().unwrap();
        // java int nextTableStart = next.isHead() ? Integer.MAX_VALUE : next.getStartCP();
        let mut nextTableStart: i32 = (if next.isHead() { Integer::MAX_VALUE } else { next.getStartCP() });
        // java if (tableEnd < nextTableStart)
        if tableEnd < nextTableStart {
          // java ITableManager.IRowEntry rowEntry = tableManager.createRow(tableEnd, this.mLevel);
          let mut rowEntry: ITableManager::IRowEntry = tableManager.createRow(tableEnd, self.mLevel);
          // java rowEntry = this.init(rowEntry, this.mLevel, true, nextTableStart);
          rowEntry = self.init(&mut rowEntry, self.mLevel, true, nextTableStart);
          // java if (null != rowEntry)
          if rowEntry.is_some() {
            // java rowEntry.recycle();
            rowEntry.recycle();
          }
        }
        // java if (!next.isHead() && this.getEndCP() == nextTableStart && next.tryMerge(this))
        if !next.isHead() && self.super_.getEndCP() == nextTableStart && next.tryMerge(self) {
          // java return next;
          return next;
        }
      }
      // java return this;
      return self;
    }

    // java  boolean tryMerge(TableInfoNode prev)
    pub fn tryMerge(&mut self, prev: &mut TableInfoNode) -> bool {
      // java int position = this.getStartCP();
      let mut position: i32 = self.super_.getStartCP();
      // java TableNode rightmost = (TableNode)prev.mTree.rightmost();
      let mut rightmost: TableNode = (prev.mTree.rightmost()).downcast_ref::<TableNode>().unwrap();
      // java TableNode leftmost = (TableNode)this.mTree.leftmost();
      let mut leftmost: TableNode = (self.mTree.leftmost()).downcast_ref::<TableNode>().unwrap();
      // java if (!TableTool.isInTheSameTable(this.getDocument(), position - rightmost.mLength, position, position, position + leftmost.mLength))
      if !TableTool::isInTheSameTable(&self.super_.getDocument(), position - rightmost.mLength, position, position, position + leftmost.mLength) {
        // java return false;
        return false;
      }
      // java else
      else {
        // java this.mParent.mTree.erase(prev);
        self.mParent.mTree.erase(prev);
        // java this.mTree.moveRight(this, -prev.mLength);
        self.mTree.moveRight(self, -prev.mLength);
        // java this.mLength += prev.mLength;
        self.mLength = self.mLength + prev.mLength;
        // java this.mTree.insertSpaceBefore(leftmost, prev.mLength);
        self.mTree.insertSpaceBefore(leftmost, prev.mLength);
        // java PositionTreeExtension.toList(prev.mTree, 0);
        PositionTreeExtension::toList(&prev.mTree, 0);
        // java while (!rightmost.isHead())
        while !rightmost.isHead() {
          // java Node left = rightmost.getLeft();
          let mut left: Box<dyn Node> = rightmost.getLeft();
          // java rightmost.setParentNode(this);
          rightmost.setParentNode(self);
          // java this.mTree.insertLeft(leftmost, rightmost);
          self.mTree.insertLeft(leftmost, rightmost);
          // java leftmost = rightmost;
          leftmost = rightmost;
          // java rightmost = (TableNode)left;
          rightmost = (left).downcast_ref::<TableNode>().unwrap();
        }
        // java this.mGeneration = Math.max(this.mGeneration, prev.mGeneration);
        self.mGeneration = Math::max(self.mGeneration, prev.mGeneration);
        // java this.invalid();
        self.invalid();
        // java return true;
        return true;
      }
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

impl std::ops::Deref for TableInfoNode {
    type Target = TableNode;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TableInfoNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
