// Java: import java.io.IOException
// Java: import java.io.ObjectInput
// Java: import java.io.ObjectOutput

use std::any::Any;
use std::sync::Arc;
use std::ops::Deref;

use crate::cn::wps::moffice::property::PropertyMap::PropertyMap;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public interface OnModifiedListener
pub trait OnModifiedListenerTrait {
    // Java: abstract void onModified(boolean var1)
    fn onModified(&self, var1: bool);
}

// Java: public interface OnModifiedListener
pub struct OnModifiedListener;


// Java: public class MutablePropertyMap extends PropertyMap implements Cloneable
pub struct MutablePropertyMap {
    pub base: PropertyMap,
    // Java field: OnModifiedListener mListener
    pub(crate) mListener: Vec<Box<dyn OnModifiedListenerTrait + Send + Sync>>,
}

impl Deref for MutablePropertyMap {
    type Target = PropertyMap;
    fn deref(&self) -> &PropertyMap {
        &self.base
    }
}

pub trait Cloneable {}
impl Cloneable for MutablePropertyMap {}


// 2. 手动实现 Clone
impl Clone for MutablePropertyMap {
  fn clone(&self) -> Self {
      MutablePropertyMap {
          base: self.base.clone(), // 假设 PropertyMap 已经实现了 Clone
          mListener: Vec::new(),   // 💡 新克隆的 Map 默认不绑定任何外部监听器
      }
  }
}

// Java: public class MutablePropertyMap extends PropertyMap implements Cloneable
impl MutablePropertyMap {
    // Java: MutablePropertyMap()
    pub fn new() -> Self {
      // Java: super();
      // LLM JavaLine Aggregation: super() creates parent PropertyMap, merged into struct init
      MutablePropertyMap {
          base: PropertyMap::new(),
          mListener: Vec::new(),
      }
    }

    pub fn getBase(&mut self) -> &PropertyMap {
      &self.base
    }

    // Java: MutablePropertyMap(int capacity)
    pub fn new_with_capacity(capacity: usize) -> Self {
      // Java: super(capacity);
      // LLM JavaLine Aggregation: super(capacity) creates parent PropertyMap, merged into struct init
      MutablePropertyMap {
          base: PropertyMap::with_capacity(capacity),
          mListener: Vec::new(),
      }
    }

    // Java: MutablePropertyMap clone()
    pub fn clone_map(&self) -> MutablePropertyMap {
      // Java: return (MutablePropertyMap)super.clone();
      MutablePropertyMap {
          base: self.base.clone(),
          mListener: Vec::new(),
      }
    }

    // Java: void remove(int nProp)
    pub fn remove(&mut self, n_prop: i32) {
      // Java: if (this.contains(nProp))
      if self.base.contains(n_prop) {
         // Java: this.map.remove(nProp);
         self.base.map.remove(&n_prop);
         // Java: this.onModified(true);
         self.onModified(true);
      }
    }

    // Java: void setBoolean(int nProp, boolean bVal)
    pub fn setBoolean(&mut self, n_prop: i32, b_val: bool) {
      // Java: this.setValue(nProp, bVal);
      self.setValue(n_prop, Some(Arc::new(b_val)));
    }

    // Java: void setFloat(int nProp, float nVal)
    pub fn setFloat(&mut self, n_prop: i32, n_val: f32) {
      // Java: this.setValue(nProp, nVal);
      self.setValue(n_prop, Some(Arc::new(n_val)));
    }

    // Java: void setInt(int nProp, int nVal)
    pub fn setInt(&mut self, n_prop: i32, n_val: i32) {
      // Java: this.setValue(nProp, nVal);
      self.setValue(n_prop, Some(Arc::new(n_val)));
    }

    // Java: void setString(int nProp, String strVal)
    pub fn setString(&mut self, n_prop: i32, str_val: String) {
      // Java: this.setValue(nProp, strVal);
      self.setValue(n_prop, Some(Arc::new(str_val)));
    }

    // Java: void setValue(int nProp, Object pValue)
    pub fn setValue(&mut self, n_prop: i32, p_value: Element) {
      // Java: Object oldValue = this.getValue(nProp);
      let oldValue = self.base.getValue(n_prop);
      // Java: if (this.isPrimaryInstance(pValue))
      if self.isPrimaryInstance(&p_value) {
         // Java: if (!pValue.equals(oldValue))
         if !Self::elements_equal(&p_value, &oldValue) {
            // Java: this.map.put(nProp, pValue);
            self.base.map.insert(n_prop, p_value);
            // Java: this.onModified(true);
            self.onModified(true);
         }
      }
      // Java: else if (oldValue != pValue)
      else if !Self::elements_ref_equal(&oldValue, &p_value) {
         // Java: this.map.put(nProp, pValue);
         self.base.map.insert(n_prop, p_value);
         // Java: this.onModified(true);
         self.onModified(true);
      }
    }

    fn elements_equal(a: &Element, b: &Element) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(a_val), Some(b_val)) => {
                if let (Some(a_i), Some(b_i)) = (a_val.downcast_ref::<i32>(), b_val.downcast_ref::<i32>()) {
                    return a_i == b_i;
                }
                if let (Some(a_b), Some(b_b)) = (a_val.downcast_ref::<bool>(), b_val.downcast_ref::<bool>()) {
                    return a_b == b_b;
                }
                if let (Some(a_s), Some(b_s)) = (a_val.downcast_ref::<String>(), b_val.downcast_ref::<String>()) {
                    return a_s == b_s;
                }
                if let (Some(a_f), Some(b_f)) = (a_val.downcast_ref::<f32>(), b_val.downcast_ref::<f32>()) {
                    return a_f == b_f;
                }
                false
            }
            _ => false,
        }
    }

    fn elements_ref_equal(a: &Element, b: &Element) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(a_val), Some(b_val)) => Arc::ptr_eq(a_val, b_val),
            _ => false,
        }
    }

    // Java: boolean isPrimaryInstance(Object pValue)
    pub fn isPrimaryInstance(&self, p_value: &Element) -> bool {
      // Java: return pValue instanceof Integer || pValue instanceof Boolean || pValue instanceof String || pValue instanceof Float;
      match p_value {
          None => false,
          Some(v) => {
              v.downcast_ref::<i32>().is_some()
              || v.downcast_ref::<bool>().is_some()
              || v.downcast_ref::<String>().is_some()
              || v.downcast_ref::<f32>().is_some()
          }
      }
    }

    // Java: void readExternal(ObjectInput input)
    pub fn readExternal(&mut self, input: &[u8]) -> Result<(), String> {
      // Java: super.readExternal(input);
      let _ = self.base.readExternal(input);
      Ok(())
    }

    // Java: void writeExternal(ObjectOutput output)
    pub fn writeExternal(&self, output: &mut Vec<u8>) -> Result<(), String> {
      // Java: super.writeExternal(output);
      let _ = self.base.writeExternal(output);
      Ok(())
    }

    // Java: void setListener(OnModifiedListener listener)
    pub fn setListener(&mut self, listener: Option<Box<dyn OnModifiedListenerTrait + Send + Sync>>) {
      // Java: this.mListener = listener;
      self.mListener = match listener {
          Some(l) => vec![l],
          None => Vec::new(),
      };
    }

    // Java: OnModifiedListener getListener()
    pub fn getListener(&self) -> Option<&Box<dyn OnModifiedListenerTrait + Send + Sync>> {
      // Java: return this.mListener;
      self.mListener.first()
    }

    // Java: void onModified(boolean flag)
    pub fn onModified(&mut self, flag: bool) {
      // Java: if (null != this.mListener)
      if let Some(listener) = self.mListener.first() {
         // Java: this.mListener.onModified(flag);
         listener.onModified(flag);
      }
    }


    // Parent methods
        pub fn contains(&self, n_prop: i32) -> bool {
          self.base.contains(n_prop)
        }
    
        pub fn dispose(&mut self) {
          self.base.dispose();
        }
    
        // Java: boolean getBoolean(int nProp)
        pub fn getBoolean(&self, n_prop: i32) -> bool {
          self.base.getBoolean(n_prop)
        }
    
        // Java: float getFloat(int nProp)
        pub fn getFloat(&self, n_prop: i32) -> f32 {
          self.base.getFloat(n_prop)
        }
    
        // Java: float getFloat(int nProp, float fDefault)
        pub fn getFloat_default(&self, n_prop: i32, f_default: f32) -> f32 {
          self.base.getFloat_default(n_prop, f_default)
        }
    
        // Java: int getInt(int nProp)
        pub fn getInt(&self, n_prop: i32) -> i32 {
          self.base.getInt(n_prop)
        }
    
        // Java: int getInt(int nProp, int iDefault)
        pub fn getInt_default(&self, n_prop: i32, i_default: i32) -> i32 {
          self.base.getInt_default(n_prop, i_default)
        }
    
        // Java: Object getObject(int nProp)
        pub fn getObject(&self, n_prop: i32) -> Element {
          self.base.getObject(n_prop)
        }
    
        // Java: Object getObject(int nProp, Object pDefault)
        pub fn getObject_default(&self, n_prop: i32, p_default: Element) -> Element {
          self.base.getObject_default(n_prop, p_default)
        }
    
        // Java: String getString(int nProp)
        pub fn getString(&self, n_prop: i32) -> String {
          self.base.getString(n_prop)
        }
    
        // Java: String getString(int nProp, String strDefault)
        pub fn getString_default(&self, n_prop: i32, str_default: &str) -> String {
          self.base.getString_default(n_prop, str_default)
        }
    
        // Java: Object getValue(int nProp)
        pub fn getValue(&self, n_prop: i32) -> Element {
          self.base.getValue(n_prop)
        }

}
