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

// Java: public class TextFont implements KoAr, KoClosable
pub struct TextFont {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: TextFont parent
    pub(crate) parent: Option<Box<TextFont>>,
}

// Java: public class TextFont implements KoAr, KoClosable
impl TextFont {
    // Java: static protected static <Value1 extends TextFont> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::TextFont_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static TextFont create()
    pub fn create() -> Self {
      // Java: return initialize(TextFont.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static TextFont open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(TextFont.class, bin);
      Self::initialize(bin)
    }

    // Java: TextFont()
    pub fn new() -> Self {
      Self {
      // Java: this.box = new KpBinBox();
      bin_box: KpBinBox::new(),
      // Java: this.map = new KpMap();
      map: KpMap::new(),
      zone: KoZone::defaultZone(), parent: None,
      }
    }

    // Java: TextFont(KoBin bin)
    pub fn TextFont_from(bin: &dyn KoBinTrait) -> Self {
      Self {
      // Java: this.box = new KpBinBox(bin);
      bin_box: KpBinBox::KpBinBox_from(Some(bin)),
      // Java: this.map = new KpMap(bin);
      map: KpMap::KpMap_from(bin),
      zone: KoZone::defaultZone(), parent: None,
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

    // Java: void load(TextFont value)
    pub fn load_value(&mut self, value: Option<&TextFont>) {
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
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() {
         let saved = self.map.save(zone);
         self.bin_box.set(Some(saved.as_ref()));
         self.bin_box.get()
      } else {
         self.bin_box.get()
      }
    }

    // Java: boolean equalsLax(TextFont o)
    pub fn equalsLax(&self, o: &TextFont) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TextFont) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &TextFont, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof TextFont)
        if true
      {
        let dst = o;
         // Java: TextFont dst = (TextFont)o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasTypeface() || !dst.hasTypeface()) && (!this.hasTypeface() || dst.hasTypeface()))
            else if (self.hasTypeface() || !dst.hasTypeface()) && (!self.hasTypeface() || dst.hasTypeface())
         {
            // Java: if (this.hasTypeface() && dst.hasTypeface() && !this.typeface().equals(dst.typeface()))
                if self.hasTypeface() && dst.hasTypeface() && self.get_typeface() != dst.get_typeface()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasPanose() || !dst.hasPanose()) && (!this.hasPanose() || dst.hasPanose()))
                else if (self.hasPanose() || !dst.hasPanose()) && (!self.hasPanose() || dst.hasPanose())
            {
               // Java: if (this.hasPanose() && dst.hasPanose() && !this.panose().equals(dst.panose()))
                    if self.hasPanose() && dst.hasPanose() && self.get_panose() != dst.get_panose()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasPitchFamily() || !dst.hasPitchFamily()) && (!this.hasPitchFamily() || dst.hasPitchFamily()))
                    else if (self.hasPitchFamily() || !dst.hasPitchFamily()) && (!self.hasPitchFamily() || dst.hasPitchFamily())
               {
                  // Java: if (this.hasPitchFamily() && dst.hasPitchFamily() && this.pitchFamily() != dst.pitchFamily())
                        if self.hasPitchFamily() && dst.hasPitchFamily() && self.get_pitchFamily() != dst.get_pitchFamily()
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasCharset() || !dst.hasCharset()) && (!this.hasCharset() || dst.hasCharset()))
                        else if (self.hasCharset() || !dst.hasCharset()) && (!self.hasCharset() || dst.hasCharset())
                  {
                     // Java: return !this.hasCharset() || !dst.hasCharset() || this.charset() == dst.charset();
                     return !self.hasCharset() || !dst.hasCharset() || self.get_charset() == dst.get_charset();
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
      // Java: if (this.hasTypeface())
        if self.hasTypeface()
      {
         // Java: ret += String.valueOf(this.typeface()).hashCode();
         ret = ret.wrapping_add(self.get_typeface().len() as i32);
      }
      // Java: if (this.hasPanose())
        if self.hasPanose()
      {
         // Java: ret += String.valueOf(this.panose()).hashCode();
         ret = ret.wrapping_add(self.get_panose().len() as i32);
      }
      // Java: if (this.hasPitchFamily())
        if self.hasPitchFamily()
      {
         // Java: ret += Integer.valueOf(this.pitchFamily()).hashCode();
         ret = ret.wrapping_add(self.get_pitchFamily());
      }
      // Java: if (this.hasCharset())
        if self.hasCharset()
      {
         // Java: ret += Integer.valueOf(this.charset()).hashCode();
         ret = ret.wrapping_add(self.get_charset());
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
    }

    // Java: void follow(TextFont parent)
    pub fn follow(&mut self, parent: Option<Box<TextFont>>) {
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

    // Java: TextFont parent()
    pub fn get_parent(&self) -> Option<&TextFont> {
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

    // Java: String typeface()
    pub fn get_typeface(&self) -> String {
      // Java: if (!this.map.has(1))
        if !self.map.has(1)
      {
         // Java: return this.parent == null ? "" : this.parent.typeface();
         return self.parent.as_ref().map_or(String::new(), |p| p.get_typeface());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(1);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasTypeface()
    pub fn hasTypeface(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasTypeface();
      self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasTypeface())
    }

    // Java: void deleteTypeface()
    pub fn deleteTypeface(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
    }

    // Java: void putTypeface(String value)
    pub fn putTypeface(&mut self, value: String) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(1, Encode::from_string(&value, self.zone));
    }

    // Java: String panose()
    pub fn get_panose(&self) -> String {
      // Java: if (!this.map.has(2))
        if !self.map.has(2)
      {
         // Java: return this.parent == null ? "" : this.parent.panose();
         return self.parent.as_ref().map_or(String::new(), |p| p.get_panose());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(2);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasPanose()
    pub fn hasPanose(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasPanose();
      self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasPanose())
    }

    // Java: void deletePanose()
    pub fn deletePanose(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
    }

    // Java: void putPanose(String value)
    pub fn putPanose(&mut self, value: String) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(2, Encode::from_string(&value, self.zone));
    }

    // Java: int pitchFamily()
    pub fn get_pitchFamily(&self) -> i32 {
      // Java: if (!this.map.has(3))
        if !self.map.has(3)
      {
         // Java: return this.parent == null ? 0 : this.parent.pitchFamily();
         return self.parent.as_ref().map_or(0, |p| p.get_pitchFamily());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(3);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasPitchFamily()
    pub fn hasPitchFamily(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasPitchFamily();
      self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasPitchFamily())
    }

    // Java: void deletePitchFamily()
    pub fn deletePitchFamily(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
    }

    // Java: void putPitchFamily(int value)
    pub fn putPitchFamily(&mut self, value: i32) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(3, Encode::from_int(value, self.zone));
    }

    // Java: int charset()
    pub fn get_charset(&self) -> i32 {
      // Java: if (!this.map.has(4))
        if !self.map.has(4)
      {
         // Java: return this.parent == null ? 0 : this.parent.charset();
         return self.parent.as_ref().map_or(0, |p| p.get_charset());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(4);
         let bin = self.map.byKey(4);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasCharset()
    pub fn hasCharset(&self) -> bool {
      // Java: return this.map.has(4) || this.parent != null && this.parent.hasCharset();
      self.map.has(4) || self.parent.as_ref().map_or(false, |p| p.hasCharset())
    }

    // Java: void deleteCharset()
    pub fn deleteCharset(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
    }

    // Java: void putCharset(int value)
    pub fn putCharset(&mut self, value: i32) {
      // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
      self.map.put(4, Encode::from_int(value, self.zone));
    }

}

impl KoAr for TextFont {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { TextFont::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { TextFont::lastBin(self, var1) }
}

impl KoClosable for TextFont {
    fn close(&mut self) { TextFont::close(self); }
}
