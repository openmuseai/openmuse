#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.blip.Blip
use crate::cn::wps::drawing::data::blip::Blip::Blip;
// Java: import cn.wps.drawing.data.color.Color
use crate::cn::wps::drawing::data::color::Color::Color;
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
// Java: import cn.wps.kso.kproto.internal.KpBool
use crate::cn::wps::kso::kproto::internal::KpBool::KpBool;
// Java: import cn.wps.kso.kproto.internal.KpList
use crate::cn::wps::kso::kproto::internal::KpList::KpList;
// Java: import cn.wps.kso.kproto.internal.KpMap
use crate::cn::wps::kso::kproto::internal::KpMap::KpMap;
// Java: import java.util.ArrayList
// Java: import java.util.Collection

use crate::cn::wps::drawing::data::text::TextCharacterProperties::TextCharacterProperties;

// Java: public class TextParagraphProperties implements KoAr, KoClosable
pub struct TextParagraphProperties {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: TextSpacing lnSpc
    pub(crate) lnSpc: Option<Box<TextSpacing>>,
    // Java field: TextSpacing spcBef
    pub(crate) spcBef: Option<Box<TextSpacing>>,
    // Java field: TextSpacing spcAft
    pub(crate) spcAft: Option<Box<TextSpacing>>,
    // Java field: Color buClr
    pub(crate) buClr: Option<Box<Color>>,
    // Java field: TextFont buFont
    pub(crate) buFont: Option<Box<TextFont>>,
    // Java field: TextAutonumberBullet buAutonNum
    pub(crate) buAutonNum: Option<Box<TextAutonumberBullet>>,
    // Java field: Blip buBlip
    pub(crate) buBlip: Option<Box<Blip>>,
    // Java field: TabStopList tabStopLst
    pub(crate) tabStopLst: Option<Box<TabStopList>>,
    // Java field: TextCharacterProperties defRPr
    pub(crate) defRPr: Option<Box<TextCharacterProperties>>,
    // Java field: TextParaOptions options
    pub(crate) options: Option<Box<TextParaOptions>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: TextParagraphProperties parent
    pub(crate) parent: Option<Box<TextParagraphProperties>>,
}

// Java: public class TextSpacing
pub struct TextSpacing {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: TextSpacing parent
    pub(crate) parent: Option<Box<TextSpacing>>,
}

// Java: public class TextAutonumberBullet
pub struct TextAutonumberBullet {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: TextAutonumberBullet parent
    pub(crate) parent: Option<Box<TextAutonumberBullet>>,
}

// Java: public class TextTabStop
pub struct TextTabStop {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: TextTabStop parent
    pub(crate) parent: Option<Box<TextTabStop>>,
}

// Java: public class TabStopList
pub struct TabStopList {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

// Java: public class TextParaOptions
pub struct TextParaOptions {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpBool map
    pub(crate) map: KpBool,
    // Java field: TextParaOptions parent
    pub(crate) parent: Option<Box<TextParaOptions>>,
}

// Java: public class TextParagraphProperties implements KoAr, KoClosable
impl TextParagraphProperties {
    // Java: static protected static <Value1 extends TextParagraphProperties> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::TextParagraphProperties_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static TextParagraphProperties create()
    pub fn create() -> Self {
      // Java: return initialize(TextParagraphProperties.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static TextParagraphProperties open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(TextParagraphProperties.class, bin);
      Self::initialize(bin)
    }

    // Java: TextParagraphProperties()
    pub fn new() -> Self {
      Self {
      // Java: this.box = new KpBinBox();
      bin_box: KpBinBox::new(),
      // Java: this.map = new KpMap();
      map: KpMap::new(),
      zone: KoZone::defaultZone(), parent: None,
      lnSpc: None, spcBef: None, spcAft: None, buClr: None,
      buFont: None, buAutonNum: None, buBlip: None,
      tabStopLst: None, defRPr: None, options: None,
      }
    }

    // Java: TextParagraphProperties(KoBin bin)
    pub fn TextParagraphProperties_from(bin: &dyn KoBinTrait) -> Self {
      Self {
      // Java: this.box = new KpBinBox(bin);
      bin_box: KpBinBox::KpBinBox_from(Some(bin)),
      // Java: this.map = new KpMap(bin);
      map: KpMap::KpMap_from(bin),
      zone: KoZone::defaultZone(), parent: None,
      lnSpc: None, spcBef: None, spcAft: None, buClr: None,
      buFont: None, buAutonNum: None, buBlip: None,
      tabStopLst: None, defRPr: None, options: None,
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
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      let zone = self.zone;
      self.loadBin(bin, zone);
    }

    // Java: void load(TextParagraphProperties value)
    pub fn load_value(&mut self, value: Option<&TextParagraphProperties>) {
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
      // Java: if (this.lnSpc != null && this.lnSpc.dirty())
        if self.lnSpc.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(1, this.lnSpc.saveBin(zone));
         let bin = self.lnSpc.as_mut().unwrap().saveBin(zone);
         self.map.put(1, bin);
      }
      // Java: if (this.spcBef != null && this.spcBef.dirty())
        if self.spcBef.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(2, this.spcBef.saveBin(zone));
         let bin = self.spcBef.as_mut().unwrap().saveBin(zone);
         self.map.put(2, bin);
      }
      // Java: if (this.spcAft != null && this.spcAft.dirty())
        if self.spcAft.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(3, this.spcAft.saveBin(zone));
         let bin = self.spcAft.as_mut().unwrap().saveBin(zone);
         self.map.put(3, bin);
      }
      // Java: if (this.buClr != null && this.buClr.dirty())
        if self.buClr.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(4, this.buClr.saveBin(zone));
         let bin = self.buClr.as_mut().unwrap().saveBin(zone);
         self.map.put(4, bin);
      }
      // Java: if (this.buFont != null && this.buFont.dirty())
        if self.buFont.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(7, this.buFont.saveBin(zone));
         let bin = self.buFont.as_mut().unwrap().saveBin(zone);
         self.map.put(7, bin);
      }
      // Java: if (this.buAutonNum != null && this.buAutonNum.dirty())
        if self.buAutonNum.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(8, this.buAutonNum.saveBin(zone));
         let bin = self.buAutonNum.as_mut().unwrap().saveBin(zone);
         self.map.put(8, bin);
      }
      // Java: if (this.buBlip != null && this.buBlip.dirty())
        if self.buBlip.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(10, this.buBlip.saveBin(zone));
         let bin = self.buBlip.as_mut().unwrap().saveBin(zone);
         self.map.put(10, bin);
      }
      // Java: if (this.tabStopLst != null && this.tabStopLst.dirty())
        if self.tabStopLst.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(11, this.tabStopLst.saveBin(zone));
         let bin = self.tabStopLst.as_mut().unwrap().saveBin(zone);
         self.map.put(11, bin);
      }
      // Java: if (this.defRPr != null && this.defRPr.dirty())
        if self.defRPr.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(12, this.defRPr.saveBin(zone));
         let bin = self.defRPr.as_mut().unwrap().saveBin(zone);
         self.map.put(12, bin);
      }
      // Java: if (this.options != null && this.options.dirty())
        if self.options.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(20, this.options.saveBin(zone));
         let bin = self.options.as_mut().unwrap().saveBin(zone);
         self.map.put(20, bin);
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

    // Java: boolean equalsLax(TextParagraphProperties o)
    pub fn equalsLax(&self, o: &TextParagraphProperties) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TextParagraphProperties) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &TextParagraphProperties, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof TextParagraphProperties)
        if true
      {
         // Java: TextParagraphProperties dst = (TextParagraphProperties)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasLnSpc() || !dst.hasLnSpc()) && (!this.hasLnSpc() || dst.hasLnSpc()))
            else if (self.hasLnSpc() || !dst.hasLnSpc()) && (!self.hasLnSpc() || dst.hasLnSpc())
         {
            // Java: if (this.hasLnSpc() && dst.hasLnSpc())
                if self.hasLnSpc() && dst.hasLnSpc()
            {
               // Java: if (isStrict && !this.lnSpc().equals(dst.lnSpc()))
                    if isStrict && !self.lnSpc().equals(dst.lnSpc())
               {
                  // Java: return false;
                  return false;
               }
               // Java: if (!isStrict && !this.lnSpc().equalsLax(dst.lnSpc()))
                    if !isStrict && !self.lnSpc().equalsLax(dst.lnSpc())
               {
                  // Java: return false;
                  return false;
               }
            }
            // Java: if ((this.hasSpcBef() || !dst.hasSpcBef()) && (!this.hasSpcBef() || dst.hasSpcBef()))
                if (self.hasSpcBef() || !dst.hasSpcBef()) && (!self.hasSpcBef() || dst.hasSpcBef())
            {
               // Java: if (this.hasSpcBef() && dst.hasSpcBef())
                    if self.hasSpcBef() && dst.hasSpcBef()
               {
                  // Java: if (isStrict && !this.spcBef().equals(dst.spcBef()))
                        if isStrict && !self.spcBef().equals(dst.spcBef())
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !this.spcBef().equalsLax(dst.spcBef()))
                        if !isStrict && !self.spcBef().equalsLax(dst.spcBef())
                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: if ((this.hasSpcAft() || !dst.hasSpcAft()) && (!this.hasSpcAft() || dst.hasSpcAft()))
                    if (self.hasSpcAft() || !dst.hasSpcAft()) && (!self.hasSpcAft() || dst.hasSpcAft())
               {
                  // Java: if (this.hasSpcAft() && dst.hasSpcAft())
                        if self.hasSpcAft() && dst.hasSpcAft()
                  {
                     // Java: if (isStrict && !this.spcAft().equals(dst.spcAft()))
                            if isStrict && !self.spcAft().equals(dst.spcAft())
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !this.spcAft().equalsLax(dst.spcAft()))
                            if !isStrict && !self.spcAft().equalsLax(dst.spcAft())
                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: if ((this.hasBuClr() || !dst.hasBuClr()) && (!this.hasBuClr() || dst.hasBuClr()))
                        if (self.hasBuClr() || !dst.hasBuClr()) && (!self.hasBuClr() || dst.hasBuClr())
                  {
                     // Java: if (this.hasBuClr() && dst.hasBuClr())
                            if self.hasBuClr() && dst.hasBuClr()
                     {
                        // Java: if (isStrict && !this.buClr().equals(dst.buClr()))
                                if isStrict && !self.buClr().equals(dst.buClr())
                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: if (!isStrict && !this.buClr().equalsLax(dst.buClr()))
                                if !isStrict && !self.buClr().equalsLax(dst.buClr())
                        {
                           // Java: return false;
                           return false;
                        }
                     }
                     // Java: if ((this.hasBuSzPct() || !dst.hasBuSzPct()) && (!this.hasBuSzPct() || dst.hasBuSzPct()))
                            if (self.hasBuSzPct() || !dst.hasBuSzPct()) && (!self.hasBuSzPct() || dst.hasBuSzPct())
                     {
                        // Java: if (this.hasBuSzPct() && dst.hasBuSzPct() && this.buSzPct() != dst.buSzPct())
                                if self.hasBuSzPct() && dst.hasBuSzPct() && self.buSzPct() != dst.buSzPct()
                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: else if ((this.hasBuSzPts() || !dst.hasBuSzPts()) && (!this.hasBuSzPts() || dst.hasBuSzPts()))
                                else if (self.hasBuSzPts() || !dst.hasBuSzPts()) && (!self.hasBuSzPts() || dst.hasBuSzPts())
                        {
                           // Java: if (this.hasBuSzPts() && dst.hasBuSzPts() && this.buSzPts() != dst.buSzPts())
                                    if self.hasBuSzPts() && dst.hasBuSzPts() && self.buSzPts() != dst.buSzPts()
                           {
                              // Java: return false;
                              return false;
                           }
                           // Java: else if ((this.hasBuFont() || !dst.hasBuFont()) && (!this.hasBuFont() || dst.hasBuFont()))
                                    else if (self.hasBuFont() || !dst.hasBuFont()) && (!self.hasBuFont() || dst.hasBuFont())
                           {
                              // Java: if (this.hasBuFont() && dst.hasBuFont())
                                        if self.hasBuFont() && dst.hasBuFont()
                              {
                                 // Java: if (isStrict && !this.buFont().equals(dst.buFont()))
                                            if isStrict && !self.buFont().equals(dst.buFont())
                                 {
                                    // Java: return false;
                                    return false;
                                 }
                                 // Java: if (!isStrict && !this.buFont().equalsLax(dst.buFont()))
                                            if !isStrict && !self.buFont().equalsLax(dst.buFont())
                                 {
                                    // Java: return false;
                                    return false;
                                 }
                              }
                              // Java: if ((this.hasBuAutonNum() || !dst.hasBuAutonNum()) && (!this.hasBuAutonNum() || dst.hasBuAutonNum()))
                                        if (self.hasBuAutonNum() || !dst.hasBuAutonNum()) && (!self.hasBuAutonNum() || dst.hasBuAutonNum())
                              {
                                 // Java: if (this.hasBuAutonNum() && dst.hasBuAutonNum())
                                            if self.hasBuAutonNum() && dst.hasBuAutonNum()
                                 {
                                    // Java: if (isStrict && !this.buAutonNum().equals(dst.buAutonNum()))
                                                if isStrict && !self.buAutonNum().equals(dst.buAutonNum())
                                    {
                                       // Java: return false;
                                       return false;
                                    }
                                    // Java: if (!isStrict && !this.buAutonNum().equalsLax(dst.buAutonNum()))
                                                if !isStrict && !self.buAutonNum().equalsLax(dst.buAutonNum())
                                    {
                                       // Java: return false;
                                       return false;
                                    }
                                 }
                                 // Java: if ((this.hasBuChar() || !dst.hasBuChar()) && (!this.hasBuChar() || dst.hasBuChar()))
                                            if (self.hasBuChar() || !dst.hasBuChar()) && (!self.hasBuChar() || dst.hasBuChar())
                                 {
                                    // Java: if (this.hasBuChar() && dst.hasBuChar() && !this.buChar().equals(dst.buChar()))
                                                if self.hasBuChar() && dst.hasBuChar() && self.buChar() != dst.buChar()
                                    {
                                       // Java: return false;
                                       return false;
                                    }
                                    // Java: else if ((this.hasBuBlip() || !dst.hasBuBlip()) && (!this.hasBuBlip() || dst.hasBuBlip()))
                                                else if (self.hasBuBlip() || !dst.hasBuBlip()) && (!self.hasBuBlip() || dst.hasBuBlip())
                                    {
                                       // Java: if (this.hasBuBlip() && dst.hasBuBlip())
                                                    if self.hasBuBlip() && dst.hasBuBlip()
                                       {
                                          // Java: if (isStrict && !this.buBlip().equals(dst.buBlip()))
                                                        if isStrict && !self.buBlip().equals(dst.buBlip())
                                          {
                                             // Java: return false;
                                             return false;
                                          }
                                          // Java: if (!isStrict && !this.buBlip().equalsLax(dst.buBlip()))
                                                        if !isStrict && !self.buBlip().equalsLax(dst.buBlip())
                                          {
                                             // Java: return false;
                                             return false;
                                          }
                                       }
                                       // Java: if ((this.hasTabStopLst() || !dst.hasTabStopLst()) && (!this.hasTabStopLst() || dst.hasTabStopLst()))
                                                    if (self.hasTabStopLst() || !dst.hasTabStopLst()) && (!self.hasTabStopLst() || dst.hasTabStopLst())
                                       {
                                          // Java: if (this.hasTabStopLst() && dst.hasTabStopLst())
                                                        if self.hasTabStopLst() && dst.hasTabStopLst()
                                          {
                                             // Java: if (isStrict && !this.tabStopLst().equals(dst.tabStopLst()))
                                                            if isStrict && !self.tabStopLst().equals(dst.tabStopLst())
                                             {
                                                // Java: return false;
                                                return false;
                                             }
                                             // Java: if (!isStrict && !this.tabStopLst().equalsLax(dst.tabStopLst()))
                                                            if !isStrict && !self.tabStopLst().equalsLax(dst.tabStopLst())
                                             {
                                                // Java: return false;
                                                return false;
                                             }
                                          }
                                          // Java: if ((this.hasDefRPr() || !dst.hasDefRPr()) && (!this.hasDefRPr() || dst.hasDefRPr()))
                                                        if (self.hasDefRPr() || !dst.hasDefRPr()) && (!self.hasDefRPr() || dst.hasDefRPr())
                                          {
                                             // Java: if (this.hasDefRPr() && dst.hasDefRPr())
                                                            if self.hasDefRPr() && dst.hasDefRPr()
                                             {
                                                // Java: if (isStrict && !this.defRPr().equals(dst.defRPr()))
                                                                if isStrict && !self.defRPr().equals(dst.defRPr())
                                                {
                                                   // Java: return false;
                                                   return false;
                                                }
                                                // Java: if (!isStrict && !this.defRPr().equalsLax(dst.defRPr()))
                                                                if !isStrict && !self.defRPr().equalsLax(dst.defRPr())
                                                {
                                                   // Java: return false;
                                                   return false;
                                                }
                                             }
                                             // Java: if ((this.hasMarginLeft() || !dst.hasMarginLeft()) && (!this.hasMarginLeft() || dst.hasMarginLeft()))
                                                            if (self.hasMarginLeft() || !dst.hasMarginLeft()) && (!self.hasMarginLeft() || dst.hasMarginLeft())
                                             {
                                                // Java: if (this.hasMarginLeft() && dst.hasMarginLeft() && this.marginLeft() != dst.marginLeft())
                                                                if self.hasMarginLeft() && dst.hasMarginLeft() && self.marginLeft() != dst.marginLeft()
                                                {
                                                   // Java: return false;
                                                   return false;
                                                }
                                                // Java: else if ((this.hasMarginRight() || !dst.hasMarginRight()) && (!this.hasMarginRight() || dst.hasMarginRight()))
                                                                else if (self.hasMarginRight() || !dst.hasMarginRight()) && (!self.hasMarginRight() || dst.hasMarginRight())
                                                {
                                                   // Java: if (this.hasMarginRight() && dst.hasMarginRight() && this.marginRight() != dst.marginRight())
                                                                    if self.hasMarginRight() && dst.hasMarginRight() && self.marginRight() != dst.marginRight()
                                                   {
                                                      // Java: return false;
                                                      return false;
                                                   }
                                                   // Java: else if ((this.hasLvl() || !dst.hasLvl()) && (!this.hasLvl() || dst.hasLvl()))
                                                                    else if (self.hasLvl() || !dst.hasLvl()) && (!self.hasLvl() || dst.hasLvl())
                                                   {
                                                      // Java: if (this.hasLvl() && dst.hasLvl() && this.lvl() != dst.lvl())
                                                                        if self.hasLvl() && dst.hasLvl() && self.lvl() != dst.lvl()
                                                      {
                                                         // Java: return false;
                                                         return false;
                                                      }
                                                      // Java: else if ((this.hasIndent() || !dst.hasIndent()) && (!this.hasIndent() || dst.hasIndent()))
                                                                        else if (self.hasIndent() || !dst.hasIndent()) && (!self.hasIndent() || dst.hasIndent())
                                                      {
                                                         // Java: if (this.hasIndent() && dst.hasIndent() && this.indent() != dst.indent())
                                                                            if self.hasIndent() && dst.hasIndent() && self.indent() != dst.indent()
                                                         {
                                                            // Java: return false;
                                                            return false;
                                                         }
                                                         // Java: else if ((this.hasAlignment() || !dst.hasAlignment()) && (!this.hasAlignment() || dst.hasAlignment()))
                                                                            else if (self.hasAlignment() || !dst.hasAlignment()) && (!self.hasAlignment() || dst.hasAlignment())
                                                         {
                                                            // Java: if (this.hasAlignment() && dst.hasAlignment() && this.alignment() != dst.alignment())
                                                                                if self.hasAlignment() && dst.hasAlignment() && self.alignment() != dst.alignment()
                                                            {
                                                               // Java: return false;
                                                               return false;
                                                            }
                                                            // Java: else if ((this.hasDefTabSize() || !dst.hasDefTabSize()) && (!this.hasDefTabSize() || dst.hasDefTabSize()))
                                                                                else if (self.hasDefTabSize() || !dst.hasDefTabSize()) && (!self.hasDefTabSize() || dst.hasDefTabSize())
                                                            {
                                                               // Java: if (this.hasDefTabSize() && dst.hasDefTabSize() && this.defTabSize() != dst.defTabSize())
                                                                                    if self.hasDefTabSize() && dst.hasDefTabSize() && self.defTabSize() != dst.defTabSize()
                                                               {
                                                                  // Java: return false;
                                                                  return false;
                                                               }
                                                               // Java: else if ((this.hasFontAlgn() || !dst.hasFontAlgn()) && (!this.hasFontAlgn() || dst.hasFontAlgn()))
                                                                                    else if (self.hasFontAlgn() || !dst.hasFontAlgn()) && (!self.hasFontAlgn() || dst.hasFontAlgn())
                                                               {
                                                                  // Java: if (this.hasFontAlgn() && dst.hasFontAlgn() && this.fontAlgn() != dst.fontAlgn())
                                                                                        if self.hasFontAlgn() && dst.hasFontAlgn() && self.fontAlgn() != dst.fontAlgn()
                                                                  {
                                                                     // Java: return false;
                                                                     return false;
                                                                  }
                                                                  // Java: else if ((this.hasOptions() || !dst.hasOptions()) && (!this.hasOptions() || dst.hasOptions()))
                                                                                        else if (self.hasOptions() || !dst.hasOptions()) && (!self.hasOptions() || dst.hasOptions())
                                                                  {
                                                                     // Java: if (this.hasOptions() && dst.hasOptions())
                                                                                            if self.hasOptions() && dst.hasOptions()
                                                                     {
                                                                        // Java: if (isStrict && !this.options().equals(dst.options()))
                                                                                                if isStrict && !self.options().equals(dst.options())
                                                                        {
                                                                           // Java: return false;
                                                                           return false;
                                                                        }
                                                                        // Java: if (!isStrict && !this.options().equalsLax(dst.options()))
                                                                                                if !isStrict && !self.options().equalsLax(dst.options())
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
      // Java: if (this.hasLnSpc())
        if self.hasLnSpc()
      {
         // Java: ret += this.lnSpc().hashCode();
         ret = ret.wrapping_add(self.lnSpc().hashCode());
      }
      // Java: if (this.hasSpcBef())
        if self.hasSpcBef()
      {
         // Java: ret += this.spcBef().hashCode();
         ret = ret.wrapping_add(self.spcBef().hashCode());
      }
      // Java: if (this.hasSpcAft())
        if self.hasSpcAft()
      {
         // Java: ret += this.spcAft().hashCode();
         ret = ret.wrapping_add(self.spcAft().hashCode());
      }
      // Java: if (this.hasBuClr())
        if self.hasBuClr()
      {
         // Java: ret += this.buClr().hashCode();
         ret = ret.wrapping_add(self.buClr().hashCode());
      }
      // Java: if (this.hasBuSzPct())
        if self.hasBuSzPct()
      {
         // Java: ret += Double.valueOf(this.buSzPct()).hashCode();
         ret = ret.wrapping_add((self.buSzPct() as i64 ^ ((self.buSzPct() as i64) >> 32)) as i32);
      }
      // Java: if (this.hasBuSzPts())
        if self.hasBuSzPts()
      {
         // Java: ret += Integer.valueOf(this.buSzPts()).hashCode();
         ret = ret.wrapping_add(self.buSzPts());
      }
      // Java: if (this.hasBuFont())
        if self.hasBuFont()
      {
         // Java: ret += this.buFont().hashCode();
         ret = ret.wrapping_add(self.buFont().hashCode());
      }
      // Java: if (this.hasBuAutonNum())
        if self.hasBuAutonNum()
      {
         // Java: ret += this.buAutonNum().hashCode();
         ret = ret.wrapping_add(self.buAutonNum().hashCode());
      }
      // Java: if (this.hasBuChar())
        if self.hasBuChar()
      {
         // Java: ret += String.valueOf(this.buChar()).hashCode();
         let s = self.buChar();
         let mut h: i32 = 0;
         for c in s.chars() { h = h.wrapping_mul(31).wrapping_add(c as i32); }
         ret = ret.wrapping_add(h);
      }
      // Java: if (this.hasBuBlip())
        if self.hasBuBlip()
      {
         // Java: ret += this.buBlip().hashCode();
         ret = ret.wrapping_add(self.buBlip().hashCode());
      }
      // Java: if (this.hasTabStopLst())
        if self.hasTabStopLst()
      {
         // Java: ret += this.tabStopLst().hashCode();
         ret = ret.wrapping_add(self.tabStopLst().hashCode());
      }
      // Java: if (this.hasDefRPr())
        if self.hasDefRPr()
      {
         // Java: ret += this.defRPr().hashCode();
         ret = ret.wrapping_add(self.defRPr().hashCode());
      }
      // Java: if (this.hasMarginLeft())
        if self.hasMarginLeft()
      {
         // Java: ret += Integer.valueOf(this.marginLeft()).hashCode();
         ret = ret.wrapping_add(self.marginLeft());
      }
      // Java: if (this.hasMarginRight())
        if self.hasMarginRight()
      {
         // Java: ret += Integer.valueOf(this.marginRight()).hashCode();
         ret = ret.wrapping_add(self.marginRight());
      }
      // Java: if (this.hasLvl())
        if self.hasLvl()
      {
         // Java: ret += Integer.valueOf(this.lvl()).hashCode();
         ret = ret.wrapping_add(self.lvl());
      }
      // Java: if (this.hasIndent())
        if self.hasIndent()
      {
         // Java: ret += Integer.valueOf(this.indent()).hashCode();
         ret = ret.wrapping_add(self.indent());
      }
      // Java: if (this.hasAlignment())
        if self.hasAlignment()
      {
         // Java: ret += Integer.valueOf(this.alignment()).hashCode();
         ret = ret.wrapping_add(self.alignment());
      }
      // Java: if (this.hasDefTabSize())
        if self.hasDefTabSize()
      {
         // Java: ret += Integer.valueOf(this.defTabSize()).hashCode();
         ret = ret.wrapping_add(self.defTabSize());
      }
      // Java: if (this.hasFontAlgn())
        if self.hasFontAlgn()
      {
         // Java: ret += Integer.valueOf(this.fontAlgn()).hashCode();
         ret = ret.wrapping_add(self.fontAlgn());
      }
      // Java: if (this.hasOptions())
        if self.hasOptions()
      {
         // Java: ret += this.options().hashCode();
         ret = ret.wrapping_add(self.options().hashCode());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.lnSpc != null && this.lnSpc.dirty())
        if self.lnSpc.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.spcBef != null && this.spcBef.dirty())
      else if self.spcBef.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.spcAft != null && this.spcAft.dirty())
      else if self.spcAft.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.buClr != null && this.buClr.dirty())
      else if self.buClr.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.buFont != null && this.buFont.dirty())
      else if self.buFont.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.buAutonNum != null && this.buAutonNum.dirty())
      else if self.buAutonNum.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.buBlip != null && this.buBlip.dirty())
      else if self.buBlip.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.tabStopLst != null && this.tabStopLst.dirty())
      else if self.tabStopLst.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.defRPr != null && this.defRPr.dirty())
      else if self.defRPr.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
      else
      {
         // Java: return this.options != null && this.options.dirty() ? true : this.map.dirty();
         if self.options.as_ref().map_or(false, |s| s.dirty()) {
             return true;
         }
         self.map.dirty()
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
    fn cleanFollow(&mut self) {
      // Java: if (this.options != null)
        if let Some(ref mut opt) = self.options
      {
         // Java: this.options.unFollow();
         opt.unFollow();
      }
    }

    // Java: void cleanCache()
    fn cleanCache(&mut self) {
      // Java: this.lnSpc = null;
      self.lnSpc = None;
      // Java: this.spcBef = null;
      self.spcBef = None;
      // Java: this.spcAft = null;
      self.spcAft = None;
      // Java: this.buClr = null;
      self.buClr = None;
      // Java: this.buFont = null;
      self.buFont = None;
      // Java: this.buAutonNum = null;
      self.buAutonNum = None;
      // Java: this.buBlip = null;
      self.buBlip = None;
      // Java: this.tabStopLst = null;
      self.tabStopLst = None;
      // Java: this.defRPr = null;
      self.defRPr = None;
      // Java: this.options = null;
      self.options = None;
    }

    // Java: void follow(TextParagraphProperties parent)
    pub fn follow(&mut self, parent: Option<&mut TextParagraphProperties>) {
      // Java: if (parent != null)
        if let Some(p) = parent
      {
         // Java: parent.discard();
         let pzone = p.zone;
         p.discard_with_zone(pzone);
         // Java: this.discard();
         let zone = self.zone;
         self.discard_with_zone(zone);
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
         let zone = self.zone;
         self.discard_with_zone(zone);
         // Java: this.parent.discard();
         if let Some(ref mut p) = self.parent {
             let pzone = p.zone;
             p.discard_with_zone(pzone);
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

    // Java: TextParagraphProperties parent()
    pub fn parent(&self) -> Option<&TextParagraphProperties> {
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
    fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: TextSpacing lnSpc()
    pub fn lnSpc(&self) -> &TextSpacing {
      // Java: if (this.lnSpc == null)
        if self.lnSpc.is_none()
      {
         // Java: if (this.map.has(1))
            if self.map.has(1)
         {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: this.lnSpc = (TextSpacing)KoBin.Decode.toAny(TextSpacing.class, bin, this.zone);
            todo!()
         }
         // Java: else if (this.parent != null && this.parent.hasLnSpc())
            else if self.parent.as_ref().map_or(false, |p| p.hasLnSpc())
         {
            // Java: this.lnSpc = this.parent.lnSpc();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let null_bin = Null::empty();
            // Java: this.lnSpc = (TextSpacing)KoBin.Decode.toAny(TextSpacing.class, bin, this.zone);
            todo!()
         }
      }
      // Java: return this.lnSpc;
      self.lnSpc.as_ref().unwrap()
    }

    // Java: boolean hasLnSpc()
    pub fn hasLnSpc(&self) -> bool {
      // Java: return this.map.has(1) || this.lnSpc != null && this.lnSpc.dirty() || this.parent != null && this.parent.hasLnSpc();
      self.map.has(1) || self.lnSpc.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasLnSpc())
    }

    // Java: void deleteLnSpc()
    pub fn deleteLnSpc(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
      // Java: if (this.lnSpc != null)
        if self.lnSpc.is_some()
      {
         // Java: this.lnSpc = null;
         self.lnSpc = None;
      }
    }

    // Java: void writeLnSpc(TextSpacing value)
    pub fn writeLnSpc(&mut self, value: Option<&TextSpacing>) {
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
            // Java: if (this.lnSpc != null)
                if self.lnSpc.is_some()
            {
               // Java: this.lnSpc = null;
               self.lnSpc = None;
            }
         }
      }
    }

    // Java: void forceWriteLnSpc(TextSpacing value)
    pub fn forceWriteLnSpc(&mut self, value: Option<&TextSpacing>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         let bin = v.lastBin(self.zone);
         self.map.put(1, bin);
         // Java: if (this.lnSpc != null)
            if self.lnSpc.is_some()
         {
            // Java: this.lnSpc = null;
            self.lnSpc = None;
         }
      }
    }

    // Java: TextSpacing spcBef()
    pub fn spcBef(&self) -> &TextSpacing {
      // Java: if (this.spcBef == null)
        if self.spcBef.is_none()
      {
         // Java: if (this.map.has(2))
            if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: this.spcBef = (TextSpacing)KoBin.Decode.toAny(TextSpacing.class, bin, this.zone);
            todo!()
         }
         // Java: else if (this.parent != null && this.parent.hasSpcBef())
            else if self.parent.as_ref().map_or(false, |p| p.hasSpcBef())
         {
            // Java: this.spcBef = this.parent.spcBef();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let null_bin = Null::empty();
            // Java: this.spcBef = (TextSpacing)KoBin.Decode.toAny(TextSpacing.class, bin, this.zone);
            todo!()
         }
      }
      // Java: return this.spcBef;
      self.spcBef.as_ref().unwrap()
    }

    // Java: boolean hasSpcBef()
    pub fn hasSpcBef(&self) -> bool {
      // Java: return this.map.has(2) || this.spcBef != null && this.spcBef.dirty() || this.parent != null && this.parent.hasSpcBef();
      self.map.has(2) || self.spcBef.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasSpcBef())
    }

    // Java: void deleteSpcBef()
    pub fn deleteSpcBef(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.spcBef != null)
        if self.spcBef.is_some()
      {
         // Java: this.spcBef = null;
         self.spcBef = None;
      }
    }

    // Java: void writeSpcBef(TextSpacing value)
    pub fn writeSpcBef(&mut self, value: Option<&TextSpacing>) {
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
            // Java: if (this.spcBef != null)
                if self.spcBef.is_some()
            {
               // Java: this.spcBef = null;
               self.spcBef = None;
            }
         }
      }
    }

    // Java: void forceWriteSpcBef(TextSpacing value)
    pub fn forceWriteSpcBef(&mut self, value: Option<&TextSpacing>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         let bin = v.lastBin(self.zone);
         self.map.put(2, bin);
         // Java: if (this.spcBef != null)
            if self.spcBef.is_some()
         {
            // Java: this.spcBef = null;
            self.spcBef = None;
         }
      }
    }

    // Java: TextSpacing spcAft()
    pub fn spcAft(&self) -> &TextSpacing {
      // Java: if (this.spcAft == null)
        if self.spcAft.is_none()
      {
         // Java: if (this.map.has(3))
            if self.map.has(3)
         {
            // Java: KoBin bin = this.map.byKey(3);
            let bin = self.map.byKey(3);
            // Java: this.spcAft = (TextSpacing)KoBin.Decode.toAny(TextSpacing.class, bin, this.zone);
            todo!()
         }
         // Java: else if (this.parent != null && this.parent.hasSpcAft())
            else if self.parent.as_ref().map_or(false, |p| p.hasSpcAft())
         {
            // Java: this.spcAft = this.parent.spcAft();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let null_bin = Null::empty();
            // Java: this.spcAft = (TextSpacing)KoBin.Decode.toAny(TextSpacing.class, bin, this.zone);
            todo!()
         }
      }
      // Java: return this.spcAft;
      self.spcAft.as_ref().unwrap()
    }

    // Java: boolean hasSpcAft()
    pub fn hasSpcAft(&self) -> bool {
      // Java: return this.map.has(3) || this.spcAft != null && this.spcAft.dirty() || this.parent != null && this.parent.hasSpcAft();
      self.map.has(3) || self.spcAft.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasSpcAft())
    }

    // Java: void deleteSpcAft()
    pub fn deleteSpcAft(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
      // Java: if (this.spcAft != null)
        if self.spcAft.is_some()
      {
         // Java: this.spcAft = null;
         self.spcAft = None;
      }
    }

    // Java: void writeSpcAft(TextSpacing value)
    pub fn writeSpcAft(&mut self, value: Option<&TextSpacing>) {
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
            // Java: if (this.spcAft != null)
                if self.spcAft.is_some()
            {
               // Java: this.spcAft = null;
               self.spcAft = None;
            }
         }
      }
    }

    // Java: void forceWriteSpcAft(TextSpacing value)
    pub fn forceWriteSpcAft(&mut self, value: Option<&TextSpacing>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(3, value.lastBin(this.zone));
         let bin = v.lastBin(self.zone);
         self.map.put(3, bin);
         // Java: if (this.spcAft != null)
            if self.spcAft.is_some()
         {
            // Java: this.spcAft = null;
            self.spcAft = None;
         }
      }
    }

    // Java: Color buClr()
    pub fn buClr(&self) -> &Color {
      // Java: if (this.buClr == null)
        if self.buClr.is_none()
      {
         // Java: if (this.map.has(4))
            if self.map.has(4)
         {
            // Java: KoBin bin = this.map.byKey(4);
            let bin = self.map.byKey(4);
            // Java: this.buClr = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            todo!()
         }
         // Java: else if (this.parent != null && this.parent.hasBuClr())
            else if self.parent.as_ref().map_or(false, |p| p.hasBuClr())
         {
            // Java: this.buClr = this.parent.buClr();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let null_bin = Null::empty();
            // Java: this.buClr = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            todo!()
         }
      }
      // Java: return this.buClr;
      self.buClr.as_ref().unwrap()
    }

    // Java: boolean hasBuClr()
    pub fn hasBuClr(&self) -> bool {
      // Java: return this.map.has(4) || this.buClr != null && this.buClr.dirty() || this.parent != null && this.parent.hasBuClr();
      self.map.has(4) || self.buClr.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasBuClr())
    }

    // Java: void deleteBuClr()
    pub fn deleteBuClr(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
      // Java: if (this.buClr != null)
        if self.buClr.is_some()
      {
         // Java: this.buClr = null;
         self.buClr = None;
      }
    }

    // Java: void writeBuClr(Color value)
    pub fn writeBuClr(&mut self, value: Option<&Color>) {
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
            // Java: if (this.buClr != null)
                if self.buClr.is_some()
            {
               // Java: this.buClr = null;
               self.buClr = None;
            }
         }
      }
    }

    // Java: void forceWriteBuClr(Color value)
    pub fn forceWriteBuClr(&mut self, value: Option<&Color>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(4, value.lastBin(this.zone));
         let bin = v.lastBin(self.zone);
         self.map.put(4, bin);
         // Java: if (this.buClr != null)
            if self.buClr.is_some()
         {
            // Java: this.buClr = null;
            self.buClr = None;
         }
      }
    }

    // Java: double buSzPct()
    pub fn buSzPct(&self) -> f64 {
      // Java: if (!this.map.has(5))
        if !self.map.has(5)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.buSzPct();
         return if self.parent.is_none() { 0.0 } else { self.parent.as_ref().unwrap().buSzPct() };
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(5);
         let bin = self.map.byKey(5);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasBuSzPct()
    pub fn hasBuSzPct(&self) -> bool {
      // Java: return this.map.has(5) || this.parent != null && this.parent.hasBuSzPct();
      self.map.has(5) || self.parent.as_ref().map_or(false, |p| p.hasBuSzPct())
    }

    // Java: void deleteBuSzPct()
    pub fn deleteBuSzPct(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(5);
    }

    // Java: void putBuSzPct(double value)
    pub fn putBuSzPct(&mut self, value: f64) {
      // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_double(value, self.zone);
      self.map.put(5, bin);
    }

    // Java: int buSzPts()
    pub fn buSzPts(&self) -> i32 {
      // Java: if (!this.map.has(6))
        if !self.map.has(6)
      {
         // Java: return this.parent == null ? 0 : this.parent.buSzPts();
         return if self.parent.is_none() { 0 } else { self.parent.as_ref().unwrap().buSzPts() };
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(6);
         let bin = self.map.byKey(6);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasBuSzPts()
    pub fn hasBuSzPts(&self) -> bool {
      // Java: return this.map.has(6) || this.parent != null && this.parent.hasBuSzPts();
      self.map.has(6) || self.parent.as_ref().map_or(false, |p| p.hasBuSzPts())
    }

    // Java: void deleteBuSzPts()
    pub fn deleteBuSzPts(&mut self) {
      // Java: this.map.put(6, (KoBin)null);
      self.map.clear_key(6);
    }

    // Java: void putBuSzPts(int value)
    pub fn putBuSzPts(&mut self, value: i32) {
      // Java: this.map.put(6, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(6, bin);
    }

    // Java: TextFont buFont()
    pub fn buFont(&self) -> &TextFont {
      // Java: if (this.buFont == null)
        if self.buFont.is_none()
      {
         // Java: if (this.map.has(7))
            if self.map.has(7)
         {
            // Java: KoBin bin = this.map.byKey(7);
            let bin = self.map.byKey(7);
            // Java: this.buFont = (TextFont)KoBin.Decode.toAny(TextFont.class, bin, this.zone);
            todo!()
         }
         // Java: else if (this.parent != null && this.parent.hasBuFont())
            else if self.parent.as_ref().map_or(false, |p| p.hasBuFont())
         {
            // Java: this.buFont = this.parent.buFont();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let null_bin = Null::empty();
            // Java: this.buFont = (TextFont)KoBin.Decode.toAny(TextFont.class, bin, this.zone);
            todo!()
         }
      }
      // Java: return this.buFont;
      self.buFont.as_ref().unwrap()
    }

    // Java: boolean hasBuFont()
    pub fn hasBuFont(&self) -> bool {
      // Java: return this.map.has(7) || this.buFont != null && this.buFont.dirty() || this.parent != null && this.parent.hasBuFont();
      self.map.has(7) || self.buFont.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasBuFont())
    }

    // Java: void deleteBuFont()
    pub fn deleteBuFont(&mut self) {
      // Java: this.map.put(7, (KoBin)null);
      self.map.clear_key(7);
      // Java: if (this.buFont != null)
        if self.buFont.is_some()
      {
         // Java: this.buFont = null;
         self.buFont = None;
      }
    }

    // Java: void writeBuFont(TextFont value)
    pub fn writeBuFont(&mut self, value: Option<&TextFont>) {
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
            // Java: if (this.buFont != null)
                if self.buFont.is_some()
            {
               // Java: this.buFont = null;
               self.buFont = None;
            }
         }
      }
    }

    // Java: void forceWriteBuFont(TextFont value)
    pub fn forceWriteBuFont(&mut self, value: Option<&TextFont>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(7, value.lastBin(this.zone));
         let bin = v.lastBin(self.zone);
         self.map.put(7, bin);
         // Java: if (this.buFont != null)
            if self.buFont.is_some()
         {
            // Java: this.buFont = null;
            self.buFont = None;
         }
      }
    }

    // Java: TextAutonumberBullet buAutonNum()
    pub fn buAutonNum(&self) -> &TextAutonumberBullet {
      // Java: if (this.buAutonNum == null)
        if self.buAutonNum.is_none()
      {
         // Java: if (this.map.has(8))
            if self.map.has(8)
         {
            // Java: KoBin bin = this.map.byKey(8);
            let bin = self.map.byKey(8);
            // Java: this.buAutonNum = (TextAutonumberBullet)KoBin.Decode.toAny(TextAutonumberBullet.class, bin, this.zone);
            todo!()
         }
         // Java: else if (this.parent != null && this.parent.hasBuAutonNum())
            else if self.parent.as_ref().map_or(false, |p| p.hasBuAutonNum())
         {
            // Java: this.buAutonNum = this.parent.buAutonNum();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let null_bin = Null::empty();
            // Java: this.buAutonNum = (TextAutonumberBullet)KoBin.Decode.toAny(TextAutonumberBullet.class, bin, this.zone);
            todo!()
         }
      }
      // Java: return this.buAutonNum;
      self.buAutonNum.as_ref().unwrap()
    }

    // Java: boolean hasBuAutonNum()
    pub fn hasBuAutonNum(&self) -> bool {
      // Java: return this.map.has(8) || this.buAutonNum != null && this.buAutonNum.dirty() || this.parent != null && this.parent.hasBuAutonNum();
      self.map.has(8) || self.buAutonNum.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasBuAutonNum())
    }

    // Java: void deleteBuAutonNum()
    pub fn deleteBuAutonNum(&mut self) {
      // Java: this.map.put(8, (KoBin)null);
      self.map.clear_key(8);
      // Java: if (this.buAutonNum != null)
        if self.buAutonNum.is_some()
      {
         // Java: this.buAutonNum = null;
         self.buAutonNum = None;
      }
    }

    // Java: void writeBuAutonNum(TextAutonumberBullet value)
    pub fn writeBuAutonNum(&mut self, value: Option<&TextAutonumberBullet>) {
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
            // Java: if (this.buAutonNum != null)
                if self.buAutonNum.is_some()
            {
               // Java: this.buAutonNum = null;
               self.buAutonNum = None;
            }
         }
      }
    }

    // Java: void forceWriteBuAutonNum(TextAutonumberBullet value)
    pub fn forceWriteBuAutonNum(&mut self, value: Option<&TextAutonumberBullet>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(8, value.lastBin(this.zone));
         let bin = v.lastBin(self.zone);
         self.map.put(8, bin);
         // Java: if (this.buAutonNum != null)
            if self.buAutonNum.is_some()
         {
            // Java: this.buAutonNum = null;
            self.buAutonNum = None;
         }
      }
    }

    // Java: String buChar()
    pub fn buChar(&self) -> String {
      // Java: if (!this.map.has(9))
        if !self.map.has(9)
      {
         // Java: return this.parent == null ? "" : this.parent.buChar();
         return if self.parent.is_none() { String::new() } else { self.parent.as_ref().unwrap().buChar() };
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(9);
         let bin = self.map.byKey(9);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasBuChar()
    pub fn hasBuChar(&self) -> bool {
      // Java: return this.map.has(9) || this.parent != null && this.parent.hasBuChar();
      self.map.has(9) || self.parent.as_ref().map_or(false, |p| p.hasBuChar())
    }

    // Java: void deleteBuChar()
    pub fn deleteBuChar(&mut self) {
      // Java: this.map.put(9, (KoBin)null);
      self.map.clear_key(9);
    }

    // Java: void putBuChar(String value)
    pub fn putBuChar(&mut self, value: &str) {
      // Java: this.map.put(9, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_string(value, self.zone);
      self.map.put(9, bin);
    }

    // Java: Blip buBlip()
    pub fn buBlip(&self) -> &Blip {
      // Java: if (this.buBlip == null)
        if self.buBlip.is_none()
      {
         // Java: if (this.map.has(10))
            if self.map.has(10)
         {
            // Java: KoBin bin = this.map.byKey(10);
            let bin = self.map.byKey(10);
            // Java: this.buBlip = (Blip)KoBin.Decode.toAny(Blip.class, bin, this.zone);
            todo!()
         }
         // Java: else if (this.parent != null && this.parent.hasBuBlip())
            else if self.parent.as_ref().map_or(false, |p| p.hasBuBlip())
         {
            // Java: this.buBlip = this.parent.buBlip();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let null_bin = Null::empty();
            // Java: this.buBlip = (Blip)KoBin.Decode.toAny(Blip.class, bin, this.zone);
            todo!()
         }
      }
      // Java: return this.buBlip;
      self.buBlip.as_ref().unwrap()
    }

    // Java: boolean hasBuBlip()
    pub fn hasBuBlip(&self) -> bool {
      // Java: return this.map.has(10) || this.buBlip != null && this.buBlip.dirty() || this.parent != null && this.parent.hasBuBlip();
      self.map.has(10) || self.buBlip.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasBuBlip())
    }

    // Java: void deleteBuBlip()
    pub fn deleteBuBlip(&mut self) {
      // Java: this.map.put(10, (KoBin)null);
      self.map.clear_key(10);
      // Java: if (this.buBlip != null)
        if self.buBlip.is_some()
      {
         // Java: this.buBlip = null;
         self.buBlip = None;
      }
    }

    // Java: void writeBuBlip(Blip value)
    pub fn writeBuBlip(&mut self, value: Option<&Blip>) {
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
            // Java: if (this.buBlip != null)
                if self.buBlip.is_some()
            {
               // Java: this.buBlip = null;
               self.buBlip = None;
            }
         }
      }
    }

    // Java: void forceWriteBuBlip(Blip value)
    pub fn forceWriteBuBlip(&mut self, value: Option<&Blip>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(10, value.lastBin(this.zone));
         let bin = v.lastBin(self.zone);
         self.map.put(10, bin);
         // Java: if (this.buBlip != null)
            if self.buBlip.is_some()
         {
            // Java: this.buBlip = null;
            self.buBlip = None;
         }
      }
    }

    // Java: TabStopList tabStopLst()
    pub fn tabStopLst(&self) -> &TabStopList {
      // Java: if (this.tabStopLst == null)
        if self.tabStopLst.is_none()
      {
         // Java: if (this.map.has(11))
            if self.map.has(11)
         {
            // Java: KoBin bin = this.map.byKey(11);
            let bin = self.map.byKey(11);
            // Java: this.tabStopLst = (TabStopList)KoBin.Decode.toAny(TabStopList.class, bin, this.zone);
            todo!()
         }
         // Java: else if (this.parent != null && this.parent.hasTabStopLst())
            else if self.parent.as_ref().map_or(false, |p| p.hasTabStopLst())
         {
            // Java: this.tabStopLst = this.parent.tabStopLst();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let null_bin = Null::empty();
            // Java: this.tabStopLst = (TabStopList)KoBin.Decode.toAny(TabStopList.class, bin, this.zone);
            todo!()
         }
      }
      // Java: return this.tabStopLst;
      self.tabStopLst.as_ref().unwrap()
    }

    // Java: boolean hasTabStopLst()
    pub fn hasTabStopLst(&self) -> bool {
      // Java: return this.map.has(11) || this.tabStopLst != null && this.tabStopLst.dirty() || this.parent != null && this.parent.hasTabStopLst();
      self.map.has(11) || self.tabStopLst.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasTabStopLst())
    }

    // Java: void deleteTabStopLst()
    pub fn deleteTabStopLst(&mut self) {
      // Java: this.map.put(11, (KoBin)null);
      self.map.clear_key(11);
      // Java: if (this.tabStopLst != null)
        if self.tabStopLst.is_some()
      {
         // Java: this.tabStopLst = null;
         self.tabStopLst = None;
      }
    }

    // Java: void writeTabStopLst(TabStopList value)
    pub fn writeTabStopLst(&mut self, value: Option<&TabStopList>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(11, bin);
            self.map.put(11, bin);
            // Java: if (this.tabStopLst != null)
                if self.tabStopLst.is_some()
            {
               // Java: this.tabStopLst = null;
               self.tabStopLst = None;
            }
         }
      }
    }

    // Java: void forceWriteTabStopLst(TabStopList value)
    pub fn forceWriteTabStopLst(&mut self, value: Option<&TabStopList>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(11, value.lastBin(this.zone));
         let bin = v.lastBin(self.zone);
         self.map.put(11, bin);
         // Java: if (this.tabStopLst != null)
            if self.tabStopLst.is_some()
         {
            // Java: this.tabStopLst = null;
            self.tabStopLst = None;
         }
      }
    }

    // Java: TextCharacterProperties defRPr()
    pub fn defRPr(&self) -> &TextCharacterProperties {
      // Java: if (this.defRPr == null)
        if self.defRPr.is_none()
      {
         // Java: if (this.map.has(12))
            if self.map.has(12)
         {
            // Java: KoBin bin = this.map.byKey(12);
            let bin = self.map.byKey(12);
            // Java: this.defRPr = (TextCharacterProperties)KoBin.Decode.toAny(TextCharacterProperties.class, bin, this.zone);
            todo!()
         }
         // Java: else if (this.parent != null && this.parent.hasDefRPr())
            else if self.parent.as_ref().map_or(false, |p| p.hasDefRPr())
         {
            // Java: this.defRPr = this.parent.defRPr();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let null_bin = Null::empty();
            // Java: this.defRPr = (TextCharacterProperties)KoBin.Decode.toAny(TextCharacterProperties.class, bin, this.zone);
            todo!()
         }
      }
      // Java: return this.defRPr;
      self.defRPr.as_ref().unwrap()
    }

    // Java: boolean hasDefRPr()
    pub fn hasDefRPr(&self) -> bool {
      // Java: return this.map.has(12) || this.defRPr != null && this.defRPr.dirty() || this.parent != null && this.parent.hasDefRPr();
      self.map.has(12) || self.defRPr.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasDefRPr())
    }

    // Java: void deleteDefRPr()
    pub fn deleteDefRPr(&mut self) {
      // Java: this.map.put(12, (KoBin)null);
      self.map.clear_key(12);
      // Java: if (this.defRPr != null)
        if self.defRPr.is_some()
      {
         // Java: this.defRPr = null;
         self.defRPr = None;
      }
    }

    // Java: void writeDefRPr(TextCharacterProperties value)
    pub fn writeDefRPr(&mut self, value: Option<&TextCharacterProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(12, bin);
            self.map.put(12, bin);
            // Java: if (this.defRPr != null)
                if self.defRPr.is_some()
            {
               // Java: this.defRPr = null;
               self.defRPr = None;
            }
         }
      }
    }

    // Java: void forceWriteDefRPr(TextCharacterProperties value)
    pub fn forceWriteDefRPr(&mut self, value: Option<&TextCharacterProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(12, value.lastBin(this.zone));
         let bin = v.lastBin(self.zone);
         self.map.put(12, bin);
         // Java: if (this.defRPr != null)
            if self.defRPr.is_some()
         {
            // Java: this.defRPr = null;
            self.defRPr = None;
         }
      }
    }

    // Java: int marginLeft()
    pub fn marginLeft(&self) -> i32 {
      // Java: if (!this.map.has(13))
        if !self.map.has(13)
      {
         // Java: return this.parent == null ? 347663 : this.parent.marginLeft();
         return if self.parent.is_none() { 347663 } else { self.parent.as_ref().unwrap().marginLeft() };
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(13);
         let bin = self.map.byKey(13);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasMarginLeft()
    pub fn hasMarginLeft(&self) -> bool {
      // Java: return this.map.has(13) || this.parent != null && this.parent.hasMarginLeft();
      self.map.has(13) || self.parent.as_ref().map_or(false, |p| p.hasMarginLeft())
    }

    // Java: void deleteMarginLeft()
    pub fn deleteMarginLeft(&mut self) {
      // Java: this.map.put(13, (KoBin)null);
      self.map.clear_key(13);
    }

    // Java: void putMarginLeft(int value)
    pub fn putMarginLeft(&mut self, value: i32) {
      // Java: this.map.put(13, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(13, bin);
    }

    // Java: int marginRight()
    pub fn marginRight(&self) -> i32 {
      // Java: if (!this.map.has(14))
        if !self.map.has(14)
      {
         // Java: return this.parent == null ? 0 : this.parent.marginRight();
         return if self.parent.is_none() { 0 } else { self.parent.as_ref().unwrap().marginRight() };
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(14);
         let bin = self.map.byKey(14);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasMarginRight()
    pub fn hasMarginRight(&self) -> bool {
      // Java: return this.map.has(14) || this.parent != null && this.parent.hasMarginRight();
      self.map.has(14) || self.parent.as_ref().map_or(false, |p| p.hasMarginRight())
    }

    // Java: void deleteMarginRight()
    pub fn deleteMarginRight(&mut self) {
      // Java: this.map.put(14, (KoBin)null);
      self.map.clear_key(14);
    }

    // Java: void putMarginRight(int value)
    pub fn putMarginRight(&mut self, value: i32) {
      // Java: this.map.put(14, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(14, bin);
    }

    // Java: int lvl()
    pub fn lvl(&self) -> i32 {
      // Java: if (!this.map.has(15))
        if !self.map.has(15)
      {
         // Java: return this.parent == null ? 0 : this.parent.lvl();
         return if self.parent.is_none() { 0 } else { self.parent.as_ref().unwrap().lvl() };
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(15);
         let bin = self.map.byKey(15);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasLvl()
    pub fn hasLvl(&self) -> bool {
      // Java: return this.map.has(15) || this.parent != null && this.parent.hasLvl();
      self.map.has(15) || self.parent.as_ref().map_or(false, |p| p.hasLvl())
    }

    // Java: void deleteLvl()
    pub fn deleteLvl(&mut self) {
      // Java: this.map.put(15, (KoBin)null);
      self.map.clear_key(15);
    }

    // Java: void putLvl(int value)
    pub fn putLvl(&mut self, value: i32) {
      // Java: this.map.put(15, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(15, bin);
    }

    // Java: int indent()
    pub fn indent(&self) -> i32 {
      // Java: if (!this.map.has(16))
        if !self.map.has(16)
      {
         // Java: return this.parent == null ? -342900 : this.parent.indent();
         return if self.parent.is_none() { -342900 } else { self.parent.as_ref().unwrap().indent() };
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(16);
         let bin = self.map.byKey(16);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasIndent()
    pub fn hasIndent(&self) -> bool {
      // Java: return this.map.has(16) || this.parent != null && this.parent.hasIndent();
      self.map.has(16) || self.parent.as_ref().map_or(false, |p| p.hasIndent())
    }

    // Java: void deleteIndent()
    pub fn deleteIndent(&mut self) {
      // Java: this.map.put(16, (KoBin)null);
      self.map.clear_key(16);
    }

    // Java: void putIndent(int value)
    pub fn putIndent(&mut self, value: i32) {
      // Java: this.map.put(16, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(16, bin);
    }

    // Java: int alignment()
    pub fn alignment(&self) -> i32 {
      // Java: if (!this.map.has(17))
        if !self.map.has(17)
      {
         // Java: return this.parent == null ? 0 : this.parent.alignment();
         return if self.parent.is_none() { 0 } else { self.parent.as_ref().unwrap().alignment() };
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(17);
         let bin = self.map.byKey(17);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasAlignment()
    pub fn hasAlignment(&self) -> bool {
      // Java: return this.map.has(17) || this.parent != null && this.parent.hasAlignment();
      self.map.has(17) || self.parent.as_ref().map_or(false, |p| p.hasAlignment())
    }

    // Java: void deleteAlignment()
    pub fn deleteAlignment(&mut self) {
      // Java: this.map.put(17, (KoBin)null);
      self.map.clear_key(17);
    }

    // Java: void putAlignment(int value)
    pub fn putAlignment(&mut self, value: i32) {
      // Java: this.map.put(17, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(17, bin);
    }

    // Java: int defTabSize()
    pub fn defTabSize(&self) -> i32 {
      // Java: if (!this.map.has(18))
        if !self.map.has(18)
      {
         // Java: return this.parent == null ? 0 : this.parent.defTabSize();
         return if self.parent.is_none() { 0 } else { self.parent.as_ref().unwrap().defTabSize() };
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(18);
         let bin = self.map.byKey(18);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasDefTabSize()
    pub fn hasDefTabSize(&self) -> bool {
      // Java: return this.map.has(18) || this.parent != null && this.parent.hasDefTabSize();
      self.map.has(18) || self.parent.as_ref().map_or(false, |p| p.hasDefTabSize())
    }

    // Java: void deleteDefTabSize()
    pub fn deleteDefTabSize(&mut self) {
      // Java: this.map.put(18, (KoBin)null);
      self.map.clear_key(18);
    }

    // Java: void putDefTabSize(int value)
    pub fn putDefTabSize(&mut self, value: i32) {
      // Java: this.map.put(18, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(18, bin);
    }

    // Java: int fontAlgn()
    pub fn fontAlgn(&self) -> i32 {
      // Java: if (!this.map.has(19))
        if !self.map.has(19)
      {
         // Java: return this.parent == null ? 0 : this.parent.fontAlgn();
         return if self.parent.is_none() { 0 } else { self.parent.as_ref().unwrap().fontAlgn() };
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(19);
         let bin = self.map.byKey(19);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasFontAlgn()
    pub fn hasFontAlgn(&self) -> bool {
      // Java: return this.map.has(19) || this.parent != null && this.parent.hasFontAlgn();
      self.map.has(19) || self.parent.as_ref().map_or(false, |p| p.hasFontAlgn())
    }

    // Java: void deleteFontAlgn()
    pub fn deleteFontAlgn(&mut self) {
      // Java: this.map.put(19, (KoBin)null);
      self.map.clear_key(19);
    }

    // Java: void putFontAlgn(int value)
    pub fn putFontAlgn(&mut self, value: i32) {
      // Java: this.map.put(19, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(19, bin);
    }

    // Java: TextParaOptions options()
    pub fn options(&self) -> &TextParaOptions {
      // Java: if (this.options == null)
        if self.options.is_none()
      {
         // Java: KoBin bin = this.map.byKey(20);
         let bin = self.map.byKey(20);
         // Java: this.options = (TextParaOptions)KoBin.Decode.toAny(TextParaOptions.class, bin, this.zone);
         todo!()
      }
      // Java: if (this.parent != null && this.options.parent() == null && this.parent.hasOptions())
        if self.parent.is_some() && self.options.as_ref().unwrap().parent().is_none() && self.parent.as_ref().unwrap().hasOptions()
      {
         // Java: this.options.follow(this.parent.options());
         todo!()
      }
      // Java: return this.options;
      self.options.as_ref().unwrap()
    }

    // Java: boolean hasOptions()
    pub fn hasOptions(&self) -> bool {
      // Java: return this.map.has(20) || this.options != null && this.options.dirty() || this.parent != null && this.parent.hasOptions();
      self.map.has(20) || self.options.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasOptions())
    }

    // Java: void deleteOptions()
    pub fn deleteOptions(&mut self) {
      // Java: this.map.put(20, (KoBin)null);
      self.map.clear_key(20);
      // Java: if (this.options != null)
        if self.options.is_some()
      {
         // Java: this.options = null;
         self.options = None;
      }
    }

    // Java: void writeOptions(TextParaOptions value)
    pub fn writeOptions(&mut self, value: Option<&TextParaOptions>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(20, bin);
            self.map.put(20, bin);
            // Java: if (this.options != null)
                if self.options.is_some()
            {
               // Java: this.options = null;
               self.options = None;
            }
         }
      }
    }

    // Java: void forceWriteOptions(TextParaOptions value)
    pub fn forceWriteOptions(&mut self, value: Option<&TextParaOptions>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(20, value.lastBin(this.zone));
         let bin = v.lastBin(self.zone);
         self.map.put(20, bin);
         // Java: if (this.options != null)
            if self.options.is_some()
         {
            // Java: this.options = null;
            self.options = None;
         }
      }
    }
}

impl KoAr for TextParagraphProperties {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for TextParagraphProperties {
    fn close(&mut self) { self.close(); }
}

// ============================================================
// Inner class: TextSpacing
// ============================================================

// Java: public class TextSpacing implements KoAr, KoClosable
impl TextSpacing {
    // Java: static protected static <Value1 extends TextSpacing> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::TextSpacing_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static TextSpacing create()
    pub fn create() -> Self {
      // Java: return initialize(TextSpacing.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static TextSpacing open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(TextSpacing.class, bin);
      Self::initialize(bin)
    }

    // Java: TextSpacing()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Self { bin_box: KpBinBox::new(), map: KpMap::new(), zone: KoZone::defaultZone(), parent: None }
    }

    // Java: TextSpacing(KoBin bin)
    pub fn TextSpacing_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Self { bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), zone: KoZone::defaultZone(), parent: None }
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

    // Java: void load(TextSpacing value)
    pub fn load_value(&mut self, value: Option<&TextSpacing>) {
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
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(TextSpacing o)
    pub fn equalsLax(&self, o: &TextSpacing) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TextSpacing) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &TextSpacing, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof TextSpacing)
        if true
      {
         // Java: TextSpacing dst = (TextSpacing)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasSpcPct() || !dst.hasSpcPct()) && (!this.hasSpcPct() || dst.hasSpcPct()))
            else if (self.hasSpcPct() || !dst.hasSpcPct()) && (!self.hasSpcPct() || dst.hasSpcPct())
         {
            // Java: if (this.hasSpcPct() && dst.hasSpcPct() && this.spcPct() != dst.spcPct())
                if self.hasSpcPct() && dst.hasSpcPct() && self.spcPct() != dst.spcPct()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasSpcPts() || !dst.hasSpcPts()) && (!this.hasSpcPts() || dst.hasSpcPts()))
                else if (self.hasSpcPts() || !dst.hasSpcPts()) && (!self.hasSpcPts() || dst.hasSpcPts())
            {
               // Java: return !this.hasSpcPts() || !dst.hasSpcPts() || this.spcPts() == dst.spcPts();
               return !self.hasSpcPts() || !dst.hasSpcPts() || self.spcPts() == dst.spcPts();
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
      // Java: if (this.hasSpcPct())
        if self.hasSpcPct()
      {
         // Java: ret += Double.valueOf(this.spcPct()).hashCode();
         ret = ret.wrapping_add((self.spcPct().to_bits() as i64 ^ ((self.spcPct().to_bits() as i64) >> 32)) as i32);
      }
      // Java: if (this.hasSpcPts())
        if self.hasSpcPts()
      {
         // Java: ret += Double.valueOf(this.spcPts()).hashCode();
         ret = ret.wrapping_add((self.spcPts().to_bits() as i64 ^ ((self.spcPts().to_bits() as i64) >> 32)) as i32);
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
    fn cleanCache(&mut self) {}

    // Java: void follow(TextSpacing parent)
    pub fn follow(&mut self, parent: Option<&mut TextSpacing>) {
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

    // Java: TextSpacing parent()
    pub fn parent(&self) -> Option<&TextSpacing> {
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

    // Java: double spcPct()
    pub fn spcPct(&self) -> f64 {
      // Java: if (!this.map.has(1))
        if !self.map.has(1)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.spcPct();
         return if self.parent.is_none() { 0.0 } else { self.parent.as_ref().unwrap().spcPct() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(1);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasSpcPct()
    pub fn hasSpcPct(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasSpcPct();
      self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasSpcPct())
    }

    // Java: void deleteSpcPct()
    pub fn deleteSpcPct(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
    }

    // Java: void putSpcPct(double value)
    pub fn putSpcPct(&mut self, value: f64) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_double(value, self.zone);
      self.map.put(1, bin);
    }

    // Java: double spcPts()
    pub fn spcPts(&self) -> f64 {
      // Java: if (!this.map.has(2))
        if !self.map.has(2)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.spcPts();
         return if self.parent.is_none() { 0.0 } else { self.parent.as_ref().unwrap().spcPts() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(2);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasSpcPts()
    pub fn hasSpcPts(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasSpcPts();
      self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasSpcPts())
    }

    // Java: void deleteSpcPts()
    pub fn deleteSpcPts(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
    }

    // Java: void putSpcPts(double value)
    pub fn putSpcPts(&mut self, value: f64) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_double(value, self.zone);
      self.map.put(2, bin);
    }

}

impl KoAr for TextSpacing {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for TextSpacing { fn close(&mut self) { self.close(); } }

// ============================================================
// Inner class: TextAutonumberBullet
// ============================================================

impl TextAutonumberBullet {
    // Java: static protected static <Value1 extends TextAutonumberBullet> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::TextAutonumberBullet_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static TextAutonumberBullet create()
    pub fn create() -> Self {
      // Java: return initialize(TextAutonumberBullet.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static TextAutonumberBullet open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(TextAutonumberBullet.class, bin);
      Self::initialize(bin)
    }

    // Java: TextAutonumberBullet()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Self { bin_box: KpBinBox::new(), map: KpMap::new(), zone: KoZone::defaultZone(), parent: None }
    }

    // Java: TextAutonumberBullet(KoBin bin)
    pub fn TextAutonumberBullet_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Self { bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), zone: KoZone::defaultZone(), parent: None }
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

    // Java: void load(TextAutonumberBullet value)
    pub fn load_value(&mut self, value: Option<&TextAutonumberBullet>) {
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
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(TextAutonumberBullet o)
    pub fn equalsLax(&self, o: &TextAutonumberBullet) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TextAutonumberBullet) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &TextAutonumberBullet, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof TextAutonumberBullet)
        if true
      {
         // Java: TextAutonumberBullet dst = (TextAutonumberBullet)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasStartAt() || !dst.hasStartAt()) && (!this.hasStartAt() || dst.hasStartAt()))
            else if (self.hasStartAt() || !dst.hasStartAt()) && (!self.hasStartAt() || dst.hasStartAt())
         {
            // Java: if (this.hasStartAt() && dst.hasStartAt() && this.startAt() != dst.startAt())
                if self.hasStartAt() && dst.hasStartAt() && self.startAt() != dst.startAt()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasType() || !dst.hasType()) && (!this.hasType() || dst.hasType()))
                else if (self.hasType() || !dst.hasType()) && (!self.hasType() || dst.hasType())
            {
               // Java: return !this.hasType() || !dst.hasType() || this.type() == dst.type();
               return !self.hasType() || !dst.hasType() || self.r#type() == dst.r#type();
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
      // Java: if (this.hasStartAt())
        if self.hasStartAt()
      {
         // Java: ret += Integer.valueOf(this.startAt()).hashCode();
         ret = ret.wrapping_add(self.startAt());
      }
      // Java: if (this.hasType())
        if self.hasType()
      {
         // Java: ret += Integer.valueOf(this.type()).hashCode();
         ret = ret.wrapping_add(self.r#type());
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
    fn cleanCache(&mut self) {}

    // Java: void follow(TextAutonumberBullet parent)
    pub fn follow(&mut self, parent: Option<&mut TextAutonumberBullet>) {
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

    // Java: TextAutonumberBullet parent()
    pub fn parent(&self) -> Option<&TextAutonumberBullet> {
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

    // Java: int startAt()
    pub fn startAt(&self) -> i32 {
      // Java: if (!this.map.has(1))
        if !self.map.has(1)
      {
         // Java: return this.parent == null ? 1 : this.parent.startAt();
         return if self.parent.is_none() { 1 } else { self.parent.as_ref().unwrap().startAt() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(1);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasStartAt()
    pub fn hasStartAt(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasStartAt();
      self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasStartAt())
    }

    // Java: void deleteStartAt()
    pub fn deleteStartAt(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
    }

    // Java: void putStartAt(int value)
    pub fn putStartAt(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(1, bin);
    }

    // Java: int type()
    pub fn r#type(&self) -> i32 {
      // Java: if (!this.map.has(2))
        if !self.map.has(2)
      {
         // Java: return this.parent == null ? 8 : this.parent.type();
         return if self.parent.is_none() { 8 } else { self.parent.as_ref().unwrap().r#type() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(2);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasType()
    pub fn hasType(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasType();
      self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasType())
    }

    // Java: void deleteType()
    pub fn deleteType(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
    }

    // Java: void putType(int value)
    pub fn putType(&mut self, value: i32) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(2, bin);
    }

}

impl KoAr for TextAutonumberBullet {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for TextAutonumberBullet { fn close(&mut self) { self.close(); } }

// ============================================================
// Inner class: TextTabStop
// ============================================================

impl TextTabStop {
    // Java: static protected static <Value1 extends TextTabStop> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::TextTabStop_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static TextTabStop create()
    pub fn create() -> Self {
      // Java: return initialize(TextTabStop.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static TextTabStop open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(TextTabStop.class, bin);
      Self::initialize(bin)
    }

    // Java: TextTabStop()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Self { bin_box: KpBinBox::new(), map: KpMap::new(), zone: KoZone::defaultZone(), parent: None }
    }

    // Java: TextTabStop(KoBin bin)
    pub fn TextTabStop_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Self { bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), zone: KoZone::defaultZone(), parent: None }
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

    // Java: void load(TextTabStop value)
    pub fn load_value(&mut self, value: Option<&TextTabStop>) {
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
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(TextTabStop o)
    pub fn equalsLax(&self, o: &TextTabStop) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TextTabStop) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &TextTabStop, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof TextTabStop)
        if true
      {
         // Java: TextTabStop dst = (TextTabStop)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasPos() || !dst.hasPos()) && (!this.hasPos() || dst.hasPos()))
            else if (self.hasPos() || !dst.hasPos()) && (!self.hasPos() || dst.hasPos())
         {
            // Java: if (this.hasPos() && dst.hasPos() && this.pos() != dst.pos())
                if self.hasPos() && dst.hasPos() && self.pos() != dst.pos()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasAlignment() || !dst.hasAlignment()) && (!this.hasAlignment() || dst.hasAlignment()))
                else if (self.hasAlignment() || !dst.hasAlignment()) && (!self.hasAlignment() || dst.hasAlignment())
            {
               // Java: return !this.hasAlignment() || !dst.hasAlignment() || this.alignment() == dst.alignment();
               return !self.hasAlignment() || !dst.hasAlignment() || self.alignment() == dst.alignment();
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
      // Java: if (this.hasPos())
        if self.hasPos()
      {
         // Java: ret += Integer.valueOf(this.pos()).hashCode();
         ret = ret.wrapping_add(self.pos());
      }
      // Java: if (this.hasAlignment())
        if self.hasAlignment()
      {
         // Java: ret += Integer.valueOf(this.alignment()).hashCode();
         ret = ret.wrapping_add(self.alignment());
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
    fn cleanCache(&mut self) {}

    // Java: void follow(TextTabStop parent)
    pub fn follow(&mut self, parent: Option<&mut TextTabStop>) {
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

    // Java: TextTabStop parent()
    pub fn parent(&self) -> Option<&TextTabStop> {
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

    // Java: int pos()
    pub fn pos(&self) -> i32 {
      // Java: if (!this.map.has(1))
        if !self.map.has(1)
      {
         // Java: return this.parent == null ? 0 : this.parent.pos();
         return if self.parent.is_none() { 0 } else { self.parent.as_ref().unwrap().pos() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(1);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasPos()
    pub fn hasPos(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasPos();
      self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasPos())
    }

    // Java: void deletePos()
    pub fn deletePos(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
    }

    // Java: void putPos(int value)
    pub fn putPos(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(1, bin);
    }

    // Java: int alignment()
    pub fn alignment(&self) -> i32 {
      // Java: if (!this.map.has(2))
        if !self.map.has(2)
      {
         // Java: return this.parent == null ? 1 : this.parent.alignment();
         return if self.parent.is_none() { 1 } else { self.parent.as_ref().unwrap().alignment() };
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(2);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasAlignment()
    pub fn hasAlignment(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasAlignment();
      self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasAlignment())
    }

    // Java: void deleteAlignment()
    pub fn deleteAlignment(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
    }

    // Java: void putAlignment(int value)
    pub fn putAlignment(&mut self, value: i32) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      let bin = Encode::from_int(value, self.zone);
      self.map.put(2, bin);
    }

}

impl KoAr for TextTabStop {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for TextTabStop { fn close(&mut self) { self.close(); } }

// ============================================================
// Inner class: TabStopList
// ============================================================

impl TabStopList {
    // Java: TabStopList()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.list = new KpList(this.zone);
      let zone = KoZone::defaultZone();
      Self { bin_box: KpBinBox::new(), list: KpList::new(zone), zone }
    }

    // Java: TabStopList(KoBin bin)
    pub fn TabStopList_from(bin: &dyn KoBinTrait) -> Self {
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

    // Java: void load(TabStopList value)
    pub fn load_value(&mut self, value: Option<&TabStopList>) {
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

    // Java: boolean equalsLax(TabStopList o)
    pub fn equalsLax(&self, o: &TabStopList) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TabStopList) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &TabStopList, isStrict: bool) -> bool {
      // Java: if (null == o)
      // Java: return false;
      // Java: else
      {
         // Java: TabStopList dst = (TabStopList)o;
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
            // Java: ArrayList<TextTabStop> dstList = new ArrayList();
            let mut dstList: Vec<TextTabStop> = Vec::new();
            // Java: dst.read(dstList);
            dst.read(&mut dstList);
            // Java: ArrayList<TextTabStop> srcList = new ArrayList();
            let mut srcList: Vec<TextTabStop> = Vec::new();
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
                  // Java: TextTabStop dstClrTrans = (TextTabStop)dstList.get(i);
                  let dstClrTrans = &dstList[i];
                  // Java: TextTabStop srcClrTrans = (TextTabStop)srcList.get(i);
                  let srcClrTrans = &srcList[i];
                  // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                     if isStrict && !dstClrTrans.equals(srcClrTrans)
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                     if !isStrict && !dstClrTrans.equalsLax(srcClrTrans)
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

    // Java: static protected static <Value1 extends TabStopList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::TabStopList_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static TabStopList create()
    pub fn create() -> Self {
      // Java: return initialize(TabStopList.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static TabStopList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(TabStopList.class, bin);
      Self::initialize(bin)
    }

    // Java: void read(Collection<TextTabStop> valueLst)
    pub fn read(&self, valueLst: &mut Vec<TextTabStop>) {
      // Java: valueLst.clear();
      valueLst.clear();
      // Java: int size = this.list.size();
      let size = self.list.size();
      // Java: for(int i = 0; i < size; ++i)
        for i in 0..size
      {
         // Java: valueLst.add((TextTabStop)KoBin.Decode.toAny(TextTabStop.class, this.list.get(i), this.zone));
         todo!()
      }
    }

    // Java: void write(Iterable<TextTabStop> valueLst)
    pub fn write(&mut self, valueLst: &[TextTabStop]) {
      // Java: this.list.clear();
      self.list.clear();
      // Java: for(TextTabStop i : valueLst)
        for i in valueLst
      {
         // Java: KoBin bin = i.lastBin(this.zone);
         let bin = i.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.list.add(bin);
            self.list.add(bin);
         }
      }
    }

    // Java: void forceWrite(Iterable<TextTabStop> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[TextTabStop]) {
      // Java: this.list.clear();
      self.list.clear();
      // Java: for(TextTabStop i : valueLst)
        for i in valueLst
      {
         // Java: this.list.add(i.lastBin(this.zone));
         self.list.add(i.lastBin(self.zone));
      }
    }
}

impl KoAr for TabStopList {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for TabStopList { fn close(&mut self) { self.close(); } }

// ============================================================
// Inner class: TextParaOptions
// ============================================================

impl TextParaOptions {
    // Java: static protected static <Value1 extends TextParaOptions> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::TextParaOptions_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static TextParaOptions create()
    pub fn create() -> Self {
      // Java: return initialize(TextParaOptions.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static TextParaOptions open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(TextParaOptions.class, bin);
      Self::initialize(bin)
    }

    // Java: TextParaOptions()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpBool();
      Self { bin_box: KpBinBox::new(), map: KpBool::new(), zone: KoZone::defaultZone(), parent: None }
    }

    // Java: TextParaOptions(KoBin bin)
    pub fn TextParaOptions_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpBool(bin);
      Self { bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpBool::KpBool_from(bin), zone: KoZone::defaultZone(), parent: None }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
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

    // Java: void load(TextParaOptions value)
    pub fn load_value(&mut self, value: Option<&TextParaOptions>) {
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
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(TextParaOptions o)
    pub fn equalsLax(&self, o: &TextParaOptions) -> bool {
      // Java: return this.equals(o);
      self.equals(o)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TextParaOptions) -> bool {
      // Java: if (null != o && o instanceof TextParaOptions)
        if true
      {
         // Java: TextParaOptions dst = (TextParaOptions)o;
         let dst = o;
         // Java: return this == dst ? true : this.map.equals(dst.map);
         if std::ptr::eq(self, dst) { return true; }
         return self.map.equals(&dst.map);
      }
      // Java: else
      // Java: return false;
      return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: return this.map.hashCode();
      self.map.hashCode()
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

    // Java: void follow(TextParaOptions parent)
    pub fn follow(&mut self, parent: Option<&mut TextParaOptions>) {
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

    // Java: TextParaOptions parent()
    pub fn parent(&self) -> Option<&TextParaOptions> {
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

    // Java: boolean isBuClrFollowTx()
    pub fn isBuClrFollowTx(&self) -> bool {
      // Java: if (!this.map.has(1))
        if !self.map.has(1)
      {
         // Java: return this.parent == null ? false : this.parent.isBuClrFollowTx();
         return if self.parent.is_none() { false } else { self.parent.as_ref().unwrap().isBuClrFollowTx() };
      }
      // Java: else
        else
      {
         // Java: return this.map.get(1);
         return self.map.get(1);
      }
    }

    // Java: boolean hasBuClrFollowTx()
    pub fn hasBuClrFollowTx(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasBuClrFollowTx();
      self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasBuClrFollowTx())
    }

    // Java: void deleteBuClrFollowTx()
    pub fn deleteBuClrFollowTx(&mut self) {
      // Java: this.map.clear(1);
      self.map.clear(1);
    }

    // Java: void putBuClrFollowTx(boolean value)
    pub fn putBuClrFollowTx(&mut self, value: bool) {
      // Java: this.map.set(1, value);
      self.map.set(1, value);
    }

    // Java: boolean isBuSzFollowTx()
    pub fn isBuSzFollowTx(&self) -> bool {
      // Java: if (!this.map.has(2))
        if !self.map.has(2)
      {
         // Java: return this.parent == null ? false : this.parent.isBuSzFollowTx();
         return if self.parent.is_none() { false } else { self.parent.as_ref().unwrap().isBuSzFollowTx() };
      }
      // Java: else
        else
      {
         // Java: return this.map.get(2);
         return self.map.get(2);
      }
    }

    // Java: boolean hasBuSzFollowTx()
    pub fn hasBuSzFollowTx(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasBuSzFollowTx();
      self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasBuSzFollowTx())
    }

    // Java: void deleteBuSzFollowTx()
    pub fn deleteBuSzFollowTx(&mut self) {
      // Java: this.map.clear(2);
      self.map.clear(2);
    }

    // Java: void putBuSzFollowTx(boolean value)
    pub fn putBuSzFollowTx(&mut self, value: bool) {
      // Java: this.map.set(2, value);
      self.map.set(2, value);
    }

    // Java: boolean isBuFontFollowTx()
    pub fn isBuFontFollowTx(&self) -> bool {
      // Java: if (!this.map.has(3))
        if !self.map.has(3)
      {
         // Java: return this.parent == null ? false : this.parent.isBuFontFollowTx();
         return if self.parent.is_none() { false } else { self.parent.as_ref().unwrap().isBuFontFollowTx() };
      }
      // Java: else
        else
      {
         // Java: return this.map.get(3);
         return self.map.get(3);
      }
    }

    // Java: boolean hasBuFontFollowTx()
    pub fn hasBuFontFollowTx(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasBuFontFollowTx();
      self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasBuFontFollowTx())
    }

    // Java: void deleteBuFontFollowTx()
    pub fn deleteBuFontFollowTx(&mut self) {
      // Java: this.map.clear(3);
      self.map.clear(3);
    }

    // Java: void putBuFontFollowTx(boolean value)
    pub fn putBuFontFollowTx(&mut self, value: bool) {
      // Java: this.map.set(3, value);
      self.map.set(3, value);
    }

    // Java: boolean isBuNone()
    pub fn isBuNone(&self) -> bool {
      // Java: if (!this.map.has(4))
        if !self.map.has(4)
      {
         // Java: return this.parent == null ? false : this.parent.isBuNone();
         return if self.parent.is_none() { false } else { self.parent.as_ref().unwrap().isBuNone() };
      }
      // Java: else
        else
      {
         // Java: return this.map.get(4);
         return self.map.get(4);
      }
    }

    // Java: boolean hasBuNone()
    pub fn hasBuNone(&self) -> bool {
      // Java: return this.map.has(4) || this.parent != null && this.parent.hasBuNone();
      self.map.has(4) || self.parent.as_ref().map_or(false, |p| p.hasBuNone())
    }

    // Java: void deleteBuNone()
    pub fn deleteBuNone(&mut self) {
      // Java: this.map.clear(4);
      self.map.clear(4);
    }

    // Java: void putBuNone(boolean value)
    pub fn putBuNone(&mut self, value: bool) {
      // Java: this.map.set(4, value);
      self.map.set(4, value);
    }

    // Java: boolean isRightToLeft()
    pub fn isRightToLeft(&self) -> bool {
      // Java: if (!this.map.has(5))
        if !self.map.has(5)
      {
         // Java: return this.parent == null ? false : this.parent.isRightToLeft();
         return if self.parent.is_none() { false } else { self.parent.as_ref().unwrap().isRightToLeft() };
      }
      // Java: else
        else
      {
         // Java: return this.map.get(5);
         return self.map.get(5);
      }
    }

    // Java: boolean hasRightToLeft()
    pub fn hasRightToLeft(&self) -> bool {
      // Java: return this.map.has(5) || this.parent != null && this.parent.hasRightToLeft();
      self.map.has(5) || self.parent.as_ref().map_or(false, |p| p.hasRightToLeft())
    }

    // Java: void deleteRightToLeft()
    pub fn deleteRightToLeft(&mut self) {
      // Java: this.map.clear(5);
      self.map.clear(5);
    }

    // Java: void putRightToLeft(boolean value)
    pub fn putRightToLeft(&mut self, value: bool) {
      // Java: this.map.set(5, value);
      self.map.set(5, value);
    }

    // Java: boolean isEaLnBrk()
    pub fn isEaLnBrk(&self) -> bool {
      // Java: if (!this.map.has(6))
        if !self.map.has(6)
      {
         // Java: return this.parent == null ? true : this.parent.isEaLnBrk();
         return if self.parent.is_none() { true } else { self.parent.as_ref().unwrap().isEaLnBrk() };
      }
      // Java: else
        else
      {
         // Java: return this.map.get(6);
         return self.map.get(6);
      }
    }

    // Java: boolean hasEaLnBrk()
    pub fn hasEaLnBrk(&self) -> bool {
      // Java: return this.map.has(6) || this.parent != null && this.parent.hasEaLnBrk();
      self.map.has(6) || self.parent.as_ref().map_or(false, |p| p.hasEaLnBrk())
    }

    // Java: void deleteEaLnBrk()
    pub fn deleteEaLnBrk(&mut self) {
      // Java: this.map.clear(6);
      self.map.clear(6);
    }

    // Java: void putEaLnBrk(boolean value)
    pub fn putEaLnBrk(&mut self, value: bool) {
      // Java: this.map.set(6, value);
      self.map.set(6, value);
    }

    // Java: boolean isLatinLnBrk()
    pub fn isLatinLnBrk(&self) -> bool {
      // Java: if (!this.map.has(7))
        if !self.map.has(7)
      {
         // Java: return this.parent == null ? true : this.parent.isLatinLnBrk();
         return if self.parent.is_none() { true } else { self.parent.as_ref().unwrap().isLatinLnBrk() };
      }
      // Java: else
        else
      {
         // Java: return this.map.get(7);
         return self.map.get(7);
      }
    }

    // Java: boolean hasLatinLnBrk()
    pub fn hasLatinLnBrk(&self) -> bool {
      // Java: return this.map.has(7) || this.parent != null && this.parent.hasLatinLnBrk();
      self.map.has(7) || self.parent.as_ref().map_or(false, |p| p.hasLatinLnBrk())
    }

    // Java: void deleteLatinLnBrk()
    pub fn deleteLatinLnBrk(&mut self) {
      // Java: this.map.clear(7);
      self.map.clear(7);
    }

    // Java: void putLatinLnBrk(boolean value)
    pub fn putLatinLnBrk(&mut self, value: bool) {
      // Java: this.map.set(7, value);
      self.map.set(7, value);
    }

    // Java: boolean isHangingPunct()
    pub fn isHangingPunct(&self) -> bool {
      // Java: if (!this.map.has(8))
        if !self.map.has(8)
      {
         // Java: return this.parent == null ? false : this.parent.isHangingPunct();
         return if self.parent.is_none() { false } else { self.parent.as_ref().unwrap().isHangingPunct() };
      }
      // Java: else
        else
      {
         // Java: return this.map.get(8);
         return self.map.get(8);
      }
    }

    // Java: boolean hasHangingPunct()
    pub fn hasHangingPunct(&self) -> bool {
      // Java: return this.map.has(8) || this.parent != null && this.parent.hasHangingPunct();
      self.map.has(8) || self.parent.as_ref().map_or(false, |p| p.hasHangingPunct())
    }

    // Java: void deleteHangingPunct()
    pub fn deleteHangingPunct(&mut self) {
      // Java: this.map.clear(8);
      self.map.clear(8);
    }

    // Java: void putHangingPunct(boolean value)
    pub fn putHangingPunct(&mut self, value: bool) {
      // Java: this.map.set(8, value);
      self.map.set(8, value);
    }

}

impl KoAr for TextParaOptions {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for TextParaOptions { fn close(&mut self) { self.close(); } }
