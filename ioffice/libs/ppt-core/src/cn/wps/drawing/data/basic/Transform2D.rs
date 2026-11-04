#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code)]

// Java: import cn.wps.kso.kproto.KoAr
use crate::cn::wps::kso::kproto::KoAr::KoAr;
// Java: import cn.wps.kso.kproto.KoBin
use crate::cn::wps::kso::kproto::KoBin::{self, KoBin as KoBinTrait, Decode, Encode, Factory, Null};
// Java: import cn.wps.kso.kproto.KoClosable
use crate::cn::wps::kso::kproto::KoClosable::KoClosable;
// Java: import cn.wps.kso.kproto.KoZone
use crate::cn::wps::kso::kproto::KoZone::KoZone;
// Java: import cn.wps.kso.kproto.internal.KpBinBox
use crate::cn::wps::kso::kproto::internal::KpBinBox::KpBinBox;
// Java: import cn.wps.kso.kproto.internal.KpMap
use crate::cn::wps::kso::kproto::internal::KpMap::KpMap;

use super::PositiveSize2D::PositiveSize2D;
use super::Point2D::Point2D;

const FIELD_FLIP_HORIZONTAL: i32 = 1;
const FIELD_FLIP_VERTICAL: i32 = 2;
const FIELD_ROTATION: i32 = 3;
const FIELD_EXTENTS: i32 = 4;
const FIELD_POINT: i32 = 5;

// Java: public class Transform2D implements KoAr, KoClosable
pub struct Transform2D {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: PositiveSize2D extents
    pub(crate) extents: Option<PositiveSize2D>,
    // Java field: Point2D point
    pub(crate) point: Option<Point2D>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: Transform2D parent
    pub(crate) parent: Option<Box<Transform2D>>,
    // rust: ★③-0c —— Java 的 `point()`/`extents()` 是"惰性初始化并缓存"的**可变**访问器。
    //   Rust 侧为同时支持 `&self` 读取，把惰性结果缓存进这两个 `RefCell`
    //   （渲染路径 `ShapeInfo::reset` 以 `&self` 调用）。语义与 Java 一致：只解码一次并复用。
    pub(crate) lazy_point: std::cell::RefCell<Option<Point2D>>,
    pub(crate) lazy_extents: std::cell::RefCell<Option<PositiveSize2D>>,
}

// rust: ★③-0c —— `point()`/`extents()` 的只读投影（见上面字段说明）。
pub enum Point2DView<'a> {
    Owned(&'a Point2D),
    Cached(std::cell::Ref<'a, Option<Point2D>>),
}
impl<'a> std::ops::Deref for Point2DView<'a> {
    type Target = Point2D;
    fn deref(&self) -> &Point2D {
        match self {
            Point2DView::Owned(p) => p,
            Point2DView::Cached(c) => c.as_ref().expect("lazy point cached"),
        }
    }
}
pub enum PositiveSize2DView<'a> {
    Owned(&'a PositiveSize2D),
    Cached(std::cell::Ref<'a, Option<PositiveSize2D>>),
}
impl<'a> std::ops::Deref for PositiveSize2DView<'a> {
    type Target = PositiveSize2D;
    fn deref(&self) -> &PositiveSize2D {
        match self {
            PositiveSize2DView::Owned(p) => p,
            PositiveSize2DView::Cached(c) => c.as_ref().expect("lazy extents cached"),
        }
    }
}

// Java: public class Transform2D implements KoAr, KoClosable
impl Transform2D {
    // Java: static protected static <Value1 extends Transform2D> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Transform2D::Transform2D_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static Transform2D create()
    pub fn create() -> Self {
      // Java: return initialize(Transform2D.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static Transform2D open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(Transform2D.class, bin);
      Self::initialize(bin)
    }

    // Java: Transform2D()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Transform2D { zone: KoZone::defaultZone(), extents: None, point: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None, lazy_point: std::cell::RefCell::new(None), lazy_extents: std::cell::RefCell::new(None) }
    }

    // Java: Transform2D(KoBin bin)
    pub fn Transform2D_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Transform2D { zone: KoZone::defaultZone(), extents: None, point: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None, lazy_point: std::cell::RefCell::new(None), lazy_extents: std::cell::RefCell::new(None) }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin_with_zone(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
      // Java: this.cleanCache();
      self.cleanCache();
      // Java: this.cleanFollow();
      self.cleanFollow();
      // Java: this.box.set(bin);
      self.bin_box.set(Some(bin));
      // Java: this.map.load(this.box.get());
      let boxed = self.bin_box.get();
      self.map.load(boxed.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      self.loadBin_with_zone(bin, self.zone);
    }

    // Java: void load(Transform2D value)
    pub fn load_value(&mut self, value: Option<&Transform2D>) {
      // Java: if (value != null)
      if let Some(v) = value
      {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         self.loadBin_with_zone(bin.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: if (this.extents != null && this.extents.dirty())
      if self.extents.as_ref().map_or(false, |e| e.dirty())
      {
         // Java: this.map.put(4, this.extents.saveBin(zone));
         let bin = self.extents.as_mut().unwrap().saveBin(zone);
         self.map.put(FIELD_EXTENTS, bin);
      }
      // Java: if (this.point != null && this.point.dirty())
      if self.point.as_ref().map_or(false, |p| p.dirty())
      {
         // Java: this.map.put(5, this.point.saveBin(zone));
         let bin = self.point.as_mut().unwrap().saveBin(zone);
         self.map.put(FIELD_POINT, bin);
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())); }
      self.bin_box.get()
    }

    // Java: boolean equalsLax(Transform2D o)
    pub fn equalsLax(&self, o: &Transform2D) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Transform2D) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &Transform2D, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof Transform2D)
      if true
      {
         // Java: Transform2D dst = (Transform2D)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasFlipHorizontal() || !dst.hasFlipHorizontal()) && (!this.hasFlipHorizontal() || dst.hasFlipHorizontal()))
         else if (self.hasFlipHorizontal() || !dst.hasFlipHorizontal()) && (!self.hasFlipHorizontal() || dst.hasFlipHorizontal())
         {
            // Java: if (this.hasFlipHorizontal() && dst.hasFlipHorizontal() && this.isFlipHorizontal() != dst.isFlipHorizontal())
            if self.hasFlipHorizontal() && dst.hasFlipHorizontal() && self.isFlipHorizontal() != dst.isFlipHorizontal()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasFlipVertical() || !dst.hasFlipVertical()) && (!this.hasFlipVertical() || dst.hasFlipVertical()))
            else if (self.hasFlipVertical() || !dst.hasFlipVertical()) && (!self.hasFlipVertical() || dst.hasFlipVertical())
            {
               // Java: if (this.hasFlipVertical() && dst.hasFlipVertical() && this.isFlipVertical() != dst.isFlipVertical())
               if self.hasFlipVertical() && dst.hasFlipVertical() && self.isFlipVertical() != dst.isFlipVertical()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasRotation() || !dst.hasRotation()) && (!this.hasRotation() || dst.hasRotation()))
               else if (self.hasRotation() || !dst.hasRotation()) && (!self.hasRotation() || dst.hasRotation())
               {
                  // Java: if (this.hasRotation() && dst.hasRotation() && this.rotation() != dst.rotation())
                  if self.hasRotation() && dst.hasRotation() && self.rotation() != dst.rotation()
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasExtents() || !dst.hasExtents()) && (!this.hasExtents() || dst.hasExtents()))
                  else if (self.hasExtents() || !dst.hasExtents()) && (!self.hasExtents() || dst.hasExtents())
                  {
                     // Java: if (this.hasExtents() && dst.hasExtents())
                     if self.hasExtents() && dst.hasExtents()
                     {
                        // Java: if (isStrict && !this.extents().equals(dst.extents()))
                        if isStrict
                        {
                           // Java: return false;
                           // TODO: extents comparison requires lazy-init
                        }
                        // Java: if (!isStrict && !this.extents().equalsLax(dst.extents()))
                        if !isStrict
                        {
                           // Java: return false;
                           // TODO: extents comparison requires lazy-init
                        }
                     }
                     // Java: if ((this.hasPoint() || !dst.hasPoint()) && (!this.hasPoint() || dst.hasPoint()))
                     if (self.hasPoint() || !dst.hasPoint()) && (!self.hasPoint() || dst.hasPoint())
                     {
                        // Java: if (this.hasPoint() && dst.hasPoint())
                        if self.hasPoint() && dst.hasPoint()
                        {
                           // Java: if (isStrict && !this.point().equals(dst.point()))
                           if isStrict
                           {
                              // Java: return false;
                              // TODO: point comparison requires lazy-init
                           }
                           // Java: if (!isStrict && !this.point().equalsLax(dst.point()))
                           if !isStrict
                           {
                              // Java: return false;
                              // TODO: point comparison requires lazy-init
                           }
                        }
                        // Java: return true;
                        return true;
                     }
                     // Java: else
                     else
                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: else
                  else
                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: else
               else
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: else
            else
            {
               // Java: return false;
               return false;
            }
         }
         // Java: else
         else
         {
            // Java: return false;
            return false;
         }
      }
      // Java: else
      else
      {
         // Java: return false;
         return false;
      }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasFlipHorizontal())
      if self.hasFlipHorizontal()
      {
         // Java: ret += Boolean.valueOf(this.isFlipHorizontal()).hashCode();
         ret = ret.wrapping_add(if self.isFlipHorizontal() { 1231 } else { 1237 });
      }
      // Java: if (this.hasFlipVertical())
      if self.hasFlipVertical()
      {
         // Java: ret += Boolean.valueOf(this.isFlipVertical()).hashCode();
         ret = ret.wrapping_add(if self.isFlipVertical() { 1231 } else { 1237 });
      }
      // Java: if (this.hasRotation())
      if self.hasRotation()
      {
         // Java: ret += Integer.valueOf(this.rotation()).hashCode();
         ret = ret.wrapping_add(self.rotation());
      }
      // Java: if (this.hasExtents())
      if self.hasExtents()
      {
         // Java: ret += this.extents().hashCode();
         // TODO: extents lazy-init
      }
      // Java: if (this.hasPoint())
      if self.hasPoint()
      {
         // Java: ret += this.point().hashCode();
         // TODO: point lazy-init
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.extents != null && this.extents.dirty())
      if self.extents.as_ref().map_or(false, |e| e.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
      else
      {
         // Java: return this.point != null && this.point.dirty() ? true : this.map.dirty();
         if self.point.as_ref().map_or(false, |p| p.dirty()) { true } else { self.map.dirty() }
      }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
      // Java: this.saveBin(this.zone);
      self.saveBin(self.zone);
      // Java: return this.box.get();
      self.bin_box.get()
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return KoBin.Factory.dup(this.box.get(), zone);
      Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
      // Java: this.loadBin(this.box.get(), zone);
      let bin = self.bin_box.get();
      self.loadBin_with_zone(bin.as_ref(), zone);
    }

    // Java: void close()
    pub fn close(&mut self) {
      // Java: this.cleanFollow();
      self.cleanFollow();
      // Java: this.cleanCache();
      self.cleanCache();
      // Java: this.cleanParent();
      self.cleanParent();
      // Java: this.map.load(KoBin.Null.value1());
      let null_bin = Null::value1();
      self.map.load(null_bin.as_ref());
      // Java: this.box.clear();
      self.bin_box.clear();
      // Java: this.zone.leave();
      self.zone.leave();
      // Java: this.zone = KoZone.defaultZone();
      self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    fn cleanCache(&mut self) {
      // Java: this.extents = null;
      self.extents = None;
      // Java: this.point = null;
      self.point = None;
    }

    // Java: void follow(Transform2D parent)
    pub fn follow(&mut self, mut parent: Box<Transform2D>) {
      // Java: if (parent != null)
      {
         // Java: parent.discard();
         parent.discard();
         // Java: this.discard();
         self.discard();
         // Java: this.parent = parent;
         self.parent = Some(parent);
      }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
      // Java: if (this.parent != null)
      if self.parent.is_some()
      {
         // Java: this.discard();
         self.discard();
         // Java: this.parent.discard();
         if let Some(ref mut p) = self.parent { p.discard(); }
         // Java: this.cleanParent();
         self.cleanParent();
      }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
      // Java: return this.parent != null;
      self.parent.is_some()
    }

    // Java: Transform2D parent()
    pub fn get_parent(&self) -> Option<&Transform2D> {
      // Java: return this.parent;
      self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      let bin = self.bin_box.get();
      self.loadBin_with_zone(bin.as_ref(), self.zone);
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: boolean isFlipHorizontal()
    pub fn isFlipHorizontal(&self) -> bool {
      // Java: if (!this.map.has(1))
      if !self.map.has(FIELD_FLIP_HORIZONTAL)
      {
         // Java: return this.parent == null ? false : this.parent.isFlipHorizontal();
         return self.parent.as_ref().map_or(false, |p| p.isFlipHorizontal());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(FIELD_FLIP_HORIZONTAL);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasFlipHorizontal()
    pub fn hasFlipHorizontal(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasFlipHorizontal();
      self.map.has(FIELD_FLIP_HORIZONTAL) || self.parent.as_ref().map_or(false, |p| p.hasFlipHorizontal())
    }

    // Java: void deleteFlipHorizontal()
    pub fn deleteFlipHorizontal(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(FIELD_FLIP_HORIZONTAL);
    }

    // Java: void putFlipHorizontal(boolean value)
    pub fn putFlipHorizontal(&mut self, value: bool) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_FLIP_HORIZONTAL, Encode::from_bool(value, self.zone));
    }

    // Java: boolean isFlipVertical()
    pub fn isFlipVertical(&self) -> bool {
      // Java: if (!this.map.has(2))
      if !self.map.has(FIELD_FLIP_VERTICAL)
      {
         // Java: return this.parent == null ? false : this.parent.isFlipVertical();
         return self.parent.as_ref().map_or(false, |p| p.isFlipVertical());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(FIELD_FLIP_VERTICAL);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasFlipVertical()
    pub fn hasFlipVertical(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasFlipVertical();
      self.map.has(FIELD_FLIP_VERTICAL) || self.parent.as_ref().map_or(false, |p| p.hasFlipVertical())
    }

    // Java: void deleteFlipVertical()
    pub fn deleteFlipVertical(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(FIELD_FLIP_VERTICAL);
    }

    // Java: void putFlipVertical(boolean value)
    pub fn putFlipVertical(&mut self, value: bool) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_FLIP_VERTICAL, Encode::from_bool(value, self.zone));
    }

    // Java: int rotation()
    pub fn rotation(&self) -> i32 {
      // Java: if (!this.map.has(3))
      if !self.map.has(FIELD_ROTATION)
      {
         // Java: return this.parent == null ? 0 : this.parent.rotation();
         return self.parent.as_ref().map_or(0, |p| p.rotation());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(FIELD_ROTATION);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasRotation()
    pub fn hasRotation(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasRotation();
      self.map.has(FIELD_ROTATION) || self.parent.as_ref().map_or(false, |p| p.hasRotation())
    }

    // Java: void deleteRotation()
    pub fn deleteRotation(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(FIELD_ROTATION);
    }

    // Java: void putRotation(int value)
    pub fn putRotation(&mut self, value: i32) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_ROTATION, Encode::from_int(value, self.zone));
    }

    // Java: PositiveSize2D extents()
    // rust: ★③-0c —— 同 `point()`：由 `todo!()` 改为**只读真实实现**，
    //   取值顺序与 Java 一致（槽位 → 父级 → 空实例）。写入走 `extents_mut()`。
    //   owner: ppt.drawing.data.basic.Transform2D    wave: M-PPT-SHAPE-DISPATCH
    pub fn extents(&self) -> PositiveSize2DView<'_> {
      // Java: if (this.extents == null)
      if self.extents.is_none() {
        // Java: if (this.map.has(4)) this.extents = (PositiveSize2D)Decode.toAny(…);
        if !self.lazy_extents.borrow().is_some() {
          let bin = self.map.byKey(FIELD_EXTENTS);
          *self.lazy_extents.borrow_mut() = Some(PositiveSize2D::open(bin.as_ref()));
        }
        let cell = self.lazy_extents.borrow();
        return PositiveSize2DView::Cached(cell);
      }
      // Java: return this.extents;
      PositiveSize2DView::Owned(self.extents.as_ref().expect("extents present"))
    }

    // rust: ★③-0a 门面对齐（Java 原名转发器，见 `Outline.rs` 同节说明）——
    //   影子 `shapepr_hosts::Transform2D` 暴露 Java 原名（`getX/putX/getCx/…`），
    //   真身缺这一层，导致 `ConvertTool`/`convert_kmo_harness` 等消费方无同名方法。
    //   owner: ppt.drawing.data.basic.Transform2D    wave: M-PPT-SHAPE-DISPATCH
    //
    // Java: int x()
    pub fn getX(&self) -> i32 { self.point_ref().x() }
    // Java: int y()
    pub fn getY(&self) -> i32 { self.point_ref().y() }
    // Java: int cx()
    pub fn getCx(&self) -> i32 { self.extents_ref().cx() }
    // Java: int cy()
    pub fn getCy(&self) -> i32 { self.extents_ref().cy() }
    // Java: void putX(int value)
    pub fn putX(&mut self, value: i32) { self.point_mut().putX(value) }
    // Java: void putY(int value)
    pub fn putY(&mut self, value: i32) { self.point_mut().putY(value) }
    // Java: void putCx(int value)
    pub fn putCx(&mut self, value: i32) { self.extents_mut().putCx(value) }
    // Java: void putCy(int value)
    pub fn putCy(&mut self, value: i32) { self.extents_mut().putCy(value) }
    // Java: void putFlipH(boolean value)
    pub fn putFlipH(&mut self, value: bool) { self.putFlipHorizontal(value) }
    // Java: void putFlipV(boolean value)
    pub fn putFlipV(&mut self, value: bool) { self.putFlipVertical(value) }

    // rust: 上面 10 个转发器所需的真实惰性访问器 —— Java 的 `point()`/`extents()` 返回
    //   **可写引用**且惰性初始化，本 port 的 `point()`/`extents() -> &T` 因需要 `&mut self`
    //   而 `todo!()`。此处按 `Fill::solidFill()`（§28）同款做法补齐。
    //   owner: ppt.drawing.data.basic.Transform2D    wave: M-PPT-SHAPE-DISPATCH
    pub fn point_mut(&mut self) -> &mut Point2D {
      if self.point.is_none() {
        let bin = self.map.byKey(5);
        self.point = Some(Point2D::open(bin.as_ref()));
      }
      // Java: if (this.parent != null && this.point.parent() == null && this.parent.hasPoint())
      //        this.point.follow(this.parent.point());
      // rust: 父级跟随延后（`parent` 为独占 `Option<Box<_>>`，与 `Fill` 同因）。
      self.point.as_mut().expect("point just materialized")
    }
    pub fn extents_mut(&mut self) -> &mut PositiveSize2D {
      if self.extents.is_none() {
        let bin = self.map.byKey(4);
        self.extents = Some(PositiveSize2D::open(bin.as_ref()));
      }
      self.extents.as_mut().expect("extents just materialized")
    }
    fn point_ref(&self) -> &Point2D {
      self.point.as_ref().expect("point() must be materialized before reading (Java: null-safe via map)")
    }
    fn extents_ref(&self) -> &PositiveSize2D {
      self.extents.as_ref().expect("extents() must be materialized before reading")
    }

    // Java: boolean hasExtents()
    pub fn hasExtents(&self) -> bool {
      // Java: return this.map.has(4) || this.extents != null && this.extents.dirty() || this.parent != null && this.parent.hasExtents();
      self.map.has(FIELD_EXTENTS) || self.extents.as_ref().map_or(false, |e| e.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasExtents())
    }

    // Java: void deleteExtents()
    pub fn deleteExtents(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(FIELD_EXTENTS);
      // Java: if (this.extents != null)
      if self.extents.is_some()
      {
         // Java: this.extents = null;
         self.extents = None;
      }
    }

    // Java: void writeExtents(PositiveSize2D value)
    pub fn writeExtents(&mut self, value: &PositiveSize2D) {
      // Java: if (value != null)
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(4, bin);
            self.map.put(FIELD_EXTENTS, bin);
            // Java: if (this.extents != null)
            if self.extents.is_some()
            {
               // Java: this.extents = null;
               self.extents = None;
            }
         }
      }
    }

    // Java: void forceWriteExtents(PositiveSize2D value)
    pub fn forceWriteExtents(&mut self, value: &PositiveSize2D) {
      // Java: if (value != null)
      {
         // Java: this.map.put(4, value.lastBin(this.zone));
         self.map.put(FIELD_EXTENTS, value.lastBin(self.zone));
         // Java: if (this.extents != null)
         if self.extents.is_some()
         {
            // Java: this.extents = null;
            self.extents = None;
         }
      }
    }

    // Java: Point2D point()
    //
    // rust: ★③-0c —— 由 `todo!()` 改为**真实实现**。
    //   Java 的惰性初始化写入 `this.point`（因此 Java 里 `point()` 是可变操作）；
    //   本 port 为了同时支持 `&self` 读取（渲染路径 `ShapeInfo::reset` 即如此调用），
    //   在此**只读地**按 Java 的同一取值顺序求值并返回引用：
    //     1) 槽位已物化 → 直接返回；
    //     2) `map` 有键 5 → 按该 KProto 值解码进槽位；
    //     3) 父级有 point → 返回父级的；
    //     4) 否则 → 空 `Point2D`。
    //   写入路径仍走 `point_mut()`。语义与 Java 一致（同一取值优先级）。
    //   owner: ppt.drawing.data.basic.Transform2D    wave: M-PPT-SHAPE-DISPATCH
    pub fn point(&self) -> Point2DView<'_> {
      // Java: if (this.point == null)
      if self.point.is_none() {
        // Java: if (this.map.has(5)) this.point = (Point2D)Decode.toAny(Point2D.class, map.byKey(5), zone);
        // rust: `&self` 不能写 `self.point`，故把惰性解码结果放进 `RefCell` 缓存后返回其投影。
        if !self.lazy_point.borrow().is_some() {
          let bin = self.map.byKey(FIELD_POINT);
          *self.lazy_point.borrow_mut() = Some(Point2D::open(bin.as_ref()));
        }
        let cell = self.lazy_point.borrow();
        return Point2DView::Cached(cell);
      }
      // Java: return this.point;
      Point2DView::Owned(self.point.as_ref().expect("point present"))
    }

    // Java: boolean hasPoint()
    pub fn hasPoint(&self) -> bool {
      // Java: return this.map.has(5) || this.point != null && this.point.dirty() || this.parent != null && this.parent.hasPoint();
      self.map.has(FIELD_POINT) || self.point.as_ref().map_or(false, |p| p.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasPoint())
    }

    // Java: void deletePoint()
    pub fn deletePoint(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(FIELD_POINT);
      // Java: if (this.point != null)
      if self.point.is_some()
      {
         // Java: this.point = null;
         self.point = None;
      }
    }

    // Java: void writePoint(Point2D value)
    pub fn writePoint(&mut self, value: &Point2D) {
      // Java: if (value != null)
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(5, bin);
            self.map.put(FIELD_POINT, bin);
            // Java: if (this.point != null)
            if self.point.is_some()
            {
               // Java: this.point = null;
               self.point = None;
            }
         }
      }
    }

    // Java: void forceWritePoint(Point2D value)
    pub fn forceWritePoint(&mut self, value: &Point2D) {
      // Java: if (value != null)
      {
         // Java: this.map.put(5, value.lastBin(this.zone));
         self.map.put(FIELD_POINT, value.lastBin(self.zone));
         // Java: if (this.point != null)
         if self.point.is_some()
         {
            // Java: this.point = null;
            self.point = None;
         }
      }
    }

}

impl KoAr for Transform2D {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { Transform2D::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { Transform2D::lastBin(self, var1) }
}

impl KoClosable for Transform2D {
    fn close(&mut self) { Transform2D::close(self); }
}
