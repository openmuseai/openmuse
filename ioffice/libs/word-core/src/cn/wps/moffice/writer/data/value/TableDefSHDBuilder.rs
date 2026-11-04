// Java: import cn.wps.moffice.drawing.graphics.SHD
use drawing::cn::wps::moffice::drawing::graphics::SHD::*;
// Java: import java.lang.System
use stubs::java::lang::System::*;
// Java: import java.util.HashMap
// Java: import cn.wps.moffice.writer.data.value.TableDefSHD
use crate::cn::wps::moffice::writer::data::value::TableDefSHD::*;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

// java private static HashMap<TableDefSHDBuilder, TableDefSHDBuilder> sPool = new HashMap();
// rust: Java 的 `HashMap` 以 `Object.hashCode()/equals()` 做键，Rust 需 `Hash + Eq`；
//   `TableDefSHDBuilder` 无法安全派生（含 `super_` 的 `Vec<Option<SHD>>`），
//   故以 `(mCount, 各 shd 的 hashCode)` 组合键复刻 Java 的键相等语义。
static sPool: LazyLock<Mutex<HashMap<u64, TableDefSHDBuilder>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

// java public class TableDefSHDBuilder extends TableDefSHD
#[derive(Clone, Default)]
pub struct TableDefSHDBuilder {
    // java TableDefSHD super_
    pub(crate) super_: TableDefSHD,
    // java private boolean mFrozen;
    mFrozen: bool,
}

// java public class TableDefSHDBuilder extends TableDefSHD
impl TableDefSHDBuilder {
    // java public  TableDefSHDBuilder()
    pub fn new() -> Self {
      // java super(0);
      Self { super_: TableDefSHD::new(0), mFrozen: Default::default() }
    }

    // java public  TableDefSHDBuilder(int count)
    pub fn new__int(count: i32) -> Self {
      // java super(count);
      Self { super_: TableDefSHD::new(count), mFrozen: Default::default() }
    }

    // java public  TableDefSHDBuilder(TableDefSHD tableDefSHD)
    pub fn new__tabledefshd(tableDefSHD: &TableDefSHD) -> Self {
      // java this(tableDefSHD.mCount);
      let mut this = Self::new__int(tableDefSHD.mCount);
      // java int size = tableDefSHD.mCount;
      let mut size: i32 = tableDefSHD.mCount;
      // java for(int i = 0; i < size; ++i)
      for i in 0..size {
        // java SHD shd = tableDefSHD.getCellSHD(i);
        let mut shd: Option<SHD> = tableDefSHD.getCellSHD(i);
        // java if (null != shd)
        if shd.is_some() {
          // java this.setCellSHD(i, shd);
          this.setCellSHD__int_shd(i, shd);
        }
      }
      this
    }

    // java private void checkFrozen()
    pub fn checkFrozen(&mut self) {
    }

    // java public void setCellSHD(int iCell, SHD shd)
    // rust: 覆盖 `TableDefSHD.setCellSHD` —— Java 此处含**扩容**逻辑，
    //   故用独立名避免与父类辅助方法冲突，并在 `Deref` 下保持可调用。
    pub fn setCellSHD__int_shd(&mut self, iCell: i32, shd: Option<SHD>) {
      // java this.checkFrozen();
      self.checkFrozen();
      // java if (iCell >= this.mCount && null != shd)
      if iCell >= self.super_.mCount && shd.is_some() {
        // java SHD[] new_shds = new SHD[iCell + 1];
        let mut new_shds: Vec<Option<SHD>> = vec![None; (iCell + 1) as usize];
        // java System.arraycopy(this.shds, 0, new_shds, 0, this.shds.length);
        // rust: Java 数组按值复制引用；Rust `Vec<Option<SHD>>` 用 `clone_from_slice`。
        let old: Vec<Option<SHD>> = self.super_.shds.clone().unwrap_or_default();
        let n = old.len();
        new_shds[..n].clone_from_slice(&old[..n]);
        let _ = &new_shds;
        // java this.shds = new_shds;
        self.super_.shds = Some(new_shds);
        // java this.mCount = new_shds.length;
        self.super_.mCount = iCell + 1;
      }

      // java if (iCell >= 0 && null != shd)
      if iCell >= 0 && shd.is_some() {
        // java this.shds[iCell] = shd;
        self.super_.setCellSHD(iCell, shd);
      }
    }

    // java public TableDefSHD freeze()
    // rust: Java 用 `synchronized(sPool)` + `HashMap.get/put`；Rust 侧以
    //   `Mutex<HashMap>` 复刻同一临界区语义。Java 的 `cached == null` 是
    //   **对象引用**判空（`get` 未命中返回 null），Rust 对应 `Option`。
    pub fn freeze(&mut self) -> TableDefSHD {
        // java synchronized(sPool)
        let mut pool = sPool.lock().expect("sPool lock");
        // java TableDefSHDBuilder cached = (TableDefSHDBuilder)sPool.get(this);
        let key = self.poolKey();
        let cached: Option<TableDefSHDBuilder> = pool.get(&key).cloned();
        // java if (cached == null)
        let cached = match cached {
            Some(c) => c,
            None => {
                // java cached = this;
                let c = self.clone();
                // java this.mFrozen = true;
                self.mFrozen = true;
                // java sPool.put(this, this);
                pool.insert(key, c.clone());
                c
            }
        };
        // java return cached;
        return cached.super_.clone();
    }

    // java public static void clear()
    pub fn clear() {
        // java synchronized(sPool)
        let mut pool = sPool.lock().expect("sPool lock");
        // java sPool.clear();
        pool.clear();
    }

    // rust: 非 Java 成员 —— `sPool` 的键。Java 依赖 `HashMap` 默认的
    //   `Object.hashCode()/equals()`，即 `TableDefSHD.hashCode()`（内容哈希）。
    fn poolKey(&self) -> u64 {
        // Java 等价：this.hashCode()（继承自 TableDefSHD）
        let h = self.super_.hashCode();
        h as u64
    }
}

impl std::ops::Deref for TableDefSHDBuilder {
    type Target = TableDefSHD;

    fn deref(&self) -> &Self::Target {
        &self.super_
    }
}

impl std::ops::DerefMut for TableDefSHDBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.super_
    }
}
