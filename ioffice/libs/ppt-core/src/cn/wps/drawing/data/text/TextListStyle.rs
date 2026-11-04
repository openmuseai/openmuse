#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.text.TextParagraphProperties
use crate::cn::wps::drawing::data::text::TextParagraphProperties::TextParagraphProperties;
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

// Java: public class TextListStyle implements KoAr, KoClosable
pub struct TextListStyle {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: TextParagraphProperties defpPr
    pub(crate) defpPr: Option<Box<TextParagraphProperties>>,
    // Java field: TextParagraphProperties lvl1pPr
    pub(crate) lvl1pPr: Option<Box<TextParagraphProperties>>,
    // Java field: TextParagraphProperties lvl2pPr
    pub(crate) lvl2pPr: Option<Box<TextParagraphProperties>>,
    // Java field: TextParagraphProperties lvl3pPr
    pub(crate) lvl3pPr: Option<Box<TextParagraphProperties>>,
    // Java field: TextParagraphProperties lvl4pPr
    pub(crate) lvl4pPr: Option<Box<TextParagraphProperties>>,
    // Java field: TextParagraphProperties lvl5pPr
    pub(crate) lvl5pPr: Option<Box<TextParagraphProperties>>,
    // Java field: TextParagraphProperties lvl6pPr
    pub(crate) lvl6pPr: Option<Box<TextParagraphProperties>>,
    // Java field: TextParagraphProperties lvl7pPr
    pub(crate) lvl7pPr: Option<Box<TextParagraphProperties>>,
    // Java field: TextParagraphProperties lvl8pPr
    pub(crate) lvl8pPr: Option<Box<TextParagraphProperties>>,
    // Java field: TextParagraphProperties lvl9pPr
    pub(crate) lvl9pPr: Option<Box<TextParagraphProperties>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: TextListStyle parent
    pub(crate) parent: Option<Box<TextListStyle>>,
}

// Java: public class TextListStyle implements KoAr, KoClosable
impl TextListStyle {
    // Java: static protected static <Value1 extends TextListStyle> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::TextListStyle_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static TextListStyle create()
    pub fn create() -> Self {
      // Java: return initialize(TextListStyle.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static TextListStyle open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(TextListStyle.class, bin);
      Self::initialize(bin)
    }

    // Java: TextListStyle()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Self { bin_box: KpBinBox::new(), map: KpMap::new(), zone: KoZone::defaultZone(), parent: None,
        defpPr: None, lvl1pPr: None, lvl2pPr: None, lvl3pPr: None, lvl4pPr: None, lvl5pPr: None, lvl6pPr: None, lvl7pPr: None, lvl8pPr: None, lvl9pPr: None }
    }

    // Java: TextListStyle(KoBin bin)
    pub fn TextListStyle_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Self { bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), zone: KoZone::defaultZone(), parent: None,
        defpPr: None, lvl1pPr: None, lvl2pPr: None, lvl3pPr: None, lvl4pPr: None, lvl5pPr: None, lvl6pPr: None, lvl7pPr: None, lvl8pPr: None, lvl9pPr: None }
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

    // Java: void load(TextListStyle value)
    pub fn load_value(&mut self, value: Option<&TextListStyle>) {
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
      // Java: if (this.defpPr != null && this.defpPr.dirty())
        if self.defpPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(1, this.defpPr.saveBin(zone));
         let bin = self.defpPr.as_mut().unwrap().saveBin(zone);
         self.map.put(1, bin);
      }

      // Java: if (this.lvl1pPr != null && this.lvl1pPr.dirty())
        if self.lvl1pPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(2, this.lvl1pPr.saveBin(zone));
         let bin = self.lvl1pPr.as_mut().unwrap().saveBin(zone);
         self.map.put(2, bin);
      }

      // Java: if (this.lvl2pPr != null && this.lvl2pPr.dirty())
        if self.lvl2pPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(3, this.lvl2pPr.saveBin(zone));
         let bin = self.lvl2pPr.as_mut().unwrap().saveBin(zone);
         self.map.put(3, bin);
      }

      // Java: if (this.lvl3pPr != null && this.lvl3pPr.dirty())
        if self.lvl3pPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(4, this.lvl3pPr.saveBin(zone));
         let bin = self.lvl3pPr.as_mut().unwrap().saveBin(zone);
         self.map.put(4, bin);
      }

      // Java: if (this.lvl4pPr != null && this.lvl4pPr.dirty())
        if self.lvl4pPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(5, this.lvl4pPr.saveBin(zone));
         let bin = self.lvl4pPr.as_mut().unwrap().saveBin(zone);
         self.map.put(5, bin);
      }

      // Java: if (this.lvl5pPr != null && this.lvl5pPr.dirty())
        if self.lvl5pPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(6, this.lvl5pPr.saveBin(zone));
         let bin = self.lvl5pPr.as_mut().unwrap().saveBin(zone);
         self.map.put(6, bin);
      }

      // Java: if (this.lvl6pPr != null && this.lvl6pPr.dirty())
        if self.lvl6pPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(7, this.lvl6pPr.saveBin(zone));
         let bin = self.lvl6pPr.as_mut().unwrap().saveBin(zone);
         self.map.put(7, bin);
      }

      // Java: if (this.lvl7pPr != null && this.lvl7pPr.dirty())
        if self.lvl7pPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(8, this.lvl7pPr.saveBin(zone));
         let bin = self.lvl7pPr.as_mut().unwrap().saveBin(zone);
         self.map.put(8, bin);
      }

      // Java: if (this.lvl8pPr != null && this.lvl8pPr.dirty())
        if self.lvl8pPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(9, this.lvl8pPr.saveBin(zone));
         let bin = self.lvl8pPr.as_mut().unwrap().saveBin(zone);
         self.map.put(9, bin);
      }

      // Java: if (this.lvl9pPr != null && this.lvl9pPr.dirty())
        if self.lvl9pPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(10, this.lvl9pPr.saveBin(zone));
         let bin = self.lvl9pPr.as_mut().unwrap().saveBin(zone);
         self.map.put(10, bin);
      }

      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(TextListStyle o)
    pub fn equalsLax(&self, o: &TextListStyle) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TextListStyle) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &TextListStyle, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof TextListStyle)
        if true
      {
         // Java: TextListStyle dst = (TextListStyle)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasDefpPr() || !dst.hasDefpPr()) && (!this.hasDefpPr() || dst.hasDefpPr()))
            else if (self.hasDefpPr() || !dst.hasDefpPr()) && (!self.hasDefpPr() || dst.hasDefpPr())
         {
            // Java: if (this.hasDefpPr() && dst.hasDefpPr())
               if self.hasDefpPr() && dst.hasDefpPr()
            {
               // Java: if (isStrict && !this.defpPr().equals(dst.defpPr()))
                  if isStrict
               {
                  // Java: return false;
               }

               // Java: if (!isStrict && !this.defpPr().equalsLax(dst.defpPr()))
                  if !isStrict
               {
                  // Java: return false;
               }
            }

            // Java: if ((this.hasLvl1pPr() || !dst.hasLvl1pPr()) && (!this.hasLvl1pPr() || dst.hasLvl1pPr()))
               if (self.hasLvl1pPr() || !dst.hasLvl1pPr()) && (!self.hasLvl1pPr() || dst.hasLvl1pPr())
            {
               // Java: if (this.hasLvl1pPr() && dst.hasLvl1pPr())
                  if self.hasLvl1pPr() && dst.hasLvl1pPr()
               {
                  // Java: if (isStrict && !this.lvl1pPr().equals(dst.lvl1pPr()))
                     if isStrict
                  {
                     // Java: return false;
                  }

                  // Java: if (!isStrict && !this.lvl1pPr().equalsLax(dst.lvl1pPr()))
                     if !isStrict
                  {
                     // Java: return false;
                  }
               }

               // Java: if ((this.hasLvl2pPr() || !dst.hasLvl2pPr()) && (!this.hasLvl2pPr() || dst.hasLvl2pPr()))
                  if (self.hasLvl2pPr() || !dst.hasLvl2pPr()) && (!self.hasLvl2pPr() || dst.hasLvl2pPr())
               {
                  // Java: if (this.hasLvl2pPr() && dst.hasLvl2pPr())
                     if self.hasLvl2pPr() && dst.hasLvl2pPr()
                  {
                     // Java: if (isStrict && !this.lvl2pPr().equals(dst.lvl2pPr()))
                        if isStrict
                     {
                        // Java: return false;
                     }

                     // Java: if (!isStrict && !this.lvl2pPr().equalsLax(dst.lvl2pPr()))
                        if !isStrict
                     {
                        // Java: return false;
                     }
                  }

                  // Java: if ((this.hasLvl3pPr() || !dst.hasLvl3pPr()) && (!this.hasLvl3pPr() || dst.hasLvl3pPr()))
                     if (self.hasLvl3pPr() || !dst.hasLvl3pPr()) && (!self.hasLvl3pPr() || dst.hasLvl3pPr())
                  {
                     // Java: if (this.hasLvl3pPr() && dst.hasLvl3pPr())
                        if self.hasLvl3pPr() && dst.hasLvl3pPr()
                     {
                        // Java: if (isStrict && !this.lvl3pPr().equals(dst.lvl3pPr()))
                           if isStrict
                        {
                           // Java: return false;
                        }

                        // Java: if (!isStrict && !this.lvl3pPr().equalsLax(dst.lvl3pPr()))
                           if !isStrict
                        {
                           // Java: return false;
                        }
                     }

                     // Java: if ((this.hasLvl4pPr() || !dst.hasLvl4pPr()) && (!this.hasLvl4pPr() || dst.hasLvl4pPr()))
                        if (self.hasLvl4pPr() || !dst.hasLvl4pPr()) && (!self.hasLvl4pPr() || dst.hasLvl4pPr())
                     {
                        // Java: if (this.hasLvl4pPr() && dst.hasLvl4pPr())
                           if self.hasLvl4pPr() && dst.hasLvl4pPr()
                        {
                           // Java: if (isStrict && !this.lvl4pPr().equals(dst.lvl4pPr()))
                              if isStrict
                           {
                              // Java: return false;
                           }

                           // Java: if (!isStrict && !this.lvl4pPr().equalsLax(dst.lvl4pPr()))
                              if !isStrict
                           {
                              // Java: return false;
                           }
                        }

                        // Java: if ((this.hasLvl5pPr() || !dst.hasLvl5pPr()) && (!this.hasLvl5pPr() || dst.hasLvl5pPr()))
                           if (self.hasLvl5pPr() || !dst.hasLvl5pPr()) && (!self.hasLvl5pPr() || dst.hasLvl5pPr())
                        {
                           // Java: if (this.hasLvl5pPr() && dst.hasLvl5pPr())
                              if self.hasLvl5pPr() && dst.hasLvl5pPr()
                           {
                              // Java: if (isStrict && !this.lvl5pPr().equals(dst.lvl5pPr()))
                                 if isStrict
                              {
                                 // Java: return false;
                              }

                              // Java: if (!isStrict && !this.lvl5pPr().equalsLax(dst.lvl5pPr()))
                                 if !isStrict
                              {
                                 // Java: return false;
                              }
                           }

                           // Java: if ((this.hasLvl6pPr() || !dst.hasLvl6pPr()) && (!this.hasLvl6pPr() || dst.hasLvl6pPr()))
                              if (self.hasLvl6pPr() || !dst.hasLvl6pPr()) && (!self.hasLvl6pPr() || dst.hasLvl6pPr())
                           {
                              // Java: if (this.hasLvl6pPr() && dst.hasLvl6pPr())
                                 if self.hasLvl6pPr() && dst.hasLvl6pPr()
                              {
                                 // Java: if (isStrict && !this.lvl6pPr().equals(dst.lvl6pPr()))
                                    if isStrict
                                 {
                                    // Java: return false;
                                 }

                                 // Java: if (!isStrict && !this.lvl6pPr().equalsLax(dst.lvl6pPr()))
                                    if !isStrict
                                 {
                                    // Java: return false;
                                 }
                              }

                              // Java: if ((this.hasLvl7pPr() || !dst.hasLvl7pPr()) && (!this.hasLvl7pPr() || dst.hasLvl7pPr()))
                                 if (self.hasLvl7pPr() || !dst.hasLvl7pPr()) && (!self.hasLvl7pPr() || dst.hasLvl7pPr())
                              {
                                 // Java: if (this.hasLvl7pPr() && dst.hasLvl7pPr())
                                    if self.hasLvl7pPr() && dst.hasLvl7pPr()
                                 {
                                    // Java: if (isStrict && !this.lvl7pPr().equals(dst.lvl7pPr()))
                                       if isStrict
                                    {
                                       // Java: return false;
                                    }

                                    // Java: if (!isStrict && !this.lvl7pPr().equalsLax(dst.lvl7pPr()))
                                       if !isStrict
                                    {
                                       // Java: return false;
                                    }
                                 }

                                 // Java: if ((this.hasLvl8pPr() || !dst.hasLvl8pPr()) && (!this.hasLvl8pPr() || dst.hasLvl8pPr()))
                                    if (self.hasLvl8pPr() || !dst.hasLvl8pPr()) && (!self.hasLvl8pPr() || dst.hasLvl8pPr())
                                 {
                                    // Java: if (this.hasLvl8pPr() && dst.hasLvl8pPr())
                                       if self.hasLvl8pPr() && dst.hasLvl8pPr()
                                    {
                                       // Java: if (isStrict && !this.lvl8pPr().equals(dst.lvl8pPr()))
                                          if isStrict
                                       {
                                          // Java: return false;
                                       }

                                       // Java: if (!isStrict && !this.lvl8pPr().equalsLax(dst.lvl8pPr()))
                                          if !isStrict
                                       {
                                          // Java: return false;
                                       }
                                    }

                                    // Java: if ((this.hasLvl9pPr() || !dst.hasLvl9pPr()) && (!this.hasLvl9pPr() || dst.hasLvl9pPr()))
                                       if (self.hasLvl9pPr() || !dst.hasLvl9pPr()) && (!self.hasLvl9pPr() || dst.hasLvl9pPr())
                                    {
                                       // Java: if (this.hasLvl9pPr() && dst.hasLvl9pPr())
                                          if self.hasLvl9pPr() && dst.hasLvl9pPr()
                                       {
                                          // Java: if (isStrict && !this.lvl9pPr().equals(dst.lvl9pPr()))
                                             if isStrict
                                          {
                                             // Java: return false;
                                          }

                                          // Java: if (!isStrict && !this.lvl9pPr().equalsLax(dst.lvl9pPr()))
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
      // Java: return false;
      return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasDefpPr())
        if self.hasDefpPr()
      {
         // Java: ret += this.defpPr().hashCode();
         todo!()
      }
      // Java: if (this.hasLvl1pPr())
        if self.hasLvl1pPr()
      {
         // Java: ret += this.lvl1pPr().hashCode();
         todo!()
      }
      // Java: if (this.hasLvl2pPr())
        if self.hasLvl2pPr()
      {
         // Java: ret += this.lvl2pPr().hashCode();
         todo!()
      }
      // Java: if (this.hasLvl3pPr())
        if self.hasLvl3pPr()
      {
         // Java: ret += this.lvl3pPr().hashCode();
         todo!()
      }
      // Java: if (this.hasLvl4pPr())
        if self.hasLvl4pPr()
      {
         // Java: ret += this.lvl4pPr().hashCode();
         todo!()
      }
      // Java: if (this.hasLvl5pPr())
        if self.hasLvl5pPr()
      {
         // Java: ret += this.lvl5pPr().hashCode();
         todo!()
      }
      // Java: if (this.hasLvl6pPr())
        if self.hasLvl6pPr()
      {
         // Java: ret += this.lvl6pPr().hashCode();
         todo!()
      }
      // Java: if (this.hasLvl7pPr())
        if self.hasLvl7pPr()
      {
         // Java: ret += this.lvl7pPr().hashCode();
         todo!()
      }
      // Java: if (this.hasLvl8pPr())
        if self.hasLvl8pPr()
      {
         // Java: ret += this.lvl8pPr().hashCode();
         todo!()
      }
      // Java: if (this.hasLvl9pPr())
        if self.hasLvl9pPr()
      {
         // Java: ret += this.lvl9pPr().hashCode();
         todo!()
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.defpPr != null && this.defpPr.dirty())
        if self.defpPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.lvl1pPr != null && this.lvl1pPr.dirty())
        else if self.lvl1pPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.lvl2pPr != null && this.lvl2pPr.dirty())
        else if self.lvl2pPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.lvl3pPr != null && this.lvl3pPr.dirty())
        else if self.lvl3pPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.lvl4pPr != null && this.lvl4pPr.dirty())
        else if self.lvl4pPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.lvl5pPr != null && this.lvl5pPr.dirty())
        else if self.lvl5pPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.lvl6pPr != null && this.lvl6pPr.dirty())
        else if self.lvl6pPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.lvl7pPr != null && this.lvl7pPr.dirty())
        else if self.lvl7pPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.lvl8pPr != null && this.lvl8pPr.dirty())
        else if self.lvl8pPr.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
        else
      {
         // Java: return this.lvl9pPr != null && this.lvl9pPr.dirty() ? true : this.map.dirty();
         return self.lvl9pPr.as_ref().map_or(false, |v| v.dirty()) || self.map.dirty();
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
      // Java: this.defpPr = null;
      self.defpPr = None;
      // Java: this.lvl1pPr = null;
      self.lvl1pPr = None;
      // Java: this.lvl2pPr = null;
      self.lvl2pPr = None;
      // Java: this.lvl3pPr = null;
      self.lvl3pPr = None;
      // Java: this.lvl4pPr = null;
      self.lvl4pPr = None;
      // Java: this.lvl5pPr = null;
      self.lvl5pPr = None;
      // Java: this.lvl6pPr = null;
      self.lvl6pPr = None;
      // Java: this.lvl7pPr = null;
      self.lvl7pPr = None;
      // Java: this.lvl8pPr = null;
      self.lvl8pPr = None;
      // Java: this.lvl9pPr = null;
      self.lvl9pPr = None;
    }

    // Java: void follow(TextListStyle parent)
    pub fn follow(&mut self, parent: Option<&mut TextListStyle>) {
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

    // Java: TextListStyle parent()
    pub fn parent(&self) -> Option<&TextListStyle> {
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

    // Java: TextParagraphProperties defpPr()
    pub fn defpPr(&mut self) -> &TextParagraphProperties {
      // Java: if (this.defpPr == null)
        if self.defpPr.is_none()
      {
         // Java: if (this.map.has(1))
            if self.map.has(1)
         {
            // Java: KoBin bin = this.map.byKey(1);
            // Java: this.defpPr = (TextParagraphProperties)KoBin.Decode.toAny(TextParagraphProperties.class, bin, this.zone);
            let bin = self.map.byKey(1);
            self.defpPr = Some(Box::new(TextParagraphProperties::TextParagraphProperties_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasDefpPr())
            else if self.parent.as_ref().map_or(false, |p| p.hasDefpPr())
         {
            // Java: this.defpPr = this.parent.defpPr();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.defpPr = (TextParagraphProperties)KoBin.Decode.toAny(TextParagraphProperties.class, bin, this.zone);
            let null_bin = Null::empty();
            self.defpPr = Some(Box::new(TextParagraphProperties::TextParagraphProperties_from(null_bin.as_ref())));
         }
      }
      // Java: return this.defpPr;
      self.defpPr.as_deref().unwrap()
    }

    // Java: boolean hasDefpPr()
    pub fn hasDefpPr(&self) -> bool {
      // Java: return this.map.has(1) || this.defpPr != null && this.defpPr.dirty() || this.parent != null && this.parent.hasDefpPr();
      self.map.has(1) || self.defpPr.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasDefpPr())
    }

    // Java: void deleteDefpPr()
    pub fn deleteDefpPr(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
      // Java: if (this.defpPr != null)
        if self.defpPr.is_some()
      {
         // Java: this.defpPr = null;
         self.defpPr = None;
      }
    }

    // Java: void writeDefpPr(TextParagraphProperties value)
    pub fn writeDefpPr(&mut self, value: Option<&TextParagraphProperties>) {
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
            // Java: if (this.defpPr != null)
               if self.defpPr.is_some()
            {
               // Java: this.defpPr = null;
               self.defpPr = None;
            }
         }
      }
    }

    // Java: void forceWriteDefpPr(TextParagraphProperties value)
    pub fn forceWriteDefpPr(&mut self, value: Option<&TextParagraphProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(1, v.lastBin(self.zone));
         // Java: if (this.defpPr != null)
            if self.defpPr.is_some()
         {
            // Java: this.defpPr = null;
            self.defpPr = None;
         }
      }
    }

    // Java: TextParagraphProperties lvl1pPr()
    pub fn lvl1pPr(&mut self) -> &TextParagraphProperties {
      // Java: if (this.lvl1pPr == null)
        if self.lvl1pPr.is_none()
      {
         // Java: if (this.map.has(2))
            if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            // Java: this.lvl1pPr = (TextParagraphProperties)KoBin.Decode.toAny(TextParagraphProperties.class, bin, this.zone);
            let bin = self.map.byKey(2);
            self.lvl1pPr = Some(Box::new(TextParagraphProperties::TextParagraphProperties_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasLvl1pPr())
            else if self.parent.as_ref().map_or(false, |p| p.hasLvl1pPr())
         {
            // Java: this.lvl1pPr = this.parent.lvl1pPr();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.lvl1pPr = (TextParagraphProperties)KoBin.Decode.toAny(TextParagraphProperties.class, bin, this.zone);
            let null_bin = Null::empty();
            self.lvl1pPr = Some(Box::new(TextParagraphProperties::TextParagraphProperties_from(null_bin.as_ref())));
         }
      }
      // Java: return this.lvl1pPr;
      self.lvl1pPr.as_deref().unwrap()
    }

    // Java: boolean hasLvl1pPr()
    pub fn hasLvl1pPr(&self) -> bool {
      // Java: return this.map.has(2) || this.lvl1pPr != null && this.lvl1pPr.dirty() || this.parent != null && this.parent.hasLvl1pPr();
      self.map.has(2) || self.lvl1pPr.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasLvl1pPr())
    }

    // Java: void deleteLvl1pPr()
    pub fn deleteLvl1pPr(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.lvl1pPr != null)
        if self.lvl1pPr.is_some()
      {
         // Java: this.lvl1pPr = null;
         self.lvl1pPr = None;
      }
    }

    // Java: void writeLvl1pPr(TextParagraphProperties value)
    pub fn writeLvl1pPr(&mut self, value: Option<&TextParagraphProperties>) {
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
            // Java: if (this.lvl1pPr != null)
               if self.lvl1pPr.is_some()
            {
               // Java: this.lvl1pPr = null;
               self.lvl1pPr = None;
            }
         }
      }
    }

    // Java: void forceWriteLvl1pPr(TextParagraphProperties value)
    pub fn forceWriteLvl1pPr(&mut self, value: Option<&TextParagraphProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.lvl1pPr != null)
            if self.lvl1pPr.is_some()
         {
            // Java: this.lvl1pPr = null;
            self.lvl1pPr = None;
         }
      }
    }

    // Java: TextParagraphProperties lvl2pPr()
    pub fn lvl2pPr(&mut self) -> &TextParagraphProperties {
      // Java: if (this.lvl2pPr == null)
        if self.lvl2pPr.is_none()
      {
         // Java: if (this.map.has(3))
            if self.map.has(3)
         {
            // Java: KoBin bin = this.map.byKey(3);
            // Java: this.lvl2pPr = (TextParagraphProperties)KoBin.Decode.toAny(TextParagraphProperties.class, bin, this.zone);
            let bin = self.map.byKey(3);
            self.lvl2pPr = Some(Box::new(TextParagraphProperties::TextParagraphProperties_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasLvl2pPr())
            else if self.parent.as_ref().map_or(false, |p| p.hasLvl2pPr())
         {
            // Java: this.lvl2pPr = this.parent.lvl2pPr();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.lvl2pPr = (TextParagraphProperties)KoBin.Decode.toAny(TextParagraphProperties.class, bin, this.zone);
            let null_bin = Null::empty();
            self.lvl2pPr = Some(Box::new(TextParagraphProperties::TextParagraphProperties_from(null_bin.as_ref())));
         }
      }
      // Java: return this.lvl2pPr;
      self.lvl2pPr.as_deref().unwrap()
    }

    // Java: boolean hasLvl2pPr()
    pub fn hasLvl2pPr(&self) -> bool {
      // Java: return this.map.has(3) || this.lvl2pPr != null && this.lvl2pPr.dirty() || this.parent != null && this.parent.hasLvl2pPr();
      self.map.has(3) || self.lvl2pPr.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasLvl2pPr())
    }

    // Java: void deleteLvl2pPr()
    pub fn deleteLvl2pPr(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
      // Java: if (this.lvl2pPr != null)
        if self.lvl2pPr.is_some()
      {
         // Java: this.lvl2pPr = null;
         self.lvl2pPr = None;
      }
    }

    // Java: void writeLvl2pPr(TextParagraphProperties value)
    pub fn writeLvl2pPr(&mut self, value: Option<&TextParagraphProperties>) {
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
            // Java: if (this.lvl2pPr != null)
               if self.lvl2pPr.is_some()
            {
               // Java: this.lvl2pPr = null;
               self.lvl2pPr = None;
            }
         }
      }
    }

    // Java: void forceWriteLvl2pPr(TextParagraphProperties value)
    pub fn forceWriteLvl2pPr(&mut self, value: Option<&TextParagraphProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(3, value.lastBin(this.zone));
         self.map.put(3, v.lastBin(self.zone));
         // Java: if (this.lvl2pPr != null)
            if self.lvl2pPr.is_some()
         {
            // Java: this.lvl2pPr = null;
            self.lvl2pPr = None;
         }
      }
    }

    // Java: TextParagraphProperties lvl3pPr()
    pub fn lvl3pPr(&mut self) -> &TextParagraphProperties {
      // Java: if (this.lvl3pPr == null)
        if self.lvl3pPr.is_none()
      {
         // Java: if (this.map.has(4))
            if self.map.has(4)
         {
            // Java: KoBin bin = this.map.byKey(4);
            // Java: this.lvl3pPr = (TextParagraphProperties)KoBin.Decode.toAny(TextParagraphProperties.class, bin, this.zone);
            let bin = self.map.byKey(4);
            self.lvl3pPr = Some(Box::new(TextParagraphProperties::TextParagraphProperties_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasLvl3pPr())
            else if self.parent.as_ref().map_or(false, |p| p.hasLvl3pPr())
         {
            // Java: this.lvl3pPr = this.parent.lvl3pPr();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.lvl3pPr = (TextParagraphProperties)KoBin.Decode.toAny(TextParagraphProperties.class, bin, this.zone);
            let null_bin = Null::empty();
            self.lvl3pPr = Some(Box::new(TextParagraphProperties::TextParagraphProperties_from(null_bin.as_ref())));
         }
      }
      // Java: return this.lvl3pPr;
      self.lvl3pPr.as_deref().unwrap()
    }

    // Java: boolean hasLvl3pPr()
    pub fn hasLvl3pPr(&self) -> bool {
      // Java: return this.map.has(4) || this.lvl3pPr != null && this.lvl3pPr.dirty() || this.parent != null && this.parent.hasLvl3pPr();
      self.map.has(4) || self.lvl3pPr.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasLvl3pPr())
    }

    // Java: void deleteLvl3pPr()
    pub fn deleteLvl3pPr(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
      // Java: if (this.lvl3pPr != null)
        if self.lvl3pPr.is_some()
      {
         // Java: this.lvl3pPr = null;
         self.lvl3pPr = None;
      }
    }

    // Java: void writeLvl3pPr(TextParagraphProperties value)
    pub fn writeLvl3pPr(&mut self, value: Option<&TextParagraphProperties>) {
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
            // Java: if (this.lvl3pPr != null)
               if self.lvl3pPr.is_some()
            {
               // Java: this.lvl3pPr = null;
               self.lvl3pPr = None;
            }
         }
      }
    }

    // Java: void forceWriteLvl3pPr(TextParagraphProperties value)
    pub fn forceWriteLvl3pPr(&mut self, value: Option<&TextParagraphProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(4, value.lastBin(this.zone));
         self.map.put(4, v.lastBin(self.zone));
         // Java: if (this.lvl3pPr != null)
            if self.lvl3pPr.is_some()
         {
            // Java: this.lvl3pPr = null;
            self.lvl3pPr = None;
         }
      }
    }

    // Java: TextParagraphProperties lvl4pPr()
    pub fn lvl4pPr(&mut self) -> &TextParagraphProperties {
      // Java: if (this.lvl4pPr == null)
        if self.lvl4pPr.is_none()
      {
         // Java: if (this.map.has(5))
            if self.map.has(5)
         {
            // Java: KoBin bin = this.map.byKey(5);
            // Java: this.lvl4pPr = (TextParagraphProperties)KoBin.Decode.toAny(TextParagraphProperties.class, bin, this.zone);
            let bin = self.map.byKey(5);
            self.lvl4pPr = Some(Box::new(TextParagraphProperties::TextParagraphProperties_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasLvl4pPr())
            else if self.parent.as_ref().map_or(false, |p| p.hasLvl4pPr())
         {
            // Java: this.lvl4pPr = this.parent.lvl4pPr();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.lvl4pPr = (TextParagraphProperties)KoBin.Decode.toAny(TextParagraphProperties.class, bin, this.zone);
            let null_bin = Null::empty();
            self.lvl4pPr = Some(Box::new(TextParagraphProperties::TextParagraphProperties_from(null_bin.as_ref())));
         }
      }
      // Java: return this.lvl4pPr;
      self.lvl4pPr.as_deref().unwrap()
    }

    // Java: boolean hasLvl4pPr()
    pub fn hasLvl4pPr(&self) -> bool {
      // Java: return this.map.has(5) || this.lvl4pPr != null && this.lvl4pPr.dirty() || this.parent != null && this.parent.hasLvl4pPr();
      self.map.has(5) || self.lvl4pPr.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasLvl4pPr())
    }

    // Java: void deleteLvl4pPr()
    pub fn deleteLvl4pPr(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(5);
      // Java: if (this.lvl4pPr != null)
        if self.lvl4pPr.is_some()
      {
         // Java: this.lvl4pPr = null;
         self.lvl4pPr = None;
      }
    }

    // Java: void writeLvl4pPr(TextParagraphProperties value)
    pub fn writeLvl4pPr(&mut self, value: Option<&TextParagraphProperties>) {
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
            // Java: if (this.lvl4pPr != null)
               if self.lvl4pPr.is_some()
            {
               // Java: this.lvl4pPr = null;
               self.lvl4pPr = None;
            }
         }
      }
    }

    // Java: void forceWriteLvl4pPr(TextParagraphProperties value)
    pub fn forceWriteLvl4pPr(&mut self, value: Option<&TextParagraphProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(5, value.lastBin(this.zone));
         self.map.put(5, v.lastBin(self.zone));
         // Java: if (this.lvl4pPr != null)
            if self.lvl4pPr.is_some()
         {
            // Java: this.lvl4pPr = null;
            self.lvl4pPr = None;
         }
      }
    }

    // Java: TextParagraphProperties lvl5pPr()
    pub fn lvl5pPr(&mut self) -> &TextParagraphProperties {
      // Java: if (this.lvl5pPr == null)
        if self.lvl5pPr.is_none()
      {
         // Java: if (this.map.has(6))
            if self.map.has(6)
         {
            // Java: KoBin bin = this.map.byKey(6);
            // Java: this.lvl5pPr = (TextParagraphProperties)KoBin.Decode.toAny(TextParagraphProperties.class, bin, this.zone);
            let bin = self.map.byKey(6);
            self.lvl5pPr = Some(Box::new(TextParagraphProperties::TextParagraphProperties_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasLvl5pPr())
            else if self.parent.as_ref().map_or(false, |p| p.hasLvl5pPr())
         {
            // Java: this.lvl5pPr = this.parent.lvl5pPr();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.lvl5pPr = (TextParagraphProperties)KoBin.Decode.toAny(TextParagraphProperties.class, bin, this.zone);
            let null_bin = Null::empty();
            self.lvl5pPr = Some(Box::new(TextParagraphProperties::TextParagraphProperties_from(null_bin.as_ref())));
         }
      }
      // Java: return this.lvl5pPr;
      self.lvl5pPr.as_deref().unwrap()
    }

    // Java: boolean hasLvl5pPr()
    pub fn hasLvl5pPr(&self) -> bool {
      // Java: return this.map.has(6) || this.lvl5pPr != null && this.lvl5pPr.dirty() || this.parent != null && this.parent.hasLvl5pPr();
      self.map.has(6) || self.lvl5pPr.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasLvl5pPr())
    }

    // Java: void deleteLvl5pPr()
    pub fn deleteLvl5pPr(&mut self) {
      // Java: this.map.put(6, (KoBin)null);
      self.map.clear_key(6);
      // Java: if (this.lvl5pPr != null)
        if self.lvl5pPr.is_some()
      {
         // Java: this.lvl5pPr = null;
         self.lvl5pPr = None;
      }
    }

    // Java: void writeLvl5pPr(TextParagraphProperties value)
    pub fn writeLvl5pPr(&mut self, value: Option<&TextParagraphProperties>) {
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
            // Java: if (this.lvl5pPr != null)
               if self.lvl5pPr.is_some()
            {
               // Java: this.lvl5pPr = null;
               self.lvl5pPr = None;
            }
         }
      }
    }

    // Java: void forceWriteLvl5pPr(TextParagraphProperties value)
    pub fn forceWriteLvl5pPr(&mut self, value: Option<&TextParagraphProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(6, value.lastBin(this.zone));
         self.map.put(6, v.lastBin(self.zone));
         // Java: if (this.lvl5pPr != null)
            if self.lvl5pPr.is_some()
         {
            // Java: this.lvl5pPr = null;
            self.lvl5pPr = None;
         }
      }
    }

    // Java: TextParagraphProperties lvl6pPr()
    pub fn lvl6pPr(&mut self) -> &TextParagraphProperties {
      // Java: if (this.lvl6pPr == null)
        if self.lvl6pPr.is_none()
      {
         // Java: if (this.map.has(7))
            if self.map.has(7)
         {
            // Java: KoBin bin = this.map.byKey(7);
            // Java: this.lvl6pPr = (TextParagraphProperties)KoBin.Decode.toAny(TextParagraphProperties.class, bin, this.zone);
            let bin = self.map.byKey(7);
            self.lvl6pPr = Some(Box::new(TextParagraphProperties::TextParagraphProperties_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasLvl6pPr())
            else if self.parent.as_ref().map_or(false, |p| p.hasLvl6pPr())
         {
            // Java: this.lvl6pPr = this.parent.lvl6pPr();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.lvl6pPr = (TextParagraphProperties)KoBin.Decode.toAny(TextParagraphProperties.class, bin, this.zone);
            let null_bin = Null::empty();
            self.lvl6pPr = Some(Box::new(TextParagraphProperties::TextParagraphProperties_from(null_bin.as_ref())));
         }
      }
      // Java: return this.lvl6pPr;
      self.lvl6pPr.as_deref().unwrap()
    }

    // Java: boolean hasLvl6pPr()
    pub fn hasLvl6pPr(&self) -> bool {
      // Java: return this.map.has(7) || this.lvl6pPr != null && this.lvl6pPr.dirty() || this.parent != null && this.parent.hasLvl6pPr();
      self.map.has(7) || self.lvl6pPr.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasLvl6pPr())
    }

    // Java: void deleteLvl6pPr()
    pub fn deleteLvl6pPr(&mut self) {
      // Java: this.map.put(7, (KoBin)null);
      self.map.clear_key(7);
      // Java: if (this.lvl6pPr != null)
        if self.lvl6pPr.is_some()
      {
         // Java: this.lvl6pPr = null;
         self.lvl6pPr = None;
      }
    }

    // Java: void writeLvl6pPr(TextParagraphProperties value)
    pub fn writeLvl6pPr(&mut self, value: Option<&TextParagraphProperties>) {
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
            // Java: if (this.lvl6pPr != null)
               if self.lvl6pPr.is_some()
            {
               // Java: this.lvl6pPr = null;
               self.lvl6pPr = None;
            }
         }
      }
    }

    // Java: void forceWriteLvl6pPr(TextParagraphProperties value)
    pub fn forceWriteLvl6pPr(&mut self, value: Option<&TextParagraphProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(7, value.lastBin(this.zone));
         self.map.put(7, v.lastBin(self.zone));
         // Java: if (this.lvl6pPr != null)
            if self.lvl6pPr.is_some()
         {
            // Java: this.lvl6pPr = null;
            self.lvl6pPr = None;
         }
      }
    }

    // Java: TextParagraphProperties lvl7pPr()
    pub fn lvl7pPr(&mut self) -> &TextParagraphProperties {
      // Java: if (this.lvl7pPr == null)
        if self.lvl7pPr.is_none()
      {
         // Java: if (this.map.has(8))
            if self.map.has(8)
         {
            // Java: KoBin bin = this.map.byKey(8);
            // Java: this.lvl7pPr = (TextParagraphProperties)KoBin.Decode.toAny(TextParagraphProperties.class, bin, this.zone);
            let bin = self.map.byKey(8);
            self.lvl7pPr = Some(Box::new(TextParagraphProperties::TextParagraphProperties_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasLvl7pPr())
            else if self.parent.as_ref().map_or(false, |p| p.hasLvl7pPr())
         {
            // Java: this.lvl7pPr = this.parent.lvl7pPr();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.lvl7pPr = (TextParagraphProperties)KoBin.Decode.toAny(TextParagraphProperties.class, bin, this.zone);
            let null_bin = Null::empty();
            self.lvl7pPr = Some(Box::new(TextParagraphProperties::TextParagraphProperties_from(null_bin.as_ref())));
         }
      }
      // Java: return this.lvl7pPr;
      self.lvl7pPr.as_deref().unwrap()
    }

    // Java: boolean hasLvl7pPr()
    pub fn hasLvl7pPr(&self) -> bool {
      // Java: return this.map.has(8) || this.lvl7pPr != null && this.lvl7pPr.dirty() || this.parent != null && this.parent.hasLvl7pPr();
      self.map.has(8) || self.lvl7pPr.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasLvl7pPr())
    }

    // Java: void deleteLvl7pPr()
    pub fn deleteLvl7pPr(&mut self) {
      // Java: this.map.put(8, (KoBin)null);
      self.map.clear_key(8);
      // Java: if (this.lvl7pPr != null)
        if self.lvl7pPr.is_some()
      {
         // Java: this.lvl7pPr = null;
         self.lvl7pPr = None;
      }
    }

    // Java: void writeLvl7pPr(TextParagraphProperties value)
    pub fn writeLvl7pPr(&mut self, value: Option<&TextParagraphProperties>) {
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
            // Java: if (this.lvl7pPr != null)
               if self.lvl7pPr.is_some()
            {
               // Java: this.lvl7pPr = null;
               self.lvl7pPr = None;
            }
         }
      }
    }

    // Java: void forceWriteLvl7pPr(TextParagraphProperties value)
    pub fn forceWriteLvl7pPr(&mut self, value: Option<&TextParagraphProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(8, value.lastBin(this.zone));
         self.map.put(8, v.lastBin(self.zone));
         // Java: if (this.lvl7pPr != null)
            if self.lvl7pPr.is_some()
         {
            // Java: this.lvl7pPr = null;
            self.lvl7pPr = None;
         }
      }
    }

    // Java: TextParagraphProperties lvl8pPr()
    pub fn lvl8pPr(&mut self) -> &TextParagraphProperties {
      // Java: if (this.lvl8pPr == null)
        if self.lvl8pPr.is_none()
      {
         // Java: if (this.map.has(9))
            if self.map.has(9)
         {
            // Java: KoBin bin = this.map.byKey(9);
            // Java: this.lvl8pPr = (TextParagraphProperties)KoBin.Decode.toAny(TextParagraphProperties.class, bin, this.zone);
            let bin = self.map.byKey(9);
            self.lvl8pPr = Some(Box::new(TextParagraphProperties::TextParagraphProperties_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasLvl8pPr())
            else if self.parent.as_ref().map_or(false, |p| p.hasLvl8pPr())
         {
            // Java: this.lvl8pPr = this.parent.lvl8pPr();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.lvl8pPr = (TextParagraphProperties)KoBin.Decode.toAny(TextParagraphProperties.class, bin, this.zone);
            let null_bin = Null::empty();
            self.lvl8pPr = Some(Box::new(TextParagraphProperties::TextParagraphProperties_from(null_bin.as_ref())));
         }
      }
      // Java: return this.lvl8pPr;
      self.lvl8pPr.as_deref().unwrap()
    }

    // Java: boolean hasLvl8pPr()
    pub fn hasLvl8pPr(&self) -> bool {
      // Java: return this.map.has(9) || this.lvl8pPr != null && this.lvl8pPr.dirty() || this.parent != null && this.parent.hasLvl8pPr();
      self.map.has(9) || self.lvl8pPr.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasLvl8pPr())
    }

    // Java: void deleteLvl8pPr()
    pub fn deleteLvl8pPr(&mut self) {
      // Java: this.map.put(9, (KoBin)null);
      self.map.clear_key(9);
      // Java: if (this.lvl8pPr != null)
        if self.lvl8pPr.is_some()
      {
         // Java: this.lvl8pPr = null;
         self.lvl8pPr = None;
      }
    }

    // Java: void writeLvl8pPr(TextParagraphProperties value)
    pub fn writeLvl8pPr(&mut self, value: Option<&TextParagraphProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(9, bin);
            self.map.put(9, bin);
            // Java: if (this.lvl8pPr != null)
               if self.lvl8pPr.is_some()
            {
               // Java: this.lvl8pPr = null;
               self.lvl8pPr = None;
            }
         }
      }
    }

    // Java: void forceWriteLvl8pPr(TextParagraphProperties value)
    pub fn forceWriteLvl8pPr(&mut self, value: Option<&TextParagraphProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(9, value.lastBin(this.zone));
         self.map.put(9, v.lastBin(self.zone));
         // Java: if (this.lvl8pPr != null)
            if self.lvl8pPr.is_some()
         {
            // Java: this.lvl8pPr = null;
            self.lvl8pPr = None;
         }
      }
    }

    // Java: TextParagraphProperties lvl9pPr()
    pub fn lvl9pPr(&mut self) -> &TextParagraphProperties {
      // Java: if (this.lvl9pPr == null)
        if self.lvl9pPr.is_none()
      {
         // Java: if (this.map.has(10))
            if self.map.has(10)
         {
            // Java: KoBin bin = this.map.byKey(10);
            // Java: this.lvl9pPr = (TextParagraphProperties)KoBin.Decode.toAny(TextParagraphProperties.class, bin, this.zone);
            let bin = self.map.byKey(10);
            self.lvl9pPr = Some(Box::new(TextParagraphProperties::TextParagraphProperties_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasLvl9pPr())
            else if self.parent.as_ref().map_or(false, |p| p.hasLvl9pPr())
         {
            // Java: this.lvl9pPr = this.parent.lvl9pPr();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.lvl9pPr = (TextParagraphProperties)KoBin.Decode.toAny(TextParagraphProperties.class, bin, this.zone);
            let null_bin = Null::empty();
            self.lvl9pPr = Some(Box::new(TextParagraphProperties::TextParagraphProperties_from(null_bin.as_ref())));
         }
      }
      // Java: return this.lvl9pPr;
      self.lvl9pPr.as_deref().unwrap()
    }

    // Java: boolean hasLvl9pPr()
    pub fn hasLvl9pPr(&self) -> bool {
      // Java: return this.map.has(10) || this.lvl9pPr != null && this.lvl9pPr.dirty() || this.parent != null && this.parent.hasLvl9pPr();
      self.map.has(10) || self.lvl9pPr.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasLvl9pPr())
    }

    // Java: void deleteLvl9pPr()
    pub fn deleteLvl9pPr(&mut self) {
      // Java: this.map.put(10, (KoBin)null);
      self.map.clear_key(10);
      // Java: if (this.lvl9pPr != null)
        if self.lvl9pPr.is_some()
      {
         // Java: this.lvl9pPr = null;
         self.lvl9pPr = None;
      }
    }

    // Java: void writeLvl9pPr(TextParagraphProperties value)
    pub fn writeLvl9pPr(&mut self, value: Option<&TextParagraphProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(10, bin);
            self.map.put(10, bin);
            // Java: if (this.lvl9pPr != null)
               if self.lvl9pPr.is_some()
            {
               // Java: this.lvl9pPr = null;
               self.lvl9pPr = None;
            }
         }
      }
    }

    // Java: void forceWriteLvl9pPr(TextParagraphProperties value)
    pub fn forceWriteLvl9pPr(&mut self, value: Option<&TextParagraphProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(10, value.lastBin(this.zone));
         self.map.put(10, v.lastBin(self.zone));
         // Java: if (this.lvl9pPr != null)
            if self.lvl9pPr.is_some()
         {
            // Java: this.lvl9pPr = null;
            self.lvl9pPr = None;
         }
      }
    }

}

impl KoAr for TextListStyle {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for TextListStyle { fn close(&mut self) { self.close(); } }