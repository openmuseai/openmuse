#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.text.TextParagraphProperties
use crate::cn::wps::drawing::data::text::TextParagraphProperties::TextParagraphProperties;
// Java: import cn.wps.drawing.data.text.TextCharacterProperties
use crate::cn::wps::drawing::data::text::TextCharacterProperties::TextCharacterProperties;
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
// Java: import cn.wps.kso.kproto.internal.KpList
use crate::cn::wps::kso::kproto::internal::KpList::KpList;
// Java: import cn.wps.kso.kproto.internal.KpMap
use crate::cn::wps::kso::kproto::internal::KpMap::KpMap;
// Java: import java.util.ArrayList
// Java: import java.util.Collection

// Java: public class TextParagraph implements KoAr, KoClosable
pub struct TextParagraph {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: TextParagraphProperties paraPr
    pub(crate) paraPr: Option<Box<TextParagraphProperties>>,
    // Java field: TextCharacterProperties endParaRPr
    pub(crate) endParaRPr: Option<Box<TextCharacterProperties>>,
    // Java field: TextRunList textruns
    pub(crate) textruns: Option<Box<TextRunList>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: TextParagraph parent
    pub(crate) parent: Option<Box<TextParagraph>>,
}

// Java: public class TextRunList
pub struct TextRunList {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

impl TextParagraph {
    // Java: static protected static <Value1 extends TextParagraph> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::TextParagraph_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static TextParagraph create()
    pub fn create() -> Self {
      // Java: return initialize(TextParagraph.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static TextParagraph open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(TextParagraph.class, bin);
      Self::initialize(bin)
    }

    // Java: TextParagraph()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Self { bin_box: KpBinBox::new(), map: KpMap::new(), zone: KoZone::defaultZone(), parent: None,
        paraPr: None, endParaRPr: None, textruns: None }
    }

    // Java: TextParagraph(KoBin bin)
    pub fn TextParagraph_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Self { bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), zone: KoZone::defaultZone(), parent: None,
        paraPr: None, endParaRPr: None, textruns: None }
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

    // Java: void load(TextParagraph value)
    pub fn load_value(&mut self, value: Option<&TextParagraph>) {
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
      // Java: if (this.paraPr != null && this.paraPr.dirty())
        if self.paraPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(1, this.paraPr.saveBin(zone));
         let bin = self.paraPr.as_mut().unwrap().saveBin(zone);
         self.map.put(1, bin);
      }

      // Java: if (this.endParaRPr != null && this.endParaRPr.dirty())
        if self.endParaRPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(2, this.endParaRPr.saveBin(zone));
         let bin = self.endParaRPr.as_mut().unwrap().saveBin(zone);
         self.map.put(2, bin);
      }

      // Java: if (this.textruns != null && this.textruns.dirty())
        if self.textruns.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(3, this.textruns.saveBin(zone));
         let bin = self.textruns.as_mut().unwrap().saveBin(zone);
         self.map.put(3, bin);
      }

      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(TextParagraph o)
    pub fn equalsLax(&self, o: &TextParagraph) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TextParagraph) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &TextParagraph, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof TextParagraph)
        if true
      {
         // Java: TextParagraph dst = (TextParagraph)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasParaPr() || !dst.hasParaPr()) && (!this.hasParaPr() || dst.hasParaPr()))
            else if (self.hasParaPr() || !dst.hasParaPr()) && (!self.hasParaPr() || dst.hasParaPr())
         {
            // Java: if (this.hasParaPr() && dst.hasParaPr())
               if self.hasParaPr() && dst.hasParaPr()
            {
               // Java: if (isStrict && !this.paraPr().equals(dst.paraPr()))
                  if isStrict
               {
                  // Java: return false;
               }

               // Java: if (!isStrict && !this.paraPr().equalsLax(dst.paraPr()))
                  if !isStrict
               {
                  // Java: return false;
               }
            }

            // Java: if ((this.hasEndParaRPr() || !dst.hasEndParaRPr()) && (!this.hasEndParaRPr() || dst.hasEndParaRPr()))
               if (self.hasEndParaRPr() || !dst.hasEndParaRPr()) && (!self.hasEndParaRPr() || dst.hasEndParaRPr())
            {
               // Java: if (this.hasEndParaRPr() && dst.hasEndParaRPr())
                  if self.hasEndParaRPr() && dst.hasEndParaRPr()
               {
                  // Java: if (isStrict && !this.endParaRPr().equals(dst.endParaRPr()))
                     if isStrict
                  {
                     // Java: return false;
                  }

                  // Java: if (!isStrict && !this.endParaRPr().equalsLax(dst.endParaRPr()))
                     if !isStrict
                  {
                     // Java: return false;
                  }
               }

               // Java: if ((this.hasTextruns() || !dst.hasTextruns()) && (!this.hasTextruns() || dst.hasTextruns()))
                  if (self.hasTextruns() || !dst.hasTextruns()) && (!self.hasTextruns() || dst.hasTextruns())
               {
                  // Java: if (this.hasTextruns() && dst.hasTextruns())
                     if self.hasTextruns() && dst.hasTextruns()
                  {
                     // Java: if (isStrict && !this.textruns().equals(dst.textruns()))
                        if isStrict
                     {
                        // Java: return false;
                     }

                     // Java: if (!isStrict && !this.textruns().equalsLax(dst.textruns()))
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
      // Java: else
      // Java: return false;
      return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasParaPr())
        if self.hasParaPr()
      {
         // Java: ret += this.paraPr().hashCode();
         todo!()
      }
      // Java: if (this.hasEndParaRPr())
        if self.hasEndParaRPr()
      {
         // Java: ret += this.endParaRPr().hashCode();
         todo!()
      }
      // Java: if (this.hasTextruns())
        if self.hasTextruns()
      {
         // Java: ret += this.textruns().hashCode();
         todo!()
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.paraPr != null && this.paraPr.dirty())
        if self.paraPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.endParaRPr != null && this.endParaRPr.dirty())
        else if self.endParaRPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
        else
      {
         // Java: return this.textruns != null && this.textruns.dirty() ? true : this.map.dirty();
         return self.textruns.as_ref().map_or(false, |v| v.dirty()) || self.map.dirty();
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
      // Java: this.paraPr = null;
      self.paraPr = None;
      // Java: this.endParaRPr = null;
      self.endParaRPr = None;
      // Java: this.textruns = null;
      self.textruns = None;
    }

    // Java: void follow(TextParagraph parent)
    pub fn follow(&mut self, parent: Option<&mut TextParagraph>) {
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

    // Java: TextParagraph parent()
    pub fn parent(&self) -> Option<&TextParagraph> {
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

    // Java: TextParagraphProperties paraPr()
    pub fn paraPr(&mut self) -> &TextParagraphProperties {
      // Java: if (this.paraPr == null)
        if self.paraPr.is_none()
      {
         // Java: if (this.map.has(1))
            if self.map.has(1)
         {
            // Java: KoBin bin = this.map.byKey(1);
            // Java: this.paraPr = (TextParagraphProperties)KoBin.Decode.toAny(TextParagraphProperties.class, bin, this.zone);
            let bin = self.map.byKey(1);
            self.paraPr = Some(Box::new(TextParagraphProperties::TextParagraphProperties_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasParaPr())
            else if self.parent.as_ref().map_or(false, |p| p.hasParaPr())
         {
            // Java: this.paraPr = this.parent.paraPr();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.paraPr = (TextParagraphProperties)KoBin.Decode.toAny(TextParagraphProperties.class, bin, this.zone);
            let null_bin = Null::empty();
            self.paraPr = Some(Box::new(TextParagraphProperties::TextParagraphProperties_from(null_bin.as_ref())));
         }
      }
      // Java: return this.paraPr;
      self.paraPr.as_deref().unwrap()
    }

    // Java: boolean hasParaPr()
    pub fn hasParaPr(&self) -> bool {
      // Java: return this.map.has(1) || this.paraPr != null && this.paraPr.dirty() || this.parent != null && this.parent.hasParaPr();
      self.map.has(1) || self.paraPr.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasParaPr())
    }

    // Java: void deleteParaPr()
    pub fn deleteParaPr(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
      // Java: if (this.paraPr != null)
        if self.paraPr.is_some()
      {
         // Java: this.paraPr = null;
         self.paraPr = None;
      }
    }

    // Java: void writeParaPr(TextParagraphProperties value)
    pub fn writeParaPr(&mut self, value: Option<&TextParagraphProperties>) {
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
            // Java: if (this.paraPr != null)
               if self.paraPr.is_some()
            {
               // Java: this.paraPr = null;
               self.paraPr = None;
            }
         }
      }
    }

    // Java: void forceWriteParaPr(TextParagraphProperties value)
    pub fn forceWriteParaPr(&mut self, value: Option<&TextParagraphProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(1, v.lastBin(self.zone));
         // Java: if (this.paraPr != null)
            if self.paraPr.is_some()
         {
            // Java: this.paraPr = null;
            self.paraPr = None;
         }
      }
    }

    // Java: TextCharacterProperties endParaRPr()
    pub fn endParaRPr(&mut self) -> &TextCharacterProperties {
      // Java: if (this.endParaRPr == null)
        if self.endParaRPr.is_none()
      {
         // Java: if (this.map.has(2))
            if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            // Java: this.endParaRPr = (TextCharacterProperties)KoBin.Decode.toAny(TextCharacterProperties.class, bin, this.zone);
            let bin = self.map.byKey(2);
            self.endParaRPr = Some(Box::new(TextCharacterProperties::new_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasEndParaRPr())
            else if self.parent.as_ref().map_or(false, |p| p.hasEndParaRPr())
         {
            // Java: this.endParaRPr = this.parent.endParaRPr();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.endParaRPr = (TextCharacterProperties)KoBin.Decode.toAny(TextCharacterProperties.class, bin, this.zone);
            let null_bin = Null::empty();
            self.endParaRPr = Some(Box::new(TextCharacterProperties::new_from(null_bin.as_ref())));
         }
      }
      // Java: return this.endParaRPr;
      self.endParaRPr.as_deref().unwrap()
    }

    // Java: boolean hasEndParaRPr()
    pub fn hasEndParaRPr(&self) -> bool {
      // Java: return this.map.has(2) || this.endParaRPr != null && this.endParaRPr.dirty() || this.parent != null && this.parent.hasEndParaRPr();
      self.map.has(2) || self.endParaRPr.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasEndParaRPr())
    }

    // Java: void deleteEndParaRPr()
    pub fn deleteEndParaRPr(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.endParaRPr != null)
        if self.endParaRPr.is_some()
      {
         // Java: this.endParaRPr = null;
         self.endParaRPr = None;
      }
    }

    // Java: void writeEndParaRPr(TextCharacterProperties value)
    pub fn writeEndParaRPr(&mut self, value: Option<&TextCharacterProperties>) {
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
            // Java: if (this.endParaRPr != null)
               if self.endParaRPr.is_some()
            {
               // Java: this.endParaRPr = null;
               self.endParaRPr = None;
            }
         }
      }
    }

    // Java: void forceWriteEndParaRPr(TextCharacterProperties value)
    pub fn forceWriteEndParaRPr(&mut self, value: Option<&TextCharacterProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.endParaRPr != null)
            if self.endParaRPr.is_some()
         {
            // Java: this.endParaRPr = null;
            self.endParaRPr = None;
         }
      }
    }

    // Java: TextRunList textruns()
    pub fn textruns(&mut self) -> &TextRunList {
      // Java: if (this.textruns == null)
        if self.textruns.is_none()
      {
         // Java: if (this.map.has(3))
            if self.map.has(3)
         {
            // Java: KoBin bin = this.map.byKey(3);
            // Java: this.textruns = (TextRunList)KoBin.Decode.toAny(TextRunList.class, bin, this.zone);
            let bin = self.map.byKey(3);
            self.textruns = Some(Box::new(TextRunList::TextRunList_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasTextruns())
            else if self.parent.as_ref().map_or(false, |p| p.hasTextruns())
         {
            // Java: this.textruns = this.parent.textruns();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.textruns = (TextRunList)KoBin.Decode.toAny(TextRunList.class, bin, this.zone);
            let null_bin = Null::empty();
            self.textruns = Some(Box::new(TextRunList::TextRunList_from(null_bin.as_ref())));
         }
      }
      // Java: return this.textruns;
      self.textruns.as_deref().unwrap()
    }

    // Java: boolean hasTextruns()
    pub fn hasTextruns(&self) -> bool {
      // Java: return this.map.has(3) || this.textruns != null && this.textruns.dirty() || this.parent != null && this.parent.hasTextruns();
      self.map.has(3) || self.textruns.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasTextruns())
    }

    // Java: void deleteTextruns()
    pub fn deleteTextruns(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
      // Java: if (this.textruns != null)
        if self.textruns.is_some()
      {
         // Java: this.textruns = null;
         self.textruns = None;
      }
    }

    // Java: void writeTextruns(TextRunList value)
    pub fn writeTextruns(&mut self, value: Option<&TextRunList>) {
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
            // Java: if (this.textruns != null)
               if self.textruns.is_some()
            {
               // Java: this.textruns = null;
               self.textruns = None;
            }
         }
      }
    }

    // Java: void forceWriteTextruns(TextRunList value)
    pub fn forceWriteTextruns(&mut self, value: Option<&TextRunList>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(3, value.lastBin(this.zone));
         self.map.put(3, v.lastBin(self.zone));
         // Java: if (this.textruns != null)
            if self.textruns.is_some()
         {
            // Java: this.textruns = null;
            self.textruns = None;
         }
      }
    }

}

impl KoAr for TextParagraph {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for TextParagraph { fn close(&mut self) { self.close(); } }

// ============================================================
// Inner class: TextRunList
// ============================================================

impl TextRunList {
    // Java: TextRunList()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.list = new KpList(this.zone);
      let zone = KoZone::defaultZone();
      Self { bin_box: KpBinBox::new(), list: KpList::new(zone), zone }
    }

    // Java: TextRunList(KoBin bin)
    pub fn TextRunList_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.list = new KpList(bin, this.zone);
      let zone = KoZone::defaultZone();
      Self { bin_box: KpBinBox::KpBinBox_from(Some(bin)), list: KpList::KpList_from(bin, zone), zone }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
      // Java: this.box.set(bin);
      self.bin_box.set(Some(bin));
      // Java: this.list.load(this.box.get(), zone);
      let bin_ref = self.bin_box.get();
      self.list.load(bin_ref.as_ref(), zone);
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone;
      self.loadBin(bin, zone);
    }

    // Java: void load(TextRunList value)
    pub fn load_value(&mut self, value: Option<&TextRunList>) {
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
      // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
      if self.list.dirty() { let saved = self.list.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(TextRunList o)
    pub fn equalsLax(&self, o: &TextRunList) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TextRunList) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &TextRunList, isStrict: bool) -> bool {
      // Java: if (null == o)
      // Java: return false;
      // Java: else
      {
         // Java: TextRunList dst = (TextRunList)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else
            else
         {
            // Java: ArrayList<TextRun> dstList = new ArrayList();
            let mut dstList: Vec<()> = Vec::new();
            // Java: dst.read(dstList);
            dst.read(&mut dstList);
            // Java: ArrayList<TextRun> srcList = new ArrayList();
            let mut srcList: Vec<()> = Vec::new();
            // Java: this.read(srcList);
            self.read(&mut srcList);
            // Java: if (dstList.size() != srcList.size())
               if dstList.len() != srcList.len()
            {
               // Java: return false;
               return false;
            }
            // Java: else
               else
            {
               // Java: int size = dstList.size();
               let size = dstList.len();
               // Java: for(int i = 0; i < size; ++i)
                  for i in 0..size
               {
                  // Java: TextRun dstClrTrans = (TextRun)dstList.get(i);
                  let _dstClrTrans = &dstList[i];
                  // Java: TextRun srcClrTrans = (TextRun)srcList.get(i);
                  let _srcClrTrans = &srcList[i];
                  // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                     if isStrict
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                     if !isStrict
                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: return true;
               return true;
            }
         }
      }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return this.list.hashCode();
      self.list.hashCode()
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: return this.list.dirty();
      self.list.dirty()
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
      // Java: this.saveBin(this.zone);
      let zone = self.zone;
      self.saveBin(zone);
      // Java: return this.box.get();
      self.bin_box.get()
    }

    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
      // Java: this.loadBin(this.box.get(), zone);
      let bin = self.bin_box.get();
      self.loadBin(bin.as_ref(), zone);
    }

    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
      // Java: return KoBin.Factory.dup(this.box.get(), zone);
      Factory::dup(self.bin_box.get().as_ref(), zone)
    }

    // Java: void close()
    pub fn close(&mut self) {
      // Java: this.list.load(KoBin.Null.value1(), this.zone);
      let null_bin = Null::empty();
      self.list.load(null_bin.as_ref(), self.zone);
      // Java: this.zone.leave();
      // Java: this.zone = KoZone.defaultZone();
      self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends TextRunList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::TextRunList_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static TextRunList create()
    pub fn create() -> Self {
      // Java: return initialize(TextRunList.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static TextRunList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(TextRunList.class, bin);
      Self::initialize(bin)
    }

    // Java: void read(Collection<TextRun> valueLst)
    pub fn read(&self, valueLst: &mut Vec<()>) {
      // Java: valueLst.clear();
      valueLst.clear();
      // Java: int size = this.list.size();
      let size = self.list.size();
      // Java: for(int i = 0; i < size; ++i)
        for i in 0..size
      {
         // Java: valueLst.add((TextRun)KoBin.Decode.toAny(TextRun.class, this.list.get(i), this.zone));
         todo!()
      }
    }

    // Java: void write(Iterable<TextRun> valueLst)
    pub fn write(&mut self, valueLst: &[()]) {
      // Java: this.list.clear();
      self.list.clear();
      // Java: for(TextRun i : valueLst)
        for _i in valueLst
      {
         // Java: KoBin bin = i.lastBin(this.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         // Java: this.list.add(bin);
         todo!()
      }
    }

    // Java: void forceWrite(Iterable<TextRun> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[()]) {
      // Java: this.list.clear();
      self.list.clear();
      // Java: for(TextRun i : valueLst)
        for _i in valueLst
      {
         // Java: this.list.add(i.lastBin(this.zone));
         todo!()
      }
    }

}

impl KoAr for TextRunList {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for TextRunList { fn close(&mut self) { self.close(); } }
