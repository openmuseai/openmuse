// Java: import cn.wps.collections.Node
use util::collections::Node::*;
// Java: import cn.wps.collections.textrope.INotifyTextRope
use crate::cn::wps::collections::textrope::INotifyTextRope::*;
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::*;
// Java: import cn.wps.moffice.transaction.Transaction
use util::moffice::transaction::Transaction::*;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::*;
// Java: import cn.wps.moffice.writer.core.DocumentEvent
use crate::cn::wps::moffice::writer::core::DocumentEvent::*;
// Java: import cn.wps.moffice.writer.core.DocumentListener
use crate::cn::wps::moffice::writer::core::DocumentListener::*;
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
// Java: import cn.wps.moffice.writer.data.table.TableInfosImpl
use crate::cn::wps::moffice::writer::data::table::TableInfosImpl::*;
// Java: import cn.wps.moffice.writer.data.table.TableTool
use crate::cn::wps::moffice::writer::data::table::TableTool::*;
// Java: import cn.wps.moffice.writer.data.table.grid.GridResizeRequest
use crate::cn::wps::moffice::writer::data::table::grid::GridResizeRequest::*;
// Java: import cn.wps.moffice.writer.data.table.tree.CellInfoNode
use crate::cn::wps::moffice::writer::data::table::tree::CellInfoNode::*;
// Java: import cn.wps.moffice.writer.io.writer.html.table.cache.TableCache
use crate::cn::wps::moffice::writer::io::writer::html::table::cache::TableCache::*;
// Java: import cn.wps.moffice.writer.data.table.tree.IOCache
use crate::cn::wps::moffice::writer::data::table::tree::IOCache::*;
// Java: import cn.wps.moffice.writer.data.table.tree.TableInfoNode
use crate::cn::wps::moffice::writer::data::table::tree::TableInfoNode::*;
// Java: import org.apache.poi.poifs.filesystem.Entry
use io_poi_compact::org::apache::poi::poifs::filesystem::Entry::*;
// Java: import cn.wps.moffice.writer.data.table.tree.TableNode
use crate::cn::wps::moffice::writer::data::table::tree::TableNode::*;

// java public class TreeTableCache extends CellInfoNode implements TableInfosImpl.TableCache, INotifyTextRope.Listener, IPapxTable.Listener, IChpxTable.Listener, Transaction.Listener, DocumentListener
#[derive(Default)]
pub struct TreeTableCache {
    // java CellInfoNode super_
    pub(crate) super_: CellInfoNode,
    // java Document mDocument;
    pub(crate) mDocument: Option<dyn Document>,
    // java KTableManager mTableManager;
    pub(crate) mTableManager: Option<KTableManager>,
    // java IOCache mIoCache;
    pub(crate) mIoCache: Option<Box<dyn IOCache>>,
    // java TableTool mTableTool;
    pub(crate) mTableTool: Option<TableTool>,
    // java int mInvalidCP = 0;
    pub(crate) mInvalidCP: i32,
    // java boolean mIoMode;
    pub(crate) mIoMode: bool,
    // java long mRange;
    pub(crate) mRange: i64,
    // java boolean mNodeChanged;
    pub(crate) mNodeChanged: bool,
    // java AutoResizeRange mAutoResizeRange = new AutoResizeRange();
    pub(crate) mAutoResizeRange: Option<AutoResizeRange>,
    // java Data.RebuildCache mRebuildCache;
    pub(crate) mRebuildCache: Option<crate::cn::wps::moffice::writer::data::table::Data::RebuildCache>,
}

// java private class AutoResizeRange
#[derive(Default)]
pub struct AutoResizeRange {
    // java long mRangeTimely = 0L;
    pub(crate) mRangeTimely: i64,
    // java long mRangeDelay = 0L;
    pub(crate) mRangeDelay: i64,
    // java long mLastChangeRangeInSameCell = 0L;
    pub(crate) mLastChangeRangeInSameCell: i64,
    // java boolean mNodeChanged = false;
    pub(crate) mNodeChanged: bool,
}

// java public class TreeTableCache extends CellInfoNode implements TableInfosImpl.TableCache, INotifyTextRope.Listener, IPapxTable.Listener, IChpxTable.Listener, Transaction.Listener, DocumentListener
impl TreeTableCache {
    // java public GridResizeRequest getGridResizeRequest()
    pub fn getGridResizeRequest(&self) -> Box<dyn GridResizeRequest> {
      // java return this.mDocument.getTextDocument().getGridResizeRequest();
      return self.mDocument.getTextDocument().getGridResizeRequest();
    }

    // java public  TreeTableCache(Document document)
    pub fn TreeTableCache(document: &dyn Document) -> Self {
      // java super(true);
      // java this.mDocument = document;
      let mut this = Self { super_: CellInfoNode::new(true), mDocument: document, mTableManager: KTableManager::create(&document), mIoCache: IOCache::new(self), mTableTool: TableTool::new(document), mInvalidCP: Default::default(), mIoMode: true, mRange: Default::default(), mNodeChanged: Default::default(), mAutoResizeRange: Default::default(), mRebuildCache: Default::default() };
      // java this.mCache = this;
      // java this.mTableTool = new TableTool(document);
      // java this.mTableManager = KTableManager.create(document);
      // java document.getTextDocument().addTransactionListener(this);
      document.getTextDocument().addTransactionListener(self);
      // java document.getDispatcher().addDocumentListener(this);
      document.getDispatcher().addDocumentListener(self);
      // java this.mIoCache = new IOCache(this);
      // java this.mIoMode = true;
      this
    }

    // java private void changeRange(long range, boolean applyAutoSize)
    pub fn changeRange(&mut self, range: i64, applyAutoSize: bool) {
      // java GridResizeRequest resizeRequest = this.getGridResizeRequest();
      let mut resizeRequest: Box<dyn GridResizeRequest> = self.getGridResizeRequest();
      // java if (resizeRequest == null || !resizeRequest.isCommiting())
      if resizeRequest.is_none() || !resizeRequest.isCommiting() {
        // java this.mRange = range;
        self.mRange = range;
        // java if (applyAutoSize)
        if applyAutoSize {
          // java this.mAutoResizeRange.mRangeTimely = RANGEFactory.include(this.mAutoResizeRange.mRangeTimely, this.mRange);
          self.mAutoResizeRange.mRangeTimely = RANGEFactory::include__long_int(self.mAutoResizeRange.mRangeTimely, self.mRange);
        }
      }
    }

    // java public Data.RebuildCache getRebuildCache()
    pub fn getRebuildCache(&mut self) -> crate::cn::wps::moffice::writer::data::table::Data::RebuildCache {
      // java if (this.mRebuildCache == null)
      if self.mRebuildCache.is_none() {
        // java this.mRebuildCache = new Data.RebuildCache();
        self.mRebuildCache = RebuildCache::new();
      }
      // java return this.mRebuildCache;
      return self.mRebuildCache;
    }

    // java public TableInfo getRootTable(int startCP)
    pub fn getRootTable(&mut self, startCP: i32) -> TableInfo {
      // java this.updateCache(startCP);
      self.updateCache(startCP);
      // java return this.getTableByCP(startCP);
      return self.super_.getTableByCP(startCP);
    }

    // java public TableInfo getTableByIndex(int index)
    pub fn getTableByIndex(&self, index: i32) -> Option<TableInfo> {
      // java return index >= 0 && index < this.size() ? super.getTableByIndex(index) : null;
      return (if index >= 0 && index < self.size() { Some(self.super_.getTableByIndex(index)) } else { None });
    }

    // java public int size()
    pub fn size(&mut self) -> i32 {
      // java this.updateCache(this.mDocument.getLength() - 1);
      self.updateCache(self.mDocument.getLength() - 1);
      // java return super.size();
      return self.super_.size();
    }

    // java private synchronized void updateCache(int cp)
    pub fn updateCache(&mut self, cp: i32) {
      // java this.commitModify();
      self.commitModify();
      // java if (this.mIoMode)
      if self.mIoMode {
        // java this.mIoCache.collectTo(cp);
        self.mIoCache.collectTo(cp);
      }
      // java else
      else {
        // java this.updateInNormal(cp);
        self.updateInNormal(cp);
      }
    }

    // java public boolean canLookFast()
    pub fn canLookFast(&mut self) -> bool {
      // java if (this.mDocument.getTextDocument().isInIOState())
      if self.mDocument.getTextDocument().isInIOState() {
        // java return false;
        return false;
      }
      // java else if (!this.mIoMode)
      else if !self.mIoMode {
        // java return true;
        return true;
      }
      // java else
      else {
        // java TableInfoNode lastTable = (TableInfoNode)this.mTree.rightmost();
        let mut lastTable: TableInfoNode = (self.mTree.rightmost()).downcast_ref::<TableInfoNode>().unwrap();
        // java return lastTable.isHead() ? true : lastTable.isComplete();
        return (if lastTable.isHead() { true } else { lastTable.isComplete() });
      }
    }

    // java private void updateInNormal(int cp)
    pub fn updateInNormal(&mut self, cp: i32) {
      // java label19: while(true) { if (cp >= this.mInvalidCP && this.mInvalidCP < this.mDocument.getLength()) { if (!this.mTableManager.isInTable(this.mInvalidCP, 1)) { IPapxTable.Entry para = this.mTableManager.findNextTableRowStart(this.mInvalidCP); this.mInvalidCP = para.position(); continue; } while(true) { ITableManager.IRowEntry row = this.mTableManager.createRow(this.mInvalidCP, 1); TableInfoNode newTable = (TableInfoNode)this.createSubNode(); row = newTable.init(row, 1, true, Integer.MAX_VALUE); this.mTree.insertRight((TableInfoNode)this.mTree.rightmost(), newTable, this.mInvalidCP); this.mInvalidCP = row.getEnd(); row.recycle(); if (!this.mTableManager.isInTable(this.mInvalidCP, 1)) { continue label19; } } } return;
      loop {
if cp >= self.mInvalidCP && self.mInvalidCP < self.mDocument.getLength() { if !self.mTableManager.isInTable(self.mInvalidCP, 1) { let mut para: Box<dyn IPapxTableEntry> = self.mTableManager.findNextTableRowStart(self.mInvalidCP);
self.mInvalidCP = para.position();
continue; }
loop {
let mut row: ITableManager::IRowEntry = self.mTableManager.createRow__int_int(self.mInvalidCP, 1);
let mut newTable: TableInfoNode = (self.super_.createSubNode()).downcast_ref::<TableInfoNode>().unwrap();
row = newTable.init(row, 1, true, Integer::MAX_VALUE);
self.mTree.insertRight((self.mTree.rightmost()).downcast_ref::<TableInfoNode>().unwrap(), newTable, self.mInvalidCP);
self.mInvalidCP = row.getEnd();
row.recycle();
if !self.mTableManager.isInTable(self.mInvalidCP, 1) { continue; }
} }
return;
}
    }

    // java public int getStartCP()
    pub fn getStartCP(&self) -> i32 {
      // java return 0;
      return 0;
    }

    // java public String toString()
    pub fn toString(&mut self) -> String {
      // java StringBuilder builder = new StringBuilder();
      let mut builder: String = StringBuilder::new();
      // java int i = 0;
      let mut i: i32 = 0;
      let mut node: Box<dyn Node> = self.mTree.leftmost();
      // java for(Node node = this.mTree.leftmost(); !node.isHead() node = node.getNext())
      while !node.isHead() {
        // java TableNode tableNode = (TableNode)node;
        let mut tableNode: TableNode = (node).downcast_ref::<TableNode>().unwrap();
        // java builder.append("table");
        builder.append("table");
        // java builder.append(i++);
        builder.append({ let _tmp = i; i += 1; _tmp });
        // java builder.append(':');
        builder.append(':');
        // java builder.append(RANGEFactory.toString(tableNode.getRange()));
        builder.append(RANGEFactory::toString(tableNode.getRange()));
        // java builder.append(',');
        builder.append(',');
        // java builder.append(tableNode.size());
        builder.append((tableNode.len() as i32));
        // java builder.append("rows\n");
        builder.append("rows\n");
      }
      // java return builder.toString();
      return builder.toString();
    }

    // java public void beforeInsertText(int cp, int fc, int length)
    pub fn beforeInsertText(&mut self, cp: i32, fc: i32, length: i32) {
      // java this.mIoCache.clear();
      self.mIoCache.clear();
      // java if (cp < this.mInvalidCP)
      if cp < self.mInvalidCP {
        // java this.mInvalidCP += length;
        self.mInvalidCP = self.mInvalidCP + length;
        // java this.changeRange(RANGEFactory.insertText(this.mRange, cp, length), true);
        self.changeRange(RANGEFactory::insertText(self.mRange, cp, length), true);
      }
    }

    // java public void afterInsertText(int cp, int fc, int length)
    pub fn afterInsertText(&mut self, cp: i32, fc: i32, length: i32) {
      // java if (cp < this.mInvalidCP)
      if cp < self.mInvalidCP {
        // java this.positionFix(cp, length, new int[1]);
        self.super_.positionFix(cp, length, &vec![Default::default(); 1usize]);
      }
    }

    // java public void beforeRemoveText(int start, int end)
    pub fn beforeRemoveText(&mut self, start: i32, end: i32) {
      // java this.mIoCache.clear();
      self.mIoCache.clear();
      // java if (start < this.mInvalidCP)
      if start < self.mInvalidCP {
        // java this.mNodeChanged |= this.removeNodeInRange(start, end, new int[1]);
        self.mNodeChanged = self.mNodeChanged | self.super_.removeNodeInRange(start, end, &vec![Default::default(); 1usize]);
        // java AutoResizeRange var10000 = this.mAutoResizeRange;
        let mut var10000: AutoResizeRange = self.mAutoResizeRange;
        // java var10000.mNodeChanged |= this.mNodeChanged;
        var10000.mNodeChanged = var10000.mNodeChanged | self.mNodeChanged;
      }
    }

    // java public void afterRemoveText(int start, int end)
    pub fn afterRemoveText(&mut self, start: i32, end: i32) {
      // java if (start < this.mInvalidCP)
      if start < self.mInvalidCP {
        // java if (end >= this.mInvalidCP)
        if end >= self.mInvalidCP {
          // java this.mInvalidCP = start;
          self.mInvalidCP = start;
        }
        // java else
        else {
          // java this.mInvalidCP -= end - start;
          self.mInvalidCP = self.mInvalidCP - end - start;
        }
        // java this.changeRange(RANGEFactory.removeText(this.mRange, start, end), true);
        self.changeRange(RANGEFactory::removeText(self.mRange, start, end), true);
        // java if (RANGEFactory.isEmpty(this.mAutoResizeRange.mRangeTimely))
        if RANGEFactory::isEmpty(self.mAutoResizeRange.mRangeTimely) {
          // java this.mAutoResizeRange.mRangeTimely = RANGEFactory.getRANGE(start, start + 1);
          self.mAutoResizeRange.mRangeTimely = RANGEFactory::getRANGE(start, start + 1);
        }
        // java this.positionFix(start, start - end, new int[1]);
        self.super_.positionFix(start, start - end, &vec![Default::default(); 1usize]);
      }
    }

    // java public void beforeAddEntry(int cp, PropertySet data)
    pub fn beforeAddEntry(&mut self, cp: i32, data: &PropertySet) {
      // java this.mIoCache.clear();
      self.mIoCache.clear();
    }

    // java public void afterAddEntry(ITable.Entry entry)
    pub fn afterAddEntry(&mut self, entry: &mut dyn ITableEntry) {
      // java int start = entry.position();
      let mut start: i32 = entry.position();
      // java int end = entry.getNext().position();
      let mut end: i32 = entry.getNext().position();
      // java if (end > this.mInvalidCP)
      if end > self.mInvalidCP {
        // java end = this.mInvalidCP;
        end = self.mInvalidCP;
      }
      // java if (start < end)
      if start < end {
        // java this.changeRange(RANGEFactory.include(this.mRange, start, end), true);
        self.changeRange(RANGEFactory::include__long_int_int(self.mRange, start, end), true);
      }
    }

    // java public void beforeRemoveEntry(ITable.Entry entry)
    pub fn beforeRemoveEntry(&mut self, entry: &mut dyn ITableEntry) {
      // java this.mIoCache.clear();
      self.mIoCache.clear();
      // java int start = entry.position();
      let mut start: i32 = entry.position();
      // java int end = entry.getNext().position();
      let mut end: i32 = entry.getNext().position();
      // java if (end > this.mInvalidCP)
      if end > self.mInvalidCP {
        // java end = this.mInvalidCP;
        end = self.mInvalidCP;
      }
      // java if (start < end)
      if start < end {
        // java this.changeRange(RANGEFactory.include(this.mRange, start, end), true);
        self.changeRange(RANGEFactory::include__long_int_int(self.mRange, start, end), true);
      }
    }

    // java public void afterRemoveEntry(ITable.Entry entry)
    pub fn afterRemoveEntry(&mut self, entry: &dyn ITableEntry) {
    }

    // java public void beforeDataChange(ITable.Entry entry, PropertySet data)
    pub fn beforeDataChange(&mut self, entry: &mut dyn ITableEntry, data: &PropertySet) {
      // java this.mIoCache.clear();
      self.mIoCache.clear();
      // java int start = entry.position();
      let mut start: i32 = entry.position();
      // java int end = entry.getNext().position();
      let mut end: i32 = entry.getNext().position();
      // java if (end > this.mInvalidCP)
      if end > self.mInvalidCP {
        // java end = this.mInvalidCP;
        end = self.mInvalidCP;
      }
      // java if (start < end)
      if start < end {
        // java this.changeRange(RANGEFactory.include(this.mRange, start, end), true);
        self.changeRange(RANGEFactory::include__long_int_int(self.mRange, start, end), true);
      }
    }

    // java public void afterDataChange(ITable.Entry entry, PropertySet data)
    pub fn afterDataChange(&mut self, entry: &dyn ITableEntry, data: &PropertySet) {
    }

    // java public void update(DocumentEvent e)
    pub fn update(&mut self, e: &dyn DocumentEvent) {
      // java if (e.getType() == 5)
      if e.getType() == 5 {
        // java int cp = this.mDocument.getTextRope().fc2cp(e.getOffset());
        let mut cp: i32 = self.mDocument.getTextRope().fc2cp(e.getOffset());
        // java if (cp < this.mInvalidCP)
        if cp < self.mInvalidCP {
          // java this.markModify(cp);
          self.markModify(cp);
        }
      }
      // java else if (e.getType() == 6)
      else if e.getType() == 6 {
        // java int cp = e.getOffset();
        let mut cp: i32 = e.getOffset();
        // java if (cp >= 0 && cp < this.mInvalidCP)
        if cp >= 0 && cp < self.mInvalidCP {
          // java this.mAutoResizeRange.mRangeDelay = RANGEFactory.include(this.mAutoResizeRange.mRangeDelay, cp);
          self.mAutoResizeRange.mRangeDelay = RANGEFactory::include__long_int(self.mAutoResizeRange.mRangeDelay, cp);
        }
      }
    }

    // java private void cleanMark()
    pub fn cleanMark(&mut self) {
      // java this.mRange = 0L;
      self.mRange = 0;
      // java this.mNodeChanged = false;
      self.mNodeChanged = false;
    }

    // java  void markModify(int cp)
    pub fn markModify(&mut self, cp: i32) {
      // java this.mIoCache.clear();
      self.mIoCache.clear();
      // java this.changeRange(RANGEFactory.include(this.mRange, cp), true);
      self.changeRange(RANGEFactory::include__long_int(self.mRange, cp), true);
    }

    // java private void startModify()
    pub fn startModify(&mut self) {
      // java this.cleanMark();
      self.cleanMark();
    }

    // java private synchronized void commitModify()
    pub fn commitModify(&mut self) {
          // java if (!this.mNodeChanged && this.isInDeepestCellNoNodeChanged(this.mRange))
          if !self.mNodeChanged && self.isInDeepestCellNoNodeChanged(self.mRange) {
            // java this.mAutoResizeRange.mLastChangeRangeInSameCell = this.mRange;
            self.mAutoResizeRange.mLastChangeRangeInSameCell = self.mRange;
          }
          // java else if (!RANGEFactory.isEmpty(this.mRange))
          else if !RANGEFactory::isEmpty(self.mRange) {
            // java int[] offsetBuf = new int[1];
            let mut offsetBuf: Vec<i32> = vec![Default::default(); 1usize];
            // java TableNode rootNode = this.getRootNode(offsetBuf);
            let mut rootNode: TableNode = self.getRootNode(&offsetBuf);
            // java if (rootNode == null)
            if rootNode.is_none() {
              // java this.invalidAfter(RANGEFactory.getStart(this.mRange));
              self.invalidAfter(RANGEFactory::getStart(self.mRange));
            }
            // java else
            else {
              // java int position = rootNode.getStartCP();
              let mut position: i32 = rootNode.getStartCP();
              // java int start = RANGEFactory.getStart(this.mRange) - position;
              let mut start: i32 = RANGEFactory::getStart(self.mRange) - position;
              // java int end = RANGEFactory.getEnd(this.mRange) - position;
              let mut end: i32 = RANGEFactory::getEnd(self.mRange) - position;
              // java this.getRebuildCache().start();
              self.getRebuildCache().start();
              // java rootNode.removeNodeInRange(start, end, offsetBuf);
              rootNode.removeNodeInRange(start, end, offsetBuf);
              // java rootNode.valid(start, end);
              rootNode.valid(start, end);
              // java this.mAutoResizeRange.mergeRangFormRebuildCache();
              self.mAutoResizeRange.mergeRangFormRebuildCache();
              // java this.getRebuildCache().end();
              self.getRebuildCache().end();
              // java TableNode lastTable = (TableNode)this.mTree.rightmost();
              let mut lastTable: TableNode = (self.mTree.rightmost()).downcast_ref::<TableNode>().unwrap();
              // java if (!lastTable.isHead())
              if !lastTable.isHead() {
                // java this.mInvalidCP = Math.max(this.mInvalidCP, lastTable.getEndCP());
                self.mInvalidCP = Math::max(self.mInvalidCP, lastTable.getEndCP());
              }
            }
          }
          // java this.cleanMark();
          self.cleanMark();
          // java if (!this.mDocument.getTextDocument().isInIOState())
          if !self.mDocument.getTextDocument().isInIOState() {
            // java if (this.mIoMode)
            if self.mIoMode {
              // java this.mIoCache.apply(true);
              self.mIoCache.apply(true);
            }
            // java this.mIoMode = false;
            self.mIoMode = false;
            // java this.tryAppend();
            self.tryAppend();
          }
          // java Exception var7
          todo!();
            // java this.invalidAfter(0);
            self.invalidAfter(0);
    }

    // java public void doGridAutoResize(boolean fourceRequest)
    pub fn doGridAutoResize(&mut self, fourceRequest: bool) {
      // java long autoSieRange = this.mAutoResizeRange.finishRangeDelay();
      let mut autoSieRange: i64 = self.mAutoResizeRange.finishRangeDelay();
      // java if (!RANGEFactory.isEmpty(autoSieRange))
      if !RANGEFactory::isEmpty(autoSieRange) {
        // java GridResizeRequest resizeRequest = this.getGridResizeRequest();
        let mut resizeRequest: Box<dyn GridResizeRequest> = self.getGridResizeRequest();
        // java if (resizeRequest != null)
        if resizeRequest.is_some() {
          // java resizeRequest.requestWithReTypo(autoSieRange, this.mDocument);
          resizeRequest.requestWithReTypo(autoSieRange, self.mDocument);
        }
      }
      // java else if (fourceRequest)
      else if fourceRequest {
        // java GridResizeRequest resizeRequest = this.getGridResizeRequest();
        let mut resizeRequest: Box<dyn GridResizeRequest> = self.getGridResizeRequest();
        // java if (resizeRequest != null)
        if resizeRequest.is_some() {
          // java resizeRequest.requestWithReTypo(0L, this.mDocument);
          resizeRequest.requestWithReTypo(0, self.mDocument);
        }
      }
    }

    // java public void invalidAfter(int cp)
    pub fn invalidAfter(&mut self, cp: i32) {
      // java this.mIoCache.clear();
      self.mIoCache.clear();
      // java if (cp < this.mInvalidCP)
      if cp < self.mInvalidCP {
        // java if (!RANGEFactory.isEmpty(this.mRange))
        if !RANGEFactory::isEmpty(self.mRange) {
          // java cp = Math.min(RANGEFactory.getStart(this.mRange), cp);
          cp = Math::min(RANGEFactory::getStart(self.mRange), cp);
          // java this.mRange = 0L;
          self.mRange = 0;
        }
        // java this.mInvalidCP = cp;
        self.mInvalidCP = cp;
        // java TableNode prev;
        let mut prev: TableNode = TableNode::default();
        let mut lastTable: TableNode = (self.mTree.rightmost()).downcast_ref::<TableNode>().unwrap();
        // java for(TableNode lastTable = (TableNode)this.mTree.rightmost(); !lastTable.isHead() && lastTable.getEndCP() > cp lastTable = prev)
        while !lastTable.isHead() && lastTable.getEndCP() > cp {
          // java this.mInvalidCP = Math.min(this.mInvalidCP, lastTable.getStartCP());
          self.mInvalidCP = Math::min(self.mInvalidCP, lastTable.getStartCP());
          // java prev = (TableNode)lastTable.getPrev();
          prev = (lastTable.getPrev()).downcast_ref::<TableNode>().unwrap();
          // java this.eraseSubNode(lastTable);
          self.super_.eraseSubNode(&mut lastTable);
        }
        // java this.tryAppend();
        self.tryAppend();
      }
    }

    // java public void beforeStart(String description)
    pub fn beforeStart(&mut self, description: &str) {
      // java this.startModify();
      self.startModify();
    }

    // java public void afterCommit(String description)
    pub fn afterCommit(&mut self, description: &str) {
      // java this.commitModify();
      self.commitModify();
      // java this.doGridAutoResize(false);
      self.doGridAutoResize(false);
    }

    // java public void afterRollback()
    pub fn afterRollback(&mut self) {
      // java this.commitModify();
      self.commitModify();
    }

    // java public void beforeUndo(String description)
    pub fn beforeUndo(&mut self, description: &str) {
      // java this.startModify();
      self.startModify();
    }

    // java public void afterUndo(String description)
    pub fn afterUndo(&mut self, description: &str) {
      // java this.commitModify();
      self.commitModify();
      // java this.doGridAutoResize(false);
      self.doGridAutoResize(false);
    }

    // java public void beforeRedo(String description)
    pub fn beforeRedo(&mut self, description: &str) {
      // java this.startModify();
      self.startModify();
    }

    // java public void afterRedo(String description)
    pub fn afterRedo(&mut self, description: &str) {
      // java this.commitModify();
      self.commitModify();
      // java this.doGridAutoResize(false);
      self.doGridAutoResize(false);
    }

    // java private boolean isInDeepestCellNoNodeChanged(long changeRange)
    pub fn isInDeepestCellNoNodeChanged(&self, changeRange: i64) -> bool {
      // java if (RANGEFactory.isEmpty(changeRange))
      if RANGEFactory::isEmpty(changeRange) {
        // java return false;
        return false;
      }
      // java else
      else {
        // java int start = RANGEFactory.getStart(changeRange);
        let mut start: i32 = RANGEFactory::getStart(changeRange);
        // java int end = RANGEFactory.getEnd(changeRange);
        let mut end: i32 = RANGEFactory::getEnd(changeRange);
        // java int sameCellLevel = this.mTableManager.getSameCellDeepestLevel(start, end);
        let mut sameCellLevel: i32 = self.mTableManager.getSameCellDeepestLevel(start, end);
        // java if (sameCellLevel == 0)
        if sameCellLevel == 0 {
          // java return false;
          return false;
        }
        // java else
        else {
          // java CellInfo cellInfo = this.getDeepestCell(start);
          let mut cellInfo: CellInfo = self.getDeepestCell(start);
          // java if (cellInfo != null && cellInfo.getParent().getParent().getLevel() == sameCellLevel)
          if cellInfo.is_some() && cellInfo.getParent().getParent().getLevel() == sameCellLevel {
            // java if (!RANGEFactory.isSubset(cellInfo.getRange(), changeRange))
            if !RANGEFactory::isSubset(cellInfo.getRange(), changeRange) {
              // java return false;
              return false;
            }
            // java else
            else {
              // java int i = 0;
              let mut i: i32 = 0;
              let mut size: i32 = (cellInfo.len() as i32);
              // java for(int size = cellInfo.size(); i < size; ++i)
              while i < size {
                // java if (RANGEFactory.isIntersect(cellInfo.getTableByIndex(i).getRange(), changeRange))
                if RANGEFactory::isIntersect__long_long(cellInfo.getTableByIndex(i).getRange(), changeRange) {
                  // java return false;
                  return false;
                }
              }
              // java return true;
              return true;
            }
          }
          // java else
          else {
            // java return false;
            return false;
          }
        }
      }
    }

    // java private CellInfo getDeepestCell(int cp)
    pub fn getDeepestCell(&self, cp: i32) -> Option<CellInfo> {
      // java TableInfo rootTable = this.getTableByCP(cp);
      let mut rootTable: TableInfo = self.super_.getTableByCP(cp);
      // java if (null == rootTable)
      if rootTable.is_none() {
        // java return null;
        return None;
      }
      // java else
      else {
        // java RowInfo row = rootTable.getRowByCP(cp);
        let mut row: RowInfo = rootTable.getRowByCP(cp);
        // java CellInfo cell = row.getCellByCP(cp);
        let mut cell: CellInfo = row.getCellByCP(cp);
        // java if (null == cell)
        if cell.is_none() {
          // java return null;
          return None;
        }
        // java else
        else {
          let mut childTable: TableInfo = cell.getTableByCP(cp);
          // java for(TableInfo childTable = cell.getTableByCP(cp); null != childTable childTable = cell.getTableByCP(cp))
          while childTable.is_some() {
            // java row = childTable.getRowByCP(cp);
            row = childTable.getRowByCP(cp);
            // java cell = row.getCellByCP(cp);
            cell = row.getCellByCP(cp);
            // java if (null == cell)
            if cell.is_none() {
              // java return null;
              return None;
            }
          }
          // java return cell;
          return cell;
        }
      }
    }

    // java private TableNode getRootNode(int[] offsetBuf)
    pub fn getRootNode(&mut self, offsetBuf: &Vec<i32>) -> Option<TableNode> {
      // java int start = RANGEFactory.getStart(this.mRange);
      let mut start: i32 = RANGEFactory::getStart(self.mRange);
      // java int end = RANGEFactory.getEnd(this.mRange);
      let mut end: i32 = RANGEFactory::getEnd(self.mRange);
      // java int rootLevel = this.mTableManager.getMinLevel(start, end);
      let mut rootLevel: i32 = self.mTableManager.getMinLevel(start, end);
      // java int minLevel = Math.max(1, rootLevel);
      let mut minLevel: i32 = Math::max(1, rootLevel);
      // java start = Math.min(start, this.mTableManager.getRowStart(start, minLevel));
      start = Math::min(start, self.mTableManager.getRowStart(start, minLevel));
      // java end = Math.max(end, this.mTableManager.getRowEnd(end - 1, minLevel));
      end = Math::max(end, self.mTableManager.getRowEnd(end - 1, minLevel));
      // java CellInfoNode rootCell = this.getRootCell(start, minLevel, offsetBuf);
      let mut rootCell: CellInfoNode = self.getRootCell(start, minLevel, &offsetBuf);
      // java if (rootCell != null && !rootCell.isHead())
      if rootCell.is_some() && !rootCell.isHead() {
        // java TableInfo startTable = rootCell.getTableByCP(start);
        let mut startTable: TableInfo = rootCell.getTableByCP(start);
        // java if (startTable != null)
        if startTable.is_some() {
          // java RowInfo startRow = startTable.getRowByCP(start);
          let mut startRow: RowInfo = startTable.getRowByCP(start);
          // java if (startRow != null)
          if startRow.is_some() {
            // java start = startRow.getStartCP();
            start = startRow.getStartCP();
          }
        }
        // java TableInfo endTable = rootCell.getTableByCP(end - 1);
        let mut endTable: TableInfo = rootCell.getTableByCP(end - 1);
        // java if (endTable != null)
        if endTable.is_some() {
          // java RowInfo endRow = endTable.getRowByCP(end - 1);
          let mut endRow: RowInfo = endTable.getRowByCP(end - 1);
          // java if (endRow != null)
          if endRow.is_some() {
            // java end = endRow.getEndCP();
            end = endRow.getEndCP();
          }
        }
        // java if (startTable == endTable && startTable != null)
        if startTable == endTable && startTable.is_some() {
          // java if (rootLevel == 0)
          if rootLevel == 0 {
            // java end = startTable.getEndCP();
            end = startTable.getEndCP();
          }
          // java else if (start != startTable.getStartCP() && end != startTable.getEndCP())
          else if start != startTable.getStartCP() && end != startTable.getEndCP() {
            // java this.mRange = RANGEFactory.getRANGE(start, end);
            self.mRange = RANGEFactory::getRANGE(start, end);
            // java return (TableNode)startTable;
            return (startTable).downcast_ref::<TableNode>().unwrap();
          }
        }
        // java this.changeRange(RANGEFactory.getRANGE(start, end), false);
        self.changeRange(RANGEFactory::getRANGE(start, end), false);
        // java return rootCell;
        return rootCell;
      }
      // java else
      else {
        // java return null;
        return None;
      }
    }

    // java private CellInfoNode getRootCell(int cp, int level, int[] offsetBuf)
    pub fn getRootCell(&self, cp: i32, level: i32, offsetBuf: &Vec<i32>) -> Option<CellInfoNode> {
      // java if (level == 1)
      if level == 1 {
        // java return this;
        return self;
      }
      // java else
      else {
        // java --level;
        { let _tmp = level; level -= 1; _tmp };
        // java TableInfoNode tableInfoNode;
        let mut tableInfoNode: TableInfoNode = TableInfoNode::default();
        // java TableNode cellInfoNode;
        let mut cellInfoNode: TableNode = TableNode::default();
        tableInfoNode = (self.super_.getNodeStrict(cp, &offsetBuf)).downcast_ref::<TableInfoNode>().unwrap();
        // java for(tableInfoNode = (TableInfoNode)this.getNodeStrict(cp, offsetBuf) tableInfoNode != null && tableInfoNode.getLevel() != level tableInfoNode = (TableInfoNode)cellInfoNode.getNodeStrict(offsetBuf[0], offsetBuf))
        while tableInfoNode.is_some() && tableInfoNode.getLevel() != level {
          // java TableNode rowInfoNode = tableInfoNode.getNodeByOffset(offsetBuf[0], offsetBuf);
          let mut rowInfoNode: TableNode = tableInfoNode.getNodeByOffset(offsetBuf[0usize], offsetBuf);
          // java cellInfoNode = rowInfoNode.getNodeByOffset(offsetBuf[0], offsetBuf);
          cellInfoNode = rowInfoNode.getNodeByOffset(offsetBuf[0usize], offsetBuf);
        }
        // java if (tableInfoNode == null)
        if tableInfoNode.is_none() {
          // java return null;
          return None;
        }
        // java else
        else {
          // java TableNode rowInfoNode = tableInfoNode.getNodeByOffset(offsetBuf[0], offsetBuf);
          let mut rowInfoNode: TableNode = tableInfoNode.getNodeByOffset(offsetBuf[0usize], offsetBuf);
          // java cellInfoNode = rowInfoNode.getNodeByOffset(offsetBuf[0], offsetBuf);
          cellInfoNode = rowInfoNode.getNodeByOffset(offsetBuf[0usize], offsetBuf);
          // java return (CellInfoNode)cellInfoNode;
          return (cellInfoNode).downcast_ref::<CellInfoNode>().unwrap();
        }
      }
    }

    // java private void tryAppend()
    pub fn tryAppend(&mut self) {
      // java if (this.mInvalidCP < this.mDocument.getLength() && this.mTableManager.isInTable(this.mInvalidCP, 1))
      if self.mInvalidCP < self.mDocument.getLength() && self.mTableManager.isInTable(self.mInvalidCP, 1) {
        // java TableInfoNode lastTable = (TableInfoNode)this.mTree.rightmost();
        let mut lastTable: TableInfoNode = (self.mTree.rightmost()).downcast_ref::<TableInfoNode>().unwrap();
        // java if (!lastTable.isHead() && lastTable.getEndCP() == this.mInvalidCP)
        if !lastTable.isHead() && lastTable.getEndCP() == self.mInvalidCP {
          // java lastTable = lastTable.tryAppend();
          lastTable = lastTable.tryAppend();
          // java this.mInvalidCP = lastTable.getEndCP();
          self.mInvalidCP = lastTable.getEndCP();
          // java while (this.mTableManager.isInTable(this.mInvalidCP, 1))
          while self.mTableManager.isInTable(self.mInvalidCP, 1) {
            // java ITableManager.IRowEntry row = this.mTableManager.createRow(this.mInvalidCP, 1);
            let mut row: ITableManager::IRowEntry = self.mTableManager.createRow__int_int(self.mInvalidCP, 1);
            // java TableInfoNode newTable = (TableInfoNode)this.createSubNode();
            let mut newTable: TableInfoNode = (self.super_.createSubNode()).downcast_ref::<TableInfoNode>().unwrap();
            // java row = newTable.init(row, 1, true, Integer.MAX_VALUE);
            row = newTable.init(row, 1, true, Integer::MAX_VALUE);
            // java this.mTree.insertRight((TableInfoNode)this.mTree.rightmost(), newTable, this.mInvalidCP);
            self.mTree.insertRight((self.mTree.rightmost()).downcast_ref::<TableInfoNode>().unwrap(), newTable, self.mInvalidCP);
            // java this.mInvalidCP = row.getEnd();
            self.mInvalidCP = row.getEnd();
            // java row.recycle();
            row.recycle();
          }
        }
      }
    }

}

impl std::ops::Deref for TreeTableCache {
    type Target = CellInfoNode;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TreeTableCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java private class AutoResizeRange
impl AutoResizeRange {
    // java   AutoResizeRange()
    pub fn AutoResizeRange() -> Self {
      Self { mRangeTimely: Default::default(), mRangeDelay: Default::default(), mLastChangeRangeInSameCell: Default::default(), mNodeChanged: Default::default() }
    }

    // java  long popRangeTimely()
    pub fn popRangeTimely(&mut self) -> i64 {
      // java long range = this.mRangeTimely;
      let mut range: i64 = self.mRangeTimely;
      // java this.mRangeTimely = 0L;
      self.mRangeTimely = 0;
      // java return range;
      return range;
    }

    // java  long popRangeDelay()
    pub fn popRangeDelay(&mut self) -> i64 {
      // java long range = RANGEFactory.include(this.mRangeDelay, this.popRangeTimely());
      let mut range: i64 = RANGEFactory::include__long_int(self.mRangeDelay, self.popRangeTimely());
      // java this.mRangeDelay = 0L;
      self.mRangeDelay = 0;
      // java return range;
      return range;
    }

    // java  long popLastChangeRangeInSameCell()
    pub fn popLastChangeRangeInSameCell(&mut self) -> i64 {
      // java long range = this.mLastChangeRangeInSameCell;
      let mut range: i64 = self.mLastChangeRangeInSameCell;
      // java this.mLastChangeRangeInSameCell = 0L;
      self.mLastChangeRangeInSameCell = 0;
      // java return range;
      return range;
    }

    // java  void mergeRangFormRebuildCache()
    pub fn mergeRangFormRebuildCache(&mut self) {
      // java this.mRangeTimely = RANGEFactory.include(this.mRangeTimely, TreeTableCache.this.getRebuildCache().getInvalidRange());
      self.mRangeTimely = RANGEFactory::include__long_int(self.mRangeTimely, self.getRebuildCache().getInvalidRange());
      // java if (TreeTableCache.this.getRebuildCache().getInvalidCount() > 1L && !RANGEFactory.isEmpty(TreeTableCache.this.mRange))
      if self.getRebuildCache().getInvalidCount() > 1 && !RANGEFactory::isEmpty(self.mRange) {
        // java this.mRangeTimely = RANGEFactory.include(this.mRangeTimely, TreeTableCache.this.mRange);
        self.mRangeTimely = RANGEFactory::include__long_int(self.mRangeTimely, self.mRange);
      }
    }

    // java  long finishRangeTimely()
    pub fn finishRangeTimely(&mut self) -> i64 {
      // java long range = this.intersectAndInvalidGrid(this.popRangeTimely());
      let mut range: i64 = self.intersectAndInvalidGrid(self.popRangeTimely());
      // java return range;
      return range;
    }

    // java  long finishRangeDelay()
    pub fn finishRangeDelay(&mut self) -> i64 {
      // java long range = this.intersectAndInvalidGrid(this.popRangeDelay());
      let mut range: i64 = self.intersectAndInvalidGrid(self.popRangeDelay());
      // java return range;
      return range;
    }

    // java  long intersectAndInvalidGrid(long changeRange)
    pub fn intersectAndInvalidGrid(&mut self, changeRange: i64) -> Option<i64> {
      // java long lastChangeRangeInSameCell = this.popLastChangeRangeInSameCell();
      let mut lastChangeRangeInSameCell: i64 = self.popLastChangeRangeInSameCell();
      // java boolean nodeChanged = this.mNodeChanged;
      let mut nodeChanged: bool = self.mNodeChanged;
      // java this.mNodeChanged = false;
      self.mNodeChanged = false;
      // java if (RANGEFactory.isEmpty(changeRange))
      if RANGEFactory::isEmpty(changeRange) {
        // java return 0L;
        return 0;
      }
      // java else if (lastChangeRangeInSameCell != 0L && lastChangeRangeInSameCell == changeRange)
      else if lastChangeRangeInSameCell != 0 && lastChangeRangeInSameCell == changeRange {
        // java TableInfo tableInfo = TreeTableCache.this.getTableByCP(RANGEFactory.getStart(changeRange));
        let mut tableInfo: TableInfo = self.getTableByCP(RANGEFactory::getStart(changeRange));
        // java return tableInfo != null && tableInfo.isTableFit() ? changeRange : 0L;
        return (if tableInfo.is_some() && tableInfo.isTableFit() { Some(changeRange) } else { Some(0) });
      }
      // java else
      else {
        // java int tblSize = TreeTableCache.this.mTree.size();
        let mut tblSize: i32 = (self.mTree.len() as i32);
        // java if (tblSize == 0)
        if tblSize == 0 {
          // java return 0L;
          return 0;
        }
        // java else
        else {
          // java int changeStart = RANGEFactory.getStart(changeRange);
          let mut changeStart: i32 = RANGEFactory::getStart(changeRange);
          // java int changeEnd = RANGEFactory.getEnd(changeRange);
          let mut changeEnd: i32 = RANGEFactory::getEnd(changeRange);
          // java if (nodeChanged)
          if nodeChanged {
            // java TableInfo tableInfo = TreeTableCache.this.getTableByCP(changeStart - 1);
            let mut tableInfo: TableInfo = self.getTableByCP(changeStart - 1);
            // java if (tableInfo != null && tableInfo.isTableFit() && tableInfo.getEndCP() == changeStart)
            if tableInfo.is_some() && tableInfo.isTableFit() && tableInfo.getEndCP() == changeStart {
              // java --changeStart;
              { let _tmp = changeStart; changeStart -= 1; _tmp };
            }
            // java CellInfo cellInfo = TreeTableCache.this.getDeepestCell(changeStart);
            let mut cellInfo: CellInfo = self.getDeepestCell(changeStart);
            // java if (cellInfo != null && cellInfo.getParent().getParent().isTableFit() && RANGEFactory.isSubset(cellInfo.getRange(), changeRange))
            if cellInfo.is_some() && cellInfo.getParent().getParent().isTableFit() && RANGEFactory::isSubset(cellInfo.getRange(), changeRange) {
              // java changeRange = RANGEFactory.include(changeRange, cellInfo.getParent().getEndCP() - 1);
              changeRange = RANGEFactory::include__long_int(changeRange, cellInfo.getParent().getEndCP() - 1);
              // java changeStart = RANGEFactory.getStart(changeRange);
              changeStart = RANGEFactory::getStart(changeRange);
              // java changeEnd = RANGEFactory.getEnd(changeRange);
              changeEnd = RANGEFactory::getEnd(changeRange);
            }
          }
          // java tblSize = TreeTableCache.this.mTree.size();
          tblSize = (self.mTree.len() as i32);
          // java if (tblSize == 0)
          if tblSize == 0 {
            // java return 0L;
            return 0;
          }
          // java else
          else {
            // java int firstStart = ((TableInfo)TreeTableCache.this.mTree.get(0)).getStartCP();
            let mut firstStart: i32 = (self.mTree.get(0)).downcast_ref::<TableInfo>().unwrap().getStartCP();
            // java int lastEnd = ((TableInfo)TreeTableCache.this.mTree.get(tblSize - 1)).getEndCP();
            let mut lastEnd: i32 = (self.mTree.get(tblSize - 1)).downcast_ref::<TableInfo>().unwrap().getEndCP();
            // java if (lastEnd > changeStart && firstStart < changeEnd)
            if lastEnd > changeStart && firstStart < changeEnd {
              // java if (firstStart >= changeStart)
              if firstStart >= changeStart {
                // java changeStart = firstStart;
                changeStart = firstStart;
              }
              // java else
              else {
                // java int nextRowStart = TreeTableCache.this.mTableManager.findNextRowStart(changeStart, changeEnd, 1);
                let mut nextRowStart: i32 = self.mTableManager.findNextRowStart(changeStart, changeEnd, 1);
                // java if (nextRowStart > changeStart)
                if nextRowStart > changeStart {
                  // java changeStart = nextRowStart;
                  changeStart = nextRowStart;
                }
              }
              // java changeStart = Math.max(changeStart, firstStart);
              changeStart = Math::max(changeStart, firstStart);
              // java changeEnd = Math.min(changeEnd, lastEnd);
              changeEnd = Math::min(changeEnd, lastEnd);
              // java if (changeEnd <= changeStart)
              if changeEnd <= changeStart {
                // java return 0L;
                return 0;
              }
              // java else
              else {
                // java TableInfo table = TreeTableCache.this.getTableByCP(changeStart);
                let mut table: TableInfo = self.getTableByCP(changeStart);
                // java boolean findHasFit;
                let mut findHasFit: bool;
                // java int tableIndex;
                let mut tableIndex: i32;
                findHasFit = false;
                // java for(findHasFit = false table != null && table.getStartCP() < changeEnd table = (TableInfo)TreeTableCache.this.mTree.get(tableIndex + 1))
                while table.is_some() && table.getStartCP() < changeEnd {
                  // java Data.TableData data = table.getData();
                  let mut data: crate::cn::wps::moffice::writer::data::table::Data::TableData = table.getData();
                  // java if (data.mGridCols != null)
                  if data.mGridCols.is_some() {
                    // java data.mGridCols.invalid();
                    data.mGridCols.invalid();
                  }
                  // java if (table.isTableFit() && !findHasFit)
                  if table.isTableFit() && !findHasFit {
                    // java findHasFit = true;
                    findHasFit = true;
                    // java tableIndex = table.getStartCP();
                    tableIndex = table.getStartCP();
                    // java if (tableIndex > changeStart)
                    if tableIndex > changeStart {
                      // java changeStart = tableIndex;
                      changeStart = tableIndex;
                    }
                  }
                  // java tableIndex = ((TableInfoNode)table).index();
                  tableIndex = (table).downcast_ref::<TableInfoNode>().unwrap().index();
                  // java if (tableIndex >= tblSize - 1)
                  if tableIndex >= tblSize - 1 {
                    // java break;
                    break;
                  }
                }
                // java if (findHasFit && changeEnd > changeStart)
                if findHasFit && changeEnd > changeStart {
                  // java int nextRowStart = TreeTableCache.this.mTableManager.findNextRowStart(changeStart, changeEnd, 1);
                  let mut nextRowStart: i32 = self.mTableManager.findNextRowStart(changeStart, changeEnd, 1);
                  // java if (nextRowStart > changeStart)
                  if nextRowStart > changeStart {
                    // java changeStart = nextRowStart;
                    changeStart = nextRowStart;
                  }
                  // java if (changeEnd <= changeStart)
                  if changeEnd <= changeStart {
                    // java return 0L;
                    return 0;
                  }
                  // java else
                  else {
                    // java changeRange = RANGEFactory.include(0L, changeStart, changeEnd);
                    changeRange = RANGEFactory::include__long_int_int(0, changeStart, changeEnd);
                    // java return changeRange;
                    return changeRange;
                  }
                }
                // java else
                else {
                  // java return 0L;
                  return 0;
                }
              }
            }
            // java else
            else {
              // java return 0L;
              return 0;
            }
          }
        }
      }
    }

}
