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

const FIELD_REF: i32 = 1;

// Java: public class EffectReference implements KoAr, KoClosable
pub struct EffectReference {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: EffectReference parent
    pub(crate) parent: Option<Box<EffectReference>>,
}

// Java: public class EffectReference implements KoAr, KoClosable
impl EffectReference {
    // Java: static protected static <Value1 extends EffectReference> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = EffectReference::EffectReference_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static EffectReference create()
    pub fn create() -> Self {
      // Java: return initialize(EffectReference.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static EffectReference open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(EffectReference.class, bin);
      Self::initialize(bin)
    }

    // Java: EffectReference()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      EffectReference { zone: KoZone::defaultZone(), bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: EffectReference(KoBin bin)
    pub fn EffectReference_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      EffectReference { zone: KoZone::defaultZone(), bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(EffectReference value)
    pub fn load_value(&mut self, value: Option<&EffectReference>) {
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

    // Java: boolean equalsLax(EffectReference o)
    pub fn equalsLax(&self, o: &EffectReference) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &EffectReference) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &EffectReference, _isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof EffectReference)
      if true
      {
         // Java: EffectReference dst = (EffectReference)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasRef() || !dst.hasRef()) && (!this.hasRef() || dst.hasRef()))
         else if (self.hasRef() || !dst.hasRef()) && (!self.hasRef() || dst.hasRef())
         {
            // Java: return !this.hasRef() || !dst.hasRef() || this.ref().equals(dst.ref());
            return !self.hasRef() || !dst.hasRef() || self.get_ref() == dst.get_ref();
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
      // Java: if (this.hasRef())
      if self.hasRef()
      {
         // Java: ret += String.valueOf(this.ref()).hashCode();
         let s = self.get_ref();
         let mut h: i32 = 0;
         for c in s.chars() { h = h.wrapping_mul(31).wrapping_add(c as i32); }
         ret = ret.wrapping_add(h);
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

    // Java: void follow(EffectReference parent)
    pub fn follow(&mut self, mut parent: Box<EffectReference>) {
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

    // Java: EffectReference parent()
    pub fn get_parent(&self) -> Option<&EffectReference> {
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

    // Java: String ref()
    pub fn get_ref(&self) -> String {
      // Java: if (!this.map.has(1))
      if !self.map.has(FIELD_REF)
      {
         // Java: return this.parent == null ? "" : this.parent.ref();
         return self.parent.as_ref().map_or(String::new(), |p| p.get_ref());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(FIELD_REF);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasRef()
    pub fn hasRef(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasRef();
      self.map.has(FIELD_REF) || self.parent.as_ref().map_or(false, |p| p.hasRef())
    }

    // Java: void deleteRef()
    pub fn deleteRef(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(FIELD_REF);
    }

    // Java: void putRef(String value)
    pub fn putRef(&mut self, value: &str) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_REF, Encode::from_string(value, self.zone));
    }

}

impl KoAr for EffectReference {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { EffectReference::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { EffectReference::lastBin(self, var1) }
}

impl KoClosable for EffectReference {
    fn close(&mut self) { EffectReference::close(self); }
}
