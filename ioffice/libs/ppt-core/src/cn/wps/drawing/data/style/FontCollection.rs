#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.font.TextFont
use crate::cn::wps::drawing::data::font::TextFont::TextFont;
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
// (no Rust equivalent needed)
// Java: import java.util.Collection
// (no Rust equivalent needed)

// Java: public class FontCollection implements KoAr, KoClosable
pub struct FontCollection {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: TextFont latin
    pub(crate) latin: Option<Box<TextFont>>,
    // Java field: TextFont ea
    pub(crate) ea: Option<Box<TextFont>>,
    // Java field: TextFont cs
    pub(crate) cs: Option<Box<TextFont>>,
    // Java field: SupplementalFontList extLst
    pub(crate) extLst: Option<Box<SupplementalFontList>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: FontCollection parent
    pub(crate) parent: Option<Box<FontCollection>>,
}

// Java: public class SupplementalFont
pub struct SupplementalFont {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: SupplementalFont parent
    pub(crate) parent: Option<Box<SupplementalFont>>,
}

// Java: public class SupplementalFontList
pub struct SupplementalFontList {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

// Java: public class FontCollection implements KoAr, KoClosable
impl FontCollection {
    // Java: private static final int $latin = 1
    const FIELD_LATIN: i32 = 1;
    // Java: private static final int $ea = 2
    const FIELD_EA: i32 = 2;
    // Java: private static final int $cs = 3
    const FIELD_CS: i32 = 3;
    // Java: private static final int $extLst = 4
    const FIELD_EXTLST: i32 = 4;

    // Java: static protected static <Value1 extends FontCollection> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::FontCollection_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      // (load already done in FontCollection_from)
      // Java: return inst;
      inst
    }

    // Java: static FontCollection create()
    pub fn create() -> Self {
      // Java: return initialize(FontCollection.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static FontCollection open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(FontCollection.class, bin);
      Self::initialize(bin)
    }

    // Java: FontCollection()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Self {
        zone: KoZone::defaultZone(),
        latin: None,
        ea: None,
        cs: None,
        extLst: None,
        bin_box: KpBinBox::new(),
        map: KpMap::new(),
        parent: None,
      }
    }

    // Java: FontCollection(KoBin bin)
    pub fn FontCollection_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Self {
        zone: KoZone::defaultZone(),
        latin: None,
        ea: None,
        cs: None,
        extLst: None,
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
      self.loadBin(bin, self.zone);
    }

    // Java: void load(FontCollection value)
    pub fn load_from(&mut self, value: Option<&mut FontCollection>) {
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
      // Java: if (this.latin != null && this.latin.dirty())
      if let Some(ref mut f) = self.latin { if f.dirty()
      {
         // Java: this.map.put(1, this.latin.saveBin(zone));
         let bin = f.saveBin(zone);
         self.map.put(1, bin);
      }}
      // Java: if (this.ea != null && this.ea.dirty())
      if let Some(ref mut f) = self.ea { if f.dirty()
      {
         // Java: this.map.put(2, this.ea.saveBin(zone));
         let bin = f.saveBin(zone);
         self.map.put(2, bin);
      }}
      // Java: if (this.cs != null && this.cs.dirty())
      if let Some(ref mut f) = self.cs { if f.dirty()
      {
         // Java: this.map.put(3, this.cs.saveBin(zone));
         let bin = f.saveBin(zone);
         self.map.put(3, bin);
      }}
      // Java: if (this.extLst != null && this.extLst.dirty())
      if let Some(ref mut f) = self.extLst { if f.dirty()
      {
         // Java: this.map.put(4, this.extLst.saveBin(zone));
         let bin = f.saveBin(zone);
         self.map.put(4, bin);
      }}
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() {
        let saved = self.map.save(zone);
        self.bin_box.set(Some(saved.as_ref()));
        self.bin_box.get()
      } else {
        self.bin_box.get()
      }
    }

    // Java: boolean equalsLax(FontCollection o)
    pub fn equalsLax(&mut self, o: &mut FontCollection) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&mut self, o: &mut FontCollection) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&mut self, o: &mut FontCollection, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof FontCollection)
      {
         // Java: FontCollection dst = (FontCollection)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self as *const _, dst as *const _)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasLatin() || !dst.hasLatin()) && (!this.hasLatin() || dst.hasLatin()))
         else if (self.hasLatin() || !dst.hasLatin()) && (!self.hasLatin() || dst.hasLatin())
         {
            // Java: if (this.hasLatin() && dst.hasLatin())
            if self.hasLatin() && dst.hasLatin()
            {
               // Java: if (isStrict && !this.latin().equals(dst.latin()))
               if isStrict && !self.latin().equals(dst.latin())
               {
                  // Java: return false;
                  return false;
               }
               // Java: if (!isStrict && !this.latin().equalsLax(dst.latin()))
               if !isStrict && !self.latin().equalsLax(dst.latin())
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: if ((this.hasEa() || !dst.hasEa()) && (!this.hasEa() || dst.hasEa()))
            if (self.hasEa() || !dst.hasEa()) && (!self.hasEa() || dst.hasEa())
            {
               // Java: if (this.hasEa() && dst.hasEa())
               if self.hasEa() && dst.hasEa()
               {
                  // Java: if (isStrict && !this.ea().equals(dst.ea()))
                  if isStrict && !self.ea().equals(dst.ea())
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !this.ea().equalsLax(dst.ea()))
                  if !isStrict && !self.ea().equalsLax(dst.ea())
                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: if ((this.hasCs() || !dst.hasCs()) && (!this.hasCs() || dst.hasCs()))
               if (self.hasCs() || !dst.hasCs()) && (!self.hasCs() || dst.hasCs())
               {
                  // Java: if (this.hasCs() && dst.hasCs())
                  if self.hasCs() && dst.hasCs()
                  {
                     // Java: if (isStrict && !this.cs().equals(dst.cs()))
                     if isStrict && !self.cs().equals(dst.cs())
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !this.cs().equalsLax(dst.cs()))
                     if !isStrict && !self.cs().equalsLax(dst.cs())
                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: if ((this.hasExtLst() || !dst.hasExtLst()) && (!this.hasExtLst() || dst.hasExtLst()))
                  if (self.hasExtLst() || !dst.hasExtLst()) && (!self.hasExtLst() || dst.hasExtLst())
                  {
                     // Java: if (this.hasExtLst() && dst.hasExtLst())
                     if self.hasExtLst() && dst.hasExtLst()
                     {
                        // Java: if (isStrict && !this.extLst().equals(dst.extLst()))
                        if isStrict && !self.extLst().equals(dst.extLst())
                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: if (!isStrict && !this.extLst().equalsLax(dst.extLst()))
                        if !isStrict && !self.extLst().equalsLax(dst.extLst())
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
      {
         // Java: return false;
         return false;
      }
    }

    // Java: int hashCode()
    pub fn hashCode(&mut self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasLatin())
      if self.hasLatin()
      {
         // Java: ret += this.latin().hashCode();
         ret = ret.wrapping_add(self.latin().hashCode());
      }
      // Java: if (this.hasEa())
      if self.hasEa()
      {
         // Java: ret += this.ea().hashCode();
         ret = ret.wrapping_add(self.ea().hashCode());
      }
      // Java: if (this.hasCs())
      if self.hasCs()
      {
         // Java: ret += this.cs().hashCode();
         ret = ret.wrapping_add(self.cs().hashCode());
      }
      // Java: if (this.hasExtLst())
      if self.hasExtLst()
      {
         // Java: ret += this.extLst().hashCode();
         ret = ret.wrapping_add(self.extLst().hashCode());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.latin != null && this.latin.dirty())
      if self.latin.as_ref().map_or(false, |f| f.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.ea != null && this.ea.dirty())
      if self.ea.as_ref().map_or(false, |f| f.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.cs != null && this.cs.dirty())
      if self.cs.as_ref().map_or(false, |f| f.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
      {
         // Java: return this.extLst != null && this.extLst.dirty() ? true : this.map.dirty();
         if self.extLst.as_ref().map_or(false, |f| f.dirty()) { return true; }
         return self.map.dirty();
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
      let bin = self.bin_box.get();
      Factory::dup(bin.as_ref(), zone)
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
      let null_bin = Null::value1();
      self.map.load(null_bin.as_ref());
      // Java: this.box.clear();
      self.bin_box.clear();
      // Java: this.zone.leave();
      // (zone leave not needed in Rust)
      // Java: this.zone = KoZone.defaultZone();
      self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
      // Java: this.latin = null;
      self.latin = None;
      // Java: this.ea = null;
      self.ea = None;
      // Java: this.cs = null;
      self.cs = None;
      // Java: this.extLst = null;
      self.extLst = None;
    }

    // Java: void follow(FontCollection parent)
    pub fn follow(&mut self, parent: Option<Box<FontCollection>>) {
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

    // Java: FontCollection parent()
    pub fn parent(&self) -> &FontCollection {
      // Java: return this.parent;
      self.parent.as_ref().unwrap()
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

    // Java: TextFont latin()
    pub fn latin(&mut self) -> &mut TextFont {
      // Java: if (this.latin == null)
      if self.latin.is_none()
      {
         // Java: if (this.map.has(1))
         if self.map.has(1)
         {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: this.latin = (TextFont)KoBin.Decode.toAny(TextFont.class, bin, this.zone);
            self.latin = Some(Box::new(TextFont::TextFont_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasLatin())
         else if self.parent.is_some() && self.parent.as_ref().unwrap().hasLatin()
         {
            // Java: this.latin = this.parent.latin();
            todo!("parent delegation for latin");
         }
         // Java: else
         else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.latin = (TextFont)KoBin.Decode.toAny(TextFont.class, bin, this.zone);
            self.latin = Some(Box::new(TextFont::TextFont_from(bin.as_ref())));
         }
      }
      // Java: return this.latin;
      self.latin.as_mut().unwrap()
    }

    // Java: boolean hasLatin()
    pub fn hasLatin(&self) -> bool {
      // Java: return this.map.has(1) || this.latin != null && this.latin.dirty() || this.parent != null && this.parent.hasLatin();
      self.map.has(1)
        || self.latin.as_ref().map_or(false, |f| f.dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasLatin())
    }

    // Java: void deleteLatin()
    pub fn deleteLatin(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
      // Java: if (this.latin != null)
      if self.latin.is_some()
      {
         // Java: this.latin = null;
         self.latin = None;
      }
    }

    // Java: void writeLatin(TextFont value)
    pub fn writeLatin(&mut self, value: Option<&TextFont>) {
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
            // Java: if (this.latin != null)
            if self.latin.is_some()
            {
               // Java: this.latin = null;
               self.latin = None;
            }
         }
      }
    }

    // Java: void forceWriteLatin(TextFont value)
    pub fn forceWriteLatin(&mut self, value: Option<&TextFont>) {
      // Java: if (value != null)
      if let Some(v) = value
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         let bin = v.lastBin(self.zone);
         self.map.put(1, bin);
         // Java: if (this.latin != null)
         if self.latin.is_some()
         {
            // Java: this.latin = null;
            self.latin = None;
         }
      }
    }

    // Java: TextFont ea()
    pub fn ea(&mut self) -> &mut TextFont {
      // Java: if (this.ea == null)
      if self.ea.is_none()
      {
         // Java: if (this.map.has(2))
         if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: this.ea = (TextFont)KoBin.Decode.toAny(TextFont.class, bin, this.zone);
            self.ea = Some(Box::new(TextFont::TextFont_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasEa())
         else if self.parent.is_some() && self.parent.as_ref().unwrap().hasEa()
         {
            // Java: this.ea = this.parent.ea();
            todo!("parent delegation for ea");
         }
         // Java: else
         else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.ea = (TextFont)KoBin.Decode.toAny(TextFont.class, bin, this.zone);
            self.ea = Some(Box::new(TextFont::TextFont_from(bin.as_ref())));
         }
      }
      // Java: return this.ea;
      self.ea.as_mut().unwrap()
    }

    // Java: boolean hasEa()
    pub fn hasEa(&self) -> bool {
      // Java: return this.map.has(2) || this.ea != null && this.ea.dirty() || this.parent != null && this.parent.hasEa();
      self.map.has(2)
        || self.ea.as_ref().map_or(false, |f| f.dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasEa())
    }

    // Java: void deleteEa()
    pub fn deleteEa(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.ea != null)
      if self.ea.is_some()
      {
         // Java: this.ea = null;
         self.ea = None;
      }
    }

    // Java: void writeEa(TextFont value)
    pub fn writeEa(&mut self, value: Option<&TextFont>) {
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
            // Java: if (this.ea != null)
            if self.ea.is_some()
            {
               // Java: this.ea = null;
               self.ea = None;
            }
         }
      }
    }

    // Java: void forceWriteEa(TextFont value)
    pub fn forceWriteEa(&mut self, value: Option<&TextFont>) {
      // Java: if (value != null)
      if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         let bin = v.lastBin(self.zone);
         self.map.put(2, bin);
         // Java: if (this.ea != null)
         if self.ea.is_some()
         {
            // Java: this.ea = null;
            self.ea = None;
         }
      }
    }

    // Java: TextFont cs()
    pub fn cs(&mut self) -> &mut TextFont {
      // Java: if (this.cs == null)
      if self.cs.is_none()
      {
         // Java: if (this.map.has(3))
         if self.map.has(3)
         {
            // Java: KoBin bin = this.map.byKey(3);
            let bin = self.map.byKey(3);
            // Java: this.cs = (TextFont)KoBin.Decode.toAny(TextFont.class, bin, this.zone);
            self.cs = Some(Box::new(TextFont::TextFont_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasCs())
         else if self.parent.is_some() && self.parent.as_ref().unwrap().hasCs()
         {
            // Java: this.cs = this.parent.cs();
            todo!("parent delegation for cs");
         }
         // Java: else
         else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.cs = (TextFont)KoBin.Decode.toAny(TextFont.class, bin, this.zone);
            self.cs = Some(Box::new(TextFont::TextFont_from(bin.as_ref())));
         }
      }
      // Java: return this.cs;
      self.cs.as_mut().unwrap()
    }

    // Java: boolean hasCs()
    pub fn hasCs(&self) -> bool {
      // Java: return this.map.has(3) || this.cs != null && this.cs.dirty() || this.parent != null && this.parent.hasCs();
      self.map.has(3)
        || self.cs.as_ref().map_or(false, |f| f.dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasCs())
    }

    // Java: void deleteCs()
    pub fn deleteCs(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
      // Java: if (this.cs != null)
      if self.cs.is_some()
      {
         // Java: this.cs = null;
         self.cs = None;
      }
    }

    // Java: void writeCs(TextFont value)
    pub fn writeCs(&mut self, value: Option<&TextFont>) {
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
            // Java: if (this.cs != null)
            if self.cs.is_some()
            {
               // Java: this.cs = null;
               self.cs = None;
            }
         }
      }
    }

    // Java: void forceWriteCs(TextFont value)
    pub fn forceWriteCs(&mut self, value: Option<&TextFont>) {
      // Java: if (value != null)
      if let Some(v) = value
      {
         // Java: this.map.put(3, value.lastBin(this.zone));
         let bin = v.lastBin(self.zone);
         self.map.put(3, bin);
         // Java: if (this.cs != null)
         if self.cs.is_some()
         {
            // Java: this.cs = null;
            self.cs = None;
         }
      }
    }

    // Java: SupplementalFontList extLst()
    pub fn extLst(&mut self) -> &mut SupplementalFontList {
      // Java: if (this.extLst == null)
      if self.extLst.is_none()
      {
         // Java: if (this.map.has(4))
         if self.map.has(4)
         {
            // Java: KoBin bin = this.map.byKey(4);
            let bin = self.map.byKey(4);
            // Java: this.extLst = (SupplementalFontList)KoBin.Decode.toAny(SupplementalFontList.class, bin, this.zone);
            self.extLst = Some(Box::new(SupplementalFontList::SupplementalFontList_from(bin.as_ref(), self.zone)));
         }
         // Java: else if (this.parent != null && this.parent.hasExtLst())
         else if self.parent.is_some() && self.parent.as_ref().unwrap().hasExtLst()
         {
            // Java: this.extLst = this.parent.extLst();
            todo!("parent delegation for extLst");
         }
         // Java: else
         else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.extLst = (SupplementalFontList)KoBin.Decode.toAny(SupplementalFontList.class, bin, this.zone);
            self.extLst = Some(Box::new(SupplementalFontList::SupplementalFontList_from(bin.as_ref(), self.zone)));
         }
      }
      // Java: return this.extLst;
      self.extLst.as_mut().unwrap()
    }

    // Java: boolean hasExtLst()
    pub fn hasExtLst(&self) -> bool {
      // Java: return this.map.has(4) || this.extLst != null && this.extLst.dirty() || this.parent != null && this.parent.hasExtLst();
      self.map.has(4)
        || self.extLst.as_ref().map_or(false, |f| f.dirty())
        || self.parent.as_ref().map_or(false, |p| p.hasExtLst())
    }

    // Java: void deleteExtLst()
    pub fn deleteExtLst(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
      // Java: if (this.extLst != null)
      if self.extLst.is_some()
      {
         // Java: this.extLst = null;
         self.extLst = None;
      }
    }

    // Java: void writeExtLst(SupplementalFontList value)
    pub fn writeExtLst(&mut self, value: Option<&SupplementalFontList>) {
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
            // Java: if (this.extLst != null)
            if self.extLst.is_some()
            {
               // Java: this.extLst = null;
               self.extLst = None;
            }
         }
      }
    }

    // Java: void forceWriteExtLst(SupplementalFontList value)
    pub fn forceWriteExtLst(&mut self, value: Option<&SupplementalFontList>) {
      // Java: if (value != null)
      if let Some(v) = value
      {
         // Java: this.map.put(4, value.lastBin(this.zone));
         let bin = v.lastBin(self.zone);
         self.map.put(4, bin);
         // Java: if (this.extLst != null)
         if self.extLst.is_some()
         {
            // Java: this.extLst = null;
            self.extLst = None;
         }
      }
    }

}

impl KoAr for FontCollection {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for FontCollection { fn close(&mut self) { self.close(); } }

// Java: public class SupplementalFont
impl SupplementalFont {
    // Java: static protected static <Value1 extends SupplementalFont> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         let mut inst = Self::SupplementalFont_from(bin);
         // Java: inst.zone = zone;
         inst.zone = zone;
         // Java: inst.load(bin);
         // (load already done in SupplementalFont_from)
         // Java: return inst;
         inst
    }

    // Java: static SupplementalFont create()
    pub fn create() -> Self {
         // Java: return initialize(SupplementalFont.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static SupplementalFont open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(SupplementalFont.class, bin);
         Self::initialize(bin)
    }

    // Java: SupplementalFont()
    pub fn new() -> Self {
         // Java: this.box = new KpBinBox();
         // Java: this.map = new KpMap();
         Self {
           zone: KoZone::defaultZone(),
           bin_box: KpBinBox::new(),
           map: KpMap::new(),
           parent: None,
         }
    }

    // Java: SupplementalFont(KoBin bin)
    pub fn SupplementalFont_from(bin: &dyn KoBinTrait) -> Self {
         // Java: this.box = new KpBinBox(bin);
         // Java: this.map = new KpMap(bin);
         Self {
           zone: KoZone::defaultZone(),
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
         self.loadBin(bin, self.zone);
    }

    // Java: void load(SupplementalFont value)
    pub fn load_from(&mut self, value: Option<&SupplementalFont>) {
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
         // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
         if self.map.dirty() {
           let saved = self.map.save(zone);
           self.bin_box.set(Some(saved.as_ref()));
           self.bin_box.get()
         } else {
           self.bin_box.get()
         }
    }

    // Java: boolean equalsLax(SupplementalFont o)
    pub fn equalsLax(&self, o: &SupplementalFont) -> bool {
         // Java: return this.equals(o, false);
         self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &SupplementalFont) -> bool {
         // Java: return this.equals(o, true);
         self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &SupplementalFont, _isStrict: bool) -> bool {
         // Java: if (null != o && o instanceof SupplementalFont)
         {
            // Java: SupplementalFont dst = (SupplementalFont)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst)
            {
               // Java: return true;
               return true;
            }
            // Java: else if ((this.hasScript() || !dst.hasScript()) && (!this.hasScript() || dst.hasScript()))
            else if (self.hasScript() || !dst.hasScript()) && (!self.hasScript() || dst.hasScript())
            {
               // Java: if (this.hasScript() && dst.hasScript() && !this.script().equals(dst.script()))
               if self.hasScript() && dst.hasScript() && self.script() != dst.script()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasTypeface() || !dst.hasTypeface()) && (!this.hasTypeface() || dst.hasTypeface()))
               else if (self.hasTypeface() || !dst.hasTypeface()) && (!self.hasTypeface() || dst.hasTypeface())
               {
                  // Java: return !this.hasTypeface() || !dst.hasTypeface() || this.typeface().equals(dst.typeface());
                  return !self.hasTypeface() || !dst.hasTypeface() || self.typeface() == dst.typeface();
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
         {
            // Java: return false;
            return false;
         }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
         // Java: int ret = 0;
         let mut ret: i32 = 0;
         // Java: if (this.hasScript())
         if self.hasScript()
         {
            // Java: ret += String.valueOf(this.script()).hashCode();
            ret = ret.wrapping_add(java_string_hash(&self.script()));
         }
         // Java: if (this.hasTypeface())
         if self.hasTypeface()
         {
            // Java: ret += String.valueOf(this.typeface()).hashCode();
            ret = ret.wrapping_add(java_string_hash(&self.typeface()));
         }
         // Java: return ret;
         ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
         // Java: return this.map.dirty();
         self.map.dirty()
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
         let bin = self.bin_box.get();
         Factory::dup(bin.as_ref(), zone)
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
         let null_bin = Null::value1();
         self.map.load(null_bin.as_ref());
         // Java: this.box.clear();
         self.bin_box.clear();
         // Java: this.zone.leave();
         // (zone leave not needed in Rust)
         // Java: this.zone = KoZone.defaultZone();
         self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    pub fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    pub fn cleanCache(&mut self) {
    }

    // Java: void follow(SupplementalFont parent)
    pub fn follow(&mut self, parent: Option<Box<SupplementalFont>>) {
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

    // Java: SupplementalFont parent()
    pub fn parent(&self) -> &SupplementalFont {
         // Java: return this.parent;
         self.parent.as_ref().unwrap()
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

    // Java: String script()
    pub fn script(&self) -> String {
         // Java: if (!this.map.has(1))
         if !self.map.has(1)
         {
            // Java: return this.parent == null ? "" : this.parent.script();
            return self.parent.as_ref().map_or(String::new(), |p| p.script());
         }
         // Java: else
         else
         {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toText(bin);
            Decode::toText(bin.as_ref())
         }
    }

    // Java: boolean hasScript()
    pub fn hasScript(&self) -> bool {
         // Java: return this.map.has(1) || this.parent != null && this.parent.hasScript();
         self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasScript())
    }

    // Java: void deleteScript()
    pub fn deleteScript(&mut self) {
         // Java: this.map.put(1, (KoBin)null);
         self.map.clear_key(1);
    }

    // Java: void putScript(String value)
    pub fn putScript(&mut self, value: &str) {
         // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
         self.map.put(1, Encode::from_string(value, self.zone));
    }

    // Java: String typeface()
    pub fn typeface(&self) -> String {
         // Java: if (!this.map.has(2))
         if !self.map.has(2)
         {
            // Java: return this.parent == null ? "" : this.parent.typeface();
            return self.parent.as_ref().map_or(String::new(), |p| p.typeface());
         }
         // Java: else
         else
         {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: return KoBin.Decode.toText(bin);
            Decode::toText(bin.as_ref())
         }
    }

    // Java: boolean hasTypeface()
    pub fn hasTypeface(&self) -> bool {
         // Java: return this.map.has(2) || this.parent != null && this.parent.hasTypeface();
         self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasTypeface())
    }

    // Java: void deleteTypeface()
    pub fn deleteTypeface(&mut self) {
         // Java: this.map.put(2, (KoBin)null);
         self.map.clear_key(2);
    }

    // Java: void putTypeface(String value)
    pub fn putTypeface(&mut self, value: &str) {
         // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
         self.map.put(2, Encode::from_string(value, self.zone));
    }

}

impl KoAr for SupplementalFont {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for SupplementalFont { fn close(&mut self) { self.close(); } }

// Java: public class SupplementalFontList
impl SupplementalFontList {
    // Java: SupplementalFontList()
    pub fn new() -> Self {
         // Java: this.box = new KpBinBox();
         // Java: this.list = new KpList(this.zone);
         let zone = KoZone::defaultZone();
         Self {
           bin_box: KpBinBox::new(),
           list: KpList::new(zone),
           zone,
         }
    }

    // Java: SupplementalFontList(KoBin bin)
    pub fn SupplementalFontList_from(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
         // Java: this.box = new KpBinBox(bin);
         // Java: this.list = new KpList(bin, this.zone);
         Self {
           bin_box: KpBinBox::KpBinBox_from(Some(bin)),
           list: KpList::KpList_from(bin, zone),
           zone,
         }
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
         self.loadBin(bin, self.zone);
    }

    // Java: void load(SupplementalFontList value)
    pub fn load_from(&mut self, value: Option<&SupplementalFontList>) {
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
         if self.list.dirty() {
           let saved = self.list.save(zone);
           self.bin_box.set(Some(saved.as_ref()));
           self.bin_box.get()
         } else {
           self.bin_box.get()
         }
    }

    // Java: boolean equalsLax(SupplementalFontList o)
    pub fn equalsLax(&mut self, o: &mut SupplementalFontList) -> bool {
         // Java: return this.equals(o, false);
         self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&mut self, o: &mut SupplementalFontList) -> bool {
         // Java: return this.equals(o, true);
         self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&mut self, o: &mut SupplementalFontList, isStrict: bool) -> bool {
         // Java: if (null == o)
         if false
         {
            // Java: return false;
            return false;
         }
         // Java: else
         else
         {
            // Java: SupplementalFontList dst = (SupplementalFontList)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self as *const _, dst as *const _)
            {
               // Java: return true;
               return true;
            }
            // Java: else
            else
            {
               // Java: ArrayList<SupplementalFont> dstList = new ArrayList();
               // Java: dst.read(dstList);
               let dstList = dst.read();
               // Java: ArrayList<SupplementalFont> srcList = new ArrayList();
               // Java: this.read(srcList);
               let srcList = self.read();
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
                     // Java: SupplementalFont dstClrTrans = (SupplementalFont)dstList.get(i);
                     let dstItem = &dstList[i];
                     // Java: SupplementalFont srcClrTrans = (SupplementalFont)srcList.get(i);
                     let srcItem = &srcList[i];
                     // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                     if isStrict && !dstItem.equals(srcItem)
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                     if !isStrict && !dstItem.equalsLax(srcItem)
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
         self.saveBin(self.zone);
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
         let bin = self.bin_box.get();
         Factory::dup(bin.as_ref(), zone)
    }

    // Java: void close()
    pub fn close(&mut self) {
         // Java: this.list.load(KoBin.Null.value1(), this.zone);
         let null_bin = Null::value1();
         self.list.load(null_bin.as_ref(), self.zone);
         // Java: this.zone.leave();
         // (zone leave not needed in Rust)
         // Java: this.zone = KoZone.defaultZone();
         self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends SupplementalFontList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
         // Java: KoZone zone = KoZone.enter();
         let zone = KoZone::enter();
         // Java: Value1 inst = (Value1)(zone.allocate(clazz));
         let mut inst = Self::SupplementalFontList_from(bin, zone);
         // Java: inst.zone = zone;
         inst.zone = zone;
         // Java: inst.load(bin);
         // (load already done in SupplementalFontList_from)
         // Java: return inst;
         inst
    }

    // Java: static SupplementalFontList create()
    pub fn create() -> Self {
         // Java: return initialize(SupplementalFontList.class, KoBin.Null.empty());
         let empty = Null::empty();
         Self::initialize(empty.as_ref())
    }

    // Java: static SupplementalFontList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
         // Java: return initialize(SupplementalFontList.class, bin);
         Self::initialize(bin)
    }

    // Java: void read(Collection<SupplementalFont> valueLst)
    pub fn read(&self) -> Vec<SupplementalFont> {
         // Java: valueLst.clear();
         let mut result = Vec::new();
         // Java: int size = this.list.size();
         let size = self.list.size();
         // Java: for(int i = 0; i < size; ++i)
         for i in 0..size
         {
            // Java: valueLst.add((SupplementalFont)KoBin.Decode.toAny(SupplementalFont.class, this.list.get(i), this.zone));
            let bin = self.list.get(i);
            result.push(SupplementalFont::SupplementalFont_from(bin));
         }
         result
    }

    // Java: void write(Iterable<SupplementalFont> valueLst)
    pub fn write(&mut self, valueLst: &[SupplementalFont]) {
         // Java: this.list.clear();
         self.list.clear();
         // Java: for(SupplementalFont i : valueLst)
         for item in valueLst
         {
            // Java: KoBin bin = i.lastBin(this.zone);
            let bin = item.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
            {
               // Java: this.list.add(bin);
               self.list.add(bin);
            }
         }
    }

    // Java: void forceWrite(Iterable<SupplementalFont> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[SupplementalFont]) {
         // Java: this.list.clear();
         self.list.clear();
         // Java: for(SupplementalFont i : valueLst)
         for item in valueLst
         {
            // Java: this.list.add(i.lastBin(this.zone));
            let bin = item.lastBin(self.zone);
            self.list.add(bin);
         }
    }

}

impl KoAr for SupplementalFontList {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for SupplementalFontList { fn close(&mut self) { self.close(); } }

fn java_string_hash(s: &str) -> i32 {
    let mut h: i32 = 0;
    for c in s.encode_utf16() {
        h = h.wrapping_mul(31).wrapping_add(c as i32);
    }
    h
}
