// Java: import java.util.HashMap
// Java: import cn.wps.moffice.writer.data.value.TableLength
use crate::cn::wps::moffice::writer::data::value::TableLength::*;
// Java: import cn.wps.moffice.writer.data.value.ReadOnlyArray
use crate::cn::wps::moffice::writer::data::value::ReadOnlyArray::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{LazyLock, Mutex};

// java public static final int TOP = 1;
pub(crate) const TOP: i32 = 1;
// java public static final int LEFT = 2;
pub(crate) const LEFT: i32 = 2;
// java public static final int BOTTOM = 4;
pub(crate) const BOTTOM: i32 = 4;
// java public static final int RIGHT = 8;
pub(crate) const RIGHT: i32 = 8;
// java public static ReadOnlyArray.Builder<CellMargin> sArrayBuilder = new ReadOnlyArray.Builder<CellMargin>();
pub(crate) static sArrayBuilder: LazyLock<Mutex<Builder<CellMargin>>> =
                    LazyLock::new(|| Mutex::new(Builder::new()));
// java private static HashMap<CellMargin, CellMargin> sPool = new HashMap();
static sPool: LazyLock<Mutex<HashMap<CellMargin, CellMargin>>> =
                    LazyLock::new(|| Mutex::new(HashMap::new()));
// java private static CellMargin sTemp = new CellMargin();
static sTemp: LazyLock<Mutex<CellMargin>> =
                    LazyLock::new(|| Mutex::new(CellMargin::new()));

// java public class CellMargin
#[derive(Default, Clone)]
pub struct CellMargin {
    // java int first;
    pub(crate) first: i32,
    // java int beyond;
    pub(crate) beyond: i32,
    // java int grbitChg;
    pub(crate) grbitChg: i32,
    // java TableLength cellMarginValue;
    pub(crate) cellMarginValue: Option<TableLength>,
}

// java public class CellMargin
impl CellMargin {
    // java public  CellMargin()
    pub fn new() -> Self {
      // java this.grbitChg = 0;
      Self { first: Default::default(), beyond: Default::default(), grbitChg: 0, cellMarginValue: Some(TableLength::new()) }
      // java this.cellMarginValue = new TableLength();
    }

    // java public  CellMargin(int first, int beyond, int grbitChg, int type, int value)
    pub fn new__int_int_int_int_int(first: i32, beyond: i32, grbitChg: i32, r#type: i32, value: i32) -> Self {
      // java this.grbitChg = grbitChg;
      Self { first: first, beyond: beyond, grbitChg: grbitChg, cellMarginValue: Some(TableLength::obtain(r#type, value)) }
      // java this.cellMarginValue = TableLength.obtain(type, value);
      // java this.first = first;
      // java this.beyond = beyond;
    }

    // java public int getFirst()
    pub fn getFirst(&self) -> i32 {
      // java return this.first;
      return self.first;
    }

    // java public int getBeyond()
    pub fn getBeyond(&self) -> i32 {
      // java return this.beyond;
      return self.beyond;
    }

    // java public int getGrbitChg()
    pub fn getGrbitChg(&self) -> i32 {
      // java return this.grbitChg;
      return self.grbitChg;
    }

    // java public TableLength getCellSpacingValue()
    pub fn getCellSpacingValue(&self) -> TableLength {
      // java return this.cellMarginValue;
      self.cellMarginValue.clone().unwrap_or_else(TableLength::new)
    }

    // java public boolean equals(Object rhs)
    pub fn equals(&self, rhs: &dyn Object) -> bool {
      // java if (!(rhs instanceof CellMargin))
      let Some(r) = rhs.as_any().downcast_ref::<CellMargin>() else {
        // java return false;
        return false;
      };
      // java CellMargin r = (CellMargin)rhs;
      // java return this.first == r.first && this.beyond == r.beyond && this.grbitChg == r.grbitChg && this.cellMarginValue.equals(r.cellMarginValue);
      self.first == r.first
        && self.beyond == r.beyond
        && self.grbitChg == r.grbitChg
        && match (&self.cellMarginValue, &r.cellMarginValue) {
            (Some(left), Some(right)) => left.equals(right),
            (None, None) => true,
            _ => false,
        }
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
      // java return this.first + this.beyond + this.grbitChg + this.cellMarginValue.hashCode();
      self.first + self.beyond + self.grbitChg + self.cellMarginValue.as_ref().map(TableLength::hashCode).unwrap_or(0)
    }

    // java public static synchronized CellMargin obtain(int first, int beyond, int grbitChg, int type, int value)
    pub fn obtain__int_int_int_int_int(first: i32, beyond: i32, grbitChg: i32, r#type: i32, value: i32) -> CellMargin {
      let mut pool = sPool.lock().expect("sPool");
      let mut s_temp = sTemp.lock().expect("sTemp");
      // java sTemp.first = first;
      s_temp.first = first;
      // java sTemp.beyond = beyond;
      s_temp.beyond = beyond;
      // java sTemp.grbitChg = grbitChg;
      s_temp.grbitChg = grbitChg;
      // java sTemp.cellMarginValue.type = type;
      // java sTemp.cellMarginValue.value = value;
      if let Some(length) = s_temp.cellMarginValue.as_mut() {
        length.r#type = r#type;
        length.value = value;
      }
      // java CellMargin cached = (CellMargin)sPool.get(sTemp);
      if let Some(cached) = pool.get(&*s_temp).cloned() {
        // java return cached;
        return cached;
      }
      // java cached = new CellMargin(first, beyond, grbitChg, type, value);
      let cached = CellMargin::new__int_int_int_int_int(first, beyond, grbitChg, r#type, value);
      // java sPool.put(cached, cached);
      pool.insert(cached.clone(), cached.clone());
      // java return cached;
      cached
    }

    // java public static CellMargin obtain(CellMargin orgin, int grbitChg)
    pub fn obtain__cellmargin_int(orgin: &CellMargin, grbitChg: i32) -> CellMargin {
      // java return obtain(orgin.first, orgin.beyond, grbitChg, orgin.cellMarginValue.getType(), orgin.cellMarginValue.getValue());
      let length = orgin.cellMarginValue.as_ref();
      CellMargin::obtain__int_int_int_int_int(
        orgin.first,
        orgin.beyond,
        grbitChg,
        length.map(TableLength::getType).unwrap_or(0),
        length.map(TableLength::getValue).unwrap_or(0),
      )
    }

    // java public static synchronized void clear()
    pub fn clear() {
      // java sPool.clear();
      sPool.lock().expect("sPool").clear();
      // java sArrayBuilder.clear();
      sArrayBuilder.lock().expect("sArrayBuilder").clear();
    }

}

impl PartialEq for CellMargin {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl Eq for CellMargin {}

impl Hash for CellMargin {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hashCode().hash(state);
    }
}
