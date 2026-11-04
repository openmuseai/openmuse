// Java: import java.util.HashMap
// Java: import cn.wps.moffice.writer.data.value.TableLength
use crate::cn::wps::moffice::writer::data::value::TableLength::TableLength;
// Java: import java.lang.Object
use stubs::java::lang::Object::Object;
use std::collections::{HashMap};
use std::hash::{Hash, Hasher};

// java private static HashMap<CellSpacing, CellSpacing> sPool = new HashMap();
static sPool: std::sync::LazyLock<std::sync::Mutex<HashMap<CellSpacing, CellSpacing>>> =
                    std::sync::LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));
// java private static CellSpacing sTemp = new CellSpacing();
static sTemp: std::sync::LazyLock<std::sync::Mutex<CellSpacing>> =
                    std::sync::LazyLock::new(|| std::sync::Mutex::new(CellSpacing::CellSpacing()));

// java public class CellSpacing
#[derive(Clone)]
pub struct CellSpacing {
    // java int wUnknown = 256;
    pub(crate) wUnknown: i32,
    // java int grbitChg;
    pub(crate) grbitChg: i32,
    // java TableLength cellSpacingValue;
    pub(crate) cellSpacingValue: TableLength,
}

// java public class CellSpacing
impl CellSpacing {
    // java public  CellSpacing()
    pub fn CellSpacing() -> Self {
      // java this.grbitChg = 0;
      // java this.cellSpacingValue = new TableLength(0, 0);
      Self { wUnknown: 256, grbitChg: 0, cellSpacingValue: TableLength::new__int_int(0, 0) }
    }

    // java public  CellSpacing(int grbitChg, int type, int value)
    pub fn CellSpacing__int_int_int(grbitChg: i32, r#type: i32, value: i32) -> Self {
      // java this.grbitChg = grbitChg;
      // java this.cellSpacingValue = TableLength.obtain(type, value);
      Self { wUnknown: 256, grbitChg, cellSpacingValue: TableLength::obtain(r#type, value) }
    }

    // java public int getGrbitChg()
    pub fn getGrbitChg(&self) -> i32 {
      // java return this.grbitChg;
      return self.grbitChg;
    }

    // java public TableLength getCellSpacingValue()
    pub fn getCellSpacingValue(&self) -> TableLength {
      // java return this.cellSpacingValue;
      return self.cellSpacingValue.clone();
    }

    // java public boolean equals(Object rhs)
    pub fn equals(&self, rhs: &dyn Object) -> bool {
      // java if (!(rhs instanceof CellSpacing))
      if rhs.as_any().downcast_ref::<CellSpacing>().is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java CellSpacing r = (CellSpacing)rhs;
        let r: &CellSpacing = rhs.as_any().downcast_ref::<CellSpacing>().unwrap();
        // java return this.wUnknown == r.wUnknown && this.grbitChg == r.grbitChg && this.cellSpacingValue.equals(r.cellSpacingValue);
        return self.wUnknown == r.wUnknown && self.grbitChg == r.grbitChg && self.cellSpacingValue.equals(&r.cellSpacingValue);
      }
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
      // java return this.wUnknown + this.grbitChg + this.cellSpacingValue.hashCode();
      return self.wUnknown + self.grbitChg + self.cellSpacingValue.hashCode();
    }

    // java public static synchronized CellSpacing obtain(int grbitChg, int type, int value)
    pub fn obtain(grbitChg: i32, r#type: i32, value: i32) -> Self {
      let key = {
        let mut temp = sTemp.lock().unwrap();
        // java sTemp.grbitChg = grbitChg;
        temp.grbitChg = grbitChg;
        // java sTemp.cellSpacingValue.type = type;
        temp.cellSpacingValue.r#type = r#type;
        // java sTemp.cellSpacingValue.value = value;
        temp.cellSpacingValue.value = value;
        temp.clone()
      };
      let mut pool = sPool.lock().unwrap();
      // java CellSpacing cached = (CellSpacing)sPool.get(sTemp);
      let mut cached: Option<CellSpacing> = pool.get(&key).cloned();
      // java if (cached == null)
      if cached.is_none() {
        // java cached = new CellSpacing(grbitChg, type, value);
        cached = Some(CellSpacing::CellSpacing__int_int_int(grbitChg, r#type, value));
        // java sPool.put(cached, cached);
        let put = cached.clone().unwrap();
        pool.insert(put.clone(), put);
      }
      // java return cached;
      return cached.unwrap();
    }

    // java public static synchronized void clear()
    pub fn clear() {
      // java sPool.clear();
      sPool.lock().unwrap().clear();
    }

}

impl PartialEq for CellSpacing {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}
impl Eq for CellSpacing {}
impl Hash for CellSpacing {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hashCode().hash(state);
    }
}
impl Default for CellSpacing {
    fn default() -> Self {
        Self::CellSpacing()
    }
}
