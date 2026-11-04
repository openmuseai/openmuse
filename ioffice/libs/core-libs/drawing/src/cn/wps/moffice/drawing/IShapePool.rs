#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.moffice.property.MutablePropertyMap
// Java: import java.util.Collection

use std::sync::Arc;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public interface IShapePool
pub trait IShapePool {
    // Java: abstract Shape findShapeById(int var1)
    fn findShapeById(&self, var1: i32) -> Element;

    // Java: abstract void addToShapeMap(Integer var1, Shape var2)
    fn addToShapeMap(&mut self, var1: i32, var2: Element);

    // Java: abstract void delFromShapeMap(int var1)
    fn delFromShapeMap(&mut self, var1: i32);

    // Java: abstract Collection<? extends Shape> getAllShapesWithChild()
    fn getAllShapesWithChild(&self) -> Vec<Element>;

    // Java: abstract void propChanged(Shape var1, boolean var2)
    fn propChanged(&self, var1: Element, var2: bool);

    // Java: abstract MutablePropertyMap getProperty(PropBase var1)
    fn getProperty(&self, var1: Element) -> Element;

    // Java: abstract void dispose()
    fn dispose(&mut self);

    // Java: abstract void lock()
    fn lock(&self);

    // Java: abstract void unlock()
    fn unlock(&self);

}
