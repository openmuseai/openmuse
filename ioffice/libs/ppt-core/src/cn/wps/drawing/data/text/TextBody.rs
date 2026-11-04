#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.text.TextListStyle
use crate::cn::wps::drawing::data::text::textbody_hosts::TextListStyle;
// Java: import cn.wps.drawing.data.text.TextBodyProperties
use crate::cn::wps::drawing::data::text::textbody_hosts::TextBodyProperties;
// Java: import cn.wps.drawing.data.text.TextParagraph
use crate::cn::wps::drawing::data::text::textbody_hosts::TextParagraph;
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

// Java: public class TextBody implements KoAr, KoClosable
pub struct TextBody {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: TextListStyle listStyle
    pub(crate) listStyle: Option<Box<TextListStyle>>,
    // Java field: TextBodyProperties bodyPr
    pub(crate) bodyPr: Option<Box<TextBodyProperties>>,
    // Java field: ParagraphList paraLst
    pub(crate) paraLst: Option<Box<ParagraphList>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: TextBody parent
    pub(crate) parent: Option<Box<TextBody>>,
}

// Java: public class ParagraphList
pub struct ParagraphList {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

impl TextBody {
    // Java: static protected static <Value1 extends TextBody> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::TextBody_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load(bin);
      // Java: return inst;
      inst
    }

    // Java: static TextBody create()
    pub fn create() -> Self {
      // Java: return initialize(TextBody.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static TextBody open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(TextBody.class, bin);
      Self::initialize(bin)
    }

    // Java: TextBody()
    pub fn TextBody() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Self { bin_box: KpBinBox::new(), map: KpMap::new(), zone: KoZone::defaultZone(), parent: None,
        listStyle: None, bodyPr: None, paraLst: None }
    }

    // Java: TextBody(KoBin bin)
    pub fn TextBody_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Self { bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), zone: KoZone::defaultZone(), parent: None,
        listStyle: None, bodyPr: None, paraLst: None }
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

    // Java: void load(TextBody value)
    pub fn load_value(&mut self, value: Option<&TextBody>) {
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
      // Java: if (this.listStyle != null && this.listStyle.dirty())
        if self.listStyle.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(1, this.listStyle.saveBin(zone));
         let bin = self.listStyle.as_mut().unwrap().saveBin(zone);
         self.map.put(1, bin);
      }

      // Java: if (this.bodyPr != null && this.bodyPr.dirty())
        if self.bodyPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(2, this.bodyPr.saveBin(zone));
         let bin = self.bodyPr.as_mut().unwrap().saveBin(zone);
         self.map.put(2, bin);
      }

      // Java: if (this.paraLst != null && this.paraLst.dirty())
        if self.paraLst.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(3, this.paraLst.saveBin(zone));
         let bin = self.paraLst.as_mut().unwrap().saveBin(zone);
         self.map.put(3, bin);
      }

      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(TextBody o)
    pub fn equalsLax(&self, o: &TextBody) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TextBody) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &TextBody, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof TextBody)
        if true
      {
         // Java: TextBody dst = (TextBody)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasListStyle() || !dst.hasListStyle()) && (!this.hasListStyle() || dst.hasListStyle()))
            else if (self.hasListStyle() || !dst.hasListStyle()) && (!self.hasListStyle() || dst.hasListStyle())
         {
            // Java: if (this.hasListStyle() && dst.hasListStyle())
               if self.hasListStyle() && dst.hasListStyle()
            {
               // Java: if (isStrict && !this.listStyle().equals(dst.listStyle()))
                  if isStrict
               {
                  // Java: return false;
               }

               // Java: if (!isStrict && !this.listStyle().equalsLax(dst.listStyle()))
                  if !isStrict
               {
                  // Java: return false;
               }
            }

            // Java: if ((this.hasBodyPr() || !dst.hasBodyPr()) && (!this.hasBodyPr() || dst.hasBodyPr()))
               if (self.hasBodyPr() || !dst.hasBodyPr()) && (!self.hasBodyPr() || dst.hasBodyPr())
            {
               // Java: if (this.hasBodyPr() && dst.hasBodyPr())
                  if self.hasBodyPr() && dst.hasBodyPr()
               {
                  // Java: if (isStrict && !this.bodyPr().equals(dst.bodyPr()))
                     if isStrict
                  {
                     // Java: return false;
                  }

                  // Java: if (!isStrict && !this.bodyPr().equalsLax(dst.bodyPr()))
                     if !isStrict
                  {
                     // Java: return false;
                  }
               }

               // Java: if ((this.hasParaLst() || !dst.hasParaLst()) && (!this.hasParaLst() || dst.hasParaLst()))
                  if (self.hasParaLst() || !dst.hasParaLst()) && (!self.hasParaLst() || dst.hasParaLst())
               {
                  // Java: if (this.hasParaLst() && dst.hasParaLst())
                     if self.hasParaLst() && dst.hasParaLst()
                  {
                     // Java: if (isStrict && !this.paraLst().equals(dst.paraLst()))
                        if isStrict
                     {
                        // Java: return false;
                     }

                     // Java: if (!isStrict && !this.paraLst().equalsLax(dst.paraLst()))
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
      // Java: if (this.hasListStyle())
        if self.hasListStyle()
      {
         // Java: ret += this.listStyle().hashCode();
         todo!()
      }
      // Java: if (this.hasBodyPr())
        if self.hasBodyPr()
      {
         // Java: ret += this.bodyPr().hashCode();
         todo!()
      }
      // Java: if (this.hasParaLst())
        if self.hasParaLst()
      {
         // Java: ret += this.paraLst().hashCode();
         todo!()
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.listStyle != null && this.listStyle.dirty())
        if self.listStyle.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.bodyPr != null && this.bodyPr.dirty())
        else if self.bodyPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
        else
      {
         // Java: return this.paraLst != null && this.paraLst.dirty() ? true : this.map.dirty();
         return self.paraLst.as_ref().map_or(false, |v| v.dirty()) || self.map.dirty();
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
      // Java: this.listStyle = null;
      self.listStyle = None;
      // Java: this.bodyPr = null;
      self.bodyPr = None;
      // Java: this.paraLst = null;
      self.paraLst = None;
    }

    // Java: void follow(TextBody parent)
    pub fn follow(&mut self, parent: Option<&mut TextBody>) {
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

    // Java: TextBody parent()
    pub fn parent(&self) -> Option<&TextBody> {
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

    // Java: TextListStyle listStyle()
    pub fn listStyle(&mut self) -> &TextListStyle {
      // Java: if (this.listStyle == null)
        if self.listStyle.is_none()
      {
         // Java: if (this.map.has(1))
            if self.map.has(1)
         {
            // Java: KoBin bin = this.map.byKey(1);
            // Java: this.listStyle = (TextListStyle)KoBin.Decode.toAny(TextListStyle.class, bin, this.zone);
            let bin = self.map.byKey(1);
            self.listStyle = Some(Box::new(TextListStyle::TextListStyle_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasListStyle())
            else if self.parent.as_ref().map_or(false, |p| p.hasListStyle())
         {
            // Java: this.listStyle = this.parent.listStyle();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.listStyle = (TextListStyle)KoBin.Decode.toAny(TextListStyle.class, bin, this.zone);
            let null_bin = Null::empty();
            self.listStyle = Some(Box::new(TextListStyle::TextListStyle_from(null_bin.as_ref())));
         }
      }
      // Java: return this.listStyle;
      self.listStyle.as_deref().unwrap()
    }

    // Java: boolean hasListStyle()
    pub fn hasListStyle(&self) -> bool {
      // Java: return this.map.has(1) || this.listStyle != null && this.listStyle.dirty() || this.parent != null && this.parent.hasListStyle();
      self.map.has(1) || self.listStyle.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasListStyle())
    }

    // Java: void deleteListStyle()
    pub fn deleteListStyle(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
      // Java: if (this.listStyle != null)
        if self.listStyle.is_some()
      {
         // Java: this.listStyle = null;
         self.listStyle = None;
      }
    }

    // Java: void writeListStyle(TextListStyle value)
    pub fn writeListStyle(&mut self, value: Option<&TextListStyle>) {
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
            // Java: if (this.listStyle != null)
               if self.listStyle.is_some()
            {
               // Java: this.listStyle = null;
               self.listStyle = None;
            }
         }
      }
    }

    // Java: void forceWriteListStyle(TextListStyle value)
    pub fn forceWriteListStyle(&mut self, value: Option<&TextListStyle>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(1, v.lastBin(self.zone));
         // Java: if (this.listStyle != null)
            if self.listStyle.is_some()
         {
            // Java: this.listStyle = null;
            self.listStyle = None;
         }
      }
    }

    // Java: TextBodyProperties bodyPr()
    pub fn bodyPr(&mut self) -> &TextBodyProperties {
      // Java: if (this.bodyPr == null)
        if self.bodyPr.is_none()
      {
         // Java: if (this.map.has(2))
            if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            // Java: this.bodyPr = (TextBodyProperties)KoBin.Decode.toAny(TextBodyProperties.class, bin, this.zone);
            let bin = self.map.byKey(2);
            self.bodyPr = Some(Box::new(TextBodyProperties::TextBodyProperties_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasBodyPr())
            else if self.parent.as_ref().map_or(false, |p| p.hasBodyPr())
         {
            // Java: this.bodyPr = this.parent.bodyPr();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.bodyPr = (TextBodyProperties)KoBin.Decode.toAny(TextBodyProperties.class, bin, this.zone);
            let null_bin = Null::empty();
            self.bodyPr = Some(Box::new(TextBodyProperties::TextBodyProperties_from(null_bin.as_ref())));
         }
      }
      // Java: return this.bodyPr;
      self.bodyPr.as_deref().unwrap()
    }

    // Java: boolean hasBodyPr()
    pub fn hasBodyPr(&self) -> bool {
      // Java: return this.map.has(2) || this.bodyPr != null && this.bodyPr.dirty() || this.parent != null && this.parent.hasBodyPr();
      self.map.has(2) || self.bodyPr.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasBodyPr())
    }

    // Java: void deleteBodyPr()
    pub fn deleteBodyPr(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.bodyPr != null)
        if self.bodyPr.is_some()
      {
         // Java: this.bodyPr = null;
         self.bodyPr = None;
      }
    }

    // Java: void writeBodyPr(TextBodyProperties value)
    pub fn writeBodyPr(&mut self, value: Option<&TextBodyProperties>) {
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
            // Java: if (this.bodyPr != null)
               if self.bodyPr.is_some()
            {
               // Java: this.bodyPr = null;
               self.bodyPr = None;
            }
         }
      }
    }

    // Java: void forceWriteBodyPr(TextBodyProperties value)
    pub fn forceWriteBodyPr(&mut self, value: Option<&TextBodyProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.bodyPr != null)
            if self.bodyPr.is_some()
         {
            // Java: this.bodyPr = null;
            self.bodyPr = None;
         }
      }
    }

    // Java: ParagraphList paraLst()
    pub fn paraLst(&mut self) -> &mut ParagraphList {
      // Java: if (this.paraLst == null)
        if self.paraLst.is_none()
      {
         // Java: if (this.map.has(3))
            if self.map.has(3)
         {
            // Java: KoBin bin = this.map.byKey(3);
            // Java: this.paraLst = (ParagraphList)KoBin.Decode.toAny(ParagraphList.class, bin, this.zone);
            let bin = self.map.byKey(3);
            self.paraLst = Some(Box::new(ParagraphList::ParagraphList_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasParaLst())
            else if self.parent.as_ref().map_or(false, |p| p.hasParaLst())
         {
            // Java: this.paraLst = this.parent.paraLst();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.paraLst = (ParagraphList)KoBin.Decode.toAny(ParagraphList.class, bin, this.zone);
            let null_bin = Null::empty();
            self.paraLst = Some(Box::new(ParagraphList::ParagraphList_from(null_bin.as_ref())));
         }
      }
      // Java: return this.paraLst;
      self.paraLst.as_deref_mut().unwrap()
    }

    // Java: boolean hasParaLst()
    pub fn hasParaLst(&self) -> bool {
      // Java: return this.map.has(3) || this.paraLst != null && this.paraLst.dirty() || this.parent != null && this.parent.hasParaLst();
      self.map.has(3) || self.paraLst.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasParaLst())
    }

    // Java: void deleteParaLst()
    pub fn deleteParaLst(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
      // Java: if (this.paraLst != null)
        if self.paraLst.is_some()
      {
         // Java: this.paraLst = null;
         self.paraLst = None;
      }
    }

    // Java: void writeParaLst(ParagraphList value)
    pub fn writeParaLst(&mut self, value: Option<&ParagraphList>) {
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
            // Java: if (this.paraLst != null)
               if self.paraLst.is_some()
            {
               // Java: this.paraLst = null;
               self.paraLst = None;
            }
         }
      }
    }

    // Java: void forceWriteParaLst(ParagraphList value)
    pub fn forceWriteParaLst(&mut self, value: Option<&ParagraphList>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(3, value.lastBin(this.zone));
         self.map.put(3, v.lastBin(self.zone));
         // Java: if (this.paraLst != null)
            if self.paraLst.is_some()
         {
            // Java: this.paraLst = null;
            self.paraLst = None;
         }
      }
    }

}

impl KoAr for TextBody {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for TextBody { fn close(&mut self) { self.close(); } }

impl Default for TextBody {
    fn default() -> Self {
        Self::create()
    }
}

// ============================================================
// Inner class: ParagraphList
// ============================================================

impl ParagraphList {
    // Java: ParagraphList()
    pub fn ParagraphList() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.list = new KpList(this.zone);
      let zone = KoZone::defaultZone();
      Self { bin_box: KpBinBox::new(), list: KpList::new(zone), zone }
    }

    // Java: ParagraphList(KoBin bin)
    pub fn ParagraphList_from(bin: &dyn KoBinTrait) -> Self {
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
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone;
      self.loadBin(bin, zone);
    }

    // Java: void load(ParagraphList value)
    pub fn load_value(&mut self, value: Option<&ParagraphList>) {
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

    // Java: boolean equalsLax(ParagraphList o)
    pub fn equalsLax(&self, o: &ParagraphList) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &ParagraphList) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &ParagraphList, isStrict: bool) -> bool {
      // Java: if (null == o)
      // Java: return false;
      // Java: else
      {
         // Java: ParagraphList dst = (ParagraphList)o;
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
            // Java: ArrayList<TextParagraph> dstList = new ArrayList();
            let mut dstList: Vec<()> = Vec::new();
            // Java: dst.read(dstList);
            dst.read(&mut dstList);
            // Java: ArrayList<TextParagraph> srcList = new ArrayList();
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
                  // Java: TextParagraph dstClrTrans = (TextParagraph)dstList.get(i);
                  let _dstClrTrans = &dstList[i];
                  // Java: TextParagraph srcClrTrans = (TextParagraph)srcList.get(i);
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

    // Java: static protected static <Value1 extends ParagraphList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::ParagraphList_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load(bin);
      // Java: return inst;
      inst
    }

    // Java: static ParagraphList create()
    pub fn create() -> Self {
      // Java: return initialize(ParagraphList.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static ParagraphList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(ParagraphList.class, bin);
      Self::initialize(bin)
    }

    // Java: void read(Collection<TextParagraph> valueLst)
    pub fn read(&self, valueLst: &mut Vec<()>) {
      // Java: valueLst.clear();
      valueLst.clear();
      // Java: int size = this.list.size();
      let size = self.list.size();
      // Java: for(int i = 0; i < size; ++i)
        for i in 0..size
      {
         // Java: valueLst.add((TextParagraph)KoBin.Decode.toAny(TextParagraph.class, this.list.get(i), this.zone));
         todo!()
      }
    }

    // Java: void write(Iterable<TextParagraph> valueLst)
    pub fn write(&mut self, valueLst: &[()]) {
      // Java: this.list.clear();
      self.list.clear();
      // Java: for(TextParagraph i : valueLst)
        for _i in valueLst
      {
         // Java: KoBin bin = i.lastBin(this.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
         // Java: this.list.add(bin);
         todo!()
      }
    }

    // Java: void forceWrite(Iterable<TextParagraph> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[std::rc::Rc<std::cell::RefCell<TextParagraph>>]) {
      // Java: this.list.clear();
      self.list.clear();
      // Java: for(TextParagraph i : valueLst)
        for i in valueLst
      {
         // Java: this.list.add(i.lastBin(this.zone));
         let bin = i.borrow().lastBin(self.zone);
         self.list.add(bin);
      }
    }

    pub fn size(&self) -> i32 {
        self.list.size()
    }
}

impl KoAr for ParagraphList {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for ParagraphList { fn close(&mut self) { self.close(); } }