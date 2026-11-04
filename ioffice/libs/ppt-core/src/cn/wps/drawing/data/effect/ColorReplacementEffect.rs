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

const FIELD_COLOR_REPLACE: i32 = 1;

// Java: public class ColorReplacementEffect implements KoAr, KoClosable
pub struct ColorReplacementEffect {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Color colorReplace
    pub(crate) colorReplace: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: ColorReplacementEffect parent
    pub(crate) parent: Option<Box<ColorReplacementEffect>>,
}

// Java: public class ColorReplacementEffect implements KoAr, KoClosable
impl ColorReplacementEffect {
    // Java: static protected static <Value1 extends ColorReplacementEffect> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = ColorReplacementEffect::ColorReplacementEffect_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static ColorReplacementEffect create()
    pub fn create() -> Self {
      // Java: return initialize(ColorReplacementEffect.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static ColorReplacementEffect open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(ColorReplacementEffect.class, bin);
      Self::initialize(bin)
    }

    // Java: ColorReplacementEffect()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      ColorReplacementEffect { zone: KoZone::defaultZone(), colorReplace: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: ColorReplacementEffect(KoBin bin)
    pub fn ColorReplacementEffect_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      ColorReplacementEffect { zone: KoZone::defaultZone(), colorReplace: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(ColorReplacementEffect value)
    pub fn load_value(&mut self, value: Option<&ColorReplacementEffect>) {
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
      // Java: if (this.colorReplace != null && this.colorReplace.dirty())
      if self.colorReplace.as_ref().map_or(false, |c| c.dirty())
      {
         // Java: this.map.put(1, this.colorReplace.saveBin(zone));
         // TODO: Color type not yet available for saveBin
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())); }
      self.bin_box.get()
    }

    // Java: boolean equalsLax(ColorReplacementEffect o)
    pub fn equalsLax(&self, o: &ColorReplacementEffect) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &ColorReplacementEffect) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &ColorReplacementEffect, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof ColorReplacementEffect)
      if true
      {
         // Java: ColorReplacementEffect dst = (ColorReplacementEffect)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasColorReplace() || !dst.hasColorReplace()) && (!this.hasColorReplace() || dst.hasColorReplace()))
         else if (self.hasColorReplace() || !dst.hasColorReplace()) && (!self.hasColorReplace() || dst.hasColorReplace())
         {
            // Java: if (this.hasColorReplace() && dst.hasColorReplace())
            if self.hasColorReplace() && dst.hasColorReplace()
            {
               // Java: if (isStrict && !this.colorReplace().equals(dst.colorReplace()))
               if isStrict
               {
                  // Java: return false;
                  // TODO: Color lazy-init not available
               }
               // Java: if (!isStrict && !this.colorReplace().equalsLax(dst.colorReplace()))
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

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasColorReplace())
      if self.hasColorReplace()
      {
         // Java: ret += this.colorReplace().hashCode();
         // TODO: Color lazy-init not available
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: return this.colorReplace != null && this.colorReplace.dirty() ? true : this.map.dirty();
      if self.colorReplace.as_ref().map_or(false, |c| c.dirty()) { true } else { self.map.dirty() }
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
      // Java: this.colorReplace = null;
      self.colorReplace = None;
    }

    // Java: void follow(ColorReplacementEffect parent)
    pub fn follow(&mut self, mut parent: Box<ColorReplacementEffect>) {
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

    // Java: ColorReplacementEffect parent()
    pub fn get_parent(&self) -> Option<&ColorReplacementEffect> {
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

    // Java: Color colorReplace()
    pub fn colorReplace(&self) -> &dyn KoAr {
      // Java: if (this.colorReplace == null)
      // Java: if (this.map.has(1))
      // Java: KoBin bin = this.map.byKey(1);
      // Java: this.colorReplace = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
      // Java: else if (this.parent != null && this.parent.hasColorReplace())
      // Java: this.colorReplace = this.parent.colorReplace();
      // Java: else
      // Java: KoBin bin = KoBin.Null.value1();
      // Java: this.colorReplace = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
      // Java: return this.colorReplace;
      todo!("Color lazy-init requires &mut self and Color type not yet available")
    }

    // Java: boolean hasColorReplace()
    pub fn hasColorReplace(&self) -> bool {
      // Java: return this.map.has(1) || this.colorReplace != null && this.colorReplace.dirty() || this.parent != null && this.parent.hasColorReplace();
      self.map.has(FIELD_COLOR_REPLACE) || self.colorReplace.as_ref().map_or(false, |c| c.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasColorReplace())
    }

    // Java: void deleteColorReplace()
    pub fn deleteColorReplace(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(FIELD_COLOR_REPLACE);
      // Java: if (this.colorReplace != null)
      if self.colorReplace.is_some()
      {
         // Java: this.colorReplace = null;
         self.colorReplace = None;
      }
    }

    // Java: void writeColorReplace(Color value)
    pub fn writeColorReplace(&mut self, value: &dyn KoAr) {
      // Java: if (value != null)
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(1, bin);
            self.map.put(FIELD_COLOR_REPLACE, bin);
            // Java: if (this.colorReplace != null)
            if self.colorReplace.is_some()
            {
               // Java: this.colorReplace = null;
               self.colorReplace = None;
            }
         }
      }
    }

    // Java: void forceWriteColorReplace(Color value)
    pub fn forceWriteColorReplace(&mut self, value: &dyn KoAr) {
      // Java: if (value != null)
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(FIELD_COLOR_REPLACE, value.lastBin(self.zone));
         // Java: if (this.colorReplace != null)
         if self.colorReplace.is_some()
         {
            // Java: this.colorReplace = null;
            self.colorReplace = None;
         }
      }
    }

}

impl KoAr for ColorReplacementEffect {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { ColorReplacementEffect::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { ColorReplacementEffect::lastBin(self, var1) }
}

impl KoClosable for ColorReplacementEffect {
    fn close(&mut self) { ColorReplacementEffect::close(self); }
}
