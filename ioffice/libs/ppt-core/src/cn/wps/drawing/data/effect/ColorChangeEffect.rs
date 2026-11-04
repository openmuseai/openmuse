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

const FIELD_USE_A: i32 = 1;
const FIELD_COLOR_FROM: i32 = 2;
const FIELD_COLOR_TO: i32 = 3;

// Java: public class ColorChangeEffect implements KoAr, KoClosable
pub struct ColorChangeEffect {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Color colorFrom
    pub(crate) colorFrom: Option<Box<dyn KoAr>>,
    // Java field: Color colorTo
    pub(crate) colorTo: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: ColorChangeEffect parent
    pub(crate) parent: Option<Box<ColorChangeEffect>>,
}

// Java: public class ColorChangeEffect implements KoAr, KoClosable
impl ColorChangeEffect {
    // Java: static protected static <Value1 extends ColorChangeEffect> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = ColorChangeEffect::ColorChangeEffect_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static ColorChangeEffect create()
    pub fn create() -> Self {
      // Java: return initialize(ColorChangeEffect.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static ColorChangeEffect open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(ColorChangeEffect.class, bin);
      Self::initialize(bin)
    }

    // Java: ColorChangeEffect()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      ColorChangeEffect { zone: KoZone::defaultZone(), colorFrom: None, colorTo: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: ColorChangeEffect(KoBin bin)
    pub fn ColorChangeEffect_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      ColorChangeEffect { zone: KoZone::defaultZone(), colorFrom: None, colorTo: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(ColorChangeEffect value)
    pub fn load_value(&mut self, value: Option<&ColorChangeEffect>) {
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
      // Java: if (this.colorFrom != null && this.colorFrom.dirty())
      if self.colorFrom.as_ref().map_or(false, |c| c.dirty())
      {
         // Java: this.map.put(2, this.colorFrom.saveBin(zone));
         // TODO: Color type not yet available for saveBin
      }
      // Java: if (this.colorTo != null && this.colorTo.dirty())
      if self.colorTo.as_ref().map_or(false, |c| c.dirty())
      {
         // Java: this.map.put(3, this.colorTo.saveBin(zone));
         // TODO: Color type not yet available for saveBin
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())); }
      self.bin_box.get()
    }

    // Java: boolean equalsLax(ColorChangeEffect o)
    pub fn equalsLax(&self, o: &ColorChangeEffect) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &ColorChangeEffect) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &ColorChangeEffect, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof ColorChangeEffect)
      if true
      {
         // Java: ColorChangeEffect dst = (ColorChangeEffect)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasUseA() || !dst.hasUseA()) && (!this.hasUseA() || dst.hasUseA()))
         else if (self.hasUseA() || !dst.hasUseA()) && (!self.hasUseA() || dst.hasUseA())
         {
            // Java: if (this.hasUseA() && dst.hasUseA() && this.isUseA() != dst.isUseA())
            if self.hasUseA() && dst.hasUseA() && self.isUseA() != dst.isUseA()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasColorFrom() || !dst.hasColorFrom()) && (!this.hasColorFrom() || dst.hasColorFrom()))
            else if (self.hasColorFrom() || !dst.hasColorFrom()) && (!self.hasColorFrom() || dst.hasColorFrom())
            {
               // Java: if (this.hasColorFrom() && dst.hasColorFrom())
               if self.hasColorFrom() && dst.hasColorFrom()
               {
                  // Java: if (isStrict && !this.colorFrom().equals(dst.colorFrom()))
                  if isStrict
                  {
                     // Java: return false;
                     // TODO: Color lazy-init not available
                  }
                  // Java: if (!isStrict && !this.colorFrom().equalsLax(dst.colorFrom()))
                  if !isStrict
                  {
                     // Java: return false;
                     // TODO: Color lazy-init not available
                  }
               }
               // Java: if ((this.hasColorTo() || !dst.hasColorTo()) && (!this.hasColorTo() || dst.hasColorTo()))
               if (self.hasColorTo() || !dst.hasColorTo()) && (!self.hasColorTo() || dst.hasColorTo())
               {
                  // Java: if (this.hasColorTo() && dst.hasColorTo())
                  if self.hasColorTo() && dst.hasColorTo()
                  {
                     // Java: if (isStrict && !this.colorTo().equals(dst.colorTo()))
                     if isStrict
                     {
                        // Java: return false;
                        // TODO: Color lazy-init not available
                     }
                     // Java: if (!isStrict && !this.colorTo().equalsLax(dst.colorTo()))
                     if !isStrict
                     {
                        // Java: return false;
                        // TODO: Color lazy-init not available
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

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasUseA())
      if self.hasUseA()
      {
         // Java: ret += Boolean.valueOf(this.isUseA()).hashCode();
         ret = ret.wrapping_add(if self.isUseA() { 1231 } else { 1237 });
      }
      // Java: if (this.hasColorFrom())
      if self.hasColorFrom()
      {
         // Java: ret += this.colorFrom().hashCode();
         // TODO: Color lazy-init not available
      }
      // Java: if (this.hasColorTo())
      if self.hasColorTo()
      {
         // Java: ret += this.colorTo().hashCode();
         // TODO: Color lazy-init not available
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.colorFrom != null && this.colorFrom.dirty())
      if self.colorFrom.as_ref().map_or(false, |c| c.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
      else
      {
         // Java: return this.colorTo != null && this.colorTo.dirty() ? true : this.map.dirty();
         if self.colorTo.as_ref().map_or(false, |c| c.dirty()) { true } else { self.map.dirty() }
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
      // Java: this.colorFrom = null;
      self.colorFrom = None;
      // Java: this.colorTo = null;
      self.colorTo = None;
    }

    // Java: void follow(ColorChangeEffect parent)
    pub fn follow(&mut self, mut parent: Box<ColorChangeEffect>) {
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

    // Java: ColorChangeEffect parent()
    pub fn get_parent(&self) -> Option<&ColorChangeEffect> {
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

    // Java: boolean isUseA()
    pub fn isUseA(&self) -> bool {
      // Java: if (!this.map.has(1))
      if !self.map.has(FIELD_USE_A)
      {
         // Java: return this.parent == null ? true : this.parent.isUseA();
         return self.parent.as_ref().map_or(true, |p| p.isUseA());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(FIELD_USE_A);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasUseA()
    pub fn hasUseA(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasUseA();
      self.map.has(FIELD_USE_A) || self.parent.as_ref().map_or(false, |p| p.hasUseA())
    }

    // Java: void deleteUseA()
    pub fn deleteUseA(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(FIELD_USE_A);
    }

    // Java: void putUseA(boolean value)
    pub fn putUseA(&mut self, value: bool) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_USE_A, Encode::from_bool(value, self.zone));
    }

    // Java: Color colorFrom()
    pub fn colorFrom(&self) -> &dyn KoAr {
      // Java: if (this.colorFrom == null)
      // Java: if (this.map.has(2))
      // Java: KoBin bin = this.map.byKey(2);
      // Java: this.colorFrom = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
      // Java: else if (this.parent != null && this.parent.hasColorFrom())
      // Java: this.colorFrom = this.parent.colorFrom();
      // Java: else
      // Java: KoBin bin = KoBin.Null.value1();
      // Java: this.colorFrom = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
      // Java: return this.colorFrom;
      todo!("Color lazy-init requires &mut self and Color type not yet available")
    }

    // Java: boolean hasColorFrom()
    pub fn hasColorFrom(&self) -> bool {
      // Java: return this.map.has(2) || this.colorFrom != null && this.colorFrom.dirty() || this.parent != null && this.parent.hasColorFrom();
      self.map.has(FIELD_COLOR_FROM) || self.colorFrom.as_ref().map_or(false, |c| c.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasColorFrom())
    }

    // Java: void deleteColorFrom()
    pub fn deleteColorFrom(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(FIELD_COLOR_FROM);
      // Java: if (this.colorFrom != null)
      if self.colorFrom.is_some()
      {
         // Java: this.colorFrom = null;
         self.colorFrom = None;
      }
    }

    // Java: void writeColorFrom(Color value)
    pub fn writeColorFrom(&mut self, value: &dyn KoAr) {
      // Java: if (value != null)
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(2, bin);
            self.map.put(FIELD_COLOR_FROM, bin);
            // Java: if (this.colorFrom != null)
            if self.colorFrom.is_some()
            {
               // Java: this.colorFrom = null;
               self.colorFrom = None;
            }
         }
      }
    }

    // Java: void forceWriteColorFrom(Color value)
    pub fn forceWriteColorFrom(&mut self, value: &dyn KoAr) {
      // Java: if (value != null)
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(FIELD_COLOR_FROM, value.lastBin(self.zone));
         // Java: if (this.colorFrom != null)
         if self.colorFrom.is_some()
         {
            // Java: this.colorFrom = null;
            self.colorFrom = None;
         }
      }
    }

    // Java: Color colorTo()
    pub fn colorTo(&self) -> &dyn KoAr {
      // Java: if (this.colorTo == null)
      // Java: if (this.map.has(3))
      // Java: KoBin bin = this.map.byKey(3);
      // Java: this.colorTo = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
      // Java: else if (this.parent != null && this.parent.hasColorTo())
      // Java: this.colorTo = this.parent.colorTo();
      // Java: else
      // Java: KoBin bin = KoBin.Null.value1();
      // Java: this.colorTo = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
      // Java: return this.colorTo;
      todo!("Color lazy-init requires &mut self and Color type not yet available")
    }

    // Java: boolean hasColorTo()
    pub fn hasColorTo(&self) -> bool {
      // Java: return this.map.has(3) || this.colorTo != null && this.colorTo.dirty() || this.parent != null && this.parent.hasColorTo();
      self.map.has(FIELD_COLOR_TO) || self.colorTo.as_ref().map_or(false, |c| c.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasColorTo())
    }

    // Java: void deleteColorTo()
    pub fn deleteColorTo(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(FIELD_COLOR_TO);
      // Java: if (this.colorTo != null)
      if self.colorTo.is_some()
      {
         // Java: this.colorTo = null;
         self.colorTo = None;
      }
    }

    // Java: void writeColorTo(Color value)
    pub fn writeColorTo(&mut self, value: &dyn KoAr) {
      // Java: if (value != null)
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(3, bin);
            self.map.put(FIELD_COLOR_TO, bin);
            // Java: if (this.colorTo != null)
            if self.colorTo.is_some()
            {
               // Java: this.colorTo = null;
               self.colorTo = None;
            }
         }
      }
    }

    // Java: void forceWriteColorTo(Color value)
    pub fn forceWriteColorTo(&mut self, value: &dyn KoAr) {
      // Java: if (value != null)
      {
         // Java: this.map.put(3, value.lastBin(this.zone));
         self.map.put(FIELD_COLOR_TO, value.lastBin(self.zone));
         // Java: if (this.colorTo != null)
         if self.colorTo.is_some()
         {
            // Java: this.colorTo = null;
            self.colorTo = None;
         }
      }
    }

}

impl KoAr for ColorChangeEffect {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { ColorChangeEffect::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { ColorChangeEffect::lastBin(self, var1) }
}

impl KoClosable for ColorChangeEffect {
    fn close(&mut self) { ColorChangeEffect::close(self); }
}
