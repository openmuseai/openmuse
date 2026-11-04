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

const FIELD_TYPE: i32 = 1;
const FIELD_TINT: i32 = 2;
const FIELD_SHADE: i32 = 3;
const FIELD_ALPHA: i32 = 4;
const FIELD_ALPHA_MOD: i32 = 5;
const FIELD_ALPHA_OFF: i32 = 6;
const FIELD_RED: i32 = 7;
const FIELD_RED_MOD: i32 = 8;
const FIELD_RED_OFF: i32 = 9;
const FIELD_GREEN: i32 = 10;
const FIELD_GREEN_MOD: i32 = 11;
const FIELD_GREEN_OFF: i32 = 12;
const FIELD_BLUE: i32 = 13;
const FIELD_BLUE_MOD: i32 = 14;
const FIELD_BLUE_OFF: i32 = 15;
const FIELD_HUE: i32 = 16;
const FIELD_HUE_MOD: i32 = 17;
const FIELD_HUE_OFF: i32 = 18;
const FIELD_SAT: i32 = 19;
const FIELD_SAT_MOD: i32 = 20;
const FIELD_SAT_OFF: i32 = 21;
const FIELD_LUM: i32 = 22;
const FIELD_LUM_MOD: i32 = 23;
const FIELD_LUM_OFF: i32 = 24;

// Java: public class ColorTransform implements KoAr, KoClosable
pub struct ColorTransform {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: ColorTransform parent
    pub(crate) parent: Option<Box<ColorTransform>>,
}

// Java: public class ColorTransform implements KoAr, KoClosable
impl ColorTransform {
    // Java: static protected static <Value1 extends ColorTransform> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = ColorTransform::ColorTransform_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static ColorTransform create()
    pub fn create() -> Self {
      // Java: return initialize(ColorTransform.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static ColorTransform open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(ColorTransform.class, bin);
      Self::initialize(bin)
    }

    // Java: ColorTransform()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      ColorTransform { zone: KoZone::defaultZone(), bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: ColorTransform(KoBin bin)
    pub fn ColorTransform_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      ColorTransform { zone: KoZone::defaultZone(), bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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
      let boxed = self.bin_box.get();
      self.map.load(boxed.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load_bin(&mut self, bin: &dyn KoBinTrait) {
      // Java: this.loadBin(bin, this.zone);
      self.loadBin_with_zone(bin, self.zone);
    }

    // Java: void load(ColorTransform value)
    pub fn load_value(&mut self, value: Option<&ColorTransform>) {
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
      if self.map.dirty() { let s = self.map.save(zone); self.bin_box.set(Some(s.as_ref())); }
      self.bin_box.get()
    }

    // Java: boolean equalsLax(ColorTransform o)
    pub fn equalsLax(&self, o: &ColorTransform) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &ColorTransform) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &ColorTransform, _isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof ColorTransform)
      if true
      {
         // Java: ColorTransform dst = (ColorTransform)o;
         let dst = o;
         // Java: if (this == dst)
         if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasType() || !dst.hasType()) && (!this.hasType() || dst.hasType()))
         else if (self.hasType() || !dst.hasType()) && (!self.hasType() || dst.hasType())
         {
            // Java: if (this.hasType() && dst.hasType() && this.type() != dst.type())
            if self.hasType() && dst.hasType() && self.get_type() != dst.get_type()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasTint() || !dst.hasTint()) && (!this.hasTint() || dst.hasTint()))
            else if (self.hasTint() || !dst.hasTint()) && (!self.hasTint() || dst.hasTint())
            {
               // Java: if (this.hasTint() && dst.hasTint() && this.tint() != dst.tint())
               if self.hasTint() && dst.hasTint() && self.tint() != dst.tint()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasShade() || !dst.hasShade()) && (!this.hasShade() || dst.hasShade()))
               else if (self.hasShade() || !dst.hasShade()) && (!self.hasShade() || dst.hasShade())
               {
                  // Java: if (this.hasShade() && dst.hasShade() && this.shade() != dst.shade())
                  if self.hasShade() && dst.hasShade() && self.shade() != dst.shade()
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasAlpha() || !dst.hasAlpha()) && (!this.hasAlpha() || dst.hasAlpha()))
                  else if (self.hasAlpha() || !dst.hasAlpha()) && (!self.hasAlpha() || dst.hasAlpha())
                  {
                     // Java: if (this.hasAlpha() && dst.hasAlpha() && this.alpha() != dst.alpha())
                     if self.hasAlpha() && dst.hasAlpha() && self.alpha() != dst.alpha()
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: else if ((this.hasAlphaMod() || !dst.hasAlphaMod()) && (!this.hasAlphaMod() || dst.hasAlphaMod()))
                     else if (self.hasAlphaMod() || !dst.hasAlphaMod()) && (!self.hasAlphaMod() || dst.hasAlphaMod())
                     {
                        // Java: if (this.hasAlphaMod() && dst.hasAlphaMod() && this.alphaMod() != dst.alphaMod())
                        if self.hasAlphaMod() && dst.hasAlphaMod() && self.alphaMod() != dst.alphaMod()
                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: else if ((this.hasAlphaOff() || !dst.hasAlphaOff()) && (!this.hasAlphaOff() || dst.hasAlphaOff()))
                        else if (self.hasAlphaOff() || !dst.hasAlphaOff()) && (!self.hasAlphaOff() || dst.hasAlphaOff())
                        {
                           // Java: if (this.hasAlphaOff() && dst.hasAlphaOff() && this.alphaOff() != dst.alphaOff())
                           if self.hasAlphaOff() && dst.hasAlphaOff() && self.alphaOff() != dst.alphaOff()
                           {
                              // Java: return false;
                              return false;
                           }
                           // Java: else if ((this.hasRed() || !dst.hasRed()) && (!this.hasRed() || dst.hasRed()))
                           else if (self.hasRed() || !dst.hasRed()) && (!self.hasRed() || dst.hasRed())
                           {
                              // Java: if (this.hasRed() && dst.hasRed() && this.red() != dst.red())
                              if self.hasRed() && dst.hasRed() && self.red() != dst.red()
                              {
                                 // Java: return false;
                                 return false;
                              }
                              // Java: else if ((this.hasRedMod() || !dst.hasRedMod()) && (!this.hasRedMod() || dst.hasRedMod()))
                              else if (self.hasRedMod() || !dst.hasRedMod()) && (!self.hasRedMod() || dst.hasRedMod())
                              {
                                 // Java: if (this.hasRedMod() && dst.hasRedMod() && this.redMod() != dst.redMod())
                                 if self.hasRedMod() && dst.hasRedMod() && self.redMod() != dst.redMod()
                                 {
                                    // Java: return false;
                                    return false;
                                 }
                                 // Java: else if ((this.hasRedOff() || !dst.hasRedOff()) && (!this.hasRedOff() || dst.hasRedOff()))
                                 else if (self.hasRedOff() || !dst.hasRedOff()) && (!self.hasRedOff() || dst.hasRedOff())
                                 {
                                    // Java: if (this.hasRedOff() && dst.hasRedOff() && this.redOff() != dst.redOff())
                                    if self.hasRedOff() && dst.hasRedOff() && self.redOff() != dst.redOff()
                                    {
                                       // Java: return false;
                                       return false;
                                    }
                                    // Java: else if ((this.hasGreen() || !dst.hasGreen()) && (!this.hasGreen() || dst.hasGreen()))
                                    else if (self.hasGreen() || !dst.hasGreen()) && (!self.hasGreen() || dst.hasGreen())
                                    {
                                       // Java: if (this.hasGreen() && dst.hasGreen() && this.green() != dst.green())
                                       if self.hasGreen() && dst.hasGreen() && self.green() != dst.green()
                                       {
                                          // Java: return false;
                                          return false;
                                       }
                                       // Java: else if ((this.hasGreenMod() || !dst.hasGreenMod()) && (!this.hasGreenMod() || dst.hasGreenMod()))
                                       else if (self.hasGreenMod() || !dst.hasGreenMod()) && (!self.hasGreenMod() || dst.hasGreenMod())
                                       {
                                          // Java: if (this.hasGreenMod() && dst.hasGreenMod() && this.greenMod() != dst.greenMod())
                                          if self.hasGreenMod() && dst.hasGreenMod() && self.greenMod() != dst.greenMod()
                                          {
                                             // Java: return false;
                                             return false;
                                          }
                                          // Java: else if ((this.hasGreenOff() || !dst.hasGreenOff()) && (!this.hasGreenOff() || dst.hasGreenOff()))
                                          else if (self.hasGreenOff() || !dst.hasGreenOff()) && (!self.hasGreenOff() || dst.hasGreenOff())
                                          {
                                             // Java: if (this.hasGreenOff() && dst.hasGreenOff() && this.greenOff() != dst.greenOff())
                                             if self.hasGreenOff() && dst.hasGreenOff() && self.greenOff() != dst.greenOff()
                                             {
                                                // Java: return false;
                                                return false;
                                             }
                                             // Java: else if ((this.hasBlue() || !dst.hasBlue()) && (!this.hasBlue() || dst.hasBlue()))
                                             else if (self.hasBlue() || !dst.hasBlue()) && (!self.hasBlue() || dst.hasBlue())
                                             {
                                                // Java: if (this.hasBlue() && dst.hasBlue() && this.blue() != dst.blue())
                                                if self.hasBlue() && dst.hasBlue() && self.blue() != dst.blue()
                                                {
                                                   // Java: return false;
                                                   return false;
                                                }
                                                // Java: else if ((this.hasBlueMod() || !dst.hasBlueMod()) && (!this.hasBlueMod() || dst.hasBlueMod()))
                                                else if (self.hasBlueMod() || !dst.hasBlueMod()) && (!self.hasBlueMod() || dst.hasBlueMod())
                                                {
                                                   // Java: if (this.hasBlueMod() && dst.hasBlueMod() && this.blueMod() != dst.blueMod())
                                                   if self.hasBlueMod() && dst.hasBlueMod() && self.blueMod() != dst.blueMod()
                                                   {
                                                      // Java: return false;
                                                      return false;
                                                   }
                                                   // Java: else if ((this.hasBlueOff() || !dst.hasBlueOff()) && (!this.hasBlueOff() || dst.hasBlueOff()))
                                                   else if (self.hasBlueOff() || !dst.hasBlueOff()) && (!self.hasBlueOff() || dst.hasBlueOff())
                                                   {
                                                      // Java: if (this.hasBlueOff() && dst.hasBlueOff() && this.blueOff() != dst.blueOff())
                                                      if self.hasBlueOff() && dst.hasBlueOff() && self.blueOff() != dst.blueOff()
                                                      {
                                                         // Java: return false;
                                                         return false;
                                                      }
                                                      // Java: else if ((this.hasHue() || !dst.hasHue()) && (!this.hasHue() || dst.hasHue()))
                                                      else if (self.hasHue() || !dst.hasHue()) && (!self.hasHue() || dst.hasHue())
                                                      {
                                                         // Java: if (this.hasHue() && dst.hasHue() && this.hue() != dst.hue())
                                                         if self.hasHue() && dst.hasHue() && self.hue() != dst.hue()
                                                         {
                                                            // Java: return false;
                                                            return false;
                                                         }
                                                         // Java: else if ((this.hasHueMod() || !dst.hasHueMod()) && (!this.hasHueMod() || dst.hasHueMod()))
                                                         else if (self.hasHueMod() || !dst.hasHueMod()) && (!self.hasHueMod() || dst.hasHueMod())
                                                         {
                                                            // Java: if (this.hasHueMod() && dst.hasHueMod() && this.hueMod() != dst.hueMod())
                                                            if self.hasHueMod() && dst.hasHueMod() && self.hueMod() != dst.hueMod()
                                                            {
                                                               // Java: return false;
                                                               return false;
                                                            }
                                                            // Java: else if ((this.hasHueOff() || !dst.hasHueOff()) && (!this.hasHueOff() || dst.hasHueOff()))
                                                            else if (self.hasHueOff() || !dst.hasHueOff()) && (!self.hasHueOff() || dst.hasHueOff())
                                                            {
                                                               // Java: if (this.hasHueOff() && dst.hasHueOff() && this.hueOff() != dst.hueOff())
                                                               if self.hasHueOff() && dst.hasHueOff() && self.hueOff() != dst.hueOff()
                                                               {
                                                                  // Java: return false;
                                                                  return false;
                                                               }
                                                               // Java: else if ((this.hasSat() || !dst.hasSat()) && (!this.hasSat() || dst.hasSat()))
                                                               else if (self.hasSat() || !dst.hasSat()) && (!self.hasSat() || dst.hasSat())
                                                               {
                                                                  // Java: if (this.hasSat() && dst.hasSat() && this.sat() != dst.sat())
                                                                  if self.hasSat() && dst.hasSat() && self.sat() != dst.sat()
                                                                  {
                                                                     // Java: return false;
                                                                     return false;
                                                                  }
                                                                  // Java: else if ((this.hasSatMod() || !dst.hasSatMod()) && (!this.hasSatMod() || dst.hasSatMod()))
                                                                  else if (self.hasSatMod() || !dst.hasSatMod()) && (!self.hasSatMod() || dst.hasSatMod())
                                                                  {
                                                                     // Java: if (this.hasSatMod() && dst.hasSatMod() && this.satMod() != dst.satMod())
                                                                     if self.hasSatMod() && dst.hasSatMod() && self.satMod() != dst.satMod()
                                                                     {
                                                                        // Java: return false;
                                                                        return false;
                                                                     }
                                                                     // Java: else if ((this.hasSatOff() || !dst.hasSatOff()) && (!this.hasSatOff() || dst.hasSatOff()))
                                                                     else if (self.hasSatOff() || !dst.hasSatOff()) && (!self.hasSatOff() || dst.hasSatOff())
                                                                     {
                                                                        // Java: if (this.hasSatOff() && dst.hasSatOff() && this.satOff() != dst.satOff())
                                                                        if self.hasSatOff() && dst.hasSatOff() && self.satOff() != dst.satOff()
                                                                        {
                                                                           // Java: return false;
                                                                           return false;
                                                                        }
                                                                        // Java: else if ((this.hasLum() || !dst.hasLum()) && (!this.hasLum() || dst.hasLum()))
                                                                        else if (self.hasLum() || !dst.hasLum()) && (!self.hasLum() || dst.hasLum())
                                                                        {
                                                                           // Java: if (this.hasLum() && dst.hasLum() && this.lum() != dst.lum())
                                                                           if self.hasLum() && dst.hasLum() && self.lum() != dst.lum()
                                                                           {
                                                                              // Java: return false;
                                                                              return false;
                                                                           }
                                                                           // Java: else if ((this.hasLumMod() || !dst.hasLumMod()) && (!this.hasLumMod() || dst.hasLumMod()))
                                                                           else if (self.hasLumMod() || !dst.hasLumMod()) && (!self.hasLumMod() || dst.hasLumMod())
                                                                           {
                                                                              // Java: if (this.hasLumMod() && dst.hasLumMod() && this.lumMod() != dst.lumMod())
                                                                              if self.hasLumMod() && dst.hasLumMod() && self.lumMod() != dst.lumMod()
                                                                              {
                                                                                 // Java: return false;
                                                                                 return false;
                                                                              }
                                                                              // Java: else if ((this.hasLumOff() || !dst.hasLumOff()) && (!this.hasLumOff() || dst.hasLumOff()))
                                                                              else if (self.hasLumOff() || !dst.hasLumOff()) && (!self.hasLumOff() || dst.hasLumOff())
                                                                              {
                                                                                 // Java: return !this.hasLumOff() || !dst.hasLumOff() || this.lumOff() == dst.lumOff();
                                                                                 return !self.hasLumOff() || !dst.hasLumOff() || self.lumOff() == dst.lumOff();
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
      // Java: if (this.hasType())
      if self.hasType()
      {
         // Java: ret += Integer.valueOf(this.type()).hashCode();
         ret = ret.wrapping_add(self.get_type());
      }
      // Java: if (this.hasTint())
      if self.hasTint()
      {
         // Java: ret += Double.valueOf(this.tint()).hashCode();
         ret = ret.wrapping_add(double_hash_code(self.tint()));
      }
      // Java: if (this.hasShade())
      if self.hasShade()
      {
         // Java: ret += Double.valueOf(this.shade()).hashCode();
         ret = ret.wrapping_add(double_hash_code(self.shade()));
      }
      // Java: if (this.hasAlpha())
      if self.hasAlpha()
      {
         // Java: ret += Double.valueOf(this.alpha()).hashCode();
         ret = ret.wrapping_add(double_hash_code(self.alpha()));
      }
      // Java: if (this.hasAlphaMod())
      if self.hasAlphaMod()
      {
         // Java: ret += Double.valueOf(this.alphaMod()).hashCode();
         ret = ret.wrapping_add(double_hash_code(self.alphaMod()));
      }
      // Java: if (this.hasAlphaOff())
      if self.hasAlphaOff()
      {
         // Java: ret += Double.valueOf(this.alphaOff()).hashCode();
         ret = ret.wrapping_add(double_hash_code(self.alphaOff()));
      }
      // Java: if (this.hasRed())
      if self.hasRed()
      {
         // Java: ret += Double.valueOf(this.red()).hashCode();
         ret = ret.wrapping_add(double_hash_code(self.red()));
      }
      // Java: if (this.hasRedMod())
      if self.hasRedMod()
      {
         // Java: ret += Double.valueOf(this.redMod()).hashCode();
         ret = ret.wrapping_add(double_hash_code(self.redMod()));
      }
      // Java: if (this.hasRedOff())
      if self.hasRedOff()
      {
         // Java: ret += Double.valueOf(this.redOff()).hashCode();
         ret = ret.wrapping_add(double_hash_code(self.redOff()));
      }
      // Java: if (this.hasGreen())
      if self.hasGreen()
      {
         // Java: ret += Double.valueOf(this.green()).hashCode();
         ret = ret.wrapping_add(double_hash_code(self.green()));
      }
      // Java: if (this.hasGreenMod())
      if self.hasGreenMod()
      {
         // Java: ret += Double.valueOf(this.greenMod()).hashCode();
         ret = ret.wrapping_add(double_hash_code(self.greenMod()));
      }
      // Java: if (this.hasGreenOff())
      if self.hasGreenOff()
      {
         // Java: ret += Double.valueOf(this.greenOff()).hashCode();
         ret = ret.wrapping_add(double_hash_code(self.greenOff()));
      }
      // Java: if (this.hasBlue())
      if self.hasBlue()
      {
         // Java: ret += Double.valueOf(this.blue()).hashCode();
         ret = ret.wrapping_add(double_hash_code(self.blue()));
      }
      // Java: if (this.hasBlueMod())
      if self.hasBlueMod()
      {
         // Java: ret += Double.valueOf(this.blueMod()).hashCode();
         ret = ret.wrapping_add(double_hash_code(self.blueMod()));
      }
      // Java: if (this.hasBlueOff())
      if self.hasBlueOff()
      {
         // Java: ret += Double.valueOf(this.blueOff()).hashCode();
         ret = ret.wrapping_add(double_hash_code(self.blueOff()));
      }
      // Java: if (this.hasHue())
      if self.hasHue()
      {
         // Java: ret += Double.valueOf(this.hue()).hashCode();
         ret = ret.wrapping_add(double_hash_code(self.hue()));
      }
      // Java: if (this.hasHueMod())
      if self.hasHueMod()
      {
         // Java: ret += Double.valueOf(this.hueMod()).hashCode();
         ret = ret.wrapping_add(double_hash_code(self.hueMod()));
      }
      // Java: if (this.hasHueOff())
      if self.hasHueOff()
      {
         // Java: ret += Double.valueOf(this.hueOff()).hashCode();
         ret = ret.wrapping_add(double_hash_code(self.hueOff()));
      }
      // Java: if (this.hasSat())
      if self.hasSat()
      {
         // Java: ret += Double.valueOf(this.sat()).hashCode();
         ret = ret.wrapping_add(double_hash_code(self.sat()));
      }
      // Java: if (this.hasSatMod())
      if self.hasSatMod()
      {
         // Java: ret += Double.valueOf(this.satMod()).hashCode();
         ret = ret.wrapping_add(double_hash_code(self.satMod()));
      }
      // Java: if (this.hasSatOff())
      if self.hasSatOff()
      {
         // Java: ret += Double.valueOf(this.satOff()).hashCode();
         ret = ret.wrapping_add(double_hash_code(self.satOff()));
      }
      // Java: if (this.hasLum())
      if self.hasLum()
      {
         // Java: ret += Double.valueOf(this.lum()).hashCode();
         ret = ret.wrapping_add(double_hash_code(self.lum()));
      }
      // Java: if (this.hasLumMod())
      if self.hasLumMod()
      {
         // Java: ret += Double.valueOf(this.lumMod()).hashCode();
         ret = ret.wrapping_add(double_hash_code(self.lumMod()));
      }
      // Java: if (this.hasLumOff())
      if self.hasLumOff()
      {
         // Java: ret += Double.valueOf(this.lumOff()).hashCode();
         ret = ret.wrapping_add(double_hash_code(self.lumOff()));
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
    fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    fn cleanCache(&mut self) {
    }

    // Java: void follow(ColorTransform parent)
    pub fn follow(&mut self, mut parent: Box<ColorTransform>) {
      // Java: if (parent != null)
      {
         // Java: parent.discard();
         parent.discard();
         // Java: this.discard();
         self.discard();
         // Java: this.parent = parent;
         self.parent = Some(parent);
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

    // Java: ColorTransform parent()
    pub fn get_parent(&self) -> Option<&ColorTransform> {
      // Java: return this.parent;
      self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      let bin = self.bin_box.get();
      self.loadBin_with_zone(bin.as_ref(), self.zone);
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: int type()
    pub fn get_type(&self) -> i32 {
      // Java: if (!this.map.has(1))
      if !self.map.has(FIELD_TYPE)
      {
         // Java: return this.parent == null ? 0 : this.parent.type();
         return self.parent.as_ref().map_or(0, |p| p.get_type());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(FIELD_TYPE);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasType()
    pub fn hasType(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasType();
      self.map.has(FIELD_TYPE) || self.parent.as_ref().map_or(false, |p| p.hasType())
    }

    // Java: void deleteType()
    pub fn deleteType(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(FIELD_TYPE);
    }

    // Java: void putType(int value)
    pub fn putType(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_TYPE, Encode::from_int(value, self.zone));
    }

    // Java: double tint()
    pub fn tint(&self) -> f64 {
      // Java: if (!this.map.has(2))
      if !self.map.has(FIELD_TINT)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.tint();
         return self.parent.as_ref().map_or(0.0, |p| p.tint());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(FIELD_TINT);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasTint()
    pub fn hasTint(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasTint();
      self.map.has(FIELD_TINT) || self.parent.as_ref().map_or(false, |p| p.hasTint())
    }

    // Java: void deleteTint()
    pub fn deleteTint(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(FIELD_TINT);
    }

    // Java: void putTint(double value)
    pub fn putTint(&mut self, value: f64) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_TINT, Encode::from_double(value, self.zone));
    }

    // Java: double shade()
    pub fn shade(&self) -> f64 {
      // Java: if (!this.map.has(3))
      if !self.map.has(FIELD_SHADE)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.shade();
         return self.parent.as_ref().map_or(0.0, |p| p.shade());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(FIELD_SHADE);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasShade()
    pub fn hasShade(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasShade();
      self.map.has(FIELD_SHADE) || self.parent.as_ref().map_or(false, |p| p.hasShade())
    }

    // Java: void deleteShade()
    pub fn deleteShade(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(FIELD_SHADE);
    }

    // Java: void putShade(double value)
    pub fn putShade(&mut self, value: f64) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_SHADE, Encode::from_double(value, self.zone));
    }

    // Java: double alpha()
    pub fn alpha(&self) -> f64 {
      // Java: if (!this.map.has(4))
      if !self.map.has(FIELD_ALPHA)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.alpha();
         return self.parent.as_ref().map_or(0.0, |p| p.alpha());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(4);
         let bin = self.map.byKey(FIELD_ALPHA);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasAlpha()
    pub fn hasAlpha(&self) -> bool {
      // Java: return this.map.has(4) || this.parent != null && this.parent.hasAlpha();
      self.map.has(FIELD_ALPHA) || self.parent.as_ref().map_or(false, |p| p.hasAlpha())
    }

    // Java: void deleteAlpha()
    pub fn deleteAlpha(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(FIELD_ALPHA);
    }

    // Java: void putAlpha(double value)
    pub fn putAlpha(&mut self, value: f64) {
      // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_ALPHA, Encode::from_double(value, self.zone));
    }

    // Java: double alphaMod()
    pub fn alphaMod(&self) -> f64 {
      // Java: if (!this.map.has(5))
      if !self.map.has(FIELD_ALPHA_MOD)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.alphaMod();
         return self.parent.as_ref().map_or(0.0, |p| p.alphaMod());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(5);
         let bin = self.map.byKey(FIELD_ALPHA_MOD);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasAlphaMod()
    pub fn hasAlphaMod(&self) -> bool {
      // Java: return this.map.has(5) || this.parent != null && this.parent.hasAlphaMod();
      self.map.has(FIELD_ALPHA_MOD) || self.parent.as_ref().map_or(false, |p| p.hasAlphaMod())
    }

    // Java: void deleteAlphaMod()
    pub fn deleteAlphaMod(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(FIELD_ALPHA_MOD);
    }

    // Java: void putAlphaMod(double value)
    pub fn putAlphaMod(&mut self, value: f64) {
      // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_ALPHA_MOD, Encode::from_double(value, self.zone));
    }

    // Java: double alphaOff()
    pub fn alphaOff(&self) -> f64 {
      // Java: if (!this.map.has(6))
      if !self.map.has(FIELD_ALPHA_OFF)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.alphaOff();
         return self.parent.as_ref().map_or(0.0, |p| p.alphaOff());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(6);
         let bin = self.map.byKey(FIELD_ALPHA_OFF);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasAlphaOff()
    pub fn hasAlphaOff(&self) -> bool {
      // Java: return this.map.has(6) || this.parent != null && this.parent.hasAlphaOff();
      self.map.has(FIELD_ALPHA_OFF) || self.parent.as_ref().map_or(false, |p| p.hasAlphaOff())
    }

    // Java: void deleteAlphaOff()
    pub fn deleteAlphaOff(&mut self) {
      // Java: this.map.put(6, (KoBin)null);
      self.map.clear_key(FIELD_ALPHA_OFF);
    }

    // Java: void putAlphaOff(double value)
    pub fn putAlphaOff(&mut self, value: f64) {
      // Java: this.map.put(6, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_ALPHA_OFF, Encode::from_double(value, self.zone));
    }

    // Java: double red()
    pub fn red(&self) -> f64 {
      // Java: if (!this.map.has(7))
      if !self.map.has(FIELD_RED)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.red();
         return self.parent.as_ref().map_or(0.0, |p| p.red());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(7);
         let bin = self.map.byKey(FIELD_RED);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasRed()
    pub fn hasRed(&self) -> bool {
      // Java: return this.map.has(7) || this.parent != null && this.parent.hasRed();
      self.map.has(FIELD_RED) || self.parent.as_ref().map_or(false, |p| p.hasRed())
    }

    // Java: void deleteRed()
    pub fn deleteRed(&mut self) {
      // Java: this.map.put(7, (KoBin)null);
      self.map.clear_key(FIELD_RED);
    }

    // Java: void putRed(double value)
    pub fn putRed(&mut self, value: f64) {
      // Java: this.map.put(7, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_RED, Encode::from_double(value, self.zone));
    }

    // Java: double redMod()
    pub fn redMod(&self) -> f64 {
      // Java: if (!this.map.has(8))
      if !self.map.has(FIELD_RED_MOD)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.redMod();
         return self.parent.as_ref().map_or(0.0, |p| p.redMod());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(8);
         let bin = self.map.byKey(FIELD_RED_MOD);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasRedMod()
    pub fn hasRedMod(&self) -> bool {
      // Java: return this.map.has(8) || this.parent != null && this.parent.hasRedMod();
      self.map.has(FIELD_RED_MOD) || self.parent.as_ref().map_or(false, |p| p.hasRedMod())
    }

    // Java: void deleteRedMod()
    pub fn deleteRedMod(&mut self) {
      // Java: this.map.put(8, (KoBin)null);
      self.map.clear_key(FIELD_RED_MOD);
    }

    // Java: void putRedMod(double value)
    pub fn putRedMod(&mut self, value: f64) {
      // Java: this.map.put(8, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_RED_MOD, Encode::from_double(value, self.zone));
    }

    // Java: double redOff()
    pub fn redOff(&self) -> f64 {
      // Java: if (!this.map.has(9))
      if !self.map.has(FIELD_RED_OFF)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.redOff();
         return self.parent.as_ref().map_or(0.0, |p| p.redOff());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(9);
         let bin = self.map.byKey(FIELD_RED_OFF);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasRedOff()
    pub fn hasRedOff(&self) -> bool {
      // Java: return this.map.has(9) || this.parent != null && this.parent.hasRedOff();
      self.map.has(FIELD_RED_OFF) || self.parent.as_ref().map_or(false, |p| p.hasRedOff())
    }

    // Java: void deleteRedOff()
    pub fn deleteRedOff(&mut self) {
      // Java: this.map.put(9, (KoBin)null);
      self.map.clear_key(FIELD_RED_OFF);
    }

    // Java: void putRedOff(double value)
    pub fn putRedOff(&mut self, value: f64) {
      // Java: this.map.put(9, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_RED_OFF, Encode::from_double(value, self.zone));
    }

    // Java: double green()
    pub fn green(&self) -> f64 {
      // Java: if (!this.map.has(10))
      if !self.map.has(FIELD_GREEN)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.green();
         return self.parent.as_ref().map_or(0.0, |p| p.green());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(10);
         let bin = self.map.byKey(FIELD_GREEN);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasGreen()
    pub fn hasGreen(&self) -> bool {
      // Java: return this.map.has(10) || this.parent != null && this.parent.hasGreen();
      self.map.has(FIELD_GREEN) || self.parent.as_ref().map_or(false, |p| p.hasGreen())
    }

    // Java: void deleteGreen()
    pub fn deleteGreen(&mut self) {
      // Java: this.map.put(10, (KoBin)null);
      self.map.clear_key(FIELD_GREEN);
    }

    // Java: void putGreen(double value)
    pub fn putGreen(&mut self, value: f64) {
      // Java: this.map.put(10, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_GREEN, Encode::from_double(value, self.zone));
    }

    // Java: double greenMod()
    pub fn greenMod(&self) -> f64 {
      // Java: if (!this.map.has(11))
      if !self.map.has(FIELD_GREEN_MOD)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.greenMod();
         return self.parent.as_ref().map_or(0.0, |p| p.greenMod());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(11);
         let bin = self.map.byKey(FIELD_GREEN_MOD);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasGreenMod()
    pub fn hasGreenMod(&self) -> bool {
      // Java: return this.map.has(11) || this.parent != null && this.parent.hasGreenMod();
      self.map.has(FIELD_GREEN_MOD) || self.parent.as_ref().map_or(false, |p| p.hasGreenMod())
    }

    // Java: void deleteGreenMod()
    pub fn deleteGreenMod(&mut self) {
      // Java: this.map.put(11, (KoBin)null);
      self.map.clear_key(FIELD_GREEN_MOD);
    }

    // Java: void putGreenMod(double value)
    pub fn putGreenMod(&mut self, value: f64) {
      // Java: this.map.put(11, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_GREEN_MOD, Encode::from_double(value, self.zone));
    }

    // Java: double greenOff()
    pub fn greenOff(&self) -> f64 {
      // Java: if (!this.map.has(12))
      if !self.map.has(FIELD_GREEN_OFF)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.greenOff();
         return self.parent.as_ref().map_or(0.0, |p| p.greenOff());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(12);
         let bin = self.map.byKey(FIELD_GREEN_OFF);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasGreenOff()
    pub fn hasGreenOff(&self) -> bool {
      // Java: return this.map.has(12) || this.parent != null && this.parent.hasGreenOff();
      self.map.has(FIELD_GREEN_OFF) || self.parent.as_ref().map_or(false, |p| p.hasGreenOff())
    }

    // Java: void deleteGreenOff()
    pub fn deleteGreenOff(&mut self) {
      // Java: this.map.put(12, (KoBin)null);
      self.map.clear_key(FIELD_GREEN_OFF);
    }

    // Java: void putGreenOff(double value)
    pub fn putGreenOff(&mut self, value: f64) {
      // Java: this.map.put(12, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_GREEN_OFF, Encode::from_double(value, self.zone));
    }

    // Java: double blue()
    pub fn blue(&self) -> f64 {
      // Java: if (!this.map.has(13))
      if !self.map.has(FIELD_BLUE)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.blue();
         return self.parent.as_ref().map_or(0.0, |p| p.blue());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(13);
         let bin = self.map.byKey(FIELD_BLUE);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasBlue()
    pub fn hasBlue(&self) -> bool {
      // Java: return this.map.has(13) || this.parent != null && this.parent.hasBlue();
      self.map.has(FIELD_BLUE) || self.parent.as_ref().map_or(false, |p| p.hasBlue())
    }

    // Java: void deleteBlue()
    pub fn deleteBlue(&mut self) {
      // Java: this.map.put(13, (KoBin)null);
      self.map.clear_key(FIELD_BLUE);
    }

    // Java: void putBlue(double value)
    pub fn putBlue(&mut self, value: f64) {
      // Java: this.map.put(13, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_BLUE, Encode::from_double(value, self.zone));
    }

    // Java: double blueMod()
    pub fn blueMod(&self) -> f64 {
      // Java: if (!this.map.has(14))
      if !self.map.has(FIELD_BLUE_MOD)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.blueMod();
         return self.parent.as_ref().map_or(0.0, |p| p.blueMod());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(14);
         let bin = self.map.byKey(FIELD_BLUE_MOD);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasBlueMod()
    pub fn hasBlueMod(&self) -> bool {
      // Java: return this.map.has(14) || this.parent != null && this.parent.hasBlueMod();
      self.map.has(FIELD_BLUE_MOD) || self.parent.as_ref().map_or(false, |p| p.hasBlueMod())
    }

    // Java: void deleteBlueMod()
    pub fn deleteBlueMod(&mut self) {
      // Java: this.map.put(14, (KoBin)null);
      self.map.clear_key(FIELD_BLUE_MOD);
    }

    // Java: void putBlueMod(double value)
    pub fn putBlueMod(&mut self, value: f64) {
      // Java: this.map.put(14, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_BLUE_MOD, Encode::from_double(value, self.zone));
    }

    // Java: double blueOff()
    pub fn blueOff(&self) -> f64 {
      // Java: if (!this.map.has(15))
      if !self.map.has(FIELD_BLUE_OFF)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.blueOff();
         return self.parent.as_ref().map_or(0.0, |p| p.blueOff());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(15);
         let bin = self.map.byKey(FIELD_BLUE_OFF);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasBlueOff()
    pub fn hasBlueOff(&self) -> bool {
      // Java: return this.map.has(15) || this.parent != null && this.parent.hasBlueOff();
      self.map.has(FIELD_BLUE_OFF) || self.parent.as_ref().map_or(false, |p| p.hasBlueOff())
    }

    // Java: void deleteBlueOff()
    pub fn deleteBlueOff(&mut self) {
      // Java: this.map.put(15, (KoBin)null);
      self.map.clear_key(FIELD_BLUE_OFF);
    }

    // Java: void putBlueOff(double value)
    pub fn putBlueOff(&mut self, value: f64) {
      // Java: this.map.put(15, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_BLUE_OFF, Encode::from_double(value, self.zone));
    }

    // Java: double hue()
    pub fn hue(&self) -> f64 {
      // Java: if (!this.map.has(16))
      if !self.map.has(FIELD_HUE)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.hue();
         return self.parent.as_ref().map_or(0.0, |p| p.hue());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(16);
         let bin = self.map.byKey(FIELD_HUE);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasHue()
    pub fn hasHue(&self) -> bool {
      // Java: return this.map.has(16) || this.parent != null && this.parent.hasHue();
      self.map.has(FIELD_HUE) || self.parent.as_ref().map_or(false, |p| p.hasHue())
    }

    // Java: void deleteHue()
    pub fn deleteHue(&mut self) {
      // Java: this.map.put(16, (KoBin)null);
      self.map.clear_key(FIELD_HUE);
    }

    // Java: void putHue(double value)
    pub fn putHue(&mut self, value: f64) {
      // Java: this.map.put(16, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_HUE, Encode::from_double(value, self.zone));
    }

    // Java: double hueMod()
    pub fn hueMod(&self) -> f64 {
      // Java: if (!this.map.has(17))
      if !self.map.has(FIELD_HUE_MOD)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.hueMod();
         return self.parent.as_ref().map_or(0.0, |p| p.hueMod());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(17);
         let bin = self.map.byKey(FIELD_HUE_MOD);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasHueMod()
    pub fn hasHueMod(&self) -> bool {
      // Java: return this.map.has(17) || this.parent != null && this.parent.hasHueMod();
      self.map.has(FIELD_HUE_MOD) || self.parent.as_ref().map_or(false, |p| p.hasHueMod())
    }

    // Java: void deleteHueMod()
    pub fn deleteHueMod(&mut self) {
      // Java: this.map.put(17, (KoBin)null);
      self.map.clear_key(FIELD_HUE_MOD);
    }

    // Java: void putHueMod(double value)
    pub fn putHueMod(&mut self, value: f64) {
      // Java: this.map.put(17, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_HUE_MOD, Encode::from_double(value, self.zone));
    }

    // Java: double hueOff()
    pub fn hueOff(&self) -> f64 {
      // Java: if (!this.map.has(18))
      if !self.map.has(FIELD_HUE_OFF)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.hueOff();
         return self.parent.as_ref().map_or(0.0, |p| p.hueOff());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(18);
         let bin = self.map.byKey(FIELD_HUE_OFF);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasHueOff()
    pub fn hasHueOff(&self) -> bool {
      // Java: return this.map.has(18) || this.parent != null && this.parent.hasHueOff();
      self.map.has(FIELD_HUE_OFF) || self.parent.as_ref().map_or(false, |p| p.hasHueOff())
    }

    // Java: void deleteHueOff()
    pub fn deleteHueOff(&mut self) {
      // Java: this.map.put(18, (KoBin)null);
      self.map.clear_key(FIELD_HUE_OFF);
    }

    // Java: void putHueOff(double value)
    pub fn putHueOff(&mut self, value: f64) {
      // Java: this.map.put(18, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_HUE_OFF, Encode::from_double(value, self.zone));
    }

    // Java: double sat()
    pub fn sat(&self) -> f64 {
      // Java: if (!this.map.has(19))
      if !self.map.has(FIELD_SAT)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.sat();
         return self.parent.as_ref().map_or(0.0, |p| p.sat());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(19);
         let bin = self.map.byKey(FIELD_SAT);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasSat()
    pub fn hasSat(&self) -> bool {
      // Java: return this.map.has(19) || this.parent != null && this.parent.hasSat();
      self.map.has(FIELD_SAT) || self.parent.as_ref().map_or(false, |p| p.hasSat())
    }

    // Java: void deleteSat()
    pub fn deleteSat(&mut self) {
      // Java: this.map.put(19, (KoBin)null);
      self.map.clear_key(FIELD_SAT);
    }

    // Java: void putSat(double value)
    pub fn putSat(&mut self, value: f64) {
      // Java: this.map.put(19, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_SAT, Encode::from_double(value, self.zone));
    }

    // Java: double satMod()
    pub fn satMod(&self) -> f64 {
      // Java: if (!this.map.has(20))
      if !self.map.has(FIELD_SAT_MOD)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.satMod();
         return self.parent.as_ref().map_or(0.0, |p| p.satMod());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(20);
         let bin = self.map.byKey(FIELD_SAT_MOD);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasSatMod()
    pub fn hasSatMod(&self) -> bool {
      // Java: return this.map.has(20) || this.parent != null && this.parent.hasSatMod();
      self.map.has(FIELD_SAT_MOD) || self.parent.as_ref().map_or(false, |p| p.hasSatMod())
    }

    // Java: void deleteSatMod()
    pub fn deleteSatMod(&mut self) {
      // Java: this.map.put(20, (KoBin)null);
      self.map.clear_key(FIELD_SAT_MOD);
    }

    // Java: void putSatMod(double value)
    pub fn putSatMod(&mut self, value: f64) {
      // Java: this.map.put(20, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_SAT_MOD, Encode::from_double(value, self.zone));
    }

    // Java: double satOff()
    pub fn satOff(&self) -> f64 {
      // Java: if (!this.map.has(21))
      if !self.map.has(FIELD_SAT_OFF)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.satOff();
         return self.parent.as_ref().map_or(0.0, |p| p.satOff());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(21);
         let bin = self.map.byKey(FIELD_SAT_OFF);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasSatOff()
    pub fn hasSatOff(&self) -> bool {
      // Java: return this.map.has(21) || this.parent != null && this.parent.hasSatOff();
      self.map.has(FIELD_SAT_OFF) || self.parent.as_ref().map_or(false, |p| p.hasSatOff())
    }

    // Java: void deleteSatOff()
    pub fn deleteSatOff(&mut self) {
      // Java: this.map.put(21, (KoBin)null);
      self.map.clear_key(FIELD_SAT_OFF);
    }

    // Java: void putSatOff(double value)
    pub fn putSatOff(&mut self, value: f64) {
      // Java: this.map.put(21, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_SAT_OFF, Encode::from_double(value, self.zone));
    }

    // Java: double lum()
    pub fn lum(&self) -> f64 {
      // Java: if (!this.map.has(22))
      if !self.map.has(FIELD_LUM)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.lum();
         return self.parent.as_ref().map_or(0.0, |p| p.lum());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(22);
         let bin = self.map.byKey(FIELD_LUM);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasLum()
    pub fn hasLum(&self) -> bool {
      // Java: return this.map.has(22) || this.parent != null && this.parent.hasLum();
      self.map.has(FIELD_LUM) || self.parent.as_ref().map_or(false, |p| p.hasLum())
    }

    // Java: void deleteLum()
    pub fn deleteLum(&mut self) {
      // Java: this.map.put(22, (KoBin)null);
      self.map.clear_key(FIELD_LUM);
    }

    // Java: void putLum(double value)
    pub fn putLum(&mut self, value: f64) {
      // Java: this.map.put(22, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_LUM, Encode::from_double(value, self.zone));
    }

    // Java: double lumMod()
    pub fn lumMod(&self) -> f64 {
      // Java: if (!this.map.has(23))
      if !self.map.has(FIELD_LUM_MOD)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.lumMod();
         return self.parent.as_ref().map_or(0.0, |p| p.lumMod());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(23);
         let bin = self.map.byKey(FIELD_LUM_MOD);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasLumMod()
    pub fn hasLumMod(&self) -> bool {
      // Java: return this.map.has(23) || this.parent != null && this.parent.hasLumMod();
      self.map.has(FIELD_LUM_MOD) || self.parent.as_ref().map_or(false, |p| p.hasLumMod())
    }

    // Java: void deleteLumMod()
    pub fn deleteLumMod(&mut self) {
      // Java: this.map.put(23, (KoBin)null);
      self.map.clear_key(FIELD_LUM_MOD);
    }

    // Java: void putLumMod(double value)
    pub fn putLumMod(&mut self, value: f64) {
      // Java: this.map.put(23, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_LUM_MOD, Encode::from_double(value, self.zone));
    }

    // Java: double lumOff()
    pub fn lumOff(&self) -> f64 {
      // Java: if (!this.map.has(24))
      if !self.map.has(FIELD_LUM_OFF)
      {
         // Java: return this.parent == null ? (double)0.0F : this.parent.lumOff();
         return self.parent.as_ref().map_or(0.0, |p| p.lumOff());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(24);
         let bin = self.map.byKey(FIELD_LUM_OFF);
         // Java: return KoBin.Decode.toDouble(bin);
         return Decode::toDouble(bin.as_ref());
      }
    }

    // Java: boolean hasLumOff()
    pub fn hasLumOff(&self) -> bool {
      // Java: return this.map.has(24) || this.parent != null && this.parent.hasLumOff();
      self.map.has(FIELD_LUM_OFF) || self.parent.as_ref().map_or(false, |p| p.hasLumOff())
    }

    // Java: void deleteLumOff()
    pub fn deleteLumOff(&mut self) {
      // Java: this.map.put(24, (KoBin)null);
      self.map.clear_key(FIELD_LUM_OFF);
    }

    // Java: void putLumOff(double value)
    pub fn putLumOff(&mut self, value: f64) {
      // Java: this.map.put(24, KoBin.Encode.from(value, this.zone));
      self.map.put(FIELD_LUM_OFF, Encode::from_double(value, self.zone));
    }

}

fn double_hash_code(v: f64) -> i32 {
    let bits = v.to_bits() as i64;
    (bits ^ (bits >> 32)) as i32
}

impl KoAr for ColorTransform {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { ColorTransform::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { ColorTransform::lastBin(self, var1) }
}

impl KoClosable for ColorTransform {
    fn close(&mut self) { ColorTransform::close(self); }
}
