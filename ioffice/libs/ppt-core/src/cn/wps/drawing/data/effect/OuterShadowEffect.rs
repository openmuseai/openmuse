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

const FIELD_ALGN: i32 = 1;
const FIELD_BLUR_RADIUS: i32 = 2;
const FIELD_DIRECTION: i32 = 3;
const FIELD_DISTANCE: i32 = 4;
const FIELD_KX: i32 = 5;
const FIELD_KY: i32 = 6;
const FIELD_ROTATE_WITH_SHAPE: i32 = 7;
const FIELD_SX: i32 = 8;
const FIELD_SY: i32 = 9;
const FIELD_CLR: i32 = 10;

// Java: public class OuterShadowEffect implements KoAr, KoClosable
pub struct OuterShadowEffect {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Color clr
    pub(crate) clr: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: OuterShadowEffect parent
    pub(crate) parent: Option<Box<OuterShadowEffect>>,
}

// Java: public class OuterShadowEffect implements KoAr, KoClosable
impl OuterShadowEffect {
    // Java: static protected static <Value1 extends OuterShadowEffect> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = OuterShadowEffect::OuterShadowEffect_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static OuterShadowEffect create()
    pub fn create() -> Self {
      // Java: return initialize(OuterShadowEffect.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static OuterShadowEffect open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(OuterShadowEffect.class, bin);
      Self::initialize(bin)
    }

    // Java: OuterShadowEffect()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      OuterShadowEffect { zone: KoZone::defaultZone(), clr: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: OuterShadowEffect(KoBin bin)
    pub fn OuterShadowEffect_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      OuterShadowEffect { zone: KoZone::defaultZone(), clr: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(OuterShadowEffect value)
    pub fn load_value(&mut self, value: Option<&OuterShadowEffect>) {
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
         // Java: this.map.put(10, this.clr.saveBin(zone));
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())); }
      self.bin_box.get()
    }

    // Java: boolean equalsLax(OuterShadowEffect o)
    pub fn equalsLax(&self, o: &OuterShadowEffect) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &OuterShadowEffect) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &OuterShadowEffect, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof OuterShadowEffect)
      if true
      {
         // Java: OuterShadowEffect dst = (OuterShadowEffect)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasAlgn() || !dst.hasAlgn()) && (!this.hasAlgn() || dst.hasAlgn()))
         else if (self.hasAlgn() || !dst.hasAlgn()) && (!self.hasAlgn() || dst.hasAlgn())
         {
            // Java: if (this.hasAlgn() && dst.hasAlgn() && this.algn() != dst.algn())
            if self.hasAlgn() && dst.hasAlgn() && self.algn() != dst.algn()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasBlurRadius() || !dst.hasBlurRadius()) && (!this.hasBlurRadius() || dst.hasBlurRadius()))
            else if (self.hasBlurRadius() || !dst.hasBlurRadius()) && (!self.hasBlurRadius() || dst.hasBlurRadius())
            {
               // Java: if (this.hasBlurRadius() && dst.hasBlurRadius() && this.blurRadius() != dst.blurRadius())
               if self.hasBlurRadius() && dst.hasBlurRadius() && self.blurRadius() != dst.blurRadius()
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
                     // Java: else if ((this.hasKx() || !dst.hasKx()) && (!this.hasKx() || dst.hasKx()))
                     else if (self.hasKx() || !dst.hasKx()) && (!self.hasKx() || dst.hasKx())
                     {
                        // Java: if (this.hasKx() && dst.hasKx() && this.kx() != dst.kx())
                        if self.hasKx() && dst.hasKx() && self.kx() != dst.kx()
                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: else if ((this.hasKy() || !dst.hasKy()) && (!this.hasKy() || dst.hasKy()))
                        else if (self.hasKy() || !dst.hasKy()) && (!self.hasKy() || dst.hasKy())
                        {
                           // Java: if (this.hasKy() && dst.hasKy() && this.ky() != dst.ky())
                           if self.hasKy() && dst.hasKy() && self.ky() != dst.ky()
                           {
                              // Java: return false;
                              return false;
                           }
                           // Java: else if ((this.hasRotateWithShape() || !dst.hasRotateWithShape()) && (!this.hasRotateWithShape() || dst.hasRotateWithShape()))
                           else if (self.hasRotateWithShape() || !dst.hasRotateWithShape()) && (!self.hasRotateWithShape() || dst.hasRotateWithShape())
                           {
                              // Java: if (this.hasRotateWithShape() && dst.hasRotateWithShape() && this.isRotateWithShape() != dst.isRotateWithShape())
                              if self.hasRotateWithShape() && dst.hasRotateWithShape() && self.isRotateWithShape() != dst.isRotateWithShape()
                              {
                                 // Java: return false;
                                 return false;
                              }
                              // Java: else if ((this.hasSx() || !dst.hasSx()) && (!this.hasSx() || dst.hasSx()))
                              else if (self.hasSx() || !dst.hasSx()) && (!self.hasSx() || dst.hasSx())
                              {
                                 // Java: if (this.hasSx() && dst.hasSx() && this.sx() != dst.sx())
                                 if self.hasSx() && dst.hasSx() && self.sx() != dst.sx()
                                 {
                                    // Java: return false;
                                    return false;
                                 }
                                 // Java: else if ((this.hasSy() || !dst.hasSy()) && (!this.hasSy() || dst.hasSy()))
                                 else if (self.hasSy() || !dst.hasSy()) && (!self.hasSy() || dst.hasSy())
                                 {
                                    // Java: if (this.hasSy() && dst.hasSy() && this.sy() != dst.sy())
                                    if self.hasSy() && dst.hasSy() && self.sy() != dst.sy()
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
                                    else {
                                       // Java: return false;
                                       return false;
                                    }
                                 }
                                 // Java: else
                                 else {
                                    // Java: return false;
                                    return false;
                                 }
                              }
                              // Java: else
                              else {
                                 // Java: return false;
                                 return false;
                              }
                           }
                           // Java: else
                           else {
                              // Java: return false;
                              return false;
                           }
                        }
                        // Java: else
                        else {
                           // Java: return false;
                           return false;
                        }
                     }
                     // Java: else
                     else {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: else
                  else {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: else
               else {
                  // Java: return false;
                  return false;
               }
            }
            // Java: else
            else {
               // Java: return false;
               return false;
            }
         }
         // Java: else
         else {
            // Java: return false;
            return false;
         }
      }
      // Java: else
      else {
         // Java: return false;
         return false;
      }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasAlgn())
      if self.hasAlgn()
      {
         // Java: ret += Integer.valueOf(this.algn()).hashCode();
         ret = ret.wrapping_add(self.algn());
      }
      // Java: if (this.hasBlurRadius())
      if self.hasBlurRadius()
      {
         // Java: ret += Integer.valueOf(this.blurRadius()).hashCode();
         ret = ret.wrapping_add(self.blurRadius());
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
      // Java: if (this.hasKx())
      if self.hasKx()
      {
         // Java: ret += Integer.valueOf(this.kx()).hashCode();
         ret = ret.wrapping_add(self.kx());
      }
      // Java: if (this.hasKy())
      if self.hasKy()
      {
         // Java: ret += Integer.valueOf(this.ky()).hashCode();
         ret = ret.wrapping_add(self.ky());
      }
      // Java: if (this.hasRotateWithShape())
      if self.hasRotateWithShape()
      {
         // Java: ret += Boolean.valueOf(this.isRotateWithShape()).hashCode();
         ret = ret.wrapping_add(if self.isRotateWithShape() { 1231 } else { 1237 });
      }
      // Java: if (this.hasSx())
      if self.hasSx()
      {
         // Java: ret += Double.valueOf(this.sx()).hashCode();
         let bits = self.sx().to_bits() as i64;
         ret = ret.wrapping_add((bits ^ (bits >> 32)) as i32);
      }
      // Java: if (this.hasSy())
      if self.hasSy()
      {
         // Java: ret += Double.valueOf(this.sy()).hashCode();
         let bits = self.sy().to_bits() as i64;
         ret = ret.wrapping_add((bits ^ (bits >> 32)) as i32);
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

    // Java: void follow(OuterShadowEffect parent)
    pub fn follow(&mut self, mut parent: Box<OuterShadowEffect>) {
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

    // Java: OuterShadowEffect parent()
    pub fn get_parent(&self) -> Option<&OuterShadowEffect> {
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

    // Java: int algn()
    pub fn algn(&self) -> i32 {
      // Java: if (!this.map.has(1))
      if !self.map.has(FIELD_ALGN) {
         // Java: return this.parent == null ? 0 : this.parent.algn();
         return self.parent.as_ref().map_or(0, |p| p.algn());
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(FIELD_ALGN);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasAlgn()
    pub fn hasAlgn(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasAlgn();
      self.map.has(FIELD_ALGN) || self.parent.as_ref().map_or(false, |p| p.hasAlgn())
    }

    // Java: void deleteAlgn()
    pub fn deleteAlgn(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(FIELD_ALGN);
    }

    // Java: void putAlgn(int value)
    pub fn putAlgn(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_ALGN, Encode::from_int(value, self.zone));
    }

    // Java: int blurRadius()
    pub fn blurRadius(&self) -> i32 {
      // Java: if (!this.map.has(2))
      if !self.map.has(FIELD_BLUR_RADIUS) {
         // Java: return this.parent == null ? 0 : this.parent.blurRadius();
         return self.parent.as_ref().map_or(0, |p| p.blurRadius());
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(FIELD_BLUR_RADIUS);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasBlurRadius()
    pub fn hasBlurRadius(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasBlurRadius();
      self.map.has(FIELD_BLUR_RADIUS) || self.parent.as_ref().map_or(false, |p| p.hasBlurRadius())
    }

    // Java: void deleteBlurRadius()
    pub fn deleteBlurRadius(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(FIELD_BLUR_RADIUS);
    }

    // Java: void putBlurRadius(int value)
    pub fn putBlurRadius(&mut self, value: i32) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_BLUR_RADIUS, Encode::from_int(value, self.zone));
    }

    // Java: int direction()
    pub fn direction(&self) -> i32 {
      // Java: if (!this.map.has(3))
      if !self.map.has(FIELD_DIRECTION) {
         // Java: return this.parent == null ? 0 : this.parent.direction();
         return self.parent.as_ref().map_or(0, |p| p.direction());
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(FIELD_DIRECTION);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasDirection()
    pub fn hasDirection(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasDirection();
      self.map.has(FIELD_DIRECTION) || self.parent.as_ref().map_or(false, |p| p.hasDirection())
    }

    // Java: void deleteDirection()
    pub fn deleteDirection(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(FIELD_DIRECTION);
    }

    // Java: void putDirection(int value)
    pub fn putDirection(&mut self, value: i32) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_DIRECTION, Encode::from_int(value, self.zone));
    }

    // Java: int distance()
    pub fn distance(&self) -> i32 {
      // Java: if (!this.map.has(4))
      if !self.map.has(FIELD_DISTANCE) {
         // Java: return this.parent == null ? 0 : this.parent.distance();
         return self.parent.as_ref().map_or(0, |p| p.distance());
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(4);
         let bin = self.map.byKey(FIELD_DISTANCE);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasDistance()
    pub fn hasDistance(&self) -> bool {
      // Java: return this.map.has(4) || this.parent != null && this.parent.hasDistance();
      self.map.has(FIELD_DISTANCE) || self.parent.as_ref().map_or(false, |p| p.hasDistance())
    }

    // Java: void deleteDistance()
    pub fn deleteDistance(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(FIELD_DISTANCE);
    }

    // Java: void putDistance(int value)
    pub fn putDistance(&mut self, value: i32) {
      // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_DISTANCE, Encode::from_int(value, self.zone));
    }

    // Java: int kx()
    pub fn kx(&self) -> i32 {
      // Java: if (!this.map.has(5))
      if !self.map.has(FIELD_KX) {
         // Java: return this.parent == null ? 0 : this.parent.kx();
         return self.parent.as_ref().map_or(0, |p| p.kx());
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(5);
         let bin = self.map.byKey(FIELD_KX);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasKx()
    pub fn hasKx(&self) -> bool {
      // Java: return this.map.has(5) || this.parent != null && this.parent.hasKx();
      self.map.has(FIELD_KX) || self.parent.as_ref().map_or(false, |p| p.hasKx())
    }

    // Java: void deleteKx()
    pub fn deleteKx(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(FIELD_KX);
    }

    // Java: void putKx(int value)
    pub fn putKx(&mut self, value: i32) {
      // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_KX, Encode::from_int(value, self.zone));
    }

    // Java: int ky()
    pub fn ky(&self) -> i32 {
      // Java: if (!this.map.has(6))
      if !self.map.has(FIELD_KY) {
         // Java: return this.parent == null ? 0 : this.parent.ky();
         return self.parent.as_ref().map_or(0, |p| p.ky());
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(6);
         let bin = self.map.byKey(FIELD_KY);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasKy()
    pub fn hasKy(&self) -> bool {
      // Java: return this.map.has(6) || this.parent != null && this.parent.hasKy();
      self.map.has(FIELD_KY) || self.parent.as_ref().map_or(false, |p| p.hasKy())
    }

    // Java: void deleteKy()
    pub fn deleteKy(&mut self) {
      // Java: this.map.put(6, (KoBin)null);
      self.map.clear_key(FIELD_KY);
    }

    // Java: void putKy(int value)
    pub fn putKy(&mut self, value: i32) {
      // Java: this.map.put(6, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_KY, Encode::from_int(value, self.zone));
    }

    // Java: boolean isRotateWithShape()
    pub fn isRotateWithShape(&self) -> bool {
      // Java: if (!this.map.has(7))
      if !self.map.has(FIELD_ROTATE_WITH_SHAPE) {
         // Java: return this.parent == null ? true : this.parent.isRotateWithShape();
         return self.parent.as_ref().map_or(true, |p| p.isRotateWithShape());
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(7);
         let bin = self.map.byKey(FIELD_ROTATE_WITH_SHAPE);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasRotateWithShape()
    pub fn hasRotateWithShape(&self) -> bool {
      // Java: return this.map.has(7) || this.parent != null && this.parent.hasRotateWithShape();
      self.map.has(FIELD_ROTATE_WITH_SHAPE) || self.parent.as_ref().map_or(false, |p| p.hasRotateWithShape())
    }

    // Java: void deleteRotateWithShape()
    pub fn deleteRotateWithShape(&mut self) {
      // Java: this.map.put(7, (KoBin)null);
      self.map.clear_key(FIELD_ROTATE_WITH_SHAPE);
    }

    // Java: void putRotateWithShape(boolean value)
    pub fn putRotateWithShape(&mut self, value: bool) {
      // Java: this.map.put(7, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_ROTATE_WITH_SHAPE, Encode::from_bool(value, self.zone));
    }

    // Java: double sx()
    pub fn sx(&self) -> f64 {
      // Java: if (!this.map.has(8))
      if !self.map.has(FIELD_SX) {
         // Java: return this.parent == null ? (double)1.0F : this.parent.sx();
         return self.parent.as_ref().map_or(1.0f64, |p| p.sx());
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(8);
         let bin = self.map.byKey(FIELD_SX);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasSx()
    pub fn hasSx(&self) -> bool {
      // Java: return this.map.has(8) || this.parent != null && this.parent.hasSx();
      self.map.has(FIELD_SX) || self.parent.as_ref().map_or(false, |p| p.hasSx())
    }

    // Java: void deleteSx()
    pub fn deleteSx(&mut self) {
      // Java: this.map.put(8, (KoBin)null);
      self.map.clear_key(FIELD_SX);
    }

    // Java: void putSx(double value)
    pub fn putSx(&mut self, value: f64) {
      // Java: this.map.put(8, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_SX, Encode::from_double(value, self.zone));
    }

    // Java: double sy()
    pub fn sy(&self) -> f64 {
      // Java: if (!this.map.has(9))
      if !self.map.has(FIELD_SY) {
         // Java: return this.parent == null ? (double)1.0F : this.parent.sy();
         return self.parent.as_ref().map_or(1.0f64, |p| p.sy());
      }
      // Java: else
      else {
         // Java: KoBin bin = this.map.byKey(9);
         let bin = self.map.byKey(FIELD_SY);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasSy()
    pub fn hasSy(&self) -> bool {
      // Java: return this.map.has(9) || this.parent != null && this.parent.hasSy();
      self.map.has(FIELD_SY) || self.parent.as_ref().map_or(false, |p| p.hasSy())
    }

    // Java: void deleteSy()
    pub fn deleteSy(&mut self) {
      // Java: this.map.put(9, (KoBin)null);
      self.map.clear_key(FIELD_SY);
    }

    // Java: void putSy(double value)
    pub fn putSy(&mut self, value: f64) {
      // Java: this.map.put(9, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_SY, Encode::from_double(value, self.zone));
    }

    // Java: Color clr()
    pub fn clr(&self) -> &dyn KoAr {
      // Java: if (this.clr == null)
      // Java: if (this.map.has(10))
      // Java: KoBin bin = this.map.byKey(10);
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
      // Java: return this.map.has(10) || this.clr != null && this.clr.dirty() || this.parent != null && this.parent.hasClr();
      self.map.has(FIELD_CLR) || self.clr.as_ref().map_or(false, |c| c.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasClr())
    }

    // Java: void deleteClr()
    pub fn deleteClr(&mut self) {
      // Java: this.map.put(10, (KoBin)null);
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
            // Java: this.map.put(10, bin);
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
         // Java: this.map.put(10, value.lastBin(this.zone));
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

impl KoAr for OuterShadowEffect {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { OuterShadowEffect::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { OuterShadowEffect::lastBin(self, var1) }
}

impl KoClosable for OuterShadowEffect {
    fn close(&mut self) { OuterShadowEffect::close(self); }
}
