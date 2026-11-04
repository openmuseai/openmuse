#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.color.Color
// TODO: Color not yet available as concrete type
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

const FIELD_RADIUS: i32 = 1;
const FIELD_DIRECTION: i32 = 2;
const FIELD_DISTANCE: i32 = 3;
const FIELD_CLR: i32 = 4;

// Java: public class InnerShadowEffect implements KoAr, KoClosable
pub struct InnerShadowEffect {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Color clr
    pub(crate) clr: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: InnerShadowEffect parent
    pub(crate) parent: Option<Box<InnerShadowEffect>>,
}

// Java: public class InnerShadowEffect implements KoAr, KoClosable
impl InnerShadowEffect {
    // Java: static protected static <Value1 extends InnerShadowEffect> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = InnerShadowEffect::InnerShadowEffect_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static InnerShadowEffect create()
    pub fn create() -> Self {
      // Java: return initialize(InnerShadowEffect.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static InnerShadowEffect open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(InnerShadowEffect.class, bin);
      Self::initialize(bin)
    }

    // Java: InnerShadowEffect()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      InnerShadowEffect { zone: KoZone::defaultZone(), clr: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: InnerShadowEffect(KoBin bin)
    pub fn InnerShadowEffect_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      InnerShadowEffect { zone: KoZone::defaultZone(), clr: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(InnerShadowEffect value)
    pub fn load_value(&mut self, value: Option<&InnerShadowEffect>) {
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
      // Java: if (this.clr != null && this.clr.dirty())
      if self.clr.as_ref().map_or(false, |c| c.dirty())
      {
         // Java: this.map.put(4, this.clr.saveBin(zone));
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())); }
      self.bin_box.get()
    }

    // Java: boolean equalsLax(InnerShadowEffect o)
    pub fn equalsLax(&self, o: &InnerShadowEffect) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &InnerShadowEffect) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &InnerShadowEffect, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof InnerShadowEffect)
      if true
      {
         // Java: InnerShadowEffect dst = (InnerShadowEffect)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasRadius() || !dst.hasRadius()) && (!this.hasRadius() || dst.hasRadius()))
         else if (self.hasRadius() || !dst.hasRadius()) && (!self.hasRadius() || dst.hasRadius())
         {
            // Java: if (this.hasRadius() && dst.hasRadius() && this.radius() != dst.radius())
            if self.hasRadius() && dst.hasRadius() && self.radius() != dst.radius()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasDirection() || !dst.hasDirection()) && (!this.hasDirection() || dst.hasDirection()))
            else if (self.hasDirection() || !dst.hasDirection()) && (!self.hasDirection() || dst.hasDirection())
            {
               // Java: if (this.hasDirection() && dst.hasDirection() && this.direction() != dst.direction())
               if self.hasDirection() && dst.hasDirection() && self.direction() != dst.direction()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasDistance() || !dst.hasDistance()) && (!this.hasDistance() || dst.hasDistance()))
               else if (self.hasDistance() || !dst.hasDistance()) && (!self.hasDistance() || dst.hasDistance())
               {
                  // Java: if (this.hasDistance() && dst.hasDistance() && this.distance() != dst.distance())
                  if self.hasDistance() && dst.hasDistance() && self.distance() != dst.distance()
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasClr() || !dst.hasClr()) && (!this.hasClr() || dst.hasClr()))
                  else if (self.hasClr() || !dst.hasClr()) && (!self.hasClr() || dst.hasClr())
                  {
                     // Java: if (this.hasClr() && dst.hasClr())
                     if self.hasClr() && dst.hasClr()
                     {
                        // Java: if (isStrict && !this.clr().equals(dst.clr()))
                        if isStrict
                        {
                           // Java: return false;
                        }
                        // Java: if (!isStrict && !this.clr().equalsLax(dst.clr()))
                        if !isStrict
                        {
                           // Java: return false;
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

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasRadius())
      if self.hasRadius()
      {
         // Java: ret += Integer.valueOf(this.radius()).hashCode();
         ret = ret.wrapping_add(self.radius());
      }
      // Java: if (this.hasDirection())
      if self.hasDirection()
      {
         // Java: ret += Integer.valueOf(this.direction()).hashCode();
         ret = ret.wrapping_add(self.direction());
      }
      // Java: if (this.hasDistance())
      if self.hasDistance()
      {
         // Java: ret += Integer.valueOf(this.distance()).hashCode();
         ret = ret.wrapping_add(self.distance());
      }
      // Java: if (this.hasClr())
      if self.hasClr()
      {
         // Java: ret += this.clr().hashCode();
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: return this.clr != null && this.clr.dirty() ? true : this.map.dirty();
      if self.clr.as_ref().map_or(false, |c| c.dirty()) { true } else { self.map.dirty() }
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
      // Java: this.clr = null;
      self.clr = None;
    }

    // Java: void follow(InnerShadowEffect parent)
    pub fn follow(&mut self, mut parent: Box<InnerShadowEffect>) {
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

    // Java: InnerShadowEffect parent()
    pub fn get_parent(&self) -> Option<&InnerShadowEffect> {
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

    // Java: int radius()
    pub fn radius(&self) -> i32 {
      // Java: if (!this.map.has(1))
      if !self.map.has(FIELD_RADIUS)
      {
         // Java: return this.parent == null ? 0 : this.parent.radius();
         return self.parent.as_ref().map_or(0, |p| p.radius());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(FIELD_RADIUS);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasRadius()
    pub fn hasRadius(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasRadius();
      self.map.has(FIELD_RADIUS) || self.parent.as_ref().map_or(false, |p| p.hasRadius())
    }

    // Java: void deleteRadius()
    pub fn deleteRadius(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(FIELD_RADIUS);
    }

    // Java: void putRadius(int value)
    pub fn putRadius(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_RADIUS, Encode::from_int(value, self.zone));
    }

    // Java: int direction()
    pub fn direction(&self) -> i32 {
      // Java: if (!this.map.has(2))
      if !self.map.has(FIELD_DIRECTION)
      {
         // Java: return this.parent == null ? 0 : this.parent.direction();
         return self.parent.as_ref().map_or(0, |p| p.direction());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(FIELD_DIRECTION);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasDirection()
    pub fn hasDirection(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasDirection();
      self.map.has(FIELD_DIRECTION) || self.parent.as_ref().map_or(false, |p| p.hasDirection())
    }

    // Java: void deleteDirection()
    pub fn deleteDirection(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(FIELD_DIRECTION);
    }

    // Java: void putDirection(int value)
    pub fn putDirection(&mut self, value: i32) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_DIRECTION, Encode::from_int(value, self.zone));
    }

    // Java: int distance()
    pub fn distance(&self) -> i32 {
      // Java: if (!this.map.has(3))
      if !self.map.has(FIELD_DISTANCE)
      {
         // Java: return this.parent == null ? 0 : this.parent.distance();
         return self.parent.as_ref().map_or(0, |p| p.distance());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(FIELD_DISTANCE);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasDistance()
    pub fn hasDistance(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasDistance();
      self.map.has(FIELD_DISTANCE) || self.parent.as_ref().map_or(false, |p| p.hasDistance())
    }

    // Java: void deleteDistance()
    pub fn deleteDistance(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(FIELD_DISTANCE);
    }

    // Java: void putDistance(int value)
    pub fn putDistance(&mut self, value: i32) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_DISTANCE, Encode::from_int(value, self.zone));
    }

    // Java: Color clr()
    pub fn clr(&self) -> &dyn KoAr {
      // Java: if (this.clr == null)
      // Java: if (this.map.has(4))
      // Java: KoBin bin = this.map.byKey(4);
      // Java: this.clr = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
      // Java: else if (this.parent != null && this.parent.hasClr())
      // Java: this.clr = this.parent.clr();
      // Java: else
      // Java: KoBin bin = KoBin.Null.value1();
      // Java: this.clr = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
      // Java: return this.clr;
      todo!("Color lazy-init requires &mut self and Color type not yet available")
    }

    // Java: boolean hasClr()
    pub fn hasClr(&self) -> bool {
      // Java: return this.map.has(4) || this.clr != null && this.clr.dirty() || this.parent != null && this.parent.hasClr();
      self.map.has(FIELD_CLR) || self.clr.as_ref().map_or(false, |c| c.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasClr())
    }

    // Java: void deleteClr()
    pub fn deleteClr(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(FIELD_CLR);
      // Java: if (this.clr != null)
      if self.clr.is_some()
      {
         // Java: this.clr = null;
         self.clr = None;
      }
    }

    // Java: void writeClr(Color value)
    pub fn writeClr(&mut self, value: &dyn KoAr) {
      // Java: if (value != null)
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(4, bin);
            self.map.put(FIELD_CLR, bin);
            // Java: if (this.clr != null)
            if self.clr.is_some()
            {
               // Java: this.clr = null;
               self.clr = None;
            }
         }
      }
    }

    // Java: void forceWriteClr(Color value)
    pub fn forceWriteClr(&mut self, value: &dyn KoAr) {
      // Java: if (value != null)
      {
         // Java: this.map.put(4, value.lastBin(this.zone));
         self.map.put(FIELD_CLR, value.lastBin(self.zone));
         // Java: if (this.clr != null)
         if self.clr.is_some()
         {
            // Java: this.clr = null;
            self.clr = None;
         }
      }
    }

}

impl KoAr for InnerShadowEffect {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { InnerShadowEffect::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { InnerShadowEffect::lastBin(self, var1) }
}

impl KoClosable for InnerShadowEffect {
    fn close(&mut self) { InnerShadowEffect::close(self); }
}
