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

const FIELD_BRIGHT: i32 = 1;
const FIELD_CONTRAST: i32 = 2;

// Java: public class LumEffect implements KoAr, KoClosable
pub struct LumEffect {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: LumEffect parent
    pub(crate) parent: Option<Box<LumEffect>>,
}

// Java: public class LumEffect implements KoAr, KoClosable
impl LumEffect {
    // Java: static protected static <Value1 extends LumEffect> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = LumEffect::LumEffect_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static LumEffect create()
    pub fn create() -> Self {
      // Java: return initialize(LumEffect.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static LumEffect open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(LumEffect.class, bin);
      Self::initialize(bin)
    }

    // Java: LumEffect()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      LumEffect { zone: KoZone::defaultZone(), bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: LumEffect(KoBin bin)
    pub fn LumEffect_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      LumEffect { zone: KoZone::defaultZone(), bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(LumEffect value)
    pub fn load_value(&mut self, value: Option<&LumEffect>) {
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

    // Java: boolean equalsLax(LumEffect o)
    pub fn equalsLax(&self, o: &LumEffect) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &LumEffect) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &LumEffect, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof LumEffect)
      if true
      {
         // Java: LumEffect dst = (LumEffect)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasBright() || !dst.hasBright()) && (!this.hasBright() || dst.hasBright()))
         else if (self.hasBright() || !dst.hasBright()) && (!self.hasBright() || dst.hasBright())
         {
            // Java: if (this.hasBright() && dst.hasBright() && this.bright() != dst.bright())
            if self.hasBright() && dst.hasBright() && self.bright() != dst.bright()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasContrast() || !dst.hasContrast()) && (!this.hasContrast() || dst.hasContrast()))
            else if (self.hasContrast() || !dst.hasContrast()) && (!self.hasContrast() || dst.hasContrast())
            {
               // Java: return !this.hasContrast() || !dst.hasContrast() || this.contrast() == dst.contrast();
               return !self.hasContrast() || !dst.hasContrast() || self.contrast() == dst.contrast();
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
      // Java: if (this.hasBright())
      if self.hasBright()
      {
         // Java: ret += Double.valueOf(this.bright()).hashCode();
         let bits = self.bright().to_bits() as i64;
         ret = ret.wrapping_add((bits ^ (bits >> 32)) as i32);
      }
      // Java: if (this.hasContrast())
      if self.hasContrast()
      {
         // Java: ret += Double.valueOf(this.contrast()).hashCode();
         let bits = self.contrast().to_bits() as i64;
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

    // Java: void follow(LumEffect parent)
    pub fn follow(&mut self, mut parent: Box<LumEffect>) {
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

    // Java: LumEffect parent()
    pub fn get_parent(&self) -> Option<&LumEffect> {
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

    // Java: double bright()
    pub fn bright(&self) -> f64 {
      // Java: if (!this.map.has(1))
      if !self.map.has(FIELD_BRIGHT)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.bright();
         return self.parent.as_ref().map_or(0.0f64, |p| p.bright());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(FIELD_BRIGHT);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasBright()
    pub fn hasBright(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasBright();
      self.map.has(FIELD_BRIGHT) || self.parent.as_ref().map_or(false, |p| p.hasBright())
    }

    // Java: void deleteBright()
    pub fn deleteBright(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(FIELD_BRIGHT);
    }

    // Java: void putBright(double value)
    pub fn putBright(&mut self, value: f64) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_BRIGHT, Encode::from_double(value, self.zone));
    }

    // Java: double contrast()
    pub fn contrast(&self) -> f64 {
      // Java: if (!this.map.has(2))
      if !self.map.has(FIELD_CONTRAST)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.contrast();
         return self.parent.as_ref().map_or(0.0f64, |p| p.contrast());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(FIELD_CONTRAST);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasContrast()
    pub fn hasContrast(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasContrast();
      self.map.has(FIELD_CONTRAST) || self.parent.as_ref().map_or(false, |p| p.hasContrast())
    }

    // Java: void deleteContrast()
    pub fn deleteContrast(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(FIELD_CONTRAST);
    }

    // Java: void putContrast(double value)
    pub fn putContrast(&mut self, value: f64) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_CONTRAST, Encode::from_double(value, self.zone));
    }

}

impl KoAr for LumEffect {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { LumEffect::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { LumEffect::lastBin(self, var1) }
}

impl KoClosable for LumEffect {
    fn close(&mut self) { LumEffect::close(self); }
}
