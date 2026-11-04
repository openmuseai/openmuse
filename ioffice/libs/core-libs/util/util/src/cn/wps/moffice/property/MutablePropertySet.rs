// Java: import cn.wps.base.assertion.Assert
use crate::base::assertion::Assert::Assert;
use super::PropertySet::{PropertySet, PropertyValue};
use std::sync::Arc;
use stubs::java::lang::Object::Object;

// Java: public class MutablePropertySet implements Cloneable
#[derive(Default)]
pub struct MutablePropertySet {
    // Java field: PropertySet data
    pub(crate) data: PropertySet,
}

// Java: public class MutablePropertySet implements Cloneable
impl MutablePropertySet {
    // Java: MutablePropertySet()
    pub fn new() -> Self {
        MutablePropertySet { data: PropertySet::new() }
    }

    // Java: MutablePropertySet(MutablePropertySet prop)
    pub fn new_from_mutable(prop: &MutablePropertySet) -> Self {
        let mut mps = Self::new();
        mps.add_mutable(prop);
        mps
    }

    // Java: MutablePropertySet(PropertySet prop)
    pub fn new_from_ps(prop: &PropertySet) -> Self {
        let mut mps = Self::new();
        mps.add(prop);
        mps
    }

    pub fn _expandBucket(&mut self, nBucket: i32) {
        self.data._expandBucket(nBucket);
    }

    pub fn add_mutable(&mut self, props: &MutablePropertySet) {
        self.data._add(Some(&props.data));
    }

    // Java: void add(PropertySet props)
    pub fn add(&mut self, props: &PropertySet) {
        self.data._add(Some(props));
    }

    pub fn add_ps(&mut self, props: &PropertySet) {
        self.add(props);
    }

    pub fn addLimit(&mut self, prop: &PropertySet, b: i32, e: i32) {
        self.data._addLimit(Some(prop), b, e);
    }

    pub fn clear(&mut self) {
        self.data._clear();
    }

    pub fn clone_mps(&self) -> MutablePropertySet {
        MutablePropertySet { data: self.data.clone() }
    }

    pub fn contains(&self, nProp: i32) -> bool {
        self.data.contains(nProp)
    }

    pub fn differenceSet_mutable(&mut self, p2: &MutablePropertySet) {
        self.data._differenceSet(&p2.data);
    }

    pub fn differenceSet(&mut self, p2: &PropertySet) {
        self.data._differenceSet(p2);
    }

    // Java: PropertySet freeze() — 返回冻结后的拷贝（owned），供 compose/intersect/merge
    pub fn freeze(&mut self) -> PropertySet {
        self.data._freeze();
        self.data.clone()
    }

    pub fn get(&self) -> &PropertySet {
        &self.data
    }

    pub fn getAll(&self, ids: &mut Vec<i32>, _values: Option<&mut Vec<Option<PropertyValue>>>) -> i32 {
        self.data.getAll(ids, _values)
    }

    pub fn getBoolean(&self, nProp: i32) -> bool {
        self.data.getBoolean(nProp)
    }

    pub fn getBoolean_with_default(&self, nProp: i32, bDefault: bool) -> bool {
        self.data.getBoolean_default(nProp, bDefault)
    }

    pub fn getFloat(&self, nProp: i32) -> f32 {
        self.data.getFloat(nProp)
    }

    pub fn getFloat_with_default(&self, nProp: i32, fDefault: f32) -> f32 {
        self.data.getFloat_default(nProp, fDefault)
    }

    pub fn getInt(&self, nProp: i32) -> i32 {
        self.data.getInt(nProp)
    }

    pub fn getInt_with_default(&self, nProp: i32, iDefault: i32) -> i32 {
        self.data.getInt_default(nProp, iDefault)
    }

    pub fn getObject(&self, nProp: i32) -> Option<&PropertyValue> {
        // Java getObject throws if missing; MPS Option wrapper keeps prior call sites
        Some(self.data.getObject(nProp))
    }

    pub fn getObject_with_default<'a>(&'a self, nProp: i32, _p_default: &'a dyn Object) -> Option<&'a PropertyValue> {
        Some(self.data.getObject(nProp))
    }

    pub fn getString(&self, nProp: i32) -> String {
        self.data.getString(nProp)
    }

    pub fn getString_with_default(&self, nProp: i32, strDefault: &str) -> String {
        self.data.getString_default(nProp, strDefault)
    }

    pub fn getValue(&self, nProp: i32) -> Option<&PropertyValue> {
        self.data.getValue(nProp)
    }

    pub fn intersect_mutable(&mut self, p2: &MutablePropertySet) {
        self.data._intersect(&p2.data);
    }

    pub fn intersect(&mut self, p2: &PropertySet) {
        self.data._intersect(p2);
    }

    pub fn isEmpty(&self) -> bool {
        self.data.isEmpty()
    }

    pub fn remove(&mut self, nProp: i32) {
        self.setValue(nProp, None);
    }

    pub fn setFloat_primitive(&mut self, nProp: i32, nVal: f32) {
        self.setFloat(nProp, nVal);
    }

    pub fn setInt_primitive(&mut self, nProp: i32, nVal: i32) {
        self.setInt(nProp, nVal);
    }

    pub fn setString(&mut self, nProp: i32, str_val: String) {
        self.setValue(nProp, Some(Arc::new(str_val)));
    }

    pub fn setValue(&mut self, nProp: i32, pValue: Option<PropertyValue>) {
        match pValue {
            Some(v) => self.data._setValue(nProp, Some(v)),
            None => {
                self.data._setValue(nProp, None);
            }
        }
    }

    pub fn setBoolean(&mut self, nProp: i32, b_val: bool) {
        self.setValue(nProp, Some(Arc::new(b_val)));
    }

    pub fn setFloat(&mut self, nProp: i32, n_val: f32) {
        self.setValue(nProp, Some(Arc::new(n_val)));
    }

    pub fn setInt(&mut self, nProp: i32, n_val: i32) {
        self.setValue(nProp, Some(Arc::new(n_val)));
    }

    pub fn size(&self) -> i32 {
        self.data.size()
    }
}
