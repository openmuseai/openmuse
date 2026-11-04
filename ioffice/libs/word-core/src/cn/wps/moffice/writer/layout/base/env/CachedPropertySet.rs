#![allow(dead_code, unused_variables, unused_imports, non_snake_case, unused_mut, unused_assignments, unreachable_code)]
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::PropertySet;
// Java: import cn.wps.moffice.writer.data.IChpxTable
use crate::cn::wps::moffice::writer::data::IChpxTable::{IChpxTable, Entry as ChpxEntry};
// Java: import cn.wps.moffice.writer.data.IPapxTable
use crate::cn::wps::moffice::writer::data::IPapxTable::{IPapxTable, Entry as PapxEntry};

// java final class CachedPropertySet extends PropertySet implements Cloneable
// rust: wr-29 GR-D3 —— Java 继承 PropertySet；Rust 组合：property + para/run mark。
//       cacheID/mark 位运算逐行对齐 CachedPropertySet.java:1-193。
pub struct CachedPropertySet {
    // java PropertySet super_（组合）
    pub(crate) property: PropertySet,
    // java private long mParaMark = -1L;
    pub(crate) mParaMark: i64,
    // java private long mRunMark = -1L;
    pub(crate) mRunMark: i64,
}

// java public static final long INVALID_RUN_CACHE_ID = -1L;
pub const INVALID_RUN_CACHE_ID: i64 = -1;
// java private static final long INVALID_MARK = -1L;
const INVALID_MARK: i64 = -1;
// java private static final long LONG_BIT_REVISION_FINAL = 4611686018427387904L;
pub const LONG_BIT_REVISION_FINAL: i64 = 4611686018427387904;
// java private static final long LONG_BIT_MERGED_FROM_STYLE = 2305843009213693952L;
const LONG_BIT_MERGED_FROM_STYLE: i64 = 2305843009213693952;
// java private static final int LONG_OFFSET_EXTRA_ID = 29;
const LONG_OFFSET_EXTRA_ID: i32 = 29;
// java private static final int LONG_OFFSET_PSID = 0;
const LONG_OFFSET_PSID: i32 = 0;
// java private static final long LONG_MASK_PSID = 536870911L;
const LONG_MASK_PSID: i64 = 536870911;

// java final class CachedPropertySet extends PropertySet implements Cloneable
impl CachedPropertySet {
    // java public  CachedPropertySet(int[] ids, Object[] values, int n)
    pub fn CachedPropertySet(property: PropertySet) -> Self {
      // java this._freeze();
      Self { property, mParaMark: -1, mRunMark: -1 }
    }

    // java public void setParaMark(IPapxTable.Entry para, int tableStyleIdUseToPara, boolean isRevFinal)
    pub fn setParaMark(&mut self, para: &dyn PapxEntry, tableStyleIdUseToPara: i32, isRevFinal: bool) {
      // java this.mRunMark = -1L;
      self.mRunMark = -1;
      // java this.mParaMark = getMark(para.getProperty(), para.getPropertySetId(), tableStyleIdUseToPara, isRevFinal);
      let paraProp = PapxEntry::getProperty(para);
      self.mParaMark = Self::getMark(paraProp, para.getPropertySetId(), tableStyleIdUseToPara, isRevFinal);
    }

    // java public void setRunMark(CachedPropertySet paraProp, IChpxTable.Entry run, boolean revFinal)
    pub fn setRunMark(&mut self, paraProp: &CachedPropertySet, run: &dyn ChpxEntry, revFinal: bool) {
      // java long paraMark = paraProp.mParaMark;
      let paraMark: i64 = paraProp.mParaMark;
      // java if (revFinal != getFlagBit(paraMark, 4611686018427387904L))
      if revFinal != getFlagBit(paraMark, LONG_BIT_REVISION_FINAL) {
        // java this.mParaMark = -1L;
        self.mParaMark = -1;
        // java this.mRunMark = -1L;
        self.mRunMark = -1;
      }
      // java else
      else {
        // java this.mRunMark = getMark(run.getProperty(), run.getPropertySetId(), 4095, revFinal);
        let runProp = ChpxEntry::getProperty(run);
        self.mRunMark = Self::getMark(runProp, run.getPropertySetId(), 4095, revFinal);
        // java this.mParaMark = paraMark;
        self.mParaMark = paraMark;
      }
    }

    // java public boolean isMergedFromStyle()
    pub fn isMergedFromStyle(&self) -> bool {
      // java if (-1L != this.mRunMark && getFlagBit(this.mRunMark, 2305843009213693952L))
      if -1 != self.mRunMark && getFlagBit(self.mRunMark, LONG_BIT_MERGED_FROM_STYLE) {
        // java return true;
        return true;
      }
      // java else
      else {
        // java return getFlagBit(this.mParaMark, 2305843009213693952L);
        return getFlagBit(self.mParaMark, LONG_BIT_MERGED_FROM_STYLE);
      }
    }

    // java private static boolean hasStyles(PropertySet prop)
    pub fn hasStyles(prop: Option<&PropertySet>) -> bool {
      // java return prop.contains(191) || prop.contains(2) || prop.contains(292);
      return prop.map(|p| p.contains(191) || p.contains(2) || p.contains(292)).unwrap_or(false);
    }

    // java private static long getMark(PropertySet prop, int psid, int extraId, boolean isRevFinal)
    pub fn getMark(prop: Option<&PropertySet>, psid: i32, extraId: i32, isRevFinal: bool) -> i64 {
      // java long mark = 0L;
      let mut mark: i64 = 0;
      // java mark = setFlagBit(mark, 2305843009213693952L, hasStyles(prop));
      mark = setFlagBit(mark, LONG_BIT_MERGED_FROM_STYLE, Self::hasStyles(prop));
      // java mark = setFlagBit(mark, 4611686018427387904L, isRevFinal);
      mark = setFlagBit(mark, LONG_BIT_REVISION_FINAL, isRevFinal);
      // java mark = setFlagValue(mark, 0, 536870911L, (long)psid);
      mark = setFlagValue(mark, LONG_OFFSET_PSID, LONG_MASK_PSID, psid as i64);
      // java mark = setFlagValue(mark, 29, 536870911L, (long)extraId);
      mark = setFlagValue(mark, LONG_OFFSET_EXTRA_ID, LONG_MASK_PSID, extraId as i64);
      // java return mark;
      return mark;
    }

    // java public static long getParaPropCacheID(IPapxTable.Entry para, int extraId, boolean isRevFinal)
    pub fn getParaPropCacheID(para: &dyn PapxEntry, extraId: i32, isRevFinal: bool) -> i64 {
      // java return getMark(para.getProperty(), para.getPropertySetId(), extraId, isRevFinal);
      return Self::getMark(PapxEntry::getProperty(para), para.getPropertySetId(), extraId, isRevFinal);
    }

    // java private static long getRunPropCacheID(int paraPsid, int runPsid, boolean isRevFinal, boolean isMergedFromStyle)
    fn getRunPropCacheID(paraPsid: i32, runPsid: i32, isRevFinal: bool, isMergedFromStyle: bool) -> i64 {
      // java long flag = 0L;
      let mut flag: i64 = 0;
      // java flag = setFlagValue(flag, 29, 536870911L, (long)paraPsid);
      flag = setFlagValue(flag, LONG_OFFSET_EXTRA_ID, LONG_MASK_PSID, paraPsid as i64);
      // java flag = setFlagValue(flag, 0, 536870911L, (long)runPsid);
      flag = setFlagValue(flag, LONG_OFFSET_PSID, LONG_MASK_PSID, runPsid as i64);
      // java flag = setFlagBit(flag, 4611686018427387904L, isRevFinal);
      flag = setFlagBit(flag, LONG_BIT_REVISION_FINAL, isRevFinal);
      // java flag = setFlagBit(flag, 2305843009213693952L, isMergedFromStyle);
      flag = setFlagBit(flag, LONG_BIT_MERGED_FROM_STYLE, isMergedFromStyle);
      // java return flag;
      return flag;
    }

    // java public static long getRunPropCacheID(PropertySet runProp)
    // rust: 实例方法：prop.mParaMark/mRunMark（Java `instanceof CachedPropertySet` 内联）
    pub fn getRunPropCacheID__property(paraMark: i64, runMark: i64) -> i64 {
      // java if (!(runProp instanceof CachedPropertySet)) { return -1L; }
      // java long paraMark = prop.mParaMark; long runMark = prop.mRunMark;
      // java if (-1L != paraMark && -1L != runMark)
      if -1 != paraMark && -1 != runMark {
        // java boolean paraRevFinal = getFlagBit(paraMark, 4611686018427387904L);
        let paraRevFinal: bool = getFlagBit(paraMark, LONG_BIT_REVISION_FINAL);
        // java boolean runRevFinal = getFlagBit(runMark, 4611686018427387904L);
        let runRevFinal: bool = getFlagBit(runMark, LONG_BIT_REVISION_FINAL);
        // java if (paraRevFinal != runRevFinal)
        if paraRevFinal != runRevFinal {
          // java return -1L;
          return -1;
        }
        // java else
        else {
          // java int paraPsid = (int)getFlagValue(paraMark, 0, 536870911L);
          let paraPsid: i32 = getFlagValue(paraMark, LONG_OFFSET_PSID, LONG_MASK_PSID);
          // java int runPsid = (int)getFlagValue(runMark, 0, 536870911L);
          let runPsid: i32 = getFlagValue(runMark, LONG_OFFSET_PSID, LONG_MASK_PSID);
          // java boolean paraMergedFromStyle = getFlagBit(paraMark, 2305843009213693952L);
          let paraMergedFromStyle: bool = getFlagBit(paraMark, LONG_BIT_MERGED_FROM_STYLE);
          // java boolean runMergedFromStyle = getFlagBit(runMark, 2305843009213693952L);
          let runMergedFromStyle: bool = getFlagBit(runMark, LONG_BIT_MERGED_FROM_STYLE);
          // java boolean mergedFromStyle = paraMergedFromStyle || runMergedFromStyle;
          let mergedFromStyle: bool = paraMergedFromStyle || runMergedFromStyle;
          // java return getRunPropCacheID(paraPsid, runPsid, paraRevFinal, mergedFromStyle);
          return Self::getRunPropCacheID(paraPsid, runPsid, paraRevFinal, mergedFromStyle);
        }
      }
      // java else
      else {
        // java return -1L;
        return -1;
      }
    }

    // java public static long getRunPropCacheID(PropertySet paraProperty, IChpxTable.Entry run, boolean isRevFinal)
    pub fn getRunPropCacheID__property_entry_boolean(paraMark: i64, run: &dyn ChpxEntry, isRevFinal: bool) -> i64 {
      // java if (!(paraProperty instanceof CachedPropertySet))
      // rust: paraMark == -1 等价非 CachedPropertySet（未 setParaMark）
      // java long paraMark = paraProp.mParaMark;
      // java if (-1L == paraMark)
      if -1 == paraMark {
        // java return -1L;
        return -1;
      }
      // java else if (isRevFinal != getFlagBit(paraMark, 4611686018427387904L))
      else if isRevFinal != getFlagBit(paraMark, LONG_BIT_REVISION_FINAL) {
        // java return -1L;
        return -1;
      }
      // java else
      else {
        // java int paraPsid = (int)getFlagValue(paraMark, 0, 536870911L);
        let paraPsid: i32 = getFlagValue(paraMark, LONG_OFFSET_PSID, LONG_MASK_PSID);
        // java PropertySet runProp = run.getProperty();
        let runProp: Option<&PropertySet> = ChpxEntry::getProperty(run);
        // java boolean hasStyles = paraProp.isMergedFromStyle() || hasStyles(runProp);
        let paraMergedFromStyle: bool = getFlagBit(paraMark, LONG_BIT_MERGED_FROM_STYLE);
        let hasStyles: bool = paraMergedFromStyle || Self::hasStyles(runProp);
        // java return getRunPropCacheID(paraPsid, run.getPropertySetId(), isRevFinal, hasStyles);
        return Self::getRunPropCacheID(paraPsid, run.getPropertySetId(), isRevFinal, hasStyles);
      }
    }
}

// java private static long setFlagValue(long flag, int offset, long mask, long val)
pub fn setFlagValue(flag: i64, offset: i32, mask: i64, val: i64) -> i64 {
  // java long shiftedMask = mask << offset;
  let shiftedMask: i64 = mask << offset as i32;
  // java flag &= ~shiftedMask;
  let mut flag: i64 = flag;
  flag &= !shiftedMask;
  // java long shiftedVal = val << offset;
  let shiftedVal: i64 = val << offset as i32;
  // java flag |= shiftedVal;
  flag |= shiftedVal;
  // java return flag;
  return flag;
}

// java private static long getFlagValue(long flag, int offset, long mask)
pub fn getFlagValue(flag: i64, offset: i32, mask: i64) -> i32 {
  // java flag >>= offset;
  // java flag &= mask;
  // java (int)getFlagValue(...) —— Java 返回 long，调用处强转 int；bit ops 结果同 i32
  let shifted: i64 = flag >> offset as i32;
  (shifted & mask) as i32
}

// java private static long setFlagBit(long flag, long bit, boolean val)
pub fn setFlagBit(flag: i64, bit: i64, val: bool) -> i64 {
  // java return val ? flag | bit : flag & ~bit;
  return if val { flag | bit } else { flag & !bit };
}

// java private static boolean getFlagBit(long val, long bit)
pub fn getFlagBit(val: i64, bit: i64) -> bool {
  // java return 0L != (val & bit);
  return 0 != (val & bit);
}