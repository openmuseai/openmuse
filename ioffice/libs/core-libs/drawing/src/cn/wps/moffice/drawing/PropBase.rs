// Java: import cn.wps.moffice.property.MutablePropertyMap
use core_libs_util::cn::wps::moffice::property::MutablePropertyMap::{MutablePropertyMap, OnModifiedListenerTrait};
// Java: import java.io.Externalizable
// Java: import java.io.IOException
// Java: import java.io.ObjectInput
// Java: import java.io.ObjectOutput

use crate::cn::wps::moffice::drawing::IShapePool::IShapePool;

use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

type Element = Option<Arc<dyn Any + Send + Sync>>;
#[derive(Clone)]
// Java: public class PropBase implements Cloneable, Externalizable
pub struct PropBase {
    // Java field: MutablePropertyMap mProperty
    pub mProperty: Option<MutablePropertyMap>,
    pub _property_map: HashMap<i32, Element>,
    pub shape_pool: Option<Arc<dyn IShapePool + Send + Sync>>,
}

pub trait Cloneable {}
impl Cloneable for PropBase {}

pub trait Externalizable {}
impl Externalizable for PropBase {}

// Java: public class PropBase implements Cloneable, Externalizable
impl PropBase {
    // Java: PropBase()
    pub fn new() -> Self {
      // Java: super();
      // LLM JavaLine Aggregation: no super() needed in Rust
      // Java: this.setProperty(new MutablePropertyMap());
      // LLM JavaLine Aggregation: merged into struct initialization; self not available in constructor
      PropBase { mProperty: Some(MutablePropertyMap::new()), _property_map: HashMap::new(), shape_pool: None }
    }

    // Java: PropBase(int capacity)
    pub fn new_with_capacity(capacity: usize) -> Self {
      // Java: super();
      // LLM JavaLine Aggregation: no super() needed in Rust
      // Java: this.setProperty(new MutablePropertyMap(capacity));
      // LLM JavaLine Aggregation: merged into struct initialization; self not available in constructor
      PropBase { mProperty: Some(MutablePropertyMap::new_with_capacity(capacity)), _property_map: HashMap::with_capacity(capacity), shape_pool: None }
    }

    // Java: PropBase clone()
    pub fn clone(&mut self) -> Result<PropBase, String> {
      // Java: PropBase shape = (PropBase)super.clone();
      let mut shape = PropBase { mProperty: None, _property_map: HashMap::new(), shape_pool: None };
      // Java: shape.setProperty(null != this.getProperty() ? this.getProperty().clone() : null);
      if self.mProperty.is_some() {
          shape.mProperty = Some(self.getProperty().clone_map());
      }
      // Java: shape.setListener((MutablePropertyMap.OnModifiedListener)null);
      if let Some(ref mut prop) = shape.mProperty {
          prop.setListener(None);
      }
      // Java: return shape;
      Ok(shape)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: Option<&PropBase>) -> bool {
      // Java: if (o == null)
      if o.is_none() {
         // Java: return false;
         return false;
      }
      // Java: else
      else {
         // Java: return o == this ? true : this.mProperty.equals(((PropBase)o).getProperty());
         let o = o.unwrap();
         if std::ptr::eq(self, o) {
             return true;
         }
         match (&self.mProperty, &o.mProperty) {
             (Some(a), Some(b)) => a.base.equals(Some(&b.base)),
             (None, None) => true,
             _ => false,
         }
      }
    }

    // Java: void dispose()
    pub fn dispose(&mut self) {
      // Java: if (null != this.mProperty)
      if self.mProperty.is_some() {
         // Java: this.mProperty.dispose();
         if let Some(ref mut prop) = self.mProperty {
             prop.base.dispose();
         }
         // Java: this.mProperty = null;
         self.mProperty = None;
      }
    }

    // Java: boolean contains(int nProp)
    pub fn contains(&mut self, n_prop: i32) -> bool {
      // Java: return this.getProperty().contains(nProp);
      self.getProperty().base.contains(n_prop)
    }

    // Java: Object getValue(int nProp)
    pub fn getValue(&mut self, n_prop: i32) -> Element {
      // Java: return this.getProperty().getValue(nProp);
      self.getProperty().base.getValue(n_prop)
    }

    pub fn setValue(&mut self, n_prop: i32, value: Element) {
      // Java: this.getProperty().setValue(nProp, value);
      self.setValue(n_prop, value);
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&mut self) -> bool {
      // Java: return this.getProperty().isEmpty();
      self.getProperty().base.isEmpty()
    }

    // Java: MutablePropertyMap getProperty()
    pub fn getProperty(&mut self) -> &MutablePropertyMap {
      // Java: MutablePropertyMap prop = this.mProperty;
      // LLM JavaLine Aggregation: prop is self.mProperty reference
      // Java: if (this instanceof Shape)
      if self.shape_pool.is_some() {
         // Java: IShapePool shapePool = ((Shape)this).getShapePool();
         let shapePool = self.shape_pool.clone();
         // Java: if (shapePool != null)
         if let Some(ref shapePool) = shapePool {
            // Java: shapePool.lock();
            shapePool.lock();
            // Java: if (null == this.mProperty)
            if self.mProperty.is_none() {
               // Java: this.mProperty = shapePool.getProperty(this);
               let pool_result = shapePool.getProperty(None);
               if let Some(ref prop_arc) = pool_result {
                   if let Some(prop_map) = prop_arc.downcast_ref::<MutablePropertyMap>() {
                       self.mProperty = Some(prop_map.clone_map());
                   }
               }
            }
            // Java: prop = this.mProperty;
            let _prop = self.mProperty.as_ref();
            // Java: shapePool.unlock();
            shapePool.unlock();
         }
      }
      // Java: return prop;
      self.mProperty.as_ref().expect("mProperty is None")
    }


    // Java: MutablePropertyMap getProperty() 的可变借用版本
    pub fn getProperty_mut(&mut self) -> &mut MutablePropertyMap {
      // 1. 镜像原 getProperty() 的懒加载/动态初始化逻辑
      if self.shape_pool.is_some() {
         let shapePool = self.shape_pool.clone();
         if let Some(ref shapePool) = shapePool {
            shapePool.lock();
            if self.mProperty.is_none() {
               let pool_result = shapePool.getProperty(None);
               if let Some(ref prop_arc) = pool_result {
                   if let Some(prop_map) = prop_arc.downcast_ref::<MutablePropertyMap>() {
                       self.mProperty = Some(prop_map.clone_map());
                   }
               }
            }
            shapePool.unlock();
         }
      }
      // 2. 返回可变借用
      self.mProperty.as_mut().expect("mProperty is None")
    }

    // Java: void setProperty(MutablePropertyMap prop)
    pub fn setProperty(&mut self, prop: Option<MutablePropertyMap>) {
      // Java: if (prop != null)
      if prop.is_some() {
         // Java: this.mProperty = prop;
         self.mProperty = prop;
      }
    }

    // Java: void readExternal(ObjectInput input)
    pub fn readExternal(&mut self, input: &[u8]) -> Result<(), String> {
      // Java: this.mProperty.readExternal(input);
      if let Some(ref mut prop) = self.mProperty {
          let _ = prop.readExternal(input);
      }
      Ok(())
    }

    // Java: void writeExternal(ObjectOutput output)
    pub fn writeExternal(&self, output: &mut Vec<u8>) -> Result<(), String> {
      // Java: this.mProperty.writeExternal(output);
      if let Some(ref prop) = self.mProperty {
          let _ = prop.writeExternal(output);
      }
      Ok(())
    }

    // Java: void setListener(MutablePropertyMap.OnModifiedListener listener)
    pub fn setListener(&mut self, listener: Option<Box<dyn OnModifiedListenerTrait + Send + Sync>>) {
      // Java: this.mProperty.setListener(listener);
      if let Some(ref mut prop) = self.mProperty {
          prop.setListener(listener);
      }
    }

    // Java: MutablePropertyMap.OnModifiedListener getListener()
    pub fn getListener(&self) -> Option<&Box<dyn OnModifiedListenerTrait + Send + Sync>> {
      // Java: return this.mProperty.getListener();
      match &self.mProperty {
          Some(prop) => prop.getListener(),
          None => None,
      }
    }

}
