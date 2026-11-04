// Java: import cn.wps.collections.KList
use util::collections::KList::KList;
// Java: import cn.wps.kfc.impl.arrayList.WIntArrayList
use util::kfc::r#impl::arrayList::WIntArrayList::WIntArrayList;
// Java: import cn.wps.moffice.writer.cache.TypoDrawing
use crate::cn::wps::moffice::writer::cache::TypoDrawing::TypoDrawing;
// Java: import cn.wps.moffice.writer.cache.TypoDrawingLayers
use crate::cn::wps::moffice::writer::cache::TypoDrawingLayers::TypoDrawingLayers;
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
// Java: import cn.wps.moffice.writer.data.table.CellInfo
use crate::cn::wps::moffice::writer::data::table::CellInfo::CellInfo;
// Java: import cn.wps.moffice.writer.data.table.Data
use crate::cn::wps::moffice::writer::data::table::Data::Data;
// Java: import cn.wps.moffice.writer.layout.base.drawing.DrawingResizeTool
use crate::cn::wps::moffice::writer::layout::base::drawing::DrawingResizeTool::DrawingResizeTool;
// Java: import cn.wps.moffice.writer.layout.base.env.MainState
use crate::cn::wps::moffice::writer::layout::base::env::MainState::MainState;
// Java: import cn.wps.moffice.writer.layout.base.env.TableCellLayoutState
use crate::cn::wps::moffice::writer::layout::base::env::TableCellLayoutState::TableCellLayoutState;
// Java: import cn.wps.moffice.writer.layout.base.table.width.IWidthInfo
use crate::cn::wps::moffice::writer::layout::base::table::width::IWidthInfo::IWidthInfo;
// Java: import cn.wps.moffice.writer.layout.base.table.width.IWidthReceiver
use crate::cn::wps::moffice::writer::layout::base::table::width::IWidthReceiver::IWidthReceiver;
// Java: import java.lang.ThreadLocal
use stubs::java::lang::ThreadLocal::ThreadLocal;

// java private static final String TAG = TableCellWidthReceiver.class.getSimpleName();
const TAG: i32 = todo!().getSimpleName();
// java private static ThreadLocal<FitState> sFitStateMap = new ThreadLocal();
static sFitStateMap: std::sync::LazyLock<i32> =
                    std::sync::LazyLock::new(|| ThreadLocal::new());

// java public class TableCellWidthReceiver extends KList.RecycableNode implements IWidthReceiver
#[derive(Default)]
pub struct TableCellWidthReceiver {
    // java KList.RecycableNode super_
    pub(crate) super_: RecycableNode,
    // java private TableCellLayoutState mState;
    mState: Option<TableCellLayoutStateHolder>,
    // java private MainState mMainState;
    mMainState: Option<MainStateHolder>,
    // java private CellInfo mCellInfo;
    mCellInfo: Option<CellInfoHolder>,
    // java private Data.AutoFitInfo mCacheFitInfo = new Data.AutoFitInfo();
    mCacheFitInfo: Option<Data_AutoFitInfoHolder>,
}

// java public static class FitState
#[derive(Default)]
pub struct FitState {
    // java boolean anyCellChanged = false;
    pub(crate) anyCellChanged: bool,
    // java int endRowIndex = -1;
    pub(crate) endRowIndex: i32,
    // java boolean active = false;
    pub(crate) active: bool,
}

// java public static class Alloc extends KList.RecycableNodeAlloc<TableCellWidthReceiver>
#[derive(Default)]
pub struct Alloc {
    // java KList.RecycableNodeAlloc<TableCellWidthReceiver> super_
    pub(crate) super_: RecycableNodeAlloc<TableCellWidthReceiver>,
}

// java public interface IWidthReceiver
impl IWidthReceiver for TableCellWidthReceiver {
    // java public boolean isComplete()
    fn isComplete(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mCellInfo.getData().getAutoFitInfo().isComplete();
      return self.mCellInfo.getData().getAutoFitInfo().isComplete();
      */
    }

    // java public void submitInfo(IWidthInfo info)
    fn submitInfo(&self, info: Option<IWidthInfoHolder>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.isTextDirNormal())
      if self.isTextDirNormal() {
        // java if (this.mCacheFitInfo.commitNewInfo(info.getMinWidth(), info.getMaxWidth(), info.getAutoWidthTotal()))
        if self.mCacheFitInfo.commitNewInfo(info.getMinWidth(), info.getMaxWidth(), info.getAutoWidthTotal()) {
        }
      }
      // java else
      else {
        // java this.mCacheFitInfo.addNewMaxInfo(info.getMinWidth(), info.getMaxWidth());
        self.mCacheFitInfo.addNewMaxInfo(info.getMinWidth(), info.getMaxWidth());
      }
      */
    }

    // java public boolean isIgnoreGraphParse()
    fn isIgnoreGraphParse(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.isFitText();
      return self.isFitText();
      */
    }

    // java public boolean isTextDirNormal()
    fn isTextDirNormal(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mState.curPage instanceof TypoTableCell)
      if self.mState.curPage.downcast_ref::<TypoTableCell>().is_some() {
        // java TypoTableCell typoTableCell = (TypoTableCell)this.mState.curPage;
        let mut typoTableCell: TypoTableCell = (self.mState.curPage).downcast_ref::<TypoTableCell>().unwrap();
        // java if (typoTableCell.getTextFlow() == 0)
        if typoTableCell.getTextFlow() == 0 {
          // java return true;
          return true;
        }
      }
      // java return false;
      return false;
      */
    }

    // java public boolean isAutoWidth()
    fn isAutoWidth(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java CellInfo cellInfo = this.getCell();
      let mut cellInfo: CellInfo = { let __x: CellInfo = todo!("no align strategy matched"); __x };
      // java return cellInfo != null ? cellInfo.isAutoWidth() : false;
      return (if true { cellInfo.isAutoWidth() } else { false });
      */
    }

    // java public boolean isAloneCache()
    fn isAloneCache(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return false;
      return false;
      */
    }

    // java public boolean isInTableHeader()
    fn isInTableHeader(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return false;
      return false;
      */
    }

    // java public boolean finish()
    fn finish(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.isFitText())
      if self.isFitText() {
        // java this.commitForFitText();
        self.commitForFitText();
      }
      // java else
      else {
        // java this.commitWrapObject();
        self.commitWrapObject();
      }
      // java boolean updated = false;
      let mut updated: bool = false;
      // java if (this.mCacheFitInfo.isComplete())
      if self.mCacheFitInfo.isComplete() {
        // java CellInfo cellInfo = this.getCell();
        let mut cellInfo: CellInfo = { let __x: CellInfo = todo!("no align strategy matched"); __x };
        // java if (cellInfo != null)
        if true {
          // java updated = cellInfo.getData().getAutoFitInfo().updateFrom(this.mCacheFitInfo);
          updated = cellInfo.getData().getAutoFitInfo().updateFrom(self.mCacheFitInfo);
          // java getFitState().anyCellChanged = updated;
          Self::getFitState().anyCellChanged = updated;
        }
      }
      // java return updated;
      return updated;
      */
    }

}

// java public class TableCellWidthReceiver extends KList.RecycableNode implements IWidthReceiver
impl TableCellWidthReceiver {
    // java public static FitState getFitState()
    pub fn getFitState() -> Option<FitStateHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java FitState state = (FitState)sFitStateMap.get();
      let mut state: FitState = (sFitStateMap.get()).downcast_ref::<FitState>().unwrap();
      // java if (state == null)
      if false {
        // java state = new FitState();
        state = FitState::new();
        // java sFitStateMap.set(state);
        sFitStateMap.set(state);
      }
      // java return state;
      return { let __x: Option<FitStateHolder> = todo!("no align strategy matched"); __x };
      */
    }

    // java public boolean init(TableCellLayoutState env, MainState mainState)
    pub fn init(&mut self, env: &TableCellLayoutState, mainState: &MainState) -> Option<bool> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mState = env;
      self.mState = env;
      // java this.mMainState = mainState;
      self.mMainState = mainState;
      // java this.mCellInfo = this.getCell();
      self.mCellInfo = self.getCell();
      // java return this.mCellInfo != null;
      return self.mCellInfo.is_some();
      */
    }

    // java public  TableCellWidthReceiver()
    pub fn TableCellWidthReceiver() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java super(false);
      Self { super_: RecycableNode::new(false), mState: Default::default(), mMainState: Default::default(), mCellInfo: Default::default(), mCacheFitInfo: Default::default() }
      */
    }

    // java private void commitWrapObject()
    pub fn commitWrapObject(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mState.curPage instanceof TypoTableCell)
      if self.mState.curPage.downcast_ref::<TypoTableCell>().is_some() {
        // java TypoDrawingLayers layers = this.mMainState.getDrawingLayers(this.mState.document);
        let mut layers: TypoDrawingLayers = self.mMainState.getDrawingLayers__document(self.mState.document);
        // java if (layers != null)
        if true {
          // java TypoSnapshot snapshot = this.mMainState.snapshot;
          let mut snapshot: TypoSnapshot = self.mMainState.snapshot;
          // java TypoToolsManager typoTools = snapshot.getTypoTools();
          let mut typoTools: TypoToolsManager = snapshot.getTypoTools();
          // java WIntArrayList floatList = typoTools.obtainIntArrayList();
          let mut floatList: WIntArrayList = typoTools.obtainIntArrayList();
          // java if (layers.getWraps(floatList, this.mState.curPage.getPointer()) > 0)
          if layers.getWraps(floatList, self.mState.curPage.getPointer()) > 0 {
            // java int drawingMaxWidth = 0;
            let mut drawingMaxWidth: i32 = 0;
            // java TypoRect rect = typoTools.obtainRect();
            let mut rect: TypoRect = typoTools.obtainRect();
            // java TypoDrawing drawing = typoTools.obtainTypoDrawing();
            let mut drawing: TypoDrawing = typoTools.obtainTypoDrawing();
            // java int i = 0;
            let mut i: i32 = 0;
            let mut size: i32 = (floatList.len() as i32);
            // java for(int size = floatList.size(); i < size; ++i)
            while i < size {
              // java drawing.assignment(floatList.getQuick(i), snapshot);
              drawing.assignment(floatList.getQuick(i), snapshot);
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
            // java typoTools.recycle(rect);
            typoTools.recycle(rect);
            // java if (drawingMaxWidth > 0)
            if drawingMaxWidth > 0 {
              // java this.mCacheFitInfo.commitNewInfo(0, drawingMaxWidth, 0);
              self.mCacheFitInfo.commitNewInfo(0, drawingMaxWidth, 0);
            }
          }
          // java typoTools.recycle(floatList);
          typoTools.recycle(floatList);
        }
      }
      */
    }

    // java public void clear()
    pub fn clear(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mState = null;
      self.mState = None;
      // java this.mCellInfo = null;
      self.mCellInfo = None;
      // java this.mCacheFitInfo.clear();
      self.mCacheFitInfo.clear();
      */
    }

    // java private CellInfo getCell()
    pub fn getCell(&self) -> Option<CellInfoHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mCellInfo == null)
      if self.mCellInfo.is_none() {
        // java CellInfo cellInfo = this.mState.document.getTableInfos().getCellInfoByLevel(this.mState.cp, this.mState.tablePeriLayer - 1);
        let mut cellInfo: CellInfo = self.mState.document.getTableInfos().getCellInfoByLevel(self.mState.cp, self.mState.tablePeriLayer - 1);
        // java this.mCellInfo = cellInfo;
        self.mCellInfo = cellInfo;
      }
      // java return this.mCellInfo;
      return self.mCellInfo;
      */
    }

    // java private boolean isFitText()
    pub fn isFitText(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mState.curPage instanceof TypoTableCell)
      if self.mState.curPage.downcast_ref::<TypoTableCell>().is_some() {
        // java TypoTableCell typoTableCell = (TypoTableCell)this.mState.curPage;
        let mut typoTableCell: TypoTableCell = (self.mState.curPage).downcast_ref::<TypoTableCell>().unwrap();
        // java if (typoTableCell.isFitText() && typoTableCell.getTextFlow() == 0)
        if typoTableCell.isFitText() && typoTableCell.getTextFlow() == 0 {
          // java return true;
          return true;
        }
      }
      // java return false;
      return false;
      */
    }

    // java private void commitForFitText()
    pub fn commitForFitText(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java CellInfo cellInfo = this.getCell();
      let mut cellInfo: CellInfo = { let __x: CellInfo = todo!("no align strategy matched"); __x };
      // java int fitLayoutWidth = this.mState.pageEnv.getCoreWidth();
      let mut fitLayoutWidth: i32 = self.mState.pageEnv.getCoreWidth();
      // java if (cellInfo != null)
      if true {
        // java fitLayoutWidth = cellInfo.getWidth() - cellInfo.getMarginLeft() - cellInfo.getMarginRight();
        fitLayoutWidth = cellInfo.getWidth() - cellInfo.getMarginLeft() - cellInfo.getMarginRight();
      }
      // java this.mCacheFitInfo.commitNewInfo(fitLayoutWidth, fitLayoutWidth, 0);
      self.mCacheFitInfo.commitNewInfo(fitLayoutWidth, fitLayoutWidth, 0);
      */
    }

}

impl std::ops::Deref for TableCellWidthReceiver {
    type Target = RecycableNode;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TableCellWidthReceiver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public static class FitState
impl FitState {
    // java public void reSet(boolean active, int endRowIndex, boolean hasChange)
    pub fn reSet(&mut self, active: bool, endRowIndex: i32, hasChange: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.active = active;
      self.active = active;
      // java this.anyCellChanged = hasChange;
      self.anyCellChanged = hasChange;
      // java this.endRowIndex = endRowIndex;
      self.endRowIndex = endRowIndex;
      */
    }

    // java public boolean isActive()
    pub fn isActive(&self) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.active;
      return self.active;
      */
    }

}

// java public static class Alloc extends KList.RecycableNodeAlloc<TableCellWidthReceiver>
impl Alloc {
    // java public TableCellWidthReceiver alloc()
    pub fn alloc(&self) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return new TableCellWidthReceiver();
      return { let __x: Self = todo!("no align strategy matched"); __x };
      */
    }

    // java public void init(TableCellWidthReceiver node)
    pub fn init(&mut self, node: &mut TableCellWidthReceiver) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java super.init(node);
      self.super_.init(node);
      // java node.clear();
      node.clear();
      */
    }

}

impl std::ops::Deref for Alloc {
    type Target = RecycableNodeAlloc<TableCellWidthReceiver>;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for Alloc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
