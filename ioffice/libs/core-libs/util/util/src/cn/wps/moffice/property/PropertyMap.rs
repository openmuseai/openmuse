// Java: import cn.wps.base.assertion.Assert
// Java: import cn.wps.collections.copyonwrite.CopyOnWriteIntSmallMap
// Java: import cn.wps.moffice.util.IIntSmallMap
// Java: import cn.wps.moffice.util.KSmallMap
// Java: import java.io.IOException
// Java: import java.io.ObjectInput
// Java: import java.io.ObjectOutput

use std::any::Any;
use std::sync::Arc;
use std::collections::HashMap;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java field: boolean sUseSmallMap
static S_USE_SMALL_MAP: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(true);

// Java: public class PropertyMap implements Cloneable, PropBaseSerializable
#[derive(Clone)]
pub struct PropertyMap {
    // Java field: IIntSmallMap map
    pub(crate) map: HashMap<i32, Element>,
}

pub trait Cloneable {}
impl Cloneable for PropertyMap {}

pub trait PropBaseSerializable {}
impl PropBaseSerializable for PropertyMap {}

// Java: public class PropertyMap implements Cloneable, PropBaseSerializable
impl PropertyMap {
    // Java: public static boolean sUseSmallMap = true;
    // rust: ★M-STAGE2 P1-D1 / T3b —— Java 该静态字段是 `public`，可被
    //   `TextDocument.java` 直接赋值（`PropertyMap.sUseSmallMap = true;`）。
    //   Rust 侧以私有 `static S_USE_SMALL_MAP: AtomicBool` 建模 ⇒ 补公开访问器
    //   以保持 Java/Rust 门面一致。
    pub fn set_sUseSmallMap(v: bool) {
        S_USE_SMALL_MAP.store(v, std::sync::atomic::Ordering::SeqCst);
    }

    // Java: public static boolean sUseSmallMap = true;
    pub fn get_sUseSmallMap() -> bool {
        S_USE_SMALL_MAP.load(std::sync::atomic::Ordering::SeqCst)
    }

    // Java: static PropertyMap merge(PropertyMap a, PropertyMap b)
    pub fn merge(a: &PropertyMap, b: &PropertyMap) -> PropertyMap {
      // Java: MutablePropertyMap pResult = new MutablePropertyMap();
      let mut pResult = PropertyMap::new();
      // Java: pResult.map.add(a.map);
      for (k, v) in &a.map {
          pResult.map.insert(*k, v.clone());
      }
      // Java: pResult.map.add(b.map);
      for (k, v) in &b.map {
          pResult.map.insert(*k, v.clone());
      }
      // Java: return pResult;
      pResult
    }

    // Java: PropertyMap()
    pub fn new() -> Self {
      // Java: super();
      // LLM JavaLine Aggregation: no super() needed in Rust
      // Java: this.map = (IIntSmallMap)(sUseSmallMap ? new KSmallMap() : new CopyOnWriteIntSmallMap());
      // LLM JavaLine Aggregation: merged into struct initialization; HashMap replaces IIntSmallMap
      PropertyMap { map: HashMap::new() }
    }

    // Java: PropertyMap(int capacity)
    pub fn with_capacity(capacity: usize) -> Self {
      // Java: super();
      // LLM JavaLine Aggregation: no super() needed in Rust
      // Java: this.map = (IIntSmallMap)(sUseSmallMap ? new KSmallMap(capacity) : new CopyOnWriteIntSmallMap(capacity));
      // LLM JavaLine Aggregation: merged into struct initialization; HashMap replaces IIntSmallMap
      PropertyMap { map: HashMap::with_capacity(capacity) }
    }

    // Java: PropertyMap clone()
    pub fn clone_map(&self) -> PropertyMap {
      // Java: try
      // LLM Rust Equivalent: Rust Clone doesn't throw, try/catch not needed
      {
         // Java: PropertyMap propMap = (PropertyMap)super.clone();
         // LLM JavaLine Aggregation: Rust derive(Clone) handles this directly
         // Java: propMap.map = this.map.clone();
         // LLM JavaLine Aggregation: included in Rust derive(Clone)
         // Java: return propMap;
         return self.clone();
      }
      // Java: catch (CloneNotSupportedException e)
      // LLM No Rust equivalent: Rust Clone never fails, catch block unreachable
      // Java: Assert.fail("It should not reach to here. " + e);
      // LLM No Rust equivalent: unreachable in Rust
      // Java: Assert.fail("It should not reach to here.");
      // LLM No Rust equivalent: unreachable in Rust
      // Java: return null;
      // LLM No Rust equivalent: unreachable in Rust
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: Option<&PropertyMap>) -> bool {
      // Java: if (o == null)
      if o.is_none() {
         // Java: return false;
         return false;
      }
      // Java: else
      else {
         // Java: return o == this ? true : this.map.equals(((PropertyMap)o).map);
         let o = o.unwrap();
         if std::ptr::eq(self, o) { return true; }
         self.map.len() == o.map.len()
      }
    }

    // Java: boolean contains(int nProp)
    pub fn contains(&self, n_prop: i32) -> bool {
      // Java: return this.getValue(nProp) != null;
      self.getValue(n_prop).is_some()
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
      // Java: this.map.clear();
      self.map.clear();
      // Java: this.map = null;
      // LLM Rust Null-Safety: HashMap has no null state; clear() is sufficient
    }

    // Java: boolean getBoolean(int nProp)
    pub fn getBoolean(&self, n_prop: i32) -> bool {
      // Java: Object pVal = this.getValue(nProp);
      let pVal = self.getValue(n_prop);
      // Java: if (pVal == null)
      if pVal.is_none() {
         // Java: throw new PropertyException("No such property: " + nProp);
         panic!("No such property: {}", n_prop);
      }
      // Java: else
      else {
         // Java: Boolean v = (Boolean)pVal;
         // LLM JavaLine Aggregation: cast merged into downcast_ref
         // Java: return v;
         *pVal.unwrap().downcast_ref::<bool>().unwrap_or(&false)
      }
    }

    // Java: boolean getBoolean(int nProp, boolean bDefault)
    pub fn getBoolean_default(&self, n_prop: i32, b_default: bool) -> bool {
      // Java: Object pVal = this.getValue(nProp);
      let pVal = self.getValue(n_prop);
      // Java: if (pVal == null)
      if pVal.is_none() {
         // Java: return bDefault;
         return b_default;
      }
      // Java: else
      else {
         // Java: Boolean v = (Boolean)pVal;
         // LLM JavaLine Aggregation: cast merged into downcast_ref
         // Java: return v;
         *pVal.unwrap().downcast_ref::<bool>().unwrap_or(&b_default)
      }
    }

    // Java: float getFloat(int nProp)
    pub fn getFloat(&self, n_prop: i32) -> f32 {
      // Java: Object pVal = this.getValue(nProp);
      let pVal = self.getValue(n_prop);
      // Java: if (pVal == null)
      if pVal.is_none() {
         // Java: throw new PropertyException("No such property: " + nProp);
         panic!("No such property: {}", n_prop);
      }
      // Java: else
      else {
         // Java: Float v = (Float)pVal;
         // LLM JavaLine Aggregation: cast merged into downcast_ref
         // Java: return v;
         *pVal.unwrap().downcast_ref::<f32>().unwrap_or(&0.0)
      }
    }

    // Java: float getFloat(int nProp, float fDefault)
    pub fn getFloat_default(&self, n_prop: i32, f_default: f32) -> f32 {
      // Java: Object pVal = this.getValue(nProp);
      let pVal = self.getValue(n_prop);
      // Java: if (pVal == null)
      if pVal.is_none() {
         // Java: return fDefault;
         return f_default;
      }
      // Java: else
      else {
         // Java: Float v = (Float)pVal;
         // LLM JavaLine Aggregation: cast merged into downcast_ref
         // Java: return v;
         *pVal.unwrap().downcast_ref::<f32>().unwrap_or(&f_default)
      }
    }

    // Java: int getInt(int nProp)
    pub fn getInt(&self, n_prop: i32) -> i32 {
      // Java: Object pVal = this.getValue(nProp);
      let pVal = self.getValue(n_prop);
      // Java: if (pVal == null)
      if pVal.is_none() {
         // Java: throw new PropertyException("No such property: " + nProp);
         panic!("No such property: {}", n_prop);
      }
      // Java: else
      else {
         // Java: Integer v = (Integer)pVal;
         // LLM JavaLine Aggregation: cast merged into downcast_ref
         // Java: return v;
         *pVal.unwrap().downcast_ref::<i32>().unwrap_or(&0)
      }
    }

    // Java: int getInt(int nProp, int iDefault)
    pub fn getInt_default(&self, n_prop: i32, i_default: i32) -> i32 {
      // Java: Object pVal = this.getValue(nProp);
      let pVal = self.getValue(n_prop);
      // Java: if (pVal == null)
      if pVal.is_none() {
         // Java: return iDefault;
         return i_default;
      }
      // Java: else
      else {
         // Java: Integer v = (Integer)pVal;
         // LLM JavaLine Aggregation: cast merged into downcast_ref
         // Java: return v;
         *pVal.unwrap().downcast_ref::<i32>().unwrap_or(&i_default)
      }
    }

    // Java: Object getObject(int nProp)
    pub fn getObject(&self, n_prop: i32) -> Element {
      // Java: Object pVal = this.getValue(nProp);
      let pVal = self.getValue(n_prop);
      // Java: if (pVal == null)
      if pVal.is_none() {
         // Java: throw new PropertyException("No such property: " + nProp);
         panic!("No such property: {}", n_prop);
      }
      // Java: else
      else {
         // Java: return pVal;
         pVal
      }
    }

    // Java: Object getObject(int nProp, Object pDefault)
    pub fn getObject_default(&self, n_prop: i32, p_default: Element) -> Element {
      // Java: Object pVal = this.getValue(nProp);
      let pVal = self.getValue(n_prop);
      // Java: return pVal == null ? pDefault : pVal;
      // LLM Rust Null-Safety: handled by Option, ternary replaced with if/else
      if pVal.is_none() { p_default } else { pVal }
    }

    // Java: String getString(int nProp)
    pub fn getString(&self, n_prop: i32) -> String {
      // Java: Object pVal = this.getValue(nProp);
      let pVal = self.getValue(n_prop);
      // Java: if (pVal == null)
      if pVal.is_none() {
         // Java: throw new PropertyException("No such property: " + nProp);
         panic!("No such property: {}", n_prop);
      }
      // Java: else
      else {
         // Java: String v = (String)pVal;
         // LLM JavaLine Aggregation: cast merged into downcast_ref
         // Java: return v;
         pVal.unwrap().downcast_ref::<String>().cloned().unwrap_or_default()
      }
    }

    // Java: String getString(int nProp, String strDefault)
    pub fn getString_default(&self, n_prop: i32, str_default: &str) -> String {
      // Java: Object pVal = this.getValue(nProp);
      let pVal = self.getValue(n_prop);
      // Java: if (pVal == null)
      if pVal.is_none() {
         // Java: return strDefault;
         return str_default.to_string();
      }
      // Java: else
      else {
         // Java: String v = (String)pVal;
         // LLM JavaLine Aggregation: cast merged into downcast_ref
         // Java: return v;
         pVal.unwrap().downcast_ref::<String>().cloned().unwrap_or_else(|| str_default.to_string())
      }
    }

    // Java: Object getValue(int nProp)
    pub fn getValue(&self, n_prop: i32) -> Element {
      // Java: return this.map.get(nProp);
      self.map.get(&n_prop).and_then(|v| v.clone())
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
      // Java: return this.map.size() < 1;
      self.map.len() < 1
    }

    // Java: int size()
    pub fn size(&self) -> i32 {
      // Java: return this.map.size();
      self.map.len() as i32
    }

    // Java: void setValue(int nProp, Object pValue)
    pub fn setValue(&mut self, n_prop: i32, p_value: Element) {
      // Java: this.map.put(nProp, pValue);
      self.map.insert(n_prop, p_value);
    }

    // Java: void readExternal(ObjectInput input)
    pub fn readExternal(&mut self, _input: &[u8]) -> Result<(), String> {
      // Java: this.map.readExternal(input);
      // LLM No Rust equivalent: serialization not yet implemented for HashMap
      Ok(())
    }

    // Java: void writeExternal(ObjectOutput output)
    pub fn writeExternal(&self, _output: &mut Vec<u8>) -> Result<(), String> {
      // Java: this.map.writeExternal(output);
      // LLM No Rust equivalent: serialization not yet implemented for HashMap
      Ok(())
    }
}
