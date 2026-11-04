#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.basic.Transform2D
use crate::cn::wps::drawing::data::basic::Transform2D::Transform2D;
// Java: import cn.wps.drawing.data.text.TextBody
use crate::cn::wps::drawing::data::text::TextBody::TextBody;
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

// Java: public class TextShape implements KoAr, KoClosable
pub struct TextShape {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: TextBody textBody
    pub(crate) textBody: Option<Box<TextBody>>,
    // Java field: Transform2D transform2D
    pub(crate) transform2D: Option<Box<Transform2D>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: TextShape parent
    pub(crate) parent: Option<Box<TextShape>>,
}

impl TextShape {
    // Java: static protected static <Value1 extends TextShape> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::TextShape_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static TextShape create()
    pub fn create() -> Self {
      // Java: return initialize(TextShape.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static TextShape open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(TextShape.class, bin);
      Self::initialize(bin)
    }

    // Java: TextShape()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Self { bin_box: KpBinBox::new(), map: KpMap::new(), zone: KoZone::defaultZone(), parent: None,
        textBody: None, transform2D: None }
    }

    // Java: TextShape(KoBin bin)
    pub fn TextShape_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Self { bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), zone: KoZone::defaultZone(), parent: None,
        textBody: None, transform2D: None }
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
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone;
      self.loadBin(bin, zone);
    }

    // Java: void load(TextShape value)
    pub fn load_value(&mut self, value: Option<&TextShape>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.loadBin(value.lastBin(this.zone), this.zone);
         let bin = v.lastBin(self.zone);
         self.loadBin(bin.as_ref(), self.zone);
      }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: if (this.textBody != null && this.textBody.dirty())
        if self.textBody.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(1, this.textBody.saveBin(zone));
         let bin = self.textBody.as_mut().unwrap().saveBin(zone);
         self.map.put(1, bin);
      }

      // Java: if (this.transform2D != null && this.transform2D.dirty())
        if self.transform2D.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(2, this.transform2D.saveBin(zone));
         let bin = self.transform2D.as_mut().unwrap().saveBin(zone);
         self.map.put(2, bin);
      }

      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(TextShape o)
    pub fn equalsLax(&self, o: &TextShape) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TextShape) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &TextShape, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof TextShape)
        if true
      {
         // Java: TextShape dst = (TextShape)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasTextBody() || !dst.hasTextBody()) && (!this.hasTextBody() || dst.hasTextBody()))
            else if (self.hasTextBody() || !dst.hasTextBody()) && (!self.hasTextBody() || dst.hasTextBody())
         {
            // Java: if (this.hasTextBody() && dst.hasTextBody())
               if self.hasTextBody() && dst.hasTextBody()
            {
               // Java: if (isStrict && !this.textBody().equals(dst.textBody()))
                  if isStrict
               {
                  // Java: return false;
               }

               // Java: if (!isStrict && !this.textBody().equalsLax(dst.textBody()))
                  if !isStrict
               {
                  // Java: return false;
               }
            }

            // Java: if ((this.hasTransform2D() || !dst.hasTransform2D()) && (!this.hasTransform2D() || dst.hasTransform2D()))
               if (self.hasTransform2D() || !dst.hasTransform2D()) && (!self.hasTransform2D() || dst.hasTransform2D())
            {
               // Java: if (this.hasTransform2D() && dst.hasTransform2D())
                  if self.hasTransform2D() && dst.hasTransform2D()
               {
                  // Java: if (isStrict && !this.transform2D().equals(dst.transform2D()))
                     if isStrict
                  {
                     // Java: return false;
                  }

                  // Java: if (!isStrict && !this.transform2D().equalsLax(dst.transform2D()))
                     if !isStrict
                  {
                     // Java: return false;
                  }
               }

               // Java: if ((this.hasUssShapeRect() || !dst.hasUssShapeRect()) && (!this.hasUssShapeRect() || dst.hasUssShapeRect()))
                  if (self.hasUssShapeRect() || !dst.hasUssShapeRect()) && (!self.hasUssShapeRect() || dst.hasUssShapeRect())
               {
                  // Java: return !this.hasUssShapeRect() || !dst.hasUssShapeRect() || this.isUssShapeRect() == dst.isUssShapeRect();
                  return !self.hasUssShapeRect() || !dst.hasUssShapeRect() || self.isUssShapeRect() == dst.isUssShapeRect();
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
      // Java: return false;
      return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasTextBody())
        if self.hasTextBody()
      {
         // Java: ret += this.textBody().hashCode();
         todo!()
      }
      // Java: if (this.hasTransform2D())
        if self.hasTransform2D()
      {
         // Java: ret += this.transform2D().hashCode();
         todo!()
      }
      // Java: if (this.hasUssShapeRect())
        if self.hasUssShapeRect()
      {
         // Java: ret += Boolean.valueOf(this.isUssShapeRect()).hashCode();
         ret = ret.wrapping_add(if self.isUssShapeRect() { 1231 } else { 1237 });
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.textBody != null && this.textBody.dirty())
        if self.textBody.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
        else
      {
         // Java: return this.transform2D != null && this.transform2D.dirty() ? true : this.map.dirty();
         return self.transform2D.as_ref().map_or(false, |v| v.dirty()) || self.map.dirty();
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
    fn cleanFollow(&mut self) {}

    // Java: void cleanCache()
    fn cleanCache(&mut self) {
      // Java: this.textBody = null;
      self.textBody = None;
      // Java: this.transform2D = null;
      self.transform2D = None;
    }

    // Java: void follow(TextShape parent)
    pub fn follow(&mut self, parent: Option<&mut TextShape>) {
      // Java: if (parent != null)
        if let Some(p) = parent
      {
         // Java: parent.discard();
         let pz = p.zone;
         p.discard_with_zone(pz);
         // Java: this.discard();
         let z = self.zone;
         self.discard_with_zone(z);
         // Java: this.parent = parent;
         todo!()
      }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
      // Java: if (this.parent != null)
        if self.parent.is_some()
      {
         // Java: this.discard();
         let z = self.zone;
         self.discard_with_zone(z);
         // Java: this.parent.discard();
         if let Some(ref mut p) = self.parent { let pz = p.zone; p.discard_with_zone(pz); }
         // Java: this.cleanParent();
         self.cleanParent();
      }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
      // Java: return this.parent != null;
      self.parent.is_some()
    }

    // Java: TextShape parent()
    pub fn parent(&self) -> Option<&TextShape> {
      // Java: return this.parent;
      self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      let z = self.zone;
      self.discard_with_zone(z);
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: TextBody textBody()
    pub fn textBody(&mut self) -> &TextBody {
      // Java: if (this.textBody == null)
        if self.textBody.is_none()
      {
         // Java: if (this.map.has(1))
            if self.map.has(1)
         {
            // Java: KoBin bin = this.map.byKey(1);
            // Java: this.textBody = (TextBody)KoBin.Decode.toAny(TextBody.class, bin, this.zone);
            let bin = self.map.byKey(1);
            self.textBody = Some(Box::new(TextBody::TextBody_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasTextBody())
            else if self.parent.as_ref().map_or(false, |p| p.hasTextBody())
         {
            // Java: this.textBody = this.parent.textBody();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.textBody = (TextBody)KoBin.Decode.toAny(TextBody.class, bin, this.zone);
            let null_bin = Null::empty();
            self.textBody = Some(Box::new(TextBody::TextBody_from(null_bin.as_ref())));
         }
      }
      // Java: return this.textBody;
      self.textBody.as_deref().unwrap()
    }

    // Java: boolean hasTextBody()
    pub fn hasTextBody(&self) -> bool {
      // Java: return this.map.has(1) || this.textBody != null && this.textBody.dirty() || this.parent != null && this.parent.hasTextBody();
      self.map.has(1) || self.textBody.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasTextBody())
    }

    // Java: void deleteTextBody()
    pub fn deleteTextBody(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
      // Java: if (this.textBody != null)
        if self.textBody.is_some()
      {
         // Java: this.textBody = null;
         self.textBody = None;
      }
    }

    // Java: void writeTextBody(TextBody value)
    pub fn writeTextBody(&mut self, value: Option<&TextBody>) {
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
            // Java: if (this.textBody != null)
               if self.textBody.is_some()
            {
               // Java: this.textBody = null;
               self.textBody = None;
            }
         }
      }
    }

    // Java: void forceWriteTextBody(TextBody value)
    pub fn forceWriteTextBody(&mut self, value: Option<&TextBody>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(1, v.lastBin(self.zone));
         // Java: if (this.textBody != null)
            if self.textBody.is_some()
         {
            // Java: this.textBody = null;
            self.textBody = None;
         }
      }
    }

    // Java: Transform2D transform2D()
    pub fn transform2D(&mut self) -> &Transform2D {
      // Java: if (this.transform2D == null)
        if self.transform2D.is_none()
      {
         // Java: if (this.map.has(2))
            if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            // Java: this.transform2D = (Transform2D)KoBin.Decode.toAny(Transform2D.class, bin, this.zone);
            let bin = self.map.byKey(2);
            self.transform2D = Some(Box::new(Transform2D::Transform2D_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasTransform2D())
            else if self.parent.as_ref().map_or(false, |p| p.hasTransform2D())
         {
            // Java: this.transform2D = this.parent.transform2D();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.transform2D = (Transform2D)KoBin.Decode.toAny(Transform2D.class, bin, this.zone);
            let null_bin = Null::empty();
            self.transform2D = Some(Box::new(Transform2D::Transform2D_from(null_bin.as_ref())));
         }
      }
      // Java: return this.transform2D;
      self.transform2D.as_deref().unwrap()
    }

    // Java: boolean hasTransform2D()
    pub fn hasTransform2D(&self) -> bool {
      // Java: return this.map.has(2) || this.transform2D != null && this.transform2D.dirty() || this.parent != null && this.parent.hasTransform2D();
      self.map.has(2) || self.transform2D.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasTransform2D())
    }

    // Java: void deleteTransform2D()
    pub fn deleteTransform2D(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.transform2D != null)
        if self.transform2D.is_some()
      {
         // Java: this.transform2D = null;
         self.transform2D = None;
      }
    }

    // Java: void writeTransform2D(Transform2D value)
    pub fn writeTransform2D(&mut self, value: Option<&Transform2D>) {
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
            // Java: if (this.transform2D != null)
               if self.transform2D.is_some()
            {
               // Java: this.transform2D = null;
               self.transform2D = None;
            }
         }
      }
    }

    // Java: void forceWriteTransform2D(Transform2D value)
    pub fn forceWriteTransform2D(&mut self, value: Option<&Transform2D>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.transform2D != null)
            if self.transform2D.is_some()
         {
            // Java: this.transform2D = null;
            self.transform2D = None;
         }
      }
    }

    // Java: boolean isUssShapeRect()
    pub fn isUssShapeRect(&self) -> bool {
      // Java: if (!this.map.has(3))
        if !self.map.has(3)
      {
         // Java: return this.parent == null ? false : this.parent.isUssShapeRect();
         return self.parent.as_ref().map_or(false, |p| p.isUssShapeRect());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(3);
         // Java: return KoBin.Decode.toBool(bin);
         Decode::toBool(bin.as_ref())
      }
    }

    // Java: boolean hasUssShapeRect()
    pub fn hasUssShapeRect(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasUssShapeRect();
      self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasUssShapeRect())
    }

    // Java: void deleteUssShapeRect()
    pub fn deleteUssShapeRect(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
    }

    // Java: void putUssShapeRect(boolean value)
    pub fn putUssShapeRect(&mut self, value: bool) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(3, Encode::from_bool(value, self.zone));
    }

}

impl KoAr for TextShape {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for TextShape { fn close(&mut self) { self.close(); } }
