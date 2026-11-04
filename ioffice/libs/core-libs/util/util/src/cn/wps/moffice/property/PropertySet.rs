// Java: import cn.wps.base.assertion.Assert
use crate::base::assertion::Assert::Assert;
// Java: import java.util.Arrays
// Java: import java.lang.Cloneable
use stubs::java::lang::Cloneable::*;
// Java: import cn.wps.moffice.property.IPropertySetDescription
use super::IPropertySetDescription::*;
// Java: import java.lang.Object
use stubs::java::lang::Object::*;
// Java: import cn.wps.moffice.property.MutablePropertySet
use super::MutablePropertySet::*;
// Java: import cn.wps.moffice.property.PropertyException
use super::PropertyException::*;
// Java: import cn.wps.moffice.property.CloneNotSupportedException
use stubs::java::lang::CloneNotSupportedException::*;

use std::sync::Arc;
pub type PropertyValue = Arc<dyn Object + Send + Sync>;
type PropCell = Option<PropertyValue>;
type PropBucket = Vec<PropCell>;
type PropMap = Vec<Option<PropBucket>>;

fn ptr_eq_opt(a: Option<&PropertyValue>, b: Option<&PropertyValue>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(x), Some(y)) => Arc::ptr_eq(x, y),
        _ => false,
    }
}

fn downcast_ref<T: 'static>(v: &PropertyValue) -> Option<&T> {
    v.as_ref().as_any().downcast_ref::<T>()
}

fn property_values_equal(a: &PropertyValue, b: &PropertyValue) -> bool {
    if Arc::ptr_eq(a, b) {
        return true;
    }
    if let (Some(x), Some(y)) = (downcast_ref::<i32>(a), downcast_ref::<i32>(b)) {
        return x == y;
    }
    if let (Some(x), Some(y)) = (downcast_ref::<bool>(a), downcast_ref::<bool>(b)) {
        return x == y;
    }
    if let (Some(x), Some(y)) = (downcast_ref::<f32>(a), downcast_ref::<f32>(b)) {
        return x == y;
    }
    if let (Some(x), Some(y)) = (downcast_ref::<String>(a), downcast_ref::<String>(b)) {
        return x == y;
    }
    a.equals(b.as_ref()) || PropertySet::isArrayEqual(a, b)
}

// java static final int BUCKET_SIZE_SHIFT = 3;
pub const BUCKET_SIZE_SHIFT: i32 = 3;
// java static final int BUCKET_SIZE = 8;
pub const BUCKET_SIZE: i32 = 8;
// java public static IPropertySetDescription debug_description = null;
pub static debug_description: std::sync::LazyLock<Option<Box<dyn IPropertySetDescription + Send + Sync>>> =
    std::sync::LazyLock::new(|| None);
// java public static final PropertySet empty = createEmpty();
pub static empty: std::sync::LazyLock<PropertySet> =
    std::sync::LazyLock::new(|| PropertySet::createEmpty());
// java static final int FREEZE_BIT = 30;
pub const FREEZE_BIT: i32 = 30;
// java static final int FREEZE_MASK = 1073741824;
pub const FREEZE_MASK: i32 = 1073741824;

// java public class PropertySet implements Cloneable
#[derive(Default, Clone)]
pub struct PropertySet {
    // java private int mCount;
    pub(crate) mCount: i32,
    // java private int mFlags;
    pub(crate) mFlags: i32,
    // java private int mHash;
    pub(crate) mHash: i32,
    // java private Object[][] mMap;
    pub(crate) mMap: Option<PropMap>,
}

// java public class PropertySet implements Cloneable
impl PropertySet {

    // java public static final PropertySet compose(PropertySet oldProp, PropertySet newProp, int mode)
    pub fn compose(oldProp: &PropertySet, newProp: &PropertySet, mode: i32) -> PropertySet {
      // java MutablePropertySet r = null;
      let mut r: Option<MutablePropertySet> = None;
      // java switch (mode)
      match mode {
        // java case 0:
        0 => {
          // java return oldProp;
          return oldProp.clone();
        }
        // java case 1:
        1 => {
          // java return newProp;
          return newProp.clone();
        }
        // java case 2:
        2 => {
          // java r = new MutablePropertySet();
          r = Some(MutablePropertySet::new());
          // java r.add(oldProp);
          r.as_mut().unwrap().add(oldProp);
          // java r.add(newProp);
          r.as_mut().unwrap().add(newProp);
          // java return r.freeze();
          return r.as_mut().unwrap().freeze();
        }
        // java case 3:
        3 => {
          // java r = new MutablePropertySet();
          r = Some(MutablePropertySet::new());
          // java r.add(oldProp);
          r.as_mut().unwrap().add(oldProp);
          // java int[] keys = new int[newProp.size()];
          let mut keys: Vec<i32> = vec![0; newProp.size() as usize];
          // java newProp.getAll(keys, (Object[])null);
          newProp.getAll(&mut keys, None);
          // java for (int key : keys)
          for key in keys.iter() {
            // java r.remove(key);
            r.as_mut().unwrap().remove(*key);
          }
          // java return r.freeze();
          return r.as_mut().unwrap().freeze();
        }
        // java default:
        _ => {
          // java return oldProp;
          return oldProp.clone();
        }
      }
    }

    // java private static final PropertySet createEmpty()
    pub fn createEmpty() -> PropertySet {
      // java PropertySet p = new PropertySet();
      let mut p: PropertySet = PropertySet::new();
      // java p._freeze();
      p._freeze();
      // java return p;
      return p;
    }

    // java public static final PropertySet intersect(PropertySet p1, PropertySet p2)
    pub fn intersect(p1: &PropertySet, p2: &PropertySet) -> PropertySet {
      // java if (p1.mMap != null && p2.mMap != null)
      if let (Some(p1_mMap), Some(p2_mMap)) = (p1.mMap.as_ref(), p2.mMap.as_ref()) {
        // java MutablePropertySet result = new MutablePropertySet();
        let mut result: MutablePropertySet = MutablePropertySet::new();
        // java int nBucket = Math.min(p1.mMap.length, p2.mMap.length);
        let nBucket: i32 = i32::min(p1_mMap.len() as i32, p2_mMap.len() as i32);
        let mut iBucket: i32 = 0;
        // java for (int iBucket = 0; iBucket < nBucket ++iBucket)
        while iBucket < nBucket {
          // java Object[] pb1 = p1.mMap[iBucket];
          let pb1: Option<&PropBucket> = p1_mMap[iBucket as usize].as_ref();
          // java Object[] pb2 = p2.mMap[iBucket];
          let pb2: Option<&PropBucket> = p2_mMap[iBucket as usize].as_ref();
          // java if (pb1 != null && pb2 != null)
          if let (Some(pb1), Some(pb2)) = (pb1, pb2) {
            let mut i: i32 = 0;
            // java for (int i = 0; i < 8 ++i)
            while i < 8 {
              // java Object v1 = pb1[i];
              let v1: Option<&PropertyValue> = pb1[i as usize].as_ref();
              // java Object v2 = pb2[i];
              let v2: Option<&PropertyValue> = pb2[i as usize].as_ref();
              // java if (v1 != null && v2 != null && v1.equals(v2))
              if let (Some(v1), Some(v2)) = (v1, v2) {
                if property_values_equal(v1, v2) {
                // java result.setValue((iBucket << 3) + i, v1);
                result.setValue((iBucket << 3) + i, Some((*v1).clone()));
                }
              }
              i += 1;
            }
          }
          iBucket += 1;
        }
        // java return result.freeze();
        return result.freeze();
      }
      // java else
      else {
        // java return empty;
        return (*empty).clone();
      }
    }

    // java static final boolean isArrayEqual(Object a1, Object a2)
    pub fn isArrayEqual(a1: &PropertyValue, a2: &PropertyValue) -> bool {
      // java if (!a1.getClass().isArray())
      let a1_is_arr = downcast_ref::<Vec<Option<PropertyValue>>>(a1).is_some()
          || downcast_ref::<Vec<bool>>(a1).is_some() || downcast_ref::<Vec<char>>(a1).is_some()
          || downcast_ref::<Vec<i8>>(a1).is_some() || downcast_ref::<Vec<i16>>(a1).is_some()
          || downcast_ref::<Vec<i32>>(a1).is_some() || downcast_ref::<Vec<i64>>(a1).is_some()
          || downcast_ref::<Vec<f32>>(a1).is_some() || downcast_ref::<Vec<f64>>(a1).is_some();
      if !a1_is_arr {
        // java return false;
        return false;
      }
      // java else if (!a2.getClass().isArray())
      else {
      let a2_is_arr = downcast_ref::<Vec<Option<PropertyValue>>>(a2).is_some()
          || downcast_ref::<Vec<bool>>(a2).is_some() || downcast_ref::<Vec<char>>(a2).is_some()
          || downcast_ref::<Vec<i8>>(a2).is_some() || downcast_ref::<Vec<i16>>(a2).is_some()
          || downcast_ref::<Vec<i32>>(a2).is_some() || downcast_ref::<Vec<i64>>(a2).is_some()
          || downcast_ref::<Vec<f32>>(a2).is_some() || downcast_ref::<Vec<f64>>(a2).is_some();
      if !a2_is_arr {
        // java return false;
        return false;
      }
      // java else if (a1.getClass() != a2.getClass())
      else if (downcast_ref::<Vec<Option<PropertyValue>>>(a1).is_some())
          != (downcast_ref::<Vec<Option<PropertyValue>>>(a2).is_some()) {
        // java return false;
        return false;
      }
      // java else if (a1.getClass().getComponentType() != a2.getClass().getComponentType())
      else if let (Some(x), Some(y)) = (
          downcast_ref::<Vec<Option<PropertyValue>>>(a1),
          downcast_ref::<Vec<Option<PropertyValue>>>(a2),
      ) {
        // java return false;
        // Object[] path (component both Object) — Arrays.equals semantics
        if x.len() != y.len() { return false; }
        let mut i = 0usize;
        while i < x.len() {
          if !Self::isObjectEqual(x[i].as_ref(), y[i].as_ref()) { return false; }
          i += 1;
        }
        return true;
      }
      // java else
      else {
        // java return a1.getClass().getComponentType().isPrimitive() ? primitiveArrayEqual(a1, a2) : Arrays.equals((Object[])a1, (Object[])a2);
        return Self::primitiveArrayEqual(a1, a2);
      }
      }
    }

    // java public static final boolean isObjectEqual(Object a, Object b)
    pub fn isObjectEqual(a: Option<&PropertyValue>, b: Option<&PropertyValue>) -> bool {
      // java if (a == b)
      if ptr_eq_opt(a, b) {
        // java return true;
        return true;
      }
      // java else if (a != null && b != null)
      else if let (Some(a), Some(b)) = (a, b) {
        // java return a.equals(b) || isArrayEqual(a, b);
        return property_values_equal(a, b);
      }
      // java else
      else {
        // java return false;
        return false;
      }
    }

    // java public static boolean isPropertyEqual(PropertySet a, PropertySet b)
    pub fn isPropertyEqual(a: Option<&PropertySet>, b: Option<&PropertySet>) -> bool {
      // java if (a == b)
      if match (a, b) {
        (Some(x), Some(y)) => std::ptr::eq(x as *const PropertySet, y as *const PropertySet),
        (None, None) => true,
        _ => false,
      } {
        // java return true;
        return true;
      }
      // java else if (a == null && b == null)
      else if a.is_none() && b.is_none() {
        // java return true;
        return true;
      }
      // java else
      else {
        // java return a != null ? a.equals(b) : b.equals(a);
        return if a.is_some() { a.unwrap().equals(b) } else { b.unwrap().equals(a) };
      }
    }

    // java public static final PropertySet merge(PropertySet a, PropertySet b)
    pub fn merge(a: &PropertySet, b: &PropertySet) -> PropertySet {
      // java MutablePropertySet pResult = new MutablePropertySet();
      let mut pResult: MutablePropertySet = MutablePropertySet::new();
      // java pResult.add(a);
      pResult.add(a);
      // java pResult.add(b);
      pResult.add(b);
      // java return pResult.freeze();
      return pResult.freeze();
    }

    // java private static boolean primitiveArrayEqual(Object a1, Object a2)
    pub fn primitiveArrayEqual(a1: &PropertyValue, a2: &PropertyValue) -> bool {
      // java Assert.assertNotNull("a1 should not be null!", a1);
      Assert::assertNotNull_msg("a1 should not be null!", &Some(()));
      // java Assert.assertNotNull("a2 should not be null!", a2);
      Assert::assertNotNull_msg("a2 should not be null!", &Some(()));
      // java Assert.assertTrue("a1.getClass().getComponentType().isPrimitive() should be true!", a1.getClass().getComponentType().isPrimitive());
      Assert::assertTrue_msg("a1.getClass().getComponentType().isPrimitive() should be true!", true);
      // java if (a1.getClass().getComponentType() != a2.getClass().getComponentType())
      let same_prim = (downcast_ref::<Vec<bool>>(a1).is_some() && downcast_ref::<Vec<bool>>(a2).is_some())
          || (downcast_ref::<Vec<char>>(a1).is_some() && downcast_ref::<Vec<char>>(a2).is_some())
          || (downcast_ref::<Vec<i8>>(a1).is_some() && downcast_ref::<Vec<i8>>(a2).is_some())
          || (downcast_ref::<Vec<i16>>(a1).is_some() && downcast_ref::<Vec<i16>>(a2).is_some())
          || (downcast_ref::<Vec<i32>>(a1).is_some() && downcast_ref::<Vec<i32>>(a2).is_some())
          || (downcast_ref::<Vec<i64>>(a1).is_some() && downcast_ref::<Vec<i64>>(a2).is_some())
          || (downcast_ref::<Vec<f32>>(a1).is_some() && downcast_ref::<Vec<f32>>(a2).is_some())
          || (downcast_ref::<Vec<f64>>(a1).is_some() && downcast_ref::<Vec<f64>>(a2).is_some());
      if !same_prim
          && !(downcast_ref::<Vec<bool>>(a1).is_none() && downcast_ref::<Vec<bool>>(a2).is_none()
               && downcast_ref::<Vec<char>>(a1).is_none() && downcast_ref::<Vec<char>>(a2).is_none()
               && downcast_ref::<Vec<i8>>(a1).is_none() && downcast_ref::<Vec<i8>>(a2).is_none()
               && downcast_ref::<Vec<i16>>(a1).is_none() && downcast_ref::<Vec<i16>>(a2).is_none()
               && downcast_ref::<Vec<i32>>(a1).is_none() && downcast_ref::<Vec<i32>>(a2).is_none()
               && downcast_ref::<Vec<i64>>(a1).is_none() && downcast_ref::<Vec<i64>>(a2).is_none()
               && downcast_ref::<Vec<f32>>(a1).is_none() && downcast_ref::<Vec<f32>>(a2).is_none()
               && downcast_ref::<Vec<f64>>(a1).is_none() && downcast_ref::<Vec<f64>>(a2).is_none()) {
        // java return false;
        return false;
      }
      // java else if (a1.getClass().getComponentType() == Boolean.TYPE)
      else if let (Some(x), Some(y)) = (downcast_ref::<Vec<bool>>(a1), downcast_ref::<Vec<bool>>(a2)) {
        // java return Arrays.equals((boolean[])a1, (boolean[])a2);
        return x == y;
      }
      // java else if (a1.getClass().getComponentType() == Character.TYPE)
      else if let (Some(x), Some(y)) = (downcast_ref::<Vec<char>>(a1), downcast_ref::<Vec<char>>(a2)) {
        // java return Arrays.equals((char[])a1, (char[])a2);
        return x == y;
      }
      // java else if (a1.getClass().getComponentType() == Byte.TYPE)
      else if let (Some(x), Some(y)) = (downcast_ref::<Vec<i8>>(a1), downcast_ref::<Vec<i8>>(a2)) {
        // java return Arrays.equals((byte[])a1, (byte[])a2);
        return x == y;
      }
      // java else if (a1.getClass().getComponentType() == Short.TYPE)
      else if let (Some(x), Some(y)) = (downcast_ref::<Vec<i16>>(a1), downcast_ref::<Vec<i16>>(a2)) {
        // java return Arrays.equals((short[])a1, (short[])a2);
        return x == y;
      }
      // java else if (a1.getClass().getComponentType() == Integer.TYPE)
      else if let (Some(x), Some(y)) = (downcast_ref::<Vec<i32>>(a1), downcast_ref::<Vec<i32>>(a2)) {
        // java return Arrays.equals((int[])a1, (int[])a2);
        return x == y;
      }
      // java else if (a1.getClass().getComponentType() == Long.TYPE)
      else if let (Some(x), Some(y)) = (downcast_ref::<Vec<i64>>(a1), downcast_ref::<Vec<i64>>(a2)) {
        // java return Arrays.equals((long[])a1, (long[])a2);
        return x == y;
      }
      // java else if (a1.getClass().getComponentType() == Float.TYPE)
      else if let (Some(x), Some(y)) = (downcast_ref::<Vec<f32>>(a1), downcast_ref::<Vec<f32>>(a2)) {
        // java return Arrays.equals((float[])a1, (float[])a2);
        return x == y;
      }
      // java else if (a1.getClass().getComponentType() == Double.TYPE)
      else if let (Some(x), Some(y)) = (downcast_ref::<Vec<f64>>(a1), downcast_ref::<Vec<f64>>(a2)) {
        // java return Arrays.equals((double[])a1, (double[])a2);
        return x == y;
      }
      // java else
      else {
        // java Assert.fail("It should not reach here!");
        Assert::fail_msg("It should not reach here!");
        // java return false;
        return false;
      }
    }

    // java public static PropertySet valueOf(int id, Object pValue)
    pub fn valueOf(id: i32, pValue: PropertyValue) -> Self {
      // java return new PropertySet(id, pValue);
      return PropertySet::new__int_object(id, pValue);
    }

    // java public static PropertySet valueOf(int[] ids, Object[] values)
    pub fn valueOf__intarr_objectarr(ids: &[i32], values: &[Option<PropertyValue>]) -> PropertySet {
      // java return new PropertySet(ids, values);
      return PropertySet::new__intarr_objectarr(ids, values);
    }

    // java   PropertySet()
    // rust: ★M-STAGE2 P1-D1 / T6 —— Java `public static final PropertySet empty = createEmpty();`
    //   本模块已有模块级 `pub static empty: LazyLock<PropertySet>`，但模板按
    //   **关联路径** 生成调用 `PropertySet::empty`（全仓库 50 处）⇒ 补关联函数
    //   作为同义门面（语义：冻结的空属性集，与 `createEmpty()` 一致）。
    pub fn empty() -> PropertySet {
        (*empty).clone()
    }

    pub fn new() -> Self {
      // java super();
      // java this.mCount = 0;
      Self { mCount: 0, mFlags: 0, mHash: 0, mMap: None }
      // java this.mFlags = 0;
      // java this.mMap = null;
    }

    // java @Deprecated public  PropertySet(int id, Object pValue)
    pub fn new__int_object(id: i32, pValue: PropertyValue) -> Self {
      // java super();
      // java this.mCount = 0;
      let mut this = Self { mCount: 0, mFlags: 0, mHash: 0, mMap: None };
      // java this.mFlags = 0;
      // java this.mMap = null;
      // java this._setValue(id, pValue);
      this._setValue(id, Some(pValue));
      // java this._freeze();
      this._freeze();
      this
    }

    // java @Deprecated public  PropertySet(int[] ids, Object[] values)
    pub fn new__intarr_objectarr(ids: &[i32], values: &[Option<PropertyValue>]) -> Self {
      // java this(ids, values, Math.min(ids.length, values.length));
      let this = Self::new__intarr_objectarr_int(ids, values, i32::min(ids.len() as i32, values.len() as i32));
      this
    }

    // java @Deprecated public  PropertySet(int[] ids, Object[] values, int n)
    pub fn new__intarr_objectarr_int(ids: &[i32], values: &[Option<PropertyValue>], n: i32) -> Self {
      // java super();
      // java this.mCount = 0;
      let mut this = Self { mCount: 0, mFlags: 0, mHash: 0, mMap: None };
      // java this.mFlags = 0;
      // java this.mMap = null;
      // java 
      let mut i: i32 = 0;
      while i < n {
        this._setValue(ids[i as usize], values[i as usize].clone());
        i += 1;
      }
      // java this._freeze();
      this._freeze();
      this
    }

    // java final void _add(PropertySet props)
    pub fn _add(&mut self, props: Option<&PropertySet>) {
      // java if (this.isFrozen())
      if self.isFrozen() {
        // java throw new PropertyException("This PropertySet is frozen");
        panic!("{}", PropertyException::new("This PropertySet is frozen"));
      }
      // java else if (props != null && props.size() != 0)
      else if let Some(props) = props {
        if props.size() != 0 {
          // java this._expandBucket(props.mMap.length);
          self._expandBucket(props.mMap.as_ref().map(|a| a.len()).unwrap_or(0) as i32);
          let mut iBucket: i32 = 0;
          // java for (int iBucket = 0; iBucket < props.mMap.length ++iBucket)
          while iBucket < (props.mMap.as_ref().map(|a| a.len()).unwrap_or(0) as i32) {
            // java Object[] pSrc = props.mMap[iBucket];
            let pSrc: Option<&PropBucket> = props.mMap.as_ref().unwrap()[iBucket as usize].as_ref();
            // java Object[] pDest = this.mMap[iBucket];
            // pDest resolved after possible create
            // java if (pSrc != null)
            if let Some(pSrc) = pSrc {
              // java if (pDest == null)
              if self.mMap.as_ref().unwrap()[iBucket as usize].is_none() {
                // java pDest = this.mMap[iBucket] = new Object[8];
                self.mMap.as_mut().unwrap()[iBucket as usize] = Some(vec![None; 8]);
              }
              let pDest = self.mMap.as_mut().unwrap()[iBucket as usize].as_mut().unwrap();
              let mut iOffset: i32 = 0;
              // java for (int iOffset = 0; iOffset < 8 ++iOffset)
              while iOffset < 8 {
                // java if (pSrc[iOffset] != null)
                if pSrc[iOffset as usize].is_some() {
                  // java if (pDest[iOffset] == null)
                  if pDest[iOffset as usize].is_none() {
                    // java ++this.mCount;
                    self.mCount += 1;
                  }
                  // java pDest[iOffset] = pSrc[iOffset];
                  pDest[iOffset as usize] = pSrc[iOffset as usize].clone();
                }
                iOffset += 1;
              }
            }
            iBucket += 1;
          }
          // java this.check_size();
          self.check_size();
        }
      }
    }

    // java final void _addLimit(PropertySet props, int beginId, int endId)
    pub fn _addLimit(&mut self, props: Option<&PropertySet>, beginId: i32, endId: i32) {
      // java if (props != null && props.size() != 0)
      if let Some(props) = props {
        if props.size() != 0 {
          // java int iBucket = beginId >> 3;
          let mut iBucket: i32 = beginId >> 3;
          // java int len = (endId >> 3) + 1;
          let mut len: i32 = (endId >> 3) + 1;
          // java if (props.mMap.length < len)
          if (props.mMap.as_ref().map(|a| a.len()).unwrap_or(0) as i32) < len {
            // java len = props.mMap.length;
            len = props.mMap.as_ref().map(|a| a.len()).unwrap_or(0) as i32;
          }
          // java this._expandBucket(len);
          self._expandBucket(len);
          // java for (iBucket < len ++iBucket)
          while iBucket < len {
            // java Object[] pSrc = props.mMap[iBucket];
            let pSrc: Option<&PropBucket> = props.mMap.as_ref().unwrap()[iBucket as usize].as_ref();
            // java Object[] pDest = this.mMap[iBucket];
            // java if (pSrc != null)
            if let Some(pSrc) = pSrc {
              // java if (pDest == null)
              if self.mMap.as_ref().unwrap()[iBucket as usize].is_none() {
                // java pDest = this.mMap[iBucket] = new Object[8];
                self.mMap.as_mut().unwrap()[iBucket as usize] = Some(vec![None; 8]);
              }
              let pDest = self.mMap.as_mut().unwrap()[iBucket as usize].as_mut().unwrap();
              let mut iOffset: i32 = 0;
              // java for (int iOffset = 0; iOffset < 8 ++iOffset)
              while iOffset < 8 {
                // java int this_id = iBucket * 8 + iOffset;
                let this_id: i32 = iBucket * 8 + iOffset;
                // java if (this_id >= beginId && this_id < endId && pSrc[iOffset] != null)
                if this_id >= beginId && this_id < endId && pSrc[iOffset as usize].is_some() {
                  // java if (pDest[iOffset] == null)
                  if pDest[iOffset as usize].is_none() {
                    // java ++this.mCount;
                    self.mCount += 1;
                  }
                  // java pDest[iOffset] = pSrc[iOffset];
                  pDest[iOffset as usize] = pSrc[iOffset as usize].clone();
                }
                iOffset += 1;
              }
            }
            iBucket += 1;
          }
          // java this.check_size();
          self.check_size();
        }
      }
    }

    // java final void _clear()
    pub fn _clear(&mut self) {
      // java if (this.isFrozen())
      if self.isFrozen() {
        // java throw new PropertyException("This PropertySet is frozen");
        panic!("{}", PropertyException::new("This PropertySet is frozen"));
      }
      // java else if (null != this.mMap)
      else if let Some(mMap) = self.mMap.as_mut() {
        let mut i: i32 = 0;
        // java for (int i = 0; i < this.mMap.length ++i)
        while i < (mMap.len() as i32) {
          // java Object[] objs = this.mMap[i];
          let objs: Option<&mut PropBucket> = mMap[i as usize].as_mut();
          // java if (null != objs)
          if let Some(objs) = objs {
            let mut j: i32 = 0;
            // java for (int j = 0; j < 8 ++j)
            while j < 8 {
              // java objs[j] = null;
              objs[j as usize] = None;
              j += 1;
            }
          }
          // java this.mMap[i] = null;
          mMap[i as usize] = None;
          i += 1;
        }
        // java this.mCount = 0;
        self.mCount = 0;
        // java this.check_size();
        self.check_size();
      }
    }

    // java public void _differenceSet(PropertySet p2)
    pub fn _differenceSet(&mut self, p2: &PropertySet) {
      // java if (this.mMap != null)
      if let Some(mMap) = self.mMap.as_mut() {
        // java if (p2.mMap != null)
        if let Some(p2_mMap) = p2.mMap.as_ref() {
          // java int nBucket = Math.min(this.mMap.length, p2.mMap.length);
          let nBucket: i32 = i32::min(mMap.len() as i32, p2_mMap.len() as i32);
          let mut iBucket: i32 = 0;
          // java for (int iBucket = 0; iBucket < nBucket ++iBucket)
          while iBucket < nBucket {
            // java Object[] pb1 = this.mMap[iBucket];
            let pb1_is_some = mMap[iBucket as usize].is_some();
            // java Object[] pb2 = p2.mMap[iBucket];
            let pb2: Option<&PropBucket> = p2_mMap[iBucket as usize].as_ref();
            // java if (pb1 != null && pb2 != null)
            if pb1_is_some {
              if let Some(pb2) = pb2 {
              let mut i: i32 = 0;
              // java for (int i = 0; i < 8 ++i)
              while i < 8 {
                // java Object v1 = pb1[i];
                let v1 = mMap[iBucket as usize].as_ref().unwrap()[i as usize].clone();
                // java Object v2 = pb2[i];
                let v2 = pb2[i as usize].as_ref();
                // java if (v1 != null && v2 != null && isObjectEqual(v1, v2))
                if v1.is_some() && v2.is_some() && Self::isObjectEqual(v1.as_ref(), v2) {
                  // java pb1[i] = null;
                  mMap[iBucket as usize].as_mut().unwrap()[i as usize] = None;
                  // java --this.mCount;
                  self.mCount -= 1;
                }
                i += 1;
              }
              }
            }
            iBucket += 1;
          }
          // java this.check_size();
          self.check_size();
        }
      }
    }

    // java final void _expandBucket(int nBucket)
    pub fn _expandBucket(&mut self, nBucket: i32) {
      // java int nOldBucket = this.mMap != null ? this.mMap.length : 0;
      let nOldBucket: i32 = if self.mMap.is_some() { self.mMap.as_ref().unwrap().len() as i32 } else { 0 };
      // java if (nBucket >= nOldBucket)
      if nBucket >= nOldBucket {
        // java Object[][] pNew = new Object[nBucket][];
        let mut pNew: PropMap = vec![None; nBucket as usize];
        let mut iBucket: i32 = 0;
        // java for (int iBucket = 0; iBucket < nOldBucket ++iBucket)
        while iBucket < nOldBucket {
          // java pNew[iBucket] = this.mMap[iBucket];
          pNew[iBucket as usize] = self.mMap.as_mut().unwrap()[iBucket as usize].take();
          iBucket += 1;
        }
        // java this.mMap = pNew;
        self.mMap = Some(pNew);
      }
    }

    // java protected void _freeze()
    pub fn _freeze(&mut self) {
      // java if (!this.isFrozen())
      if !self.isFrozen() {
        // java this.setFrozen();
        self.setFrozen();
        // java this.mHash = 0;
        self.mHash = 0;
      }
    }

    // java final void _intersect(PropertySet p2)
    pub fn _intersect(&mut self, p2: &PropertySet) {
      // java if (this.mMap != null)
      if self.mMap.is_some() {
        // java if (p2.mMap == null)
        if p2.mMap.is_none() {
          // java this.mMap = null;
          self.mMap = None;
          // java this.mCount = 0;
          self.mCount = 0;
        }
        // java else
        else {
          // java int nBucket = Math.min(this.mMap.length, p2.mMap.length);
          let nBucket: i32 = i32::min(self.mMap.as_ref().unwrap().len() as i32, p2.mMap.as_ref().unwrap().len() as i32);
          let mut iBucket: i32 = 0;
          // java for (int iBucket = 0; iBucket < nBucket ++iBucket)
          while iBucket < nBucket {
            // java Object[] pb1 = this.mMap[iBucket];
            let pb1_some = self.mMap.as_ref().unwrap()[iBucket as usize].is_some();
            // java Object[] pb2 = p2.mMap[iBucket];
            let pb2: Option<&PropBucket> = p2.mMap.as_ref().unwrap()[iBucket as usize].as_ref();
            // java if (pb1 != null)
            if pb1_some {
              // java if (pb2 == null)
              if pb2.is_none() {
                // java this.mCount -= this.count(this.mMap[iBucket]);
                self.mCount -= self.count(self.mMap.as_ref().unwrap()[iBucket as usize].as_ref().unwrap());
                // java this.mMap[iBucket] = null;
                self.mMap.as_mut().unwrap()[iBucket as usize] = None;
              }
              // java else
              else {
                let mut i: i32 = 0;
                // java for (int i = 0; i < 8 ++i)
                while i < 8 {
                  // java Object v1 = pb1[i];
                  let v1 = self.mMap.as_ref().unwrap()[iBucket as usize].as_ref().unwrap()[i as usize].clone();
                  // java Object v2 = pb2[i];
                  let v2 = pb2.unwrap()[i as usize].as_ref();
                  // java if (v1 != null)
                  if v1.is_some() {
                    // java if (v2 == null)
                    if v2.is_none() {
                      // java pb1[i] = null;
                      self.mMap.as_mut().unwrap()[iBucket as usize].as_mut().unwrap()[i as usize] = None;
                      // java --this.mCount;
                      self.mCount -= 1;
                    }
                    // java else if (!isObjectEqual(v1, v2))
                    else if !Self::isObjectEqual(v1.as_ref(), v2) {
                      // java pb1[i] = null;
                      self.mMap.as_mut().unwrap()[iBucket as usize].as_mut().unwrap()[i as usize] = None;
                      // java --this.mCount;
                      self.mCount -= 1;
                    }
                  }
                  i += 1;
                }
              }
            }
            iBucket += 1;
          }
          let mut iBucket: i32 = nBucket;
          // java for (int iBucket = nBucket; iBucket < this.mMap.length ++iBucket)
          while iBucket < (self.mMap.as_ref().unwrap().len() as i32) {
            // java Object[] pb1 = this.mMap[iBucket];
            let pb1_some = self.mMap.as_ref().unwrap()[iBucket as usize].is_some();
            // java if (pb1 != null)
            if pb1_some {
              let mut i: i32 = 0;
              // java for (int i = 0; i < pb1.length ++i)
              let pb_len = self.mMap.as_ref().unwrap()[iBucket as usize].as_ref().unwrap().len() as i32;
              while i < pb_len {
                // java Object v1 = pb1[i];
                let v1_some = self.mMap.as_ref().unwrap()[iBucket as usize].as_ref().unwrap()[i as usize].is_some();
                // java if (v1 != null)
                if v1_some {
                  // java --this.mCount;
                  self.mCount -= 1;
                  // java pb1[i] = null;
                  self.mMap.as_mut().unwrap()[iBucket as usize].as_mut().unwrap()[i as usize] = None;
                }
                i += 1;
              }
              // java this.mMap[iBucket] = null;
              self.mMap.as_mut().unwrap()[iBucket as usize] = None;
            }
            iBucket += 1;
          }
          // java this.check_size();
          self.check_size();
        }
      }
    }

    // java final void _setValue(int nProp, Object pValue)
    pub fn _setValue(&mut self, nProp: i32, pValue: Option<PropertyValue>) {
      // java if (this.isFrozen())
      if self.isFrozen() {
        // java throw new PropertyException("This PropertySet is frozen");
        panic!("{}", PropertyException::new("This PropertySet is frozen"));
      }
      // java else
      else {
        // java int iBucket = nProp >> 3;
        let iBucket: i32 = nProp >> 3;
        // java int iOffset = nProp & 7;
        let iOffset: i32 = nProp & 7;
        // java this._expandBucket(iBucket + 1);
        self._expandBucket(iBucket + 1);
        // java if (this.mMap[iBucket] == null)
        if self.mMap.as_ref().unwrap()[iBucket as usize].is_none() {
          // java this.mMap[iBucket] = new Object[8];
          self.mMap.as_mut().unwrap()[iBucket as usize] = Some(vec![None; 8]);
        }
        // java Object[] pBucket = this.mMap[iBucket];
        let pBucket = self.mMap.as_mut().unwrap()[iBucket as usize].as_mut().unwrap();
        // java if (pBucket[iOffset] == null)
        if pBucket[iOffset as usize].is_none() {
          // java if (pValue != null)
          if pValue.is_some() {
            // java ++this.mCount;
            self.mCount += 1;
          }
        }
        // java else if (pValue == null)
        else if pValue.is_none() {
          // java --this.mCount;
          self.mCount -= 1;
        }
        // java pBucket[iOffset] = pValue;
        pBucket[iOffset as usize] = pValue;
        // java this.check_size();
        self.check_size();
      }
    }

    // java private void check_size()
    pub fn check_size(&mut self) {
    }

    // java public PropertySet clone()
    pub fn clone(&self) -> PropertySet {
      // java PropertySet propSet = (PropertySet)super.clone();
      let mut propSet = PropertySet { mCount: self.mCount, mFlags: self.mFlags, mHash: self.mHash, mMap: None };
      // java propSet.mMap = null;
      propSet.mMap = None;
      // java if (null != this.mMap)
      if let Some(mMap) = self.mMap.as_ref() {
        // java propSet.mMap = this.mMap.clone();
        propSet.mMap = Some(vec![None; mMap.len()]);
        // java int i = 0;
        let mut i: i32 = 0;
        let len: i32 = mMap.len() as i32;
        // java for (int len = this.mMap.length; i < len ++i)
        while i < len {
          // java if (null != this.mMap[i])
          if mMap[i as usize].is_some() {
            // java propSet.mMap[i] = this.mMap[i].clone();
            propSet.mMap.as_mut().unwrap()[i as usize] = mMap[i as usize].clone();
          }
          i += 1;
        }
      }
      // java return propSet;
      return propSet;
    }

    // java private int computeHash()
    pub fn computeHash(&self) -> i32 {
      // java if (this.mMap == null)
      if self.mMap.is_none() {
        // java return 0;
        return 0;
      }
      // java else
      else {
        // java int nHash = 0;
        let mut nHash: i32 = 0;
        let mut iBucket: i32 = 0;
        // java for (int iBucket = 0; iBucket < this.mMap.length ++iBucket)
        while iBucket < (self.mMap.as_ref().unwrap().len() as i32) {
          // java Object[] pBucket = this.mMap[iBucket];
          let pBucket: Option<&PropBucket> = self.mMap.as_ref().unwrap()[iBucket as usize].as_ref();
          // java if (pBucket != null)
          if let Some(pBucket) = pBucket {
            let mut i: i32 = 0;
            // java for (int i = 0; i < pBucket.length ++i)
            while i < (pBucket.len() as i32) {
              // java if (pBucket[i] != null)
              if pBucket[i as usize].is_some() {
                // java nHash += (iBucket << 3) + i;
                nHash = nHash + ((iBucket << 3) + i);
                // java nHash += pBucket[i].hashCode();
                nHash = nHash + pBucket[i as usize].as_ref().unwrap().hashCode();
              }
              i += 1;
            }
          }
          iBucket += 1;
        }
        // java return nHash;
        return nHash;
      }
    }

    // java public final boolean contain(PropertySet prop)
    pub fn contain(&self, prop: Option<&PropertySet>) -> bool {
      // java if (prop == null)
      let prop: &PropertySet = if prop.is_none() {
        // java prop = empty;
        &*empty
      } else {
        prop.unwrap()
      };
      // java int size = prop.size();
      let size: i32 = prop.size();
      // java int[] ids = new int[size];
      let mut ids: Vec<i32> = vec![0; size as usize];
      // java Object[] values1 = new Object[size];
      let mut values1: Vec<Option<PropertyValue>> = vec![None; size as usize];
      // java prop.getAll(ids, values1);
      prop.getAll(&mut ids, Some(&mut values1));
      // java Object[] values2 = new Object[size];
      let mut values2: Vec<Option<PropertyValue>> = vec![None; size as usize];
      let mut i: i32 = 0;
      // java for (int i = 0; i < size ++i)
      while i < size {
        // java values2[i] = this.getValue(ids[i]);
        values2[i as usize] = self.getValue(ids[i as usize]).cloned();
        i += 1;
      }
      let mut i: i32 = 0;
      // java for (int i = 0; i < size ++i)
      while i < size {
        // java Object obj1 = values1[i];
        let obj1 = values1[i as usize].as_ref();
        // java Object obj2 = values2[i];
        let obj2 = values2[i as usize].as_ref();
        // java if (obj1 == null)
        if obj1.is_none() {
          // java if (obj2 != null)
          if obj2.is_some() {
            // java return false;
            return false;
          }
        }
        // java else if (obj2 == null || !obj1.equals(obj2))
        else if obj2.is_none() || !property_values_equal(obj1.unwrap(), obj2.unwrap()) {
          // java return false;
          return false;
        }
        i += 1;
      }
      // java return true;
      return true;
    }

    // java public final boolean contains(int nProp)
    pub fn contains(&self, nProp: i32) -> bool {
      // java return this.getValue(nProp) != null;
      return self.getValue(nProp).is_some();
    }

    // java private int count(Object[] objects)
    pub fn count(&self, objects: &PropBucket) -> i32 {
      // java int n = 0;
      let mut n: i32 = 0;
      let mut i: i32 = 0;
      // java for (int i = 0; i < 8 ++i)
      while i < 8 {
        // java if (objects[i] != null)
        if objects[i as usize].is_some() {
          // java ++n;
          n += 1;
        }
        i += 1;
      }
      // java return n;
      return n;
    }

    // java public void dispose()
    pub fn dispose(&mut self) {
      // java this.mMap = null;
      self.mMap = None;
      // java this.mCount = 0;
      self.mCount = 0;
      // java this.check_size();
      self.check_size();
    }

    // java public boolean equals(Object r)
    pub fn equals(&self, r: Option<&PropertySet>) -> bool {
      // java if (r == this)
      if match r { Some(x) => std::ptr::eq(x as *const PropertySet, self as *const PropertySet), None => false } {
        // java return true;
        return true;
      }
      // java else
      else {
        // java if (r == null)
        let rprop: &PropertySet = if r.is_none() {
          // java r = empty;
          &*empty
        } else {
          r.unwrap()
        };
        // java PropertySet rprop = (PropertySet)r;
        // r already PropertySet
        // java int size1 = this.size();
        let size1: i32 = self.size();
        // java int size2 = rprop.size();
        let size2: i32 = rprop.size();
        // java if (size1 != size2)
        if size1 != size2 {
          // java return false;
          return false;
        }
        // java else if (size1 == size2 && size1 == 0)
        else if size1 == size2 && size1 == 0 {
          // java return true;
          return true;
        }
        // java else
        else {
          // java int[] id1 = new int[size1];
          let mut id1: Vec<i32> = vec![0; size1 as usize];
          // java Object[] val1 = new Object[size1];
          let mut val1: Vec<Option<PropertyValue>> = vec![None; size1 as usize];
          // java int[] id2 = new int[size2];
          let mut id2: Vec<i32> = vec![0; size2 as usize];
          // java Object[] val2 = new Object[size2];
          let mut val2: Vec<Option<PropertyValue>> = vec![None; size2 as usize];
          // java this.getAll(id1, val1);
          self.getAll(&mut id1, Some(&mut val1));
          // java rprop.getAll(id2, val2);
          rprop.getAll(&mut id2, Some(&mut val2));
          let mut i: i32 = 0;
          // java for (int i = 0; i < size1 ++i)
          while i < size1 {
            // java if (id1[i] != id2[i])
            if id1[i as usize] != id2[i as usize] {
              // java return false;
              return false;
            }
            i += 1;
          }
          let mut i: i32 = 0;
          // java for (int i = 0; i < size1 ++i)
          while i < size1 {
            // java Object v1 = val1[i];
            let v1 = val1[i as usize].as_ref();
            // java Object v2 = val2[i];
            let v2 = val2[i as usize].as_ref();
            // java if (v1 == null || v2 == null)
            if v1.is_none() || v2.is_none() {
              // java return false;
              return false;
            }
            // java if (!v1.equals(v2))
            if !property_values_equal(v1.unwrap(), v2.unwrap()) {
              // java return false;
              return false;
            }
            i += 1;
          }
          // java return true;
          return true;
        }
      }
    }

    // java public final int getAll(int[] ids, Object[] values)
    pub fn getAll(&self, ids: &mut Vec<i32>, mut values: Option<&mut Vec<Option<PropertyValue>>>) -> i32 {
      // java int n = 0;
      let mut n: i32 = 0;
      // java int nBucket = this.mMap != null ? this.mMap.length : 0;
      let nBucket: i32 = if self.mMap.is_some() { self.mMap.as_ref().unwrap().len() as i32 } else { 0 };
      let mut iBucket: i32 = 0;
      // java for (int iBucket = 0; iBucket < nBucket ++iBucket)
      while iBucket < nBucket {
        // java Object[] pBucket = this.mMap[iBucket];
        let pBucket: Option<&PropBucket> = self.mMap.as_ref().unwrap()[iBucket as usize].as_ref();
        // java if (pBucket != null)
        if let Some(pBucket) = pBucket {
          // java int id = iBucket << 3;
          let mut id: i32 = iBucket << 3;
          let mut iOffset: i32 = 0;
          // java for (int iOffset = 0; iOffset < 8 ++iOffset)
          while iOffset < 8 {
            // java if (pBucket[iOffset] != null)
            if pBucket[iOffset as usize].is_some() {
              // java if (ids != null)
              // ids always provided in rust as &mut Vec
              {
                // java ids[n] = id;
                ids[n as usize] = id;
              }
              // java if (values != null)
              if let Some(ref mut values) = values {
                // java values[n] = pBucket[iOffset];
                values[n as usize] = pBucket[iOffset as usize].clone();
              }
              // java ++n;
              n += 1;
            }
            // java ++id;
            id += 1;
            iOffset += 1;
          }
        }
        iBucket += 1;
      }
      // java return n;
      return n;
    }

    // java public final boolean getBoolean(int nProp)
    pub fn getBoolean(&self, nProp: i32) -> bool {
      // java Object pVal = this.getValue(nProp);
      let pVal: Option<&PropertyValue> = self.getValue(nProp);
      // java if (pVal == null)
      if pVal.is_none() {
        // java throw new PropertyException("No such property: " + nProp);
        panic!("{}", PropertyException::new(&format!("No such property: {}", nProp)));
      }
      // java else
      else {
        // java Boolean v = (Boolean)pVal;
        let v: bool = *downcast_ref::<bool>(pVal.unwrap()).unwrap_or(&false);
        // java return v;
        return v;
      }
    }

    // java public final boolean getBoolean(int nProp, boolean bDefault)
    pub fn getBoolean__int_boolean(&self, nProp: i32, bDefault: bool) -> bool {
      // java Object pVal = this.getValue(nProp);
      let pVal: Option<&PropertyValue> = self.getValue(nProp);
      // java if (pVal == null)
      if pVal.is_none() {
        // java return bDefault;
        return bDefault;
      }
      // java else
      else {
        // java Boolean v = (Boolean)pVal;
        let v: bool = *downcast_ref::<bool>(pVal.unwrap()).unwrap_or(&bDefault);
        // java return v;
        return v;
      }
    }
    pub fn getBoolean_default(&self, nProp: i32, bDefault: bool) -> bool { self.getBoolean__int_boolean(nProp, bDefault) }

    // java public final float getFloat(int nProp)
    pub fn getFloat(&self, nProp: i32) -> f32 {
      // java Object pVal = this.getValue(nProp);
      let pVal: Option<&PropertyValue> = self.getValue(nProp);
      // java if (pVal == null)
      if pVal.is_none() {
        // java throw new PropertyException("No such property: " + nProp);
        panic!("{}", PropertyException::new(&format!("No such property: {}", nProp)));
      }
      // java else
      else {
        // java Float v = (Float)pVal;
        let v: f32 = *downcast_ref::<f32>(pVal.unwrap()).unwrap_or(&0.0);
        // java return v;
        return v;
      }
    }

    // java public final float getFloat(int nProp, float fDefault)
    pub fn getFloat__int_float(&self, nProp: i32, fDefault: f32) -> f32 {
      // java Object pVal = this.getValue(nProp);
      let pVal: Option<&PropertyValue> = self.getValue(nProp);
      // java if (pVal == null)
      if pVal.is_none() {
        // java return fDefault;
        return fDefault;
      }
      // java else
      else {
        // java Float v = (Float)pVal;
        let v: f32 = *downcast_ref::<f32>(pVal.unwrap()).unwrap_or(&fDefault);
        // java return v;
        return v;
      }
    }
    pub fn getFloat_default(&self, nProp: i32, fDefault: f32) -> f32 { self.getFloat__int_float(nProp, fDefault) }

    // java public final int getInt(int nProp)
    pub fn getInt(&self, nProp: i32) -> i32 {
      // java Object pVal = this.getValue(nProp);
      let pVal: Option<&PropertyValue> = self.getValue(nProp);
      // java if (pVal == null)
      if pVal.is_none() {
        // java throw new PropertyException("No such property: " + nProp);
        panic!("{}", PropertyException::new(&format!("No such property: {}", nProp)));
      }
      // java else
      else {
        // java Integer v = (Integer)pVal;
        let v: i32 = *downcast_ref::<i32>(pVal.unwrap()).unwrap_or(&0);
        // java return v;
        return v;
      }
    }

    // java public final int getInt(int nProp, int iDefault)
    pub fn getInt__int_int(&self, nProp: i32, iDefault: i32) -> i32 {
      // java Object pVal = this.getValue(nProp);
      let pVal: Option<&PropertyValue> = self.getValue(nProp);
      // java if (pVal == null)
      if pVal.is_none() {
        // java return iDefault;
        return iDefault;
      }
      // java else
      else {
        // java Integer v = (Integer)pVal;
        let v: i32 = *downcast_ref::<i32>(pVal.unwrap()).unwrap_or(&iDefault);
        // java return v;
        return v;
      }
    }
    pub fn getInt_default(&self, nProp: i32, iDefault: i32) -> i32 { self.getInt__int_int(nProp, iDefault) }

    // java public final Object getObject(int nProp)
    pub fn getObject(&self, nProp: i32) -> &PropertyValue {
      // java Object pVal = this.getValue(nProp);
      let pVal: Option<&PropertyValue> = self.getValue(nProp);
      // java if (pVal == null)
      if pVal.is_none() {
        // java throw new PropertyException("No such property: " + nProp);
        panic!("{}", PropertyException::new(&format!("No such property: {}", nProp)));
      }
      // java else
      else {
        // java return pVal;
        return pVal.unwrap();
      }
    }

    // java public final Object getObject(int nProp, Object pDefault)
    pub fn getObject__int_object<'a>(&'a self, nProp: i32, pDefault: &'a PropertyValue) -> &'a PropertyValue {
      // java Object pVal = this.getValue(nProp);
      let pVal: Option<&PropertyValue> = self.getValue(nProp);
      // java return pVal == null ? pDefault : pVal;
      return if pVal.is_none() { pDefault } else { pVal.unwrap() };
    }

    // java public final String getString(int nProp)
    pub fn getString(&self, nProp: i32) -> String {
      // java Object pVal = this.getValue(nProp);
      let pVal: Option<&PropertyValue> = self.getValue(nProp);
      // java if (pVal == null)
      if pVal.is_none() {
        // java throw new PropertyException("No such property: " + nProp);
        panic!("{}", PropertyException::new(&format!("No such property: {}", nProp)));
      }
      // java else
      else {
        // java String v = (String)pVal;
        let v: String = downcast_ref::<String>(pVal.unwrap()).cloned().unwrap_or_default();
        // java return v;
        return v;
      }
    }

    // java public final String getString(int nProp, String strDefault)
    pub fn getString__int_string(&self, nProp: i32, strDefault: &str) -> String {
      // java Object pVal = this.getValue(nProp);
      let pVal: Option<&PropertyValue> = self.getValue(nProp);
      // java if (pVal == null)
      if pVal.is_none() {
        // java return strDefault;
        return strDefault.to_string();
      }
      // java else
      else {
        // java String v = (String)pVal;
        let v: String = downcast_ref::<String>(pVal.unwrap()).cloned().unwrap_or_else(|| strDefault.to_string());
        // java return v;
        return v;
      }
    }
    
    pub fn getString_default(&self, nProp: i32, strDefault: &str) -> String { self.getString__int_string(nProp, strDefault) }

    // java public final Object getValue(int nProp)
    pub fn getValue(&self, nProp: i32) -> Option<&PropertyValue> {
      // java int iBucket = nProp >> 3;
      let iBucket: i32 = nProp >> 3;
      // java int iOffset = nProp & 7;
      let iOffset: i32 = nProp & 7;
      // java return this.mMap != null && iBucket < this.mMap.length && this.mMap[iBucket] != null ? this.mMap[iBucket][iOffset] : null;
      return if self.mMap.is_some()
          && iBucket < (self.mMap.as_ref().unwrap().len() as i32)
          && self.mMap.as_ref().unwrap()[iBucket as usize].is_some()
      {
          self.mMap.as_ref().unwrap()[iBucket as usize].as_ref().unwrap()[iOffset as usize].as_ref()
      } else {
          None
      };
    }

    // java public int hashCode()
    pub fn hashCode(&mut self) -> i32 {
      // java if (this.isFrozen())
      if self.isFrozen() {
        // java if (this.mHash == 0)
        if self.mHash == 0 {
          // java this.mHash = this.computeHash();
          self.mHash = self.computeHash();
        }
        // java return this.mHash;
        return self.mHash;
      }
      // java else
      else {
        // java return this.computeHash();
        return self.computeHash();
      }
    }

    // java public final boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
      // java return this.size() == 0;
      return self.size() == 0;
    }

    // java final boolean isFrozen()
    pub fn isFrozen(&self) -> bool {
      // java return (this.mFlags & 1073741824) != 0;
      return (self.mFlags & 1073741824) != 0;
    }

    // java final void setFrozen()
    pub fn setFrozen(&mut self) {
      // java this.mFlags |= 1073741824;
      self.mFlags |= 1073741824;
    }

    // java public final int size()
    pub fn size(&self) -> i32 {
      // java return this.mCount;
      return self.mCount;
    }

    // java public String toString()
    pub fn toString(&self) -> String {
      // java return debug_description == null ? super.toString() : debug_description.getDescription(this);
      return if debug_description.is_none() {
          format!("PropertySet(count={})", self.mCount)
      } else {
          debug_description.as_ref().unwrap().getDescription(self)
      };
    }

}

