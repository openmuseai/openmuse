#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.effect.EffectProperties
use crate::cn::wps::drawing::data::effect::EffectProperties::EffectProperties;
// Java: import cn.wps.drawing.data.style.StyleMatrixReference
use crate::cn::wps::drawing::data::style::StyleMatrixReference::StyleMatrixReference;
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

// Java: public class ThemeableEffectStyle implements KoAr, KoClosable
pub struct ThemeableEffectStyle {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: EffectProperties effect
    pub(crate) effect: Option<Box<EffectProperties>>,
    // Java field: StyleMatrixReference effectRef
    pub(crate) effectRef: Option<Box<StyleMatrixReference>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: ThemeableEffectStyle parent
    pub(crate) parent: Option<Box<ThemeableEffectStyle>>,
}

// Java: public class ThemeableEffectStyle implements KoAr, KoClosable
impl ThemeableEffectStyle {
    // Java: static protected static <Value1 extends ThemeableEffectStyle> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::ThemeableEffectStyle_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static ThemeableEffectStyle create()
    pub fn create() -> Self {
      // Java: return initialize(ThemeableEffectStyle.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static ThemeableEffectStyle open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(ThemeableEffectStyle.class, bin);
      Self::initialize(bin)
    }

    // Java: ThemeableEffectStyle()
    pub fn new() -> Self {
      Self {
      // Java: this.box = new KpBinBox();
      bin_box: KpBinBox::new(),
      // Java: this.map = new KpMap();
      map: KpMap::new(),
      zone: KoZone::defaultZone(), effect: None, effectRef: None, parent: None,
      }
    }

    // Java: ThemeableEffectStyle(KoBin bin)
    pub fn ThemeableEffectStyle_from(bin: &dyn KoBinTrait) -> Self {
      Self {
      // Java: this.box = new KpBinBox(bin);
      bin_box: KpBinBox::KpBinBox_from(Some(bin)),
      // Java: this.map = new KpMap(bin);
      map: KpMap::KpMap_from(bin),
      zone: KoZone::defaultZone(), effect: None, effectRef: None, parent: None,
      }
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
      let bin_ref = self.bin_box.get();
      self.map.load(bin_ref.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone;
      self.loadBin_with_zone(bin, zone);
    }

    // Java: void load(ThemeableEffectStyle value)
    pub fn load_value(&mut self, value: Option<&ThemeableEffectStyle>) {
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
      // Java: if (this.effect != null && this.effect.dirty())
        if self.effect.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(1, this.effect.saveBin(zone));
         let bin = self.effect.as_mut().unwrap().saveBin(zone);
         self.map.put(1, bin);
      }
      // Java: if (this.effectRef != null && this.effectRef.dirty())
        if self.effectRef.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(2, this.effectRef.saveBin(zone));
         let bin = self.effectRef.as_mut().unwrap().saveBin(zone);
         self.map.put(2, bin);
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() {
         let saved = self.map.save(zone);
         self.bin_box.set(Some(saved.as_ref()));
         self.bin_box.get()
      } else {
         self.bin_box.get()
      }
    }

    // Java: boolean equalsLax(ThemeableEffectStyle o)
    pub fn equalsLax(&self, o: &ThemeableEffectStyle) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &ThemeableEffectStyle) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &ThemeableEffectStyle, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof ThemeableEffectStyle)
        if true
      {
         // Java: ThemeableEffectStyle dst = (ThemeableEffectStyle)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasEffect() || !dst.hasEffect()) && (!this.hasEffect() || dst.hasEffect()))
            else if (self.hasEffect() || !dst.hasEffect()) && (!self.hasEffect() || dst.hasEffect())
         {
            // Java: if (this.hasEffect() && dst.hasEffect())
                if self.hasEffect() && dst.hasEffect()
            {
               // Java: if (isStrict && !this.effect().equals(dst.effect()))
                    if isStrict && !self.get_effect().equals(dst.get_effect())
               {
                  // Java: return false;
                  return false;
               }
               // Java: if (!isStrict && !this.effect().equalsLax(dst.effect()))
                    if !isStrict && !self.get_effect().equalsLax(dst.get_effect())
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: if ((this.hasEffectRef() || !dst.hasEffectRef()) && (!this.hasEffectRef() || dst.hasEffectRef()))
                if (self.hasEffectRef() || !dst.hasEffectRef()) && (!self.hasEffectRef() || dst.hasEffectRef())
            {
               // Java: if (this.hasEffectRef() && dst.hasEffectRef())
                    if self.hasEffectRef() && dst.hasEffectRef()
               {
                  // Java: if (isStrict && !this.effectRef().equals(dst.effectRef()))
                        if isStrict && !self.get_effectRef().equals(dst.get_effectRef())
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !this.effectRef().equalsLax(dst.effectRef()))
                        if !isStrict && !self.get_effectRef().equalsLax(dst.get_effectRef())
                  {
                     // Java: return false;
                     return false;
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
    pub fn hashCode(&mut self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasEffect())
        if self.hasEffect()
      {
         // Java: ret += this.effect().hashCode();
         ret = ret.wrapping_add(self.get_effect().hashCode());
      }
      // Java: if (this.hasEffectRef())
        if self.hasEffectRef()
      {
         // Java: ret += this.effectRef().hashCode();
         ret = ret.wrapping_add(self.get_effectRef_mut().hashCode());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.effect != null && this.effect.dirty())
        if self.effect.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
        else
      {
         // Java: return this.effectRef != null && this.effectRef.dirty() ? true : this.map.dirty();
         if self.effectRef.as_ref().map_or(false, |s| s.dirty()) { true } else { self.map.dirty() }
      }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
      // Java: this.saveBin(this.zone);
      let zone = self.zone;
      self.saveBin(zone);
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
    pub fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
      // Java: this.effect = null;
      self.effect = None;
      // Java: this.effectRef = null;
      self.effectRef = None;
    }

    // Java: void follow(ThemeableEffectStyle parent)
    pub fn follow(&mut self, parent: Option<Box<ThemeableEffectStyle>>) {
      // Java: if (parent != null)
        if let Some(mut p) = parent
      {
         // Java: parent.discard();
         p.discard();
         // Java: this.discard();
         self.discard();
         // Java: this.parent = parent;
         self.parent = Some(p);
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

    // Java: ThemeableEffectStyle parent()
    pub fn get_parent(&self) -> Option<&ThemeableEffectStyle> {
      // Java: return this.parent;
      self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      let zone = self.zone;
      self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    pub fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    pub fn get_effectRef_mut(&mut self) -> &mut StyleMatrixReference {
      if self.effectRef.is_none() {
        if self.map.has(2) {
          todo!("lazy init effectRef from map");
        } else if self.parent.as_ref().map_or(false, |p| p.hasEffectRef()) {
          todo!("copy effectRef from parent");
        } else {
          todo!("default init effectRef");
        }
      }
      self.effectRef.as_deref_mut().unwrap()
    }

    // Java: EffectProperties effect()
    pub fn get_effect(&self) -> &EffectProperties {
      // Java: if (this.effect == null)
        if self.effect.is_none()
      {
         // Java: if (this.map.has(1))
            if self.map.has(1)
         {
            // Java: KoBin bin = this.map.byKey(1);
            // Java: this.effect = (EffectProperties)KoBin.Decode.toAny(EffectProperties.class, bin, this.zone);
            todo!("lazy init effect from map");
         }
         // Java: else if (this.parent != null && this.parent.hasEffect())
            else if self.parent.as_ref().map_or(false, |p| p.hasEffect())
         {
            // Java: this.effect = this.parent.effect();
            todo!("copy effect from parent");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.effect = (EffectProperties)KoBin.Decode.toAny(EffectProperties.class, bin, this.zone);
            todo!("default init effect");
         }
      }
      // Java: return this.effect;
      self.effect.as_deref().unwrap()
    }

    // Java: boolean hasEffect()
    pub fn hasEffect(&self) -> bool {
      // Java: return this.map.has(1) || this.effect != null && this.effect.dirty() || this.parent != null && this.parent.hasEffect();
      self.map.has(1) || self.effect.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasEffect())
    }

    // Java: void deleteEffect()
    pub fn deleteEffect(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
      // Java: if (this.effect != null)
        if self.effect.is_some()
      {
         // Java: this.effect = null;
         self.effect = None;
      }
    }

    // Java: void writeEffect(EffectProperties value)
    pub fn writeEffect(&mut self, value: Option<&EffectProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(1, bin);
            self.map.put(1, bin);
            // Java: if (this.effect != null)
                if self.effect.is_some()
            {
               // Java: this.effect = null;
               self.effect = None;
            }
         }
      }
    }

    // Java: void forceWriteEffect(EffectProperties value)
    pub fn forceWriteEffect(&mut self, value: Option<&EffectProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(1, v.lastBin(self.zone));
         // Java: if (this.effect != null)
            if self.effect.is_some()
         {
            // Java: this.effect = null;
            self.effect = None;
         }
      }
    }

    // Java: StyleMatrixReference effectRef()
    pub fn get_effectRef(&self) -> &StyleMatrixReference {
      // Java: if (this.effectRef == null)
        if self.effectRef.is_none()
      {
         // Java: if (this.map.has(2))
            if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            // Java: this.effectRef = (StyleMatrixReference)KoBin.Decode.toAny(StyleMatrixReference.class, bin, this.zone);
            todo!("lazy init effectRef from map");
         }
         // Java: else if (this.parent != null && this.parent.hasEffectRef())
            else if self.parent.as_ref().map_or(false, |p| p.hasEffectRef())
         {
            // Java: this.effectRef = this.parent.effectRef();
            todo!("copy effectRef from parent");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.effectRef = (StyleMatrixReference)KoBin.Decode.toAny(StyleMatrixReference.class, bin, this.zone);
            todo!("default init effectRef");
         }
      }
      // Java: return this.effectRef;
      self.effectRef.as_deref().unwrap()
    }

    // Java: boolean hasEffectRef()
    pub fn hasEffectRef(&self) -> bool {
      // Java: return this.map.has(2) || this.effectRef != null && this.effectRef.dirty() || this.parent != null && this.parent.hasEffectRef();
      self.map.has(2) || self.effectRef.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasEffectRef())
    }

    // Java: void deleteEffectRef()
    pub fn deleteEffectRef(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.effectRef != null)
        if self.effectRef.is_some()
      {
         // Java: this.effectRef = null;
         self.effectRef = None;
      }
    }

    // Java: void writeEffectRef(StyleMatrixReference value)
    pub fn writeEffectRef(&mut self, value: Option<&StyleMatrixReference>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(2, bin);
            self.map.put(2, bin);
            // Java: if (this.effectRef != null)
                if self.effectRef.is_some()
            {
               // Java: this.effectRef = null;
               self.effectRef = None;
            }
         }
      }
    }

    // Java: void forceWriteEffectRef(StyleMatrixReference value)
    pub fn forceWriteEffectRef(&mut self, value: Option<&StyleMatrixReference>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.effectRef != null)
            if self.effectRef.is_some()
         {
            // Java: this.effectRef = null;
            self.effectRef = None;
         }
      }
    }

}

impl KoAr for ThemeableEffectStyle {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { ThemeableEffectStyle::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { ThemeableEffectStyle::lastBin(self, var1) }
}

impl KoClosable for ThemeableEffectStyle {
    fn close(&mut self) { ThemeableEffectStyle::close(self); }
}
