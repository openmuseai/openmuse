#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

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

// Java: public class ShapeStyle implements KoAr, KoClosable
pub struct ShapeStyle {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: StyleMatrixReference effectRef
    pub(crate) effectRef: Option<Box<StyleMatrixReference>>,
    // Java field: StyleMatrixReference fillRef
    pub(crate) fillRef: Option<Box<StyleMatrixReference>>,
    // Java field: StyleMatrixReference fontRef
    pub(crate) fontRef: Option<Box<StyleMatrixReference>>,
    // Java field: StyleMatrixReference lnRef
    pub(crate) lnRef: Option<Box<StyleMatrixReference>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: ShapeStyle parent
    pub(crate) parent: Option<Box<ShapeStyle>>,
}

// Java: public class ShapeStyle implements KoAr, KoClosable
impl ShapeStyle {
    // Java: private static final int $effectRef = 1
    const FIELD_EFFECTREF: i32 = 1;
    // Java: private static final int $fillRef = 2
    const FIELD_FILLREF: i32 = 2;
    // Java: private static final int $fontRef = 3
    const FIELD_FONTREF: i32 = 3;
    // Java: private static final int $lnRef = 4
    const FIELD_LNREF: i32 = 4;

    // Java: static protected static <Value1 extends ShapeStyle> Value1 initialize(Class<Value1> clazz, KoBin bin)
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

    // Java: static ShapeStyle create()
    pub fn create() -> Self {
      // Java: return initialize(ShapeStyle.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static ShapeStyle open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(ShapeStyle.class, bin);
      Self::initialize(bin)
    }

    // Java: ShapeStyle()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Self {
        zone: KoZone::defaultZone(),
        effectRef: None,
        fillRef: None,
        fontRef: None,
        lnRef: None,
        bin_box: KpBinBox::new(),
        map: KpMap::new(),
        parent: None,
      }
    }

    // Java: ShapeStyle(KoBin bin)
    pub fn new_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Self {
        zone: KoZone::defaultZone(),
        effectRef: None,
        fillRef: None,
        fontRef: None,
        lnRef: None,
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

    // Java: void load(ShapeStyle value)
    pub fn load_from(&mut self, value: Option<&ShapeStyle>) {
      // Java: if (value != null)
      if let Some(v) = value {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         self.loadBin(bin.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: if (this.effectRef != null && this.effectRef.dirty())
        if self.effectRef.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(1, this.effectRef.saveBin(zone));
         let bin = self.effectRef.as_mut().unwrap().saveBin(zone);
         self.map.put(1, bin);
      }

      // Java: if (this.fillRef != null && this.fillRef.dirty())
        if self.fillRef.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(2, this.fillRef.saveBin(zone));
         let bin = self.fillRef.as_mut().unwrap().saveBin(zone);
         self.map.put(2, bin);
      }

      // Java: if (this.fontRef != null && this.fontRef.dirty())
        if self.fontRef.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(3, this.fontRef.saveBin(zone));
         let bin = self.fontRef.as_mut().unwrap().saveBin(zone);
         self.map.put(3, bin);
      }

      // Java: if (this.lnRef != null && this.lnRef.dirty())
        if self.lnRef.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(4, this.lnRef.saveBin(zone));
         let bin = self.lnRef.as_mut().unwrap().saveBin(zone);
         self.map.put(4, bin);
      }

      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(ShapeStyle o)
    pub fn equalsLax(&self, o: &ShapeStyle) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &ShapeStyle) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &ShapeStyle, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof ShapeStyle)
      {
         // Java: ShapeStyle dst = (ShapeStyle)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasEffectRef() || !dst.hasEffectRef()) && (!this.hasEffectRef() || dst.hasEffectRef()))
            else if (self.hasEffectRef() || !dst.hasEffectRef()) && (!self.hasEffectRef() || dst.hasEffectRef())
         {
            // Java: if (this.hasEffectRef() && dst.hasEffectRef())
               if self.hasEffectRef() && dst.hasEffectRef()
            {
               // Java: if (isStrict && !this.effectRef().equals(dst.effectRef()))
                  if isStrict
               {
                  // Java: return false;
                  todo!("effectRef equals");
               }
               // Java: if (!isStrict && !this.effectRef().equalsLax(dst.effectRef()))
                  if !isStrict
               {
                  // Java: return false;
                  todo!("effectRef equalsLax");
               }
            }
            // Java: if ((this.hasFillRef() || !dst.hasFillRef()) && (!this.hasFillRef() || dst.hasFillRef()))
               if (self.hasFillRef() || !dst.hasFillRef()) && (!self.hasFillRef() || dst.hasFillRef())
            {
               // Java: if (this.hasFillRef() && dst.hasFillRef())
                  if self.hasFillRef() && dst.hasFillRef()
               {
                  // Java: if (isStrict && !this.fillRef().equals(dst.fillRef()))
                     if isStrict
                  {
                     // Java: return false;
                     todo!("fillRef equals");
                  }
                  // Java: if (!isStrict && !this.fillRef().equalsLax(dst.fillRef()))
                     if !isStrict
                  {
                     // Java: return false;
                     todo!("fillRef equalsLax");
                  }
               }
               // Java: if ((this.hasFontRef() || !dst.hasFontRef()) && (!this.hasFontRef() || dst.hasFontRef()))
                  if (self.hasFontRef() || !dst.hasFontRef()) && (!self.hasFontRef() || dst.hasFontRef())
               {
                  // Java: if (this.hasFontRef() && dst.hasFontRef())
                     if self.hasFontRef() && dst.hasFontRef()
                  {
                     // Java: if (isStrict && !this.fontRef().equals(dst.fontRef()))
                        if isStrict
                     {
                        // Java: return false;
                        todo!("fontRef equals");
                     }
                     // Java: if (!isStrict && !this.fontRef().equalsLax(dst.fontRef()))
                        if !isStrict
                     {
                        // Java: return false;
                        todo!("fontRef equalsLax");
                     }
                  }
                  // Java: if ((this.hasLnRef() || !dst.hasLnRef()) && (!this.hasLnRef() || dst.hasLnRef()))
                     if (self.hasLnRef() || !dst.hasLnRef()) && (!self.hasLnRef() || dst.hasLnRef())
                  {
                     // Java: if (this.hasLnRef() && dst.hasLnRef())
                        if self.hasLnRef() && dst.hasLnRef()
                     {
                        // Java: if (isStrict && !this.lnRef().equals(dst.lnRef()))
                           if isStrict
                        {
                           // Java: return false;
                           todo!("lnRef equals");
                        }
                        // Java: if (!isStrict && !this.lnRef().equalsLax(dst.lnRef()))
                           if !isStrict
                        {
                           // Java: return false;
                           todo!("lnRef equalsLax");
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
      // Java: if (this.hasEffectRef())
      if self.hasEffectRef() {
         // Java: ret += this.effectRef().hashCode();
         self.effectRef();
         ret = ret.wrapping_add(self.effectRef.as_mut().unwrap().hashCode());
      }
      // Java: if (this.hasFillRef())
      if self.hasFillRef() {
         // Java: ret += this.fillRef().hashCode();
         self.fillRef();
         ret = ret.wrapping_add(self.fillRef.as_mut().unwrap().hashCode());
      }
      // Java: if (this.hasFontRef())
      if self.hasFontRef() {
         // Java: ret += this.fontRef().hashCode();
         self.fontRef();
         ret = ret.wrapping_add(self.fontRef.as_mut().unwrap().hashCode());
      }
      // Java: if (this.hasLnRef())
      if self.hasLnRef() {
         // Java: ret += this.lnRef().hashCode();
         self.lnRef();
         ret = ret.wrapping_add(self.lnRef.as_mut().unwrap().hashCode());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.effectRef != null && this.effectRef.dirty())
        if self.effectRef.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.fillRef != null && this.fillRef.dirty())
        if self.fillRef.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.fontRef != null && this.fontRef.dirty())
        if self.fontRef.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
      // Java: return this.lnRef != null && this.lnRef.dirty() ? true : this.map.dirty();
      self.lnRef.as_ref().map_or(false, |v| v.dirty()) || self.map.dirty()
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
      // Java: this.effectRef = null;
      self.effectRef = None;
      // Java: this.fillRef = null;
      self.fillRef = None;
      // Java: this.fontRef = null;
      self.fontRef = None;
      // Java: this.lnRef = null;
      self.lnRef = None;
    }

    // Java: void follow(ShapeStyle parent)
    pub fn follow(&mut self, parent: Option<Box<ShapeStyle>>) {
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

    // Java: ShapeStyle parent()
    pub fn parent(&self) -> &ShapeStyle {
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

    // Java: StyleMatrixReference effectRef()
    pub fn effectRef(&mut self) -> &StyleMatrixReference {
      // Java: if (this.effectRef == null)
        if self.effectRef.is_none()
      {
         // Java: if (this.map.has(1))
            if self.map.has(1)
         {
            // Java: KoBin bin = this.map.byKey(1);
            // Java: this.effectRef = (StyleMatrixReference)KoBin.Decode.toAny(StyleMatrixReference.class, bin, this.zone);
            let bin = self.map.byKey(1);
            self.effectRef = Some(Box::new(StyleMatrixReference::new_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasEffectRef())
            else if self.parent.as_ref().map_or(false, |p| p.hasEffectRef())
         {
            // Java: this.effectRef = this.parent.effectRef();
            todo!();
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.effectRef = (StyleMatrixReference)KoBin.Decode.toAny(StyleMatrixReference.class, bin, this.zone);
            let null_bin = Null::empty();
            self.effectRef = Some(Box::new(StyleMatrixReference::new_from(null_bin.as_ref())));
         }
      }
      // Java: return this.effectRef;
      self.effectRef.as_deref().unwrap()
    }

    // Java: boolean hasEffectRef()
    pub fn hasEffectRef(&self) -> bool {
      // Java: return this.map.has(1) || this.effectRef != null && this.effectRef.dirty() || this.parent != null && this.parent.hasEffectRef();
      self.map.has(1)
        || self.effectRef.as_ref().map_or(false, |v| v.dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasEffectRef())
    }

    // Java: void deleteEffectRef()
    pub fn deleteEffectRef(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
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
            // Java: this.map.put(1, bin);
            self.map.put(1, bin);
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
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(1, v.lastBin(self.zone));
         // Java: if (this.effectRef != null)
            if self.effectRef.is_some()
         {
            // Java: this.effectRef = null;
            self.effectRef = None;
         }
      }
    }

    // Java: StyleMatrixReference fillRef()
    pub fn fillRef(&mut self) -> &StyleMatrixReference {
      // Java: if (this.fillRef == null)
        if self.fillRef.is_none()
      {
         // Java: if (this.map.has(2))
            if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            // Java: this.fillRef = (StyleMatrixReference)KoBin.Decode.toAny(StyleMatrixReference.class, bin, this.zone);
            let bin = self.map.byKey(2);
            self.fillRef = Some(Box::new(StyleMatrixReference::new_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasFillRef())
            else if self.parent.as_ref().map_or(false, |p| p.hasFillRef())
         {
            // Java: this.fillRef = this.parent.fillRef();
            todo!();
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.fillRef = (StyleMatrixReference)KoBin.Decode.toAny(StyleMatrixReference.class, bin, this.zone);
            let null_bin = Null::empty();
            self.fillRef = Some(Box::new(StyleMatrixReference::new_from(null_bin.as_ref())));
         }
      }
      // Java: return this.fillRef;
      self.fillRef.as_deref().unwrap()
    }

    // Java: boolean hasFillRef()
    pub fn hasFillRef(&self) -> bool {
      // Java: return this.map.has(2) || this.fillRef != null && this.fillRef.dirty() || this.parent != null && this.parent.hasFillRef();
      self.map.has(2)
        || self.fillRef.as_ref().map_or(false, |v| v.dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasFillRef())
    }

    // Java: void deleteFillRef()
    pub fn deleteFillRef(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.fillRef != null)
        if self.fillRef.is_some()
      {
         // Java: this.fillRef = null;
         self.fillRef = None;
      }
    }

    // Java: void writeFillRef(StyleMatrixReference value)
    pub fn writeFillRef(&mut self, value: Option<&StyleMatrixReference>) {
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
            // Java: if (this.fillRef != null)
               if self.fillRef.is_some()
            {
               // Java: this.fillRef = null;
               self.fillRef = None;
            }
         }
      }
    }

    // Java: void forceWriteFillRef(StyleMatrixReference value)
    pub fn forceWriteFillRef(&mut self, value: Option<&StyleMatrixReference>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.fillRef != null)
            if self.fillRef.is_some()
         {
            // Java: this.fillRef = null;
            self.fillRef = None;
         }
      }
    }

    // Java: StyleMatrixReference fontRef()
    pub fn fontRef(&mut self) -> &StyleMatrixReference {
      // Java: if (this.fontRef == null)
        if self.fontRef.is_none()
      {
         // Java: if (this.map.has(3))
            if self.map.has(3)
         {
            // Java: KoBin bin = this.map.byKey(3);
            // Java: this.fontRef = (StyleMatrixReference)KoBin.Decode.toAny(StyleMatrixReference.class, bin, this.zone);
            let bin = self.map.byKey(3);
            self.fontRef = Some(Box::new(StyleMatrixReference::new_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasFontRef())
            else if self.parent.as_ref().map_or(false, |p| p.hasFontRef())
         {
            // Java: this.fontRef = this.parent.fontRef();
            todo!();
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.fontRef = (StyleMatrixReference)KoBin.Decode.toAny(StyleMatrixReference.class, bin, this.zone);
            let null_bin = Null::empty();
            self.fontRef = Some(Box::new(StyleMatrixReference::new_from(null_bin.as_ref())));
         }
      }
      // Java: return this.fontRef;
      self.fontRef.as_deref().unwrap()
    }

    // Java: boolean hasFontRef()
    pub fn hasFontRef(&self) -> bool {
      // Java: return this.map.has(3) || this.fontRef != null && this.fontRef.dirty() || this.parent != null && this.parent.hasFontRef();
      self.map.has(3)
        || self.fontRef.as_ref().map_or(false, |v| v.dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasFontRef())
    }

    // Java: void deleteFontRef()
    pub fn deleteFontRef(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
      // Java: if (this.fontRef != null)
        if self.fontRef.is_some()
      {
         // Java: this.fontRef = null;
         self.fontRef = None;
      }
    }

    // Java: void writeFontRef(StyleMatrixReference value)
    pub fn writeFontRef(&mut self, value: Option<&StyleMatrixReference>) {
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
            // Java: if (this.fontRef != null)
               if self.fontRef.is_some()
            {
               // Java: this.fontRef = null;
               self.fontRef = None;
            }
         }
      }
    }

    // Java: void forceWriteFontRef(StyleMatrixReference value)
    pub fn forceWriteFontRef(&mut self, value: Option<&StyleMatrixReference>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(3, value.lastBin(this.zone));
         self.map.put(3, v.lastBin(self.zone));
         // Java: if (this.fontRef != null)
            if self.fontRef.is_some()
         {
            // Java: this.fontRef = null;
            self.fontRef = None;
         }
      }
    }

    // Java: StyleMatrixReference lnRef()
    pub fn lnRef(&mut self) -> &StyleMatrixReference {
      // Java: if (this.lnRef == null)
        if self.lnRef.is_none()
      {
         // Java: if (this.map.has(4))
            if self.map.has(4)
         {
            // Java: KoBin bin = this.map.byKey(4);
            // Java: this.lnRef = (StyleMatrixReference)KoBin.Decode.toAny(StyleMatrixReference.class, bin, this.zone);
            let bin = self.map.byKey(4);
            self.lnRef = Some(Box::new(StyleMatrixReference::new_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasLnRef())
            else if self.parent.as_ref().map_or(false, |p| p.hasLnRef())
         {
            // Java: this.lnRef = this.parent.lnRef();
            todo!();
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.lnRef = (StyleMatrixReference)KoBin.Decode.toAny(StyleMatrixReference.class, bin, this.zone);
            let null_bin = Null::empty();
            self.lnRef = Some(Box::new(StyleMatrixReference::new_from(null_bin.as_ref())));
         }
      }
      // Java: return this.lnRef;
      self.lnRef.as_deref().unwrap()
    }

    // Java: boolean hasLnRef()
    pub fn hasLnRef(&self) -> bool {
      // Java: return this.map.has(4) || this.lnRef != null && this.lnRef.dirty() || this.parent != null && this.parent.hasLnRef();
      self.map.has(4)
        || self.lnRef.as_ref().map_or(false, |v| v.dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasLnRef())
    }

    // Java: void deleteLnRef()
    pub fn deleteLnRef(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
      // Java: if (this.lnRef != null)
        if self.lnRef.is_some()
      {
         // Java: this.lnRef = null;
         self.lnRef = None;
      }
    }

    // Java: void writeLnRef(StyleMatrixReference value)
    pub fn writeLnRef(&mut self, value: Option<&StyleMatrixReference>) {
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
            // Java: if (this.lnRef != null)
               if self.lnRef.is_some()
            {
               // Java: this.lnRef = null;
               self.lnRef = None;
            }
         }
      }
    }

    // Java: void forceWriteLnRef(StyleMatrixReference value)
    pub fn forceWriteLnRef(&mut self, value: Option<&StyleMatrixReference>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(4, value.lastBin(this.zone));
         self.map.put(4, v.lastBin(self.zone));
         // Java: if (this.lnRef != null)
            if self.lnRef.is_some()
         {
            // Java: this.lnRef = null;
            self.lnRef = None;
         }
      }
    }

}

impl KoAr for ShapeStyle {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for ShapeStyle { fn close(&mut self) { self.close(); } }

