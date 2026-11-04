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

const FIELD_THRESH: i32 = 1;

// Java: public class AlphaBiLevelEffect implements KoAr, KoClosable
pub struct AlphaBiLevelEffect {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: AlphaBiLevelEffect parent
    pub(crate) parent: Option<Box<AlphaBiLevelEffect>>,
}

// Java: public class AlphaBiLevelEffect implements KoAr, KoClosable
impl AlphaBiLevelEffect {
    // Java: static protected static <Value1 extends AlphaBiLevelEffect> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = AlphaBiLevelEffect::AlphaBiLevelEffect_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static AlphaBiLevelEffect create()
    pub fn create() -> Self {
      // Java: return initialize(AlphaBiLevelEffect.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static AlphaBiLevelEffect open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(AlphaBiLevelEffect.class, bin);
      Self::initialize(bin)
    }

    // Java: AlphaBiLevelEffect()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      AlphaBiLevelEffect { zone: KoZone::defaultZone(), bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: AlphaBiLevelEffect(KoBin bin)
    pub fn AlphaBiLevelEffect_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      AlphaBiLevelEffect { zone: KoZone::defaultZone(), bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(AlphaBiLevelEffect value)
    pub fn load_value(&mut self, value: Option<&AlphaBiLevelEffect>) {
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

    // Java: boolean equalsLax(AlphaBiLevelEffect o)
    pub fn equalsLax(&self, o: &AlphaBiLevelEffect) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &AlphaBiLevelEffect) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &AlphaBiLevelEffect, _isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof AlphaBiLevelEffect)
      if true
      {
         // Java: AlphaBiLevelEffect dst = (AlphaBiLevelEffect)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasThresh() || !dst.hasThresh()) && (!this.hasThresh() || dst.hasThresh()))
         else if (self.hasThresh() || !dst.hasThresh()) && (!self.hasThresh() || dst.hasThresh())
         {
            // Java: return !this.hasThresh() || !dst.hasThresh() || this.thresh() == dst.thresh();
            return !self.hasThresh() || !dst.hasThresh() || self.thresh() == dst.thresh();
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
      // Java: if (this.hasThresh())
      if self.hasThresh()
      {
         // Java: ret += Double.valueOf(this.thresh()).hashCode();
         let bits = self.thresh().to_bits() as i64;
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

    // Java: void follow(AlphaBiLevelEffect parent)
    pub fn follow(&mut self, mut parent: Box<AlphaBiLevelEffect>) {
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

    // Java: AlphaBiLevelEffect parent()
    pub fn get_parent(&self) -> Option<&AlphaBiLevelEffect> {
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

    // Java: double thresh()
    pub fn thresh(&self) -> f64 {
      // Java: if (!this.map.has(1))
      if !self.map.has(FIELD_THRESH)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.thresh();
         return self.parent.as_ref().map_or(0.0f64, |p| p.thresh());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(FIELD_THRESH);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasThresh()
    pub fn hasThresh(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasThresh();
      self.map.has(FIELD_THRESH) || self.parent.as_ref().map_or(false, |p| p.hasThresh())
    }

    // Java: void deleteThresh()
    pub fn deleteThresh(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(FIELD_THRESH);
    }

    // Java: void putThresh(double value)
    pub fn putThresh(&mut self, value: f64) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_THRESH, Encode::from_double(value, self.zone));
    }

}

impl KoAr for AlphaBiLevelEffect {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { AlphaBiLevelEffect::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { AlphaBiLevelEffect::lastBin(self, var1) }
}

impl KoClosable for AlphaBiLevelEffect {
    fn close(&mut self) { AlphaBiLevelEffect::close(self); }
}
