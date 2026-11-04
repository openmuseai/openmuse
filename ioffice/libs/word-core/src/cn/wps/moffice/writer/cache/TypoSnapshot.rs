#![allow(dead_code, unused_variables, unused_imports, non_snake_case, unused_mut, unreachable_code, unused_assignments)]
// Java: import cn.wps.base.assertion.Assert
// Java: import cn.wps.collections.IShareObjIndexdMap
// Java: import cn.wps.kfc.api.list.IIntList
// Java: import cn.wps.kfc.api.map.IIntObjectMap
// Java: import cn.wps.kfc.impl.arrayList.WIntArrayList
// Java: import cn.wps.kfc.impl.map.WIntObjectHashMap
// Java: import cn.wps.moffice.util.BoolBitSet
// Java: import cn.wps.moffice.util.IntBitSet
// Java: import cn.wps.moffice.writer.cache.disk.BlocksSwapManager
// Java: import cn.wps.moffice.writer.cache.disk.CrackRanges
// Java: import cn.wps.moffice.writer.cache.disk.SwapConfig
// Java: import cn.wps.moffice.writer.cache.disk.TypoMemBlockEx
// Java: import cn.wps.moffice.writer.cache.firstcache.FirstCacheBase
// Java: import cn.wps.moffice.writer.cache.firstcache.PersistObjs
// Java: import cn.wps.moffice.writer.cache.firstcache.TypoFirstInfo
// Java: import cn.wps.moffice.writer.core.Document
use crate::cn::wps::moffice::writer::core::Document::Document;
// Java: import cn.wps.moffice.writer.core.TextDocument
// Java: import cn.wps.moffice.writer.core.pool.ObjectPool
// Java: import cn.wps.moffice.writer.core.pool.PoolableObjectFactory
// Java: import cn.wps.moffice.writer.data.RANGEFactory
// Java: import cn.wps.moffice.writer.layout.base.smallerscale.IPosVisitor
// Java: import cn.wps.moffice.writer.layout.base.smallerscale.PagePositionMap
// Java: import cn.wps.moffice.writer.layout.base.smallerscale.PosVistitorImpl
// Java: import java.util.ArrayList
// Java: import java.util.Arrays
// Java: import cn.wps.moffice.writer.cache.TypoToolsManager
// Java: import cn.wps.moffice.writer.cache.TypoMemBlock
// Java: import cn.wps.moffice.writer.cache.TypoMemoryObj
// Java: import cn.wps.moffice.writer.cache.TypoDocument
// Java: import cn.wps.moffice.writer.cache.TypoMemBlockManager
// Java: import java.lang.Object
// Java: import cn.wps.moffice.writer.cache.TypoObjsIndexdMap
// Java: import cn.wps.moffice.writer.cache.RevisionCache
// Java: import cn.wps.kfc.html.reader.lexical.token.Item
// Java: import cn.wps.moffice.writer.cache.TypoCache
// Java: import cn.wps.moffice.writer.cache.ITypoRect
use crate::cn::wps::moffice::writer::cache::ITypoRect::*;

#[derive(Default, Clone)]
pub struct IPosVisitor;
impl IPosVisitor {
    pub fn getContentRect(&self) -> IPosContentRect {
        IPosContentRect
    }
}
#[derive(Default, Clone)]
pub struct IPosContentRect;
impl IPosContentRect {
    pub fn getBottom(&self) -> i32 {
        0
    }
}

#[derive(Default, Clone)]
pub struct WIntArrayList;
#[derive(Default, Clone)]
struct TypoMemBlockManager;
impl TypoMemBlockManager { fn getInstance() -> i32 { 0 } }
#[derive(Default, Clone)]
struct TypoObjectFactory;
#[derive(Default, Clone)]
struct ObjectPool;
impl ObjectPool {
    fn new(_a: bool, _b: PoolableObjectFactory) -> i32 { 0 }
}
#[derive(Default, Clone)]
struct PoolableObjectFactory;
trait IShareObjIndexdMap {}
trait IIntList {}
impl PoolableObjectFactory {
    fn new() -> Self { Self }
}
// java private static String TAG = "";
static TAG: i32 = 0;
// java private static final boolean DEBUG = false;
const DEBUG: i32 = 0;
// java private static final boolean VERIFY_THREAD = false;
const VERIFY_THREAD: i32 = 0;
// java private static final boolean LOG_STACK = false;
const LOG_STACK: i32 = 0;
// java public static final int VER_EMPTY = -1;
pub(crate) const VER_EMPTY: i32 = -1;
// java public static final int VER_FIRSTCACHE = 0;
pub(crate) const VER_FIRSTCACHE: i32 = 0;
// java public static final int VER_VALIDBEGIN = 1;
pub(crate) const VER_VALIDBEGIN: i32 = 1;
// java private static final int POINTER_OF_CACHE = 1;
const POINTER_OF_CACHE: i32 = 1;
// java private static final ObjectPool<TypoSnapshot> sTypoSnapshotPool = new ObjectPool<TypoSnapshot>(true, new PoolableObjectFactory<TypoSnapshot>() { private int count = 0; public TypoSnapshot newObject() { return new TypoSnapshot(); } });
static sTypoSnapshotPool: std::sync::LazyLock<i32> =
                    std::sync::LazyLock::new(|| ObjectPool::new(true, PoolableObjectFactory::new()));
// java private static ArrayList<TypoSnapshot> sTraceSnapshotList;
static sTraceSnapshotList: std::sync::OnceLock<i32> =
                std::sync::OnceLock::new();
// java private static ArrayList<SafeTrace> sTraceList = new ArrayList();
static sTraceList: std::sync::LazyLock<i32> =
                    std::sync::LazyLock::new(|| 0);
// java private static final ObjectPool<Data> sDataPool = new ObjectPool<Data>(true, new PoolableObjectFactory<Data>() { public Data newObject() { return new Data(); } });
static sDataPool: std::sync::LazyLock<i32> =
                    std::sync::LazyLock::new(|| ObjectPool::new(true, PoolableObjectFactory::new()));
// java private static final TypoMemBlockManager sMemBlockManager = TypoMemBlockManager.getInstance();
static sMemBlockManager: std::sync::LazyLock<i32> =
                    std::sync::LazyLock::new(|| TypoMemBlockManager::getInstance());

// java TypoSections.startSection. Item stride is 7. Type word is 25.
fn section_capacity(start: i32, sec_count: i32) -> i32 {
    let mut capacity = start;
    while capacity < sec_count {
        let next = capacity << 1;
        if next <= capacity {
            break;
        }
        capacity = next;
    }
    capacity
}

fn alloc_typo_sections(snapshot: &mut TypoSnapshot, capacity: i32) -> i32 {
    let alloc_size = 3 + capacity * 7;
    let pointer = snapshot.alloc(alloc_size);
    snapshot.set__int_int_int(pointer, 25, alloc_size);
    pointer
}

fn start_typo_section(old_sections: i32, sec_index: i32, sec_count: i32, page_index: i32, restart_page_num: bool, page_index_start_at: i32, cur_sec_mso_nfc: i32, snapshot: &mut TypoSnapshot) -> i32 {
    let sections = if old_sections == 0 {
        let capacity = section_capacity(4, sec_count);
        let pointer = alloc_typo_sections(snapshot, capacity);
        snapshot.set__int_int(pointer + 2, sec_index + 1);
        pointer
    } else {
        let old_capacity = (snapshot.get__int(old_sections + 1) - 3) / 7;
        let pointer = if sec_count <= old_capacity {
            old_sections
        } else {
            let capacity = section_capacity(old_capacity, sec_count);
            let pointer = alloc_typo_sections(snapshot, capacity);
            let size = snapshot.get__int(old_sections + 2);
            snapshot.copy__int_int_int(old_sections + 3, pointer + 3, size * 7);
            snapshot.set__int_int(pointer + 2, size);
            pointer
        };
        let new_size = snapshot.get__int(pointer + 2).max(sec_index + 1);
        snapshot.set__int_int(pointer + 2, new_size);
        pointer
    };
    let item = sections + 3 + sec_index * 7;
    snapshot.set__int_int_int_int_int(item, page_index, page_index_start_at, if restart_page_num { 1 } else { 0 }, cur_sec_mso_nfc);
    sections
}

fn brc_fields_same(a: &crate::cn::wps::moffice::writer::data::value::BRC::BRC, b: &crate::cn::wps::moffice::writer::data::value::BRC::BRC) -> bool {
  (a.dptLineWidth * 8.0) as i32 == (b.dptLineWidth * 8.0) as i32
    && a.brcType == b.brcType
    && a.color == b.color
    && (a.dptSpace * 8.0) as i32 == (b.dptSpace * 8.0) as i32
    && a.fShadow == b.fShadow
    && a.fFrame == b.fFrame
}

// java public final class TypoSnapshot
#[derive(Default, Clone)]
pub struct TypoSnapshot {
    // java private Throwable mStack;
    mStack: i32,
    // java private final TypoToolsManager mTypoToolsManager = TypoToolsManager.newInstance(this);
    mTypoToolsManager: i32,
    // java private int mCurBlockIndex = -1;
    mCurBlockIndex: i32,
    // java private int mCurBlockStart = -1;
    mCurBlockStart: i32,
    // java private int mCurBlockEnd = -1;
    mCurBlockEnd: i32,
    // java private boolean mCurBlockWriteable;
    mCurBlockWriteable: i32,
    // java private TypoMemBlock mCurBlock;
    mCurBlock: i32,
    // java private Data mData;
    mData: Data,
    // rust: ★wr-29 05 —— 真实数据面（flat heap）：Java 的 TypoMemBlock 块管理降压为
    //       指针寻址 flat Vec（alloc/get/set 语义逐位等价；块加载/交换 GR-D4）
    pub(crate) heap: Vec<i32>,
    // Java TypoObjsIndexdMap shape slots. Index 0 is null; prepareAddShape returns 1-based indexes.
    pub(crate) shape_slots: Vec<Option<std::sync::Arc<std::sync::Mutex<drawing::cn::wps::moffice::drawing::Shape::Shape>>>>,
    // Java TypoObjsIndexdMap.addPos/getPos: the live table path stores TablePos
    // by one-based index in TypoDrawing's +8 slot, just as shape_slots uses +9.
    pub(crate) table_pos_slots: Vec<Option<crate::cn::wps::moffice::writer::data::table::TablePos::TablePos>>,
    // Java ShareObjIndexedMapImpl slots for BRC. Index 0 is null; add_brc returns 1-based indexes.
    pub(crate) brc_slots: Vec<Option<crate::cn::wps::moffice::writer::data::value::BRC::BRC>>,
    // Java TypoObjsIndexdMap.addTypeFace. Index 0 is null.
    pub(crate) typeface_slots: Vec<Option<std::sync::Arc<dyn thirdparty_font::cn::wps::font::ITypeface::ITypeface>>>,
    // Java TypoSnapshot.addObj(SHD). Index 0 is null.
    pub(crate) shd_slots: Vec<Option<drawing::cn::wps::moffice::drawing::graphics::SHD::SHD>>,
    // Java RevisionCache lists. Index is docType * 6 + revisionType. Each item is start, end, authorId.
    pub(crate) revision_slots: Vec<Vec<i32>>,
    // Java TypoObjsIndexdMap.addGeoText. GeoText is still a unit until the drawing type lands.
    pub(crate) geo_slots: Vec<Option<()>>,
    pub(crate) pages: Vec<crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage>,
    // java private IPosVisitor mPosVisitor;
    mPosVisitor: i32,
    // java private boolean mIsRefClone;
    mIsRefClone: i32,
    // java private long mUsingThread = 0L;
    mUsingThread: i32,
    // java private String mUsingThreadName;
    mUsingThreadName: i32,
    // java private boolean __released;
    __released: i32,
    // java private IEntranceObjAlloc mEntranceObjAlloc;
    mEntranceObjAlloc: i32,
    // java private TypoMemoryObj mEntraceObj;
    mEntraceObj: i32,
}

// java public static class SafeTrace implements TypoDocument.ISnapshotListener
#[derive(Default)]
pub struct SafeTrace {
    // java private final String tag;
    tag: i32,
    // java private TypoDocument typoDocument;
    typoDocument: i32,
    // java private TypoSnapshot ref;
    r#ref: i32,
    // java private long usingThread;
    usingThread: i32,
    // java private volatile boolean dirty = true;
    dirty: i32,
}

// java public static class IntArrayList extends WIntArrayList
#[derive(Default)]
pub struct IntArrayList {
    // java WIntArrayList super_
    pub(crate) super_: WIntArrayList,
}

// java static class Data
#[derive(Default, Clone)]
pub struct Data {
    // java protected IShareObjIndexdMap shareObjIndexdMap;
    pub(crate) shareObjIndexdMap: i32,
    // java protected TypoObjsIndexdMap normalObjIndexdMap;
    pub(crate) normalObjIndexdMap: i32,
    // java public ArrayList<TypoMemBlock> blocks = new ArrayList();
    pub(crate) blocks: i32,
    // java public int size = 1;
    pub(crate) size: i32,
    // java public IntArrayList freeList = new IntArrayList();
    pub(crate) freeList: i32,
    // java IntArrayList[] cpRefs = new IntArrayList[7];
    pub(crate) cpRefs: i32,
    pub(crate) cp_ref_slots: [Option<Vec<i32>>; 7],
    // java IntArrayList[] cpRefFreeLists = new IntArrayList[7];
    pub(crate) cpRefFreeLists: i32,
    // java IntArrayList[] revisions;
    pub(crate) revisions: i32,
    // java CrackRanges crackRanges;
    pub(crate) crackRanges: i32,
    // java PagePositionMap pagePositionMap;
    pub(crate) pagePositionMap: i32,
    // java long version = -1L;
    pub(crate) version: i64,
    // java private int refCount;
    refCount: i32,
    // java long modifyPageRange = 0L;
    pub(crate) modifyPageRange: i32,
}

// java public final class TypoSnapshot
impl TypoSnapshot {
    // java public TypoSnapshot newObject()
    pub fn newObject() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return new TypoSnapshot();
      return TypoSnapshot::default();
      */
    }

    // java public static void reuseSnapshotAndCache(TypoDocument typoDocument)
    pub fn reuseSnapshotAndCache(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TypoSnapshot.SafeTrace.releaseAssociatedTraceInstance(typoDocument);
      TypoSnapshot::SafeTrace.releaseAssociatedTraceInstance(typoDocument);
      // java TypoSnapshot.Data.sMemBlockManager.clear();
      TypoSnapshot::Data.sMemBlockManager.clear();
      // java SnapshotVisitorImpl.onClose((TypoDocument)null);
      SnapshotVisitorImpl::onClose(None);
      // java BlocksSwapManager.close();
      BlocksSwapManager::close();
      **/
    }

    // java public static TypoSnapshot createTypoSnapshot(IEntranceObjAlloc entranceObjAlloc)
    pub fn createTypoSnapshot(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TypoSnapshot snapshot = sTypoSnapshotPool.borrowObject();
      let mut snapshot: TypoSnapshot = sTypoSnapshotPool.borrowObject();
      // java snapshot.setEntranceObjAlloc(entranceObjAlloc);
      snapshot.setEntranceObjAlloc(entranceObjAlloc);
      // java snapshot.__released = false;
      snapshot.__released = false;
      // java return snapshot;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java public static void recycleTypoSnaptshot(TypoSnapshot snapshot)
    pub fn recycleTypoSnaptshot(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java snapshot.__released = true;
      snapshot.__released = true;
      // java sTypoSnapshotPool.returnObject(snapshot);
      sTypoSnapshotPool.returnObject(snapshot);
      **/
    }

    // java protected  TypoSnapshot()
    pub fn TypoSnapshot() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      Self { mStack: Default::default(), mTypoToolsManager: Default::default(), mCurBlockIndex: Default::default(), mCurBlockStart: Default::default(), mCurBlockEnd: Default::default(), mCurBlockWriteable: Default::default(), mCurBlock: Default::default(), mData: Default::default(), mPosVisitor: Default::default(), mIsRefClone: Default::default(), mUsingThread: Default::default(), mUsingThreadName: Default::default(), __released: Default::default(), mEntranceObjAlloc: Default::default(), mEntraceObj: Default::default() }
      **/
    }

    // java private void setEntranceObjAlloc(IEntranceObjAlloc entranceObjAlloc)
    pub fn setEntranceObjAlloc(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mEntranceObjAlloc = entranceObjAlloc;
      self.mEntranceObjAlloc = entranceObjAlloc;
      // java this.mEntraceObj = entranceObjAlloc.get();
      self.mEntraceObj = entranceObjAlloc.get();
      **/
    }

    // java public void initCache(IShareObjIndexdMap shareObjIndexdMap, long version)
    pub fn initCache__ishareobjindexdmap_long(&mut self, shareObjIndexdMap: &dyn IShareObjIndexdMap, version: i64) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mTypoToolsManager.updatePool();
      self.mTypoToolsManager.updatePool();
      // java this.mData = new Data(shareObjIndexdMap);
      self.mData = Data::new(shareObjIndexdMap);
      // java this.mData.version = version;
      self.mData.version = version;
      // java this.mIsRefClone = false;
      self.mIsRefClone = false;
      // java this.mData.incRef();
      self.mData.incRef();
      // java this.mEntraceObj.alloc(this);
      self.mEntraceObj.alloc(self);
      **/
    }

    // java private void initCache(Data data)
    pub fn initCache__data(&mut self, data: &Data) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mTypoToolsManager.updatePool();
      self.mTypoToolsManager.updatePool();
      // java this.mData = data;
      self.mData = data;
      // java this.mIsRefClone = false;
      self.mIsRefClone = false;
      // java this.mData.incRef();
      self.mData.incRef();
      // java this.mEntraceObj.alloc(this);
      self.mEntraceObj.alloc(self);
      // java this.mCurBlockIndex = -1;
      self.mCurBlockIndex = -1;
      // java this.mCurBlockStart = -1;
      self.mCurBlockStart = -1;
      // java this.mCurBlockEnd = -1;
      self.mCurBlockEnd = -1;
      **/
    }

    // java  Data getData()
    pub fn getData(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mData;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java  void assignmentData(Data data, boolean isRefClone)
    pub fn assignmentData(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mTypoToolsManager.updatePool();
      self.mTypoToolsManager.updatePool();
      // java this.mData = data;
      self.mData = data;
      // java this.mIsRefClone = isRefClone;
      self.mIsRefClone = isRefClone;
      // java data.incRef();
      data.incRef();
      // java this.mCurBlockIndex = -1;
      self.mCurBlockIndex = -1;
      // java this.mCurBlockStart = -1;
      self.mCurBlockStart = -1;
      // java this.mCurBlockEnd = -1;
      self.mCurBlockEnd = -1;
      // java this.mEntraceObj.assignment(1, this);
      self.mEntraceObj.assignment__int_typosnapshot(1, self);
      **/
    }

    // java public final TypoSnapshot cloneSnapshot()
    pub fn cloneSnapshot(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.cloneSnapshot((TypoSnapshot)null);
      return self.cloneSnapshot__typosnapshot(None);
      **/
    }

    // java public final TypoSnapshot cloneSnapshot(TypoSnapshot reuse)
    pub fn cloneSnapshot__typosnapshot(&self, reuse: Option<&mut TypoSnapshot>) -> TypoSnapshot {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null == reuse)
      if reuse.is_none() {
        // java reuse = createTypoSnapshot(this.mEntranceObjAlloc);
        reuse = Self::createTypoSnapshot(self.mEntranceObjAlloc);
      }
      // java reuse.assignmentData(this.mData, true);
      reuse.assignmentData(self.mData, true);
      // java return reuse;
      return reuse;
      **/
    }

    // java public void release()
    pub fn release(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mPosVisitor != null)
      if true {
        // java this.mPosVisitor.recycle();
        self.mPosVisitor.recycle();
        // java this.mPosVisitor = null;
        self.mPosVisitor = None;
      }
      // java this.mCurBlock = null;
      self.mCurBlock = None;
      // java this.mCurBlockIndex = -1;
      self.mCurBlockIndex = -1;
      // java this.mCurBlockStart = -1;
      self.mCurBlockStart = -1;
      // java this.mCurBlockEnd = -1;
      self.mCurBlockEnd = -1;
      // java this.mCurBlockWriteable = false;
      self.mCurBlockWriteable = false;
      // java this.mUsingThread = 0L;
      self.mUsingThread = 0;
      // java this.mUsingThreadName = null;
      self.mUsingThreadName = None;
      // java this.mData.decRef();
      self.mData.decRef();
      // java this.mData = null;
      self.mData = None;
      // java this.mIsRefClone = false;
      self.mIsRefClone = false;
      // java if (this.mEntraceObj != null && this.mEntranceObjAlloc != null)
      if true && true {
        // java this.mEntranceObjAlloc.recycle(this.mEntraceObj);
        self.mEntranceObjAlloc.recycle(self.mEntraceObj);
      }
      // java this.mEntranceObjAlloc = null;
      self.mEntranceObjAlloc = None;
      // java this.mEntraceObj = null;
      self.mEntraceObj = None;
      // java recycleTypoSnaptshot(this);
      Self::recycleTypoSnaptshot(self);
      **/
    }

    // java public TypoSnapshot fork()
    pub fn fork(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Data newData = this.mData.fork();
      let mut newData: Box<Data> = { let __x: Box<Box<Data>> = todo!("no align strategy matched"); __x };
      // java TypoSnapshot snapshot = createTypoSnapshot(this.mEntranceObjAlloc);
      let mut snapshot: TypoSnapshot = Self::createTypoSnapshot(self.mEntranceObjAlloc);
      // java snapshot.assignmentData(newData, false);
      snapshot.assignmentData(newData, false);
      // java return snapshot;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java public TypoSnapshot cleanFork()
    pub fn cleanFork(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Data newData = this.mData.cleanFork();
      let mut newData: Box<Data> = { let __x: Box<Box<Data>> = todo!("no align strategy matched"); __x };
      // java TypoSnapshot snapshot = createTypoSnapshot(this.mEntranceObjAlloc);
      let mut snapshot: TypoSnapshot = Self::createTypoSnapshot(self.mEntranceObjAlloc);
      // java snapshot.initCache(newData);
      snapshot.initCache(newData);
      // java return snapshot;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java public void reset()
    pub fn reset(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mData.getRefCount() > 1)
      if self.mData.getRefCount() > 1 {
        // java throw new RuntimeException("can NOT reset shared snapshot.");
        panic!("{}", RuntimeException::new("can NOT reset shared snapshot."));
      }
      // java else
      else {
        // java Data var10000 = this.mData;
        let mut var10000: Box<Data> = { let __x: Box<Box<Data>> = todo!("no align strategy matched"); __x };
        // java TypoSnapshot.Data.sMemBlockManager.recycleBlock(this.mData.blocks);
        TypoSnapshot::Data.sMemBlockManager.recycleBlock(self.mData.blocks);
        // java this.mData.resetMaps();
        self.mData.resetMaps();
        // java this.mData.normalObjIndexdMap = this.mData.normalObjIndexdMap.recycleCopy(true);
        self.mData.normalObjIndexdMap = self.mData.normalObjIndexdMap.recycleCopy(true);
        // java this.mData.size = 1;
        self.mData.size = 1;
        // java this.mData.version = -1L;
        self.mData.version = -1;
        // java this.mCurBlockIndex = -1;
        self.mCurBlockIndex = -1;
        // java this.mCurBlockStart = -1;
        self.mCurBlockStart = -1;
        // java this.mCurBlockEnd = -1;
        self.mCurBlockEnd = -1;
        // java this.mCurBlock = null;
        self.mCurBlock = None;
        // java this.mEntraceObj.alloc(this);
        self.mEntraceObj.alloc(self);
      }
      **/
    }

    // java public void recycleNormalObjIndexs(IntArrayList waitRecycleIndexs)
    pub fn recycleNormalObjIndexs(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int curWaitCount = this.mData.normalObjIndexdMap.addRecycleShapeIndexs(waitRecycleIndexs);
      let mut curWaitCount: i32 = self.mData.normalObjIndexdMap.addRecycleShapeIndexs(waitRecycleIndexs);
      // java if (curWaitCount > 1000)
      if curWaitCount > 1000 {
        // java this.mData.normalObjIndexdMap = this.mData.normalObjIndexdMap.recycleCopy(false);
        self.mData.normalObjIndexdMap = self.mData.normalObjIndexdMap.recycleCopy(false);
      }
      **/
    }

    // java public void switchUsingThread()
    pub fn switchUsingThread(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mTypoToolsManager.updatePool();
      self.mTypoToolsManager.updatePool();
      **/
    }

    // java public final boolean isInSameThread()
    pub fn isInSameThread(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return Thread.currentThread().getId() == this.mUsingThread;
      return Thread::currentThread().getId() == self.mUsingThread;
      **/
    }

    // java public final void verifySameThread()
    pub fn verifySameThread(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!this.isInSameThread())
      if !self.isInSameThread() {
        // java if (this.__released)
        if self.__released {
          // java throw new RuntimeException("snapshot already released!");
          panic!("{}", RuntimeException::new("snapshot already released!"));
        }
        // java else
        else {
          // java throw new RuntimeException(this.mUsingThread == 0L ? "typosnapshot guess be recycled" : "must call from same thread, using thread:" + this.mUsingThreadName + " current thread:" + Thread.currentThread().getName());
          panic!("{}", RuntimeException::new((if self.mUsingThread == 0 { "typosnapshot guess be recycled" } else { "must call from same thread, using thread:" + self.mUsingThreadName + " current thread:" + Thread::currentThread().getName() })));
        }
      }
      // java else if (this.__released)
      else if self.__released {
        // java throw new RuntimeException("snapshot already released!");
        panic!("{}", RuntimeException::new("snapshot already released!"));
      }
      **/
    }

    // java public final TypoToolsManager getTypoTools()
    // java public final TypoToolsManager getTypoTools()
    pub fn getTypoTools(&mut self) -> crate::cn::wps::moffice::writer::cache::TypoToolsManager::TypoToolsManager {
      // java return this.mTypoToolsManager;
      crate::cn::wps::moffice::writer::cache::TypoToolsManager::TypoToolsManager::newInstance(self as *mut Self)
    }

    // Java TypoObjsIndexdMap.prepareAddShape: store the shape and return a non-zero index.
    pub fn prepareAddShape(&mut self, shape: std::sync::Arc<std::sync::Mutex<drawing::cn::wps::moffice::drawing::Shape::Shape>>) -> i32 {
      self.shape_slots.push(Some(shape));
      self.shape_slots.len() as i32
    }

    pub fn getShapeSlot(&self, index: i32) -> Option<std::sync::Arc<std::sync::Mutex<drawing::cn::wps::moffice::drawing::Shape::Shape>>> {
      if index <= 0 { return None; }
      self.shape_slots.get((index as usize) - 1).and_then(|slot| slot.clone())
    }

    pub fn addTablePos(&mut self, pos: crate::cn::wps::moffice::writer::data::table::TablePos::TablePos) -> i32 {
      self.table_pos_slots.push(Some(pos));
      self.table_pos_slots.len() as i32
    }

    pub fn getTablePos(&self, index: i32) -> Option<crate::cn::wps::moffice::writer::data::table::TablePos::TablePos> {
      if index <= 0 { return None; }
      self.table_pos_slots.get((index as usize) - 1).and_then(|slot| slot.clone())
    }

    // java TypoObjsIndexdMap.cancleAddShape drops the prepared shape at this index.
    pub fn clear_shape_slot(&mut self, index: i32) {
      if index <= 0 { return; }
      let slot = (index as usize) - 1;
      if slot < self.shape_slots.len() {
        self.shape_slots[slot] = None;
      }
    }

    // java ShareObjIndexedMapImpl.getObjIndex for a BRC: index 0 means a new object.
    pub fn add_brc(&mut self, brc: &mut crate::cn::wps::moffice::writer::data::value::BRC::BRC) -> i32 {
      let mut index = brc.getIndex();
      if index > 0 {
        let same = self.brc_slots.get((index as usize).wrapping_sub(1)).and_then(|slot| slot.as_ref()).map(|stored| stored.getIndex() == index).unwrap_or(false);
        if same {
          self.brc_slots[(index as usize) - 1] = Some(brc.clone());
          return index;
        }
        brc.setIndex(0);
        index = 0;
      }
      if index == 0 {
        self.brc_slots.push(Some(brc.clone()));
        index = self.brc_slots.len() as i32;
        brc.setIndex(index);
        if let Some(slot) = self.brc_slots.last_mut() {
          if let Some(stored) = slot.as_mut() {
            stored.setIndex(index);
          }
        }
      }
      index
    }

    // java TypoObjsIndexdMap.addTypeFace: index 0 is a null typeface.
    pub fn add_typeface(&mut self, face: &Option<std::sync::Arc<dyn thirdparty_font::cn::wps::font::ITypeface::ITypeface>>) -> i32 {
      let Some(face) = face else { return 0; };
      for (i, slot) in self.typeface_slots.iter().enumerate() {
        if let Some(existing) = slot {
          if std::sync::Arc::ptr_eq(existing, face) {
            return (i as i32) + 1;
          }
        }
      }
      self.typeface_slots.push(Some(face.clone()));
      self.typeface_slots.len() as i32
    }

    // java TypoSnapshot.addObj(SHD): null returns 0. Equal shading shares one index.
    pub fn add_shd(&mut self, shd: Option<&drawing::cn::wps::moffice::drawing::graphics::SHD::SHD>) -> i32 {
      let Some(shd) = shd else { return 0; };
      for (i, slot) in self.shd_slots.iter().enumerate() {
        if slot.as_ref().map(|stored| stored == shd).unwrap_or(false) {
          return (i as i32) + 1;
        }
      }
      self.shd_slots.push(Some(shd.clone()));
      let index = self.shd_slots.len() as i32;
      if let Some(slot) = self.shd_slots.last_mut() {
        if let Some(stored) = slot.as_mut() {
          stored.setIndex(index);
        }
      }
      index
    }

    pub fn get_typeface(&self, index: i32) -> Option<std::sync::Arc<dyn thirdparty_font::cn::wps::font::ITypeface::ITypeface>> {
      if index <= 0 { return None; }
      self.typeface_slots.get((index as usize) - 1).and_then(|slot| slot.clone())
    }

    pub fn get_shd(&self, index: i32) -> Option<drawing::cn::wps::moffice::drawing::graphics::SHD::SHD> {
      if index <= 0 { return None; }
      self.shd_slots.get((index as usize) - 1).and_then(|slot| slot.clone())
    }

    // java ShareObjIndexedMapImpl.getObjIndex for a BRC, matched by BRC.equals fields.
    pub fn index_brc(&mut self, brc: &crate::cn::wps::moffice::writer::data::value::BRC::BRC) -> i32 {
      for (i, slot) in self.brc_slots.iter().enumerate() {
        if let Some(stored) = slot {
          if brc_fields_same(stored, brc) {
            return (i as i32) + 1;
          }
        }
      }
      self.brc_slots.push(Some(brc.clone()));
      let index = self.brc_slots.len() as i32;
      if let Some(slot) = self.brc_slots.last_mut() {
        if let Some(stored) = slot.as_mut() {
          stored.setIndex(index);
        }
      }
      index
    }

    // java TypoObjsIndexdMap.addGeoText: null returns 0.
    pub fn add_geo_text(&mut self, geo: Option<()>) -> i32 {
      let Some(geo) = geo else { return 0; };
      if self.geo_slots.is_empty() {
        self.geo_slots.push(Some(geo));
      }
      1
    }

    pub fn get_brc(&self, index: i32) -> Option<crate::cn::wps::moffice::writer::data::value::BRC::BRC> {
      if index <= 0 { return None; }
      self.brc_slots.get((index as usize) - 1).and_then(|slot| slot.clone())
    }

    // java public int alloc(int size)
    // rust: ★wr-29 05 —— 真实 flat alloc：ptr = heap.len()（0 保留为 null），扩容 size。
    //       Java Data.size/blocks 语义等价（getAllocSize 同步）
    pub fn alloc(&mut self, size: i32) -> i32 {
      // Java Data.size starts at 1, so pointer 0 stays NULL.
      if self.heap.is_empty() {
        self.heap.push(0);
        self.mData.size = 1;
      }
      let pointer: i32 = self.heap.len() as i32;
      let grow = size.max(0) as usize;
      for _ in 0..grow { self.heap.push(0); }
      self.mData.size = self.heap.len() as i32;
      pointer
      /* java Data data = this.mData;
      let mut data: Box<Data> = { let __x: Box<Box<Data>> = todo!("no align strategy matched"); __x };
      // java ArrayList<TypoMemBlock> blocks = data.blocks;
      let mut blocks: Vec<TypoMemBlock> = { let __x: Vec<TypoMemBlock> = todo!("no align strategy matched"); __x };
      // java int pointer = data.size;
      let mut pointer: i32 = data.size;
      // java data.size += size;
      data.size = data.size + size;
      // java Data var10000 = this.mData;
      let mut var10000: Box<Data> = { let __x: Box<Box<Data>> = todo!("no align strategy matched"); __x };
      // java TypoMemBlockManager memBlockManager = TypoSnapshot.Data.sMemBlockManager;
      let mut memBlockManager: TypoMemBlockManager = { let __x: TypoMemBlockManager = todo!("no align strategy matched"); __x };
      // java int startBlockIndex = (pointer & ~memBlockManager.mask) >> memBlockManager.bits;
      let mut startBlockIndex: i32 = pointer & !memBlockManager.mask >> memBlockManager.bits;
      // java int endBlockIndex = (data.size - 1 & ~memBlockManager.mask) >> memBlockManager.bits;
      let mut endBlockIndex: i32 = data.size - 1 & !memBlockManager.mask >> memBlockManager.bits;
      // java if (startBlockIndex != this.mCurBlockIndex || this.mCurBlock.refCount != 1)
      if startBlockIndex != self.mCurBlockIndex || self.mCurBlock.refCount != 1 {
        // java if (startBlockIndex == blocks.size())
        if startBlockIndex == (blocks.len() as i32) {
          // java this.mCurBlock = memBlockManager.allocNormalBlock((TypoMemBlock)null);
          self.mCurBlock = memBlockManager.allocNormalBlock(None);
          // java blocks.add(this.mCurBlock);
          blocks.add(self.mCurBlock);
          // java this.mCurBlockIndex = startBlockIndex;
          self.mCurBlockIndex = startBlockIndex;
          // java this.mCurBlockStart = startBlockIndex << memBlockManager.bits;
          self.mCurBlockStart = startBlockIndex << memBlockManager.bits;
          // java this.mCurBlockEnd = this.mCurBlockStart + memBlockManager.blockSize;
          self.mCurBlockEnd = self.mCurBlockStart + memBlockManager.blockSize;
        }
        // java else if (startBlockIndex > blocks.size())
        else if startBlockIndex > (blocks.len() as i32) {
          // java Assert.fail();
          Assert::fail();
        }
        // java else
        else {
          // java this.mCurBlock = (TypoMemBlock)blocks.get(startBlockIndex);
          self.mCurBlock = (blocks.get(startBlockIndex)).downcast_ref::<TypoMemBlock>().unwrap();
          // java if (this.mCurBlock.refCount != 1)
          if self.mCurBlock.refCount != 1 {
            // java this.mCurBlock = memBlockManager.allocNormalBlock(this.mCurBlock);
            self.mCurBlock = memBlockManager.allocNormalBlock(self.mCurBlock);
            // java blocks.set(startBlockIndex, this.mCurBlock);
            blocks.set(startBlockIndex, self.mCurBlock);
          }
          // java this.mCurBlockIndex = startBlockIndex;
          self.mCurBlockIndex = startBlockIndex;
          // java this.mCurBlockStart = startBlockIndex << memBlockManager.bits;
          self.mCurBlockStart = startBlockIndex << memBlockManager.bits;
          // java this.mCurBlockEnd = this.mCurBlockStart + memBlockManager.blockSize;
          self.mCurBlockEnd = self.mCurBlockStart + memBlockManager.blockSize;
        }
      }
      let mut curBlockIndex: i32 = startBlockIndex + 1;
      // java for(int curBlockIndex = startBlockIndex + 1; curBlockIndex <= endBlockIndex ++curBlockIndex)
      while curBlockIndex <= endBlockIndex {
        // java if (curBlockIndex == blocks.size())
        if curBlockIndex == (blocks.len() as i32) {
          // java blocks.add(memBlockManager.allocNormalBlock((TypoMemBlock)null));
          blocks.add(memBlockManager.allocNormalBlock(None));
        }
        // java else if (curBlockIndex > blocks.size())
        else if curBlockIndex > (blocks.len() as i32) {
          // java Assert.fail();
          Assert::fail();
        }
        // java else
        else {
          // java TypoMemBlock block = (TypoMemBlock)blocks.get(curBlockIndex);
          let mut block: TypoMemBlock = (blocks.get(curBlockIndex)).downcast_ref::<TypoMemBlock>().unwrap();
          // java block.valid();
          block.valid();
          // java if (block.refCount != 1)
          if block.refCount != 1 {
            // java blocks.set(curBlockIndex, memBlockManager.allocNormalBlock(block));
            blocks.set(curBlockIndex, memBlockManager.allocNormalBlock(block));
          }
        }
      }
      // java return pointer;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java public final boolean checkExtendSelfEnable(int srcPointer, int srcLen, int extendLen)
    pub fn checkExtendSelfEnable(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return srcPointer + srcLen == this.mData.size;
      return srcPointer + srcLen == self.mData.size;
      **/
    }

    // java public final boolean checkIn(int srcPointer)
    pub fn checkIn(&self, srcPointer: i32) -> bool {
      srcPointer >= 1 && !self.hasFree__int(srcPointer)
    }

    // java public final boolean checkMem(int pointer, int len)
    pub fn checkMem(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return pointer >= 1 && !this.hasFree(pointer, len);
      return pointer >= 1 && !self.hasFree__int_int(pointer, len);
      **/
    }

    // java public int extendAlloc(int srcPointer, int srcLen, int extendLen)
    pub fn extendAlloc(&mut self, srcPointer: i32, srcLen: i32, extendLen: i32) -> i32 {
      // Java: if (srcPointer + srcLen == this.mData.size) {
      if srcPointer + srcLen == self.mData.size {
        // Java: this.alloc(extendLen); return srcPointer;
        self.alloc(extendLen);
        return srcPointer;
      }
      // Java: int pointer = this.alloc(srcLen + extendLen);
      let pointer = self.alloc(srcLen + extendLen);
      // Java: this.copy(srcPointer, pointer, srcLen); return pointer;
      self.copy__int_int_int(srcPointer, pointer, srcLen);
      pointer
    }

    // java public boolean hasFree(int pointer)
    pub fn hasFree__int(&self, pointer: i32) -> bool {
      // Java returns true past Data.size, and false when the free-range list is empty.
      if pointer >= self.mData.size {
        return true;
      }
      false
      /* DEFERRED_NON_FUNCTION
      // java if (pointer >= this.mData.size)
      if pointer >= self.mData.size {
        // java return true;
        return true;
      }
      // java else
      else {
        // java WIntArrayList freeList = this.mData.freeList;
        let mut freeList: WIntArrayList = { let __x: WIntArrayList = todo!("no align strategy matched"); __x };
        // java if (freeList.isEmpty())
        if freeList.is_empty() {
          // java return false;
          return false;
        }
        // java else
        else {
          // java int pos = freeList.binarySearch(pointer);
          let mut pos: i32 = freeList.binarySearch(pointer);
          // java if (pos >= 0)
          if pos >= 0 {
            // java if (pos % 2 == 0)
            if pos % 2 == 0 {
              // java return true;
              return true;
            }
          }
          // java else
          else {
            // java pos = -pos - 1;
            pos = -pos - 1;
            // java if (pos % 2 == 1)
            if pos % 2 == 1 {
              // java return true;
              return true;
            }
          }
          // java return false;
          return false;
        }
      }
      **/
    }

    // java public boolean hasFree(int pointer, int len)
    pub fn hasFree__int_int(&self, pointer: i32, len: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int end;
      let mut end: i32;
      // java if (pointer < this.mData.size && (end = pointer + len) <= this.mData.size)
      if pointer < self.mData.size && { end = pointer + len; end } <= self.mData.size {
        // java WIntArrayList freeList = this.mData.freeList;
        let mut freeList: WIntArrayList = { let __x: WIntArrayList = todo!("no align strategy matched"); __x };
        // java if (freeList.isEmpty())
        if freeList.is_empty() {
          // java return false;
          return false;
        }
        // java else
        else {
          // java int pos = freeList.binarySearch(pointer);
          let mut pos: i32 = freeList.binarySearch(pointer);
          // java if (pos >= 0)
          if pos >= 0 {
            // java if (pos % 2 == 0)
            if pos % 2 == 0 {
              // java return true;
              return true;
            }
            // java if (pos < freeList.size() - 1 && end > freeList.getQuick(pos + 1))
            if pos < (freeList.len() as i32) - 1 && end > freeList.getQuick(pos + 1) {
              // java return true;
              return true;
            }
          }
          // java else
          else {
            // java pos = -pos - 1;
            pos = -pos - 1;
            // java if (pos % 2 == 1)
            if pos % 2 == 1 {
              // java return true;
              return true;
            }
            // java if (pos < freeList.size() && end > freeList.getQuick(pos))
            if pos < (freeList.len() as i32) && end > freeList.getQuick(pos) {
              // java return true;
              return true;
            }
          }
          // java return false;
          return false;
        }
      }
      // java else
      else {
        // java return true;
        return true;
      }
      **/
    }

    // java public void free(int pointer, int len)
    pub fn free(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int end = pointer + len;
      let mut end: i32 = pointer + len;
      // java WIntArrayList freeList = this.mData.freeList;
      let mut freeList: WIntArrayList = { let __x: WIntArrayList = todo!("no align strategy matched"); __x };
      // java addRange(pointer, end, freeList);
      Self::addRange(pointer, end, freeList);
      // java this.doFreeRanges(freeList);
      self.doFreeRanges(freeList);
      **/
    }

    // java public void freeRanges(WIntArrayList aFreeList)
    pub fn freeRanges(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java WIntArrayList freeList = this.mData.freeList;
      let mut freeList: WIntArrayList = { let __x: WIntArrayList = todo!("no align strategy matched"); __x };
      // java freeList.addAll(aFreeList);
      freeList.addAll(aFreeList);
      // java this.doFreeRanges(freeList);
      self.doFreeRanges(freeList);
      **/
    }

    // java private void doFreeRanges(WIntArrayList freeList)
    pub fn doFreeRanges(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int newDataSize = this.mData.size;
      let mut newDataSize: i32 = self.mData.size;
      // java int rangeSize = freeList.size();
      let mut rangeSize: i32 = freeList.size();
      // java if (rangeSize > 0)
      if rangeSize > 0 {
        // java if (freeList.getQuick(rangeSize - 1) == newDataSize)
        if freeList.getQuick(rangeSize - 1) == newDataSize {
          // java newDataSize = freeList.getQuick(rangeSize - 2);
          newDataSize = freeList.getQuick(rangeSize - 2);
          // java freeList.remove(rangeSize - 2, 2);
          freeList.remove__int_int(rangeSize - 2, 2);
        }
        // java if (this.mData.size != newDataSize)
        if self.mData.size != newDataSize {
          // java Data var10000 = this.mData;
          let mut var10000: Box<Data> = { let __x: Box<Box<Data>> = todo!("no align strategy matched"); __x };
          // java TypoMemBlockManager memBlockManager = TypoSnapshot.Data.sMemBlockManager;
          let mut memBlockManager: TypoMemBlockManager = { let __x: TypoMemBlockManager = todo!("no align strategy matched"); __x };
          // java int blockIndex = (newDataSize - 1 & ~memBlockManager.mask) >> memBlockManager.bits;
          let mut blockIndex: i32 = newDataSize - 1 & !memBlockManager.mask >> memBlockManager.bits;
          // java ArrayList<TypoMemBlock> blocks = this.mData.blocks;
          let mut blocks: Vec<TypoMemBlock> = { let __x: Vec<TypoMemBlock> = todo!("no align strategy matched"); __x };
          // java if (blockIndex < blocks.size() - 1)
          if blockIndex < (blocks.len() as i32) - 1 {
              // java (memBlockManager)
              memBlockManager;
                let mut i: i32 = (blocks.len() as i32) - 1;
                // java for(int i = blocks.size() - 1; i > blockIndex --i)
                while i > blockIndex {
                  // java TypoMemBlock lastBlock = (TypoMemBlock)blocks.remove(i);
                  let mut lastBlock: TypoMemBlock = (blocks.remove(i)).downcast_ref::<TypoMemBlock>().unwrap();
                    // java (lastBlock)
                    lastBlock;
                      // java if (--lastBlock.refCount <= 0)
                      if { let _tmp = lastBlock.refCount; lastBlock.refCount -= 1; _tmp } <= 0 {
                        // java memBlockManager.recycleBlock(lastBlock);
                        memBlockManager.recycleBlock(lastBlock);
                      }
                }
          }
          // java int blockOffset = newDataSize & memBlockManager.mask;
          let mut blockOffset: i32 = newDataSize & memBlockManager.mask;
          // java if (blockOffset > 0)
          if blockOffset > 0 {
            // java int[] date = this.getWriteBlockData(newDataSize, (int[])null);
            let mut date: Vec<i32> = self.getWriteBlockData(newDataSize, None);
            // java Arrays.fill(date, blockOffset, date.length, 0);
            Arrays::fill(date, blockOffset, (date.len() as i32), 0);
          }
          // java this.mData.size = newDataSize;
          self.mData.size = newDataSize;
        }
      }
      **/
    }

    // java public void mergeFreeRange(WIntArrayList freeRange)
    pub fn mergeFreeRange(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java WIntArrayList srcFreeRange = this.mData.freeList;
      let mut srcFreeRange: WIntArrayList = { let __x: WIntArrayList = todo!("no align strategy matched"); __x };
      // java if (freeRange.isEmpty())
      if freeRange.isEmpty() {
        // java freeRange.addAll(srcFreeRange);
        freeRange.addAll__collection(srcFreeRange);
      }
      // java else
      else {
        // java int pos = 0;
        let mut pos: i32 = 0;
        // java int max = srcFreeRange.size();
        let mut max: i32 = (srcFreeRange.len() as i32);
        // java while (pos < max)
        while pos < max {
          // java int start = srcFreeRange.getQuick(pos++);
          let mut start: i32 = srcFreeRange.getQuick({ let _tmp = pos; pos += 1; _tmp });
          // java int end = srcFreeRange.getQuick(pos++);
          let mut end: i32 = srcFreeRange.getQuick({ let _tmp = pos; pos += 1; _tmp });
          // java addRange(start, end, freeRange);
          Self::addRange(start, end, freeRange);
        }
      }
      // java srcFreeRange.resetQuick();
      srcFreeRange.resetQuick();
      **/
    }

    // java public static void addRange(int memStart, int memEnd, WIntArrayList freeRanges)
    pub fn addRange(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (memEnd > memStart)
      if memEnd > memStart {
        // java if (freeRanges.isEmpty())
        if freeRanges.isEmpty() {
          // java freeRanges.add(memStart);
          freeRanges.add__int(memStart);
          // java freeRanges.add(memEnd);
          freeRanges.add__int(memEnd);
        }
        // java else
        else {
          // java int rangSize = freeRanges.size();
          let mut rangSize: i32 = freeRanges.size();
          // java int pos = freeRanges.binarySearch(memStart);
          let mut pos: i32 = freeRanges.binarySearch__int(memStart);
          // java if (pos >= 0)
          if pos >= 0 {
            // java if (pos % 2 != 1)
            if pos % 2 != 1 {
              // java if (pos < rangSize - 1)
              if pos < rangSize - 1 {
                // java int rangeEnd = freeRanges.get(pos + 1);
                let mut rangeEnd: i32 = freeRanges.get(pos + 1);
                // java if (memEnd > rangeEnd && rangeEnd > memStart)
                if memEnd > rangeEnd && rangeEnd > memStart {
                  // java addRange(rangeEnd, memEnd, freeRanges);
                  Self::addRange(rangeEnd, memEnd, freeRanges);
                }
              }
            }
            // java else if (pos < rangSize - 1)
            else if pos < rangSize - 1 {
              // java int nextRangeStart = freeRanges.getQuick(pos + 1);
              let mut nextRangeStart: i32 = freeRanges.getQuick(pos + 1);
              // java if (memEnd == nextRangeStart)
              if memEnd == nextRangeStart {
                // java freeRanges.remove(pos, 2);
                freeRanges.remove__int_int(pos, 2);
              }
              // java else if (memEnd < nextRangeStart)
              else if memEnd < nextRangeStart {
                // java freeRanges.setQuick(pos, memEnd);
                freeRanges.setQuick(pos, memEnd);
              }
              // java else
              else {
                // java freeRanges.remove(pos, 2);
                freeRanges.remove__int_int(pos, 2);
                // java if (nextRangeStart > memStart)
                if nextRangeStart > memStart {
                  // java addRange(nextRangeStart, memEnd, freeRanges);
                  Self::addRange(nextRangeStart, memEnd, freeRanges);
                }
              }
            }
            // java else
            else {
              // java freeRanges.setQuick(pos, memEnd);
              freeRanges.setQuick(pos, memEnd);
            }
          }
          // java else
          else {
            // java pos = -pos - 1;
            pos = -pos - 1;
            // java if (pos % 2 != 0)
            if pos % 2 != 0 {
              // java int rangEnd = freeRanges.get(pos);
              let mut rangEnd: i32 = freeRanges.get(pos);
              // java if (memEnd > rangEnd && rangEnd > memStart)
              if memEnd > rangEnd && rangEnd > memStart {
                // java addRange(rangEnd, memEnd, freeRanges);
                Self::addRange(rangEnd, memEnd, freeRanges);
              }
            }
            // java else if (pos < freeRanges.size())
            else if pos < freeRanges.size() {
              // java int nextRangeStart = freeRanges.getQuick(pos);
              let mut nextRangeStart: i32 = freeRanges.getQuick(pos);
              // java if (memEnd == nextRangeStart)
              if memEnd == nextRangeStart {
                // java freeRanges.setQuick(pos, memStart);
                freeRanges.setQuick(pos, memStart);
              }
              // java else if (memEnd < nextRangeStart)
              else if memEnd < nextRangeStart {
                // java freeRanges.insert(pos, memEnd);
                freeRanges.insert__int_int(pos, memEnd);
                // java freeRanges.insert(pos, memStart);
                freeRanges.insert__int_int(pos, memStart);
              }
              // java else
              else {
                // java freeRanges.setQuick(pos, memStart);
                freeRanges.setQuick(pos, memStart);
                // java if (nextRangeStart > memStart)
                if nextRangeStart > memStart {
                  // java addRange(nextRangeStart, memEnd, freeRanges);
                  Self::addRange(nextRangeStart, memEnd, freeRanges);
                }
              }
            }
            // java else
            else {
              // java freeRanges.insert(pos, memEnd);
              freeRanges.insert__int_int(pos, memEnd);
              // java freeRanges.insert(pos, memStart);
              freeRanges.insert__int_int(pos, memStart);
            }
          }
        }
      }
      **/
    }

    // java public int getAllocSize()
    pub fn getAllocSize(&self) -> i64 {
      // java return this.mData.size;
      // rust: flat heap 同步（alloc/set 已维护）
      (self.heap.len() as i64)
    }

    // java public long getVersion()
    pub fn getVersion(&self) -> i64 {
      // java return this.mData.version;
      return self.mData.version;
    }

    pub fn new() -> Self {
      // rust: ★wr-29 05 —— Java Data.size=1（0 保留 null）
      let mut s = Self::default();
      s.heap.push(0);
      s
    }

    pub fn add_page(&mut self, page: crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage) {
      self.pages.push(page);
    }

    pub fn page_count(&self) -> i32 {
      self.pages.len() as i32
    }

    pub fn get_page(&self, i: i32) -> Option<&crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage> {
      if i < 0 {
        return None;
      }
      self.pages.get(i as usize)
    }

    /// Owned-layout bridge used by Stage2 export and hit testing.
    pub fn layout_pages(&self) -> &[crate::cn::wps::moffice::writer::cache::TypoPage::TypoPage] {
        &self.pages
    }

    pub fn bump_version(&mut self) {
      self.mData.version += 1;
    }

    pub fn set_version_for_layout(&mut self, v: i64) {
      self.mData.version = v;
    }

    // java public long getModifyPageRange()
    pub fn getModifyPageRange(&self) -> i64 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mData.modifyPageRange;
      return self.mData.modifyPageRange;
      **/
    }

    // java public void setModify(int pageIndex)
    pub fn setModify(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mData.modifyPageRange = RANGEFactory.include(this.mData.modifyPageRange, pageIndex);
      self.mData.modifyPageRange = RANGEFactory::include__long_int(self.mData.modifyPageRange, pageIndex);
      **/
    }

    // java public void setModifyFrom(int pageIndex)
    pub fn setModifyFrom(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int pageEndIndex = Math.max(this.getLayoutPageSize(), pageIndex + 1);
      let mut pageEndIndex: i32 = Math::max(self.getLayoutPageSize(), pageIndex + 1);
      // java this.setModifyRange(pageIndex, pageEndIndex);
      self.setModifyRange(pageIndex, pageEndIndex);
      **/
    }

    // java public void setModifyRange(int startPageIndex, int endPageIndex)
    pub fn setModifyRange(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mData.modifyPageRange = RANGEFactory.include(this.mData.modifyPageRange, startPageIndex, endPageIndex);
      self.mData.modifyPageRange = RANGEFactory::include__long_int_int(self.mData.modifyPageRange, startPageIndex, endPageIndex);
      **/
    }

    // java private void loadBlock(int pointer)
    pub fn loadBlock(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Data var10000 = this.mData;
      let mut var10000: Box<Data> = { let __x: Box<Box<Data>> = todo!("no align strategy matched"); __x };
      // java TypoMemBlockManager memBlockManager = TypoSnapshot.Data.sMemBlockManager;
      let mut memBlockManager: TypoMemBlockManager = { let __x: TypoMemBlockManager = todo!("no align strategy matched"); __x };
      // java int blockIndex = (pointer & ~memBlockManager.mask) >> memBlockManager.bits;
      let mut blockIndex: i32 = pointer & !memBlockManager.mask >> memBlockManager.bits;
      // java Data data = this.mData;
      let mut data: Box<Data> = { let __x: Box<Box<Data>> = todo!("no align strategy matched"); __x };
      // java ArrayList<TypoMemBlock> blocks = data.blocks;
      let mut blocks: Vec<TypoMemBlock> = { let __x: Vec<TypoMemBlock> = todo!("no align strategy matched"); __x };
      // java this.mCurBlock = ((TypoMemBlock)blocks.get(blockIndex)).valid();
      self.mCurBlock = (blocks.get(blockIndex)).downcast_ref::<TypoMemBlock>().unwrap().valid();
      // java this.mCurBlockStart = blockIndex << memBlockManager.bits;
      self.mCurBlockStart = blockIndex << memBlockManager.bits;
      // java this.mCurBlockEnd = this.mCurBlockStart + memBlockManager.blockSize;
      self.mCurBlockEnd = self.mCurBlockStart + memBlockManager.blockSize;
      // java this.mCurBlockIndex = blockIndex;
      self.mCurBlockIndex = blockIndex;
      // java this.mCurBlockWriteable = this.mCurBlock.refCount == 1;
      self.mCurBlockWriteable = self.mCurBlock.refCount == 1;
      **/
    }

    // java private void prepareWriteableBlock(int pointer)
    pub fn prepareWriteableBlock(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mIsRefClone)
      if self.mIsRefClone {
        // java throw new RuntimeException("clone TypoSnapshot no allow to write");
        panic!("{}", RuntimeException::new("clone TypoSnapshot no allow to write"));
      }
      // java else
      else {
        // java Data var10000 = this.mData;
        let mut var10000: Box<Data> = { let __x: Box<Box<Data>> = todo!("no align strategy matched"); __x };
        // java TypoMemBlockManager memBlockManager = TypoSnapshot.Data.sMemBlockManager;
        let mut memBlockManager: TypoMemBlockManager = { let __x: TypoMemBlockManager = todo!("no align strategy matched"); __x };
        // java int blockIndex = (pointer & ~memBlockManager.mask) >> memBlockManager.bits;
        let mut blockIndex: i32 = pointer & !memBlockManager.mask >> memBlockManager.bits;
        // java Data data = this.mData;
        let mut data: Box<Data> = { let __x: Box<Box<Data>> = todo!("no align strategy matched"); __x };
        // java ArrayList<TypoMemBlock> blocks = data.blocks;
        let mut blocks: Vec<TypoMemBlock> = { let __x: Vec<TypoMemBlock> = todo!("no align strategy matched"); __x };
        // java this.mCurBlock = ((TypoMemBlock)blocks.get(blockIndex)).valid();
        self.mCurBlock = (blocks.get(blockIndex)).downcast_ref::<TypoMemBlock>().unwrap().valid();
        // java this.mCurBlockStart = blockIndex << memBlockManager.bits;
        self.mCurBlockStart = blockIndex << memBlockManager.bits;
        // java this.mCurBlockEnd = this.mCurBlockStart + memBlockManager.blockSize;
        self.mCurBlockEnd = self.mCurBlockStart + memBlockManager.blockSize;
        // java this.mCurBlockIndex = blockIndex;
        self.mCurBlockIndex = blockIndex;
        // java if (this.mCurBlock.refCount != 1)
        if self.mCurBlock.refCount != 1 {
          // java this.mCurBlock = memBlockManager.allocNormalBlock(this.mCurBlock);
          self.mCurBlock = memBlockManager.allocNormalBlock(self.mCurBlock);
          // java blocks.set(blockIndex, this.mCurBlock);
          blocks.set(blockIndex, self.mCurBlock);
        }
        // java this.mCurBlockWriteable = true;
        self.mCurBlockWriteable = true;
      }
      **/
    }

    // java public int[] get(int pointer, int[] offset)
    pub fn get__int_intarr(&mut self, pointer: i32, offset: Option<&mut Vec<i32>>) -> Vec<i32> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (pointer < this.mCurBlockStart || pointer >= this.mCurBlockEnd)
      if pointer < self.mCurBlockStart || pointer >= self.mCurBlockEnd {
        // java this.loadBlock(pointer);
        self.loadBlock(pointer);
      }
      // java if (null != offset)
      if let Some(offset) = offset {
        // java offset[0] = pointer - this.mCurBlockStart;
        offset[0usize] = pointer - self.mCurBlockStart;
      }
      // java return this.mCurBlock.data;
      return self.mCurBlock.data;
      **/
    }

    // java public int[] getWriteBlockData(int pointer, int[] offset)
    pub fn getWriteBlockData(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (pointer < this.mCurBlockStart || pointer >= this.mCurBlockEnd || !this.mCurBlockWriteable)
      if pointer < self.mCurBlockStart || pointer >= self.mCurBlockEnd || !self.mCurBlockWriteable {
        // java this.prepareWriteableBlock(pointer);
        self.prepareWriteableBlock(pointer);
      }
      // java if (null != offset)
      if true {
        // java offset[0] = pointer - this.mCurBlockStart;
        offset[0usize] = pointer - self.mCurBlockStart;
      }
      // java return this.mCurBlock.data;
      return self.mCurBlock.data;
      **/
    }

    // java public int get(int pointer)
    // rust: ★wr-29 05 —— 真实 flat 读（OOB → 0，Java 块外读等价于未分配 0）
    pub fn get__int(&self, pointer: i32) -> i32 {
      if pointer < 0 || pointer as usize >= self.heap.len() { 0 } else { self.heap[pointer as usize] }
      /* java if (pointer < this.mCurBlockStart || pointer >= this.mCurBlockEnd)
      if pointer < self.mCurBlockStart || pointer >= self.mCurBlockEnd {
        // java this.loadBlock(pointer);
        self.loadBlock(pointer);
      }
      // java return this.mCurBlock.data[pointer - this.mCurBlockStart];
      return self.mCurBlock.data[(pointer - self.mCurBlockStart) as usize];
      **/
    }

    // java public void set(int pointer, int value)
    // rust: ★wr-29 05 —— 真实 flat 写（越界则扩堆——Java prepareWriteableBlock 新建块等价）
    pub fn set__int_int(&mut self, pointer: i32, value: i32) {
      if pointer >= 0 {
        if pointer as usize >= self.heap.len() {
          let grow = (pointer as usize + 1) - self.heap.len();
          for _ in 0..grow { self.heap.push(0); }
          self.mData.size = self.heap.len() as i32;
        }
        self.heap[pointer as usize] = value;
      }
    }

    // java public void extend(int pointer, int value)
    pub fn extend(&mut self, pointer: i32, value: i32) {
      if pointer >= 0 {
        let current = self.get__int(pointer);
        self.set__int_int(pointer, current.wrapping_add(value));
      }
      /* java if (pointer < this.mCurBlockStart || pointer >= this.mCurBlockEnd || !this.mCurBlockWriteable)
      if pointer < self.mCurBlockStart || pointer >= self.mCurBlockEnd || !self.mCurBlockWriteable {
        // java this.prepareWriteableBlock(pointer);
        self.prepareWriteableBlock(pointer);
      }
      // java this.mCurBlock.data[pointer - this.mCurBlockStart] = value;
      self.mCurBlock.data[(pointer - self.mCurBlockStart) as usize] = value;
      **/
    }

    // java public void mutableSet(int pointer, int value)
    pub fn mutableSet(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (pointer < this.mCurBlockStart || pointer >= this.mCurBlockEnd)
      if pointer < self.mCurBlockStart || pointer >= self.mCurBlockEnd {
        // java this.loadBlock(pointer);
        self.loadBlock(pointer);
      }
      // java this.mCurBlock.data[pointer - this.mCurBlockStart] = value;
      self.mCurBlock.data[(pointer - self.mCurBlockStart) as usize] = value;
      **/
    }

    // java public void fill(int pointer, int len, int value)
    pub fn fill__int_int_int(&mut self, pointer: i32, len: i32, value: i32) {
      let mut cur = pointer;
      let max = pointer.wrapping_add(len);
      while cur < max {
        self.set__int_int(cur, value);
        cur += 1;
      }
    }

    pub fn fill(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int max = pointer + len;
      let mut max: i32 = pointer + len;
      // java int fillThisTime;
      let mut fillThisTime: i32;
      let mut curPointer: i32 = pointer;
      // java for(int curPointer = pointer; curPointer < max curPointer += fillThisTime)
      while curPointer < max {
        // java if (curPointer < this.mCurBlockStart || curPointer >= this.mCurBlockEnd || !this.mCurBlockWriteable)
        if curPointer < self.mCurBlockStart || curPointer >= self.mCurBlockEnd || !self.mCurBlockWriteable {
          // java this.prepareWriteableBlock(curPointer);
          self.prepareWriteableBlock(curPointer);
        }
        // java fillThisTime = Math.min(max - curPointer, this.mCurBlockEnd - curPointer);
        fillThisTime = Math::min(max - curPointer, self.mCurBlockEnd - curPointer);
        // java int[] data = this.mCurBlock.data;
        let mut data: Vec<i32> = self.mCurBlock.data;
        // java int i = curPointer - this.mCurBlockStart;
        let mut i: i32 = curPointer - self.mCurBlockStart;
        let mut maxOffset: i32 = i + fillThisTime;
        // java for(int maxOffset = i + fillThisTime; i < maxOffset; ++i)
        while i < maxOffset {
          // java data[i] = value;
          data[(i) as usize] = value;
        }
      }
      **/
    }

    // java public void set(int pointer, int value0, int value1)
    // java public void set__int_int_int(int pointer, int value0, int value1)
    // rust: ★wr-29 05 —— 真实 flat 写（逐字 set，Java 块边界语义等价）
    pub fn set__int_int_int(&mut self, pointer: i32, value0: i32, value1: i32) {
      self.set__int_int(pointer, value0);
      self.set__int_int(pointer + 1, value1);
    }


    // java public void set(int pointer, int value0, int value1, int value2)
    // java public void set__int_int_int_int(int pointer, int value0, int value1, int value2)
    // rust: ★wr-29 05 —— 真实 flat 写（逐字 set，Java 块边界语义等价）
    pub fn set__int_int_int_int(&mut self, pointer: i32, value0: i32, value1: i32, value2: i32) {
      self.set__int_int(pointer, value0);
      self.set__int_int(pointer + 1, value1);
      self.set__int_int(pointer + 2, value2);
    }


    // java public void set(int pointer, int value0, int value1, int value2, int value3)
    // java public void set__int_int_int_int_int(int pointer, int value0, int value1, int value2, int value3)
    // rust: ★wr-29 05 —— 真实 flat 写（逐字 set，Java 块边界语义等价）
    pub fn set__int_int_int_int_int(&mut self, pointer: i32, value0: i32, value1: i32, value2: i32, value3: i32) {
      self.set__int_int(pointer, value0);
      self.set__int_int(pointer + 1, value1);
      self.set__int_int(pointer + 2, value2);
      self.set__int_int(pointer + 3, value3);
    }


    // java public void setBoolBitSet(int pointer, BoolBitSet bitset, boolean value)
    pub fn setBoolBitSet(&mut self, pointer: i32, bitset: &crate::cn::wps::moffice::util::BoolBitSet::BoolBitSet, value: bool) {
      // java data[offset] = bitset.set(data[offset], value);
      let flags = self.get__int(pointer);
      self.set__int_int(pointer, bitset.set(flags, value));
      /* DEFERRED_NON_FUNCTION
      // java if (pointer < this.mCurBlockStart || pointer >= this.mCurBlockEnd || !this.mCurBlockWriteable)
      if pointer < self.mCurBlockStart || pointer >= self.mCurBlockEnd || !self.mCurBlockWriteable {
        // java this.prepareWriteableBlock(pointer);
        self.prepareWriteableBlock(pointer);
      }
      // java int offset = pointer - this.mCurBlockStart;
      let mut offset: i32 = pointer - self.mCurBlockStart;
      // java int[] data = this.mCurBlock.data;
      let mut data: Vec<i32> = self.mCurBlock.data;
      // java data[offset] = bitset.set(data[offset], value);
      data[(offset) as usize] = bitset.set(data[(offset) as usize], value);
      **/
    }

    // java public void setIntBitSet(int pointer, IntBitSet bitSet, int value)
    pub fn setIntBitSet(&mut self, pointer: i32, bitset: &crate::cn::wps::moffice::util::IntBitSet::IntBitSet, value: i32) {
      // java data[offset] = bitSet.set(data[offset], value);
      let flags = self.get__int(pointer);
      self.set__int_int(pointer, bitset.set(flags, value));
      /* DEFERRED_NON_FUNCTION
      // java if (pointer < this.mCurBlockStart || pointer >= this.mCurBlockEnd || !this.mCurBlockWriteable)
      if pointer < self.mCurBlockStart || pointer >= self.mCurBlockEnd || !self.mCurBlockWriteable {
        // java this.prepareWriteableBlock(pointer);
        self.prepareWriteableBlock(pointer);
      }
      // java int offset = pointer - this.mCurBlockStart;
      let mut offset: i32 = pointer - self.mCurBlockStart;
      // java int[] data = this.mCurBlock.data;
      let mut data: Vec<i32> = self.mCurBlock.data;
      // java data[offset] = bitSet.set(data[offset], value);
      data[(offset) as usize] = bitSet.set(data[(offset) as usize], value);
      **/
    }

    // java public boolean getBoolBitSet(int pointer, BoolBitSet bitset)
    pub fn getBoolBitSet(&self, pointer: i32, bitset: &crate::cn::wps::moffice::util::BoolBitSet::BoolBitSet) -> bool {
      // java return bitset.get(data[offset]);
      bitset.get(self.get__int(pointer))
      /* DEFERRED_NON_FUNCTION
      // java if (pointer < this.mCurBlockStart || pointer >= this.mCurBlockEnd)
      if pointer < self.mCurBlockStart || pointer >= self.mCurBlockEnd {
        // java this.loadBlock(pointer);
        self.loadBlock(pointer);
      }
      // java int offset = pointer - this.mCurBlockStart;
      let mut offset: i32 = pointer - self.mCurBlockStart;
      // java int[] data = this.mCurBlock.data;
      let mut data: Vec<i32> = self.mCurBlock.data;
      // java return bitset.get(data[offset]);
      return bitset.get(data[(offset) as usize]);
      **/
    }

    // java public int getIntBitSet(int pointer, IntBitSet bitset)
    pub fn getIntBitSet(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (pointer < this.mCurBlockStart || pointer >= this.mCurBlockEnd)
      if pointer < self.mCurBlockStart || pointer >= self.mCurBlockEnd {
        // java this.loadBlock(pointer);
        self.loadBlock(pointer);
      }
      // java int offset = pointer - this.mCurBlockStart;
      let mut offset: i32 = pointer - self.mCurBlockStart;
      // java int[] data = this.mCurBlock.data;
      let mut data: Vec<i32> = self.mCurBlock.data;
      // java return bitset.get(data[offset]);
      return bitset.get(data[(offset) as usize]);
      **/
    }

    // java public void set(int pointer, int[] buff, int index, int len)
    pub fn set__int_intarr_int_int(&mut self, pointer: i32, buff: &Vec<i32>, index: i32, len: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Data var10000 = this.mData;
      let mut var10000: Box<Data> = { let __x: Box<Box<Data>> = todo!("no align strategy matched"); __x };
      // java TypoMemBlockManager memBlockManager = TypoSnapshot.Data.sMemBlockManager;
      let mut memBlockManager: TypoMemBlockManager = { let __x: TypoMemBlockManager = todo!("no align strategy matched"); __x };
      // java int blockSize = memBlockManager.blockSize;
      let mut blockSize: i32 = memBlockManager.blockSize;
      // java int maxSet = Math.min(this.mData.size - pointer, Math.min(len, buff.length - index));
      let mut maxSet: i32 = Math::min(self.mData.size - pointer, Math::min(len, (buff.len() as i32) - index));
      // java int max = pointer + maxSet;
      let mut max: i32 = pointer + maxSet;
      // java int curPointer = pointer;
      let mut curPointer: i32 = pointer;
      // java int maxIndex = index + maxSet;
      let mut maxIndex: i32 = index + maxSet;
      // java int setThisTime;
      let mut setThisTime: i32;
      let mut curIndex: i32 = index;
      // java for(int curIndex = index; curPointer < max curIndex += setThisTime)
      while curPointer < max {
        // java if (curPointer < this.mCurBlockStart || curPointer >= this.mCurBlockEnd || !this.mCurBlockWriteable)
        if curPointer < self.mCurBlockStart || curPointer >= self.mCurBlockEnd || !self.mCurBlockWriteable {
          // java this.prepareWriteableBlock(curPointer);
          self.prepareWriteableBlock(curPointer);
        }
        // java int offset = curPointer - this.mCurBlockStart;
        let mut offset: i32 = curPointer - self.mCurBlockStart;
        // java setThisTime = Math.min(blockSize - offset, maxIndex - curIndex);
        setThisTime = Math::min(blockSize - offset, maxIndex - curIndex);
        // java System.arraycopy(buff, curIndex, this.mCurBlock.data, offset, setThisTime);
        System::arraycopy(buff, curIndex, self.mCurBlock.data, offset, setThisTime);
        // java curPointer += setThisTime;
        curPointer = curPointer + setThisTime;
      }
      **/
    }

    // java public void set(int pointer, WIntArrayList buff, int index, int len)
    pub fn set__int_wintarraylist_int_int(&mut self, pointer: i32, buff: &WIntArrayList, index: i32, len: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Data var10000 = this.mData;
      let mut var10000: Box<Data> = { let __x: Box<Box<Data>> = todo!("no align strategy matched"); __x };
      // java TypoMemBlockManager memBlockManager = TypoSnapshot.Data.sMemBlockManager;
      let mut memBlockManager: TypoMemBlockManager = { let __x: TypoMemBlockManager = todo!("no align strategy matched"); __x };
      // java int blockSize = memBlockManager.blockSize;
      let mut blockSize: i32 = memBlockManager.blockSize;
      // java int maxSet = Math.min(this.mData.size - pointer, Math.min(len, buff.size() - index));
      let mut maxSet: i32 = Math::min(self.mData.size - pointer, Math::min(len, buff.size() - index));
      // java int max = pointer + maxSet;
      let mut max: i32 = pointer + maxSet;
      // java int curPointer = pointer;
      let mut curPointer: i32 = pointer;
      // java int maxIndex = index + maxSet;
      let mut maxIndex: i32 = index + maxSet;
      // java int setThisTime;
      let mut setThisTime: i32;
      let mut curIndex: i32 = index;
      // java for(int curIndex = index; curPointer < max curIndex += setThisTime)
      while curPointer < max {
        // java if (curPointer < this.mCurBlockStart || curPointer >= this.mCurBlockEnd || !this.mCurBlockWriteable)
        if curPointer < self.mCurBlockStart || curPointer >= self.mCurBlockEnd || !self.mCurBlockWriteable {
          // java this.prepareWriteableBlock(curPointer);
          self.prepareWriteableBlock(curPointer);
        }
        // java int offset = curPointer - this.mCurBlockStart;
        let mut offset: i32 = curPointer - self.mCurBlockStart;
        // java setThisTime = Math.min(blockSize - offset, maxIndex - curIndex);
        setThisTime = Math::min(blockSize - offset, maxIndex - curIndex);
        // java buff.toArray(this.mCurBlock.data, curIndex, offset, setThisTime);
        buff.toArray__intarr_int_int_int(self.mCurBlock.data, curIndex, offset, setThisTime);
        // java curPointer += setThisTime;
        curPointer = curPointer + setThisTime;
      }
      **/
    }

    // java public void get(int pointer, int[] buff, int index, int len)
    pub fn get__int_intarr_int_int(&mut self, pointer: i32, buff: &Vec<i32>, index: i32, len: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Data var10000 = this.mData;
      let mut var10000: Box<Data> = { let __x: Box<Box<Data>> = todo!("no align strategy matched"); __x };
      // java TypoMemBlockManager memBlockManager = TypoSnapshot.Data.sMemBlockManager;
      let mut memBlockManager: TypoMemBlockManager = { let __x: TypoMemBlockManager = todo!("no align strategy matched"); __x };
      // java int blockSize = memBlockManager.blockSize;
      let mut blockSize: i32 = memBlockManager.blockSize;
      // java int maxSet = Math.min(this.mData.size - pointer, Math.min(len, buff.length - index));
      let mut maxSet: i32 = Math::min(self.mData.size - pointer, Math::min(len, (buff.len() as i32) - index));
      // java int max = pointer + maxSet;
      let mut max: i32 = pointer + maxSet;
      // java int curPointer = pointer;
      let mut curPointer: i32 = pointer;
      // java int maxIndex = index + maxSet;
      let mut maxIndex: i32 = index + maxSet;
      // java int setThisTime;
      let mut setThisTime: i32;
      let mut curIndex: i32 = index;
      // java for(int curIndex = index; curPointer < max curIndex += setThisTime)
      while curPointer < max {
        // java if (curPointer < this.mCurBlockStart || curPointer >= this.mCurBlockEnd)
        if curPointer < self.mCurBlockStart || curPointer >= self.mCurBlockEnd {
          // java this.loadBlock(curPointer);
          self.loadBlock(curPointer);
        }
        // java int offset = curPointer - this.mCurBlockStart;
        let mut offset: i32 = curPointer - self.mCurBlockStart;
        // java setThisTime = Math.min(blockSize - offset, maxIndex - curIndex);
        setThisTime = Math::min(blockSize - offset, maxIndex - curIndex);
        // java System.arraycopy(this.mCurBlock.data, offset, buff, curIndex, setThisTime);
        System::arraycopy(self.mCurBlock.data, offset, buff, curIndex, setThisTime);
        // java curPointer += setThisTime;
        curPointer = curPointer + setThisTime;
      }
      **/
    }

    // java public int get(int pointer, byte[] bytes, int index, int len)
    pub fn get__int_bytearr_int_int(&mut self, pointer: i32, bytes: &Vec<u8>, index: i32, len: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int byteRest = bytes.length - index;
      let mut byteRest: i32 = (bytes.len() as i32) - index;
      // java int allowInt = byteRest / 4;
      let mut allowInt: i32 = byteRest / 4;
      // java Data var10000 = this.mData;
      let mut var10000: Box<Data> = { let __x: Box<Box<Data>> = todo!("no align strategy matched"); __x };
      // java TypoMemBlockManager memBlockManager = TypoSnapshot.Data.sMemBlockManager;
      let mut memBlockManager: TypoMemBlockManager = { let __x: TypoMemBlockManager = todo!("no align strategy matched"); __x };
      // java int blockSize = memBlockManager.blockSize;
      let mut blockSize: i32 = memBlockManager.blockSize;
      // java int maxSet = Math.min(this.mData.size - pointer, Math.min(len, allowInt));
      let mut maxSet: i32 = Math::min(self.mData.size - pointer, Math::min(len, allowInt));
      // java int max = pointer + maxSet;
      let mut max: i32 = pointer + maxSet;
      // java int curPointer = pointer;
      let mut curPointer: i32 = pointer;
      // java int setThisTime;
      let mut setThisTime: i32;
      let mut byteIndex: i32 = index;
      // java for(int byteIndex = index; curPointer < max byteIndex += setThisTime * 4)
      while curPointer < max {
        // java if (curPointer < this.mCurBlockStart || curPointer >= this.mCurBlockEnd)
        if curPointer < self.mCurBlockStart || curPointer >= self.mCurBlockEnd {
          // java this.loadBlock(curPointer);
          self.loadBlock(curPointer);
        }
        // java int offset = curPointer - this.mCurBlockStart;
        let mut offset: i32 = curPointer - self.mCurBlockStart;
        // java setThisTime = Math.min(blockSize - offset, allowInt);
        setThisTime = Math::min(blockSize - offset, allowInt);
        // java transToByteArray(this.mCurBlock.data, offset, setThisTime, bytes, byteIndex);
        Self::transToByteArray(self.mCurBlock.data, offset, setThisTime, bytes, byteIndex);
        // java curPointer += setThisTime;
        curPointer = curPointer + setThisTime;
        // java allowInt -= setThisTime;
        allowInt = allowInt - setThisTime;
      }
      // java return maxSet * 4;
      return maxSet * 4;
      **/
    }

    // java public int compress(int pointer, byte[] bytes, int index, int len, int key)
    pub fn compress(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java byte[] keyMap = this.getKeyMap(key);
      let mut keyMap: Vec<u8> = self.getKeyMap(key);
      // java Data var10000 = this.mData;
      let mut var10000: Box<Data> = { let __x: Box<Box<Data>> = todo!("no align strategy matched"); __x };
      // java TypoMemBlockManager memBlockManager = TypoSnapshot.Data.sMemBlockManager;
      let mut memBlockManager: TypoMemBlockManager = { let __x: TypoMemBlockManager = todo!("no align strategy matched"); __x };
      // java int blockSize = memBlockManager.blockSize;
      let mut blockSize: i32 = memBlockManager.blockSize;
      // java int maxSet = Math.min(this.mData.size - pointer, len);
      let mut maxSet: i32 = Math::min(self.mData.size - pointer, len);
      // java int max = pointer + maxSet;
      let mut max: i32 = pointer + maxSet;
      // java int curPointer = pointer;
      let mut curPointer: i32 = pointer;
      // java int byteIndex;
      let mut byteIndex: i32;
      // java int curBlockOffset;
      let mut curBlockOffset: i32;
      byteIndex = index;
      // java for(byteIndex = index curPointer < max curPointer = curBlockOffset + this.mCurBlockStart)
      while curPointer < max {
        // java if (curPointer < this.mCurBlockStart || curPointer >= this.mCurBlockEnd)
        if curPointer < self.mCurBlockStart || curPointer >= self.mCurBlockEnd {
          // java this.loadBlock(curPointer);
          self.loadBlock(curPointer);
        }
        // java curBlockOffset = curPointer - this.mCurBlockStart;
        curBlockOffset = curPointer - self.mCurBlockStart;
        // java int thisMax = Math.min(blockSize, max - this.mCurBlockStart);
        let mut thisMax: i32 = Math::min(blockSize, max - self.mCurBlockStart);
        // java int[] ints = this.mCurBlock.data;
        let mut ints: Vec<i32> = self.mCurBlock.data;
        // java while (curBlockOffset < thisMax)
        while curBlockOffset < thisMax {
          // java int d = ints[curBlockOffset++];
          let mut d: i32 = ints[({ let _tmp = curBlockOffset; curBlockOffset += 1; _tmp }) as usize];
          // java d = d << 1 ^ d >> 31;
          d = d << 1 ^ d >> 31;
          // java if (bytes.length - byteIndex < 5)
          if (bytes.len() as i32) - byteIndex < 5 {
            // java return 0;
            return { let __x: () = todo!("no align strategy matched"); __x };
          }
          let mut i: i32 = 0;
          // java for(int i = 0; i < 5; ++i)
          while i < 5 {
            // java if ((d & -128) == 0)
            if d & -128 == 0 {
              // java bytes[byteIndex++] = (byte)((d ^ keyMap[i]) & 255);
              bytes[({ let _tmp = byteIndex; byteIndex += 1; _tmp }) as usize] = ((d ^ keyMap[(i) as usize] & 255) as u8);
              // java break;
              break;
            }
            // java bytes[byteIndex++] = (byte)(((d | 128) ^ keyMap[i]) & 255);
            bytes[({ let _tmp = byteIndex; byteIndex += 1; _tmp }) as usize] = ((d | 128 ^ keyMap[(i) as usize] & 255) as u8);
            // java d >>>= 7;
            d = d >>> 7;
          }
        }
      }
      // java return byteIndex - index;
      return byteIndex - index;
      **/
    }

    // java private byte[] getKeyMap(int key)
    pub fn getKeyMap(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java byte[] keyMap = new byte[5];
      let mut keyMap: Vec<u8> = vec![Default::default(); 5usize];
      let mut i: i32 = 0;
      // java for(int i = 0; i < 5; ++i)
      while i < 5 {
        // java keyMap[i] = (byte)(key & 127);
        keyMap[(i) as usize] = ((key & 127) as u8);
        // java key >>>= 7;
        key = key >>> 7;
      }
      // java return keyMap;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java public int decompress(int pointer, byte[] bytes, int index, int len, int key)
    pub fn decompress(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int byteIndexMax = index + len;
      let mut byteIndexMax: i32 = index + len;
      // java byte[] keyMap = this.getKeyMap(key);
      let mut keyMap: Vec<u8> = self.getKeyMap(key);
      // java Data var10000 = this.mData;
      let mut var10000: Box<Data> = { let __x: Box<Box<Data>> = todo!("no align strategy matched"); __x };
      // java TypoMemBlockManager memBlockManager = TypoSnapshot.Data.sMemBlockManager;
      let mut memBlockManager: TypoMemBlockManager = { let __x: TypoMemBlockManager = todo!("no align strategy matched"); __x };
      // java int blockSize = memBlockManager.blockSize;
      let mut blockSize: i32 = memBlockManager.blockSize;
      // java int maxSet = this.mData.size - pointer;
      let mut maxSet: i32 = self.mData.size - pointer;
      // java int max = pointer + maxSet;
      let mut max: i32 = pointer + maxSet;
      // java int curPointer;
      let mut curPointer: i32;
      // java int curBlockOffset;
      let mut curBlockOffset: i32;
      curPointer = pointer;
      // java for(curPointer = pointer curPointer < max && index < byteIndexMax curPointer = this.mCurBlockStart + curBlockOffset)
      while curPointer < max && index < byteIndexMax {
        // java if (curPointer < this.mCurBlockStart || curPointer >= this.mCurBlockEnd || !this.mCurBlockWriteable)
        if curPointer < self.mCurBlockStart || curPointer >= self.mCurBlockEnd || !self.mCurBlockWriteable {
              // java this.prepareWriteableBlock(curPointer);
              self.prepareWriteableBlock(curPointer);
              // java RuntimeException var20
              todo!();
                // java return 0;
                return { let __x: () = todo!("no align strategy matched"); __x };
        }
        // java curBlockOffset = curPointer - this.mCurBlockStart;
        curBlockOffset = curPointer - self.mCurBlockStart;
        // java int thisMax = Math.min(blockSize, max - this.mCurBlockStart);
        let mut thisMax: i32 = Math::min(blockSize, max - self.mCurBlockStart);
        // java int decodeInt;
        let mut decodeInt: i32;
        let mut ints: Vec<i32> = self.mCurBlock.data;
        // java for(int[] ints = this.mCurBlock.data; curBlockOffset < thisMax && index < byteIndexMax ints[curBlockOffset++] = decodeInt)
        while curBlockOffset < thisMax && index < byteIndexMax {
          // java decodeInt = 0;
          decodeInt = 0;
          // java int bitOffset = 0;
          let mut bitOffset: i32 = 0;
          let mut i: i32 = 0;
          // java for(int i = 0; i < 5 && index < byteIndexMax; ++i)
          while i < 5 && index < byteIndexMax {
            // java byte n = (byte)((bytes[index++] ^ keyMap[i]) & 255);
            let mut n: u8 = ((bytes[({ let _tmp = index; index += 1; _tmp }) as usize] ^ keyMap[(i) as usize] & 255) as u8);
            // java if ((n & 128) != 128)
            if n & 128 != 128 {
              // java decodeInt |= n << bitOffset;
              decodeInt = decodeInt | n << bitOffset;
              // java break;
              break;
            }
            // java decodeInt |= (n & 127) << bitOffset;
            decodeInt = decodeInt | n & 127 << bitOffset;
            // java bitOffset += 7;
            bitOffset = bitOffset + 7;
          }
          // java decodeInt = decodeInt >>> 1 ^ -(decodeInt & 1);
          decodeInt = decodeInt >> 1 ^ -decodeInt & 1;
        }
      }
      // java return curPointer - pointer;
      return curPointer - pointer;
      **/
    }

    // java final byte enOrDescryp(int data, byte key)
    pub fn enOrDescryp(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return (byte)((data ^ key) & 255);
      return ((data ^ key & 255) as u8);
      **/
    }

    // java final int intToZigZag(int n)
    pub fn intToZigZag(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return n << 1 ^ n >> 31;
      return n << 1 ^ n >> 31;
      **/
    }

    // java final int zigzagToInt(int n)
    pub fn zigzagToInt(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return n >>> 1 ^ -(n & 1);
      return n >> 1 ^ -n & 1;
      **/
    }

    // java public int set(int pointer, byte[] bytes, int index, int len)
    pub fn set__int_bytearr_int_int(&mut self, pointer: i32, bytes: &Vec<u8>, index: i32, len: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int writerIntCount = len / 4;
      let mut writerIntCount: i32 = len / 4;
      // java if (writerIntCount != 0 && pointer + writerIntCount <= this.mData.size && index + len <= bytes.length)
      if writerIntCount != 0 && pointer + writerIntCount <= self.mData.size && index + len <= (bytes.len() as i32) {
        // java Data var10000 = this.mData;
        let mut var10000: Box<Data> = { let __x: Box<Box<Data>> = todo!("no align strategy matched"); __x };
        // java TypoMemBlockManager memBlockManager = TypoSnapshot.Data.sMemBlockManager;
        let mut memBlockManager: TypoMemBlockManager = { let __x: TypoMemBlockManager = todo!("no align strategy matched"); __x };
        // java int blockSize = memBlockManager.blockSize;
        let mut blockSize: i32 = memBlockManager.blockSize;
        // java int maxSet = Math.min(this.mData.size - pointer, writerIntCount);
        let mut maxSet: i32 = Math::min(self.mData.size - pointer, writerIntCount);
        // java int max = pointer + maxSet;
        let mut max: i32 = pointer + maxSet;
        // java int curPointer = pointer;
        let mut curPointer: i32 = pointer;
        // java int setThisTime;
        let mut setThisTime: i32;
        let mut byteIndex: i32 = index;
        // java for(int byteIndex = index; curPointer < max byteIndex += setThisTime * 4)
        while curPointer < max {
          // java if (curPointer < this.mCurBlockStart || curPointer >= this.mCurBlockEnd || !this.mCurBlockWriteable)
          if curPointer < self.mCurBlockStart || curPointer >= self.mCurBlockEnd || !self.mCurBlockWriteable {
            // java this.prepareWriteableBlock(curPointer);
            self.prepareWriteableBlock(curPointer);
          }
          // java int offset = curPointer - this.mCurBlockStart;
          let mut offset: i32 = curPointer - self.mCurBlockStart;
          // java setThisTime = Math.min(blockSize - offset, writerIntCount);
          setThisTime = Math::min(blockSize - offset, writerIntCount);
          // java transToIntsArray(bytes, byteIndex, this.mCurBlock.data, offset, setThisTime);
          Self::transToIntsArray(bytes, byteIndex, self.mCurBlock.data, offset, setThisTime);
          // java curPointer += setThisTime;
          curPointer = curPointer + setThisTime;
          // java writerIntCount -= setThisTime;
          writerIntCount = writerIntCount - setThisTime;
        }
        // java return len / 4;
        return len / 4;
      }
      // java else
      else {
        // java return 0;
        return 0;
      }
      **/
    }

    // java public static void transToByteArray(int[] ints, int intOffset, int intCount, byte[] bytes, int bytePos)
    pub fn transToByteArray(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      let mut max: i32 = intOffset + intCount;
      // java for(int max = intOffset + intCount; intOffset < max ++intOffset)
      while intOffset < max {
        // java int d = ints[intOffset];
        let mut d: i32 = ints[(intOffset) as usize];
        // java bytes[bytePos++] = (byte)(d & 255);
        bytes[({ let _tmp = bytePos; bytePos += 1; _tmp }) as usize] = ((d & 255) as u8);
        // java bytes[bytePos++] = (byte)((d & '\uff00') >> 8);
        bytes[({ let _tmp = bytePos; bytePos += 1; _tmp }) as usize] = ((d & '\u{ff00}' >> 8) as u8);
        // java bytes[bytePos++] = (byte)((d & 16711680) >> 16);
        bytes[({ let _tmp = bytePos; bytePos += 1; _tmp }) as usize] = ((d & 16711680 >> 16) as u8);
        // java bytes[bytePos++] = (byte)((d & -16777216) >> 24);
        bytes[({ let _tmp = bytePos; bytePos += 1; _tmp }) as usize] = ((d & -16777216 >> 24) as u8);
      }
      **/
    }

    // java public static void transToIntsArray(byte[] bytes, int bytePos, int[] ints, int intOffset, int intCount)
    pub fn transToIntsArray(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      let mut max: i32 = intOffset + intCount;
      // java for(int max = intOffset + intCount; intOffset < max ++intOffset)
      while intOffset < max {
        // java ints[intOffset] = bytes[bytePos++] & 255 | (bytes[bytePos++] & 255) << 8 | (bytes[bytePos++] & 255) << 16 | (bytes[bytePos++] & 255) << 24;
        ints[(intOffset) as usize] = bytes[({ let _tmp = bytePos; bytePos += 1; _tmp }) as usize] & 255 | bytes[({ let _tmp = bytePos; bytePos += 1; _tmp }) as usize] & 255 << 8 | bytes[({ let _tmp = bytePos; bytePos += 1; _tmp }) as usize] & 255 << 16 | bytes[({ let _tmp = bytePos; bytePos += 1; _tmp }) as usize] & 255 << 24;
      }
      **/
    }

    // java public void get(int pointer, IIntList buff, int len)
    pub fn get__int_iintlist_int(&mut self, pointer: i32, buff: &mut dyn IIntList, len: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Data var10000 = this.mData;
      let mut var10000: Box<Data> = { let __x: Box<Box<Data>> = todo!("no align strategy matched"); __x };
      // java TypoMemBlockManager memBlockManager = TypoSnapshot.Data.sMemBlockManager;
      let mut memBlockManager: TypoMemBlockManager = { let __x: TypoMemBlockManager = todo!("no align strategy matched"); __x };
      // java int blockSize = memBlockManager.blockSize;
      let mut blockSize: i32 = memBlockManager.blockSize;
      // java int maxSet = Math.min(this.mData.size - pointer, len);
      let mut maxSet: i32 = Math::min(self.mData.size - pointer, len);
      // java int max = pointer + maxSet;
      let mut max: i32 = pointer + maxSet;
      // java int setThisTime;
      let mut setThisTime: i32;
      let mut curPointer: i32 = pointer;
      // java for(int curPointer = pointer; curPointer < max curPointer += setThisTime)
      while curPointer < max {
        // java if (curPointer < this.mCurBlockStart || curPointer >= this.mCurBlockEnd)
        if curPointer < self.mCurBlockStart || curPointer >= self.mCurBlockEnd {
          // java this.loadBlock(curPointer);
          self.loadBlock(curPointer);
        }
        // java int offset = curPointer - this.mCurBlockStart;
        let mut offset: i32 = curPointer - self.mCurBlockStart;
        // java setThisTime = Math.min(blockSize - offset, max - curPointer);
        setThisTime = Math::min(blockSize - offset, max - curPointer);
        // java buff.add(this.mCurBlock.data, offset, setThisTime);
        buff.add__intarr_int_int(self.mCurBlock.data, offset, setThisTime);
      }
      **/
    }

    // java public void extend(int pointer, int value)
    pub fn extend__int_int(&mut self, pointer: i32, value: i32) {
      // java var10000[var10001] += value;
      let cur = self.get__int(pointer);
      self.set__int_int(pointer, cur.wrapping_add(value));
      /* DEFERRED_NON_FUNCTION
      // java if (pointer < this.mCurBlockStart || pointer >= this.mCurBlockEnd || !this.mCurBlockWriteable)
      if pointer < self.mCurBlockStart || pointer >= self.mCurBlockEnd || !self.mCurBlockWriteable {
        // java this.prepareWriteableBlock(pointer);
        self.prepareWriteableBlock(pointer);
      }
      // java int[] var10000 = this.mCurBlock.data;
      let mut var10000: Vec<i32> = self.mCurBlock.data;
      // java int var10001 = pointer - this.mCurBlockStart;
      let mut var10001: i32 = pointer - self.mCurBlockStart;
      // java var10000[var10001] += value;
      var10000[(var10001) as usize] = var10000[(var10001) as usize] + value;
      **/
    }

    // java public void extend(int pointer, int len, int value)
    pub fn extend__int_int_int(&mut self, pointer: i32, len: i32, value: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int max = Math.min(pointer + len, this.mData.size);
      let mut max: i32 = Math::min(pointer + len, self.mData.size);
      // java int processThisTime;
      let mut processThisTime: i32;
      let mut curPointer: i32 = pointer;
      // java for(int curPointer = pointer; curPointer < max curPointer += processThisTime)
      while curPointer < max {
        // java if (curPointer < this.mCurBlockStart || curPointer >= this.mCurBlockEnd || !this.mCurBlockWriteable)
        if curPointer < self.mCurBlockStart || curPointer >= self.mCurBlockEnd || !self.mCurBlockWriteable {
          // java this.prepareWriteableBlock(curPointer);
          self.prepareWriteableBlock(curPointer);
        }
        // java processThisTime = Math.min(max - curPointer, this.mCurBlockEnd - curPointer);
        processThisTime = Math::min(max - curPointer, self.mCurBlockEnd - curPointer);
        // java int[] data = this.mCurBlock.data;
        let mut data: Vec<i32> = self.mCurBlock.data;
        // java int i = curPointer - this.mCurBlockStart;
        let mut i: i32 = curPointer - self.mCurBlockStart;
        // java int var10001;
        let mut var10001: i32;
        let mut maxOffset: i32 = i + processThisTime;
        // java for(int maxOffset = i + processThisTime; i < maxOffset data[var10001] += value)
        while i < maxOffset {
          // java var10001 = i++;
          var10001 = { let _tmp = i; i += 1; _tmp };
        }
      }
      **/
    }

    // java public int extendFloatRound(int pointer, int len, float value)
    pub fn extendFloatRound(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java float leaveRound = 0.0F;
      let mut leaveRound: f32 = 0;
      // java int totolAdd = 0;
      let mut totolAdd: i32 = 0;
      // java int max = Math.min(pointer + len, this.mData.size);
      let mut max: i32 = Math::min(pointer + len, self.mData.size);
      // java int processThisTime;
      let mut processThisTime: i32;
      let mut curPointer: i32 = pointer;
      // java for(int curPointer = pointer; curPointer < max curPointer += processThisTime)
      while curPointer < max {
        // java if (curPointer < this.mCurBlockStart || curPointer >= this.mCurBlockEnd || !this.mCurBlockWriteable)
        if curPointer < self.mCurBlockStart || curPointer >= self.mCurBlockEnd || !self.mCurBlockWriteable {
          // java this.prepareWriteableBlock(curPointer);
          self.prepareWriteableBlock(curPointer);
        }
        // java processThisTime = Math.min(max - curPointer, this.mCurBlockEnd - curPointer);
        processThisTime = Math::min(max - curPointer, self.mCurBlockEnd - curPointer);
        // java int[] data = this.mCurBlock.data;
        let mut data: Vec<i32> = self.mCurBlock.data;
        // java int i = curPointer - this.mCurBlockStart;
        let mut i: i32 = curPointer - self.mCurBlockStart;
        // java float fAdd;
        let mut fAdd: f32;
        // java int iAdd;
        let mut iAdd: i32;
        let mut maxOffset: i32 = i + processThisTime;
        // java for(int maxOffset = i + processThisTime; i < maxOffset leaveRound = fAdd - (float)iAdd)
        while i < maxOffset {
          // java fAdd = value + leaveRound;
          fAdd = value + leaveRound;
          // java iAdd = round(fAdd);
          iAdd = Self::round(fAdd);
          // java totolAdd += iAdd;
          totolAdd = totolAdd + iAdd;
          // java int var10001 = i++;
          let mut var10001: i32 = { let _tmp = i; i += 1; _tmp };
          // java data[var10001] += iAdd;
          data[(var10001) as usize] = data[(var10001) as usize] + iAdd;
        }
      }
      // java return totolAdd;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java static int round(float f)
    pub fn round(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return f >= 0.0F ? (int)(f + 0.5F) : (int)(f - 0.5F);
      return (if f >= 0 { ((f + 0.5) as i32) } else { ((f - 0.5) as i32) });
      **/
    }

    // java public void scale(int pointer, float scale)
    pub fn scale__int_float(&mut self, pointer: i32, scale: f32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (pointer < this.mCurBlockStart || pointer >= this.mCurBlockEnd || !this.mCurBlockWriteable)
      if pointer < self.mCurBlockStart || pointer >= self.mCurBlockEnd || !self.mCurBlockWriteable {
        // java this.prepareWriteableBlock(pointer);
        self.prepareWriteableBlock(pointer);
      }
      // java int[] var10000 = this.mCurBlock.data;
      let mut var10000: Vec<i32> = self.mCurBlock.data;
      // java int var10001 = pointer - this.mCurBlockStart;
      let mut var10001: i32 = pointer - self.mCurBlockStart;
      // java var10000[var10001] = (int)((float)var10000[var10001] * scale);
      var10000[(var10001) as usize] = ((((var10000[(var10001) as usize]) as f32) * scale) as i32);
      **/
    }

    // java public int scale(int pointer, int len, float scale, int space)
    pub fn scale__int_int_float_int(&mut self, pointer: i32, len: i32, scale: f32, space: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int max = Math.min(pointer + len, this.mData.size);
      let mut max: i32 = Math::min(pointer + len, self.mData.size);
      // java int curPointer = pointer;
      let mut curPointer: i32 = pointer;
      // java int addWidth;
      let mut addWidth: i32;
      // java int processThisTime;
      let mut processThisTime: i32;
      addWidth = 0;
      // java for(addWidth = 0 curPointer < max curPointer += processThisTime)
      while curPointer < max {
        // java if (curPointer < this.mCurBlockStart || curPointer >= this.mCurBlockEnd || !this.mCurBlockWriteable)
        if curPointer < self.mCurBlockStart || curPointer >= self.mCurBlockEnd || !self.mCurBlockWriteable {
          // java this.prepareWriteableBlock(curPointer);
          self.prepareWriteableBlock(curPointer);
        }
        // java processThisTime = Math.min(max - curPointer, this.mCurBlockEnd - curPointer);
        processThisTime = Math::min(max - curPointer, self.mCurBlockEnd - curPointer);
        // java int[] data = this.mCurBlock.data;
        let mut data: Vec<i32> = self.mCurBlock.data;
        // java int i = curPointer - this.mCurBlockStart;
        let mut i: i32 = curPointer - self.mCurBlockStart;
        let mut maxOffset: i32 = i + processThisTime;
        // java for(int maxOffset = i + processThisTime; i < maxOffset; ++i)
        while i < maxOffset {
          // java int pureWidth = data[i] - space;
          let mut pureWidth: i32 = data[(i) as usize] - space;
          // java int scalWidth = (int)((float)pureWidth * scale);
          let mut scalWidth: i32 = ((((pureWidth) as f32) * scale) as i32);
          // java addWidth += scalWidth - pureWidth;
          addWidth = addWidth + scalWidth - pureWidth;
          // java data[i] = scalWidth + space;
          data[(i) as usize] = scalWidth + space;
        }
      }
      // java return addWidth;
      return addWidth;
      **/
    }

    // java public int copy(int pointer, int len)
    pub fn copy__int_int(&mut self, pointer: i32, len: i32) -> i32 {
      // Java: int des = this.alloc(len);
      let des = self.alloc(len);
      // Java: this.copy(pointer, des, len);
      self.copy__int_int_int(pointer, des, len);
      // Java: return des;
      des
    }

    // java public void copy(int src, int des, int len)
    pub fn copy__int_int_int(&mut self, src: i32, des: i32, len: i32) {
      // Java: if (len > 0 && src != des) {
      // Java:   int maxSrc = src + len; int maxDes = des + len;
      // Java:   this.prepareWriteableBlock(des);
      // Java:   if (src >= this.mCurBlockStart && maxSrc <= this.mCurBlockEnd && maxDes <= this.mCurBlockEnd) {
      // Java:     System.arraycopy(data, src - this.mCurBlockStart, data, des - this.mCurBlockStart, len);
      // Java:   } else if (src > des) {
      // Java:     for (int curDes = des; curSrc < maxSrc; curDes += var21) {
      // Java:       System.arraycopy(srcBlock.data, srcBlockOffset, this.mCurBlock.data, desOffset, var21);
      // Java:     }
      // Java:   } else {
      // Java:     for (int curMaxDes = maxDes; curMaxSrc > src; curMaxDes -= var19) {
      // Java:       System.arraycopy(srcBlock.data, curMaxSrc - var19 & memBlockManager.mask,
      // Java:           this.mCurBlock.data, curMaxDes - var19 & memBlockManager.mask, var19);
      // Java:     }
      // Java:   }
      // Java: }
      // Rust's compiled snapshot has one flat heap. copy_within performs
      // System.arraycopy's forward/backward overlap behavior across all
      // positions, including Java's cross-block cases.
      if len > 0 && src != des {
        let start = usize::try_from(src).expect("negative snapshot source");
        let dest = usize::try_from(des).expect("negative snapshot destination");
        let end = start.checked_add(len as usize).expect("snapshot copy overflow");
        assert!(end <= self.heap.len());
        assert!(dest.checked_add(len as usize).expect("snapshot copy overflow") <= self.heap.len());
        self.heap.copy_within(start..end, dest);
      }
    }

    // java public int getSum(int pointer, int len)
    pub fn getSum(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int max = Math.min(pointer + len, this.mData.size);
      let mut max: i32 = Math::min(pointer + len, self.mData.size);
      // java int curPointer = pointer;
      let mut curPointer: i32 = pointer;
      // java int widthCount;
      let mut widthCount: i32;
      // java int processThisTime;
      let mut processThisTime: i32;
      widthCount = 0;
      // java for(widthCount = 0 curPointer < max curPointer += processThisTime)
      while curPointer < max {
        // java if (curPointer < this.mCurBlockStart || curPointer >= this.mCurBlockEnd)
        if curPointer < self.mCurBlockStart || curPointer >= self.mCurBlockEnd {
          // java this.loadBlock(curPointer);
          self.loadBlock(curPointer);
        }
        // java processThisTime = Math.min(max - curPointer, this.mCurBlockEnd - curPointer);
        processThisTime = Math::min(max - curPointer, self.mCurBlockEnd - curPointer);
        // java int[] data = this.mCurBlock.data;
        let mut data: Vec<i32> = self.mCurBlock.data;
        // java int i = curPointer - this.mCurBlockStart;
        let mut i: i32 = curPointer - self.mCurBlockStart;
        let mut maxOffset: i32 = i + processThisTime;
        // java for(int maxOffset = i + processThisTime; i < maxOffset; ++i)
        while i < maxOffset {
          // java widthCount += data[i];
          widthCount = widthCount + data[(i) as usize];
        }
      }
      // java return widthCount;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java public int getAbsSum(int pointer, int len)
    pub fn getAbsSum(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int max = Math.min(pointer + len, this.mData.size);
      let mut max: i32 = Math::min(pointer + len, self.mData.size);
      // java int curPointer = pointer;
      let mut curPointer: i32 = pointer;
      // java int widthCount;
      let mut widthCount: i32;
      // java int processThisTime;
      let mut processThisTime: i32;
      widthCount = 0;
      // java for(widthCount = 0 curPointer < max curPointer += processThisTime)
      while curPointer < max {
        // java if (curPointer < this.mCurBlockStart || curPointer >= this.mCurBlockEnd)
        if curPointer < self.mCurBlockStart || curPointer >= self.mCurBlockEnd {
          // java this.loadBlock(curPointer);
          self.loadBlock(curPointer);
        }
        // java processThisTime = Math.min(max - curPointer, this.mCurBlockEnd - curPointer);
        processThisTime = Math::min(max - curPointer, self.mCurBlockEnd - curPointer);
        // java int[] data = this.mCurBlock.data;
        let mut data: Vec<i32> = self.mCurBlock.data;
        // java int i = curPointer - this.mCurBlockStart;
        let mut i: i32 = curPointer - self.mCurBlockStart;
        let mut maxOffset: i32 = i + processThisTime;
        // java for(int maxOffset = i + processThisTime; i < maxOffset; ++i)
        while i < maxOffset {
          // java widthCount += Math.abs(data[i]);
          widthCount = widthCount + Math::abs(data[(i) as usize]);
        }
      }
      // java return widthCount;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java public void processRTLWidths(int pointer, int len)
    pub fn processRTLWidths(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int max = Math.min(pointer + len, this.mData.size);
      let mut max: i32 = Math::min(pointer + len, self.mData.size);
      // java int processThisTime;
      let mut processThisTime: i32;
      let mut curPointer: i32 = pointer;
      // java for(int curPointer = pointer; curPointer < max curPointer += processThisTime)
      while curPointer < max {
        // java if (curPointer < this.mCurBlockStart || curPointer >= this.mCurBlockEnd || !this.mCurBlockWriteable)
        if curPointer < self.mCurBlockStart || curPointer >= self.mCurBlockEnd || !self.mCurBlockWriteable {
          // java this.prepareWriteableBlock(curPointer);
          self.prepareWriteableBlock(curPointer);
        }
        // java processThisTime = Math.min(max - curPointer, this.mCurBlockEnd - curPointer);
        processThisTime = Math::min(max - curPointer, self.mCurBlockEnd - curPointer);
        // java int[] data = this.mCurBlock.data;
        let mut data: Vec<i32> = self.mCurBlock.data;
        // java int i = curPointer - this.mCurBlockStart;
        let mut i: i32 = curPointer - self.mCurBlockStart;
        let mut maxOffset: i32 = i + processThisTime;
        // java for(int maxOffset = i + processThisTime; i < maxOffset; ++i)
        while i < maxOffset {
          // java data[i] = -Math.abs(data[i]);
          data[(i) as usize] = -Math::abs(data[(i) as usize]);
        }
      }
      **/
    }

    // java public final T getObj(int index)
    pub fn getObj(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Object obj = null;
      let mut obj: Box<dyn Object> = Default::default();
      // java IShareObjIndexdMap.IShareObjIndexed shareObj = this.mData.shareObjIndexdMap.getObjValue(index);
      let mut shareObj: IShareObjIndexdMap::IShareObjIndexed = self.mData.shareObjIndexdMap.getObjValue(index);
      // java if (shareObj != null)
      if true {
        // java obj = shareObj.getHostObj();
        obj = shareObj.getHostObj();
      }
      // java T ret = (T)(obj);
      let mut ret: T = (obj).downcast_ref::<T>().unwrap();
      // java return ret;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java public final int addObj(IShareObjIndexdMap.IShareObjIndexed object)
    pub fn addObj(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return object == null ? 0 : this.mData.shareObjIndexdMap.getObjIndex(object);
      return (if false { 0 } else { self.mData.shareObjIndexdMap.getObjIndex(object) });
      **/
    }

    // java public final TypoObjsIndexdMap getObjsIndexMap()
    pub fn getObjsIndexMap(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mData.normalObjIndexdMap;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java public void beforeCommit(long version)
    pub fn beforeCommit(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mData.normalObjIndexdMap.submitAddObjs();
      self.mData.normalObjIndexdMap.submitAddObjs();
      // java this.mData.version = version;
      self.mData.version = version;
      // java if (this.mData.version == 1L)
      if self.mData.version == 1 {
        // java this.setFirstFrozenSize();
        self.setFirstFrozenSize();
      }
      // java CrackRanges crackRanges = this.mData.crackRanges;
      let mut crackRanges: CrackRanges = { let __x: CrackRanges = todo!("no align strategy matched"); __x };
      // java if (crackRanges != null && !crackRanges.isFrozen())
      if true && !crackRanges.isFrozen() {
        // java crackRanges.commit();
        crackRanges.commit();
      }
      // java PagePositionMap pagePositionMap = this.mData.pagePositionMap;
      let mut pagePositionMap: PagePositionMap = { let __x: PagePositionMap = todo!("no align strategy matched"); __x };
      // java if (pagePositionMap != null && !pagePositionMap.isFrozen())
      if true && !pagePositionMap.isFrozen() {
        // java pagePositionMap.commit(this);
        pagePositionMap.commit(self);
      }
      **/
    }

    // java public void setFirstFrozenSize()
    pub fn setFirstFrozenSize(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mData.shareObjIndexdMap.getFrozenSize() == -1)
      if self.mData.shareObjIndexdMap.getFrozenSize() == -1 {
        // java this.mData.shareObjIndexdMap.setFrozenSize();
        self.mData.shareObjIndexdMap.setFrozenSize();
      }
      // java this.mData.normalObjIndexdMap.setFirstFrozenSize();
      self.mData.normalObjIndexdMap.setFirstFrozenSize();
      **/
    }

    // java public final int getTypoType(int pointer)
    pub fn getTypoType(&self, pointer: i32) -> i32 {
      if self.checkIn(pointer) { self.get__int(pointer) } else { -1 }
    }

    // java public TypoMemoryObj getEntraceObj()
    pub fn getEntraceObj(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mEntraceObj;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java public int addCPRef(Document doc, int cp)
    pub fn addCPRef(&mut self, doc: Option<&dyn Document>, cp: i32) -> i32 {
      // java return ICPRefable.Impl.addRefCP(doc.getType(), cp, this.mData.cpRefs);
      let docType = doc.map(|d| d.getType()).unwrap_or(0);
      self.addRefCP(docType, cp)
    }

    // java public int addRefCP(int docType, int cp)
    pub fn addRefCP(&mut self, docType: i32, cp: i32) -> i32 {
      // java return ICPRefable.Impl.addRefCP(docType, cp, this.mData.cpRefs);
      let idx = docType as usize;
      if idx >= 7 {
        return i32::MIN | (docType << 28);
      }
      let slot = &mut self.mData.cp_ref_slots[idx];
      let insertPos: i32;
      if slot.is_none() {
        *slot = Some(vec![cp]);
        insertPos = 0;
      } else {
        let list = slot.as_mut().unwrap();
        let size = list.len() as i32;
        if size > 0 && cp == list[(size as usize) - 1] {
          insertPos = size - 1;
        } else {
          insertPos = size;
          list.push(cp);
        }
      }
      i32::MIN | (docType << 28) | insertPos
    }

    // java public int getCPRef(int ref)
    pub fn getCPRef(&self, r#ref: i32) -> i32 {
      // java return ICPRefable.Impl.getRefCP(ref, this.mData.cpRefs);
      if (r#ref & i32::MIN) != i32::MIN {
        return -1;
      }
      let subDocType = ((r#ref & 1879048192) >> 28) as usize;
      if subDocType >= 7 {
        return -1;
      }
      if let Some(list) = self.mData.cp_ref_slots[subDocType].as_ref() {
        let index = (r#ref & 268435455) as usize;
        if index < list.len() {
          return list[index];
        }
      }
      -1
    }

    // java public int getDocType(int ref)
    pub fn getDocType(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return ICPRefable.Impl.getDocType(ref);
      return ICPRefable::Impl.getDocType(r#ref);
      **/
    }

    // java public RevisionCache.Item getRevision(Document document, int cp, int revisionType, RevisionCache.Item item)
    pub fn getRevision(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return RevisionCache.get(document, cp, revisionType, this, item);
      return RevisionCache::get(document, cp, revisionType, self, item);
      **/
    }

    // java public RevisionCache.Item getTableRevision(Document document, int cp, int level, RevisionCache.Item item)
    pub fn getTableRevision(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return RevisionCache.getTableRevision(document, cp, level, this, item);
      return RevisionCache::getTableRevision(document, cp, level, self, item);
      **/
    }

    // java  IntArrayList[] getCPRefs()
    pub fn getCPRefs(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mData.cpRefs;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java  IntArrayList[] getCPRefFreeLists()
    pub fn getCPRefFreeLists(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mData.cpRefFreeLists;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java public int getSectionPageCount(int curPageIndex)
    pub fn getSectionPageCount(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return ((TypoCache)this.mEntraceObj).getSectionPageCount(curPageIndex);
      return (self.mEntraceObj).downcast_ref::<TypoCache>().unwrap().getSectionPageCount(curPageIndex);
      **/
    }

    // java public int getSectionPageIndex(int pageIndex)
    pub fn getSectionPageIndex(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return ((TypoCache)this.mEntraceObj).getSectionPageIndex(pageIndex);
      return (self.mEntraceObj).downcast_ref::<TypoCache>().unwrap().getSectionPageIndex(pageIndex);
      **/
    }

    // java public int getSectionStartPageIndex(int pageIndex)
    pub fn getSectionStartPageIndex(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return ((TypoCache)this.mEntraceObj).getSectionStartPageIndex(pageIndex);
      return (self.mEntraceObj).downcast_ref::<TypoCache>().unwrap().getSectionStartPageIndex(pageIndex);
      **/
    }

    // java public int getSectionMsoNfc(int pageIndex)
    pub fn getSectionMsoNfc(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return ((TypoCache)this.mEntraceObj).getSectionMsoNfc(pageIndex);
      return (self.mEntraceObj).downcast_ref::<TypoCache>().unwrap().getSectionMsoNfc(pageIndex);
      **/
    }

    // java public int getLayoutPageSize()
    pub fn getLayoutPageSize(&self) -> i32 {
      // java return TypoList.size(((TypoCache)this.mEntraceObj).getLayoutPageList(), this);
      0
    }

    // java public void startSection(int secIndex, int secCount, int curPageIndex, boolean restartPageNumber, int curSecPageIndex, int curSecMsoNfc)
    pub fn startSection(&mut self, secIndex: i32, secCount: i32, curPageIndex: i32, restartPageNumber: bool, curSecPageIndex: i32, curSecMsoNfc: i32) {
      // java ((TypoCache)this.mEntraceObj).startSection(...); sections pointer is cache word + 7.
      let entrance = self.ensure_typo_cache();
      let old = self.get__int(entrance + 7);
      let sections = start_typo_section(old, secIndex, secCount, curPageIndex, restartPageNumber, curSecPageIndex, curSecMsoNfc, self);
      if sections != old {
        self.set__int_int(entrance + 7, sections);
      }
    }

    // java TypoSections.size(((TypoCache)mEntraceObj).getSections(), this)
    pub fn sectionListSize(&self) -> i32 {
      if self.mEntraceObj == 0 {
        return 0;
      }
      let sections = self.get__int(self.mEntraceObj + 7);
      if sections == 0 { 0 } else { self.get__int(sections + 2) }
    }

    // java public int getLayoutPageList()
    pub fn getLayoutPageList(&self) -> i32 {
      // java return ((TypoCache)this.mEntraceObj).getLayoutPageList(); OFFSET_LAYOUTPAGES = 6
      if self.mEntraceObj == 0 { 0 } else { self.get__int(self.mEntraceObj + 6) }
    }

    // java TypoCache.alloc: size 16, type 0, layout-page list capacity 64.
    // Rust snapshots used by layout are not always initCache'd first.
    fn ensure_typo_cache(&mut self) -> i32 {
      if self.mEntraceObj == 0 {
        let pointer = self.alloc(16);
        self.set__int_int_int(pointer, 0, 16);
        let pages = crate::cn::wps::moffice::writer::cache::TypoList::TypoList::alloc__typosnapshot_int(self, 64);
        self.set__int_int(pointer + 6, pages);
        self.mEntraceObj = pointer;
      }
      self.mEntraceObj
    }

    // java public void setDocumentWidth(int width)
    pub fn setDocumentWidth(&mut self, width: i32) {
      // java ((TypoCache)this.mEntraceObj).setWidth(width); OFFSET_WIDTH = 2
      let pointer = self.ensure_typo_cache();
      self.set__int_int(pointer + 2, width);
    }

    // java public int getDocumentWidth()
    pub fn getDocumentWidth(&self) -> i32 {
      // java return ((TypoCache)this.mEntraceObj).width(); OFFSET_WIDTH = 2
      if self.mEntraceObj == 0 {
        return 0;
      }
      self.get__int(self.mEntraceObj + 2)
    }

    // java public void setDocumentHeight(int height)
    pub fn setDocumentHeight(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ((TypoCache)this.mEntraceObj).setHeight(height);
      (self.mEntraceObj).downcast_ref::<TypoCache>().unwrap().setHeight(height);
      **/
    }

    // java public int getDocumentHeight()
    pub fn getDocumentHeight(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return ((TypoCache)this.mEntraceObj).height();
      return (self.mEntraceObj).downcast_ref::<TypoCache>().unwrap().height();
      **/
    }

    // java public void setDocumentTop(int top)
    pub fn setDocumentTop(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ((TypoCache)this.mEntraceObj).setTop(top);
      (self.mEntraceObj).downcast_ref::<TypoCache>().unwrap().setTop(top);
      **/
    }

    // java public int getDocumentTop()
    pub fn getDocumentTop(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return ((TypoCache)this.mEntraceObj).getTop();
      return (self.mEntraceObj).downcast_ref::<TypoCache>().unwrap().getTop();
      **/
    }

    // java public void setDocumentBottom(int bottom)
    pub fn setDocumentBottom(&self, _bottom: i32) {
      // java ((TypoCache)this.mEntraceObj).setBottom(bottom);
      // rust: TypoCache off-surface; empty snapshot has no page list
    }

    // java public int getDocumentBottom()
    pub fn getDocumentBottom(&self) -> i32 {
      // java return ((TypoCache)this.mEntraceObj).getBottom();
      0
    }

    // java public int getMaxPageHeight()
    pub fn getMaxPageHeight(&self) -> i32 {
      // java ((TypoCache)this.mEntraceObj).getMaxPageHeight(); OFFSET_MAXPAGEHEIGHT = 5
      if self.mEntraceObj == 0 { 0 } else { self.get__int(self.mEntraceObj + 5) }
    }

    // java public void setMaxPageHeight(int maxPageHeight)
    pub fn setMaxPageHeight(&mut self, maxPageHeight: i32) {
      // java ((TypoCache)this.mEntraceObj).setMaxPageHeight(maxPageHeight);
      let pointer = self.ensure_typo_cache();
      self.set__int_int(pointer + 5, maxPageHeight);
    }

    // java public int getTemplete(int index)
    pub fn getTemplete(&self, index: i32) -> i32 {
      if self.mEntraceObj == 0 {
        return 0;
      }
      let sections = self.get__int(self.mEntraceObj + 7);
      if sections == 0 {
        return 0;
      }
      let section_index = index / 3;
      let template_index = index % 3;
      let size = self.get__int(sections + 2);
      if section_index >= 0 && section_index <= size && template_index >= 0 && template_index < 3 {
        self.get__int(sections + 3 + section_index * 7 + 4 + template_index)
      } else {
        0
      }
    }

    // java public int getSectionIndex(int index)
    pub fn getSectionIndex(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return ((TypoCache)this.mEntraceObj).getSectionIndex(index);
      return (self.mEntraceObj).downcast_ref::<TypoCache>().unwrap().getSectionIndex(index);
      **/
    }

    // java public void setTemplete(int index, int pointer)
    pub fn setTemplete(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ((TypoCache)this.mEntraceObj).setTemplete(pointer, index);
      (self.mEntraceObj).downcast_ref::<TypoCache>().unwrap().setTemplete(pointer, index);
      **/
    }

    // java public boolean isTemplateReachable(int template)
    pub fn isTemplateReachable(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int sections = ((TypoCache)this.mEntraceObj).getSections();
      let mut sections: i32 = { let __x: i32 = todo!("no align strategy matched"); __x };
      // java int secIndex = 0;
      let mut secIndex: i32 = 0;
      let mut sectionSize: i32 = TypoSections::size(sections, self);
      // java for(int sectionSize = TypoSections.size(sections, this); secIndex < sectionSize ++secIndex)
      while secIndex < sectionSize {
        let mut tempIndex: i32 = 0;
        // java for(int tempIndex = 0; tempIndex < 3 ++tempIndex)
        while tempIndex < 3 {
          // java if (TypoSections.getTemplete(secIndex, tempIndex, sections, this) == template)
          if TypoSections::getTemplete(secIndex, tempIndex, sections, self) == template {
            // java return true;
            return { let __x: () = todo!("no align strategy matched"); __x };
          }
        }
      }
      // java return false;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java public int getSections()
    pub fn getSections(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return ((TypoCache)this.mEntraceObj).getSections();
      return (self.mEntraceObj).downcast_ref::<TypoCache>().unwrap().getSections();
      **/
    }

    // java public void updateSections(int pSections)
    pub fn updateSections(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ((TypoCache)this.mEntraceObj).updateSections(pSections);
      (self.mEntraceObj).downcast_ref::<TypoCache>().unwrap().updateSections(pSections);
      **/
    }

    // java public int getFirstInfo()
    pub fn getFirstInfo(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return ((TypoCache)this.mEntraceObj).getFirstInfo();
      return (self.mEntraceObj).downcast_ref::<TypoCache>().unwrap().getFirstInfo();
      **/
    }

    // java public void updateLayoutPageList(int pLayoutPageList)
    pub fn updateLayoutPageList(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ((TypoCache)this.mEntraceObj).updateLayoutPageList(pLayoutPageList);
      (self.mEntraceObj).downcast_ref::<TypoCache>().unwrap().updateLayoutPageList(pLayoutPageList);
      **/
    }

    // java @Deprecated public TextDocument getDocument()
    pub fn getDocument(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return ((TypoCache)this.mEntraceObj).getDocument();
      return (self.mEntraceObj).downcast_ref::<TypoCache>().unwrap().getDocument();
      **/
    }

    // java @Deprecated public TypoDocument getTypoDocument()
    pub fn getTypoDocument(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return ((TypoCache)this.mEntraceObj).getTypoDocument();
      return (self.mEntraceObj).downcast_ref::<TypoCache>().unwrap().getTypoDocument();
      **/
    }

    // java public IPosVisitor getPosVisitor()
    pub fn getPosVisitor(&self) -> IPosVisitor {
      // rust: PosVistitorImpl/TypoCache off-surface; empty snapshot has no page grid
      IPosVisitor
    }

    // java public void insertPage(int pLayoutPage, int pagePos)
    pub fn insertPage(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ((TypoCache)this.mEntraceObj).insertPage(pLayoutPage, pagePos);
      (self.mEntraceObj).downcast_ref::<TypoCache>().unwrap().insertPage(pLayoutPage, pagePos);
      **/
    }

    // java public CrackRanges getCrashRanges()
    pub fn getCrashRanges(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mData.crackRanges;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java public CrackRanges getEditCrackRanges()
    pub fn getEditCrackRanges(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mData.crackRanges == null)
      if false {
        // java this.mData.crackRanges = new CrackRanges();
        self.mData.crackRanges = CrackRanges::new();
      }
      // java else if (this.mData.crackRanges.isFrozen())
      else if self.mData.crackRanges.isFrozen() {
        // java this.mData.crackRanges = this.mData.crackRanges.copyTo();
        self.mData.crackRanges = self.mData.crackRanges.copyTo();
      }
      // java return this.mData.crackRanges;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java public PagePositionMap getPagePosition()
    pub fn getPagePosition(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mData.pagePositionMap;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java public void getContentRect(ITypoRect typoRect)
    pub fn getContentRect(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java PagePositionMap posMap = this.mData.pagePositionMap;
      let mut posMap: PagePositionMap = { let __x: PagePositionMap = todo!("no align strategy matched"); __x };
      // java if (posMap != null)
      if true {
        // java typoRect.set(0, 0, posMap.getDocumentWidth(), posMap.getDocumentHeight());
        typoRect.set__int_int_int_int(0, 0, posMap.getDocumentWidth(), posMap.getDocumentHeight());
      }
      // java else
      else {
        // java typoRect.set(0, this.getDocumentTop(), this.getDocumentWidth(), this.getDocumentHeight());
        typoRect.set__int_int_int_int(0, self.getDocumentTop(), self.getDocumentWidth(), self.getDocumentHeight());
      }
      **/
    }

    // java public boolean hasGap()
    pub fn hasGap(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mData.pagePositionMap == null)
      if false {
        // java return false;
        return { let __x: () = todo!("no align strategy matched"); __x };
      }
      // java else
      else {
        // java return this.mData.pagePositionMap.getGap() > 0;
        return self.mData.pagePositionMap.getGap() > 0;
      }
      **/
    }

    // java public PagePositionMap getEditPagePosition()
    pub fn getEditPagePosition(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mData.pagePositionMap == null)
      if false {
        // java this.mData.pagePositionMap = new PagePositionMap(this.getTypoDocument().getPageGap());
        self.mData.pagePositionMap = PagePositionMap::new(self.getTypoDocument().getPageGap());
      }
      // java else if (this.mData.pagePositionMap.isFrozen())
      else if self.mData.pagePositionMap.isFrozen() {
        // java this.mData.pagePositionMap = this.mData.pagePositionMap.copyTo();
        self.mData.pagePositionMap = self.mData.pagePositionMap.copyTo();
      }
      // java return this.mData.pagePositionMap;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java public void swapToLocal(CrackRanges.Item crackItem, WIntArrayList excludeBlockIndex, boolean sync)
    pub fn swapToLocal(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int startBlock = SwapConfig.calStartBlockInexForAlignOrder(crackItem.getAlignOrder());
      let mut startBlock: i32 = SwapConfig::calStartBlockInexForAlignOrder(crackItem.getAlignOrder());
      // java Data var10000 = this.mData;
      let mut var10000: Box<Data> = { let __x: Box<Box<Data>> = todo!("no align strategy matched"); __x };
      // java TypoMemBlockManager memBlockManager = TypoSnapshot.Data.sMemBlockManager;
      let mut memBlockManager: TypoMemBlockManager = { let __x: TypoMemBlockManager = todo!("no align strategy matched"); __x };
      // java ArrayList<TypoMemBlock> blocks = this.mData.blocks;
      let mut blocks: Vec<TypoMemBlock> = { let __x: Vec<TypoMemBlock> = todo!("no align strategy matched"); __x };
      // java int i = startBlock;
      let mut i: i32 = startBlock;
      let mut endBlock: i32 = startBlock + 64;
      // java for(int endBlock = startBlock + 64; i < endBlock; ++i)
      while i < endBlock {
        // java if (!excludeBlockIndex.contains(i))
        if !excludeBlockIndex.contains(i) {
          // java TypoMemBlock block = (TypoMemBlock)blocks.get(i);
          let mut block: TypoMemBlock = (blocks.get(i)).downcast_ref::<TypoMemBlock>().unwrap();
          // java TypoMemBlockEx exBlock = memBlockManager.allocExBlock(block);
          let mut exBlock: TypoMemBlockEx = memBlockManager.allocExBlock(block);
          // java blocks.set(i, exBlock);
          blocks.set(i, exBlock);
        }
      }
      // java BlocksSwapManager.SwapItem swapItem = BlocksSwapManager.getInstance().swapToLocal(this.mData.blocks, crackItem.getAlignOrder(), sync);
      let mut swapItem: BlocksSwapManager::SwapItem = BlocksSwapManager::getInstance().swapToLocal(self.mData.blocks, crackItem.getAlignOrder(), sync);
      // java if (swapItem != null)
      if true {
        // java crackItem.setSwapItem(swapItem);
        crackItem.setSwapItem(swapItem);
        // java this.getEditCrackRanges().cleanItemFromLocalAll().insertItem(crackItem);
        self.getEditCrackRanges().cleanItemFromLocalAll().insertItem(crackItem);
      }
      // java else
      else {
        // java this.cancleBlockSwapWait(blocks, crackItem.getAlignOrder());
        self.cancleBlockSwapWait(blocks, crackItem.getAlignOrder());
      }
      **/
    }

    // java private void cancleBlockSwapWait(ArrayList<TypoMemBlock> blocks, int alignOrder)
    pub fn cancleBlockSwapWait(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int start = SwapConfig.calStartBlockInexForAlignOrder(alignOrder);
      let mut start: i32 = SwapConfig::calStartBlockInexForAlignOrder(alignOrder);
      // java int end = start + 63;
      let mut end: i32 = start + 63;
      // java int cancleCount = 0;
      let mut cancleCount: i32 = 0;
      let mut i: i32 = start;
      // java for(int i = start; i <= end; ++i)
      while i <= end {
        // java TypoMemBlock block = (TypoMemBlock)blocks.get(i);
        let mut block: TypoMemBlock = (blocks.get(i)).downcast_ref::<TypoMemBlock>().unwrap();
        // java if (block instanceof TypoMemBlockEx && ((TypoMemBlockEx)block).cancleSwapWait())
        if block.downcast_ref::<TypoMemBlockEx>().is_some() && (block).downcast_ref::<TypoMemBlockEx>().unwrap().cancleSwapWait() {
          // java ++cancleCount;
          { let _tmp = cancleCount; cancleCount += 1; _tmp };
        }
      }
      **/
    }

    // java public void replaceCrackRange(CrackRanges.Item oriCrackItem, ArrayList<TypoMemBlockEx> blocksForSwap)
    pub fn replaceCrackRange(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Data var10000 = this.mData;
      let mut var10000: Box<Data> = { let __x: Box<Box<Data>> = todo!("no align strategy matched"); __x };
      // java TypoMemBlockManager memBlockManager = TypoSnapshot.Data.sMemBlockManager;
      let mut memBlockManager: TypoMemBlockManager = { let __x: TypoMemBlockManager = todo!("no align strategy matched"); __x };
      // java ArrayList<TypoMemBlock> blocks = this.mData.blocks;
      let mut blocks: Vec<TypoMemBlock> = { let __x: Vec<TypoMemBlock> = todo!("no align strategy matched"); __x };
      // java CrackRanges.Item newCrackItem = oriCrackItem.copyForReplace();
      let mut newCrackItem: CrackRanges::Item = oriCrackItem.copyForReplace();
      // java BlocksSwapManager.SwapItem fileItem = oriCrackItem.getSwapItem();
      let mut fileItem: BlocksSwapManager::SwapItem = oriCrackItem.getSwapItem();
      // java if (fileItem != null)
      if true {
        // java for(TypoMemBlockEx bex : blocksForSwap)
        for bex in blocksForSwap.iter() {
          // java int blockIndex = bex.getBlockIndex();
          let mut blockIndex: i32 = bex.getBlockIndex();
          // java if (blockIndex != -1)
          if blockIndex != -1 {
            // java TypoMemBlock blockInCurSnapshot = (TypoMemBlock)blocks.get(blockIndex);
            let mut blockInCurSnapshot: TypoMemBlock = (blocks.get(blockIndex)).downcast_ref::<TypoMemBlock>().unwrap();
            // java if (blockInCurSnapshot != bex && bex.refCount > 0)
            if blockInCurSnapshot != bex && bex.refCount > 0 {
              // java bex.toLocalAgain();
              bex.toLocalAgain();
            }
            // java TypoMemBlockEx exBlock = memBlockManager.allocExBlock(blockInCurSnapshot);
            let mut exBlock: TypoMemBlockEx = memBlockManager.allocExBlock(blockInCurSnapshot);
            // java blocks.set(blockIndex, exBlock);
            blocks.set(blockIndex, exBlock);
          }
        }
      }
      // java CrackRanges crackRanges = this.getEditCrackRanges();
      let mut crackRanges: CrackRanges = { let __x: CrackRanges = todo!("no align strategy matched"); __x };
      // java crackRanges.removeItem(oriCrackItem);
      crackRanges.removeItem(oriCrackItem);
      // java BlocksSwapManager.SwapItem swapItem = BlocksSwapManager.getInstance().swapToLocal(this.mData.blocks, newCrackItem.getAlignOrder(), true);
      let mut swapItem: BlocksSwapManager::SwapItem = BlocksSwapManager::getInstance().swapToLocal(self.mData.blocks, newCrackItem.getAlignOrder(), true);
      // java if (swapItem != null)
      if true {
        // java newCrackItem.setSwapItem(swapItem);
        newCrackItem.setSwapItem(swapItem);
        // java crackRanges.insertItem(newCrackItem);
        crackRanges.insertItem(newCrackItem);
      }
      // java else
      else {
        // java this.cancleBlockSwapWait(blocks, newCrackItem.getAlignOrder());
        self.cancleBlockSwapWait(blocks, newCrackItem.getAlignOrder());
      }
      **/
    }

    // java public void swapFromLocal(CrackRanges.Item item)
    pub fn swapFromLocal(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java BlocksSwapManager.getInstance().swapFromLocal(item.getSwapItem(), true);
      BlocksSwapManager::getInstance().swapFromLocal(item.getSwapItem(), true);
      // java this.getEditCrackRanges().removeItem(item);
      self.getEditCrackRanges().removeItem(item);
      **/
    }

    // java public void updateCrackRanges()
    pub fn updateCrackRanges(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java CrackRanges crackRanges = this.getCrashRanges();
      let mut crackRanges: CrackRanges = { let __x: CrackRanges = todo!("no align strategy matched"); __x };
      // java if (crackRanges != null)
      if true {
        // java int removeStartIndex = -1;
        let mut removeStartIndex: i32 = -1;
        // java int maxIndex = this.getLayoutPageSize();
        let mut maxIndex: i32 = { let __x: i32 = todo!("no align strategy matched"); __x };
        // java if (maxIndex < 0)
        if maxIndex < 0 {
          // java maxIndex = 0;
          maxIndex = 0;
        }
        // java removeStartIndex = crackRanges.indexForPageIndex(maxIndex);
        removeStartIndex = crackRanges.indexForPageIndex(maxIndex);
        // java if (removeStartIndex < 0)
        if removeStartIndex < 0 {
          // java removeStartIndex = ~removeStartIndex;
          removeStartIndex = !removeStartIndex;
        }
        let mut i: i32 = (crackRanges.len() as i32) - 1;
        // java for(int i = crackRanges.size() - 1; i >= removeStartIndex --i)
        while i >= removeStartIndex {
          // java CrackRanges.Item item = crackRanges.get(i);
          let mut item: CrackRanges::Item = crackRanges.get(i);
          // java if (!item.isAllFromLocal() && !BlocksSwapManager.getInstance().swapFromLocal(item.getSwapItem(), true))
          if !item.isAllFromLocal() && !BlocksSwapManager::getInstance().swapFromLocal(item.getSwapItem(), true) {
            // java break;
            break;
          }
          // java if (crackRanges.isFrozen())
          if crackRanges.isFrozen() {
            // java crackRanges = this.getEditCrackRanges();
            crackRanges = self.getEditCrackRanges();
          }
          // java crackRanges.removeItem(item);
          crackRanges.removeItem(item);
        }
      }
      **/
    }

    // java public boolean checkInLocal(int pointer)
    pub fn checkInLocal(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.getCrashRanges() != null && this.checkIn(pointer))
      if self.getCrashRanges().is_some() && self.checkIn(pointer) {
        // java Data var10000 = this.mData;
        let mut var10000: Box<Data> = { let __x: Box<Box<Data>> = todo!("no align strategy matched"); __x };
        // java TypoMemBlockManager memBlockManager = TypoSnapshot.Data.sMemBlockManager;
        let mut memBlockManager: TypoMemBlockManager = { let __x: TypoMemBlockManager = todo!("no align strategy matched"); __x };
        // java int blockIndex = (pointer & ~memBlockManager.mask) >> memBlockManager.bits;
        let mut blockIndex: i32 = pointer & !memBlockManager.mask >> memBlockManager.bits;
        // java Data data = this.mData;
        let mut data: Box<Data> = { let __x: Box<Box<Data>> = todo!("no align strategy matched"); __x };
        // java ArrayList<TypoMemBlock> blocks = data.blocks;
        let mut blocks: Vec<TypoMemBlock> = { let __x: Vec<TypoMemBlock> = todo!("no align strategy matched"); __x };
        // java if (blockIndex > 0 && blockIndex < blocks.size())
        if blockIndex > 0 && blockIndex < (blocks.len() as i32) {
          // java TypoMemBlock block = (TypoMemBlock)blocks.get(blockIndex);
          let mut block: TypoMemBlock = (blocks.get(blockIndex)).downcast_ref::<TypoMemBlock>().unwrap();
          // java return block instanceof TypoMemBlockEx && block.data == null;
          return block.downcast_ref::<TypoMemBlockEx>().is_some() && block.data.is_none();
        }
        // java else
        else {
          // java return false;
          return { let __x: () = todo!("no align strategy matched"); __x };
        }
      }
      // java else
      else {
        // java return false;
        return { let __x: () = todo!("no align strategy matched"); __x };
      }
      **/
    }

    // java public boolean serializeJavaObjs()
    pub fn serializeJavaObjs(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TypoFirstInfo firstInfo = new TypoFirstInfo();
      let mut firstInfo: TypoFirstInfo = TypoFirstInfo::new();
      // java firstInfo.alloc(this);
      firstInfo.alloc(self);
      // java IntArrayList[] cpRefs = this.mData.cpRefs;
      let mut cpRefs: Vec<IntArrayList> = { let __x: Vec<IntArrayList> = todo!("no align strategy matched"); __x };
      // java if (cpRefs != null)
      if true {
        let mut subDoc: i32 = 0;
        // java for(int subDoc = 0; subDoc < cpRefs.length ++subDoc)
        while subDoc < (cpRefs.len() as i32) {
          // java IntArrayList subRefs = cpRefs[subDoc];
          let mut subRefs: IntArrayList = cpRefs[(subDoc) as usize];
          // java if (subRefs != null)
          if true {
            // java int pList = TypoList.add(subRefs, 0, this);
            let mut pList: i32 = TypoList::add__int_int_typosnapshot(subRefs, 0, self);
            // java ((TypoCache)this.mEntraceObj).setOfRefCps(pList, subDoc);
            (self.mEntraceObj).downcast_ref::<TypoCache>().unwrap().setOfRefCps(pList, subDoc);
          }
        }
      }
      // java IIntObjectMap<Object> objMaps = new WIntObjectHashMap();
      let mut objMaps: Box<dyn IIntObjectMap> = { let __x: Box<Box<dyn IIntObjectMap>> = todo!("no align strategy matched"); __x };
      // java objMaps.clear();
      objMaps.clear();
      // java if (this.mData.shareObjIndexdMap.getFrozenSize() > 0)
      if self.mData.shareObjIndexdMap.getFrozenSize() > 0 {
        // java objMaps.put(0, this.mData.shareObjIndexdMap);
        objMaps.put(0, self.mData.shareObjIndexdMap);
      }
      // java this.mData.normalObjIndexdMap.collectObjsMap(objMaps, false);
      self.mData.normalObjIndexdMap.collectObjsMap(objMaps, false);
      // java if (objMaps.size() > 0)
      if (objMaps.len() as i32) > 0 {
        // java int pTypoObjs = PersistObjs.saveToTypoObjs(objMaps, this);
        let mut pTypoObjs: i32 = PersistObjs::saveToTypoObjs__iintobjectmapobject_typosnapshot(objMaps, self);
        // java if (pTypoObjs == 0)
        if pTypoObjs == 0 {
          // java return false;
          return { let __x: () = todo!("no align strategy matched"); __x };
        }
        // java firstInfo.setTypoObjs(pTypoObjs);
        firstInfo.setTypoObjs(pTypoObjs);
      }
      // java firstInfo.setVerifyChars(TypoList.alloc(this, 8));
      firstInfo.setVerifyChars(TypoList::alloc__typosnapshot_int(self, 8));
      // java firstInfo.setMemEnd(this.getAllocSize());
      firstInfo.setMemEnd(self.getAllocSize());
      // java ((TypoCache)this.mEntraceObj).setFirstInfo(firstInfo.getPointer());
      (self.mEntraceObj).downcast_ref::<TypoCache>().unwrap().setFirstInfo(firstInfo.getPointer());
      // java this.set(0, -1996554224);
      self.set__int_int(0, -1996554224);
      // java return true;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java public boolean unserializeJavaObjs()
    pub fn unserializeJavaObjs(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int version = this.get(0);
      let mut version: i32 = self.get__int(0);
      // java if (version != -1996554224)
      if version != -1996554224 {
        // java return false;
        return { let __x: () = todo!("no align strategy matched"); __x };
      }
      // java else
      else {
        // java IIntObjectMap<Object> objMaps = new WIntObjectHashMap();
        let mut objMaps: Box<dyn IIntObjectMap> = { let __x: Box<Box<dyn IIntObjectMap>> = todo!("no align strategy matched"); __x };
        // java objMaps.put(0, this.mData.shareObjIndexdMap);
        objMaps.put(0, self.mData.shareObjIndexdMap);
        // java this.mData.normalObjIndexdMap.collectObjsMap(objMaps, true);
        self.mData.normalObjIndexdMap.collectObjsMap(objMaps, true);
        // java int pFirstInfo = ((TypoCache)this.mEntraceObj).getFirstInfo();
        let mut pFirstInfo: i32 = { let __x: i32 = todo!("no align strategy matched"); __x };
        // java if (TypoMemoryObj.getType(pFirstInfo, this) != 32)
        if TypoMemoryObj::getType__int_typosnapshot(pFirstInfo, self) != 32 {
          // java return false;
          return { let __x: () = todo!("no align strategy matched"); __x };
        }
        // java else
        else {
          // java int pTypoObjs = TypoFirstInfo.getTypoObjs(pFirstInfo, this);
          let mut pTypoObjs: i32 = TypoFirstInfo::getTypoObjs(pFirstInfo, self);
          // java if (TypoMemoryObj.getType(pTypoObjs, this) != 33)
          if TypoMemoryObj::getType__int_typosnapshot(pTypoObjs, self) != 33 {
            // java return false;
            return { let __x: () = todo!("no align strategy matched"); __x };
          }
          // java else if (!PersistObjs.loadFromTypoObjs(objMaps, pTypoObjs, this))
          else if !PersistObjs::loadFromTypoObjs(objMaps, pTypoObjs, self) {
            // java return false;
            return { let __x: () = todo!("no align strategy matched"); __x };
          }
          // java else
          else {
            // java IntArrayList[] cpRefs = this.mData.cpRefs;
            let mut cpRefs: Vec<IntArrayList> = { let __x: Vec<IntArrayList> = todo!("no align strategy matched"); __x };
            // java if (cpRefs != null)
            if true {
              let mut subDoc: i32 = 0;
              // java for(int subDoc = 0; subDoc < cpRefs.length ++subDoc)
              while subDoc < (cpRefs.len() as i32) {
                // java int pList = ((TypoCache)this.mEntraceObj).getOfRefCps(subDoc);
                let mut pList: i32 = (self.mEntraceObj).downcast_ref::<TypoCache>().unwrap().getOfRefCps(subDoc);
                // java if (pList != 0)
                if pList != 0 {
                  // java int size = TypoList.size(pList, this);
                  let mut size: i32 = TypoList::size__int_typosnapshot(pList, self);
                  // java IntArrayList subRefs = new IntArrayList(size);
                  let mut subRefs: IntArrayList = IntArrayList::new(size);
                  // java this.get(pList + 3, subRefs, size);
                  self.get__int_iintlist_int(pList + 3, subRefs, size);
                  // java cpRefs[subDoc] = subRefs;
                  cpRefs[(subDoc) as usize] = subRefs;
                }
              }
            }
            // java this.mData.size = TypoFirstInfo.getMemEnd(pFirstInfo, this);
            self.mData.size = TypoFirstInfo::getMemEnd(pFirstInfo, self);
            // java if (this.getLayoutPageSize() != 1)
            if self.getLayoutPageSize() != 1 {
            }
            // java int pFirstPage = TypoList.first(this.getLayoutPageList(), this);
            let mut pFirstPage: i32 = TypoList::first__int_typosnapshot(self.getLayoutPageList(), self);
            // java if (pFirstPage == 0)
            if pFirstPage == 0 {
            }
            // java this.getEditPagePosition().addPage(0, TypoLayoutPage.width(pFirstPage, this), TypoLayoutPage.height(pFirstPage, this), this.getDocumentWidth());
            self.getEditPagePosition().addPage(0, TypoLayoutPage::width(pFirstPage, self), TypoLayoutPage::height(pFirstPage, self), self.getDocumentWidth());
            // java TypoLayoutPage.allowNewVersionID(pFirstPage, this);
            TypoLayoutPage::allowNewVersionID(pFirstPage, self);
            // java return true;
            return { let __x: () = todo!("no align strategy matched"); __x };
          }
        }
      }
      **/
    }

    // java public String dumpCPRef()
    pub fn dumpCPRef(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java WIntArrayList[] cpRefs = this.mData.cpRefs;
      let mut cpRefs: Vec<WIntArrayList> = { let __x: Vec<WIntArrayList> = todo!("no align strategy matched"); __x };
      // java StringBuffer sb = new StringBuffer("CPRefs ");
      let mut sb: String = { let __x: String = todo!("no align strategy matched"); __x };
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = (cpRefs.len() as i32);
      // java for(int size = cpRefs.length; i < size; ++i)
      while i < size {
        // java WIntArrayList cpRef = cpRefs[i];
        let mut cpRef: WIntArrayList = cpRefs[(i) as usize];
        // java if (null == cpRef)
        if false {
          // java sb.append("0 ");
          sb.append("0 ");
        }
        // java else
        else {
          // java sb.append(cpRef.size() + " ");
          sb.append((cpRef.len() as i32) + " ");
        }
      }
      // java return sb.toString();
      return sb.toString();
      **/
    }

    // java public void dump(StringBuilder sb, boolean dumpObjDeatail)
    pub fn dump(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java sb.append(" snapshot v3:");
      sb.append__string(" snapshot v3:");
      // java if (this.mData != null)
      if true {
        // java sb.append("size ").append(this.mData.size);
        sb.append("size ").append(self.mData.size);
        // java sb.append(" blocks ").append(this.mData.blocks.size());
        sb.append(" blocks ").append((self.mData.blocks.len() as i32));
            // java sb.append(" inLocal ").append(this.getInLocalBlockSize());
            sb.append(" inLocal ").append(self.getInLocalBlockSize());
            // java sb.append(" pages ").append(this.getLayoutPageSize());
            sb.append(" pages ").append(self.getLayoutPageSize());
            // java sb.append(" top ").append(this.getDocumentTop());
            sb.append(" top ").append(self.getDocumentTop());
            // java sb.append(" bottom ").append(this.getDocumentBottom());
            sb.append(" bottom ").append(self.getDocumentBottom());
            // java Exception var4
            todo!();
        // java if (this.mData.shareObjIndexdMap != null)
        if true {
          // java sb.append(" share " + (dumpObjDeatail ? this.mData.shareObjIndexdMap.toString() : this.mData.shareObjIndexdMap.size()));
          sb.append(" share " + (if dumpObjDeatail { self.mData.shareObjIndexdMap.toString() } else { (self.mData.shareObjIndexdMap.len() as i32) }));
        }
        // java if (this.mData.normalObjIndexdMap != null)
        if true {
          // java sb.append(" normal " + (dumpObjDeatail ? this.mData.normalObjIndexdMap.toString() : this.mData.normalObjIndexdMap.size()));
          sb.append(" normal " + (if dumpObjDeatail { self.mData.normalObjIndexdMap.toString() } else { (self.mData.normalObjIndexdMap.len() as i32) }));
        }
      }
      **/
    }

    // java public int getBlockSize()
    pub fn getBlockSize(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mData != null ? this.mData.blocks.size() : 0;
      return (if true { (self.mData.blocks.len() as i32) } else { 0 });
      **/
    }

    // java public int getInLocalBlockSize()
    pub fn getInLocalBlockSize(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java CrackRanges crackRanges = this.mData.crackRanges;
      let mut crackRanges: CrackRanges = { let __x: CrackRanges = todo!("no align strategy matched"); __x };
      // java return crackRanges != null ? crackRanges.getInLocalSize() : 0;
      return (if true { crackRanges.getInLocalSize() } else { 0 });
      **/
    }

    // java public String getSummaryInfo()
    pub fn getSummaryInfo(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return " snapshot:size " + this.getBlockSize() + " share " + this.mData.shareObjIndexdMap.size() + " normal " + this.mData.normalObjIndexdMap.size();
      return " snapshot:size " + self.getBlockSize() + " share " + (self.mData.shareObjIndexdMap.len() as i32) + " normal " + (self.mData.normalObjIndexdMap.len() as i32);
      **/
    }

    // java public boolean checkFirstCacheSuitable()
    pub fn checkFirstCacheSuitable(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.mData == null)
      if false {
        // java return false;
        return { let __x: () = todo!("no align strategy matched"); __x };
      }
      // java else if (this.mData.version < 1L)
      else if self.mData.version < 1 {
        // java return false;
        return { let __x: () = todo!("no align strategy matched"); __x };
      }
      // java else
      else {
        // java if (this.mData.revisions != null)
        if true {
          let mut i: i32 = 0;
          // java for(int i = 0; i < this.mData.revisions.length; ++i)
          while i < (self.mData.revisions.len() as i32) {
            // java IntArrayList list = this.mData.revisions[i];
            let mut list: IntArrayList = self.mData.revisions[(i) as usize];
            // java if (list != null && list.size() > 0)
            if true && (list.len() as i32) > 0 {
              // java return false;
              return { let __x: () = todo!("no align strategy matched"); __x };
            }
          }
        }
        // java if (this.mData.blocks.size() > 2)
        if (self.mData.blocks.len() as i32) > 2 {
          // java return false;
          return { let __x: () = todo!("no align strategy matched"); __x };
        }
        // java else if (this.mData.normalObjIndexdMap != null && !this.mData.normalObjIndexdMap.checkFirstCacheSuitable())
        else if true && !self.mData.normalObjIndexdMap.checkFirstCacheSuitable() {
          // java return false;
          return { let __x: () = todo!("no align strategy matched"); __x };
        }
        // java else
        else {
          // java int pList = this.getLayoutPageList();
          let mut pList: i32 = (self.getLayoutPageList() as i32);
          // java int pageSize = TypoList.size(pList, this);
          let mut pageSize: i32 = TypoList::size__int_typosnapshot(pList, self);
          // java if (pageSize != 1)
          if pageSize != 1 {
            // java return false;
            return { let __x: () = todo!("no align strategy matched"); __x };
          }
          // java else
          else {
            // java int pFirstPage = TypoList.first(pList, this);
            let mut pFirstPage: i32 = TypoList::first__int_typosnapshot(pList, self);
            // java return pFirstPage != 0;
            return pFirstPage != 0;
          }
        }
      }
      **/
    }

    // java public void invalidFirstCache()
    pub fn invalidFirstCache(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java FirstCacheBase firstCacheBase = FirstCacheBase.getInstance();
      let mut firstCacheBase: FirstCacheBase = FirstCacheBase::getInstance();
      // java if (firstCacheBase != null)
      if true {
        // java firstCacheBase.onFirstCoreModify();
        firstCacheBase.onFirstCoreModify();
      }
      **/
    }

    // java public boolean isFirstCache()
    pub fn isFirstCache(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getVersion() == 0L;
      return self.getVersion() == 0;
      **/
    }

}

// java public static class SafeTrace implements TypoDocument.ISnapshotListener
impl SafeTrace {
    // java public  SafeTrace(String tag)
    pub fn SafeTrace(tag: &str) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.tag = tag;
      let mut this = Self { tag: tag, typoDocument: Default::default(), ref: Default::default(), usingThread: Default::default(), dirty: Default::default() };
      // java this.changeUsingThread();
      this.changeUsingThread();
      // java synchronized(sTraceList) { sTraceList.add(this);
      sTraceList.add(self);
      this
      **/
    }

    // java public void changeUsingThread()
    pub fn changeUsingThread(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.usingThread = Thread.currentThread().getId();
      self.usingThread = Thread::currentThread().getId();
      **/
    }

    // java public void dispose()
    pub fn dispose(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.release();
      self.release();
      // java this.typoDocument = null;
      self.typoDocument = None;
        // java (sTraceList)
        sTraceList;
          // java sTraceList.remove(this);
          sTraceList.remove(self);
      **/
    }

    // java  TypoDocument getTypoDocument()
    pub fn getTypoDocument(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.typoDocument;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java public void onSnapshotCommit(TypoDocument typoDocument)
    pub fn onSnapshotCommit(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.typoDocument = typoDocument;
      self.typoDocument = typoDocument;
      // java this.dirty = true;
      self.dirty = true;
      **/
    }

    // java public void release()
    pub fn release(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TypoSnapshot snapshot = this.detach();
      let mut snapshot: TypoSnapshot = { let __x: TypoSnapshot = todo!("no align strategy matched"); __x };
      // java if (snapshot != null)
      if true {
        // java snapshot.release();
        snapshot.release();
      }
      **/
    }

    // java public TypoSnapshot get()
    pub fn get(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (this.dirty)
      if self.dirty {
        // java this.release();
        self.release();
      }
      // java if (this.isEmpty())
      if self.isEmpty() {
        // java if (this.typoDocument != null)
        if true {
          // java this.ref = this.typoDocument.getSnapshot();
          self.r#ref = self.typoDocument.getSnapshot();
        }
        // java this.dirty = false;
        self.dirty = false;
      }
      // java return this.ref;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java public boolean isEmpty()
    pub fn isEmpty(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.ref == null;
      return false;
      **/
    }

    // java public TypoSnapshot detach()
    pub fn detach(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java TypoSnapshot old = this.ref;
      let mut old: TypoSnapshot = { let __x: TypoSnapshot = todo!("no align strategy matched"); __x };
      // java this.ref = null;
      self.r#ref = None;
      // java return old;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java static void releaseAssociatedTraceInstance(TypoDocument typoDocument)
    pub fn releaseAssociatedTraceInstance(&self) {
        todo!("DEFERRED_NON_FUNCTION");
        /* DEFERRED_NON_FUNCTION
        // java (sTraceList)
        sTraceList;
          let mut i: i32 = (sTraceList.len() as i32) - 1;
          // java for(int i = sTraceList.size() - 1; i >= 0 --i)
          while i >= 0 {
            // java SafeTrace trace = (SafeTrace)sTraceList.get(i);
            let mut trace: SafeTrace = (sTraceList.get(i)).downcast_ref::<SafeTrace>().unwrap();
            // java TypoDocument typoDoc = trace.getTypoDocument();
            let mut typoDoc: TypoDocument = { let __x: TypoDocument = todo!("no align strategy matched"); __x };
            // java if (typoDoc == null || typoDoc == typoDocument)
            if false || typoDoc == typoDocument {
              // java TypoSnapshot ref = trace.detach();
              let mut r#ref: TypoSnapshot = { let __x: TypoSnapshot = todo!("no align strategy matched"); __x };
              // java if (ref != null)
              if true {
                // java ref.release();
                r#ref.release();
              }
              // java sTraceList.remove(trace);
              sTraceList.remove(trace);
            }
          }
        **/
    }

}

// java public static class IntArrayList extends WIntArrayList
impl IntArrayList {
    // java public  IntArrayList()
    pub fn IntArrayList() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      Self { super_: WIntArrayList::new() }
      **/
    }

    // java public  IntArrayList(int capacity)
    pub fn IntArrayList__int(capacity: i32) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java super(capacity);
      Self { super_: WIntArrayList::new__int(capacity) }
      **/
    }

    // java public  IntArrayList(WIntArrayList src)
    pub fn IntArrayList__wintarraylist(src: &WIntArrayList) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.addAll(src);
      this.addAll(src);
      **/
    }

    // java public int[] getData()
    pub fn getData(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this._data;
      return self._data;
      **/
    }

    // java public void addAll(IntArrayList src)
    pub fn addAll(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int desSize = this._pos + src._pos;
      let mut desSize: i32 = self._pos + src._pos;
      // java this.ensureCapacity(desSize);
      self.super_.ensureCapacity(desSize);
      // java System.arraycopy(src._data, 0, this._data, this._pos, src._pos);
      System::arraycopy(src._data, 0, self._data, self._pos, src._pos);
      // java this._pos = desSize;
      self._pos = desSize;
      **/
    }

    // java public void offsetValues(int startValue, int offset)
    pub fn offsetValues(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int[] data = this._data;
      let mut data: Vec<i32> = self._data;
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = (data.len() as i32);
      // java for(int size = data.length; i < size; ++i)
      while i < size {
        // java int oldCP = data[i];
        let mut oldCP: i32 = data[(i) as usize];
        // java if (oldCP >= 0 && oldCP >= startValue)
        if oldCP >= 0 && oldCP >= startValue {
          // java int newCP = oldCP + offset;
          let mut newCP: i32 = oldCP + offset;
          // java if (newCP < startValue)
          if newCP < startValue {
            // java data[i] = -1;
            data[(i) as usize] = -1;
          }
          // java else
          else {
            // java data[i] = newCP;
            data[(i) as usize] = newCP;
          }
        }
      }
      **/
    }

}

// java static class Data
impl Data {
    // java public Data newObject()
    pub fn newObject() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return new Data();
      return Data::default();
      */
    }

    // java public  Data(IShareObjIndexdMap shareObjIndexdMap)
    pub fn Data__ishareobjindexdmap(shareObjIndexdMap: &dyn IShareObjIndexdMap) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.init(shareObjIndexdMap, new TypoObjsIndexdMap());
      this.init(shareObjIndexdMap, TypoObjsIndexdMap::new());
      **/
    }

    // java public  Data()
    pub fn Data() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      Self { shareObjIndexdMap: Default::default(), normalObjIndexdMap: Default::default(), blocks: Default::default(), size: Default::default(), freeList: Default::default(), cpRefs: Default::default(), cpRefFreeLists: Default::default(), revisions: Default::default(), crackRanges: Default::default(), pagePositionMap: Default::default(), version: Default::default(), refCount: Default::default(), modifyPageRange: Default::default() }
      **/
    }

    // java  void resetMaps()
    pub fn resetMaps(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.freeList.resetQuick();
      self.freeList.resetQuick();
      // java IntArrayList[] cpRefs = this.cpRefs;
      let mut cpRefs: Vec<IntArrayList> = { let __x: Vec<IntArrayList> = todo!("no align strategy matched"); __x };
      // java IntArrayList[] cpRefFreeLists = this.cpRefFreeLists;
      let mut cpRefFreeLists: Vec<IntArrayList> = { let __x: Vec<IntArrayList> = todo!("no align strategy matched"); __x };
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = (cpRefs.len() as i32);
      // java for(int size = cpRefs.length; i < size; ++i)
      while i < size {
        // java IntArrayList cpRef = cpRefs[i];
        let mut cpRef: IntArrayList = cpRefs[(i) as usize];
        // java if (null != cpRef)
        if true {
          // java cpRef.resetQuick();
          cpRef.resetQuick();
        }
        // java IntArrayList cpRefFree = cpRefFreeLists[i];
        let mut cpRefFree: IntArrayList = cpRefFreeLists[(i) as usize];
        // java if (null != cpRefFree)
        if true {
          // java cpRefFree.resetQuick();
          cpRefFree.resetQuick();
        }
      }
      // java if (null != this.revisions)
      if true {
        // java IntArrayList[] revisions = this.revisions;
        let mut revisions: Vec<IntArrayList> = { let __x: Vec<IntArrayList> = todo!("no align strategy matched"); __x };
        // java i = 0;
        i = 0;
        let mut size: i32 = (revisions.len() as i32);
        // java for(int size = revisions.length; i < size; ++i)
        while i < size {
          // java IntArrayList revision = revisions[i];
          let mut revision: IntArrayList = revisions[(i) as usize];
          // java if (null != revision)
          if true {
            // java revision.resetQuick();
            revision.resetQuick();
          }
        }
      }
      // java this.crackRanges = null;
      self.crackRanges = None;
      // java this.pagePositionMap = null;
      self.pagePositionMap = None;
      **/
    }

    // java  void clear()
    pub fn clear(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java sMemBlockManager.recycleBlock(this.blocks);
      sMemBlockManager.recycleBlock__arraylisttypomemblock(self.blocks);
      // java this.shareObjIndexdMap = null;
      self.shareObjIndexdMap = None;
      // java this.normalObjIndexdMap = null;
      self.normalObjIndexdMap = None;
      // java this.version = -1L;
      self.version = -1;
      // java this.modifyPageRange = 0L;
      self.modifyPageRange = 0;
      // java this.resetMaps();
      self.resetMaps();
      **/
    }

    // java public void init(IShareObjIndexdMap shareObjIndexdMap, TypoObjsIndexdMap normalObjIndexdMap)
    pub fn init(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.shareObjIndexdMap = shareObjIndexdMap;
      self.shareObjIndexdMap = shareObjIndexdMap;
      // java this.normalObjIndexdMap = normalObjIndexdMap;
      self.normalObjIndexdMap = normalObjIndexdMap;
      // java this.resetMaps();
      self.resetMaps();
      **/
    }

    // java  int getRefCount()
    pub fn getRefCount(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.refCount;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java synchronized void incRef()
    pub fn incRef(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java ++this.refCount;
      { let _tmp = self.refCount; self.refCount += 1; _tmp };
      **/
    }

    // java synchronized void decRef()
    pub fn decRef(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java --this.refCount;
      { let _tmp = self.refCount; self.refCount -= 1; _tmp };
      // java if (0 == this.refCount)
      if 0 == self.refCount {
        // java this.clear();
        self.clear();
        // java sDataPool.returnObject(this);
        sDataPool.returnObject(self);
      }
      **/
    }

    // java  Data fork()
    pub fn fork(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Data newData = sDataPool.borrowObject();
      let mut newData: Box<Data> = sDataPool.borrowObject();
      // java newData.init(this.shareObjIndexdMap, this.normalObjIndexdMap);
      newData.init(self.shareObjIndexdMap, self.normalObjIndexdMap);
      // java ArrayList<TypoMemBlock> newBlockData = newData.blocks;
      let mut newBlockData: Vec<TypoMemBlock> = { let __x: Vec<TypoMemBlock> = todo!("no align strategy matched"); __x };
      // java newBlockData.addAll(this.blocks);
      newBlockData.addAll(self.blocks);
      // java newData.freeList.addAll(this.freeList);
      newData.freeList.addAll(self.freeList);
      // java newData.size = this.size;
      newData.size = self.size;
      // java newData.version = this.version;
      newData.version = self.version;
      // java newData.modifyPageRange = 0L;
      newData.modifyPageRange = 0;
      // java int i = 0;
      let mut i: i32 = 0;
      let mut size: i32 = (newBlockData.len() as i32);
      // java for(int size = newBlockData.size(); i < size; ++i)
      while i < size {
        // java TypoMemBlock block = (TypoMemBlock)newBlockData.get(i);
        let mut block: TypoMemBlock = (newBlockData.get(i)).downcast_ref::<TypoMemBlock>().unwrap();
          // java (block)
          block;
            // java ++block.refCount;
            { let _tmp = block.refCount; block.refCount += 1; _tmp };
      }
      // java IntArrayList[] cpRefs = this.cpRefs;
      let mut cpRefs: Vec<IntArrayList> = { let __x: Vec<IntArrayList> = todo!("no align strategy matched"); __x };
      // java IntArrayList[] cpRefFreeLists = this.cpRefFreeLists;
      let mut cpRefFreeLists: Vec<IntArrayList> = { let __x: Vec<IntArrayList> = todo!("no align strategy matched"); __x };
      // java i = 0;
      i = 0;
      let mut size: i32 = 7;
      // java for(int size = 7; i < size; ++i)
      while i < size {
        // java IntArrayList cpRef = cpRefs[i];
        let mut cpRef: IntArrayList = cpRefs[(i) as usize];
        // java if (null != cpRef && cpRef.size() > 0)
        if true && (cpRef.len() as i32) > 0 {
          // java if (null == newData.cpRefs[i])
          if newData.cpRefs[(i) as usize].is_none() {
            // java newData.cpRefs[i] = new IntArrayList(cpRef.size());
            newData.cpRefs[(i) as usize] = IntArrayList::new((cpRef.len() as i32));
          }
          // java newData.cpRefs[i].addAll(cpRef);
          newData.cpRefs[(i) as usize].addAll(cpRef);
        }
        // java IntArrayList cpRefFree = cpRefFreeLists[i];
        let mut cpRefFree: IntArrayList = cpRefFreeLists[(i) as usize];
        // java if (null != cpRefFree && cpRefFree.size() > 0)
        if true && (cpRefFree.len() as i32) > 0 {
          // java if (null == newData.cpRefFreeLists[i])
          if newData.cpRefFreeLists[(i) as usize].is_none() {
            // java newData.cpRefFreeLists[i] = new IntArrayList(cpRefFree.size());
            newData.cpRefFreeLists[(i) as usize] = IntArrayList::new((cpRefFree.len() as i32));
          }
          // java newData.cpRefFreeLists[i].addAll(cpRefFree);
          newData.cpRefFreeLists[(i) as usize].addAll(cpRefFree);
        }
      }
      // java if (null != this.revisions)
      if true {
        // java if (null == newData.revisions)
        if false {
          // java newData.revisions = new IntArrayList[this.revisions.length];
          newData.revisions = vec![Default::default(); (self.revisions.len() as i32) as usize];
        }
        // java i = 0;
        i = 0;
        let mut size: i32 = (self.revisions.len() as i32);
        // java for(int size = this.revisions.length; i < size; ++i)
        while i < size {
          // java IntArrayList revision = this.revisions[i];
          let mut revision: IntArrayList = self.revisions[(i) as usize];
          // java if (revision != null && revision.size() > 0)
          if true && (revision.len() as i32) > 0 {
            // java if (null == newData.revisions[i])
            if newData.revisions[(i) as usize].is_none() {
              // java newData.revisions[i] = new IntArrayList(revision.size());
              newData.revisions[(i) as usize] = IntArrayList::new((revision.len() as i32));
            }
            // java newData.revisions[i].addAll(revision);
            newData.revisions[(i) as usize].addAll(revision);
          }
        }
      }
      // java newData.crackRanges = this.crackRanges;
      newData.crackRanges = self.crackRanges;
      // java newData.pagePositionMap = this.pagePositionMap;
      newData.pagePositionMap = self.pagePositionMap;
      // java return newData;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

    // java  Data cleanFork()
    pub fn cleanFork(&self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Data newData = sDataPool.borrowObject();
      let mut newData: Box<Data> = sDataPool.borrowObject();
      // java newData.init(this.shareObjIndexdMap, this.normalObjIndexdMap);
      newData.init(self.shareObjIndexdMap, self.normalObjIndexdMap);
      // java newData.size = 1;
      newData.size = 1;
      // java return newData;
      return { let __x: () = todo!("no align strategy matched"); __x };
      **/
    }

}
