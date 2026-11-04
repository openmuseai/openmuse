#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.effect.EffectProperties
use crate::cn::wps::drawing::data::effect::EffectProperties::EffectProperties;
// Java: import cn.wps.drawing.data.threeD.Apply3DShapeProperties
use crate::cn::wps::drawing::data::threeD::Apply3DShapeProperties::Apply3DShapeProperties;
// Java: import cn.wps.drawing.data.threeD.Scene3DProperties
use crate::cn::wps::drawing::data::threeD::Scene3DProperties::Scene3DProperties;
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

// Java: public class EffectStyle implements KoAr, KoClosable
pub struct EffectStyle {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: EffectProperties effect
    pub(crate) effect: Option<Box<EffectProperties>>,
    // Java field: Scene3DProperties scene3D
    pub(crate) scene3D: Option<Box<Scene3DProperties>>,
    // Java field: Apply3DShapeProperties shape3D
    pub(crate) shape3D: Option<Box<Apply3DShapeProperties>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: EffectStyle parent
    pub(crate) parent: Option<Box<EffectStyle>>,
}

// Java: public class EffectStyle implements KoAr, KoClosable
impl EffectStyle {
    // Java: private static final int $effect = 1
    const FIELD_EFFECT: i32 = 1;
    // Java: private static final int $scene3D = 2
    const FIELD_SCENE3D: i32 = 2;
    // Java: private static final int $shape3D = 3
    const FIELD_SHAPE3D: i32 = 3;

    // Java: static protected static <Value1 extends EffectStyle> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      // Java: inst.zone = zone;
      // Java: inst.load(bin);
      let mut inst = Self::new_from(bin);
      inst.zone = zone;
      // Java: return inst;
      inst
    }

    // Java: static EffectStyle create()
    pub fn create() -> Self {
      // Java: return initialize(EffectStyle.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static EffectStyle open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(EffectStyle.class, bin);
      Self::initialize(bin)
    }

    // Java: EffectStyle()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Self {
        zone: KoZone::defaultZone(),
        effect: None,
        scene3D: None,
        shape3D: None,
        bin_box: KpBinBox::new(),
        map: KpMap::new(),
        parent: None,
      }
    }

    // Java: EffectStyle(KoBin bin)
    pub fn new_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Self {
        zone: KoZone::defaultZone(),
        effect: None,
        scene3D: None,
        shape3D: None,
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        map: KpMap::KpMap_from(bin),
        parent: None,
      }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
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
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone;
      self.loadBin(bin, zone);
    }

    // Java: void load(EffectStyle value)
    pub fn load_from(&mut self, value: Option<&EffectStyle>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         self.loadBin(bin.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: if (this.effect != null && this.effect.dirty())
        if self.effect.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(1, this.effect.saveBin(zone));
         let bin = self.effect.as_mut().unwrap().saveBin(zone);
         self.map.put(1, bin);
      }

      // Java: if (this.scene3D != null && this.scene3D.dirty())
        if self.scene3D.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(2, this.scene3D.saveBin(zone));
         let bin = self.scene3D.as_mut().unwrap().saveBin(zone);
         self.map.put(2, bin);
      }

      // Java: if (this.shape3D != null && this.shape3D.dirty())
        if self.shape3D.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(3, this.shape3D.saveBin(zone));
         let bin = self.shape3D.as_mut().unwrap().saveBin(zone);
         self.map.put(3, bin);
      }

      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(EffectStyle o)
    pub fn equalsLax(&self, o: &EffectStyle) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &EffectStyle) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &EffectStyle, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof EffectStyle)
      {
         // Java: EffectStyle dst = (EffectStyle)o;
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
                  if isStrict
               {
                  // Java: return false;
                  todo!("effect equals");
               }
               // Java: if (!isStrict && !this.effect().equalsLax(dst.effect()))
                  if !isStrict
               {
                  // Java: return false;
                  todo!("effect equalsLax");
               }
            }
            // Java: if ((this.hasScene3D() || !dst.hasScene3D()) && (!this.hasScene3D() || dst.hasScene3D()))
               if (self.hasScene3D() || !dst.hasScene3D()) && (!self.hasScene3D() || dst.hasScene3D())
            {
               // Java: if (this.hasScene3D() && dst.hasScene3D())
                  if self.hasScene3D() && dst.hasScene3D()
               {
                  // Java: if (isStrict && !this.scene3D().equals(dst.scene3D()))
                     if isStrict
                  {
                     // Java: return false;
                     todo!("scene3D equals");
                  }
                  // Java: if (!isStrict && !this.scene3D().equalsLax(dst.scene3D()))
                     if !isStrict
                  {
                     // Java: return false;
                     todo!("scene3D equalsLax");
                  }
               }
               // Java: if ((this.hasShape3D() || !dst.hasShape3D()) && (!this.hasShape3D() || dst.hasShape3D()))
                  if (self.hasShape3D() || !dst.hasShape3D()) && (!self.hasShape3D() || dst.hasShape3D())
               {
                  // Java: if (this.hasShape3D() && dst.hasShape3D())
                     if self.hasShape3D() && dst.hasShape3D()
                  {
                     // Java: if (isStrict && !this.shape3D().equals(dst.shape3D()))
                        if isStrict
                     {
                        // Java: return false;
                        todo!("shape3D equals");
                     }
                     // Java: if (!isStrict && !this.shape3D().equalsLax(dst.shape3D()))
                        if !isStrict
                     {
                        // Java: return false;
                        todo!("shape3D equalsLax");
                     }
                  }
                  // Java: return true;
                  return true;
               }
               // Java: else
               else {
                  // Java: return false;
                  return false;
               }
            }
            // Java: else
            else {
               // Java: return false;
               return false;
            }
         }
         // Java: else
         else {
            // Java: return false;
            return false;
         }
      }
      // Java: else
      // Java: return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&mut self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasEffect())
      if self.hasEffect() {
         // Java: ret += this.effect().hashCode();
         ret = ret.wrapping_add(self.effect().hashCode());
      }
      // Java: if (this.hasScene3D())
      if self.hasScene3D() {
         // Java: ret += this.scene3D().hashCode();
         ret = ret.wrapping_add(self.scene3D().hashCode());
      }
      // Java: if (this.hasShape3D())
      if self.hasShape3D() {
         // Java: ret += this.shape3D().hashCode();
         ret = ret.wrapping_add(self.shape3D().hashCode());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.effect != null && this.effect.dirty())
        if self.effect.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.scene3D != null && this.scene3D.dirty())
        if self.scene3D.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
      // Java: return this.shape3D != null && this.shape3D.dirty() ? true : this.map.dirty();
      self.shape3D.as_ref().map_or(false, |v| v.dirty()) || self.map.dirty()
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
      self.loadBin(bin.as_ref(), zone);
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
      let null_bin = Null::empty();
      self.map.load(null_bin.as_ref());
      // Java: this.box.clear();
      self.bin_box.set(None);
      // Java: this.zone.leave();
      // Java: this.zone = KoZone.defaultZone();
      self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    fn cleanCache(&mut self) {
      // Java: this.effect = null;
      self.effect = None;
      // Java: this.scene3D = null;
      self.scene3D = None;
      // Java: this.shape3D = null;
      self.shape3D = None;
    }

    // Java: void follow(EffectStyle parent)
    pub fn follow(&mut self, parent: Option<Box<EffectStyle>>) {
      // Java: if (parent != null)
      if let Some(mut p) = parent {
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
      if self.parent.is_some() {
         // Java: this.discard();
         self.discard();
         // Java: this.parent.discard();
         if let Some(ref mut p) = self.parent {
           p.discard();
         }
         // Java: this.cleanParent();
         self.cleanParent();
      }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
      // Java: return this.parent != null;
      self.parent.is_some()
    }

    // Java: EffectStyle parent()
    pub fn parent(&self) -> &EffectStyle {
      // Java: return this.parent;
      self.parent.as_deref().unwrap()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      let zone = self.zone;
      self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: EffectProperties effect()
    pub fn effect(&mut self) -> &EffectProperties {
      // Java: if (this.effect == null)
        if self.effect.is_none()
      {
         // Java: if (this.map.has(1))
            if self.map.has(1)
         {
            // Java: KoBin bin = this.map.byKey(1);
            // Java: this.effect = (EffectProperties)KoBin.Decode.toAny(EffectProperties.class, bin, this.zone);
            let bin = self.map.byKey(1);
            self.effect = Some(Box::new(EffectProperties::EffectProperties_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasEffect())
            else if self.parent.as_ref().map_or(false, |p| p.hasEffect())
         {
            // Java: this.effect = this.parent.effect();
            todo!();
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.effect = (EffectProperties)KoBin.Decode.toAny(EffectProperties.class, bin, this.zone);
            let null_bin = Null::empty();
            self.effect = Some(Box::new(EffectProperties::EffectProperties_from(null_bin.as_ref())));
         }
      }
      // Java: return this.effect;
      self.effect.as_deref().unwrap()
    }

    // Java: boolean hasEffect()
    pub fn hasEffect(&self) -> bool {
      // Java: return this.map.has(1) || this.effect != null && this.effect.dirty() || this.parent != null && this.parent.hasEffect();
      self.map.has(1)
        || self.effect.as_ref().map_or(false, |v| v.dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasEffect())
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

    // Java: Scene3DProperties scene3D()
    pub fn scene3D(&mut self) -> &Scene3DProperties {
      // Java: if (this.scene3D == null)
        if self.scene3D.is_none()
      {
         // Java: if (this.map.has(2))
            if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            // Java: this.scene3D = (Scene3DProperties)KoBin.Decode.toAny(Scene3DProperties.class, bin, this.zone);
            let bin = self.map.byKey(2);
            self.scene3D = Some(Box::new(Scene3DProperties::Scene3DProperties_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasScene3D())
            else if self.parent.as_ref().map_or(false, |p| p.hasScene3D())
         {
            // Java: this.scene3D = this.parent.scene3D();
            todo!();
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.scene3D = (Scene3DProperties)KoBin.Decode.toAny(Scene3DProperties.class, bin, this.zone);
            let null_bin = Null::empty();
            self.scene3D = Some(Box::new(Scene3DProperties::Scene3DProperties_from(null_bin.as_ref())));
         }
      }
      // Java: return this.scene3D;
      self.scene3D.as_deref().unwrap()
    }

    // Java: boolean hasScene3D()
    pub fn hasScene3D(&self) -> bool {
      // Java: return this.map.has(2) || this.scene3D != null && this.scene3D.dirty() || this.parent != null && this.parent.hasScene3D();
      self.map.has(2)
        || self.scene3D.as_ref().map_or(false, |v| v.dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasScene3D())
    }

    // Java: void deleteScene3D()
    pub fn deleteScene3D(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.scene3D != null)
        if self.scene3D.is_some()
      {
         // Java: this.scene3D = null;
         self.scene3D = None;
      }
    }

    // Java: void writeScene3D(Scene3DProperties value)
    pub fn writeScene3D(&mut self, value: Option<&Scene3DProperties>) {
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
            // Java: if (this.scene3D != null)
               if self.scene3D.is_some()
            {
               // Java: this.scene3D = null;
               self.scene3D = None;
            }
         }
      }
    }

    // Java: void forceWriteScene3D(Scene3DProperties value)
    pub fn forceWriteScene3D(&mut self, value: Option<&Scene3DProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.scene3D != null)
            if self.scene3D.is_some()
         {
            // Java: this.scene3D = null;
            self.scene3D = None;
         }
      }
    }

    // Java: Apply3DShapeProperties shape3D()
    pub fn shape3D(&mut self) -> &Apply3DShapeProperties {
      // Java: if (this.shape3D == null)
        if self.shape3D.is_none()
      {
         // Java: if (this.map.has(3))
            if self.map.has(3)
         {
            // Java: KoBin bin = this.map.byKey(3);
            // Java: this.shape3D = (Apply3DShapeProperties)KoBin.Decode.toAny(Apply3DShapeProperties.class, bin, this.zone);
            let bin = self.map.byKey(3);
            self.shape3D = Some(Box::new(Apply3DShapeProperties::Apply3DShapeProperties_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasShape3D())
            else if self.parent.as_ref().map_or(false, |p| p.hasShape3D())
         {
            // Java: this.shape3D = this.parent.shape3D();
            todo!();
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.shape3D = (Apply3DShapeProperties)KoBin.Decode.toAny(Apply3DShapeProperties.class, bin, this.zone);
            let null_bin = Null::empty();
            self.shape3D = Some(Box::new(Apply3DShapeProperties::Apply3DShapeProperties_from(null_bin.as_ref())));
         }
      }
      // Java: return this.shape3D;
      self.shape3D.as_deref().unwrap()
    }

    // Java: boolean hasShape3D()
    pub fn hasShape3D(&self) -> bool {
      // Java: return this.map.has(3) || this.shape3D != null && this.shape3D.dirty() || this.parent != null && this.parent.hasShape3D();
      self.map.has(3)
        || self.shape3D.as_ref().map_or(false, |v| v.dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasShape3D())
    }

    // Java: void deleteShape3D()
    pub fn deleteShape3D(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
      // Java: if (this.shape3D != null)
        if self.shape3D.is_some()
      {
         // Java: this.shape3D = null;
         self.shape3D = None;
      }
    }

    // Java: void writeShape3D(Apply3DShapeProperties value)
    pub fn writeShape3D(&mut self, value: Option<&Apply3DShapeProperties>) {
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
            // Java: if (this.shape3D != null)
               if self.shape3D.is_some()
            {
               // Java: this.shape3D = null;
               self.shape3D = None;
            }
         }
      }
    }

    // Java: void forceWriteShape3D(Apply3DShapeProperties value)
    pub fn forceWriteShape3D(&mut self, value: Option<&Apply3DShapeProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(3, value.lastBin(this.zone));
         self.map.put(3, v.lastBin(self.zone));
         // Java: if (this.shape3D != null)
            if self.shape3D.is_some()
         {
            // Java: this.shape3D = null;
            self.shape3D = None;
         }
      }
    }

}

impl KoAr for EffectStyle {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for EffectStyle { fn close(&mut self) { self.close(); } }

