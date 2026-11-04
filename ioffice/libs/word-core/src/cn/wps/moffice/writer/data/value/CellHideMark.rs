// Java: import java.util.HashMap
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

// java private static HashMap<CellHideMark, CellHideMark> sPool = new HashMap();
static sPool: std::sync::LazyLock<std::sync::Mutex<HashMap<CellHideMark, CellHideMark>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));
// java private static CellHideMark sTemp = new CellHideMark();
static sTemp: std::sync::LazyLock<std::sync::Mutex<CellHideMark>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(CellHideMark { itcFirst: 0, itcLim: 0, mBArg: false }));

// java public final class CellHideMark
#[derive(Clone)]
pub struct CellHideMark {
    // java int itcFirst = 0;
    pub(crate) itcFirst: i32,
    // java int itcLim = 0;
    pub(crate) itcLim: i32,
    // java boolean mBArg = false;
    pub(crate) mBArg: bool,
}

// java public final class CellHideMark
impl CellHideMark {
    // java public boolean getbArg()
    pub fn getbArg(&self) -> bool {
      // java return this.mBArg;
      return self.mBArg;
    }

    // java public int getFirst()
    pub fn getFirst(&self) -> i32 {
      // java return this.itcFirst;
      return self.itcFirst;
    }

    // java public int getLast()
    pub fn getLast(&self) -> i32 {
      // java return this.itcLim;
      return self.itcLim;
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
      // java return (this.itcFirst << 16) + (this.itcLim << 8) + (this.mBArg ? 0 : 255);
      return (self.itcFirst << 16) + (self.itcLim << 8) + (if self.mBArg { 0 } else { 255 });
    }

    // java public boolean equals(Object obj)
    pub fn equals(&self, obj: &dyn Object) -> bool {
      // java if (!(obj instanceof CellHideMark))
      let Some(r) = obj.as_any().downcast_ref::<CellHideMark>() else {
        // java return false;
        return false;
      };
      // java CellHideMark r = (CellHideMark)obj;
      // java return this.itcFirst == r.itcFirst && this.itcLim == r.itcLim && this.mBArg == r.mBArg;
      return self.itcFirst == r.itcFirst && self.itcLim == r.itcLim && self.mBArg == r.mBArg;
    }

    // java public static synchronized CellHideMark obtain(int first, int last, boolean bArg)
    pub fn obtain(first: i32, last: i32, bArg: bool) -> CellHideMark {
      let key = {
        let mut temp = sTemp.lock().unwrap();
        // java sTemp.itcFirst = first;
        temp.itcFirst = first;
        // java sTemp.itcLim = last;
        temp.itcLim = last;
        // java sTemp.mBArg = bArg;
        temp.mBArg = bArg;
        temp.clone()
      };
      let mut pool = sPool.lock().unwrap();
      // java CellHideMark cached = (CellHideMark)sPool.get(sTemp);
      if let Some(cached) = pool.get(&key).cloned() {
        // java return cached;
        return cached;
      }
      // java if (cached == null)
      // java cached = new CellHideMark();
      let cached = CellHideMark { itcFirst: first, itcLim: last, mBArg: bArg };
      // java cached.itcFirst = first;
      // java cached.itcLim = last;
      // java cached.mBArg = bArg;
      // java sPool.put(cached, cached);
      pool.insert(cached.clone(), cached.clone());
      // java return cached;
      cached
    }

    // java public static synchronized void clear()
    pub fn clear() {
      // java sPool.clear();
      sPool.lock().unwrap().clear();
    }

}

impl Default for CellHideMark {
    fn default() -> Self {
        Self { itcFirst: 0, itcLim: 0, mBArg: false }
    }
}

impl PartialEq for CellHideMark {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl Eq for CellHideMark {}

impl Hash for CellHideMark {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(self.hashCode());
    }
}
