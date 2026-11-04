// Java: import cn.wps.kfc.impl.map.WLongObjectHashMap
use util::kfc::r#impl::map::WLongObjectHashMap::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;

// java static final int MAX_PERCENT = 30000;
pub(crate) const MAX_PERCENT: i32 = 30000;
// java static final int MAX_DXA = 31680;
pub(crate) const MAX_DXA: i32 = 31680;
// java private static WLongObjectHashMap<TableLength> sPool = new WLongObjectHashMap(32);
static sPool: std::sync::LazyLock<std::sync::Mutex<WLongObjectHashMap<TableLength>>> =
                    std::sync::LazyLock::new(|| std::sync::Mutex::new(WLongObjectHashMap::WLongObjectHashMap_with_capacity(32)));

// java public class TableLength
#[derive(Clone, Default)]
pub struct TableLength {
    // java int type;
    pub(crate) r#type: i32,
    // java int value;
    pub(crate) value: i32,
}

// java public class TableLength
impl TableLength {
    // java public  TableLength()
    pub fn new() -> Self {
      // java this(1, 0);
      let mut this = Self::new__int_int(1, 0);
      this
    }

    // java public  TableLength(int type, int value)
    pub fn new__int_int(r#type: i32, value: i32) -> Self {
      // java this.type = 1;
      Self { r#type, value }
      // java this.value = 0;
      // java this.type = type;
      // java this.value = value;
    }

    // java public int getType()
    pub fn getType(&self) -> i32 {
      // java return this.type;
      return self.r#type;
    }

    // java public int getValue()
    pub fn getValue(&self) -> i32 {
      // java return this.value;
      return self.value;
    }

    // java public float getPercent()
    pub fn getPercent(&self) -> f32 {
      // java assert this.type == 2 : "type error";
      assert!(self.r#type == 2, "type error");
      // java return (float)this.value / 50.0F;
      return ((self.value) as f32) / 50.0;
    }

    // java public float getPT()
    pub fn getPT(&self) -> f32 {
      // java assert this.type == 3 || this.type == 1 : "type error";
      assert!(self.r#type == 3 || self.r#type == 1, "type error");
      // java return (float)this.value / 20.0F;
      return ((self.value) as f32) / 20.0;
    }

    // java public int getTWIPS()
    pub fn getTWIPS(&self) -> i32 {
      // java assert this.type == 3 || this.type == 1 : "type error";
      assert!(self.r#type == 3 || self.r#type == 1, "type error");
      // java return this.value;
      return self.value;
    }

    // java public boolean isValid()
    pub fn isValid(&self) -> bool {
      // java if (this.type == 1 && this.value >= 0 && this.value <= 31680)
      if self.r#type == 1 && self.value >= 0 && self.value <= 31680 {
        // java return true;
        return true;
      }
      // java else if (this.type == 2 && this.value >= 0 && this.value <= 30000)
      else if self.r#type == 2 && self.value >= 0 && self.value <= 30000 {
        // java return true;
        return true;
      }
      // java else
      else {
        // java return (this.type == 3 || this.type == 0) && this.value >= 0 && this.value <= 31680;
        return (self.r#type == 3 || self.r#type == 0) && self.value >= 0 && self.value <= 31680;
      }
    }

    // java public boolean equals(Object rhs)
    pub fn equals(&self, rhs: &dyn Object) -> bool {
      // java if (!(rhs instanceof TableLength))
      if rhs.as_any().downcast_ref::<TableLength>().is_none() {
        // java return false;
        return false;
      }
      // java else
      else {
        // java TableLength r = (TableLength)rhs;
        let r: &TableLength = rhs.as_any().downcast_ref::<TableLength>().unwrap();
        // java return this.type == r.type && this.value == r.value;
        return self.r#type == r.r#type && self.value == r.value;
      }
    }

    // java public int hashCode()
    pub fn hashCode(&self) -> i32 {
      // java return (this.type << 16) + this.value;
      return (self.r#type << 16) + self.value;
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java StringBuilder sb = new StringBuilder();
      let mut sb: String = String::new();
      // java sb.append(this.getClass().getSimpleName());
      sb.push_str("TableLength");
      // java sb.append(": type = ");
      sb.push_str(": type = ");
      // java String typeStr = "unknown";
      let mut typeStr: String = "unknown".to_string();
      // java switch (this.type)
      match self.r#type {
        // java case 0:
        0 => {
          // java typeStr = "nil";
          typeStr = "nil".to_string();
        }
        // java case 1:
        1 => {
          // java typeStr = "auto";
          typeStr = "auto".to_string();
        }
        // java case 2:
        2 => {
          // java typeStr = "percent";
          typeStr = "percent".to_string();
        }
        // java case 3:
        3 => {
          // java typeStr = "dxa";
          typeStr = "dxa".to_string();
        }
        // java default:
        _ => {
        }
      }
      // java sb.append(typeStr);
      sb.push_str(&typeStr);
      // java sb.append(", value = ");
      sb.push_str(", value = ");
      // java sb.append(this.value);
      sb.push_str(&self.value.to_string());
      // java return sb.toString();
      return sb;
    }

    // java public static synchronized TableLength obtain(int type, int value)
    pub fn obtain(r#type: i32, value: i32) -> TableLength {
      // java long key = (long)type << 32 | (long)value;
      let mut key: i64 = ((r#type) as i64) << 32 | ((value) as i64);
      // java TableLength cached = (TableLength)sPool.get(key);
      let mut pool = sPool.lock().unwrap();
      let mut cached: Option<TableLength> = pool.get(key).cloned();
      // java if (cached == null)
      if cached.is_none() {
        // java cached = new TableLength(type, value);
        cached = Some(TableLength::new__int_int(r#type, value));
        // java sPool.put(key, cached);
        pool.put(key, cached.as_ref().unwrap().clone());
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
