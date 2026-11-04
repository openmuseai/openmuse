// Java: import cn.wps.moffice.writer.data.value.DefTable
use crate::cn::wps::moffice::writer::data::value::DefTable::DefTable;
// Java: import cn.wps.moffice.writer.data.value.TableCellBuilder
use crate::cn::wps::moffice::writer::data::value::TableCellBuilder::TableCellBuilder;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{LazyLock, Mutex};

// java private static HashMap<DefTableBuilder, DefTableBuilder> sPool = new HashMap();
static sPool: LazyLock<Mutex<HashMap<DefTableBuilder, DefTableBuilder>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
// java private static Stack<DefTableBuilder> sReuseStack = new Stack();
static sReuseStack: LazyLock<Mutex<Vec<DefTableBuilder>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));
// java private static final int REUSE_MAX = 8;
const REUSE_MAX: usize = 8;

// java public class DefTableBuilder extends DefTable
#[derive(Clone)]
pub struct DefTableBuilder {
    // java DefTable super_
    pub(crate) super_: DefTable,
    // java private boolean mFrozen;
    mFrozen: bool,
}

impl Default for DefTableBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// java public class DefTableBuilder extends DefTable
impl DefTableBuilder {
    // java public  DefTableBuilder()
    pub fn new() -> Self {
      Self { super_: DefTable::new(), mFrozen: false }
    }

    // java public  DefTableBuilder(DefTable defTable)
    pub fn new__deftable(defTable: Option<&DefTable>) -> Self {
      // java super(defTable);
      let mut this = Self::new();
      this.init(defTable, None);
      this
    }

    // java public  DefTableBuilder(DefTable defTables, int[] rgdxaCenter)
    pub fn new__deftable_intarr(defTables: Option<&DefTable>, rgdxaCenter: Option<&Vec<i32>>) -> Self {
      // java super(defTables, rgdxaCenter);
      let mut this = Self::new();
      this.init(defTables, rgdxaCenter);
      this
    }

    // java protected void initDefault()
    pub fn initDefault(&mut self) {
      // java super.initDefault();
      self.super_.initDefault();
      // java this.mFrozen = false;
      self.mFrozen = false;
    }

    // java private void checkFrozen()
    pub fn checkFrozen(&mut self) {
    }

    // java public void setItMax(int max)
    pub fn setItMax(&mut self, max: i32) {
      // java this.checkFrozen();
      self.checkFrozen();
      // java this.itMax = max < 0 ? 0 : max;
      self.itMax = if max < 0 { 0 } else { max };
      // java if (this.rgdxaCenter == null || this.rgdxaCenter.length < this.itMax)
      let len = self.rgdxaCenter.as_ref().map(|v| v.len() as i32).unwrap_or(0);
      if len < self.itMax {
        // java this.rgdxaCenter = new int[max];
        // java Arrays.fill(this.rgdxaCenter, 0);
        self.rgdxaCenter = Some(vec![0; self.itMax as usize]);
      }
    }

    // java public void setCellCount(int cellCount)
    pub fn setCellCount(&mut self, cellCount: i32) {
      // java this.checkFrozen();
      self.checkFrozen();
      // java this.itc = cellCount < 0 ? 0 : cellCount;
      self.itc = if cellCount < 0 { 0 } else { cellCount };
      // java if (this.rgtc == null || this.rgtc.length < this.itc)
      let len = self.rgtc.as_ref().map(|v| v.len() as i32).unwrap_or(0);
      if len < self.itc {
        // java this.rgtc = new TableCellBuilder[this.itc];
        // java Arrays.fill(this.rgtc, (Object)null);
        self.rgtc = Some(vec![None; self.itc as usize]);
      }
    }

    // java public void setRgdxa(int it, int value)
    pub fn setRgdxa(&mut self, it: i32, value: i32) {
      // java this.checkFrozen();
      self.checkFrozen();
      // java if (it >= 0 && it < this.itMax)
      if it >= 0 && it < self.itMax {
        // java this.rgdxaCenter[it] = value;
        if let Some(v) = self.rgdxaCenter.as_mut() {
          if (it as usize) < v.len() {
            v[it as usize] = value;
          }
        }
      }
    }

    // java public void setTableCellBuilder(int it, TableCellBuilder cell)
    pub fn setTableCellBuilder(&mut self, it: i32, cell: &TableCellBuilder) {
      // java this.checkFrozen();
      self.checkFrozen();
      // java if (it >= 0 && it < this.itc)
      if it >= 0 && it < self.itc {
        // java this.rgtc[it] = cell;
        if let Some(v) = self.rgtc.as_mut() {
          if (it as usize) < v.len() {
            v[it as usize] = Some(cell.clone());
          }
        }
      }
    }

    // java public TableCellBuilder getTableCellBuilder(int it)
    pub fn getTableCellBuilder(&self, it: i32) -> Option<TableCellBuilder> {
      // java return it >= 0 && it < this.itc ? this.rgtc[it] : null;
      if it >= 0 && it < self.itc {
        self.rgtc
            .as_ref()
            .and_then(|v| v.get(it as usize))
            .and_then(|c| c.clone())
      } else {
        None
      }
    }

    // java public DefTable freeze()
    pub fn freeze(&mut self) -> DefTableBuilder {
      // java synchronized(sPool)
      let mut pool = sPool.lock().unwrap();
      let mut i: i32 = 0;
      // java for(int i = 0; i < this.itc; ++i)
      while i < self.itc {
        // java TableCellBuilder tc = this.rgtc[i];
        let frozen = self
            .rgtc
            .as_mut()
            .and_then(|v| v.get_mut(i as usize))
            .and_then(|slot| slot.as_mut())
            .map(|tc| tc.freeze());
        // java if (tc != null)
        if let Some(tc) = frozen {
          // java this.rgtc[i] = tc.freeze();
          if let Some(v) = self.rgtc.as_mut() {
            v[i as usize] = Some(tc);
          }
        }
        i += 1;
      }
      self.hash = 0;
      // java DefTableBuilder cached = (DefTableBuilder)sPool.get(this);
      if let Some(cached) = pool.get(self).cloned() {
        // java else if (cached != this && !this.mFrozen)
        if !self.mFrozen {
          let mut stack = sReuseStack.lock().unwrap();
          // java if (sReuseStack.size() < 8)
          if stack.len() < REUSE_MAX {
            // java this.initDefault();
            self.initDefault();
            // java sReuseStack.push(this);
            stack.push(self.clone());
          }
        }
        // java return cached;
        return cached;
      }
      // java cached = this;
      // java this.mFrozen = true;
      self.mFrozen = true;
      // java sPool.put(this, this);
      let cached = self.clone();
      pool.insert(cached.clone(), cached.clone());
      cached
    }

    // java public static DefTableBuilder obtain(DefTable oriTable, int[] rgdxaCenter)
    pub fn obtain__deftable_intarr(oriTable: Option<&DefTable>, rgdxaCenter: Option<&Vec<i32>>) -> DefTableBuilder {
      // java synchronized(sReuseStack)
      let recycled = sReuseStack.lock().unwrap().pop();
      // java if (newTable == null)
      if let Some(mut newTable) = recycled {
        // java newTable.init(oriTable, rgdxaCenter);
        newTable.init(oriTable, rgdxaCenter);
        newTable
      } else {
        // java newTable = new DefTableBuilder(oriTable, rgdxaCenter);
        Self::new__deftable_intarr(oriTable, rgdxaCenter)
      }
    }

    // java public static DefTableBuilder obtain(DefTable defTables)
    pub fn obtain__deftable(defTables: Option<&DefTable>) -> DefTableBuilder {
      // java return obtain(defTables, (int[])null);
      Self::obtain__deftable_intarr(defTables, None)
    }

    // java public static DefTableBuilder obtain()
    pub fn obtain() -> DefTableBuilder {
      // java return obtain((DefTable)null, (int[])null);
      Self::obtain__deftable_intarr(None, None)
    }

    // java public static void clear()
    pub fn clear() {
      // java synchronized(sPool)
      // java sPool.clear();
      sPool.lock().unwrap().clear();
    }
}

impl std::ops::Deref for DefTableBuilder {
    type Target = DefTable;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for DefTableBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}

impl PartialEq for DefTableBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.super_.same_table(&other.super_)
    }
}

impl Eq for DefTableBuilder {}

impl Hash for DefTableBuilder {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut table = self.super_.clone();
        state.write_i32(table.hashCode());
    }
}
