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

const FIELD_GROW: i32 = 1;
const FIELD_RADIUS: i32 = 2;

// Java: public class BlurEffect implements KoAr, KoClosable
pub struct BlurEffect {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: BlurEffect parent
    pub(crate) parent: Option<Box<BlurEffect>>,
}

// Java: public class BlurEffect implements KoAr, KoClosable
impl BlurEffect {
    // Java: static protected static <Value1 extends BlurEffect> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = BlurEffect::BlurEffect_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static BlurEffect create()
    pub fn create() -> Self {
      // Java: return initialize(BlurEffect.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static BlurEffect open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(BlurEffect.class, bin);
      Self::initialize(bin)
    }

    // Java: BlurEffect()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      BlurEffect { zone: KoZone::defaultZone(), bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: BlurEffect(KoBin bin)
    pub fn BlurEffect_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      BlurEffect { zone: KoZone::defaultZone(), bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(BlurEffect value)
    pub fn load_value(&mut self, value: Option<&BlurEffect>) {
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

    // Java: boolean equalsLax(BlurEffect o)
    pub fn equalsLax(&self, o: &BlurEffect) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &BlurEffect) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &BlurEffect, _isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof BlurEffect)
      if true
      {
         // Java: BlurEffect dst = (BlurEffect)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasGrow() || !dst.hasGrow()) && (!this.hasGrow() || dst.hasGrow()))
         else if (self.hasGrow() || !dst.hasGrow()) && (!self.hasGrow() || dst.hasGrow())
         {
            // Java: if (this.hasGrow() && dst.hasGrow() && this.isGrow() != dst.isGrow())
            if self.hasGrow() && dst.hasGrow() && self.isGrow() != dst.isGrow()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasRadius() || !dst.hasRadius()) && (!this.hasRadius() || dst.hasRadius()))
            else if (self.hasRadius() || !dst.hasRadius()) && (!self.hasRadius() || dst.hasRadius())
            {
               // Java: return !this.hasRadius() || !dst.hasRadius() || this.radius() == dst.radius();
               return !self.hasRadius() || !dst.hasRadius() || self.radius() == dst.radius();
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
      // Java: if (this.hasGrow())
      if self.hasGrow()
      {
         // Java: ret += Boolean.valueOf(this.isGrow()).hashCode();
         ret = ret.wrapping_add(if self.isGrow() { 1231 } else { 1237 });
      }
      // Java: if (this.hasRadius())
      if self.hasRadius()
      {
         // Java: ret += Integer.valueOf(this.radius()).hashCode();
         ret = ret.wrapping_add(self.radius());
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

    // Java: void follow(BlurEffect parent)
    pub fn follow(&mut self, mut parent: Box<BlurEffect>) {
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

    // Java: BlurEffect parent()
    pub fn get_parent(&self) -> Option<&BlurEffect> {
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

    // Java: boolean isGrow()
    pub fn isGrow(&self) -> bool {
      // Java: if (!this.map.has(1))
      if !self.map.has(FIELD_GROW)
      {
         // Java: return this.parent == null ? true : this.parent.isGrow();
         return self.parent.as_ref().map_or(true, |p| p.isGrow());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(FIELD_GROW);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasGrow()
    pub fn hasGrow(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasGrow();
      self.map.has(FIELD_GROW) || self.parent.as_ref().map_or(false, |p| p.hasGrow())
    }

    // Java: void deleteGrow()
    pub fn deleteGrow(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(FIELD_GROW);
    }

    // Java: void putGrow(boolean value)
    pub fn putGrow(&mut self, value: bool) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_GROW, Encode::from_bool(value, self.zone));
    }

    // Java: int radius()
    pub fn radius(&self) -> i32 {
      // Java: if (!this.map.has(2))
      if !self.map.has(FIELD_RADIUS)
      {
         // Java: return this.parent == null ? 0 : this.parent.radius();
         return self.parent.as_ref().map_or(0, |p| p.radius());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(FIELD_RADIUS);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasRadius()
    pub fn hasRadius(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasRadius();
      self.map.has(FIELD_RADIUS) || self.parent.as_ref().map_or(false, |p| p.hasRadius())
    }

    // Java: void deleteRadius()
    pub fn deleteRadius(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(FIELD_RADIUS);
    }

    // Java: void putRadius(int value)
    pub fn putRadius(&mut self, value: i32) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_RADIUS, Encode::from_int(value, self.zone));
    }

}

impl KoAr for BlurEffect {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { BlurEffect::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { BlurEffect::lastBin(self, var1) }
}

impl KoClosable for BlurEffect {
    fn close(&mut self) { BlurEffect::close(self); }
}
