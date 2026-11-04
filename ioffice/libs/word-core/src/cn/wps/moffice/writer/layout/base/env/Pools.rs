// Java: import cn.wps.collections.KArrayDeque
use util::collections::KArrayDeque::KArrayDeque;
// Java: import cn.wps.collections.KList
use util::collections::KList::KList;
// Java: import cn.wps.collections.KPList
use util::collections::KPList::KPList;
// Java: import cn.wps.collections.KPool
use util::collections::KPool::KPool;
// Java: import cn.wps.collections.KSyncPool
use util::collections::KSyncPool::KSyncPool;
// Java: import cn.wps.moffice.writer.layout.base.balloons.CommentCacheCollector
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::balloons::CommentCacheCollector::CommentCacheCollector;
// Java: import cn.wps.moffice.writer.layout.base.bidi.BidiItem
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::bidi::BidiItem::BidiItem;
// Java: import cn.wps.moffice.writer.layout.base.edit.Event
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::edit::Event::Event;
// Java: import cn.wps.moffice.writer.layout.base.line.SubLineEnv
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::line::SubLineEnv::SubLineEnv;
// Java: import cn.wps.moffice.writer.layout.base.table.TableCellWidthReceiver
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::table::TableCellWidthReceiver::TableCellWidthReceiver;
// Java: import cn.wps.moffice.writer.layout.base.env.TableCellLayoutState
use crate::cn::wps::moffice::writer::layout::base::env::TableCellLayoutState::TableCellLayoutState;
// Java: import cn.wps.moffice.writer.layout.base.env.DrawingCacheItem
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::DrawingCacheItem::DrawingCacheItem;
// Java: import cn.wps.moffice.writer.layout.base.env.TBD
use crate::cn::wps::moffice::writer::layout::base::env::TBD::TBD;
// Java: import cn.wps.moffice.writer.layout.base.env.FootNoteCacheItem
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::FootNoteCacheItem::FootNoteCacheItem;
// Java: import cn.wps.moffice.writer.layout.base.env.ParaPropEnv
use crate::cn::wps::moffice::writer::layout::base::env::ParaPropEnv::ParaPropEnv;
// Java: import cn.wps.moffice.writer.layout.base.env.StateSnapshot
// CARGO_OFF_SURFACE use crate::cn::wps::moffice::writer::layout::base::env::StateSnapshot::StateSnapshot;


// CARGO_OFF_SURFACE_TYPES
#[derive(Default, Clone)]
pub struct TableCellWidthReceiver;
#[derive(Default, Clone)]
pub struct DrawingCacheItem;
#[derive(Default, Clone)]
pub struct BidiItem;
#[derive(Default, Clone)]
pub struct Event;
#[derive(Default, Clone)]
pub struct FootNoteCacheItem;
#[derive(Default, Clone)]
pub struct StateSnapshot;
#[derive(Default, Clone)]
pub struct SubLineEnv;
pub mod CommentCacheCollector {
    #[derive(Default, Clone)]
    pub struct CommentCacheItem;
}
// java public final class Pools
#[derive(Default)]
pub struct Pools {
    // java private KPool<TableCellLayoutState> mCellStatePool = new KPool(new TableCellLayoutState.Alloc(this));
    mCellStatePool: Option<CellStatePoolHolder>,
    // java private KPool<DrawingCacheItem> mDrawingCachePool = new KPool(new DrawingCacheItem.Alloc());
    mDrawingCachePool: Vec<crate::cn::wps::moffice::writer::layout::base::env::DrawingCacheItem::DrawingCacheItem>,
    // java private KPool<BidiItem> mBidiItem = new KPool(new BidiItem.Alloc());
    mBidiItem: Option<BidiPoolHolder>,
    // java private KPList.INodeAlloc<SubLineEnv> mSubLineEnvAlloc = new SubLineEnv.Alloc();
    mSubLineEnvAlloc: Option<SubLineEnvAllocHolder>,
    // java private KPool<TBD> mTBDPool = new KPool(new TBD.Alloc());
    mTBDPool: Vec<TBD>,
    // java private KSyncPool<Event> mEventPool = new KSyncPool(new Event.Alloc());
    mEventPool: Option<EventPoolHolder>,
    // java private KPool<CommentCacheCollector.CommentCacheItem> mCommentCacheItemPool = new KPool(new CommentCacheCollector.CommentCacheItem.Alloc());
    mCommentCacheItemPool: Option<CommentPoolHolder>,
    // java private KPool<FootNoteCacheItem> mFootEndNoteCacheItemPool = new KPool(new FootNoteCacheItem.Alloc());
    mFootEndNoteCacheItemPool: Option<FootNotePoolHolder>,
    // java private KArrayDeque<ParaPropEnv> mParaPropEnv = new KArrayDeque();
    mParaPropEnv: Vec<ParaPropEnv>,
    // java private KPool<TableCellWidthReceiver> mTableCellWidthReceiverAlloc = new KPool(new TableCellWidthReceiver.Alloc());
    mTableCellWidthReceiverAlloc: Option<CellWidthPoolHolder>,
    // java private KPool<StateSnapshot> mStateSnapshotPool = new KPool(new StateSnapshot.Alloc(this));
    mStateSnapshotPool: Option<StateSnapshotPoolHolder>,
}

// java public final class Pools
impl Pools {
    // java public void reuseClean()
    pub fn reuseClean(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mCellStatePool = new KPool(new TableCellLayoutState.Alloc(this));
      self.mCellStatePool = KPool::new(Alloc::new(self));
      */
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null != this.mCellStatePool)
      if let Some(mCellStatePool) = self.mCellStatePool.as_mut() {
        // java this.mCellStatePool.dispose();
        mCellStatePool.dispose();
        // java this.mCellStatePool = null;
        mCellStatePool = None;
      }
      // java if (null != this.mDrawingCachePool)
      if let Some(mDrawingCachePool) = self.mDrawingCachePool.as_mut() {
        // java this.mDrawingCachePool.dispose();
        mDrawingCachePool.dispose();
        // java this.mDrawingCachePool = null;
        mDrawingCachePool = None;
      }
      // java if (null != this.mBidiItem)
      if let Some(mBidiItem) = self.mBidiItem.as_mut() {
        // java this.mBidiItem.dispose();
        mBidiItem.dispose();
        // java this.mBidiItem = null;
        mBidiItem = None;
      }
      // java this.mSubLineEnvAlloc = null;
      self.mSubLineEnvAlloc = None;
      // java if (null != this.mTBDPool)
      self.mTBDPool.clear();
      // java if (null != this.mEventPool)
      if let Some(mEventPool) = self.mEventPool.as_mut() {
        // java this.mEventPool.dispose();
        mEventPool.dispose();
        // java this.mEventPool = null;
        mEventPool = None;
      }
      // java if (null != this.mCommentCacheItemPool)
      if let Some(mCommentCacheItemPool) = self.mCommentCacheItemPool.as_mut() {
        // java this.mCommentCacheItemPool.dispose();
        mCommentCacheItemPool.dispose();
        // java this.mCommentCacheItemPool = null;
        mCommentCacheItemPool = None;
      }
      // java if (null != this.mFootEndNoteCacheItemPool)
      if let Some(mFootEndNoteCacheItemPool) = self.mFootEndNoteCacheItemPool.as_mut() {
        // java this.mFootEndNoteCacheItemPool.dispose();
        mFootEndNoteCacheItemPool.dispose();
        // java this.mFootEndNoteCacheItemPool = null;
        mFootEndNoteCacheItemPool = None;
      }
      // java if (null != this.mStateSnapshotPool)
      if let Some(mStateSnapshotPool) = self.mStateSnapshotPool.as_mut() {
        // java this.mStateSnapshotPool.dispose();
        mStateSnapshotPool.dispose();
        // java this.mStateSnapshotPool = null;
        mStateSnapshotPool = None;
      }
      */
    }

    // java public TableCellLayoutState obtainTableCellLayoutState()
    pub fn obtainTableCellLayoutState(&mut self) -> TableCellLayoutState {
      // java return (TableCellLayoutState)this.mCellStatePool.obtain();
      TableCellLayoutState::TableCellLayoutState()
    }

    // java public TableCellWidthReceiver obtainTableCellWidthReceiver()
    pub fn obtainTableCellWidthReceiver(&mut self) -> TableCellWidthReceiver {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return (TableCellWidthReceiver)this.mTableCellWidthReceiverAlloc.obtain();
      return (self.mTableCellWidthReceiverAlloc.obtain()).downcast_ref::<TableCellWidthReceiver>().unwrap();
      */
    }

    // java public void recycleTableCellWidthReveiver(TableCellWidthReceiver receiver)
    pub fn recycleTableCellWidthReveiver(&mut self, receiver: TableCellWidthReceiver) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mTableCellWidthReceiverAlloc.recycle(receiver);
      self.mTableCellWidthReceiverAlloc.recycle(receiver);
      */
    }

    // java public void recycleTableCellLayoutState(TableCellLayoutState cell)
    pub fn recycleTableCellLayoutState(&mut self, cell: TableCellLayoutState) {
      // java this.mCellStatePool.recycle(cell);
      let _ = cell;
    }

    // java public void recycleTableCellLayoutStates(KList<TableCellLayoutState> cells)
    pub fn recycleTableCellLayoutStates(&mut self, cells: Vec<TableCellLayoutState>) {
      // java this.mCellStatePool.recycle(cells);
      let _ = cells;
    }

    // java public DrawingCacheItem obtainDrawingCacheItem()
    pub fn obtainDrawingCacheItem(&mut self) -> crate::cn::wps::moffice::writer::layout::base::env::DrawingCacheItem::DrawingCacheItem {
      self.mDrawingCachePool.pop().unwrap_or_else(crate::cn::wps::moffice::writer::layout::base::env::DrawingCacheItem::DrawingCacheItem::new)
    }

    // java public void recycleDrawingCacheItem(DrawingCacheItem cacheItem)
    pub fn recycleDrawingCacheItem(&mut self, mut cacheItem: crate::cn::wps::moffice::writer::layout::base::env::DrawingCacheItem::DrawingCacheItem) {
      crate::cn::wps::moffice::writer::layout::base::env::DrawingCacheItem::Alloc::new().init(&mut cacheItem);
      self.mDrawingCachePool.push(cacheItem);
    }

    // java public void recycleDrawingCacheItems(KList<DrawingCacheItem> items)
    pub fn recycleDrawingCacheItems(&mut self, items: Vec<crate::cn::wps::moffice::writer::layout::base::env::DrawingCacheItem::DrawingCacheItem>) {
      for mut item in items.into_iter().rev() {
        crate::cn::wps::moffice::writer::layout::base::env::DrawingCacheItem::Alloc::new().init(&mut item);
        self.mDrawingCachePool.push(item);
      }
    }

    // java public BidiItem obtainBidiItem()
    pub fn obtainBidiItem(&mut self) -> BidiItem {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java BidiItem item = (BidiItem)this.mBidiItem.obtain();
      let mut item: BidiItem = (self.mBidiItem.obtain()).downcast_ref::<BidiItem>().unwrap();
      // java this.recycleBidiItems(item.getChildBidis());
      self.recycleBidiItems(item.getChildBidis());
      // java return item;
      return item;
      */
    }

    // java public void recycle(BidiItem item)
    pub fn recycle(&mut self, item: BidiItem) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mBidiItem.recycle(item);
      self.mBidiItem.recycle(item);
      */
    }

    // java public void recycleBidiItems(KList<BidiItem> list)
    pub fn recycleBidiItems(&mut self, list: Vec<BidiItem>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mBidiItem.recycle(list);
      self.mBidiItem.recycle(list);
      */
    }

    // java public KPList.INodeAlloc<SubLineEnv> getSubLineEnvAlloc()
    pub fn getSubLineEnvAlloc(&self) -> Option<SubLineEnvAllocHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mSubLineEnvAlloc;
      return self.mSubLineEnvAlloc;
      */
    }

    // java public TBD obtainTBD()
    pub fn obtainTBD(&mut self) -> TBD {
      // java return (TBD)this.mTBDPool.obtain();
      self.mTBDPool.pop().unwrap_or_else(TBD::TBD)
    }

    // java public void recycleTBD(TBD tbd)
    pub fn recycleTBD(&mut self, tbd: TBD) {
      // java this.mTBDPool.recycle(tbd);
      self.mTBDPool.push(tbd);
    }

    // java public void recycleTBDs(KList<TBD> list)
    pub fn recycleTBDs(&mut self, list: Vec<TBD>) {
      // java this.mTBDPool.recycle(list);
      self.mTBDPool.extend(list);
    }

    // java public Event obtainEvent()
    pub fn obtainEvent(&mut self) -> Event {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return (Event)this.mEventPool.obtain();
      return (self.mEventPool.obtain()).downcast_ref::<Event>().unwrap();
      */
    }

    // java public void recycleEvent(Event item)
    pub fn recycleEvent(&mut self, item: Event) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mEventPool.recycle(item);
      self.mEventPool.recycle(item);
      */
    }

    // java public void recycleEvents(KList<Event> list)
    pub fn recycleEvents(&mut self, list: Vec<Event>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mEventPool.recycle(list);
      self.mEventPool.recycle(list);
      */
    }

    // java public CommentCacheCollector.CommentCacheItem obtainCommentCacheItem()
    pub fn obtainCommentCacheItem(&mut self) -> CommentCacheCollector::CommentCacheItem {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return (CommentCacheCollector.CommentCacheItem)this.mCommentCacheItemPool.obtain();
      return (self.mCommentCacheItemPool.obtain()).downcast_ref::<CommentCacheCollector::CommentCacheItem>().unwrap();
      */
    }

    // java public void recycleCommentCacheItem(CommentCacheCollector.CommentCacheItem item)
    pub fn recycleCommentCacheItem(&mut self, item: CommentCacheCollector::CommentCacheItem) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java item.commentStartNode = null;
      item.commentStartNode = None;
      // java this.mCommentCacheItemPool.recycle(item);
      self.mCommentCacheItemPool.recycle(item);
      */
    }

    // java public void recycleCommentCacheItems(KList<CommentCacheCollector.CommentCacheItem> list)
    pub fn recycleCommentCacheItems(&mut self, list: Vec<CommentCacheCollector::CommentCacheItem>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mCommentCacheItemPool.recycle(list);
      self.mCommentCacheItemPool.recycle(list);
      */
    }

    // java public FootNoteCacheItem obtainFootEndCacheItem()
    pub fn obtainFootEndCacheItem(&mut self) -> FootNoteCacheItem {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return (FootNoteCacheItem)this.mFootEndNoteCacheItemPool.obtain();
      return (self.mFootEndNoteCacheItemPool.obtain()).downcast_ref::<FootNoteCacheItem>().unwrap();
      */
    }

    // java public void recycleFootEndNoteCacheItem(FootNoteCacheItem val)
    pub fn recycleFootEndNoteCacheItem(&mut self, val: FootNoteCacheItem) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mFootEndNoteCacheItemPool.recycle(val);
      self.mFootEndNoteCacheItemPool.recycle(val);
      */
    }

    // java public void recycleFootEndNoteCacheItems(KList<FootNoteCacheItem> list)
    pub fn recycleFootEndNoteCacheItems(&mut self, list: Vec<FootNoteCacheItem>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mFootEndNoteCacheItemPool.recycle(list);
      self.mFootEndNoteCacheItemPool.recycle(list);
      */
    }

    // java public ParaPropEnv obtainParaPropEnv()
    pub fn obtainParaPropEnv(&mut self) -> ParaPropEnv {
      // java return this.mParaPropEnv.isEmpty() ? new ParaPropEnv() : (ParaPropEnv)this.mParaPropEnv.pop();
      self.mParaPropEnv.pop().unwrap_or_default()
    }

    // java public void recycleParaPropEnv(ParaPropEnv propEnv)
    pub fn recycleParaPropEnv(&mut self, propEnv: ParaPropEnv) {
      // java propEnv.init();
      let mut propEnv = propEnv;
      propEnv.init();
      // java this.mParaPropEnv.push(propEnv);
      self.mParaPropEnv.push(propEnv);
    }

    // java public StateSnapshot obtainStateSnapshot()
    pub fn obtainStateSnapshot(&mut self) -> StateSnapshot {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return (StateSnapshot)this.mStateSnapshotPool.obtain();
      return (self.mStateSnapshotPool.obtain()).downcast_ref::<StateSnapshot>().unwrap();
      */
    }

    // java public void recycleStatesSnapshot(KList<StateSnapshot> list)
    pub fn recycleStatesSnapshot(&mut self, list: Vec<StateSnapshot>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mStateSnapshotPool.recycle(list);
      self.mStateSnapshotPool.recycle(list);
      */
    }

    // java public void recycle(StateSnapshot stateSnapshot)
    pub fn recycle__statesnapshot(&mut self, stateSnapshot: StateSnapshot) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mStateSnapshotPool.recycle(stateSnapshot);
      self.mStateSnapshotPool.recycle(stateSnapshot);
      */
    }

}

#[derive(Default, Clone)]
pub struct CellStatePoolHolder;
#[derive(Default, Clone)]
pub struct DrawingCachePoolHolder;
#[derive(Default, Clone)]
pub struct BidiPoolHolder;
#[derive(Default, Clone)]
pub struct SubLineEnvAllocHolder;
#[derive(Default, Clone)]
pub struct TbdPoolHolder;
#[derive(Default, Clone)]
pub struct EventPoolHolder;
#[derive(Default, Clone)]
pub struct CommentPoolHolder;
#[derive(Default, Clone)]
pub struct FootNotePoolHolder;
#[derive(Default, Clone)]
pub struct CellWidthPoolHolder;
#[derive(Default, Clone)]
pub struct StateSnapshotPoolHolder;
