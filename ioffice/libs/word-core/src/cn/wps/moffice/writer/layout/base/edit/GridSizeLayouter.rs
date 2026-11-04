// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;
// Java: import cn.wps.base.log.Log
use util::base::log::Log::Log;
// Java: import cn.wps.kfc.api.list.IIntList
use util::kfc::api::list::IIntList::IIntList;
// Java: import cn.wps.moffice.writer.cache.TypoDocument
use crate::cn::wps::moffice::writer::cache::TypoDocument::TypoDocument;
// Java: import cn.wps.moffice.writer.cache.TypoDrawing
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
// Java: import cn.wps.moffice.writer.cache.TypoMemoryObj
use crate::cn::wps::moffice::writer::cache::TypoMemoryObj::TypoMemoryObj;
// Java: import cn.wps.moffice.writer.cache.TypoRect
use crate::cn::wps::moffice::writer::cache::TypoRect::TypoRect;
// Java: import cn.wps.moffice.writer.cache.TypoRectMemoryImpl
use crate::cn::wps::moffice::writer::cache::TypoRectMemoryImpl::TypoRectMemoryImpl;
// Java: import cn.wps.moffice.writer.cache.TypoSnapshot
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
// Java: import cn.wps.moffice.writer.cache.TypoTableCell
use crate::cn::wps::moffice::writer::cache::TypoTableCell::TypoTableCell;
// Java: import cn.wps.moffice.writer.cache.TypoToolsManager
use crate::cn::wps::moffice::writer::cache::TypoToolsManager::TypoToolsManager;
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::IPapxTable;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::RANGEFactory;
// Java: import cn.wps.moffice.writer.data.table.CellInfo
use crate::cn::wps::moffice::writer::data::table::CellInfo::CellInfo;
// Java: import cn.wps.moffice.writer.data.table.Data
use crate::cn::wps::moffice::writer::data::table::Data::Data;
// Java: import cn.wps.moffice.writer.data.table.RowInfo
use crate::cn::wps::moffice::writer::data::table::RowInfo::RowInfo;
// Java: import cn.wps.moffice.writer.data.table.TableInfo
use crate::cn::wps::moffice::writer::data::table::TableInfo::TableInfo;
// Java: import cn.wps.moffice.writer.data.table.TableInfos
use crate::cn::wps::moffice::writer::data::table::TableInfos::TableInfos;
// Java: import cn.wps.moffice.writer.data.table.grid.GridAutoSize
use crate::cn::wps::moffice::writer::data::table::grid::GridAutoSize::GridAutoSize;
// Java: import cn.wps.moffice.writer.data.table.grid.GridCols
use crate::cn::wps::moffice::writer::data::table::grid::GridCols::GridCols;
// Java: import cn.wps.moffice.writer.data.table.grid.GridSizeCommit
use crate::cn::wps::moffice::writer::data::table::grid::GridSizeCommit::GridSizeCommit;
// Java: import cn.wps.moffice.writer.data.table.tree.TableInfoNode
use crate::cn::wps::moffice::writer::data::table::tree::TableInfoNode::TableInfoNode;
// Java: import cn.wps.moffice.writer.data.value.PageSetup
use crate::cn::wps::moffice::writer::data::value::PageSetup::PageSetup;
// Java: import cn.wps.moffice.writer.layout.base.ParaLayouter
use crate::cn::wps::moffice::writer::layout::base::ParaLayouter::ParaLayouter;
// Java: import cn.wps.moffice.writer.layout.base.drawing.DrawingResizeTool
use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingResizeTool::DrawingResizeTool;
// Java: import cn.wps.moffice.writer.layout.base.env.DocumentEnv
use crate::cn::wps::moffice::writer::layout::base::env::DocumentEnv::DocumentEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.LayoutState
use crate::cn::wps::moffice::writer::layout::base::env::LayoutState::LayoutState;
// Java: import cn.wps.moffice.writer.layout.base.env.MainState
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
// Java: import cn.wps.moffice.writer.layout.base.env.PageEnv
use crate::cn::wps::moffice::writer::layout::base::env::PageEnv::PageEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.SuperEnvoy
use crate::cn::wps::moffice::writer::layout::base::env::SuperEnvoy::SuperEnvoy;
// Java: import cn.wps.moffice.writer.layout.base.env.ViewEnv
use crate::cn::wps::moffice::writer::layout::base::env::ViewEnv::ViewEnv;
// Java: import cn.wps.moffice.writer.layout.base.graph.GraphBuff
use crate::cn::wps::moffice::writer::layout::base::graph::GraphBuff::GraphBuff;
// Java: import cn.wps.moffice.writer.layout.base.graph.GraphBuffFactory
use crate::cn::wps::moffice::writer::layout::base::graph::GraphBuffFactory::GraphBuffFactory;
// Java: import cn.wps.moffice.writer.layout.base.table.TableCellParaLayouter
use crate::cn::wps::moffice::writer::layout::base::table::TableCellParaLayouter::TableCellParaLayouter;
// Java: import cn.wps.moffice.writer.layout.base.table.Tools
use crate::cn::wps::moffice::writer::layout::base::table::Tools::Tools;
// Java: import cn.wps.moffice.writer.layout.base.table.width.FastGraphImpl
use crate::cn::wps::moffice::writer::layout::base::table::width::FastGraphImpl::FastGraphImpl;
// Java: import cn.wps.moffice.writer.layout.base.table.width.IWidthInfo
use crate::cn::wps::moffice::writer::layout::base::table::width::IWidthInfo::IWidthInfo;
// Java: import cn.wps.moffice.writer.layout.base.table.width.IWidthReceiver
use crate::cn::wps::moffice::writer::layout::base::table::width::IWidthReceiver::IWidthReceiver;
// Java: import cn.wps.moffice.writer.layout.base.table.width.WidthAnalyzer
use crate::cn::wps::moffice::writer::layout::base::table::width::WidthAnalyzer::WidthAnalyzer;
// Java: import java.util.ArrayList
// Java: import org.apache.poi.poifs.filesystem.Entry
use io_poi_compact::org::apache::poi::poifs::filesystem::Entry::Entry;

// java private static final String Tag = GridSizeLayouter.class.getSimpleName();
const Tag: i32 = todo!().getSimpleName();
// java private static AutoSizeCommitImpl sInstance;
static sInstance: std::sync::OnceLock<i32> =
                std::sync::OnceLock::new();

// java public class GridSizeLayouter
#[derive(Default)]
pub struct GridSizeLayouter {
    // java private MainState mMainState = new MainState();
    mMainState: Option<MainStateHolder>,
    // java private DocumentEnv mDocEnv;
    mDocEnv: Option<DocumentEnvHolder>,
    // java private ViewEnv mViewEnv;
    mViewEnv: Option<ViewEnvHolder>,
    // java private LayoutState mLayoutState;
    mLayoutState: Option<LayoutStateHolder>,
    // java private ParaLayouter mParaLayouter;
    mParaLayouter: Option<ParaLayouterHolder>,
    // java private GraphBuff mCacheGraphBuff;
    mCacheGraphBuff: Option<GraphBuffHolder>,
    // java private FastGraphImpl mFastGraph;
    mFastGraph: Option<FastGraphImplHolder>,
    // java private boolean mReTypo = false;
    mReTypo: bool,
    // java private boolean mForceFit = false;
    mForceFit: bool,
    // java private EditingWidthReceiver mWidthReceiver = new EditingWidthReceiver();
    mWidthReceiver: Option<EditingWidthReceiverHolder>,
}

// java private class EditingWidthReceiver implements IWidthReceiver
#[derive(Default)]
pub struct EditingWidthReceiver {
    // java private CellInfo mCellInfo;
    mCellInfo: Option<CellInfoHolder>,
    // java private Data.AutoFitInfo mCacheFitInfo;
    mCacheFitInfo: Option<Data_AutoFitInfoHolder>,
    // java private FastInputCache mFastInputCache;
    mFastInputCache: Option<FastInputCacheHolder>,
    // java private boolean mReuseCacheAcived;
    mReuseCacheAcived: bool,
    // java private boolean mIsInTableHeader;
    mIsInTableHeader: bool,
}

// java private class FastInputCache
#[derive(Default)]
pub struct FastInputCache {
    // java int mCellStartCp;
    pub(crate) mCellStartCp: i32,
    // java long mParaRange;
    pub(crate) mParaRange: i32,
}

// java public static class AutoSizeCommitImpl implements GridSizeCommit
#[derive(Default)]
pub struct AutoSizeCommitImpl {
    // java private boolean mUsing = false;
    mUsing: bool,
    // java private Document mDocument;
    mDocument: Option<DocumentHolder>,
    // java private ArrayList<GridSizeCommit> mList;
    mList: Vec<Option<GridSizeCommitHolder>>,
    // java private GridCols mWaitCommitcGridCols = null;
    mWaitCommitcGridCols: Option<GridColsHolder>,
    // java private int mTypoCoreWidth;
    mTypoCoreWidth: i32,
}

// java public class GridSizeLayouter
impl GridSizeLayouter {
    // java public  GridSizeLayouter(DocumentEnv docEnv, ViewEnv viewEnv)
    pub fn GridSizeLayouter(docEnv: &DocumentEnv, viewEnv: &ViewEnv) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mDocEnv = docEnv;
      Self { mMainState: Default::default(), mDocEnv: docEnv, mViewEnv: viewEnv, mLayoutState: self.createLayoutState(), mParaLayouter: TableCellParaLayouter::new(self.mDocEnv, self.mViewEnv, self.mMainState), mCacheGraphBuff: Default::default(), mFastGraph: Default::default(), mReTypo: Default::default(), mForceFit: Default::default(), mWidthReceiver: Default::default() }
      // java this.mViewEnv = viewEnv;
      // java this.mLayoutState = this.createLayoutState();
      // java this.mParaLayouter = new TableCellParaLayouter(this.mDocEnv, this.mViewEnv, this.mMainState);
      */
    }

    // java private LayoutState createLayoutState()
    pub fn createLayoutState(&self) -> Option<LayoutStateHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java LayoutState newEnv = new LayoutState((SuperEnvoy)null);
      let mut newEnv: LayoutState = LayoutState::new(None);
      // java newEnv.document = this.mDocEnv.textDocument.getMainDocument();
      newEnv.document = self.mDocEnv.textDocument.getMainDocument();
      // java return newEnv;
      return { let __x: Option<LayoutStateHolder> = todo!("no align strategy matched"); __x };
      */
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mCacheGraphBuff != null)
      if let Some(mCacheGraphBuff) = self.mCacheGraphBuff.as_mut() {
        // java this.mCacheGraphBuff.dispose();
        mCacheGraphBuff.dispose();
        // java this.mCacheGraphBuff = null;
        mCacheGraphBuff = None;
      }
      // java if (this.mFastGraph != null)
      if let Some(mFastGraph) = self.mFastGraph.as_mut() {
        // java this.mFastGraph.dispose();
        mFastGraph.dispose();
        // java this.mFastGraph = null;
        mFastGraph = None;
      }
      // java if (this.mParaLayouter != null)
      if let Some(mParaLayouter) = self.mParaLayouter.as_mut() {
        // java this.mParaLayouter.dispose();
        mParaLayouter.dispose();
        // java this.mParaLayouter = null;
        mParaLayouter = None;
      }
      // java if (this.mLayoutState != null)
      if let Some(mLayoutState) = self.mLayoutState.as_mut() {
        // java this.mLayoutState.dispose();
        mLayoutState.dispose();
        // java this.mLayoutState = null;
        mLayoutState = None;
      }
      // java if (this.mMainState != null)
      if let Some(mMainState) = self.mMainState.as_mut() {
        // java if (this.mMainState.snapshot != null)
        if mMainState.snapshot.is_some() {
          // java this.mMainState.snapshot.release();
          mMainState.snapshot.release();
        }
        // java this.mMainState = null;
        mMainState = None;
      }
      */
    }

    // java public GridSizeCommit doGridAutoResize(long changeRange, Document document, boolean reTypo, int typoCoreWidth)
    pub fn doGridAutoResize(&self, changeRange: i32, document: Option<DocumentHolder>, reTypo: bool, typoCoreWidth: i32) -> Option<GridSizeCommitHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int changeStart = RANGEFactory.getStart(changeRange);
      let mut changeStart: i32 = RANGEFactory::getStart(changeRange);
      // java TableInfos tableInfos = document.getTableInfos();
      let mut tableInfos: Box<dyn TableInfos> = document.getTableInfos();
      // java TableInfo tableInfo = tableInfos.getTableInfoByLevel(changeStart, 1);
      let mut tableInfo: TableInfo = tableInfos.getTableInfoByLevel(changeStart, 1);
      // java if (tableInfo == null)
      if false {
        // java return null;
        return None;
      }
      // java else
      else {
        // java RowInfo rowInfo = tableInfo.getRowByCP(changeStart);
        let mut rowInfo: RowInfo = tableInfo.getRowByCP(changeStart);
        // java CellInfo cellInfo = rowInfo.getCellByCP(changeStart);
        let mut cellInfo: CellInfo = rowInfo.getCellByCP(changeStart);
        // java if (cellInfo == null)
        if false {
          // java cellInfo = rowInfo.getCellByIndex(0);
          cellInfo = rowInfo.getCellByIndex(0);
          // java changeStart = cellInfo.getStartCP();
          changeStart = cellInfo.getStartCP();
          // java changeRange = RANGEFactory.include(changeRange, changeStart);
          changeRange = RANGEFactory::include__long_int(changeRange, changeStart);
        }
        // java AutoSizeCommitImpl autoSizeCommit = GridSizeLayouter.AutoSizeCommitImpl.newInstance().init(document, typoCoreWidth);
        let mut autoSizeCommit: AutoSizeCommitImpl = { let __x: AutoSizeCommitImpl = todo!("no align strategy matched"); __x };
        // java this.initEnv(document, tableInfo);
        self.initEnv(document, { let __x: Option<TableInfoHolder> = todo!("no align strategy matched"); __x });
            // java this.mReTypo = reTypo;
            self.mReTypo = reTypo;
            // java this.mForceFit = typoCoreWidth > 0;
            self.mForceFit = typoCoreWidth > 0;
            // java boolean bContinue = true;
            let mut bContinue: bool = true;
            // java if (!tableInfo.isTableFit() && !this.mForceFit)
            if !tableInfo.isTableFit() && !self.mForceFit {
              // java if (RANGEFactory.getEnd((long)tableInfo.getEndCP()) > RANGEFactory.getEnd(changeRange))
              if RANGEFactory::getEnd(((tableInfo.getEndCP()) as i64)) > RANGEFactory::getEnd(changeRange) {
                // java bContinue = false;
                bContinue = false;
              }
            }
            // java else
            else {
              // java TableInfo deepTable = tableInfos.getTableInfoByRange(changeStart, changeStart + 1);
              let mut deepTable: TableInfo = tableInfos.getTableInfoByRange(changeStart, changeStart + 1);
              // java if (deepTable != null && deepTable.getLevel() > tableInfo.getLevel())
              if true && deepTable.getLevel() > tableInfo.getLevel() {
                // java bContinue = this.recalcCellSizeInDeepTable(deepTable, cellInfo, changeRange, autoSizeCommit);
                bContinue = self.recalcCellSizeInDeepTable({ let __x: Option<TableInfoHolder> = todo!("no align strategy matched"); __x }, { let __x: Option<CellInfoHolder> = todo!("no align strategy matched"); __x }, changeRange, { let __x: Option<AutoSizeCommitImplHolder> = todo!("no align strategy matched"); __x });
              }
              // java else
              else {
                // java bContinue = this.recalcCellSize(cellInfo, changeRange, (TableInfo)null, autoSizeCommit);
                bContinue = self.recalcCellSize({ let __x: Option<CellInfoHolder> = todo!("no align strategy matched"); __x }, changeRange, None, { let __x: Option<AutoSizeCommitImplHolder> = todo!("no align strategy matched"); __x });
              }
              // java if (bContinue)
              if bContinue {
                // java bContinue = this.recalcCellSizeInRow(rowInfo, cellInfo, changeRange, autoSizeCommit);
                bContinue = self.recalcCellSizeInRow({ let __x: Option<RowInfoHolder> = todo!("no align strategy matched"); __x }, { let __x: Option<CellInfoHolder> = todo!("no align strategy matched"); __x }, changeRange, { let __x: Option<AutoSizeCommitImplHolder> = todo!("no align strategy matched"); __x });
                // java if (bContinue)
                if bContinue {
                  // java bContinue = this.recalcCellInfoInTable(tableInfo, rowInfo, changeRange, autoSizeCommit);
                  bContinue = self.recalcCellInfoInTable({ let __x: Option<TableInfoHolder> = todo!("no align strategy matched"); __x }, { let __x: Option<RowInfoHolder> = todo!("no align strategy matched"); __x }, changeRange, { let __x: Option<AutoSizeCommitImplHolder> = todo!("no align strategy matched"); __x });
                }
              }
            }
            // java if (bContinue)
            if bContinue {
              // java this.recalcCellSizeInTables(tableInfos, tableInfo, changeRange, autoSizeCommit);
              self.recalcCellSizeInTables({ let __x: Option<TableInfosHolder> = todo!("no align strategy matched"); __x }, { let __x: Option<TableInfoHolder> = todo!("no align strategy matched"); __x }, changeRange, { let __x: Option<AutoSizeCommitImplHolder> = todo!("no align strategy matched"); __x });
            }
              // java this.closeEnv();
              self.closeEnv();
        // java return autoSizeCommit.finish();
        return autoSizeCommit.finish();
      }
      */
    }

    // java private boolean recalcCellSizeInDeepTable(TableInfo deepTable, CellInfo rootCell, long changeRange, AutoSizeCommitImpl autoSizeCommit)
    pub fn recalcCellSizeInDeepTable(&self, deepTable: Option<TableInfoHolder>, rootCell: Option<CellInfoHolder>, changeRange: i32, autoSizeCommit: Option<AutoSizeCommitImplHolder>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int changeStart = RANGEFactory.getStart(changeRange);
      let mut changeStart: i32 = RANGEFactory::getStart(changeRange);
      // java RowInfo row = deepTable.getRowByCP(changeStart);
      let mut row: RowInfo = deepTable.getRowByCP(changeStart);
      // java CellInfo cell = row.getCellByCP(changeStart);
      let mut cell: CellInfo = row.getCellByCP(changeStart);
      // java if (cell == null)
      if false {
        // java cell = row.getCellByIndex(0);
        cell = row.getCellByIndex(0);
        // java changeRange = RANGEFactory.include(changeRange, cell.getStartCP());
        changeRange = RANGEFactory::include__long_int(changeRange, cell.getStartCP());
      }
      // java boolean bContinue = true;
      let mut bContinue: bool = true;
      // java if (cell != null)
      if true {
        // java bContinue = this.recalcCellSize(cell, changeRange, (TableInfo)null, autoSizeCommit);
        bContinue = self.recalcCellSize({ let __x: Option<CellInfoHolder> = todo!("no align strategy matched"); __x }, changeRange, None, autoSizeCommit);
      }
      // java if (bContinue)
      if bContinue {
        // java bContinue = this.recalcCellSizeInRow(row, cell, changeRange, autoSizeCommit);
        bContinue = self.recalcCellSizeInRow({ let __x: Option<RowInfoHolder> = todo!("no align strategy matched"); __x }, { let __x: Option<CellInfoHolder> = todo!("no align strategy matched"); __x }, changeRange, autoSizeCommit);
      }
      // java if (bContinue)
      if bContinue {
        // java bContinue = this.recalcCellInfoInTable(deepTable, row, changeRange, autoSizeCommit);
        bContinue = self.recalcCellInfoInTable(deepTable, { let __x: Option<RowInfoHolder> = todo!("no align strategy matched"); __x }, changeRange, autoSizeCommit);
      }
      // java if (bContinue)
      if bContinue {
        // java CellInfo parentCell = deepTable.getParentCell();
        let mut parentCell: CellInfo = deepTable.getParentCell();
        // java TableInfo parentTable;
        let mut parentTable: TableInfo = TableInfo::default();
        bContinue = self.recalcCellSize({ let __x: Option<CellInfoHolder> = todo!("no align strategy matched"); __x }, changeRange, deepTable, autoSizeCommit);
        // java for(bContinue = this.recalcCellSize(parentCell, changeRange, deepTable, autoSizeCommit) bContinue && parentCell != rootCell bContinue = this.recalcCellSize(parentCell, changeRange, parentTable, autoSizeCommit))
        while bContinue && parentCell != rootCell {
          // java RowInfo parentRow = parentCell.getParent();
          let mut parentRow: RowInfo = parentCell.getParent();
          // java bContinue = this.recalcCellSizeInRow(parentRow, parentCell, changeRange, autoSizeCommit);
          bContinue = self.recalcCellSizeInRow({ let __x: Option<RowInfoHolder> = todo!("no align strategy matched"); __x }, { let __x: Option<CellInfoHolder> = todo!("no align strategy matched"); __x }, changeRange, autoSizeCommit);
          // java if (!bContinue)
          if !bContinue {
            // java break;
            break;
          }
          // java parentTable = parentRow.getParent();
          parentTable = parentRow.getParent();
          // java bContinue = this.recalcCellInfoInTable(parentTable, parentRow, changeRange, autoSizeCommit);
          bContinue = self.recalcCellInfoInTable({ let __x: Option<TableInfoHolder> = todo!("no align strategy matched"); __x }, { let __x: Option<RowInfoHolder> = todo!("no align strategy matched"); __x }, changeRange, autoSizeCommit);
          // java if (!bContinue)
          if !bContinue {
            // java break;
            break;
          }
          // java parentCell = parentTable.getParentCell();
          parentCell = parentTable.getParentCell();
        }
      }
      // java return bContinue;
      return bContinue;
      */
    }

    // java private boolean recalcCellSizeInTables(TableInfos tableInfos, TableInfo firstTableWantIgnore, long changeRange, AutoSizeCommitImpl autoSizeCommit)
    pub fn recalcCellSizeInTables(&self, tableInfos: Option<TableInfosHolder>, firstTableWantIgnore: Option<TableInfoHolder>, changeRange: i32, autoSizeCommit: Option<AutoSizeCommitImplHolder>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int firstIndex = firstTableWantIgnore != null ? ((TableInfoNode)firstTableWantIgnore).index() + 1 : 0;
      let mut firstIndex: i32 = (if firstTableWantIgnore.is_some() { (firstTableWantIgnore).downcast_ref::<TableInfoNode>().unwrap().index() + 1 } else { 0 });
      // java boolean bContinue = true;
      let mut bContinue: bool = true;
      let mut i: i32 = firstIndex;
      // java for(int i = firstIndex; i < tableInfos.size(); ++i)
      while i < (tableInfos.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) {
        // java TableInfo tableInfo = tableInfos.getTableByIndex(i);
        let mut tableInfo: TableInfo = tableInfos.getTableByIndex(i);
        // java if (tableInfo.isTableFit())
        if tableInfo.isTableFit() {
          // java bContinue = this.recalcCellInfoInTable(tableInfo, (RowInfo)null, changeRange, autoSizeCommit);
          bContinue = self.recalcCellInfoInTable({ let __x: Option<TableInfoHolder> = todo!("no align strategy matched"); __x }, None, changeRange, autoSizeCommit);
        }
        // java else
        else {
          // java bContinue = tableInfo.getEndCP() < RANGEFactory.getEnd(changeRange);
          bContinue = tableInfo.getEndCP() < RANGEFactory::getEnd(changeRange);
        }
        // java if (!bContinue)
        if !bContinue {
          // java break;
          break;
        }
      }
      // java return bContinue;
      return bContinue;
      */
    }

    // java private boolean recalcCellInfoInTable(TableInfo tableInfo, RowInfo firstRowWantIgnore, long changeRange, AutoSizeCommitImpl autoSizeCommit)
    pub fn recalcCellInfoInTable(&self, tableInfo: Option<TableInfoHolder>, firstRowWantIgnore: Option<RowInfoHolder>, changeRange: i32, autoSizeCommit: Option<AutoSizeCommitImplHolder>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int firstIndex = firstRowWantIgnore != null ? firstRowWantIgnore.getIndex() + 1 : 0;
      let mut firstIndex: i32 = (if firstRowWantIgnore.is_some() { firstRowWantIgnore.getIndex() + 1 } else { 0 });
      // java boolean bContinue = true;
      let mut bContinue: bool = true;
      let mut i: i32 = firstIndex;
      // java for(int i = firstIndex; i < tableInfo.size(); ++i)
      while i < (tableInfo.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) {
        // java RowInfo rowInfo = tableInfo.getRowByIndex(i);
        let mut rowInfo: RowInfo = tableInfo.getRowByIndex(i);
        // java bContinue = this.recalcCellSizeInRow(rowInfo, (CellInfo)null, changeRange, autoSizeCommit);
        bContinue = self.recalcCellSizeInRow({ let __x: Option<RowInfoHolder> = todo!("no align strategy matched"); __x }, None, changeRange, autoSizeCommit);
        // java if (!bContinue)
        if !bContinue {
          // java break;
          break;
        }
      }
      // java return bContinue;
      return bContinue;
      */
    }

    // java private boolean recalcCellSizeInRow(RowInfo row, CellInfo firstCellWnatIgnore, long changeRange, AutoSizeCommitImpl autoSizeCommit)
    pub fn recalcCellSizeInRow(&self, row: Option<RowInfoHolder>, firstCellWnatIgnore: Option<CellInfoHolder>, changeRange: i32, autoSizeCommit: Option<AutoSizeCommitImplHolder>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int firstIndex = firstCellWnatIgnore != null ? firstCellWnatIgnore.getIndex() + 1 : 0;
      let mut firstIndex: i32 = (if firstCellWnatIgnore.is_some() { firstCellWnatIgnore.getIndex() + 1 } else { 0 });
      // java boolean bContinue = true;
      let mut bContinue: bool = true;
      let mut i: i32 = firstIndex;
      // java for(int i = firstIndex; i < row.size(); ++i)
      while i < (row.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) {
        // java CellInfo cellInfo = row.getCellByIndex(i);
        let mut cellInfo: CellInfo = row.getCellByIndex(i);
        // java bContinue = this.recalcCellSize(cellInfo, changeRange, (TableInfo)null, autoSizeCommit);
        bContinue = self.recalcCellSize({ let __x: Option<CellInfoHolder> = todo!("no align strategy matched"); __x }, changeRange, None, autoSizeCommit);
        // java if (!bContinue)
        if !bContinue {
          // java break;
          break;
        }
      }
      // java return bContinue;
      return bContinue;
      */
    }

    // java private boolean recalcCellSize(CellInfo cellInfo, long changeRange, TableInfo firstChildTableWantIgnore, AutoSizeCommitImpl autoSizeCommit)
    pub fn recalcCellSize(&self, cellInfo: Option<CellInfoHolder>, changeRange: i32, firstChildTableWantIgnore: Option<TableInfoHolder>, autoSizeCommit: Option<AutoSizeCommitImplHolder>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!RANGEFactory.isIntersect(cellInfo.getRange(), changeRange))
      if !RANGEFactory::isIntersect__long_long(cellInfo.getRange(), changeRange) {
        // java return false;
        return false;
      }
      // java else
      else {
        // java this.initPageEnv(cellInfo);
        self.initPageEnv(cellInfo);
        // java this.mWidthReceiver.init(cellInfo);
        self.mWidthReceiver.init(cellInfo);
        // java boolean isCellInfoUpdated = false;
        let mut isCellInfoUpdated: bool = false;
        // java if (this.mReTypo || !this.mWidthReceiver.isComplete())
        if self.mReTypo || !self.mWidthReceiver.isComplete() {
          // java if (!this.mWidthReceiver.isIgnoreGraphParse())
          if !self.mWidthReceiver.isIgnoreGraphParse() {
            // java int curLevel = this.mLayoutState.tablePeriLayer - 1;
            let mut curLevel: i32 = self.mLayoutState.tablePeriLayer - 1;
            // java IPapxTable papxTable = this.mLayoutState.document.getPapxTable();
            let mut papxTable: Box<dyn IPapxTable> = self.mLayoutState.document.getPapxTable();
            // java long invalidRange = this.mWidthReceiver.getInvalidRange(changeRange, papxTable);
            let mut invalidRange: i64 = (self.mWidthReceiver.getInvalidRange(changeRange, papxTable) as i64);
            // java int invalidStartCp = RANGEFactory.getStart(invalidRange);
            let mut invalidStartCp: i32 = RANGEFactory::getStart(invalidRange);
            // java int invalidEndCp = RANGEFactory.getEnd(invalidRange);
            let mut invalidEndCp: i32 = RANGEFactory::getEnd(invalidRange);
            // java IPapxTable.Entry end = papxTable.seek(invalidEndCp);
            let mut end: Box<dyn Entry> = papxTable.seek(invalidEndCp);
            // java IPapxTable.Entry entry = papxTable.seek(invalidStartCp);
            let mut entry: Box<dyn Entry> = papxTable.seek(invalidStartCp);
            // java if (entry == end || invalidEndCp != end.position())
            if entry == end || invalidEndCp != end.position() {
              // java end = end.getNext();
              end = end.getNext();
            }
            // java IPapxTable.Entry next;
            let mut next: Box<dyn Entry> = Box::new(()) as Box<dyn Entry>;
            // java for(entry != end entry = next)
            while entry != end {
              // java next = entry.getNext();
              next = entry.getNext();
              // java int nParaLevel = entry.getProperty().getInt(224, 1);
              let mut nParaLevel: i32 = entry.getProperty().getInt(224, 1);
              // java if (nParaLevel == curLevel)
              if nParaLevel == curLevel {
                // java this.analyzeWidth(entry.position(), entry.endCp());
                self.analyzeWidth(entry.position(), entry.endCp());
              }
            }
          }
          // java isCellInfoUpdated = this.mWidthReceiver.finish();
          isCellInfoUpdated = self.mWidthReceiver.finish();
        }
        // java if (cellInfo.size() > 0)
        if (cellInfo.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32) > 0 {
          // java TableInfo firstIgnoreTable = firstChildTableWantIgnore;
          let mut firstIgnoreTable: TableInfo = { let __x: TableInfo = todo!("no align strategy matched"); __x };
          // java boolean bContinue = true;
          let mut bContinue: bool = true;
          // java int invalidStartCp = RANGEFactory.getStart(changeRange);
          let mut invalidStartCp: i32 = RANGEFactory::getStart(changeRange);
          // java if (firstChildTableWantIgnore == null && invalidStartCp > cellInfo.getStartCP())
          if firstChildTableWantIgnore.is_none() && invalidStartCp > cellInfo.getStartCP() {
            // java int i = 0;
            let mut i: i32 = 0;
            let mut count: i32 = (cellInfo.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
            // java for(int count = cellInfo.size(); i < count; ++i)
            while i < count {
              // java TableInfo tableInfo = cellInfo.getTableByIndex(i);
              let mut tableInfo: TableInfo = cellInfo.getTableByIndex(i);
              // java if (tableInfo.getEndCP() < invalidStartCp)
              if tableInfo.getEndCP() < invalidStartCp {
                // java firstIgnoreTable = tableInfo;
                firstIgnoreTable = tableInfo;
                // java if (i == count - 1)
                if i == count - 1 {
                  // java bContinue = false;
                  bContinue = false;
                }
              }
            }
          }
          // java if (bContinue)
          if bContinue {
            // java this.recalsCellSizeInChildTable(cellInfo, changeRange, firstIgnoreTable, autoSizeCommit);
            self.recalsCellSizeInChildTable(cellInfo, changeRange, { let __x: Option<TableInfoHolder> = todo!("no align strategy matched"); __x }, autoSizeCommit);
          }
        }
        // java if (isCellInfoUpdated || !this.mReTypo || !RANGEFactory.isSubset(cellInfo.getRange(), changeRange))
        if isCellInfoUpdated || !self.mReTypo || !RANGEFactory::isSubset(cellInfo.getRange(), changeRange) {
          // java autoSizeCommit.commitCell(cellInfo);
          autoSizeCommit.commitCell(cellInfo);
        }
        // java return RANGEFactory.getEnd(cellInfo.getRange()) < RANGEFactory.getEnd(changeRange);
        return RANGEFactory::getEnd(cellInfo.getRange()) < RANGEFactory::getEnd(changeRange);
      }
      */
    }

    // java private void analyzeWidth(int starCp, int endCp)
    pub fn analyzeWidth(&self, starCp: i32, endCp: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mLayoutState.cp = starCp;
      self.mLayoutState.cp = starCp;
      // java this.mParaLayouter.start(this.mLayoutState.getEnvoy());
      self.mParaLayouter.start(self.mLayoutState.getEnvoy());
      // java this.initGraphBuff(this.mLayoutState);
      self.initGraphBuff(self.mLayoutState);
      // java this.mFastGraph.prepareGraphs(this.mLayoutState, this.mCacheGraphBuff);
      self.mFastGraph.prepareGraphs(self.mLayoutState, self.mCacheGraphBuff);
      // java WidthAnalyzer widthAnalyzer = this.mDocEnv.layoutToolsManager.getWidthAnalyzer();
      let mut widthAnalyzer: WidthAnalyzer = self.mDocEnv.layoutToolsManager.getWidthAnalyzer();
      // java widthAnalyzer.setFastGraph(this.mFastGraph);
      widthAnalyzer.setFastGraph(self.mFastGraph);
      // java widthAnalyzer.setWidthReceiver(this.mWidthReceiver);
      widthAnalyzer.setWidthReceiver(self.mWidthReceiver);
      // java widthAnalyzer.doAnalyze(this.mLayoutState);
      widthAnalyzer.doAnalyze(self.mLayoutState);
      // java this.mParaLayouter.end(this.mLayoutState.getEnvoy());
      self.mParaLayouter.end(self.mLayoutState.getEnvoy());
      // java this.mCacheGraphBuff.clear(this.mLayoutState);
      self.mCacheGraphBuff.clear(self.mLayoutState);
      */
    }

    // java protected void initGraphBuff(LayoutState layoutState)
    pub fn initGraphBuff(&mut self, layoutState: Option<LayoutStateHolder>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mCacheGraphBuff == null)
      if self.mCacheGraphBuff.is_none() {
        // java this.mCacheGraphBuff = GraphBuffFactory.createGraphBuff(this.mMainState, this.mDocEnv, this.mViewEnv);
        self.mCacheGraphBuff = GraphBuffFactory::createGraphBuff(self.mMainState, self.mDocEnv, self.mViewEnv);
      }
      // java layoutState.initCoreContent(layoutState.getRangeEnd(), this.mViewEnv.isRevisionFinal());
      layoutState.initCoreContent(layoutState.getRangeEnd(), self.mViewEnv.isRevisionFinal());
      // java this.mCacheGraphBuff.initFilter(layoutState.document, layoutState.cp, layoutState.getRangeEnd(), layoutState);
      self.mCacheGraphBuff.initFilter(layoutState.document, layoutState.cp, layoutState.getRangeEnd(), layoutState);
      */
    }

    // java private boolean recalsCellSizeInChildTable(CellInfo cellInfo, long changeRange, TableInfo firstTableWantIgnore, AutoSizeCommitImpl autoSizeCommit)
    pub fn recalsCellSizeInChildTable(&self, cellInfo: Option<CellInfoHolder>, changeRange: i32, firstTableWantIgnore: Option<TableInfoHolder>, autoSizeCommit: Option<AutoSizeCommitImplHolder>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java boolean bContinue = true;
      let mut bContinue: bool = true;
      // java int firstIndex = firstTableWantIgnore != null ? ((TableInfoNode)firstTableWantIgnore).index() + 1 : 0;
      let mut firstIndex: i32 = (if firstTableWantIgnore.is_some() { (firstTableWantIgnore).downcast_ref::<TableInfoNode>().unwrap().index() + 1 } else { 0 });
      // java int i = firstIndex;
      let mut i: i32 = firstIndex;
      let mut count: i32 = (cellInfo.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      // java for(int count = cellInfo.size(); i < count; ++i)
      while i < count {
        // java TableInfo tableInfo = cellInfo.getTableByIndex(i);
        let mut tableInfo: TableInfo = cellInfo.getTableByIndex(i);
        // java bContinue = this.recalcCellInfoInTable(tableInfo, (RowInfo)null, changeRange, autoSizeCommit);
        bContinue = self.recalcCellInfoInTable({ let __x: Option<TableInfoHolder> = todo!("no align strategy matched"); __x }, None, changeRange, autoSizeCommit);
        // java if (!bContinue)
        if !bContinue {
          // java break;
          break;
        }
      }
      // java return bContinue;
      return bContinue;
      */
    }

    // java private void dumpEntryTest(IPapxTable.Entry entry)
    pub fn dumpEntryTest(&self, entry: Option<IPapxTable_EntryHolder>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int start = entry.position();
      let mut start: i32 = entry.position();
      // java int len = entry.getNext().position() - start;
      let mut len: i32 = entry.getNext().position() - start;
      // java System.out.print(start + "-" + (start + len));
      System::out.print(start + "-" + start + len);
      // java char[] buf = new char[len];
      let mut buf: Vec<char> = vec![Default::default(); (len) as usize];
      // java this.mLayoutState.document.getTextRope().getText(start, start + len, buf, 0);
      self.mLayoutState.document.getTextRope().getText(start, start + len, buf, 0);
      // java Log.d(Tag, "gridSizeLayout ParaEntry " + start + "-" + (start + len) + " :" + toString(buf));
      Log::d__string_string(Tag, "gridSizeLayout ParaEntry " + start + "-" + start + len + " :" + Self::toString__chararr(buf));
      */
    }

    // java public static String toString(char[] buf)
    pub fn toString__chararr(buf: &Vec<char>) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return toString(buf, 0, buf.length);
      return Self::toString__chararr_int_int(buf, 0, (buf.len() as i32));
      */
    }

    // java public static String toString(char[] buf, int offset, int len)
    pub fn toString__chararr_int_int(buf: &Vec<char>, offset: i32, len: i32) -> String {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java StringBuilder builder = new StringBuilder();
      let mut builder: String = { let __x: String = todo!("no align strategy matched"); __x };
      let mut i: i32 = 0;
      // java for(int i = 0; i < len; ++i)
      while i < len {
        // java char c = buf[offset + i];
        let mut c: char = buf[(offset + i) as usize];
        // java if (Character.isISOControl(c))
        if Character::isISOControl(c) {
          // java builder.append("{\\0x" + Integer.toHexString(c) + "}");
          builder.append("{\\0x" + Integer::toHexString(c) + "}");
        }
        // java else
        else {
          // java builder.append(c);
          builder.append(c);
        }
      }
      // java return builder.toString();
      return builder.toString();
      */
    }

    // java private void initPageEnv(CellInfo cellInfo)
    pub fn initPageEnv(&mut self, cellInfo: Option<CellInfoHolder>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mLayoutState.tablePeriLayer = cellInfo.getParent().getParent().getLevel() + 1;
      self.mLayoutState.tablePeriLayer = cellInfo.getParent().getParent().getLevel() + 1;
      // java this.mLayoutState.setRange(RANGEFactory.getStart(cellInfo.getRange()), RANGEFactory.getEnd(cellInfo.getRange()));
      self.mLayoutState.setRange__int_int(RANGEFactory::getStart(cellInfo.getRange()), RANGEFactory::getEnd(cellInfo.getRange()));
      // java PageEnv pageEnv = this.mLayoutState.pageEnv;
      let mut pageEnv: PageEnv = self.mLayoutState.pageEnv;
      // java int textFlow = 0;
      let mut textFlow: i32 = 0;
      // java boolean rotationAsia = this.getRotationAsia(cellInfo);
      let mut rotationAsia: bool = self.getRotationAsia(cellInfo);
      // java switch (this.getTypoTextFlow(cellInfo))
      match self.getTypoTextFlow(cellInfo) {
        // java case 1:
        1 => {
          // java pageEnv.marginTop = cellInfo.getMarginLeft();
          pageEnv.marginTop = cellInfo.getMarginLeft();
          // java pageEnv.marginLeft = cellInfo.getMarginBottom();
          pageEnv.marginLeft = cellInfo.getMarginBottom();
          // java pageEnv.marginRight = cellInfo.getMarginTop();
          pageEnv.marginRight = cellInfo.getMarginTop();
          // java pageEnv.marginBottom = cellInfo.getMarginRight();
          pageEnv.marginBottom = cellInfo.getMarginRight();
          // java textFlow = rotationAsia ? 1 : 5;
          textFlow = (if rotationAsia { 1 } else { 5 });
        }
        // java case 3:
        3 => {
          // java pageEnv.marginTop = cellInfo.getMarginRight();
          pageEnv.marginTop = cellInfo.getMarginRight();
          // java pageEnv.marginLeft = cellInfo.getMarginTop();
          pageEnv.marginLeft = cellInfo.getMarginTop();
          // java pageEnv.marginRight = cellInfo.getMarginBottom();
          pageEnv.marginRight = cellInfo.getMarginBottom();
          // java pageEnv.marginBottom = cellInfo.getMarginLeft();
          pageEnv.marginBottom = cellInfo.getMarginLeft();
          // java textFlow = 3;
          textFlow = 3;
        }
        // java default:
        _ => {
          // java pageEnv.marginTop = cellInfo.getMarginTop();
          pageEnv.marginTop = cellInfo.getMarginTop();
          // java pageEnv.marginLeft = cellInfo.getMarginLeft();
          pageEnv.marginLeft = cellInfo.getMarginLeft();
          // java pageEnv.marginRight = cellInfo.getMarginRight();
          pageEnv.marginRight = cellInfo.getMarginRight();
          // java pageEnv.marginBottom = cellInfo.getMarginBottom();
          pageEnv.marginBottom = cellInfo.getMarginBottom();
          // java if (rotationAsia)
          if rotationAsia {
            // java textFlow = 4;
            textFlow = 4;
          }
        }
      }
      // java pageEnv.textFlow = textFlow;
      pageEnv.textFlow = textFlow;
      */
    }

    // java private int getTypoTextFlow(CellInfo cellInfo)
    pub fn getTypoTextFlow(&self, cellInfo: Option<CellInfoHolder>) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int typoTextDir = 0;
      let mut typoTextDir: i32 = 0;
      // java if (cellInfo != null)
      if let Some(cellInfo) = cellInfo {
        // java switch (Tools.getTextFlow(cellInfo.getTextFlow()))
        match Tools::getTextFlow(cellInfo.getTextFlow()) {
          // java case 1: case 5:
          1 | 5 => {
            // java typoTextDir = 1;
            typoTextDir = 1;
          }
          // java case 2: case 4: default:
          2 | 4 | _ => {
          }
          // java case 3:
          3 => {
            // java typoTextDir = 3;
            typoTextDir = 3;
          }
        }
      }
      // java return typoTextDir;
      return typoTextDir;
      */
    }

    // java private boolean getRotationAsia(CellInfo cellInfo)
    pub fn getRotationAsia(&self, cellInfo: Option<CellInfoHolder>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java boolean rotationAsia = false;
      let mut rotationAsia: bool = false;
      // java switch (Tools.getTextFlow(cellInfo.getTextFlow()))
      match Tools::getTextFlow(cellInfo.getTextFlow()) {
        // java case 1:
        1 => {
          // java rotationAsia = true;
          rotationAsia = true;
        }
        // java case 4:
        4 => {
          // java rotationAsia = true;
          rotationAsia = true;
        }
        // java default:
        _ => {
        }
      }
      // java return rotationAsia;
      return rotationAsia;
      */
    }

    // java private TypoSnapshot cleanForkSnapshot(TypoDocument typoDocument)
    pub fn cleanForkSnapshot(&self, typoDocument: Option<TypoDocumentHolder>) -> Option<TypoSnapshotHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TypoSnapshot cloneSnapshot = typoDocument.getSnapshot();
      let mut cloneSnapshot: TypoSnapshot = typoDocument.getSnapshot();
      // java TypoSnapshot cleanForkSnapshot = cloneSnapshot.cleanFork();
      let mut cleanForkSnapshot: TypoSnapshot = cloneSnapshot.cleanFork();
      // java cloneSnapshot.release();
      cloneSnapshot.release();
      // java return cleanForkSnapshot;
      return { let __x: Option<TypoSnapshotHolder> = todo!("no align strategy matched"); __x };
      */
    }

    // java private void initEnv(Document document, TableInfo firstTableInfo)
    pub fn initEnv(&mut self, document: Option<DocumentHolder>, firstTableInfo: Option<TableInfoHolder>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mLayoutState.document = document;
      self.mLayoutState.document = document;
      // java if (this.mMainState.snapshot == null)
      if self.mMainState.snapshot.is_none() {
        // java this.mMainState.snapshot = this.cleanForkSnapshot(this.mDocEnv.typoDocument);
        self.mMainState.snapshot = self.cleanForkSnapshot(Some(self.mDocEnv.typoDocument));
      }
      // java this.mMainState.snapshot.switchUsingThread();
      self.mMainState.snapshot.switchUsingThread();
      // java this.mMainState.layoutPage.alloc(this.mMainState.snapshot);
      self.mMainState.layoutPage.alloc(self.mMainState.snapshot);
      // java if (this.mLayoutState.curPage == null)
      if self.mLayoutState.curPage.is_none() {
        // java TypoTableCell typoCell = this.mMainState.snapshot.getTypoTools().obtainTypoTableCell();
        let mut typoCell: TypoTableCell = self.mMainState.snapshot.getTypoTools().obtainTypoTableCell();
        // java typoCell.alloc(this.mMainState.snapshot);
        typoCell.alloc(self.mMainState.snapshot);
        // java typoCell.setParent(this.mMainState.layoutPage.getPointer());
        typoCell.setParent(self.mMainState.layoutPage.getPointer());
        // java typoCell.setDocumentType(this.mLayoutState.document.getType());
        typoCell.setDocumentType(self.mLayoutState.document.getType());
        // java this.mLayoutState.curPage = typoCell;
        self.mLayoutState.curPage = typoCell;
      }
      // java int cp = cn.wps.moffice.writer.core.table.Tools.getMainCp(document, firstTableInfo.getStartCP());
      let mut cp: i32 = cn.wps.moffice.writer.core.table.Tools.getMainCp(document, firstTableInfo.getStartCP());
      // java PageSetup pageSetup = document.getTextDocument().getSectionProperty().getPageSetup(cp);
      let mut pageSetup: PageSetup = document.getTextDocument().getSectionProperty().getPageSetup(cp);
      // java int coreWidth = pageSetup.getWidth() - pageSetup.getMarginLeft() - pageSetup.getMarginRight();
      let mut coreWidth: i32 = pageSetup.getWidth() - pageSetup.getMarginLeft() - pageSetup.getMarginRight();
      // java int coreHeight = pageSetup.getHeight() - pageSetup.getMarginTop() - pageSetup.getMarginBottom();
      let mut coreHeight: i32 = pageSetup.getHeight() - pageSetup.getMarginTop() - pageSetup.getMarginBottom();
      // java PageEnv pageEnv = this.mLayoutState.pageEnv;
      let mut pageEnv: PageEnv = self.mLayoutState.pageEnv;
      // java pageEnv.init();
      pageEnv.init();
      // java pageEnv.width = coreWidth;
      pageEnv.width = coreWidth;
      // java pageEnv.height = coreHeight;
      pageEnv.height = coreHeight;
      // java pageEnv.maxLayoutHeight = coreHeight;
      pageEnv.maxLayoutHeight = coreHeight;
      // java if (this.mFastGraph == null)
      if self.mFastGraph.is_none() {
        // java this.mFastGraph = new FastGraphImpl(this.mMainState, this.mDocEnv, this.mViewEnv);
        self.mFastGraph = FastGraphImpl::new(self.mMainState, self.mDocEnv, self.mViewEnv);
      }
      */
    }

    // java private void closeEnv()
    pub fn closeEnv(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mMainState.layoutPage.clear();
      self.mMainState.layoutPage.clear();
      // java this.mMainState.snapshot.getTypoTools().recycle((TypoMemoryObj)this.mLayoutState.curPage);
      self.mMainState.snapshot.getTypoTools().recycle((self.mLayoutState.curPage).downcast_ref::<TypoMemoryObj>().unwrap());
      // java this.mLayoutState.curPage = null;
      self.mLayoutState.curPage = None;
      // java this.mMainState.snapshot.reset();
      self.mMainState.snapshot.reset();
      */
    }

}

// java private class EditingWidthReceiver implements IWidthReceiver
impl EditingWidthReceiver {
    // java private  EditingWidthReceiver()
    pub fn EditingWidthReceiver(&self) -> Option<EditingWidthReceiverHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mCellInfo = null;
      Self { mCellInfo: None, mCacheFitInfo: AutoFitInfo::new(), mFastInputCache: FastInputCache::new(), mReuseCacheAcived: false, mIsInTableHeader: false }
      // java this.mCacheFitInfo = new Data.AutoFitInfo();
      // java this.mFastInputCache = new FastInputCache();
      // java this.mReuseCacheAcived = false;
      // java this.mIsInTableHeader = false;
      */
    }

    // java public long getInvalidRange(long changeRange, IPapxTable papxTable)
    pub fn getInvalidRange(&self, changeRange: i32, papxTable: Option<IPapxTableHolder>) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int cellStartCp = this.mCellInfo.getStartCP();
      let mut cellStartCp: i32 = self.mCellInfo.getStartCP();
      // java int cellEndCp = this.mCellInfo.getEndCP();
      let mut cellEndCp: i32 = self.mCellInfo.getEndCP();
      // java int changeStart = RANGEFactory.getStart(changeRange);
      let mut changeStart: i32 = RANGEFactory::getStart(changeRange);
      // java int changeEnd = RANGEFactory.getEnd(changeRange);
      let mut changeEnd: i32 = RANGEFactory::getEnd(changeRange);
      // java if (!this.mCellInfo.isAutoWidth() && cellStartCp <= changeStart && cellEndCp >= changeEnd)
      if !self.mCellInfo.isAutoWidth() && cellStartCp <= changeStart && cellEndCp >= changeEnd {
        // java this.mReuseCacheAcived = true;
        self.mReuseCacheAcived = true;
        // java if (this.mFastInputCache.isSameCell(cellStartCp))
        if self.mFastInputCache.isSameCell(cellStartCp) {
          // java int oldParaStart = this.mFastInputCache.paraStart();
          let mut oldParaStart: i32 = self.mFastInputCache.paraStart();
          // java int oldParaEnd = this.mFastInputCache.paraEnd();
          let mut oldParaEnd: i32 = self.mFastInputCache.paraEnd();
          // java if (changeStart >= oldParaEnd)
          if changeStart >= oldParaEnd {
            // java this.mCacheFitInfo.updateFrom(this.mCellInfo.getData().getAutoFitInfo());
            self.mCacheFitInfo.updateFrom(self.mCellInfo.getData().getAutoFitInfo());
            // java return changeRange;
            return changeRange;
          }
          // java if (changeEnd < oldParaStart && oldParaEnd > oldParaStart + 3)
          if changeEnd < oldParaStart && oldParaEnd > oldParaStart + 3 {
            // java int changelen = changeEnd - changeStart;
            let mut changelen: i32 = changeEnd - changeStart;
            // java int oldMidDis = (oldParaEnd - oldParaStart) / 2;
            let mut oldMidDis: i32 = oldParaEnd - oldParaStart / 2;
            // java if (oldMidDis > changelen)
            if oldMidDis > changelen {
              // java IPapxTable.Entry entry = papxTable.seek(oldParaStart + oldMidDis);
              let mut entry: Box<dyn Entry> = papxTable.seek(oldParaStart + oldMidDis);
              // java if (entry != null)
              if true {
                // java long adjustRange = entry.range();
                let mut adjustRange: i64 = entry.range();
                // java if (RANGEFactory.getLength(adjustRange) == oldParaEnd - oldParaStart)
                if RANGEFactory::getLength(adjustRange) == oldParaEnd - oldParaStart {
                  // java this.mFastInputCache.mParaRange = adjustRange;
                  self.mFastInputCache.mParaRange = adjustRange;
                  // java this.mCacheFitInfo.updateFrom(this.mCellInfo.getData().getAutoFitInfo());
                  self.mCacheFitInfo.updateFrom(self.mCellInfo.getData().getAutoFitInfo());
                  // java return changeRange;
                  return changeRange;
                }
              }
            }
            // java this.mFastInputCache.reset(cellStartCp);
            self.mFastInputCache.reset(cellStartCp);
          }
          // java else
          else {
            // java if (changeStart == oldParaEnd - 1 && changeEnd == oldParaEnd + 1)
            if changeStart == oldParaEnd - 1 && changeEnd == oldParaEnd + 1 {
              // java this.mFastInputCache.reset(cellStartCp);
              self.mFastInputCache.reset(cellStartCp);
              // java return RANGEFactory.getRANGE(Math.min(oldParaStart, changeStart), changeEnd);
              return RANGEFactory::getRANGE((oldParaStart).min(changeStart), changeEnd);
            }
            // java this.mFastInputCache.reset(cellStartCp);
            self.mFastInputCache.reset(cellStartCp);
          }
        }
        // java else
        else {
          // java this.mFastInputCache.reset(cellStartCp);
          self.mFastInputCache.reset(cellStartCp);
        }
      }
      // java else
      else {
        // java this.mFastInputCache.reset(-1);
        self.mFastInputCache.reset(-1);
      }
      // java return RANGEFactory.getRANGE(cellStartCp, cellEndCp);
      return RANGEFactory::getRANGE(cellStartCp, cellEndCp);
      */
    }

    // java public void init(CellInfo cellInfo)
    pub fn init(&mut self, cellInfo: &CellInfo) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mCellInfo = cellInfo;
      self.mCellInfo = cellInfo;
      // java this.mCacheFitInfo.clear();
      self.mCacheFitInfo.clear();
      // java this.mReuseCacheAcived = false;
      self.mReuseCacheAcived = false;
      // java this.mIsInTableHeader = this.mCellInfo.getParent().getIndex() == 0;
      self.mIsInTableHeader = self.mCellInfo.getParent().getIndex() == 0;
      */
    }

    // java public boolean isComplete()
    pub fn isComplete(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Assert.assertNotNull(this.mCellInfo);
      Assert::assertNotNull__object(self.mCellInfo);
      // java return this.mCellInfo.getData().getAutoFitInfo().isComplete();
      return self.mCellInfo.getData().getAutoFitInfo().isComplete();
      */
    }

    // java public void submitInfo(IWidthInfo info)
    pub fn submitInfo(&self, info: Option<IWidthInfoHolder>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.isTextDirNormal())
      if self.isTextDirNormal() {
        // java if (this.mCacheFitInfo.commitNewInfo(info.getMinWidth(), info.getMaxWidth(), info.getAutoWidthTotal()) && this.mReuseCacheAcived)
        if self.mCacheFitInfo.commitNewInfo(info.getMinWidth(), info.getMaxWidth(), info.getAutoWidthTotal()) && self.mReuseCacheAcived {
          // java this.mFastInputCache.mParaRange = info.getParaRange();
          self.mFastInputCache.mParaRange = info.getParaRange();
          // java if (this.mCacheFitInfo.getMinWidth() != info.getMinWidth())
          if self.mCacheFitInfo.getMinWidth() != info.getMinWidth() {
            // java this.mReuseCacheAcived = false;
            self.mReuseCacheAcived = false;
          }
        }
      }
      // java else
      else {
        // java this.mReuseCacheAcived = false;
        self.mReuseCacheAcived = false;
        // java this.mCacheFitInfo.addNewMaxInfo(info.getMinWidth(), info.getMaxWidth());
        self.mCacheFitInfo.addNewMaxInfo(info.getMinWidth(), info.getMaxWidth());
      }
      */
    }

    // java public boolean finish()
    pub fn finish(&mut self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.isFitText())
      if self.isFitText() {
        // java this.commitForFitText();
        self.commitForFitText();
        // java this.mReuseCacheAcived = false;
        self.mReuseCacheAcived = false;
      }
      // java else
      else {
        // java this.commitWrapObject();
        self.commitWrapObject();
      }
      // java boolean updated = false;
      let mut updated: bool = false;
      // java if (this.mCacheFitInfo.isComplete() && this.mCellInfo != null)
      if let Some(mCellInfo) = self.mCellInfo.as_mut() {
        if self.mCacheFitInfo.isComplete() {
          // java updated = this.mCellInfo.getData().getAutoFitInfo().updateFrom(this.mCacheFitInfo);
          updated = mCellInfo.getData().getAutoFitInfo().updateFrom(self.mCacheFitInfo);
        }
      }
      // java if (!this.mReuseCacheAcived || !this.mFastInputCache.isValid())
      if !self.mReuseCacheAcived || !self.mFastInputCache.isValid() {
        // java this.mFastInputCache.reset(-1);
        self.mFastInputCache.reset(-1);
      }
      // java this.mReuseCacheAcived = false;
      self.mReuseCacheAcived = false;
      // java this.mCellInfo = null;
      self.mCellInfo = None;
      // java return updated;
      return updated;
      */
    }

    // java private void commitWrapObject()
    pub fn commitWrapObject(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (GridSizeLayouter.this.mLayoutState.curPage instanceof TypoTableCell)
      if self.mLayoutState.curPage.downcast_ref::<TypoTableCell>().is_some() {
        // java TypoSnapshot snapshot = GridSizeLayouter.this.mMainState.snapshot;
        let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
        // java TypoToolsManager typoTools = snapshot.getTypoTools();
        let mut typoTools: TypoToolsManager = snapshot.getTypoTools();
        // java IIntList floatDrwingList = GridSizeLayouter.this.mFastGraph.getFloatDrawinglist();
        let mut floatDrwingList: Box<dyn IIntList> = self.mFastGraph.getFloatDrawinglist();
        // java if (!floatDrwingList.isEmpty())
        if !floatDrwingList.is_empty() {
          // java int drawingMaxWidth = 0;
          let mut drawingMaxWidth: i32 = 0;
          // java TypoRect rect = typoTools.obtainRect();
          let mut rect: TypoRect = typoTools.obtainRect();
          // java int i = 0;
          let mut i: i32 = 0;
          let mut size: i32 = (floatDrwingList.len() as i32);
          // java for(int size = floatDrwingList.size(); i < size; ++i)
          while i < size {
            // java TypoDrawing drawing = typoTools.obtainTypoDrawing(floatDrwingList.get(i));
            let mut drawing: TypoDrawing = typoTools.obtainTypoDrawing(floatDrwingList.get(i));
            // java if (drawing.isWrap())
            if drawing.isWrap() {
              // java int pRectPaddings = TypoDrawing.getLayoutRectPadding(drawing.getPointer());
              let mut pRectPaddings: i32 = TypoDrawing::getLayoutRectPadding(drawing.getPointer());
              // java DrawingResizeTool.expandRect(drawing, TypoRectMemoryImpl.getLeft(pRectPaddings, snapshot), TypoRectMemoryImpl.getTop(pRectPaddings, snapshot), TypoRectMemoryImpl.getRight(pRectPaddings, snapshot), TypoRectMemoryImpl.getBottom(pRectPaddings, snapshot), rect);
              DrawingResizeTool::expandRect__ityporect_int_int_int_int_ityporect(drawing, TypoRectMemoryImpl::getLeft__int_typosnapshot(pRectPaddings, snapshot), TypoRectMemoryImpl::getTop__int_typosnapshot(pRectPaddings, snapshot), TypoRectMemoryImpl::getRight__int_typosnapshot(pRectPaddings, snapshot), TypoRectMemoryImpl::getBottom__int_typosnapshot(pRectPaddings, snapshot), rect);
              // java if (rect.width() > drawingMaxWidth)
              if rect.width() > drawingMaxWidth {
                // java drawingMaxWidth = rect.width();
                drawingMaxWidth = rect.width();
              }
            }
            // java typoTools.recycle((TypoMemoryObj)drawing);
            typoTools.recycle((drawing).downcast_ref::<TypoMemoryObj>().unwrap());
          }
          // java typoTools.recycle(rect);
          typoTools.recycle(rect);
          // java if (drawingMaxWidth > 0)
          if drawingMaxWidth > 0 {
            // java this.mCacheFitInfo.commitNewInfo(0, drawingMaxWidth, 0);
            self.mCacheFitInfo.commitNewInfo(0, drawingMaxWidth, 0);
          }
        }
      }
      */
    }

    // java public boolean isIgnoreGraphParse()
    pub fn isIgnoreGraphParse(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.isFitText();
      return self.isFitText();
      */
    }

    // java private boolean isFitText()
    pub fn isFitText(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java boolean isFit = this.mCellInfo != null && this.mCellInfo.getData().getFitText();
      let mut isFit: bool = self.mCellInfo.is_some() && self.mCellInfo.getData().getFitText();
      // java return isFit && this.isTextDirNormal();
      return isFit && self.isTextDirNormal();
      */
    }

    // java private void commitForFitText()
    pub fn commitForFitText(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mCellInfo != null)
      if let Some(mCellInfo) = self.mCellInfo.as_ref() {
        // java int fitLayoutWidth = this.mCellInfo.getWidth() - this.mCellInfo.getMarginLeft() - this.mCellInfo.getMarginRight();
        let mut fitLayoutWidth: i32 = mCellInfo.getWidth() - mCellInfo.getMarginLeft() - mCellInfo.getMarginRight();
        // java this.mCacheFitInfo.commitNewInfo(fitLayoutWidth, fitLayoutWidth, 0);
        self.mCacheFitInfo.commitNewInfo(fitLayoutWidth, fitLayoutWidth, 0);
      }
      */
    }

    // java public boolean isTextDirNormal()
    pub fn isTextDirNormal(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return GridSizeLayouter.this.getTypoTextFlow(this.mCellInfo) == 0;
      return self.getTypoTextFlow(self.mCellInfo) == 0;
      */
    }

    // java public boolean isAutoWidth()
    pub fn isAutoWidth(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (GridSizeLayouter.this.mForceFit)
      if self.mForceFit {
        // java return true;
        return true;
      }
      // java else
      else {
        // java return this.mCellInfo != null ? this.mCellInfo.isAutoWidth() : false;
        return (if self.mCellInfo.is_some() { self.mCellInfo.isAutoWidth() } else { false });
      }
      */
    }

    // java public boolean isAloneCache()
    pub fn isAloneCache(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return true;
      return true;
      */
    }

    // java public boolean isInTableHeader()
    pub fn isInTableHeader(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mIsInTableHeader;
      return self.mIsInTableHeader;
      */
    }

}

// java private class FastInputCache
impl FastInputCache {
    // java   FastInputCache()
    pub fn FastInputCache(&self) -> Option<FastInputCacheHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.reset(-1);
      this.reset(-1);
      */
    }

    // java  void reset(int cellStartCp)
    pub fn reset(&mut self, cellStartCp: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mCellStartCp = cellStartCp;
      self.mCellStartCp = cellStartCp;
      // java this.mParaRange = 0L;
      self.mParaRange = 0;
      */
    }

    // java  boolean isValid()
    pub fn isValid(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mCellStartCp >= 0 && RANGEFactory.getLength(this.mParaRange) > 0;
      return self.mCellStartCp >= 0 && RANGEFactory::getLength(self.mParaRange) > 0;
      */
    }

    // java  boolean isSameCell(int cellStartCp)
    pub fn isSameCell(&self, cellStartCp: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.isValid() && this.mCellStartCp == cellStartCp;
      return self.isValid() && self.mCellStartCp == cellStartCp;
      */
    }

    // java  int paraStart()
    pub fn paraStart(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return RANGEFactory.getStart(this.mParaRange);
      return RANGEFactory::getStart(self.mParaRange);
      */
    }

    // java  int paraEnd()
    pub fn paraEnd(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return RANGEFactory.getEnd(this.mParaRange);
      return RANGEFactory::getEnd(self.mParaRange);
      */
    }

}

// java public static class AutoSizeCommitImpl implements GridSizeCommit
impl AutoSizeCommitImpl {
    // java public static synchronized AutoSizeCommitImpl newInstance()
    pub fn newInstance() -> Option<AutoSizeCommitImplHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (sInstance == null || sInstance.isUsing())
      if false || sInstance.isUsing() {
        // java sInstance = new AutoSizeCommitImpl();
        sInstance = AutoSizeCommitImpl::new();
      }
      // java return sInstance;
      return { let __x: Option<AutoSizeCommitImplHolder> = todo!("no align strategy matched"); __x };
      */
    }

    // java public synchronized boolean isUsing()
    pub fn isUsing(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mUsing;
      return self.mUsing;
      */
    }

    // java public synchronized AutoSizeCommitImpl init(Document document, int typoCoreWidth)
    pub fn init(&mut self, document: &dyn Document, typoCoreWidth: i32) -> AutoSizeCommitImpl {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mDocument = document;
      self.mDocument = document;
      // java this.mTypoCoreWidth = typoCoreWidth;
      self.mTypoCoreWidth = typoCoreWidth;
      // java this.mUsing = true;
      self.mUsing = true;
      // java return this;
      return self;
      */
    }

    // java public void commitToCore()
    pub fn commitToCore(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java for(GridSizeCommit commit : this.mList)
      for commit in self.mList.iter() {
        // java commit.commitToCore();
        commit.commitToCore();
      }
      // java this.recycle();
      self.recycle();
      */
    }

    // java private synchronized void recycle()
    pub fn recycle(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mList != null)
      if true {
        // java this.mList.clear();
        self.mList.clear();
      }
      // java this.mDocument = null;
      self.mDocument = None;
      // java this.mWaitCommitcGridCols = null;
      self.mWaitCommitcGridCols = None;
      // java this.mUsing = false;
      self.mUsing = false;
      */
    }

    // java private void add(GridSizeCommit commit)
    pub fn add(&mut self, commit: Option<GridSizeCommitHolder>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mList == null)
      if false {
        // java this.mList = new ArrayList();
        self.mList = Vec::new();
      }
      // java this.mList.add(commit);
      self.mList.add(commit);
      */
    }

    // java private void commitWait()
    pub fn commitWait(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mWaitCommitcGridCols != null)
      if let Some(mWaitCommitcGridCols) = self.mWaitCommitcGridCols.as_ref() {
        // java GridAutoSize autoSize = this.mWaitCommitcGridCols.getAutoSize();
        let mut autoSize: Box<dyn GridAutoSize> = mWaitCommitcGridCols.getAutoSize();
        // java if (autoSize != null && autoSize.autoFitSize(this.mTypoCoreWidth))
        if true && autoSize.autoFitSize(self.mTypoCoreWidth) {
          // java this.add(autoSize);
          self.add({ let __x: Option<GridSizeCommitHolder> = todo!("no align strategy matched"); __x });
        }
      }
      // java this.mWaitCommitcGridCols = null;
      self.mWaitCommitcGridCols = None;
      */
    }

    // java public void commitCell(CellInfo cellInfo)
    pub fn commitCell(&self, cellInfo: Option<CellInfoHolder>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TableInfo topTable = this.mDocument.getTableInfos().getTableInfoByLevel(cellInfo.getStartCP(), 1);
      let mut topTable: TableInfo = self.mDocument.getTableInfos().getTableInfoByLevel(cellInfo.getStartCP(), 1);
      // java GridCols gridCols = topTable.getGrid();
      let mut gridCols: Box<dyn GridCols> = topTable.getGrid();
      // java if (this.mWaitCommitcGridCols != null && this.mWaitCommitcGridCols != gridCols)
      if let Some(mWaitCommitcGridCols) = self.mWaitCommitcGridCols.as_ref() {
        if self.mWaitCommitcGridCols != gridCols {
          // java this.commitWait();
          self.commitWait();
        }
      }
      // java this.mWaitCommitcGridCols = gridCols;
      self.mWaitCommitcGridCols = gridCols;
      */
    }

    // java public GridSizeCommit finish()
    pub fn finish(&mut self) -> Option<Box<dyn GridSizeCommit>> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.commitWait();
      self.commitWait();
      // java GridSizeCommit commit = this.mList != null && this.mList.size() > 0 ? this : null;
      let mut commit: Box<dyn GridSizeCommit> = (if true && (self.mList.len() as i32) > 0 { self } else { <Box<Box<dyn GridSizeCommit>>>::default() });
      // java if (commit == null)
      if false {
        // java this.recycle();
        self.recycle();
      }
      // java return commit;
      return commit;
      */
    }

}
