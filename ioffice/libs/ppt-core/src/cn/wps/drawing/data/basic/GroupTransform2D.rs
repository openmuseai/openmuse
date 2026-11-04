#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

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

const FIELD_CHILD_EXTENTS: i32 = 1;
const FIELD_CHILD_POINT: i32 = 2;

// Java: public class GroupTransform2D implements KoAr, KoClosable
pub struct GroupTransform2D {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: PositiveSize2D childExtents
    pub(crate) childExtents: Option<PositiveSize2D>,
    // Java field: Point2D childPoint
    pub(crate) childPoint: Option<Point2D>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: GroupTransform2D parent
    pub(crate) parent: Option<Box<GroupTransform2D>>,
}

// Java: public class GroupTransform2D implements KoAr, KoClosable
impl GroupTransform2D {
    // Java: static protected static <Value1 extends GroupTransform2D> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = GroupTransform2D::GroupTransform2D_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static GroupTransform2D create()
    pub fn create() -> Self {
      // Java: return initialize(GroupTransform2D.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static GroupTransform2D open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(GroupTransform2D.class, bin);
      Self::initialize(bin)
    }

    // Java: GroupTransform2D()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      GroupTransform2D { zone: KoZone::defaultZone(), childExtents: None, childPoint: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: GroupTransform2D(KoBin bin)
    pub fn GroupTransform2D_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      GroupTransform2D { zone: KoZone::defaultZone(), childExtents: None, childPoint: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(GroupTransform2D value)
    pub fn load_value(&mut self, value: Option<&GroupTransform2D>) {
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
      // Java: if (this.childExtents != null && this.childExtents.dirty())
      if self.childExtents.as_ref().map_or(false, |c| c.dirty())
      {
         // Java: this.map.put(1, this.childExtents.saveBin(zone));
         let bin = self.childExtents.as_mut().unwrap().saveBin(zone);
         self.map.put(FIELD_CHILD_EXTENTS, bin);
      }
      // Java: if (this.childPoint != null && this.childPoint.dirty())
      if self.childPoint.as_ref().map_or(false, |c| c.dirty())
      {
         // Java: this.map.put(2, this.childPoint.saveBin(zone));
         let bin = self.childPoint.as_mut().unwrap().saveBin(zone);
         self.map.put(FIELD_CHILD_POINT, bin);
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())); }
      self.bin_box.get()
    }

    // Java: boolean equalsLax(GroupTransform2D o)
    pub fn equalsLax(&self, o: &GroupTransform2D) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &GroupTransform2D) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &GroupTransform2D, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof GroupTransform2D)
      if true
      {
         // Java: GroupTransform2D dst = (GroupTransform2D)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasChildExtents() || !dst.hasChildExtents()) && (!this.hasChildExtents() || dst.hasChildExtents()))
         else if (self.hasChildExtents() || !dst.hasChildExtents()) && (!self.hasChildExtents() || dst.hasChildExtents())
         {
            // Java: if (this.hasChildExtents() && dst.hasChildExtents())
            if self.hasChildExtents() && dst.hasChildExtents()
            {
               // Java: if (isStrict && !this.childExtents().equals(dst.childExtents()))
               if isStrict
               {
                  // Java: return false;
                  // TODO: childExtents comparison requires lazy-init
               }
               // Java: if (!isStrict && !this.childExtents().equalsLax(dst.childExtents()))
               if !isStrict
               {
                  // Java: return false;
                  // TODO: childExtents comparison requires lazy-init
               }
            }
            // Java: if ((this.hasChildPoint() || !dst.hasChildPoint()) && (!this.hasChildPoint() || dst.hasChildPoint()))
            if (self.hasChildPoint() || !dst.hasChildPoint()) && (!self.hasChildPoint() || dst.hasChildPoint())
            {
               // Java: if (this.hasChildPoint() && dst.hasChildPoint())
               if self.hasChildPoint() && dst.hasChildPoint()
               {
                  // Java: if (isStrict && !this.childPoint().equals(dst.childPoint()))
                  if isStrict
                  {
                     // Java: return false;
                     // TODO: childPoint comparison requires lazy-init
                  }
                  // Java: if (!isStrict && !this.childPoint().equalsLax(dst.childPoint()))
                  if !isStrict
                  {
                     // Java: return false;
                     // TODO: childPoint comparison requires lazy-init
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

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasChildExtents())
      if self.hasChildExtents()
      {
         // Java: ret += this.childExtents().hashCode();
         // TODO: childExtents lazy-init
      }
      // Java: if (this.hasChildPoint())
      if self.hasChildPoint()
      {
         // Java: ret += this.childPoint().hashCode();
         // TODO: childPoint lazy-init
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.childExtents != null && this.childExtents.dirty())
      if self.childExtents.as_ref().map_or(false, |c| c.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
      else
      {
         // Java: return this.childPoint != null && this.childPoint.dirty() ? true : this.map.dirty();
         if self.childPoint.as_ref().map_or(false, |c| c.dirty()) { true } else { self.map.dirty() }
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
      // Java: this.childExtents = null;
      self.childExtents = None;
      // Java: this.childPoint = null;
      self.childPoint = None;
    }

    // Java: void follow(GroupTransform2D parent)
    pub fn follow(&mut self, mut parent: Box<GroupTransform2D>) {
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

    // Java: GroupTransform2D parent()
    pub fn get_parent(&self) -> Option<&GroupTransform2D> {
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

    // Java: PositiveSize2D childExtents()
    pub fn childExtents(&self) -> &PositiveSize2D {
      // Java: if (this.childExtents == null)
      // Java: if (this.map.has(1))
      // Java: KoBin bin = this.map.byKey(1);
      // Java: this.childExtents = (PositiveSize2D)KoBin.Decode.toAny(PositiveSize2D.class, bin, this.zone);
      // Java: else if (this.parent != null && this.parent.hasChildExtents())
      // Java: this.childExtents = this.parent.childExtents();
      // Java: else
      // Java: KoBin bin = KoBin.Null.value1();
      // Java: this.childExtents = (PositiveSize2D)KoBin.Decode.toAny(PositiveSize2D.class, bin, this.zone);
      // Java: return this.childExtents;
      todo!("childExtents lazy-init requires &mut self")
    }

    // Java: boolean hasChildExtents()
    pub fn hasChildExtents(&self) -> bool {
      // Java: return this.map.has(1) || this.childExtents != null && this.childExtents.dirty() || this.parent != null && this.parent.hasChildExtents();
      self.map.has(FIELD_CHILD_EXTENTS) || self.childExtents.as_ref().map_or(false, |c| c.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasChildExtents())
    }

    // Java: void deleteChildExtents()
    pub fn deleteChildExtents(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(FIELD_CHILD_EXTENTS);
      // Java: if (this.childExtents != null)
      if self.childExtents.is_some()
      {
         // Java: this.childExtents = null;
         self.childExtents = None;
      }
    }

    // Java: void writeChildExtents(PositiveSize2D value)
    pub fn writeChildExtents(&mut self, value: &PositiveSize2D) {
      // Java: if (value != null)
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(1, bin);
            self.map.put(FIELD_CHILD_EXTENTS, bin);
            // Java: if (this.childExtents != null)
            if self.childExtents.is_some()
            {
               // Java: this.childExtents = null;
               self.childExtents = None;
            }
         }
      }
    }

    // Java: void forceWriteChildExtents(PositiveSize2D value)
    pub fn forceWriteChildExtents(&mut self, value: &PositiveSize2D) {
      // Java: if (value != null)
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(FIELD_CHILD_EXTENTS, value.lastBin(self.zone));
         // Java: if (this.childExtents != null)
         if self.childExtents.is_some()
         {
            // Java: this.childExtents = null;
            self.childExtents = None;
         }
      }
    }

    // Java: Point2D childPoint()
    pub fn childPoint(&self) -> &Point2D {
      // Java: if (this.childPoint == null)
      // Java: if (this.map.has(2))
      // Java: KoBin bin = this.map.byKey(2);
      // Java: this.childPoint = (Point2D)KoBin.Decode.toAny(Point2D.class, bin, this.zone);
      // Java: else if (this.parent != null && this.parent.hasChildPoint())
      // Java: this.childPoint = this.parent.childPoint();
      // Java: else
      // Java: KoBin bin = KoBin.Null.value1();
      // Java: this.childPoint = (Point2D)KoBin.Decode.toAny(Point2D.class, bin, this.zone);
      // Java: return this.childPoint;
      todo!("childPoint lazy-init requires &mut self")
    }

    // Java: boolean hasChildPoint()
    pub fn hasChildPoint(&self) -> bool {
      // Java: return this.map.has(2) || this.childPoint != null && this.childPoint.dirty() || this.parent != null && this.parent.hasChildPoint();
      self.map.has(FIELD_CHILD_POINT) || self.childPoint.as_ref().map_or(false, |c| c.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasChildPoint())
    }

    // Java: void deleteChildPoint()
    pub fn deleteChildPoint(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(FIELD_CHILD_POINT);
      // Java: if (this.childPoint != null)
      if self.childPoint.is_some()
      {
         // Java: this.childPoint = null;
         self.childPoint = None;
      }
    }

    // Java: void writeChildPoint(Point2D value)
    pub fn writeChildPoint(&mut self, value: &Point2D) {
      // Java: if (value != null)
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(2, bin);
            self.map.put(FIELD_CHILD_POINT, bin);
            // Java: if (this.childPoint != null)
            if self.childPoint.is_some()
            {
               // Java: this.childPoint = null;
               self.childPoint = None;
            }
         }
      }
    }

    // Java: void forceWriteChildPoint(Point2D value)
    pub fn forceWriteChildPoint(&mut self, value: &Point2D) {
      // Java: if (value != null)
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(FIELD_CHILD_POINT, value.lastBin(self.zone));
         // Java: if (this.childPoint != null)
         if self.childPoint.is_some()
         {
            // Java: this.childPoint = null;
            self.childPoint = None;
         }
      }
    }

}

impl KoAr for GroupTransform2D {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { GroupTransform2D::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { GroupTransform2D::lastBin(self, var1) }
}

impl KoClosable for GroupTransform2D {
    fn close(&mut self) { GroupTransform2D::close(self); }
}
