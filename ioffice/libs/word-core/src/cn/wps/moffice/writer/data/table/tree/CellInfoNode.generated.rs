// Java: import cn.wps.collections.Node
use util::collections::Node::*;
// Java: import cn.wps.moffice.drawing.graphics.SHD
use drawing::cn::wps::moffice::drawing::graphics::SHD::*;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::*;
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
// Java: import cn.wps.moffice.writer.data.table.TableTool
use crate::cn::wps::moffice::writer::data::table::TableTool::*;
// Java: import cn.wps.moffice.writer.data.value.BRC
use crate::cn::wps::moffice::writer::data::value::BRC::*;
// Java: import cn.wps.moffice.writer.data.table.tree.TableNode
use crate::cn::wps::moffice::writer::data::table::tree::TableNode::*;
// Java: import cn.wps.moffice.writer.data.table.tree.RowInfoNode
use crate::cn::wps::moffice::writer::data::table::tree::RowInfoNode::*;
// Java: import cn.wps.moffice.writer.data.table.tree.TableInfoNode
use crate::cn::wps::moffice::writer::data::table::tree::TableInfoNode::*;

// java public class CellInfoNode extends TableNode implements CellInfo
#[derive(Default)]
pub struct CellInfoNode {
    // java TableNode super_
    pub(crate) super_: TableNode,
    // java private RowInfoNode mParent;
    mParent: Option<std::rc::Weak<RowInfoNode>>,
    // java private Data.CellData mData;
    mData: Option<CellData>,
}

// java public interface CellInfo
impl CellInfo for CellInfoNode {
    // java  int size()
    fn size(&self) -> i32 {
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

    // java public CellInfo getNextCell()
    fn getNextCell(&self) -> Option<&CellInfo> {
      // java return (CellInfo)this.getNextTableNode();
      return Some(&(self.super_.getNextTableNode()).downcast_ref::<CellInfo>().unwrap());
    }

    // java public CellInfo getPrevCell()
    fn getPrevCell(&self) -> Option<&CellInfo> {
      // java return (CellInfo)this.getPrevTableNode();
      return Some(&(self.super_.getPrevTableNode()).downcast_ref::<CellInfo>().unwrap());
    }

    // java public RowInfo getParent()
    fn getParent(&self) -> &RowInfo {
      // java return this.getParentNode();
      return { let __x: &RowInfo = todo!("no align strategy matched"); __x };
    }

    // java public TableInfo getTableByIndex(int index)
    fn getTableByIndex(&self, index: i32) -> Option<&TableInfo> {
      // java return (TableInfo)this.mTree.get(index);
      return Some(&(self.mTree.get(index)).downcast_ref::<TableInfo>().unwrap());
    }

    // java public TableInfo getTableByCP(int cp)
    fn getTableByCP(&self, cp: i32) -> Option<&TableInfo> {
      // java return (TableInfo)this.getNodeByCP(cp);
      return Some(&(self.super_.getNodeByCP(cp)).downcast_ref::<TableInfo>().unwrap());
    }

    // java public CellInfo getPrevVertMergeCell()
    fn getPrevVertMergeCell(&self) -> Option<&CellInfo> {
      // java this.valid();
      self.valid();
      // java return this.getData().getPrevVertMergeCell();
      return self.getData().getPrevVertMergeCell();
    }

    // java public CellInfo getNextVertMergeCell()
    fn getNextVertMergeCell(&self) -> Option<&CellInfo> {
      // java this.valid();
      self.valid();
      // java return this.getData().getNextVertMergeCell();
      return self.getData().getNextVertMergeCell();
    }

    // java public boolean isVertMergeCell()
    fn isVertMergeCell(&mut self) -> bool {
      // java this.valid();
      self.valid();
      // java return this.getData().isVertMergeCell();
      return self.getData().isVertMergeCell();
    }

    // java public boolean isFirstVertMergeCell()
    fn isFirstVertMergeCell(&mut self) -> bool {
      // java this.valid();
      self.valid();
      // java return this.getData().isFirstVertMergeCell();
      return self.getData().isFirstVertMergeCell();
    }

    // java public boolean isLastVertMergeCell()
    fn isLastVertMergeCell(&mut self) -> bool {
      // java this.valid();
      self.valid();
      // java return this.getData().isLastVertMergeCell();
      return self.getData().isLastVertMergeCell();
    }

    // java public int getHoriMerge()
    fn getHoriMerge(&mut self) -> i32 {
      // java this.mParent.valid();
      self.mParent.valid();
      // java return this.getData().getHoriMerge();
      return self.getData().getHoriMerge();
    }

    // java public int getVertMerge()
    fn getVertMerge(&mut self) -> i32 {
      // java this.valid();
      self.valid();
      // java return this.getData().getVertMerge();
      return self.getData().getVertMerge();
    }

    // java public int getVertAlign()
    fn getVertAlign(&mut self) -> i32 {
      // java this.mParent.valid();
      self.mParent.valid();
      // java return this.getData().getVertAlign();
      return self.getData().getVertAlign();
    }

    // java public int getTextFlow()
    fn getTextFlow(&mut self) -> i32 {
      // java this.mParent.valid();
      self.mParent.valid();
      // java return this.getData().getTextFlow();
      return self.getData().getTextFlow();
    }

    // java public boolean getFitText()
    fn getFitText(&mut self) -> bool {
      // java this.mParent.valid();
      self.mParent.valid();
      // java return this.getData().getFitText();
      return self.getData().getFitText();
    }

    // java public boolean getNoWrap()
    fn getNoWrap(&mut self) -> bool {
      // java this.mParent.valid();
      self.mParent.valid();
      // java return this.getData().getNoWrap();
      return self.getData().getNoWrap();
    }

    // java public int getRgdxaLeft()
    fn getRgdxaLeft(&mut self) -> i32 {
      // java return this.mParent.getRgdxa(this.getIndex());
      return self.mParent.getRgdxa(self.super_.getIndex());
    }

    // java public int getRgdxaRight()
    fn getRgdxaRight(&mut self) -> i32 {
      // java return this.mParent.getRgdxa(this.getIndex() + 1);
      return self.mParent.getRgdxa(self.super_.getIndex() + 1);
    }

    // java public int getWidth()
    fn getWidth(&mut self) -> i32 {
      // java return this.mParent.getCellWidth(this.getIndex());
      return self.mParent.getCellWidth(self.super_.getIndex());
    }

    // java public SHD getSHD()
    fn getSHD(&mut self) -> Option<&SHD> {
      // java this.mParent.valid();
      self.mParent.valid();
      // java return this.getData().getSHD();
      return self.getData().getSHD();
    }

    // java public int getMarginTop()
    fn getMarginTop(&mut self) -> i32 {
      // java this.mParent.valid();
      self.mParent.valid();
      // java return this.getData().getMarginTop();
      return self.getData().getMarginTop();
    }

    // java public int getMarginLeft()
    fn getMarginLeft(&mut self) -> i32 {
      // java this.mParent.valid();
      self.mParent.valid();
      // java return this.getData().getMarginLeft();
      return self.getData().getMarginLeft();
    }

    // java public int getMarginBottom()
    fn getMarginBottom(&mut self) -> i32 {
      // java this.mParent.valid();
      self.mParent.valid();
      // java return this.getData().getMarginBottom();
      return self.getData().getMarginBottom();
    }

    // java public int getMarginRight()
    fn getMarginRight(&mut self) -> i32 {
      // java this.mParent.valid();
      self.mParent.valid();
      // java return this.getData().getMarginRight();
      return self.getData().getMarginRight();
    }

    // java public BRC getBrcTop()
    fn getBrcTop(&mut self) -> Option<&BRC> {
      // java this.mParent.valid();
      self.mParent.valid();
      // java return this.getData().getBrcTop();
      return self.getData().getBrcTop();
    }

    // java public BRC getBrcLeft()
    fn getBrcLeft(&mut self) -> Option<&BRC> {
      // java this.mParent.valid();
      self.mParent.valid();
      // java return this.getData().getBrcLeft();
      return self.getData().getBrcLeft();
    }

    // java public BRC getBrcBottom()
    fn getBrcBottom(&mut self) -> Option<&BRC> {
      // java this.mParent.valid();
      self.mParent.valid();
      // java return this.getData().getBrcBottom();
      return self.getData().getBrcBottom();
    }

    // java public BRC getBrcRight()
    fn getBrcRight(&mut self) -> Option<&BRC> {
      // java this.mParent.valid();
      self.mParent.valid();
      // java return this.getData().getBrcRight();
      return self.getData().getBrcRight();
    }

    // java public BRC getBrcFromTopLeftToBottomRight()
    fn getBrcFromTopLeftToBottomRight(&mut self) -> Option<&BRC> {
      // java this.mParent.valid();
      self.mParent.valid();
      // java return this.getData().getBrcFromTopLeftToBottomRight();
      return self.getData().getBrcFromTopLeftToBottomRight();
    }

    // java public BRC getBrcFromTopRightToBottomLeft()
    fn getBrcFromTopRightToBottomLeft(&mut self) -> Option<&BRC> {
      // java this.mParent.valid();
      self.mParent.valid();
      // java return this.getData().getBrcFromTopRightToBottomLeft();
      return self.getData().getBrcFromTopRightToBottomLeft();
    }

    // java public boolean isAutoWidth()
    fn isAutoWidth(&mut self) -> bool {
      // java this.mParent.valid();
      self.mParent.valid();
      // java return this.getData().isAutoWidth();
      return self.getData().isAutoWidth();
    }

    // java public Data.CellData getData()
    fn getData(&mut self) -> &mut CellData {
      // java if (this.mData == null)
      if self.mData.is_none() {
        // java this.mData = new Data.CellData();
        self.mData = CellData::new();
      }
      // java return this.mData;
      return self.mData.unwrap();
    }

}

// java public class CellInfoNode extends TableNode implements CellInfo
impl CellInfoNode {
    // java   CellInfoNode(boolean createTree)
    pub fn CellInfoNode(createTree: bool) -> Self {
      // java super(createTree);
      Self { super_: TableNode::new(createTree), mParent: Default::default(), mData: Default::default() }
    }

    // java  TableNode newSubNode()
    pub fn newSubNode(&self) -> Box<TableNode> {
      // java return new TableInfoNode();
      return { let __x: Box<TableNode> = todo!("no align strategy matched"); __x };
    }

    // java  RowInfoNode getParentNode()
    pub fn getParentNode(&self) -> Option<&RowInfoNode> {
      // java return this.mParent;
      return { let __x: Option<&RowInfoNode> = todo!("no align strategy matched"); __x };
    }

    // java  void setParentNode(TableNode parentNode)
    pub fn setParentNode(&mut self, parentNode: Option<&TableNode>) {
      // java this.mParent = (RowInfoNode)parentNode;
      self.mParent = (parentNode).downcast_ref::<RowInfoNode>().unwrap();
    }

    // java  void init(ITableManager.ICellEntry cellEntry)
    pub fn init(&mut self, cellEntry: &ICellEntry) {
      // java this.mLength = cellEntry.getEnd() - cellEntry.getStart();
      self.mLength = cellEntry.getEnd() - cellEntry.getStart();
      // java this.tryRestoreData(cellEntry.getStart());
      self.tryRestoreData(cellEntry.getStart());
      // java ITableManager.IRowEntry rowEntry = cellEntry.getFirstChildRow();
      let mut rowEntry: ITableManager::IRowEntry = cellEntry.getFirstChildRow();
      // java if (rowEntry.isValid())
      if rowEntry.isValid() {
        // java this.createTree();
        self.super_.createTree();
        // java int subLevel = this.mParent.getParent().getLevel() + 1;
        let mut subLevel: i32 = self.mParent.getParent().getLevel() + 1;
        // java Node lastTable = this.mTree.getHead();
        let mut lastTable: Box<dyn Node> = self.mTree.getHead();
        // java int lastTableStart = cellEntry.getStart();
        let mut lastTableStart: i32 = cellEntry.getStart();
        // java do (rowEntry.isValid())
        loop {
            // java int tableStart = rowEntry.getStart();
            let mut tableStart: i32 = rowEntry.getStart();
            // java TableInfoNode newTable = (TableInfoNode)this.createSubNode();
            let mut newTable: TableInfoNode = (self.super_.createSubNode()).downcast_ref::<TableInfoNode>().unwrap();
            // java newTable.setOffset(tableStart - lastTableStart);
            newTable.setOffset(tableStart - lastTableStart);
            // java rowEntry = newTable.init(rowEntry, subLevel, false, 0);
            rowEntry = newTable.init(rowEntry, subLevel, false, 0);
            // java this.mTree.insertRight(lastTable, newTable);
            self.mTree.insertRight(lastTable, newTable);
            // java lastTable = newTable;
            lastTable = newTable;
            // java lastTableStart = tableStart;
            lastTableStart = tableStart;
            // java rowEntry = rowEntry.getNext();
            rowEntry = rowEntry.getNext();
        }
      }
    }

    // java public void tryRestoreData(int startCp)
    pub fn tryRestoreData(&mut self, startCp: i32) {
      // java if (this.mCache.getRebuildCache().isRebuilding())
      if self.mCache.getRebuildCache().isRebuilding() {
        // java Data.AutoFitInfo info = this.mCache.getRebuildCache().get(this.getLevel(), startCp, this.mLength);
        let mut info: crate::cn::wps::moffice::writer::data::table::Data::AutoFitInfo = self.mCache.getRebuildCache().get(self.getLevel(), startCp, self.mLength);
        // java if (info != null)
        if true {
          // java this.getData().mAutoFitInfo = info;
          self.getData().mAutoFitInfo = info;
        }
        // java else
        else {
          // java this.mCache.getRebuildCache().addInvalidRange(RANGEFactory.getRANGE(startCp, startCp + this.mLength));
          self.mCache.getRebuildCache().addInvalidRange(RANGEFactory::getRANGE(startCp, startCp + self.mLength));
        }
      }
    }

    // java private int getLevel()
    pub fn getLevel(&self) -> i32 {
      // java return this.getParent().getParent().getLevel();
      return self.getParent().getParent().getLevel();
    }

    // java  void valid()
    pub fn valid(&mut self) {
      // java TableInfoNode tableInfoNode = this.mParent.getParentNode();
      let mut tableInfoNode: TableInfoNode = { let __x: TableInfoNode = todo!("no align strategy matched"); __x };
      // java if (tableInfoNode != null)
      if true {
        // java tableInfoNode.validCell(this);
        tableInfoNode.validCell(self);
      }
    }

    // java  void validInTableLock()
    pub fn validInTableLock(&mut self) {
      // java TableInfoNode tableInfoNode = this.mParent.getParentNode();
      let mut tableInfoNode: TableInfoNode = { let __x: TableInfoNode = todo!("no align strategy matched"); __x };
      // java if (this.mGeneration < tableInfoNode.mGeneration)
      if self.mGeneration < tableInfoNode.mGeneration {
        // java this.mGeneration = tableInfoNode.mGeneration;
        self.mGeneration = tableInfoNode.mGeneration;
        // java this.mParent.valid();
        self.mParent.valid();
        // java Data.CellData data = this.getData();
      let mut data: crate::cn::wps::moffice::writer::data::table::Data::CellData = { let __x: crate::cn::wps::moffice::writer::data::table::Data::CellData = todo!("no align strategy matched"); __x };
        // java float left = (float)this.getRgdxaLeft();
        let mut left: f32 = ((self.getRgdxaLeft()) as f32);
        // java float right = (float)this.getRgdxaRight();
        let mut right: f32 = ((self.getRgdxaRight()) as f32);
        // java int vMerged = data.mTableCell == null ? 0 : data.mTableCell.getVertMerge();
        let mut vMerged: i32 = (if data.mTableCell.is_none() { 0 } else { data.mTableCell.getVertMerge() });
        // java if (vMerged == 0)
        if vMerged == 0 {
          // java data.mPrevVertMergeCell = data.mNextVertMergeCell = null;
          data.mPrevVertMergeCell = { data.mNextVertMergeCell = None; data.mNextVertMergeCell };
        }
        // java else if (vMerged == 3)
        else if vMerged == 3 {
          // java data.mPrevVertMergeCell = null;
          data.mPrevVertMergeCell = None;
          // java RowInfo nextRow = this.mParent.getNextRow();
          let mut nextRow: RowInfo = self.mParent.getNextRow();
          // java if (nextRow != null)
          if true {
            // java ((RowInfoNode)nextRow).valid();
            (nextRow).downcast_ref::<RowInfoNode>().unwrap().valid();
          }
          // java data.mNextVertMergeCell = TableTool.getNextMergeCell(nextRow, left, right);
          data.mNextVertMergeCell = TableTool::getNextMergeCell__deftable_int_deftable(nextRow, left, right);
        }
        // java else
        else {
          // java RowInfo preRow = this.mParent.getPrevRow();
          let mut preRow: RowInfo = self.mParent.getPrevRow();
          // java if (preRow != null)
          if true {
            // java ((RowInfoNode)preRow).valid();
            (preRow).downcast_ref::<RowInfoNode>().unwrap().valid();
          }
          // java data.mPrevVertMergeCell = TableTool.getPrevMergeCell(preRow, left, right);
          data.mPrevVertMergeCell = TableTool::getPrevMergeCell__deftable_int_deftable(preRow, left, right);
          // java RowInfo nextRow = this.mParent.getNextRow();
          let mut nextRow: RowInfo = self.mParent.getNextRow();
          // java if (nextRow != null)
          if true {
            // java ((RowInfoNode)nextRow).valid();
            (nextRow).downcast_ref::<RowInfoNode>().unwrap().valid();
          }
          // java data.mNextVertMergeCell = TableTool.getNextMergeCell(nextRow, left, right);
          data.mNextVertMergeCell = TableTool::getNextMergeCell__deftable_int_deftable(nextRow, left, right);
        }
      }
    }

    // java  void valid(int start, int end)
    pub fn valid__int_int(&mut self, start: i32, end: i32) {
      // java this.createTree();
      self.super_.createTree();
      // java int[] offsetBuf = new int[1];
      let mut offsetBuf: Vec<i32> = vec![Default::default(); 1usize];
      // java this.truncateTables(start, end, offsetBuf);
      self.truncateTables(start, end, offsetBuf);
      // java TableInfoNode lastTable = (TableInfoNode)this.getNodeByOffset(start, offsetBuf);
      let mut lastTable: TableInfoNode = (self.super_.getNodeByOffset(start, offsetBuf)).downcast_ref::<TableInfoNode>().unwrap();
      // java if (!lastTable.isHead() && offsetBuf[0] == lastTable.mLength)
      if !lastTable.isHead() && offsetBuf[0usize] == lastTable.mLength {
        // java int oldLength = lastTable.mLength;
        let mut oldLength: i32 = lastTable.mLength;
        // java lastTable = lastTable.tryAppend();
        lastTable = lastTable.tryAppend();
        // java start += lastTable.mLength - oldLength;
        start = start + lastTable.mLength - oldLength;
      }
      // java int position = this.getStartCP();
      let mut position: i32 = self.super_.getStartCP();
      // java int startCP = position + start;
      let mut startCP: i32 = position + start;
      // java int endCP = position + end;
      let mut endCP: i32 = position + end;
      // java int level = this.mParent == null ? 1 : this.mParent.getParent().getLevel() + 1;
      let mut level: i32 = (if self.mParent.is_none() { 1 } else { self.mParent.getParent().getLevel() + 1 });
      // java int lastTableStart = lastTable.isHead() ? position : position + lastTable.offsetFromParent();
      let mut lastTableStart: i32 = (if lastTable.isHead() { position } else { position + lastTable.offsetFromParent() });
      // java KTableManager tableManager = this.getTableManager();
      let mut tableManager: KTableManager = self.super_.getTableManager().clone();
      // java while (startCP < endCP)
      while startCP < endCP {
        // java startCP = tableManager.findNextRowStart(startCP, endCP, level);
        startCP = tableManager.findNextRowStart(startCP, endCP, level);
        // java if (startCP < endCP)
        if startCP < endCP {
          // java ITableManager.IRowEntry rowEntry = tableManager.createRow(startCP, level);
          let mut rowEntry: ITableManager::IRowEntry = tableManager.createRow(startCP, level);
          // java TableInfoNode newTable = (TableInfoNode)this.createSubNode();
          let mut newTable: TableInfoNode = (self.super_.createSubNode()).downcast_ref::<TableInfoNode>().unwrap();
          // java newTable.setOffset(startCP - lastTableStart);
          newTable.setOffset(startCP - lastTableStart);
          // java rowEntry = newTable.init(rowEntry, level, true, endCP);
          rowEntry = newTable.init(rowEntry, level, true, endCP);
          // java this.mTree.insertRight(lastTable, newTable);
          self.mTree.insertRight(lastTable, newTable);
          // java TableInfoNode next = (TableInfoNode)newTable.getNext();
          let mut next: TableInfoNode = (newTable.getNext()).downcast_ref::<TableInfoNode>().unwrap();
          // java if (rowEntry.getEnd() == endCP && !next.isHead() && endCP == next.getStartCP())
          if rowEntry.getEnd() == endCP && !next.isHead() && endCP == next.getStartCP() {
            // java newTable = newTable.tryAppend();
            newTable = newTable.tryAppend();
          }
          // java lastTable = newTable;
          lastTable = newTable;
          // java lastTableStart = startCP;
          lastTableStart = startCP;
          // java startCP = rowEntry.getEnd();
          startCP = rowEntry.getEnd();
          // java rowEntry.recycle();
          rowEntry.recycle();
        }
      }
    }

    // java private void truncateTables(int start, int end, int[] offsetBuf)
    pub fn truncateTables(&mut self, start: i32, end: i32, offsetBuf: ()) {
      // java if (this.nodeCount() != 0)
      if self.super_.nodeCount() != 0 {
        // java TableNode startTable = this.getNodeStrict(start, offsetBuf);
        let mut startTable: TableNode = self.super_.getNodeStrict(start, offsetBuf).unwrap();
        // java if (startTable != null && offsetBuf[0] > 0)
        if true && offsetBuf[0usize] > 0 {
          // java startTable.mLength = offsetBuf[0];
          startTable.mLength = offsetBuf[0usize];
        }
        // java TableNode endTable = this.getNodeStrict(end, offsetBuf);
        let mut endTable: TableNode = self.super_.getNodeStrict(end, offsetBuf).unwrap();
        // java if (endTable != null && offsetBuf[0] > 0)
        if true && offsetBuf[0usize] > 0 {
          // java int spaceBefore = offsetBuf[0];
          let mut spaceBefore: i32 = offsetBuf[0usize];
          // java endTable.mTree.moveRight(endTable, spaceBefore);
          endTable.mTree.moveRight(endTable, spaceBefore);
          // java endTable.mLength -= spaceBefore;
          endTable.mLength = endTable.mLength - spaceBefore;
          // java endTable.mTree.insertSpaceFromUpperBound(0, -spaceBefore);
          endTable.mTree.insertSpaceFromUpperBound(0, -spaceBefore);
        }
      }
    }

    // java  void positionFix(int offsetFromThis, int steps, int[] offsetBuf)
    pub fn positionFix(&mut self, offsetFromThis: i32, steps: i32, offsetBuf: ()) {
      // java super.positionFix(offsetFromThis, steps, offsetBuf);
      self.super_.positionFix(offsetFromThis, steps, offsetBuf);
      // java if (steps < 0 && this.nodeCount() >= 2)
      if steps < 0 && self.super_.nodeCount() >= 2 {
        // java TableNode nextTable = this.getNodeByOffset(offsetFromThis, offsetBuf);
        let mut nextTable: TableNode = self.super_.getNodeByOffset(offsetFromThis, offsetBuf).clone();
        // java if (!nextTable.isHead() && offsetBuf[0] == 0)
        if !nextTable.isHead() && offsetBuf[0usize] == 0 {
          // java TableInfoNode prevTable = (TableInfoNode)nextTable.getPrev();
          let mut prevTable: TableInfoNode = (nextTable.getPrev()).downcast_ref::<TableInfoNode>().unwrap();
          // java if (!prevTable.isHead() && prevTable.offsetFromParent() + prevTable.mLength == offsetFromThis)
          if !prevTable.isHead() && prevTable.offsetFromParent() + prevTable.mLength == offsetFromThis {
            // java this.mCache.markModify(this.getStartCP() + offsetFromThis - 1);
            self.mCache.markModify(self.super_.getStartCP() + offsetFromThis - 1);
          }
        }
      }
    }

    // java protected void close()
    pub fn close(&mut self) {
      // java if (this.mData != null && this.mData.mAutoFitInfo != null)
      if let Some(mData) = self.mData.as_mut() {
        if self.mData.mAutoFitInfo.is_some() {
          // java this.mCache.getRebuildCache().backup(this.getLevel(), this.getStartCP(), this.mLength, this.mData.mAutoFitInfo);
          self.mCache.getRebuildCache().backup(self.getLevel(), self.super_.getStartCP(), self.mLength, mData.mAutoFitInfo);
          // java this.mData.mAutoFitInfo = null;
          mData.mAutoFitInfo = None;
        }
      }
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

impl std::ops::Deref for CellInfoNode {
    type Target = TableNode;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for CellInfoNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
