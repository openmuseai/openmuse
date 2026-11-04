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

const FIELD_CX: i32 = 1;
const FIELD_CY: i32 = 2;

// Java: public class PositiveSize2D implements KoAr, KoClosable
pub struct PositiveSize2D {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: PositiveSize2D parent
    pub(crate) parent: Option<Box<PositiveSize2D>>,
}

// Java: public class PositiveSize2D implements KoAr, KoClosable
impl PositiveSize2D {
    // Java: static protected static <Value1 extends PositiveSize2D> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = PositiveSize2D::PositiveSize2D_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static PositiveSize2D create()
    pub fn create() -> Self {
      // Java: return initialize(PositiveSize2D.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static PositiveSize2D open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(PositiveSize2D.class, bin);
      Self::initialize(bin)
    }

    // Java: PositiveSize2D()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      PositiveSize2D { zone: KoZone::defaultZone(), bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: PositiveSize2D(KoBin bin)
    pub fn PositiveSize2D_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      PositiveSize2D { zone: KoZone::defaultZone(), bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(PositiveSize2D value)
    pub fn load_value(&mut self, value: Option<&PositiveSize2D>) {
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

    // Java: boolean equalsLax(PositiveSize2D o)
    pub fn equalsLax(&self, o: &PositiveSize2D) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &PositiveSize2D) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &PositiveSize2D, _isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof PositiveSize2D)
      if true
      {
         // Java: PositiveSize2D dst = (PositiveSize2D)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasCx() || !dst.hasCx()) && (!this.hasCx() || dst.hasCx()))
         else if (self.hasCx() || !dst.hasCx()) && (!self.hasCx() || dst.hasCx())
         {
            // Java: if (this.hasCx() && dst.hasCx() && this.cx() != dst.cx())
            if self.hasCx() && dst.hasCx() && self.cx() != dst.cx()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasCy() || !dst.hasCy()) && (!this.hasCy() || dst.hasCy()))
            else if (self.hasCy() || !dst.hasCy()) && (!self.hasCy() || dst.hasCy())
            {
               // Java: return !this.hasCy() || !dst.hasCy() || this.cy() == dst.cy();
               return !self.hasCy() || !dst.hasCy() || self.cy() == dst.cy();
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
      // Java: if (this.hasCx())
      if self.hasCx()
      {
         // Java: ret += Integer.valueOf(this.cx()).hashCode();
         ret = ret.wrapping_add(self.cx());
      }
      // Java: if (this.hasCy())
      if self.hasCy()
      {
         // Java: ret += Integer.valueOf(this.cy()).hashCode();
         ret = ret.wrapping_add(self.cy());
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

    // Java: void follow(PositiveSize2D parent)
    pub fn follow(&mut self, mut parent: Box<PositiveSize2D>) {
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

    // Java: PositiveSize2D parent()
    pub fn get_parent(&self) -> Option<&PositiveSize2D> {
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

    // Java: int cx()
    pub fn cx(&self) -> i32 {
      // Java: if (!this.map.has(1))
      if !self.map.has(FIELD_CX)
      {
         // Java: return this.parent == null ? 0 : this.parent.cx();
         return self.parent.as_ref().map_or(0, |p| p.cx());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(FIELD_CX);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasCx()
    pub fn hasCx(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasCx();
      self.map.has(FIELD_CX) || self.parent.as_ref().map_or(false, |p| p.hasCx())
    }

    // Java: void deleteCx()
    pub fn deleteCx(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(FIELD_CX);
    }

    // Java: void putCx(int value)
    pub fn putCx(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_CX, Encode::from_int(value, self.zone));
    }

    // Java: int cy()
    pub fn cy(&self) -> i32 {
      // Java: if (!this.map.has(2))
      if !self.map.has(FIELD_CY)
      {
         // Java: return this.parent == null ? 0 : this.parent.cy();
         return self.parent.as_ref().map_or(0, |p| p.cy());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(FIELD_CY);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasCy()
    pub fn hasCy(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasCy();
      self.map.has(FIELD_CY) || self.parent.as_ref().map_or(false, |p| p.hasCy())
    }

    // Java: void deleteCy()
    pub fn deleteCy(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(FIELD_CY);
    }

    // Java: void putCy(int value)
    pub fn putCy(&mut self, value: i32) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_CY, Encode::from_int(value, self.zone));
    }

}

impl KoAr for PositiveSize2D {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { PositiveSize2D::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { PositiveSize2D::lastBin(self, var1) }
}

impl KoClosable for PositiveSize2D {
    fn close(&mut self) { PositiveSize2D::close(self); }
}
