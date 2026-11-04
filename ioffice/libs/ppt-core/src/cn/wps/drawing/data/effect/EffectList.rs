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

// Java: public class EffectList implements KoAr, KoClosable
pub struct EffectList {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: BlurEffect blur
    pub(crate) blur: Option<Box<dyn KoAr>>,
    // Java field: FillOverlayEffect fillOverlay
    pub(crate) fillOverlay: Option<Box<dyn KoAr>>,
    // Java field: GlowEffect glow
    pub(crate) glow: Option<Box<dyn KoAr>>,
    // Java field: InnerShadowEffect innerShadow
    pub(crate) innerShadow: Option<Box<dyn KoAr>>,
    // Java field: OuterShadowEffect outerShadow
    pub(crate) outerShadow: Option<Box<dyn KoAr>>,
    // Java field: ReflectionEffect reflection
    pub(crate) reflection: Option<Box<dyn KoAr>>,
    // Java field: SoftEdgeEffect softEdge
    pub(crate) softEdge: Option<Box<dyn KoAr>>,
    // Java field: PresetShadowEffect presetShadowEffect
    pub(crate) presetShadowEffect: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: EffectList parent
    pub(crate) parent: Option<Box<EffectList>>,
}

// Java: public class EffectList implements KoAr, KoClosable
impl EffectList {
    // Java: static protected static <Value1 extends EffectList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::EffectList_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static EffectList create()
    pub fn create() -> Self {
      // Java: return initialize(EffectList.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static EffectList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(EffectList.class, bin);
      Self::initialize(bin)
    }

    // Java: EffectList()
    pub fn new() -> Self {
      Self {
      // Java: this.box = new KpBinBox();
      bin_box: KpBinBox::new(),
      // Java: this.map = new KpMap();
      map: KpMap::new(),
      zone: KoZone::defaultZone(), blur: None, fillOverlay: None, glow: None, innerShadow: None, outerShadow: None, reflection: None, softEdge: None, presetShadowEffect: None, parent: None,
      }
    }

    // Java: EffectList(KoBin bin)
    pub fn EffectList_from(bin: &dyn KoBinTrait) -> Self {
      Self {
      // Java: this.box = new KpBinBox(bin);
      bin_box: KpBinBox::KpBinBox_from(Some(bin)),
      // Java: this.map = new KpMap(bin);
      map: KpMap::KpMap_from(bin),
      zone: KoZone::defaultZone(), blur: None, fillOverlay: None, glow: None, innerShadow: None, outerShadow: None, reflection: None, softEdge: None, presetShadowEffect: None, parent: None,
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

    // Java: void load(EffectList value)
    pub fn load_value(&mut self, value: Option<&EffectList>) {
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
      // Java: if (this.blur != null && this.blur.dirty())
        if self.blur.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(1, this.blur.saveBin(zone));
         let saved = self.blur.as_mut().unwrap().saveBin(zone);
         self.map.put(1, saved);
      }
      // Java: if (this.fillOverlay != null && this.fillOverlay.dirty())
        if self.fillOverlay.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(2, this.fillOverlay.saveBin(zone));
         let saved = self.fillOverlay.as_mut().unwrap().saveBin(zone);
         self.map.put(2, saved);
      }
      // Java: if (this.glow != null && this.glow.dirty())
        if self.glow.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(3, this.glow.saveBin(zone));
         let saved = self.glow.as_mut().unwrap().saveBin(zone);
         self.map.put(3, saved);
      }
      // Java: if (this.innerShadow != null && this.innerShadow.dirty())
        if self.innerShadow.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(4, this.innerShadow.saveBin(zone));
         let saved = self.innerShadow.as_mut().unwrap().saveBin(zone);
         self.map.put(4, saved);
      }
      // Java: if (this.outerShadow != null && this.outerShadow.dirty())
        if self.outerShadow.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(5, this.outerShadow.saveBin(zone));
         let saved = self.outerShadow.as_mut().unwrap().saveBin(zone);
         self.map.put(5, saved);
      }
      // Java: if (this.reflection != null && this.reflection.dirty())
        if self.reflection.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(6, this.reflection.saveBin(zone));
         let saved = self.reflection.as_mut().unwrap().saveBin(zone);
         self.map.put(6, saved);
      }
      // Java: if (this.softEdge != null && this.softEdge.dirty())
        if self.softEdge.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(7, this.softEdge.saveBin(zone));
         let saved = self.softEdge.as_mut().unwrap().saveBin(zone);
         self.map.put(7, saved);
      }
      // Java: if (this.presetShadowEffect != null && this.presetShadowEffect.dirty())
        if self.presetShadowEffect.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(8, this.presetShadowEffect.saveBin(zone));
         let saved = self.presetShadowEffect.as_mut().unwrap().saveBin(zone);
         self.map.put(8, saved);
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

    // Java: boolean equalsLax(EffectList o)
    pub fn equalsLax(&self, o: &EffectList) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &EffectList) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &EffectList, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof EffectList)
        if true
      {
          let dst = o;
         // Java: EffectList dst = (EffectList)o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasBlur() || !dst.hasBlur()) && (!this.hasBlur() || dst.hasBlur()))
            else if (self.hasBlur() || !dst.hasBlur()) && (!self.hasBlur() || dst.hasBlur())
         {
            // Java: if (this.hasBlur() && dst.hasBlur())
                if self.hasBlur() && dst.hasBlur()
            {
               // Java: if (isStrict && !this.blur().equals(dst.blur()))
                    if todo!("KoAr equals")
               {
                  // Java: return false;
                  return false;
               }
               // Java: if (!isStrict && !this.blur().equalsLax(dst.blur()))
                    if todo!("KoAr equalsLax")
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: if ((this.hasFillOverlay() || !dst.hasFillOverlay()) && (!this.hasFillOverlay() || dst.hasFillOverlay()))
                if (self.hasFillOverlay() || !dst.hasFillOverlay()) && (!self.hasFillOverlay() || dst.hasFillOverlay())
            {
               // Java: if (this.hasFillOverlay() && dst.hasFillOverlay())
                    if self.hasFillOverlay() && dst.hasFillOverlay()
               {
                  // Java: if (isStrict && !this.fillOverlay().equals(dst.fillOverlay()))
                        if todo!("KoAr equals")
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !this.fillOverlay().equalsLax(dst.fillOverlay()))
                        if todo!("KoAr equalsLax")
                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: if ((this.hasGlow() || !dst.hasGlow()) && (!this.hasGlow() || dst.hasGlow()))
                    if (self.hasGlow() || !dst.hasGlow()) && (!self.hasGlow() || dst.hasGlow())
               {
                  // Java: if (this.hasGlow() && dst.hasGlow())
                        if self.hasGlow() && dst.hasGlow()
                  {
                     // Java: if (isStrict && !this.glow().equals(dst.glow()))
                            if todo!("KoAr equals")
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !this.glow().equalsLax(dst.glow()))
                            if todo!("KoAr equalsLax")
                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: if ((this.hasInnerShadow() || !dst.hasInnerShadow()) && (!this.hasInnerShadow() || dst.hasInnerShadow()))
                        if (self.hasInnerShadow() || !dst.hasInnerShadow()) && (!self.hasInnerShadow() || dst.hasInnerShadow())
                  {
                     // Java: if (this.hasInnerShadow() && dst.hasInnerShadow())
                            if self.hasInnerShadow() && dst.hasInnerShadow()
                     {
                        // Java: if (isStrict && !this.innerShadow().equals(dst.innerShadow()))
                                if todo!("KoAr equals")
                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: if (!isStrict && !this.innerShadow().equalsLax(dst.innerShadow()))
                                if todo!("KoAr equalsLax")
                        {
                           // Java: return false;
                           return false;
                        }
                     }
                     // Java: if ((this.hasOuterShadow() || !dst.hasOuterShadow()) && (!this.hasOuterShadow() || dst.hasOuterShadow()))
                            if (self.hasOuterShadow() || !dst.hasOuterShadow()) && (!self.hasOuterShadow() || dst.hasOuterShadow())
                     {
                        // Java: if (this.hasOuterShadow() && dst.hasOuterShadow())
                                if self.hasOuterShadow() && dst.hasOuterShadow()
                        {
                           // Java: if (isStrict && !this.outerShadow().equals(dst.outerShadow()))
                                    if todo!("KoAr equals")
                           {
                              // Java: return false;
                              return false;
                           }
                           // Java: if (!isStrict && !this.outerShadow().equalsLax(dst.outerShadow()))
                                    if todo!("KoAr equalsLax")
                           {
                              // Java: return false;
                              return false;
                           }
                        }
                        // Java: if ((this.hasReflection() || !dst.hasReflection()) && (!this.hasReflection() || dst.hasReflection()))
                                if (self.hasReflection() || !dst.hasReflection()) && (!self.hasReflection() || dst.hasReflection())
                        {
                           // Java: if (this.hasReflection() && dst.hasReflection())
                                    if self.hasReflection() && dst.hasReflection()
                           {
                              // Java: if (isStrict && !this.reflection().equals(dst.reflection()))
                                        if todo!("KoAr equals")
                              {
                                 // Java: return false;
                                 return false;
                              }
                              // Java: if (!isStrict && !this.reflection().equalsLax(dst.reflection()))
                                        if todo!("KoAr equalsLax")
                              {
                                 // Java: return false;
                                 return false;
                              }
                           }
                           // Java: if ((this.hasSoftEdge() || !dst.hasSoftEdge()) && (!this.hasSoftEdge() || dst.hasSoftEdge()))
                                    if (self.hasSoftEdge() || !dst.hasSoftEdge()) && (!self.hasSoftEdge() || dst.hasSoftEdge())
                           {
                              // Java: if (this.hasSoftEdge() && dst.hasSoftEdge())
                                        if self.hasSoftEdge() && dst.hasSoftEdge()
                              {
                                 // Java: if (isStrict && !this.softEdge().equals(dst.softEdge()))
                                            if todo!("KoAr equals")
                                 {
                                    // Java: return false;
                                    return false;
                                 }
                                 // Java: if (!isStrict && !this.softEdge().equalsLax(dst.softEdge()))
                                            if todo!("KoAr equalsLax")
                                 {
                                    // Java: return false;
                                    return false;
                                 }
                              }
                              // Java: if ((this.hasPresetShadowEffect() || !dst.hasPresetShadowEffect()) && (!this.hasPresetShadowEffect() || dst.hasPresetShadowEffect()))
                                        if (self.hasPresetShadowEffect() || !dst.hasPresetShadowEffect()) && (!self.hasPresetShadowEffect() || dst.hasPresetShadowEffect())
                              {
                                 // Java: if (this.hasPresetShadowEffect() && dst.hasPresetShadowEffect())
                                            if self.hasPresetShadowEffect() && dst.hasPresetShadowEffect()
                                 {
                                    // Java: if (isStrict && !this.presetShadowEffect().equals(dst.presetShadowEffect()))
                                                if todo!("KoAr equals")
                                    {
                                       // Java: return false;
                                       return false;
                                    }
                                    // Java: if (!isStrict && !this.presetShadowEffect().equalsLax(dst.presetShadowEffect()))
                                                if todo!("KoAr equalsLax")
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
      // Java: if (this.hasBlur())
        if self.hasBlur()
      {
         // Java: ret += this.blur().hashCode();
         ret = ret.wrapping_add(todo!("blur hashCode"));
      }
      // Java: if (this.hasFillOverlay())
        if self.hasFillOverlay()
      {
         // Java: ret += this.fillOverlay().hashCode();
         ret = ret.wrapping_add(todo!("fillOverlay hashCode"));
      }
      // Java: if (this.hasGlow())
        if self.hasGlow()
      {
         // Java: ret += this.glow().hashCode();
         ret = ret.wrapping_add(todo!("glow hashCode"));
      }
      // Java: if (this.hasInnerShadow())
        if self.hasInnerShadow()
      {
         // Java: ret += this.innerShadow().hashCode();
         ret = ret.wrapping_add(todo!("innerShadow hashCode"));
      }
      // Java: if (this.hasOuterShadow())
        if self.hasOuterShadow()
      {
         // Java: ret += this.outerShadow().hashCode();
         ret = ret.wrapping_add(todo!("outerShadow hashCode"));
      }
      // Java: if (this.hasReflection())
        if self.hasReflection()
      {
         // Java: ret += this.reflection().hashCode();
         ret = ret.wrapping_add(todo!("reflection hashCode"));
      }
      // Java: if (this.hasSoftEdge())
        if self.hasSoftEdge()
      {
         // Java: ret += this.softEdge().hashCode();
         ret = ret.wrapping_add(todo!("softEdge hashCode"));
      }
      // Java: if (this.hasPresetShadowEffect())
        if self.hasPresetShadowEffect()
      {
         // Java: ret += this.presetShadowEffect().hashCode();
         ret = ret.wrapping_add(todo!("presetShadowEffect hashCode"));
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.blur != null && this.blur.dirty())
        if self.blur.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.fillOverlay != null && this.fillOverlay.dirty())
        else if self.fillOverlay.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.glow != null && this.glow.dirty())
        else if self.glow.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.innerShadow != null && this.innerShadow.dirty())
        else if self.innerShadow.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.outerShadow != null && this.outerShadow.dirty())
        else if self.outerShadow.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.reflection != null && this.reflection.dirty())
        else if self.reflection.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.softEdge != null && this.softEdge.dirty())
        else if self.softEdge.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
        else
      {
         // Java: return this.presetShadowEffect != null && this.presetShadowEffect.dirty() ? true : this.map.dirty();
         return self.presetShadowEffect.as_ref().map_or(false, |x| x.dirty()) || self.map.dirty();
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
      // Java: this.blur = null;
      self.blur = None;
      // Java: this.fillOverlay = null;
      self.fillOverlay = None;
      // Java: this.glow = null;
      self.glow = None;
      // Java: this.innerShadow = null;
      self.innerShadow = None;
      // Java: this.outerShadow = null;
      self.outerShadow = None;
      // Java: this.reflection = null;
      self.reflection = None;
      // Java: this.softEdge = null;
      self.softEdge = None;
      // Java: this.presetShadowEffect = null;
      self.presetShadowEffect = None;
    }

    // Java: void follow(EffectList parent)
    pub fn follow(&mut self, parent: Option<Box<EffectList>>) {
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

    // Java: EffectList parent()
    pub fn get_parent(&self) -> Option<&EffectList> {
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

    // Java: BlurEffect blur()
    pub fn get_blur(&self) -> &dyn KoAr {
      // Java: if (this.blur == null)
        if self.blur.is_none()
      {
         // Java: if (this.map.has(1))
            if self.map.has(1)
         {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: this.blur = (BlurEffect)KoBin.Decode.toAny(BlurEffect.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
         // Java: else if (this.parent != null && this.parent.hasBlur())
            else if self.parent.as_ref().map_or(false, |p| p.hasBlur())
         {
            // Java: this.blur = this.parent.blur();
            todo!("lazy init requires &mut self")
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.blur = (BlurEffect)KoBin.Decode.toAny(BlurEffect.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
      }
      // Java: return this.blur;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasBlur()
    pub fn hasBlur(&self) -> bool {
      // Java: return this.map.has(1) || this.blur != null && this.blur.dirty() || this.parent != null && this.parent.hasBlur();
      self.map.has(1) || self.blur.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasBlur())
    }

    // Java: void deleteBlur()
    pub fn deleteBlur(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
      // Java: if (this.blur != null)
        if self.blur.is_some()
      {
         // Java: this.blur = null;
         self.blur = None;
      }
    }

    // Java: void writeBlur(BlurEffect value)
    pub fn writeBlur(&mut self, value: Option<&dyn KoAr>) {
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
            // Java: if (this.blur != null)
                if self.blur.is_some()
            {
               // Java: this.blur = null;
               self.blur = None;
            }
         }
      }
    }

    // Java: void forceWriteBlur(BlurEffect value)
    pub fn forceWriteBlur(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(1, v.lastBin(self.zone));
         // Java: if (this.blur != null)
            if self.blur.is_some()
         {
            // Java: this.blur = null;
            self.blur = None;
         }
      }
    }

    // Java: FillOverlayEffect fillOverlay()
    pub fn get_fillOverlay(&self) -> &dyn KoAr {
      // Java: if (this.fillOverlay == null)
        if self.fillOverlay.is_none()
      {
         // Java: if (this.map.has(2))
            if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: this.fillOverlay = (FillOverlayEffect)KoBin.Decode.toAny(FillOverlayEffect.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
         // Java: else if (this.parent != null && this.parent.hasFillOverlay())
            else if self.parent.as_ref().map_or(false, |p| p.hasFillOverlay())
         {
            // Java: this.fillOverlay = this.parent.fillOverlay();
            todo!("lazy init requires &mut self")
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.fillOverlay = (FillOverlayEffect)KoBin.Decode.toAny(FillOverlayEffect.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
      }
      // Java: return this.fillOverlay;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasFillOverlay()
    pub fn hasFillOverlay(&self) -> bool {
      // Java: return this.map.has(2) || this.fillOverlay != null && this.fillOverlay.dirty() || this.parent != null && this.parent.hasFillOverlay();
      self.map.has(2) || self.fillOverlay.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasFillOverlay())
    }

    // Java: void deleteFillOverlay()
    pub fn deleteFillOverlay(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.fillOverlay != null)
        if self.fillOverlay.is_some()
      {
         // Java: this.fillOverlay = null;
         self.fillOverlay = None;
      }
    }

    // Java: void writeFillOverlay(FillOverlayEffect value)
    pub fn writeFillOverlay(&mut self, value: Option<&dyn KoAr>) {
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
            // Java: if (this.fillOverlay != null)
                if self.fillOverlay.is_some()
            {
               // Java: this.fillOverlay = null;
               self.fillOverlay = None;
            }
         }
      }
    }

    // Java: void forceWriteFillOverlay(FillOverlayEffect value)
    pub fn forceWriteFillOverlay(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.fillOverlay != null)
            if self.fillOverlay.is_some()
         {
            // Java: this.fillOverlay = null;
            self.fillOverlay = None;
         }
      }
    }

    // Java: GlowEffect glow()
    pub fn get_glow(&self) -> &dyn KoAr {
      // Java: if (this.glow == null)
        if self.glow.is_none()
      {
         // Java: if (this.map.has(3))
            if self.map.has(3)
         {
            // Java: KoBin bin = this.map.byKey(3);
            let bin = self.map.byKey(3);
            // Java: this.glow = (GlowEffect)KoBin.Decode.toAny(GlowEffect.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
         // Java: else if (this.parent != null && this.parent.hasGlow())
            else if self.parent.as_ref().map_or(false, |p| p.hasGlow())
         {
            // Java: this.glow = this.parent.glow();
            todo!("lazy init requires &mut self")
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.glow = (GlowEffect)KoBin.Decode.toAny(GlowEffect.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
      }
      // Java: return this.glow;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasGlow()
    pub fn hasGlow(&self) -> bool {
      // Java: return this.map.has(3) || this.glow != null && this.glow.dirty() || this.parent != null && this.parent.hasGlow();
      self.map.has(3) || self.glow.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasGlow())
    }

    // Java: void deleteGlow()
    pub fn deleteGlow(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
      // Java: if (this.glow != null)
        if self.glow.is_some()
      {
         // Java: this.glow = null;
         self.glow = None;
      }
    }

    // Java: void writeGlow(GlowEffect value)
    pub fn writeGlow(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(3, bin);
            self.map.put(3, bin);
            // Java: if (this.glow != null)
                if self.glow.is_some()
            {
               // Java: this.glow = null;
               self.glow = None;
            }
         }
      }
    }

    // Java: void forceWriteGlow(GlowEffect value)
    pub fn forceWriteGlow(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(3, value.lastBin(this.zone));
         self.map.put(3, v.lastBin(self.zone));
         // Java: if (this.glow != null)
            if self.glow.is_some()
         {
            // Java: this.glow = null;
            self.glow = None;
         }
      }
    }

    // Java: InnerShadowEffect innerShadow()
    pub fn get_innerShadow(&self) -> &dyn KoAr {
      // Java: if (this.innerShadow == null)
        if self.innerShadow.is_none()
      {
         // Java: if (this.map.has(4))
            if self.map.has(4)
         {
            // Java: KoBin bin = this.map.byKey(4);
            let bin = self.map.byKey(4);
            // Java: this.innerShadow = (InnerShadowEffect)KoBin.Decode.toAny(InnerShadowEffect.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
         // Java: else if (this.parent != null && this.parent.hasInnerShadow())
            else if self.parent.as_ref().map_or(false, |p| p.hasInnerShadow())
         {
            // Java: this.innerShadow = this.parent.innerShadow();
            todo!("lazy init requires &mut self")
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.innerShadow = (InnerShadowEffect)KoBin.Decode.toAny(InnerShadowEffect.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
      }
      // Java: return this.innerShadow;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasInnerShadow()
    pub fn hasInnerShadow(&self) -> bool {
      // Java: return this.map.has(4) || this.innerShadow != null && this.innerShadow.dirty() || this.parent != null && this.parent.hasInnerShadow();
      self.map.has(4) || self.innerShadow.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasInnerShadow())
    }

    // Java: void deleteInnerShadow()
    pub fn deleteInnerShadow(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
      // Java: if (this.innerShadow != null)
        if self.innerShadow.is_some()
      {
         // Java: this.innerShadow = null;
         self.innerShadow = None;
      }
    }

    // Java: void writeInnerShadow(InnerShadowEffect value)
    pub fn writeInnerShadow(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(4, bin);
            self.map.put(4, bin);
            // Java: if (this.innerShadow != null)
                if self.innerShadow.is_some()
            {
               // Java: this.innerShadow = null;
               self.innerShadow = None;
            }
         }
      }
    }

    // Java: void forceWriteInnerShadow(InnerShadowEffect value)
    pub fn forceWriteInnerShadow(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(4, value.lastBin(this.zone));
         self.map.put(4, v.lastBin(self.zone));
         // Java: if (this.innerShadow != null)
            if self.innerShadow.is_some()
         {
            // Java: this.innerShadow = null;
            self.innerShadow = None;
         }
      }
    }

    // Java: OuterShadowEffect outerShadow()
    pub fn get_outerShadow(&self) -> &dyn KoAr {
      // Java: if (this.outerShadow == null)
        if self.outerShadow.is_none()
      {
         // Java: if (this.map.has(5))
            if self.map.has(5)
         {
            // Java: KoBin bin = this.map.byKey(5);
            let bin = self.map.byKey(5);
            // Java: this.outerShadow = (OuterShadowEffect)KoBin.Decode.toAny(OuterShadowEffect.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
         // Java: else if (this.parent != null && this.parent.hasOuterShadow())
            else if self.parent.as_ref().map_or(false, |p| p.hasOuterShadow())
         {
            // Java: this.outerShadow = this.parent.outerShadow();
            todo!("lazy init requires &mut self")
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.outerShadow = (OuterShadowEffect)KoBin.Decode.toAny(OuterShadowEffect.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
      }
      // Java: return this.outerShadow;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasOuterShadow()
    pub fn hasOuterShadow(&self) -> bool {
      // Java: return this.map.has(5) || this.outerShadow != null && this.outerShadow.dirty() || this.parent != null && this.parent.hasOuterShadow();
      self.map.has(5) || self.outerShadow.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasOuterShadow())
    }

    // Java: void deleteOuterShadow()
    pub fn deleteOuterShadow(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(5);
      // Java: if (this.outerShadow != null)
        if self.outerShadow.is_some()
      {
         // Java: this.outerShadow = null;
         self.outerShadow = None;
      }
    }

    // Java: void writeOuterShadow(OuterShadowEffect value)
    pub fn writeOuterShadow(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(5, bin);
            self.map.put(5, bin);
            // Java: if (this.outerShadow != null)
                if self.outerShadow.is_some()
            {
               // Java: this.outerShadow = null;
               self.outerShadow = None;
            }
         }
      }
    }

    // Java: void forceWriteOuterShadow(OuterShadowEffect value)
    pub fn forceWriteOuterShadow(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(5, value.lastBin(this.zone));
         self.map.put(5, v.lastBin(self.zone));
         // Java: if (this.outerShadow != null)
            if self.outerShadow.is_some()
         {
            // Java: this.outerShadow = null;
            self.outerShadow = None;
         }
      }
    }

    // Java: ReflectionEffect reflection()
    pub fn get_reflection(&self) -> &dyn KoAr {
      // Java: if (this.reflection == null)
        if self.reflection.is_none()
      {
         // Java: if (this.map.has(6))
            if self.map.has(6)
         {
            // Java: KoBin bin = this.map.byKey(6);
            let bin = self.map.byKey(6);
            // Java: this.reflection = (ReflectionEffect)KoBin.Decode.toAny(ReflectionEffect.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
         // Java: else if (this.parent != null && this.parent.hasReflection())
            else if self.parent.as_ref().map_or(false, |p| p.hasReflection())
         {
            // Java: this.reflection = this.parent.reflection();
            todo!("lazy init requires &mut self")
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.reflection = (ReflectionEffect)KoBin.Decode.toAny(ReflectionEffect.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
      }
      // Java: return this.reflection;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasReflection()
    pub fn hasReflection(&self) -> bool {
      // Java: return this.map.has(6) || this.reflection != null && this.reflection.dirty() || this.parent != null && this.parent.hasReflection();
      self.map.has(6) || self.reflection.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasReflection())
    }

    // Java: void deleteReflection()
    pub fn deleteReflection(&mut self) {
      // Java: this.map.put(6, (KoBin)null);
      self.map.clear_key(6);
      // Java: if (this.reflection != null)
        if self.reflection.is_some()
      {
         // Java: this.reflection = null;
         self.reflection = None;
      }
    }

    // Java: void writeReflection(ReflectionEffect value)
    pub fn writeReflection(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(6, bin);
            self.map.put(6, bin);
            // Java: if (this.reflection != null)
                if self.reflection.is_some()
            {
               // Java: this.reflection = null;
               self.reflection = None;
            }
         }
      }
    }

    // Java: void forceWriteReflection(ReflectionEffect value)
    pub fn forceWriteReflection(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(6, value.lastBin(this.zone));
         self.map.put(6, v.lastBin(self.zone));
         // Java: if (this.reflection != null)
            if self.reflection.is_some()
         {
            // Java: this.reflection = null;
            self.reflection = None;
         }
      }
    }

    // Java: SoftEdgeEffect softEdge()
    pub fn get_softEdge(&self) -> &dyn KoAr {
      // Java: if (this.softEdge == null)
        if self.softEdge.is_none()
      {
         // Java: if (this.map.has(7))
            if self.map.has(7)
         {
            // Java: KoBin bin = this.map.byKey(7);
            let bin = self.map.byKey(7);
            // Java: this.softEdge = (SoftEdgeEffect)KoBin.Decode.toAny(SoftEdgeEffect.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
         // Java: else if (this.parent != null && this.parent.hasSoftEdge())
            else if self.parent.as_ref().map_or(false, |p| p.hasSoftEdge())
         {
            // Java: this.softEdge = this.parent.softEdge();
            todo!("lazy init requires &mut self")
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.softEdge = (SoftEdgeEffect)KoBin.Decode.toAny(SoftEdgeEffect.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
      }
      // Java: return this.softEdge;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasSoftEdge()
    pub fn hasSoftEdge(&self) -> bool {
      // Java: return this.map.has(7) || this.softEdge != null && this.softEdge.dirty() || this.parent != null && this.parent.hasSoftEdge();
      self.map.has(7) || self.softEdge.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasSoftEdge())
    }

    // Java: void deleteSoftEdge()
    pub fn deleteSoftEdge(&mut self) {
      // Java: this.map.put(7, (KoBin)null);
      self.map.clear_key(7);
      // Java: if (this.softEdge != null)
        if self.softEdge.is_some()
      {
         // Java: this.softEdge = null;
         self.softEdge = None;
      }
    }

    // Java: void writeSoftEdge(SoftEdgeEffect value)
    pub fn writeSoftEdge(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(7, bin);
            self.map.put(7, bin);
            // Java: if (this.softEdge != null)
                if self.softEdge.is_some()
            {
               // Java: this.softEdge = null;
               self.softEdge = None;
            }
         }
      }
    }

    // Java: void forceWriteSoftEdge(SoftEdgeEffect value)
    pub fn forceWriteSoftEdge(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(7, value.lastBin(this.zone));
         self.map.put(7, v.lastBin(self.zone));
         // Java: if (this.softEdge != null)
            if self.softEdge.is_some()
         {
            // Java: this.softEdge = null;
            self.softEdge = None;
         }
      }
    }

    // Java: PresetShadowEffect presetShadowEffect()
    pub fn get_presetShadowEffect(&self) -> &dyn KoAr {
      // Java: if (this.presetShadowEffect == null)
        if self.presetShadowEffect.is_none()
      {
         // Java: if (this.map.has(8))
            if self.map.has(8)
         {
            // Java: KoBin bin = this.map.byKey(8);
            let bin = self.map.byKey(8);
            // Java: this.presetShadowEffect = (PresetShadowEffect)KoBin.Decode.toAny(PresetShadowEffect.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
         // Java: else if (this.parent != null && this.parent.hasPresetShadowEffect())
            else if self.parent.as_ref().map_or(false, |p| p.hasPresetShadowEffect())
         {
            // Java: this.presetShadowEffect = this.parent.presetShadowEffect();
            todo!("lazy init requires &mut self")
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.presetShadowEffect = (PresetShadowEffect)KoBin.Decode.toAny(PresetShadowEffect.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
      }
      // Java: return this.presetShadowEffect;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasPresetShadowEffect()
    pub fn hasPresetShadowEffect(&self) -> bool {
      // Java: return this.map.has(8) || this.presetShadowEffect != null && this.presetShadowEffect.dirty() || this.parent != null && this.parent.hasPresetShadowEffect();
      self.map.has(8) || self.presetShadowEffect.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasPresetShadowEffect())
    }

    // Java: void deletePresetShadowEffect()
    pub fn deletePresetShadowEffect(&mut self) {
      // Java: this.map.put(8, (KoBin)null);
      self.map.clear_key(8);
      // Java: if (this.presetShadowEffect != null)
        if self.presetShadowEffect.is_some()
      {
         // Java: this.presetShadowEffect = null;
         self.presetShadowEffect = None;
      }
    }

    // Java: void writePresetShadowEffect(PresetShadowEffect value)
    pub fn writePresetShadowEffect(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(8, bin);
            self.map.put(8, bin);
            // Java: if (this.presetShadowEffect != null)
                if self.presetShadowEffect.is_some()
            {
               // Java: this.presetShadowEffect = null;
               self.presetShadowEffect = None;
            }
         }
      }
    }

    // Java: void forceWritePresetShadowEffect(PresetShadowEffect value)
    pub fn forceWritePresetShadowEffect(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(8, value.lastBin(this.zone));
         self.map.put(8, v.lastBin(self.zone));
         // Java: if (this.presetShadowEffect != null)
            if self.presetShadowEffect.is_some()
         {
            // Java: this.presetShadowEffect = null;
            self.presetShadowEffect = None;
         }
      }
    }

}

impl KoAr for EffectList {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { EffectList::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { EffectList::lastBin(self, var1) }
}

impl KoClosable for EffectList {
    fn close(&mut self) { EffectList::close(self); }
}
