#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.color.Color
// TODO: Color not yet available
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

const FIELD_CLR1: i32 = 1;
const FIELD_CLR2: i32 = 2;

// Java: public class DuotoneEffect implements KoAr, KoClosable
pub struct DuotoneEffect {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Color clr1
    pub(crate) clr1: Option<Box<dyn KoAr>>,
    // Java field: Color clr2
    pub(crate) clr2: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: DuotoneEffect parent
    pub(crate) parent: Option<Box<DuotoneEffect>>,
}

// Java: public class DuotoneEffect implements KoAr, KoClosable
impl DuotoneEffect {
    // Java: static protected static <Value1 extends DuotoneEffect> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = DuotoneEffect::DuotoneEffect_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static DuotoneEffect create()
    pub fn create() -> Self {
      // Java: return initialize(DuotoneEffect.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static DuotoneEffect open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(DuotoneEffect.class, bin);
      Self::initialize(bin)
    }

    // Java: DuotoneEffect()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      DuotoneEffect { zone: KoZone::defaultZone(), clr1: None, clr2: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: DuotoneEffect(KoBin bin)
    pub fn DuotoneEffect_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      DuotoneEffect { zone: KoZone::defaultZone(), clr1: None, clr2: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(DuotoneEffect value)
    pub fn load_value(&mut self, value: Option<&DuotoneEffect>) {
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
      // Java: if (this.clr1 != null && this.clr1.dirty())
      if self.clr1.as_ref().map_or(false, |c| c.dirty())
      {
         // Java: this.map.put(1, this.clr1.saveBin(zone));
      }
      // Java: if (this.clr2 != null && this.clr2.dirty())
      if self.clr2.as_ref().map_or(false, |c| c.dirty())
      {
         // Java: this.map.put(2, this.clr2.saveBin(zone));
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())); }
      self.bin_box.get()
    }

    // Java: boolean equalsLax(DuotoneEffect o)
    pub fn equalsLax(&self, o: &DuotoneEffect) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &DuotoneEffect) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &DuotoneEffect, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof DuotoneEffect)
      if true
      {
         // Java: DuotoneEffect dst = (DuotoneEffect)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasClr1() || !dst.hasClr1()) && (!this.hasClr1() || dst.hasClr1()))
         else if (self.hasClr1() || !dst.hasClr1()) && (!self.hasClr1() || dst.hasClr1())
         {
            // Java: if (this.hasClr1() && dst.hasClr1())
            if self.hasClr1() && dst.hasClr1()
            {
               // Java: if (isStrict && !this.clr1().equals(dst.clr1()))
               if isStrict
               {
                  // Java: return false;
               }
               // Java: if (!isStrict && !this.clr1().equalsLax(dst.clr1()))
               if !isStrict
               {
                  // Java: return false;
               }
            }
            // Java: if ((this.hasClr2() || !dst.hasClr2()) && (!this.hasClr2() || dst.hasClr2()))
            if (self.hasClr2() || !dst.hasClr2()) && (!self.hasClr2() || dst.hasClr2())
            {
               // Java: if (this.hasClr2() && dst.hasClr2())
               if self.hasClr2() && dst.hasClr2()
               {
                  // Java: if (isStrict && !this.clr2().equals(dst.clr2()))
                  if isStrict
                  {
                     // Java: return false;
                  }
                  // Java: if (!isStrict && !this.clr2().equalsLax(dst.clr2()))
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

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasClr1())
      if self.hasClr1()
      {
         // Java: ret += this.clr1().hashCode();
      }
      // Java: if (this.hasClr2())
      if self.hasClr2()
      {
         // Java: ret += this.clr2().hashCode();
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.clr1 != null && this.clr1.dirty())
      if self.clr1.as_ref().map_or(false, |c| c.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
      else
      {
         // Java: return this.clr2 != null && this.clr2.dirty() ? true : this.map.dirty();
         if self.clr2.as_ref().map_or(false, |c| c.dirty()) { true } else { self.map.dirty() }
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
      // Java: this.clr1 = null;
      self.clr1 = None;
      // Java: this.clr2 = null;
      self.clr2 = None;
    }

    // Java: void follow(DuotoneEffect parent)
    pub fn follow(&mut self, mut parent: Box<DuotoneEffect>) {
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

    // Java: DuotoneEffect parent()
    pub fn get_parent(&self) -> Option<&DuotoneEffect> {
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

    // Java: Color clr1()
    pub fn clr1(&self) -> &dyn KoAr {
      // Java: if (this.clr1 == null)
      // Java: if (this.map.has(1))
      // Java: KoBin bin = this.map.byKey(1);
      // Java: this.clr1 = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
      // Java: else if (this.parent != null && this.parent.hasClr1())
      // Java: this.clr1 = this.parent.clr1();
      // Java: else
      // Java: KoBin bin = KoBin.Null.value1();
      // Java: this.clr1 = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
      // Java: return this.clr1;
      todo!("Color lazy-init requires &mut self and Color type not yet available")
    }

    // Java: boolean hasClr1()
    pub fn hasClr1(&self) -> bool {
      // Java: return this.map.has(1) || this.clr1 != null && this.clr1.dirty() || this.parent != null && this.parent.hasClr1();
      self.map.has(FIELD_CLR1) || self.clr1.as_ref().map_or(false, |c| c.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasClr1())
    }

    // Java: void deleteClr1()
    pub fn deleteClr1(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(FIELD_CLR1);
      // Java: if (this.clr1 != null)
      if self.clr1.is_some()
      {
         // Java: this.clr1 = null;
         self.clr1 = None;
      }
    }

    // Java: void writeClr1(Color value)
    pub fn writeClr1(&mut self, value: &dyn KoAr) {
      // Java: if (value != null)
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(1, bin);
            self.map.put(FIELD_CLR1, bin);
            // Java: if (this.clr1 != null)
            if self.clr1.is_some()
            {
               // Java: this.clr1 = null;
               self.clr1 = None;
            }
         }
      }
    }

    // Java: void forceWriteClr1(Color value)
    pub fn forceWriteClr1(&mut self, value: &dyn KoAr) {
      // Java: if (value != null)
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(FIELD_CLR1, value.lastBin(self.zone));
         // Java: if (this.clr1 != null)
         if self.clr1.is_some()
         {
            // Java: this.clr1 = null;
            self.clr1 = None;
         }
      }
    }

    // Java: Color clr2()
    pub fn clr2(&self) -> &dyn KoAr {
      // Java: if (this.clr2 == null)
      // Java: if (this.map.has(2))
      // Java: KoBin bin = this.map.byKey(2);
      // Java: this.clr2 = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
      // Java: else if (this.parent != null && this.parent.hasClr2())
      // Java: this.clr2 = this.parent.clr2();
      // Java: else
      // Java: KoBin bin = KoBin.Null.value1();
      // Java: this.clr2 = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
      // Java: return this.clr2;
      todo!("Color lazy-init requires &mut self and Color type not yet available")
    }

    // Java: boolean hasClr2()
    pub fn hasClr2(&self) -> bool {
      // Java: return this.map.has(2) || this.clr2 != null && this.clr2.dirty() || this.parent != null && this.parent.hasClr2();
      self.map.has(FIELD_CLR2) || self.clr2.as_ref().map_or(false, |c| c.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasClr2())
    }

    // Java: void deleteClr2()
    pub fn deleteClr2(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(FIELD_CLR2);
      // Java: if (this.clr2 != null)
      if self.clr2.is_some()
      {
         // Java: this.clr2 = null;
         self.clr2 = None;
      }
    }

    // Java: void writeClr2(Color value)
    pub fn writeClr2(&mut self, value: &dyn KoAr) {
      // Java: if (value != null)
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(2, bin);
            self.map.put(FIELD_CLR2, bin);
            // Java: if (this.clr2 != null)
            if self.clr2.is_some()
            {
               // Java: this.clr2 = null;
               self.clr2 = None;
            }
         }
      }
    }

    // Java: void forceWriteClr2(Color value)
    pub fn forceWriteClr2(&mut self, value: &dyn KoAr) {
      // Java: if (value != null)
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(FIELD_CLR2, value.lastBin(self.zone));
         // Java: if (this.clr2 != null)
         if self.clr2.is_some()
         {
            // Java: this.clr2 = null;
            self.clr2 = None;
         }
      }
    }

}

impl KoAr for DuotoneEffect {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { DuotoneEffect::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { DuotoneEffect::lastBin(self, var1) }
}

impl KoClosable for DuotoneEffect {
    fn close(&mut self) { DuotoneEffect::close(self); }
}
