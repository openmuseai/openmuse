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

const FIELD_HUE: i32 = 1;
const FIELD_LUM: i32 = 2;
const FIELD_SAT: i32 = 3;

// Java: public class HSLEffect implements KoAr, KoClosable
pub struct HSLEffect {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: HSLEffect parent
    pub(crate) parent: Option<Box<HSLEffect>>,
}

// Java: public class HSLEffect implements KoAr, KoClosable
impl HSLEffect {
    // Java: static protected static <Value1 extends HSLEffect> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = HSLEffect::HSLEffect_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static HSLEffect create()
    pub fn create() -> Self {
      // Java: return initialize(HSLEffect.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static HSLEffect open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(HSLEffect.class, bin);
      Self::initialize(bin)
    }

    // Java: HSLEffect()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      HSLEffect { zone: KoZone::defaultZone(), bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: HSLEffect(KoBin bin)
    pub fn HSLEffect_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      HSLEffect { zone: KoZone::defaultZone(), bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(HSLEffect value)
    pub fn load_value(&mut self, value: Option<&HSLEffect>) {
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
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())); }
      self.bin_box.get()
    }

    // Java: boolean equalsLax(HSLEffect o)
    pub fn equalsLax(&self, o: &HSLEffect) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &HSLEffect) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &HSLEffect, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof HSLEffect)
      if true
      {
         // Java: HSLEffect dst = (HSLEffect)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasHue() || !dst.hasHue()) && (!this.hasHue() || dst.hasHue()))
         else if (self.hasHue() || !dst.hasHue()) && (!self.hasHue() || dst.hasHue())
         {
            // Java: if (this.hasHue() && dst.hasHue() && this.hue() != dst.hue())
            if self.hasHue() && dst.hasHue() && self.hue() != dst.hue()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasLum() || !dst.hasLum()) && (!this.hasLum() || dst.hasLum()))
            else if (self.hasLum() || !dst.hasLum()) && (!self.hasLum() || dst.hasLum())
            {
               // Java: if (this.hasLum() && dst.hasLum() && this.lum() != dst.lum())
               if self.hasLum() && dst.hasLum() && self.lum() != dst.lum()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasSat() || !dst.hasSat()) && (!this.hasSat() || dst.hasSat()))
               else if (self.hasSat() || !dst.hasSat()) && (!self.hasSat() || dst.hasSat())
               {
                  // Java: return !this.hasSat() || !dst.hasSat() || this.sat() == dst.sat();
                  return !self.hasSat() || !dst.hasSat() || self.sat() == dst.sat();
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
      // Java: if (this.hasHue())
      if self.hasHue()
      {
         // Java: ret += Integer.valueOf(this.hue()).hashCode();
         ret = ret.wrapping_add(self.hue());
      }
      // Java: if (this.hasLum())
      if self.hasLum()
      {
         // Java: ret += Double.valueOf(this.lum()).hashCode();
         let bits = self.lum().to_bits() as i64;
         ret = ret.wrapping_add((bits ^ (bits >> 32)) as i32);
      }
      // Java: if (this.hasSat())
      if self.hasSat()
      {
         // Java: ret += Double.valueOf(this.sat()).hashCode();
         let bits = self.sat().to_bits() as i64;
         ret = ret.wrapping_add((bits ^ (bits >> 32)) as i32);
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: return this.map.dirty();
      self.map.dirty()
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
    }

    // Java: void follow(HSLEffect parent)
    pub fn follow(&mut self, mut parent: Box<HSLEffect>) {
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

    // Java: HSLEffect parent()
    pub fn get_parent(&self) -> Option<&HSLEffect> {
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

    // Java: int hue()
    pub fn hue(&self) -> i32 {
      // Java: if (!this.map.has(1))
      if !self.map.has(FIELD_HUE)
      {
         // Java: return this.parent == null ? 0 : this.parent.hue();
         return self.parent.as_ref().map_or(0, |p| p.hue());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(FIELD_HUE);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasHue()
    pub fn hasHue(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasHue();
      self.map.has(FIELD_HUE) || self.parent.as_ref().map_or(false, |p| p.hasHue())
    }

    // Java: void deleteHue()
    pub fn deleteHue(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(FIELD_HUE);
    }

    // Java: void putHue(int value)
    pub fn putHue(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_HUE, Encode::from_int(value, self.zone));
    }

    // Java: double lum()
    pub fn lum(&self) -> f64 {
      // Java: if (!this.map.has(2))
      if !self.map.has(FIELD_LUM)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.lum();
         return self.parent.as_ref().map_or(0.0f64, |p| p.lum());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(FIELD_LUM);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasLum()
    pub fn hasLum(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasLum();
      self.map.has(FIELD_LUM) || self.parent.as_ref().map_or(false, |p| p.hasLum())
    }

    // Java: void deleteLum()
    pub fn deleteLum(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(FIELD_LUM);
    }

    // Java: void putLum(double value)
    pub fn putLum(&mut self, value: f64) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_LUM, Encode::from_double(value, self.zone));
    }

    // Java: double sat()
    pub fn sat(&self) -> f64 {
      // Java: if (!this.map.has(3))
      if !self.map.has(FIELD_SAT)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.sat();
         return self.parent.as_ref().map_or(0.0f64, |p| p.sat());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(FIELD_SAT);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasSat()
    pub fn hasSat(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasSat();
      self.map.has(FIELD_SAT) || self.parent.as_ref().map_or(false, |p| p.hasSat())
    }

    // Java: void deleteSat()
    pub fn deleteSat(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(FIELD_SAT);
    }

    // Java: void putSat(double value)
    pub fn putSat(&mut self, value: f64) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_SAT, Encode::from_double(value, self.zone));
    }

}

impl KoAr for HSLEffect {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { HSLEffect::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { HSLEffect::lastBin(self, var1) }
}

impl KoClosable for HSLEffect {
    fn close(&mut self) { HSLEffect::close(self); }
}
