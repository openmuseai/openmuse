#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.color.Color
// TODO: Color not yet available; will use todo!() for color() accessor
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

const FIELD_COLOR: i32 = 1;

// Java: public class AlphaInverseEffect implements KoAr, KoClosable
pub struct AlphaInverseEffect {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Color color
    pub(crate) color: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: AlphaInverseEffect parent
    pub(crate) parent: Option<Box<AlphaInverseEffect>>,
}

// Java: public class AlphaInverseEffect implements KoAr, KoClosable
impl AlphaInverseEffect {
    // Java: static protected static <Value1 extends AlphaInverseEffect> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = AlphaInverseEffect::AlphaInverseEffect_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static AlphaInverseEffect create()
    pub fn create() -> Self {
      // Java: return initialize(AlphaInverseEffect.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static AlphaInverseEffect open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(AlphaInverseEffect.class, bin);
      Self::initialize(bin)
    }

    // Java: AlphaInverseEffect()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      AlphaInverseEffect { zone: KoZone::defaultZone(), color: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: AlphaInverseEffect(KoBin bin)
    pub fn AlphaInverseEffect_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      AlphaInverseEffect { zone: KoZone::defaultZone(), color: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(AlphaInverseEffect value)
    pub fn load_value(&mut self, value: Option<&AlphaInverseEffect>) {
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
      // Java: if (this.color != null && this.color.dirty())
      if self.color.as_ref().map_or(false, |c| c.dirty())
      {
         // Java: this.map.put(1, this.color.saveBin(zone));
         // TODO: Color type not yet available for saveBin
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())); }
      self.bin_box.get()
    }

    // Java: boolean equalsLax(AlphaInverseEffect o)
    pub fn equalsLax(&self, o: &AlphaInverseEffect) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &AlphaInverseEffect) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &AlphaInverseEffect, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof AlphaInverseEffect)
      if true
      {
         // Java: AlphaInverseEffect dst = (AlphaInverseEffect)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasColor() || !dst.hasColor()) && (!this.hasColor() || dst.hasColor()))
         else if (self.hasColor() || !dst.hasColor()) && (!self.hasColor() || dst.hasColor())
         {
            // Java: if (this.hasColor() && dst.hasColor())
            if self.hasColor() && dst.hasColor()
            {
               // Java: if (isStrict && !this.color().equals(dst.color()))
               if isStrict
               {
                  // Java: return false;
                  // TODO: Color lazy-init not available
               }
               // Java: if (!isStrict && !this.color().equalsLax(dst.color()))
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
      // Java: if (this.hasColor())
      if self.hasColor()
      {
         // Java: ret += this.color().hashCode();
         // TODO: Color lazy-init not available
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: return this.color != null && this.color.dirty() ? true : this.map.dirty();
      if self.color.as_ref().map_or(false, |c| c.dirty()) { true } else { self.map.dirty() }
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
      // Java: this.color = null;
      self.color = None;
    }

    // Java: void follow(AlphaInverseEffect parent)
    pub fn follow(&mut self, mut parent: Box<AlphaInverseEffect>) {
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

    // Java: AlphaInverseEffect parent()
    pub fn get_parent(&self) -> Option<&AlphaInverseEffect> {
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

    // Java: Color color()
    pub fn color(&self) -> &dyn KoAr {
      // Java: if (this.color == null)
      // Java: if (this.map.has(1))
      // Java: KoBin bin = this.map.byKey(1);
      // Java: this.color = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
      // Java: else if (this.parent != null && this.parent.hasColor())
      // Java: this.color = this.parent.color();
      // Java: else
      // Java: KoBin bin = KoBin.Null.value1();
      // Java: this.color = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
      // Java: return this.color;
      todo!("Color lazy-init requires &mut self and Color type not yet available")
    }

    // Java: boolean hasColor()
    pub fn hasColor(&self) -> bool {
      // Java: return this.map.has(1) || this.color != null && this.color.dirty() || this.parent != null && this.parent.hasColor();
      self.map.has(FIELD_COLOR) || self.color.as_ref().map_or(false, |c| c.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasColor())
    }

    // Java: void deleteColor()
    pub fn deleteColor(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(FIELD_COLOR);
      // Java: if (this.color != null)
      if self.color.is_some()
      {
         // Java: this.color = null;
         self.color = None;
      }
    }

    // Java: void writeColor(Color value)
    pub fn writeColor(&mut self, value: &dyn KoAr) {
      // Java: if (value != null)
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = value.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(1, bin);
            self.map.put(FIELD_COLOR, bin);
            // Java: if (this.color != null)
            if self.color.is_some()
            {
               // Java: this.color = null;
               self.color = None;
            }
         }
      }
    }

    // Java: void forceWriteColor(Color value)
    pub fn forceWriteColor(&mut self, value: &dyn KoAr) {
      // Java: if (value != null)
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(FIELD_COLOR, value.lastBin(self.zone));
         // Java: if (this.color != null)
         if self.color.is_some()
         {
            // Java: this.color = null;
            self.color = None;
         }
      }
    }

}

impl KoAr for AlphaInverseEffect {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { AlphaInverseEffect::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { AlphaInverseEffect::lastBin(self, var1) }
}

impl KoClosable for AlphaInverseEffect {
    fn close(&mut self) { AlphaInverseEffect::close(self); }
}
