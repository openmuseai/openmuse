#![allow(non_snake_case)]

// Java: import cn.wps.base.assertion.KSAssert
// Java: import cn.wps.moffice.property.PropertySet
use util::moffice::property::PropertySet::{PropertySet, PropertyValue};
// Java: import cn.wps.moffice.util.KSLog
use util::moffice::util::KSLog::KSLog;
// Java: import java.util.HashMap
// Java: import java.util.Map
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
// Java: import cn.wps.moffice.writer.data.PropertyOperation
use crate::cn::wps::moffice::writer::data::PropertyOperation::PropertyOperation;
// Java: import java.lang.Object
use stubs::java::lang::Object::Object;

// java private static final String tag = "DefaultOperation";
const tag: &str = "DefaultOperation";

// java public class PropertyOperations
#[derive(Default)]
pub struct PropertyOperations;

/// HashMap key for Java Object values. Java's usual immutable scalar keys use
/// value equality; other Object keys use their Object.equals/hashCode behavior.
#[derive(Clone)]
struct ObjectKey(PropertyValue);

fn object_equals(left: &dyn Object, right: &dyn Object) -> bool {
    let a = left.as_any();
    let b = right.as_any();
    if let (Some(a), Some(b)) = (a.downcast_ref::<String>(), b.downcast_ref::<String>()) {
        return a == b;
    }
    if let (Some(a), Some(b)) = (a.downcast_ref::<i32>(), b.downcast_ref::<i32>()) {
        return a == b;
    }
    if let (Some(a), Some(b)) = (a.downcast_ref::<i64>(), b.downcast_ref::<i64>()) {
        return a == b;
    }
    if let (Some(a), Some(b)) = (a.downcast_ref::<bool>(), b.downcast_ref::<bool>()) {
        return a == b;
    }
    if let (Some(a), Some(b)) = (a.downcast_ref::<f32>(), b.downcast_ref::<f32>()) {
        return a.to_bits() == b.to_bits();
    }
    if let (Some(a), Some(b)) = (a.downcast_ref::<f64>(), b.downcast_ref::<f64>()) {
        return a.to_bits() == b.to_bits();
    }
    left.equals(right)
}

impl PartialEq for ObjectKey {
    fn eq(&self, other: &Self) -> bool {
        object_equals(self.0.as_ref(), other.0.as_ref())
    }
}
impl Eq for ObjectKey {}
impl Hash for ObjectKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let value = self.0.as_ref();
        let any = value.as_any();
        any.type_id().hash(state);
        if let Some(v) = any.downcast_ref::<String>() { v.hash(state); }
        else if let Some(v) = any.downcast_ref::<i32>() { v.hash(state); }
        else if let Some(v) = any.downcast_ref::<i64>() { v.hash(state); }
        else if let Some(v) = any.downcast_ref::<bool>() { v.hash(state); }
        else if let Some(v) = any.downcast_ref::<f32>() { v.to_bits().hash(state); }
        else if let Some(v) = any.downcast_ref::<f64>() { v.to_bits().hash(state); }
        else { value.hashCode().hash(state); }
    }
}

// java private static class DefaultOperation implements PropertyOperation
#[derive(Default)]
pub struct DefaultOperation {
    // java int op;
    pub(crate) op: i32,
    // java PropertySet arg;
    pub(crate) arg: PropertySet,
    // java Map<Object, Object> mHints;
    mHints: Option<HashMap<ObjectKey, PropertyValue>>,
}

// java public class PropertyOperations
impl PropertyOperations {
    // java public static PropertyOperation create(int op, PropertySet arg, Object hintKey, Object hintValue)
    pub fn create__int_propertyset_object_object(
        op: i32, arg: &PropertySet, hintKey: Option<PropertyValue>, hintValue: Option<PropertyValue>,
    ) -> Box<dyn PropertyOperation> {
        // java return new DefaultOperation(op, arg, hintKey, hintValue);
        Box::new(DefaultOperation::new__int_propertyset_object_object(op, arg, hintKey, hintValue))
    }

    // java public static PropertyOperation create(int op, int id, Object value, Object hintKey, Object hintValue)
    pub fn create__int_int_object_object_object<T: Object + Clone + Send + Sync>(
        op: i32, id: i32, value: &T, hintKey: Option<PropertyValue>, hintValue: Option<PropertyValue>,
    ) -> Box<dyn PropertyOperation> {
        // java return create(op, PropertySet.valueOf(id, value), hintKey, hintValue);
        let arg = PropertySet::valueOf(id, Arc::new(value.clone()));
        Self::create__int_propertyset_object_object(op, &arg, hintKey, hintValue)
    }

    // java public static PropertyOperation create(int op, PropertySet arg)
    pub fn create__int_propertyset(op: i32, arg: &PropertySet) -> Box<dyn PropertyOperation> {
        // java return create(op, arg, (Object)null, (Object)null);
        Self::create__int_propertyset_object_object(op, arg, None, None)
    }

    // java public static PropertyOperation create(int op, int id, Object value)
    pub fn create__int_int_object<T: Object + Clone + Send + Sync>(
        op: i32, id: i32, value: &T,
    ) -> Box<dyn PropertyOperation> {
        // java return create(op, id, value, (Object)null, (Object)null);
        Self::create__int_int_object_object_object(op, id, value, None, None)
    }

    // java public static PropertyOperation merge(int id, Object value)
    pub fn merge__int_object<T: Object + Clone + Send + Sync>(id: i32, value: &T) -> Box<dyn PropertyOperation> {
        // java return create(2, id, value);
        Self::create__int_int_object(2, id, value)
    }

    // java public static PropertyOperation merge(PropertySet arg)
    pub fn merge__propertyset(arg: &PropertySet) -> Box<dyn PropertyOperation> {
        // java return create(2, arg);
        Self::create__int_propertyset(2, arg)
    }

    // java public static PropertyOperation clear(int id, Object value)
    pub fn clear__int_object<T: Object + Clone + Send + Sync>(id: i32, value: &T) -> Box<dyn PropertyOperation> {
        // java return create(3, id, value);
        Self::create__int_int_object(3, id, value)
    }

    // java public static PropertyOperation clear(PropertySet arg)
    pub fn clear__propertyset(arg: &PropertySet) -> Box<dyn PropertyOperation> {
        // java return create(3, arg);
        Self::create__int_propertyset(3, arg)
    }

    // java public static PropertyOperation clear(PropertySet arg, Object hintKey, Object hintValue)
    pub fn clear__propertyset_object_object(
        arg: &PropertySet, hintKey: Option<PropertyValue>, hintValue: Option<PropertyValue>,
    ) -> Box<dyn PropertyOperation> {
        // java return create(3, arg, hintKey, hintValue);
        Self::create__int_propertyset_object_object(3, arg, hintKey, hintValue)
    }

    // java public static PropertyOperation replace(int id, Object value)
    pub fn replace__int_object<T: Object + Clone + Send + Sync>(id: i32, value: &T) -> Box<dyn PropertyOperation> {
        // java return create(1, id, value);
        Self::create__int_int_object(1, id, value)
    }

    // java public static PropertyOperation replace(PropertySet arg)
    pub fn replace__propertyset(arg: &PropertySet) -> Box<dyn PropertyOperation> {
        // java return create(4, arg);
        Self::create__int_propertyset(4, arg)
    }
}

// java private static class DefaultOperation implements PropertyOperation
impl DefaultOperation {
    // java public DefaultOperation(int op, PropertySet arg)
    pub fn new__int_propertyset(op: i32, arg: &PropertySet) -> Self {
        // java this.op = op;
        // java this.arg = arg;
        Self { op, arg: arg.clone(), mHints: None }
    }

    // java public DefaultOperation(int op, PropertySet arg, Object hintKey, Object hintValue)
    pub fn new__int_propertyset_object_object(
        op: i32, arg: &PropertySet, hintKey: Option<PropertyValue>, hintValue: Option<PropertyValue>,
    ) -> Self {
        // java this(op, arg);
        let mut result = Self::new__int_propertyset(op, arg);
        // java if (hintKey != null) this.setHint(hintKey, hintValue);
        if let Some(key) = hintKey {
            result.setHint(key, hintValue.expect("The hint value should never be null"));
        }
        result
    }

    // java public int getOp()
    pub fn getOp(&self) -> i32 { self.op }
    // java public void setOp(int op)
    pub fn setOp(&mut self, op: i32) { self.op = op; }
    // java public PropertySet getOperand()
    pub fn getOperand(&self) -> &PropertySet { &self.arg }

    // java public Object getHint(Object key)
    pub fn getHint(&self, key: &dyn Object) -> Option<&dyn Object> {
        // java KSAssert.assertNotNull("The hint key should never be null", key);
        // java return this.mHints == null ? null : this.mHints.get(key);
        self.mHints.as_ref()?.iter()
            .find(|(stored, _)| object_equals(stored.0.as_ref(), key))
            .map(|(_, value)| value.as_ref() as &dyn Object)
    }

    // java public void setHint(Object key, Object value)
    pub fn setHint(&mut self, key: PropertyValue, value: PropertyValue) {
        // java KSAssert.assertNotNull("The hint key should never be null", key);
        // java KSAssert.assertNotNull("The hint value should never be null", value);
        // java if (this.mHints == null) this.mHints = new HashMap();
        // java this.mHints.put(key, value);
        self.mHints.get_or_insert_with(HashMap::new).insert(ObjectKey(key), value);
    }

    // java public void removeHint(Object key)
    pub fn removeHint(&mut self, key: &dyn Object) {
        // java KSAssert.assertNotNull("The hint key should never be null", key);
        // java if (this.mHints == null) KSLog.w(...);
        // java else Object value = this.mHints.remove(key);
        // java if (value == null) KSLog.w(...);
        let Some(hints) = self.mHints.as_mut() else {
            KSLog::w(tag, "Trying to remove a nonexistent hint");
            return;
        };
        let found = hints.keys().find(|stored| object_equals(stored.0.as_ref(), key)).cloned();
        if let Some(found) = found {
            hints.remove(&found);
        } else {
            KSLog::w(tag, "Trying to remove a nonexistent hint");
        }
    }
}

impl PropertyOperation for DefaultOperation {
    fn getOp(&self) -> i32 { DefaultOperation::getOp(self) }
    fn setOp(&mut self, op: i32) { DefaultOperation::setOp(self, op); }
    fn getOperand(&self) -> &PropertySet { DefaultOperation::getOperand(self) }
    fn getHint(&self, key: &dyn Object) -> Option<&dyn Object> { DefaultOperation::getHint(self, key) }
    fn setHint(&mut self, key: PropertyValue, value: PropertyValue) { DefaultOperation::setHint(self, key, value); }
    fn removeHint(&mut self, key: &dyn Object) { DefaultOperation::removeHint(self, key); }
}
