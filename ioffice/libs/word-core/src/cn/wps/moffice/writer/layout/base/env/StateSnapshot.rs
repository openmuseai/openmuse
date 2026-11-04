// Java: import cn.wps.base.assertion.Assert
use util::base::assertion::Assert::Assert;
// Java: import cn.wps.collections.KList
use util::collections::KList::KList;
// Java: import cn.wps.kfc.api.list.IIntList
use util::kfc::api::list::IIntList::IIntList;
// Java: import cn.wps.kfc.api.map.IIntObjectMap
use util::kfc::api::map::IIntObjectMap::IIntObjectMap;
// Java: import cn.wps.kfc.impl.arrayList.WIntArrayList
use util::kfc::r#impl::arrayList::WIntArrayList::WIntArrayList;
// Java: import cn.wps.kfc.impl.map.WIntObjectHashMap
use util::kfc::r#impl::map::WIntObjectHashMap::WIntObjectHashMap;
// Java: import cn.wps.moffice.writer.cache.TypoSnapshot
use crate::cn::wps::moffice::writer::cache::TypoSnapshot::TypoSnapshot;
// Java: import cn.wps.moffice.writer.cache.TypoStateSnapshot
use crate::cn::wps::moffice::writer::cache::TypoStateSnapshot::TypoStateSnapshot;
// Java: import cn.wps.moffice.writer.data.RANGEFactory
use crate::cn::wps::moffice::writer::data::RANGEFactory::RANGEFactory;
// Java: import cn.wps.moffice.writer.layout.base.env.Pools
use crate::cn::wps::moffice::writer::layout::base::env::Pools::Pools;
// Java: import java.lang.Object
use stubs::java::lang::Object::Object;

// java public static final int KEY_START_CP = 0;
pub(crate) const KEY_START_CP: i32 = 0;
// java public static final int KEY_TABLEPERILAYER = 1;
pub(crate) const KEY_TABLEPERILAYER: i32 = 1;
// java public static final int KEY_TEXTFRAMELAYER = 2;
pub(crate) const KEY_TEXTFRAMELAYER: i32 = 2;
// java public static final int KEY_DOC_TYPE = 3;
pub(crate) const KEY_DOC_TYPE: i32 = 3;
// java public static final int KEY_RANGE = 4;
pub(crate) const KEY_RANGE: i32 = 4;
// java public static final int KEY_PAGE_AllowOverCoreHeight = 5;
pub(crate) const KEY_PAGE_AllowOverCoreHeight: i32 = 5;
// java public static final int KEY_PAGE_GridHeight = 6;
pub(crate) const KEY_PAGE_GridHeight: i32 = 6;
// java public static final int KEY_PAGE_GridWidth = 7;
pub(crate) const KEY_PAGE_GridWidth: i32 = 7;
// java public static final int KEY_PAGE_GridModel = 8;
pub(crate) const KEY_PAGE_GridModel: i32 = 8;
// java public static final int KEY_PAGE_CharSpaceForGrid = 9;
pub(crate) const KEY_PAGE_CharSpaceForGrid: i32 = 9;
// java public static final int KEY_FIRSTMERGECELL = 10;
pub(crate) const KEY_FIRSTMERGECELL: i32 = 10;
// java public static final int KEY_FIRSTMERGED = 11;
pub(crate) const KEY_FIRSTMERGED: i32 = 11;
// java public static final int KEY_LASTMERGED = 12;
pub(crate) const KEY_LASTMERGED: i32 = 12;
// java public static final int KEY_VERTMERGED = 13;
pub(crate) const KEY_VERTMERGED: i32 = 13;
// java public static final int KEY_BREAKCELL = 14;
pub(crate) const KEY_BREAKCELL: i32 = 14;
// java public static final int KEY_CANTSPLIT = 15;
pub(crate) const KEY_CANTSPLIT: i32 = 15;
// java public static final int KEY_FOOT_END_NOTE_CUR_FOOT_NOTE_REF_CP = 16;
pub(crate) const KEY_FOOT_END_NOTE_CUR_FOOT_NOTE_REF_CP: i32 = 16;
// java public static final int KEY_FOOT_END_NOTE_CUR_FOOT_NOTE_STATE = 17;
pub(crate) const KEY_FOOT_END_NOTE_CUR_FOOT_NOTE_STATE: i32 = 17;
// java public static final int KEY_FOOT_END_NOTE_FOOT_NOTE_CPS = 18;
pub(crate) const KEY_FOOT_END_NOTE_FOOT_NOTE_CPS: i32 = 18;
// java public static final int KEY_FOOT_END_NOTE_CUR_END_NOTE_REF_CP = 19;
pub(crate) const KEY_FOOT_END_NOTE_CUR_END_NOTE_REF_CP: i32 = 19;
// java public static final int KEY_FOOT_END_NOTE_CUR_END_NOTE_STATE = 20;
pub(crate) const KEY_FOOT_END_NOTE_CUR_END_NOTE_STATE: i32 = 20;
// java public static final int KEY_MAX = 21;
pub(crate) const KEY_MAX: i32 = 21;
// java public static final int KEY_CELL_STATES = 21;
pub(crate) const KEY_CELL_STATES: i32 = 21;
// java static final int KEY_MAX_ALLOW = 31;
pub(crate) const KEY_MAX_ALLOW: i32 = 31;

// java public class StateSnapshot extends KList.RecycableNode
#[derive(Default)]
pub struct StateSnapshot {
    // java KList.RecycableNode super_
    pub(crate) super_: RecycableNode,
    // java protected TStateMap mStateMap;
    pub(crate) mStateMap: Option<TStateMapHolder>,
    // java protected KList<StateSnapshot> mCellStates;
    pub(crate) mCellStates: Vec<Option<StateSnapshotHolder>>,
    // java private Range mCacheRange;
    mCacheRange: Option<RangeHolder>,
    // java final WIntArrayList tempCaps;
    pub(crate) tempCaps: Option<final_WIntArrayListHolder>,
}

// java public static class Range
#[derive(Default)]
pub struct Range {
    // java long range;
    pub(crate) range: i32,
}

// java protected static class Alloc extends KList.RecycableNodeAlloc<StateSnapshot>
#[derive(Default)]
pub struct Alloc {
    // java KList.RecycableNodeAlloc<StateSnapshot> super_
    pub(crate) super_: RecycableNodeAlloc<StateSnapshot>,
    // java protected Pools mPools;
    pub(crate) mPools: Option<PoolsHolder>,
}

// java static class TStateMap
#[derive(Default)]
pub struct TStateMap {
    // java private int mKeyMax;
    mKeyMax: i32,
    // java private int mBitsIntkeys;
    mBitsIntkeys: i32,
    // java private int mBitsObjetKeys;
    mBitsObjetKeys: i32,
    // java private int[] mIntValues;
    mIntValues: Vec<i32>,
    // java private IIntObjectMap<Object> mObjectValues;
    mObjectValues: Option<IIntObjectMap_Object_Holder>,
}

// java public class StateSnapshot extends KList.RecycableNode
impl StateSnapshot {
    // java public Range reuseRangeInstance(long range)
    pub fn reuseRangeInstance(&self, range: i32) -> Option<RangeHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mCacheRange.range = range;
      self.mCacheRange.range = range;
      // java return this.mCacheRange;
      return self.mCacheRange;
      */
    }

    // java public  StateSnapshot()
    pub fn new() -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this(21);
      let mut this = Self::new__int(21);
      this
      */
    }

    // java protected  StateSnapshot(int key_max)
    pub fn new__int(key_max: i32) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java super(false);
      // java this.mCellStates = new KList();
      Self { super_: RecycableNode::new(false), mStateMap: TStateMap::new(key_max), mCellStates: KList::new(), mCacheRange: Range::new(), tempCaps: WIntArrayList::new() }
      // java this.mCacheRange = new Range();
      // java this.tempCaps = new WIntArrayList();
      // java this.mStateMap = new TStateMap(key_max);
      */
    }

    // java public boolean forEachValue(StateEnumProcedure procedure)
    pub fn forEachValue__stateenumprocedure(&mut self, procedure: &StateEnumProcedure) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.forEachValue(procedure, 0, 20);
      return self.forEachValue__stateenumprocedure_int_int(Some(procedure), 0, 20);
      */
    }

    // java public boolean forEachValue(StateEnumProcedure procedure, int keyMin, int keyMax)
    pub fn forEachValue__stateenumprocedure_int_int(&mut self, procedure: &StateEnumProcedure, keyMin: i32, keyMax: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mStateMap.forEachValue(procedure, keyMin, keyMax);
      return self.mStateMap.forEachValue(procedure, keyMin, keyMax);
      */
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (null != this.mCellStates)
      if true {
        // java this.mCellStates.dispose();
        self.mCellStates.dispose();
        // java this.mCellStates = null;
        self.mCellStates = None;
      }
      // java super.dispose();
      self.super_.dispose();
      */
    }

    // java public static StateSnapshot createSnapshotFromTypoMem(int pList, TypoSnapshot snapshot, Pools pools)
    pub fn createSnapshotFromTypoMem__int_typosnapshot_pools(pList: i32, snapshot: &TypoSnapshot, pools: &Pools) -> Option<StateSnapshot> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return snapshot.getTypoType(pList) != 31 ? null : createSnapshotFromTypoMem(0, pList, snapshot, pools);
      return (if snapshot.getTypoType(pList) != 31 { None } else { Some(Self::createSnapshotFromTypoMem__int_int_typosnapshot_pools(0, pList, Some(snapshot), Some(pools))) });
      */
    }

    // java private static StateSnapshot createSnapshotFromTypoMem(int loadOffset, int pList, TypoSnapshot snapshot, Pools pools)
    pub fn createSnapshotFromTypoMem__int_int_typosnapshot_pools(loadOffset: i32, pList: i32, snapshot: &TypoSnapshot, pools: &mut Pools) -> StateSnapshot {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java StateSnapshot stateSnapshot = pools.obtainStateSnapshot();
      let mut stateSnapshot: StateSnapshot = pools.obtainStateSnapshot();
      // java stateSnapshot.loadFromTypoMem(pList, loadOffset, snapshot, pools);
      stateSnapshot.loadFromTypoMem(pList, loadOffset, snapshot, pools);
      // java return stateSnapshot;
      return stateSnapshot;
      */
    }

    // java public void loadFromTypoMem(int pList, TypoSnapshot snapshot, Pools pools)
    pub fn loadFromTypoMem__int_typosnapshot_pools(&mut self, pList: i32, snapshot: &TypoSnapshot, pools: &Pools) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (snapshot.getTypoType(pList) == 31)
      if snapshot.getTypoType(pList) == 31 {
        // java this.loadFromTypoMem(pList, 0, snapshot, pools);
        self.loadFromTypoMem__int_int_typosnapshot_pools(pList, 0, Some(snapshot), Some(pools));
      }
      */
    }

    // java public static boolean hasCellListFromTypoMem(int pList, TypoSnapshot snapshot)
    pub fn hasCellListFromTypoMem(pList: i32, snapshot: Option<TypoSnapshotHolder>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (snapshot.getTypoType(pList) == 31)
      if snapshot.getTypoType(pList) == 31 {
        // java int loadOffset = 0;
        let mut loadOffset: i32 = 0;
        // java int keyCount = TypoStateSnapshot.quickGet(loadOffset++, pList, snapshot);
        let mut keyCount: i32 = TypoStateSnapshot::quickGet({ let _tmp = loadOffset; loadOffset += 1; _tmp }, pList, snapshot);
        let mut i: i32 = 0;
        // java for(int i = 0; i < keyCount; ++i)
        while i < keyCount {
          // java int key = TypoStateSnapshot.quickGet(loadOffset++, pList, snapshot);
          let mut key: i32 = TypoStateSnapshot::quickGet({ let _tmp = loadOffset; loadOffset += 1; _tmp }, pList, snapshot);
          // java if (key == 21)
          if key == 21 {
            // java int cellStateLoadOffset = TypoStateSnapshot.quickGet(loadOffset++, pList, snapshot);
            let mut cellStateLoadOffset: i32 = TypoStateSnapshot::quickGet({ let _tmp = loadOffset; loadOffset += 1; _tmp }, pList, snapshot);
            // java return TypoStateSnapshot.quickGet(cellStateLoadOffset++, pList, snapshot) > 0;
            return TypoStateSnapshot::quickGet({ let _tmp = cellStateLoadOffset; cellStateLoadOffset += 1; _tmp }, pList, snapshot) > 0;
          }
          // java ++loadOffset;
          { let _tmp = loadOffset; loadOffset += 1; _tmp };
        }
      }
      // java return false;
      return false;
      */
    }

    // java public static int getStartCP(int pList, TypoSnapshot snapshot)
    pub fn getStartCP(pList: i32, snapshot: Option<TypoSnapshotHolder>) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (snapshot.getTypoType(pList) == 31)
      if snapshot.getTypoType(pList) == 31 {
        // java int loadOffset = 0;
        let mut loadOffset: i32 = 0;
        // java int keyCount = TypoStateSnapshot.quickGet(loadOffset++, pList, snapshot);
        let mut keyCount: i32 = TypoStateSnapshot::quickGet({ let _tmp = loadOffset; loadOffset += 1; _tmp }, pList, snapshot);
        let mut i: i32 = 0;
        // java for(int i = 0; i < keyCount; ++i)
        while i < keyCount {
          // java int key = TypoStateSnapshot.quickGet(loadOffset++, pList, snapshot);
          let mut key: i32 = TypoStateSnapshot::quickGet({ let _tmp = loadOffset; loadOffset += 1; _tmp }, pList, snapshot);
          // java if (key == 0)
          if key == 0 {
            // java return snapshot.getCPRef(TypoStateSnapshot.quickGet(loadOffset++, pList, snapshot));
            return snapshot.getCPRef(TypoStateSnapshot::quickGet({ let _tmp = loadOffset; loadOffset += 1; _tmp }, pList, snapshot));
          }
          // java ++loadOffset;
          { let _tmp = loadOffset; loadOffset += 1; _tmp };
        }
      }
      // java return -1;
      return -1;
      */
    }

    // java public static boolean isStartAtCP0(int pList, TypoSnapshot snapshot)
    pub fn isStartAtCP0(pList: i32, snapshot: Option<TypoSnapshotHolder>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int startCp = -1;
      let mut startCp: i32 = -1;
      // java if (snapshot.getTypoType(pList) == 31)
      if snapshot.getTypoType(pList) == 31 {
        // java int loadOffset = 0;
        let mut loadOffset: i32 = 0;
        // java int keyCount = TypoStateSnapshot.quickGet(loadOffset++, pList, snapshot);
        let mut keyCount: i32 = TypoStateSnapshot::quickGet({ let _tmp = loadOffset; loadOffset += 1; _tmp }, pList, snapshot);
        let mut i: i32 = 0;
        // java for(int i = 0; i < keyCount; ++i)
        while i < keyCount {
          // java int key = TypoStateSnapshot.quickGet(loadOffset++, pList, snapshot);
          let mut key: i32 = TypoStateSnapshot::quickGet({ let _tmp = loadOffset; loadOffset += 1; _tmp }, pList, snapshot);
          // java if (key == 0)
          if key == 0 {
            // java startCp = snapshot.getCPRef(TypoStateSnapshot.quickGet(loadOffset++, pList, snapshot));
            startCp = snapshot.getCPRef(TypoStateSnapshot::quickGet({ let _tmp = loadOffset; loadOffset += 1; _tmp }, pList, snapshot));
          }
          // java else if (key == 21)
          else if key == 21 {
            // java int cellStateLoadOffset = TypoStateSnapshot.quickGet(loadOffset++, pList, snapshot);
            let mut cellStateLoadOffset: i32 = TypoStateSnapshot::quickGet({ let _tmp = loadOffset; loadOffset += 1; _tmp }, pList, snapshot);
            // java if (TypoStateSnapshot.quickGet(cellStateLoadOffset++, pList, snapshot) > 0)
            if TypoStateSnapshot::quickGet({ let _tmp = cellStateLoadOffset; cellStateLoadOffset += 1; _tmp }, pList, snapshot) > 0 {
              // java return false;
              return false;
            }
          }
          // java else
          else {
            // java ++loadOffset;
            { let _tmp = loadOffset; loadOffset += 1; _tmp };
          }
        }
      }
      // java return startCp == 0;
      return startCp == 0;
      */
    }

    // java public void loadFromTypoMem(int pList, int loadOffset, TypoSnapshot snapshot, Pools pools)
    pub fn loadFromTypoMem__int_int_typosnapshot_pools(&mut self, pList: i32, loadOffset: i32, snapshot: &TypoSnapshot, pools: &Pools) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.clear(pools);
      self.clear(pools);
      // java int keyCount = TypoStateSnapshot.quickGet(loadOffset++, pList, snapshot);
      let mut keyCount: i32 = TypoStateSnapshot::quickGet({ let _tmp = loadOffset; loadOffset += 1; _tmp }, pList, snapshot);
      let mut i: i32 = 0;
      // java for(int i = 0; i < keyCount; ++i)
      while i < keyCount {
        // java int key = TypoStateSnapshot.quickGet(loadOffset++, pList, snapshot);
        let mut key: i32 = TypoStateSnapshot::quickGet({ let _tmp = loadOffset; loadOffset += 1; _tmp }, pList, snapshot);
        // java int value = TypoStateSnapshot.quickGet(loadOffset++, pList, snapshot);
        let mut value: i32 = TypoStateSnapshot::quickGet({ let _tmp = loadOffset; loadOffset += 1; _tmp }, pList, snapshot);
        // java switch (key)
        match key {
          // java case 0: case 16: case 19:
          0 | 16 | 19 => {
            // java this.putInt(key, snapshot.getCPRef(value));
            self.putInt(key, snapshot.getCPRef(value));
          }
          // java case 1: case 2: case 3: case 5: case 6: case 7: case 8: case 9: case 11: case 12: case 13: case 14: case 15: default:
          1 | 2 | 3 | 5 | 6 | 7 | 8 | 9 | 11 | 12 | 13 | 14 | 15 | _ => {
            // java this.putInt(key, value);
            self.putInt(key, value);
          }
          // java case 4:
          4 => {
            // java this.putObject(key, this.createRangeFromTypoMem(value, pList, snapshot));
            self.putObject(key, { let __x: Option<ObjectHolder> = todo!("no align strategy matched"); __x });
          }
          // java case 10: case 17: case 20:
          10 | 17 | 20 => {
            // java this.putObject(key, createSnapshotFromTypoMem(value, pList, snapshot, pools));
            self.putObject(key, { let __x: Option<ObjectHolder> = todo!("no align strategy matched"); __x });
          }
          // java case 18:
          18 => {
            // java this.putObject(key, this.createCPArrayFromTypoMem(value, pList, snapshot));
            self.putObject(key, { let __x: Option<ObjectHolder> = todo!("no align strategy matched"); __x });
          }
          // java case 21:
          21 => {
            // java int cellIndex = 0;
            let mut cellIndex: i32 = 0;
            // java int cellStateLoadOffset = value + 1;
            let mut cellStateLoadOffset: i32 = value + 1;
            let mut cellSize: i32 = TypoStateSnapshot::quickGet(value, pList, snapshot);
            // java for(int cellSize = TypoStateSnapshot.quickGet(value, pList, snapshot); cellIndex < cellSize ++cellIndex)
            while cellIndex < cellSize {
              // java this.mCellStates.add(createSnapshotFromTypoMem(TypoStateSnapshot.quickGet(cellStateLoadOffset++, pList, snapshot), pList, snapshot, pools));
              self.mCellStates.add(Self::createSnapshotFromTypoMem__int_int_typosnapshot_pools(TypoStateSnapshot::quickGet({ let _tmp = cellStateLoadOffset; cellStateLoadOffset += 1; _tmp }, pList, snapshot), pList, Some(snapshot), Some(pools)));
            }
          }
        }
      }
      */
    }

    // java public int saveToTypoMem(TypoSnapshot snapshot)
    pub fn saveToTypoMem(&self, snapshot: Option<TypoSnapshotHolder>) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int list = TypoStateSnapshot.alloc(snapshot, 0);
      let mut list: i32 = TypoStateSnapshot::alloc__typosnapshot_int(snapshot, 0);
      // java this.appendToTypoMem(0, list, snapshot);
      self.appendToTypoMem(0, list, snapshot);
      // java return list;
      return list;
      */
    }

    // java public int appendToTypoMem(int saveOffset, int pList, TypoSnapshot snapshot)
    pub fn appendToTypoMem(&self, saveOffset: i32, pList: i32, snapshot: Option<TypoSnapshotHolder>) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int keyCount = this.mStateMap.count();
      let mut keyCount: i32 = self.mStateMap.count();
      // java if (!this.mCellStates.isEmpty())
      if !self.mCellStates.is_empty() {
        // java ++keyCount;
        { let _tmp = keyCount; keyCount += 1; _tmp };
      }
      // java int extendSize = keyCount * 2 + 1;
      let mut extendSize: i32 = keyCount * 2 + 1;
      // java int nextSaveOffset = saveOffset + extendSize;
      let mut nextSaveOffset: i32 = saveOffset + extendSize;
      // java if (!TypoStateSnapshot.extendSelfLen(extendSize, pList, snapshot))
      if !TypoStateSnapshot::extendSelfLen(extendSize, pList, snapshot) {
        // java throw new RuntimeException("extend TypList error!!");
        panic!("{}", RuntimeException::new("extend TypList error!!"));
      }
      // java else
      else {
        // java TypoStateSnapshot.quickSet(keyCount, saveOffset++, pList, snapshot);
        TypoStateSnapshot::quickSet(keyCount, { let _tmp = saveOffset; saveOffset += 1; _tmp }, pList, snapshot);
        // java int subDocType = this.mStateMap.getInt(3, 0);
        let mut subDocType: i32 = self.mStateMap.getInt(3, 0);
        let mut i: i32 = 0;
        // java for(int i = 0; i < 21; ++i)
        while i < 21 {
          // java if (this.mStateMap.hasKey(i))
          if self.mStateMap.hasKey(i) {
            // java TypoStateSnapshot.quickSet(i, saveOffset++, pList, snapshot);
            TypoStateSnapshot::quickSet(i, { let _tmp = saveOffset; saveOffset += 1; _tmp }, pList, snapshot);
            // java switch (i)
            match i {
              // java case 0: case 16: case 19:
              0 | 16 | 19 => {
                // java int value = snapshot.addRefCP(subDocType, this.mStateMap.getInt(i, 0));
                let mut value: i32 = snapshot.addRefCP(subDocType, self.mStateMap.getInt(i, 0));
                // java TypoStateSnapshot.quickSet(value, saveOffset++, pList, snapshot);
                TypoStateSnapshot::quickSet(value, { let _tmp = saveOffset; saveOffset += 1; _tmp }, pList, snapshot);
              }
              // java case 1: case 2: case 3: case 5: case 6: case 7: case 8: case 9: case 11: case 12: case 13: case 14: case 15: default:
              1 | 2 | 3 | 5 | 6 | 7 | 8 | 9 | 11 | 12 | 13 | 14 | 15 | _ => {
                // java TypoStateSnapshot.quickSet(this.mStateMap.getInt(i, 0), saveOffset++, pList, snapshot);
                TypoStateSnapshot::quickSet(self.mStateMap.getInt(i, 0), { let _tmp = saveOffset; saveOffset += 1; _tmp }, pList, snapshot);
              }
              // java case 4:
              4 => {
                // java Range range = (Range)this.getObject(i);
                let mut range: Box<dyn Range> = { let __x: Box<Box<dyn Range>> = todo!("no align strategy matched"); __x };
                // java this.tempCaps.resetQuick();
                self.tempCaps.resetQuick();
                // java this.tempCaps.add(RANGEFactory.getStart(range.range));
                self.tempCaps.add__int(RANGEFactory::getStart(range.range));
                // java this.tempCaps.add(RANGEFactory.getEnd(range.range));
                self.tempCaps.add__int(RANGEFactory::getEnd(range.range));
                // java TypoStateSnapshot.quickSet(nextSaveOffset, saveOffset++, pList, snapshot);
                TypoStateSnapshot::quickSet(nextSaveOffset, { let _tmp = saveOffset; saveOffset += 1; _tmp }, pList, snapshot);
                // java nextSaveOffset = this.appendCPList(this.tempCaps, subDocType, nextSaveOffset, pList, snapshot);
                nextSaveOffset = self.appendCPList({ let __x: Option<IIntListHolder> = todo!("no align strategy matched"); __x }, subDocType, nextSaveOffset, pList, snapshot);
              }
              // java case 10: case 17: case 20:
              10 | 17 | 20 => {
                // java StateSnapshot snapshot1 = (StateSnapshot)this.getObject(i);
                let mut snapshot1: StateSnapshot = (self.getObject(i)).downcast_ref::<StateSnapshot>().unwrap();
                // java TypoStateSnapshot.quickSet(nextSaveOffset, saveOffset++, pList, snapshot);
                TypoStateSnapshot::quickSet(nextSaveOffset, { let _tmp = saveOffset; saveOffset += 1; _tmp }, pList, snapshot);
                // java nextSaveOffset = snapshot1.appendToTypoMem(nextSaveOffset, pList, snapshot);
                nextSaveOffset = snapshot1.appendToTypoMem(nextSaveOffset, pList, snapshot);
              }
              // java case 18:
              18 => {
                // java IIntList array = (WIntArrayList)this.getObject(i);
                let mut array: Box<dyn IIntList> = { let __x: Box<Box<dyn IIntList>> = todo!("no align strategy matched"); __x };
                // java TypoStateSnapshot.quickSet(nextSaveOffset, saveOffset++, pList, snapshot);
                TypoStateSnapshot::quickSet(nextSaveOffset, { let _tmp = saveOffset; saveOffset += 1; _tmp }, pList, snapshot);
                // java nextSaveOffset = this.appendCPList(array, subDocType, nextSaveOffset, pList, snapshot);
                nextSaveOffset = self.appendCPList({ let __x: Option<IIntListHolder> = todo!("no align strategy matched"); __x }, subDocType, nextSaveOffset, pList, snapshot);
              }
            }
          }
        }
        // java if (!this.mCellStates.isEmpty())
        if !self.mCellStates.is_empty() {
          // java TypoStateSnapshot.quickSet(21, saveOffset++, pList, snapshot);
          TypoStateSnapshot::quickSet(21, { let _tmp = saveOffset; saveOffset += 1; _tmp }, pList, snapshot);
          // java TypoStateSnapshot.quickSet(nextSaveOffset, saveOffset++, pList, snapshot);
          TypoStateSnapshot::quickSet(nextSaveOffset, { let _tmp = saveOffset; saveOffset += 1; _tmp }, pList, snapshot);
          // java nextSaveOffset = this.appendCellStates(nextSaveOffset, pList, snapshot);
          nextSaveOffset = self.appendCellStates(nextSaveOffset, pList, snapshot);
        }
        // java return nextSaveOffset;
        return nextSaveOffset;
      }
      */
    }

    // java private int appendCPList(IIntList array, int subDocType, int saveOffset, int list, TypoSnapshot snapshot)
    pub fn appendCPList(&self, array: Option<IIntListHolder>, subDocType: i32, saveOffset: i32, list: i32, snapshot: Option<TypoSnapshotHolder>) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int cpCount = array.size();
      let mut cpCount: i32 = (array.as_ref().map(|__a| __a.len()).unwrap_or(0) as i32);
      // java int extendSize = cpCount + 1;
      let mut extendSize: i32 = cpCount + 1;
      // java if (!TypoStateSnapshot.extendSelfLen(extendSize, list, snapshot))
      if !TypoStateSnapshot::extendSelfLen(extendSize, list, snapshot) {
        // java throw new RuntimeException("extend TypList error!!");
        panic!("{}", RuntimeException::new("extend TypList error!!"));
      }
      // java else
      else {
        // java int nextSaveOffset = saveOffset + extendSize;
        let mut nextSaveOffset: i32 = saveOffset + extendSize;
        // java TypoStateSnapshot.quickSet(cpCount, saveOffset++, list, snapshot);
        TypoStateSnapshot::quickSet(cpCount, { let _tmp = saveOffset; saveOffset += 1; _tmp }, list, snapshot);
        // java int i = 0;
        let mut i: i32 = 0;
        let mut size: i32 = cpCount;
        // java for(int size = cpCount; i < size; ++i)
        while i < size {
          // java TypoStateSnapshot.quickSet(snapshot.addRefCP(subDocType, array.get(i)), saveOffset++, list, snapshot);
          TypoStateSnapshot::quickSet(snapshot.addRefCP(subDocType, array.get(i)), { let _tmp = saveOffset; saveOffset += 1; _tmp }, list, snapshot);
        }
        // java return nextSaveOffset;
        return nextSaveOffset;
      }
      */
    }

    // java private int appendCellStates(int saveOffset, int list, TypoSnapshot snapshot)
    pub fn appendCellStates(&self, saveOffset: i32, list: i32, snapshot: Option<TypoSnapshotHolder>) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int extendSize = this.mCellStates.size() + 1;
      let mut extendSize: i32 = (self.mCellStates.len() as i32) + 1;
      // java if (!TypoStateSnapshot.extendSelfLen(extendSize, list, snapshot))
      if !TypoStateSnapshot::extendSelfLen(extendSize, list, snapshot) {
        // java throw new RuntimeException("extend TypList error!!");
        panic!("{}", RuntimeException::new("extend TypList error!!"));
      }
      // java else
      else {
        // java int nextSaveOffset = saveOffset + extendSize;
        let mut nextSaveOffset: i32 = saveOffset + extendSize;
        // java TypoStateSnapshot.quickSet(this.mCellStates.size(), saveOffset++, list, snapshot);
        TypoStateSnapshot::quickSet((self.mCellStates.len() as i32), { let _tmp = saveOffset; saveOffset += 1; _tmp }, list, snapshot);
        // java StateSnapshot cellSnapShot = null;
        let mut cellSnapShot: StateSnapshot = Default::default();
        // java KList.Iterator<StateSnapshot> it = this.mCellStates.iterator();
        let mut it: Vec<StateSnapshot> = self.mCellStates.iterator();
        // java it.seekHead();
        it.seekHead();
        // java while ((cellSnapShot = (StateSnapshot)it.next2()) != null)
        while { cellSnapShot = (it.next2()).downcast_ref::<StateSnapshot>().unwrap(); cellSnapShot }.is_some() {
          // java TypoStateSnapshot.quickSet(nextSaveOffset, saveOffset++, list, snapshot);
          TypoStateSnapshot::quickSet(nextSaveOffset, { let _tmp = saveOffset; saveOffset += 1; _tmp }, list, snapshot);
          // java nextSaveOffset = cellSnapShot.appendToTypoMem(nextSaveOffset, list, snapshot);
          nextSaveOffset = cellSnapShot.appendToTypoMem(nextSaveOffset, list, snapshot);
        }
        // java it.recycle();
        it.recycle();
        // java return nextSaveOffset;
        return nextSaveOffset;
      }
      */
    }

    // java private Range createRangeFromTypoMem(int loadOffset, int pList, TypoSnapshot snapshot)
    pub fn createRangeFromTypoMem(&self, loadOffset: i32, pList: i32, snapshot: Option<TypoSnapshotHolder>) -> Option<RangeHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int size = TypoStateSnapshot.quickGet(loadOffset++, pList, snapshot);
      let mut size: i32 = TypoStateSnapshot::quickGet({ let _tmp = loadOffset; loadOffset += 1; _tmp }, pList, snapshot);
      // java if (size != 2)
      if size != 2 {
        // java throw new RuntimeException("error range from mem!!!");
        panic!("{}", RuntimeException::new("error range from mem!!!"));
      }
      // java else
      else {
        // java int start = snapshot.getCPRef(TypoStateSnapshot.quickGet(loadOffset++, pList, snapshot));
        let mut start: i32 = snapshot.getCPRef(TypoStateSnapshot::quickGet({ let _tmp = loadOffset; loadOffset += 1; _tmp }, pList, snapshot));
        // java int end = snapshot.getCPRef(TypoStateSnapshot.quickGet(loadOffset++, pList, snapshot));
        let mut end: i32 = snapshot.getCPRef(TypoStateSnapshot::quickGet({ let _tmp = loadOffset; loadOffset += 1; _tmp }, pList, snapshot));
        // java return start >= 0 && start <= end && end >= 0 ? this.reuseRangeInstance(RANGEFactory.getRANGE(start, end)) : this.reuseRangeInstance(0L);
        return Some((if start >= 0 && start <= end && end >= 0 { Some(self.reuseRangeInstance(RANGEFactory::getRANGE(start, end))) } else { Some(self.reuseRangeInstance((0 as i32))) }));
      }
      */
    }

    // java private IIntList createCPArrayFromTypoMem(int loadOffset, int pList, TypoSnapshot snapshot)
    pub fn createCPArrayFromTypoMem(&self, loadOffset: i32, pList: i32, snapshot: Option<TypoSnapshotHolder>) -> Option<IIntListHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int size = TypoStateSnapshot.quickGet(loadOffset++, pList, snapshot);
      let mut size: i32 = TypoStateSnapshot::quickGet({ let _tmp = loadOffset; loadOffset += 1; _tmp }, pList, snapshot);
      // java IIntList list = new WIntArrayList(size);
      let mut list: Box<dyn IIntList> = { let __x: Box<Box<dyn IIntList>> = todo!("no align strategy matched"); __x };
      let mut i: i32 = 0;
      // java for(int i = 0; i < size; ++i)
      while i < size {
        // java list.add(snapshot.getCPRef(TypoStateSnapshot.quickGet(loadOffset++, pList, snapshot)));
        list.add(snapshot.getCPRef(TypoStateSnapshot::quickGet({ let _tmp = loadOffset; loadOffset += 1; _tmp }, pList, snapshot)));
      }
      // java return list;
      return { let __x: Option<IIntListHolder> = todo!("no align strategy matched"); __x };
      */
    }

    // java public void clear(Pools pools)
    pub fn clear(&mut self, pools: &mut Pools) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java StateSnapshot firstMergeCell = (StateSnapshot)this.getObject(10);
      let mut firstMergeCell: StateSnapshot = (self.getObject(10)).downcast_ref::<StateSnapshot>().unwrap();
      // java if (null != firstMergeCell)
      if true {
        // java pools.recycle(firstMergeCell);
        pools.recycle__bidiitem(firstMergeCell);
      }
      // java this.mStateMap.clear();
      self.mStateMap.clear();
      // java if (!this.mCellStates.isEmpty())
      if !self.mCellStates.is_empty() {
        // java pools.recycleStatesSnapshot(this.mCellStates);
        pools.recycleStatesSnapshot(self.mCellStates);
      }
      */
    }

    // java public final boolean contains(int key)
    pub fn contains(&self, key: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mStateMap.hasKey(key);
      return self.mStateMap.hasKey(key);
      */
    }

    // java public final int count()
    pub fn count(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mStateMap.count();
      return self.mStateMap.count();
      */
    }

    // java public final int getInt(int key, int defValue)
    pub fn getInt(&self, key: i32, defValue: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mStateMap.getInt(key, defValue);
      return self.mStateMap.getInt(key, defValue);
      */
    }

    // java public final void putInt(int key, int value)
    pub fn putInt(&mut self, key: i32, value: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mStateMap.putInt(key, value);
      self.mStateMap.putInt(key, value);
      */
    }

    // java public final Object getObject(int key)
    pub fn getObject(&self, key: i32) -> Box<dyn Object> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mStateMap.getObject(key);
      return self.mStateMap.getObject(key);
      */
    }

    // java public final void putObject(int key, Object obj)
    pub fn putObject(&mut self, key: i32, obj: &dyn Object) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mStateMap.putObject(key, obj);
      self.mStateMap.putObject(key, obj);
      */
    }

    // java public final boolean getBool(int key, boolean defValue)
    pub fn getBool(&self, key: i32, defValue: bool) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mStateMap.getInt(key, defValue ? 1 : 0) == 1;
      return self.mStateMap.getInt(key, (if defValue { 1 } else { 0 })) == 1;
      */
    }

    // java public final void putBool(int key, boolean value)
    pub fn putBool(&self, key: i32, value: bool) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mStateMap.putInt(key, value ? 1 : 0);
      self.mStateMap.putInt(key, (if value { 1 } else { 0 }));
      */
    }

    // java public int getStartCp()
    pub fn getStartCp(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.getInt(0, -1);
      return self.getInt(0, -1);
      */
    }

    // java public void setStartCp(int startCp)
    pub fn setStartCp(&mut self, startCp: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.putInt(0, startCp);
      self.putInt(0, startCp);
      */
    }

    // java public StateSnapshot getFirstMergeCell()
    pub fn getFirstMergeCell(&self) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return (StateSnapshot)this.getObject(10);
      return { let __x: Self = todo!("no align strategy matched"); __x };
      */
    }

    // java public void setFirstMergeCell(StateSnapshot firstMergeCellState)
    pub fn setFirstMergeCell(&mut self, firstMergeCellState: Option<StateSnapshotHolder>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java Assert.assertNull("mFirstMergeCell should be null.", this.getFirstMergeCell());
      Assert::assertNull__string_object("mFirstMergeCell should be null.", self.getFirstMergeCell());
      // java this.putObject(10, firstMergeCellState);
      self.putObject(10, { let __x: Option<ObjectHolder> = todo!("no align strategy matched"); __x });
      */
    }

    // java public KList<StateSnapshot> getCellStates()
    pub fn getCellStates(&self) -> Vec<Option<StateSnapshotHolder>> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mCellStates;
      return self.mCellStates;
      */
    }

    // java private static boolean equalsCellState(StateSnapshot cell0, StateSnapshot cell1)
    pub fn equalsCellState(cell0: Option<StateSnapshotHolder>, cell1: Option<StateSnapshotHolder>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (!equalsPageState(cell0, cell1))
      if !Self::equalsPageState(cell0, cell1) {
        // java return false;
        return false;
      }
      // java else
      else {
        // java StateSnapshot firstMergeCell0 = cell0.getFirstMergeCell();
        let mut firstMergeCell0: StateSnapshot = { let __x: StateSnapshot = todo!("no align strategy matched"); __x };
        // java StateSnapshot firstMergeCell1 = cell1.getFirstMergeCell();
        let mut firstMergeCell1: StateSnapshot = { let __x: StateSnapshot = todo!("no align strategy matched"); __x };
        // java if (firstMergeCell0 == null)
        if false {
          // java return firstMergeCell1 == null;
          return false;
        }
        // java else
        else {
          // java return firstMergeCell1 == null ? false : equalsCellState(firstMergeCell0, firstMergeCell1);
          return (if false { false } else { Self::equalsCellState({ let __x: Option<StateSnapshotHolder> = todo!("no align strategy matched"); __x }, { let __x: Option<StateSnapshotHolder> = todo!("no align strategy matched"); __x }) });
        }
      }
      */
    }

    // java private static boolean equalsPageState(StateSnapshot stateSnapshot0, StateSnapshot stateSnapshot1)
    pub fn equalsPageState(stateSnapshot0: Option<StateSnapshotHolder>, stateSnapshot1: Option<StateSnapshotHolder>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (stateSnapshot0.getStartCp() != stateSnapshot1.getStartCp())
      if stateSnapshot0.getStartCp() != stateSnapshot1.getStartCp() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java KList<StateSnapshot> cellState0 = stateSnapshot0.getCellStates();
        let mut cellState0: KList = { let __x: KList = todo!("no align strategy matched"); __x };
        // java KList<StateSnapshot> cellState1 = stateSnapshot1.getCellStates();
        let mut cellState1: KList = { let __x: KList = todo!("no align strategy matched"); __x };
        // java if (cellState0.isEmpty())
        if cellState0.is_empty() {
          // java return cellState1.isEmpty();
          return cellState1.is_empty();
        }
        // java else if (cellState1.isEmpty())
        else if cellState1.is_empty() {
          // java return false;
          return false;
        }
        // java else
        else {
          // java KList.Iterator<StateSnapshot> it0 = cellState0.iterator();
          let mut it0: Vec<StateSnapshot> = cellState0.iterator();
          // java KList.Iterator<StateSnapshot> it1 = cellState1.iterator();
          let mut it1: Vec<StateSnapshot> = cellState1.iterator();
          // java StateSnapshot cell0 = (StateSnapshot)it0.next();
          let mut cell0: StateSnapshot = (it0.next()).downcast_ref::<StateSnapshot>().unwrap();
          // java StateSnapshot cell1 = (StateSnapshot)it1.next();
          let mut cell1: StateSnapshot = (it1.next()).downcast_ref::<StateSnapshot>().unwrap();
          // java boolean rs;
          let mut rs: bool;
          rs = true;
          // java for(rs = true cell0 != null && cell1 != null cell1 = (StateSnapshot)it1.next())
          while true && true {
            // java if (!equalsCellState(cell0, cell1))
            if !Self::equalsCellState({ let __x: Option<StateSnapshotHolder> = todo!("no align strategy matched"); __x }, { let __x: Option<StateSnapshotHolder> = todo!("no align strategy matched"); __x }) {
              // java rs = false;
              rs = false;
              // java break;
              break;
            }
            // java cell0 = (StateSnapshot)it0.next();
            cell0 = (it0.next()).downcast_ref::<StateSnapshot>().unwrap();
          }
          // java if (cell0 != null || cell1 != null)
          if true || true {
            // java rs = false;
            rs = false;
          }
          // java it0.recycle();
          it0.recycle();
          // java it1.recycle();
          it1.recycle();
          // java return rs;
          return rs;
        }
      }
      */
    }

    // java public static boolean equalsStateSnapshot(StateSnapshot stateSnapshot0, StateSnapshot stateSnapshot1)
    pub fn equalsStateSnapshot(stateSnapshot0: Option<StateSnapshotHolder>, stateSnapshot1: Option<StateSnapshotHolder>) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (stateSnapshot0 == stateSnapshot1)
      if stateSnapshot0 == stateSnapshot1 {
        // java return true;
        return true;
      }
      // java else if (!equalsPageState(stateSnapshot0, stateSnapshot1))
      else if !Self::equalsPageState(stateSnapshot0, stateSnapshot1) {
        // java return false;
        return false;
      }
      // java else
      else {
        // java int curFootNoteRefCP = stateSnapshot0.getInt(16, -1);
        let mut curFootNoteRefCP: i32 = stateSnapshot0.getInt(16, -1);
        // java if (curFootNoteRefCP >= 0)
        if curFootNoteRefCP >= 0 {
          // java if (stateSnapshot1.getInt(16, -1) != curFootNoteRefCP)
          if stateSnapshot1.getInt(16, -1) != curFootNoteRefCP {
            // java return false;
            return false;
          }
          // java if (!equalsPageState((StateSnapshot)stateSnapshot0.getObject(17), (StateSnapshot)stateSnapshot1.getObject(17)))
          if !Self::equalsPageState({ let __x: Option<StateSnapshotHolder> = todo!("no align strategy matched"); __x }, { let __x: Option<StateSnapshotHolder> = todo!("no align strategy matched"); __x }) {
            // java return false;
            return false;
          }
          // java IIntList cps0 = (IIntList)stateSnapshot0.getObject(18);
          let mut cps0: Box<dyn IIntList> = { let __x: Box<Box<dyn IIntList>> = todo!("no align strategy matched"); __x };
          // java IIntList cps1 = (IIntList)stateSnapshot1.getObject(18);
          let mut cps1: Box<dyn IIntList> = { let __x: Box<Box<dyn IIntList>> = todo!("no align strategy matched"); __x };
          // java if (cps0 != null)
          if true {
            // java if (cps1 == null)
            if false {
              // java return false;
              return false;
            }
            // java if (cps0.size() != cps1.size())
            if (cps0.len() as i32) != (cps1.len() as i32) {
              // java return false;
              return false;
            }
            // java int i = 0;
            let mut i: i32 = 0;
            let mut size: i32 = (cps0.len() as i32);
            // java for(int size = cps0.size(); i < size; ++i)
            while i < size {
              // java if (cps0.get(i) != cps1.get(i))
              if cps0.get(i) != cps1.get(i) {
                // java return false;
                return false;
              }
            }
          }
          // java else if (cps1 != null)
          else if true {
            // java return false;
            return false;
          }
        }
        // java else if (stateSnapshot1.getInt(16, -1) >= 0)
        else if stateSnapshot1.getInt(16, -1) >= 0 {
          // java return false;
          return false;
        }
        // java int curEndNoteRefCP = stateSnapshot0.getInt(16, -1);
        let mut curEndNoteRefCP: i32 = stateSnapshot0.getInt(16, -1);
        // java if (curEndNoteRefCP >= 0)
        if curEndNoteRefCP >= 0 {
          // java if (stateSnapshot1.getInt(16, -1) != curEndNoteRefCP)
          if stateSnapshot1.getInt(16, -1) != curEndNoteRefCP {
            // java return false;
            return false;
          }
          // java if (!equalsPageState((StateSnapshot)stateSnapshot0.getObject(20), (StateSnapshot)stateSnapshot1.getObject(20)))
          if !Self::equalsPageState({ let __x: Option<StateSnapshotHolder> = todo!("no align strategy matched"); __x }, { let __x: Option<StateSnapshotHolder> = todo!("no align strategy matched"); __x }) {
            // java return false;
            return false;
          }
        }
        // java else if (stateSnapshot1.getInt(16, -1) >= 0)
        else if stateSnapshot1.getInt(16, -1) >= 0 {
          // java return false;
          return false;
        }
        // java return true;
        return true;
      }
      */
    }

}

impl std::ops::Deref for StateSnapshot {
    type Target = RecycableNode;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for StateSnapshot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java public static class Range
impl Range {
}

// java protected static class Alloc extends KList.RecycableNodeAlloc<StateSnapshot>
impl Alloc {
    // java protected  Alloc(Pools pools)
    pub fn Alloc(&self, pools: Option<PoolsHolder>) -> Option<AllocHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mPools = pools;
      Self { super_: RecycableNodeAlloc<StateSnapshot>::new(), mPools: pools }
      */
    }

    // java public StateSnapshot alloc()
    pub fn alloc(&self) -> Self {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return new StateSnapshot();
      return { let __x: Self = todo!("no align strategy matched"); __x };
      */
    }

    // java public void init(StateSnapshot state)
    pub fn init(&mut self, state: Option<StateSnapshotHolder>) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java super.init(state);
      self.super_.init(state);
      // java state.clear(this.mPools);
      state.clear(self.mPools);
      */
    }

}

impl std::ops::Deref for Alloc {
    type Target = RecycableNodeAlloc<StateSnapshot>;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for Alloc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

// java static class TStateMap
impl TStateMap {
    // java   TStateMap(int keyMax)
    pub fn TStateMap(&self, keyMax: i32) -> Option<TStateMapHolder> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mKeyMax = keyMax; this.mIntValues = new int[keyMax + 1]; this.mObjectValues = new WIntObjectHashMap(4);
      this.mKeyMax = keyMax;
      // java throw new RuntimeException(" Error range for keyMax(0..31):" + keyMax);
      panic!("{}", RuntimeException::new(" Error range for keyMax(0..31):" + keyMax));
      */
    }

    // java  void clear()
    pub fn clear(&mut self) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java this.mBitsIntkeys = 0;
      self.mBitsIntkeys = 0;
      // java this.mBitsObjetKeys = 0;
      self.mBitsObjetKeys = 0;
      // java this.mObjectValues.clear();
      self.mObjectValues.clear();
      */
    }

    // java public boolean forEachValue(StateEnumProcedure procedure, int keyMin, int keyMax)
    pub fn forEachValue(&self, procedure: &mut StateEnumProcedure, keyMin: i32, keyMax: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java keyMin = Math.max(keyMin, 0);
      keyMin = (keyMin).max(0);
      // java keyMax = Math.min(keyMax, this.mKeyMax);
      keyMax = (keyMax).min(self.mKeyMax);
      let mut i: i32 = keyMin;
      // java for(int i = keyMin; i <= keyMax; ++i)
      while i <= keyMax {
        // java int mask = 1 << i;
        let mut mask: i32 = 1 << i;
        // java if ((this.mBitsIntkeys & mask) == mask && !procedure.enumInt(i, this.mIntValues[i]))
        if self.mBitsIntkeys & mask == mask && !procedure.enumInt(i, self.mIntValues[(i) as usize]) {
          // java return false;
          return false;
        }
        // java if ((this.mBitsObjetKeys & mask) == mask && !procedure.enumObject(i, this.mObjectValues.get(i)))
        if self.mBitsObjetKeys & mask == mask && !procedure.enumObject(i, self.mObjectValues.get(i)) {
          // java return false;
          return false;
        }
      }
      // java return true;
      return true;
      */
    }

    // java  void changeKeyMax(int keyMax)
    pub fn changeKeyMax(&self, keyMax: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (keyMax > this.mKeyMax)
      if keyMax > self.mKeyMax {
        // java if (keyMax > 31)
        if keyMax > 31 {
          // java throw new RuntimeException("grow to max!!! key " + keyMax);
          panic!("{}", RuntimeException::new("grow to max!!! key " + keyMax));
        }
        // java if (keyMax >= this.mIntValues.length)
        if keyMax >= (self.mIntValues.len() as i32) {
          // java int[] newValues = new int[keyMax + 1];
          let mut newValues: Vec<i32> = vec![Default::default(); (keyMax + 1) as usize];
          // java System.arraycopy(this.mIntValues, 0, newValues, 0, this.mIntValues.length);
          System::arraycopy(self.mIntValues, 0, newValues, 0, (self.mIntValues.len() as i32));
          // java this.mIntValues = newValues;
          self.mIntValues = newValues;
        }
        // java int mask = -1 >>> 31 - this.mKeyMax;
        let mut mask: i32 = -1 >> 31 - self.mKeyMax;
        // java this.mBitsIntkeys &= mask;
        self.mBitsIntkeys = self.mBitsIntkeys & mask;
        // java this.mBitsObjetKeys &= mask;
        self.mBitsObjetKeys = self.mBitsObjetKeys & mask;
      }
      // java this.mKeyMax = keyMax;
      self.mKeyMax = keyMax;
      */
    }

    // java final int count()
    pub fn count(&self) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java int count = 0;
      let mut count: i32 = 0;
      let mut i: i32 = 0;
      // java for(int i = 0; i <= this.mKeyMax; ++i)
      while i <= self.mKeyMax {
        // java int mask = 1 << i;
        let mut mask: i32 = 1 << i;
        // java if ((this.mBitsIntkeys & mask) == mask || (this.mBitsObjetKeys & mask) == mask)
        if self.mBitsIntkeys & mask == mask || self.mBitsObjetKeys & mask == mask {
          // java ++count;
          { let _tmp = count; count += 1; _tmp };
        }
      }
      // java return count;
      return count;
      */
    }

    // java final boolean hasKey(int key)
    pub fn hasKey(&self, key: i32) -> bool {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (key >= 0 && key <= this.mKeyMax)
      if key >= 0 && key <= self.mKeyMax {
        // java int mask = 1 << key;
        let mut mask: i32 = 1 << key;
        // java return (this.mBitsIntkeys & mask) == mask || (this.mBitsObjetKeys & mask) == mask;
        return self.mBitsIntkeys & mask == mask || self.mBitsObjetKeys & mask == mask;
      }
      // java else
      else {
        // java return false;
        return false;
      }
      */
    }

    // java final int getInt(int key, int defValue)
    pub fn getInt(&self, key: i32, defValue: i32) -> i32 {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (key >= 0 && key <= this.mKeyMax)
      if key >= 0 && key <= self.mKeyMax {
        // java int mask = 1 << key;
        let mut mask: i32 = 1 << key;
        // java if ((this.mBitsIntkeys & mask) == mask)
        if self.mBitsIntkeys & mask == mask {
          // java return this.mIntValues[key];
          return self.mIntValues[(key) as usize];
        }
      }
      // java return defValue;
      return defValue;
      */
    }

    // java final void putInt(int key, int value)
    pub fn putInt(&mut self, key: i32, value: i32) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (key > this.mKeyMax)
      if key > self.mKeyMax {
        // java this.changeKeyMax(key);
        self.changeKeyMax(key);
      }
      // java this.mBitsIntkeys |= 1 << key;
      self.mBitsIntkeys = self.mBitsIntkeys | 1 << key;
      // java this.mIntValues[key] = value;
      self.mIntValues[(key) as usize] = value;
      */
    }

    // java final Object getObject(int key)
    pub fn getObject(&self, key: i32) -> Box<dyn Object> {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java return this.mObjectValues.get(key);
      return self.mObjectValues.get(key);
      */
    }

    // java final void putObject(int key, Object obj)
    pub fn putObject(&mut self, key: i32, obj: &dyn Object) {
      todo!("DEFERRED_NON_FUNCTION");
      /* DEFERRED_NON_FUNCTION
      // java if (key > this.mKeyMax)
      if key > self.mKeyMax {
        // java this.changeKeyMax(key);
        self.changeKeyMax(key);
      }
      // java this.mBitsObjetKeys |= 1 << key;
      self.mBitsObjetKeys = self.mBitsObjetKeys | 1 << key;
      // java this.mObjectValues.put(key, obj);
      self.mObjectValues.put(key, obj);
      */
    }

}
