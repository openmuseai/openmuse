#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.color.Color
use crate::cn::wps::drawing::data::color::Color::Color;
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

// Java: public class ColorScheme implements KoAr, KoClosable
pub struct ColorScheme {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Color dk1
    pub(crate) dk1: Option<Box<Color>>,
    // Java field: Color lt1
    pub(crate) lt1: Option<Box<Color>>,
    // Java field: Color dk2
    pub(crate) dk2: Option<Box<Color>>,
    // Java field: Color lt2
    pub(crate) lt2: Option<Box<Color>>,
    // Java field: Color accent1
    pub(crate) accent1: Option<Box<Color>>,
    // Java field: Color accent2
    pub(crate) accent2: Option<Box<Color>>,
    // Java field: Color accent3
    pub(crate) accent3: Option<Box<Color>>,
    // Java field: Color accent4
    pub(crate) accent4: Option<Box<Color>>,
    // Java field: Color accent5
    pub(crate) accent5: Option<Box<Color>>,
    // Java field: Color accent6
    pub(crate) accent6: Option<Box<Color>>,
    // Java field: Color hlink
    pub(crate) hlink: Option<Box<Color>>,
    // Java field: Color folHlink
    pub(crate) folHlink: Option<Box<Color>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: ColorScheme parent
    pub(crate) parent: Option<Box<ColorScheme>>,
}

impl ColorScheme {
    // Java: static protected static <Value1 extends ColorScheme> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::ColorScheme_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static ColorScheme create()
    pub fn create() -> Self {
      // Java: return initialize(ColorScheme.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static ColorScheme open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(ColorScheme.class, bin);
      Self::initialize(bin)
    }

    // Java: ColorScheme()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Self { bin_box: KpBinBox::new(), map: KpMap::new(), zone: KoZone::defaultZone(), parent: None,
        dk1: None, lt1: None, dk2: None, lt2: None, accent1: None, accent2: None, accent3: None, accent4: None, accent5: None, accent6: None, hlink: None, folHlink: None }
    }

    // Java: ColorScheme(KoBin bin)
    pub fn ColorScheme_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Self { bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), zone: KoZone::defaultZone(), parent: None,
        dk1: None, lt1: None, dk2: None, lt2: None, accent1: None, accent2: None, accent3: None, accent4: None, accent5: None, accent6: None, hlink: None, folHlink: None }
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

    // Java: void load(ColorScheme value)
    pub fn load_value(&mut self, value: Option<&ColorScheme>) {
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
      // Java: if (this.dk1 != null && this.dk1.dirty())
        if self.dk1.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(2, this.dk1.saveBin(zone));
         let bin = self.dk1.as_mut().unwrap().saveBin(zone);
         self.map.put(2, bin);
      }

      // Java: if (this.lt1 != null && this.lt1.dirty())
        if self.lt1.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(3, this.lt1.saveBin(zone));
         let bin = self.lt1.as_mut().unwrap().saveBin(zone);
         self.map.put(3, bin);
      }

      // Java: if (this.dk2 != null && this.dk2.dirty())
        if self.dk2.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(4, this.dk2.saveBin(zone));
         let bin = self.dk2.as_mut().unwrap().saveBin(zone);
         self.map.put(4, bin);
      }

      // Java: if (this.lt2 != null && this.lt2.dirty())
        if self.lt2.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(5, this.lt2.saveBin(zone));
         let bin = self.lt2.as_mut().unwrap().saveBin(zone);
         self.map.put(5, bin);
      }

      // Java: if (this.accent1 != null && this.accent1.dirty())
        if self.accent1.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(6, this.accent1.saveBin(zone));
         let bin = self.accent1.as_mut().unwrap().saveBin(zone);
         self.map.put(6, bin);
      }

      // Java: if (this.accent2 != null && this.accent2.dirty())
        if self.accent2.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(7, this.accent2.saveBin(zone));
         let bin = self.accent2.as_mut().unwrap().saveBin(zone);
         self.map.put(7, bin);
      }

      // Java: if (this.accent3 != null && this.accent3.dirty())
        if self.accent3.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(8, this.accent3.saveBin(zone));
         let bin = self.accent3.as_mut().unwrap().saveBin(zone);
         self.map.put(8, bin);
      }

      // Java: if (this.accent4 != null && this.accent4.dirty())
        if self.accent4.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(9, this.accent4.saveBin(zone));
         let bin = self.accent4.as_mut().unwrap().saveBin(zone);
         self.map.put(9, bin);
      }

      // Java: if (this.accent5 != null && this.accent5.dirty())
        if self.accent5.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(10, this.accent5.saveBin(zone));
         let bin = self.accent5.as_mut().unwrap().saveBin(zone);
         self.map.put(10, bin);
      }

      // Java: if (this.accent6 != null && this.accent6.dirty())
        if self.accent6.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(11, this.accent6.saveBin(zone));
         let bin = self.accent6.as_mut().unwrap().saveBin(zone);
         self.map.put(11, bin);
      }

      // Java: if (this.hlink != null && this.hlink.dirty())
        if self.hlink.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(12, this.hlink.saveBin(zone));
         let bin = self.hlink.as_mut().unwrap().saveBin(zone);
         self.map.put(12, bin);
      }

      // Java: if (this.folHlink != null && this.folHlink.dirty())
        if self.folHlink.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(13, this.folHlink.saveBin(zone));
         let bin = self.folHlink.as_mut().unwrap().saveBin(zone);
         self.map.put(13, bin);
      }

      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(ColorScheme o)
    pub fn equalsLax(&self, o: &ColorScheme) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &ColorScheme) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &ColorScheme, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof ColorScheme)
        if true
      {
         // Java: ColorScheme dst = (ColorScheme)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasName() || !dst.hasName()) && (!this.hasName() || dst.hasName()))
            else if (self.hasName() || !dst.hasName()) && (!self.hasName() || dst.hasName())
         {
            // Java: if (this.hasName() && dst.hasName() && !this.name().equals(dst.name()))
               if self.hasName() && dst.hasName() && self.name() != dst.name()
            {
               // Java: return false;
               return false;
            }
              // Java: else if ((this.hasDk1() || !dst.hasDk1()) && (!this.hasDk1() || dst.hasDk1()))
                 if (self.hasDk1() || !dst.hasDk1()) && (!self.hasDk1() || dst.hasDk1())
              {
                 // Java: if (this.hasDk1() && dst.hasDk1())
                    if self.hasDk1() && dst.hasDk1()
                 {
                    // Java: if (isStrict && !this.dk1().equals(dst.dk1()))
                       if isStrict
                    {
                       // Java: return false;
                    }

                    // Java: if (!isStrict && !this.dk1().equalsLax(dst.dk1()))
                       if !isStrict
                    {
                       // Java: return false;
                    }
                 }

                 // Java: if ((this.hasLt1() || !dst.hasLt1()) && (!this.hasLt1() || dst.hasLt1()))
                    if (self.hasLt1() || !dst.hasLt1()) && (!self.hasLt1() || dst.hasLt1())
                 {
                    // Java: if (this.hasLt1() && dst.hasLt1())
                       if self.hasLt1() && dst.hasLt1()
                    {
                       // Java: if (isStrict && !this.lt1().equals(dst.lt1()))
                          if isStrict
                       {
                          // Java: return false;
                       }

                       // Java: if (!isStrict && !this.lt1().equalsLax(dst.lt1()))
                          if !isStrict
                       {
                          // Java: return false;
                       }
                    }

                    // Java: if ((this.hasDk2() || !dst.hasDk2()) && (!this.hasDk2() || dst.hasDk2()))
                       if (self.hasDk2() || !dst.hasDk2()) && (!self.hasDk2() || dst.hasDk2())
                    {
                       // Java: if (this.hasDk2() && dst.hasDk2())
                          if self.hasDk2() && dst.hasDk2()
                       {
                          // Java: if (isStrict && !this.dk2().equals(dst.dk2()))
                             if isStrict
                          {
                             // Java: return false;
                          }

                          // Java: if (!isStrict && !this.dk2().equalsLax(dst.dk2()))
                             if !isStrict
                          {
                             // Java: return false;
                          }
                       }

                       // Java: if ((this.hasLt2() || !dst.hasLt2()) && (!this.hasLt2() || dst.hasLt2()))
                          if (self.hasLt2() || !dst.hasLt2()) && (!self.hasLt2() || dst.hasLt2())
                       {
                          // Java: if (this.hasLt2() && dst.hasLt2())
                             if self.hasLt2() && dst.hasLt2()
                          {
                             // Java: if (isStrict && !this.lt2().equals(dst.lt2()))
                                if isStrict
                             {
                                // Java: return false;
                             }

                             // Java: if (!isStrict && !this.lt2().equalsLax(dst.lt2()))
                                if !isStrict
                             {
                                // Java: return false;
                             }
                          }

                          // Java: if ((this.hasAccent1() || !dst.hasAccent1()) && (!this.hasAccent1() || dst.hasAccent1()))
                             if (self.hasAccent1() || !dst.hasAccent1()) && (!self.hasAccent1() || dst.hasAccent1())
                          {
                             // Java: if (this.hasAccent1() && dst.hasAccent1())
                                if self.hasAccent1() && dst.hasAccent1()
                             {
                                // Java: if (isStrict && !this.accent1().equals(dst.accent1()))
                                   if isStrict
                                {
                                   // Java: return false;
                                }

                                // Java: if (!isStrict && !this.accent1().equalsLax(dst.accent1()))
                                   if !isStrict
                                {
                                   // Java: return false;
                                }
                             }

                             // Java: if ((this.hasAccent2() || !dst.hasAccent2()) && (!this.hasAccent2() || dst.hasAccent2()))
                                if (self.hasAccent2() || !dst.hasAccent2()) && (!self.hasAccent2() || dst.hasAccent2())
                             {
                                // Java: if (this.hasAccent2() && dst.hasAccent2())
                                   if self.hasAccent2() && dst.hasAccent2()
                                {
                                   // Java: if (isStrict && !this.accent2().equals(dst.accent2()))
                                      if isStrict
                                   {
                                      // Java: return false;
                                   }

                                   // Java: if (!isStrict && !this.accent2().equalsLax(dst.accent2()))
                                      if !isStrict
                                   {
                                      // Java: return false;
                                   }
                                }

                                // Java: if ((this.hasAccent3() || !dst.hasAccent3()) && (!this.hasAccent3() || dst.hasAccent3()))
                                   if (self.hasAccent3() || !dst.hasAccent3()) && (!self.hasAccent3() || dst.hasAccent3())
                                {
                                   // Java: if (this.hasAccent3() && dst.hasAccent3())
                                      if self.hasAccent3() && dst.hasAccent3()
                                   {
                                      // Java: if (isStrict && !this.accent3().equals(dst.accent3()))
                                         if isStrict
                                      {
                                         // Java: return false;
                                      }

                                      // Java: if (!isStrict && !this.accent3().equalsLax(dst.accent3()))
                                         if !isStrict
                                      {
                                         // Java: return false;
                                      }
                                   }

                                   // Java: if ((this.hasAccent4() || !dst.hasAccent4()) && (!this.hasAccent4() || dst.hasAccent4()))
                                      if (self.hasAccent4() || !dst.hasAccent4()) && (!self.hasAccent4() || dst.hasAccent4())
                                   {
                                      // Java: if (this.hasAccent4() && dst.hasAccent4())
                                         if self.hasAccent4() && dst.hasAccent4()
                                      {
                                         // Java: if (isStrict && !this.accent4().equals(dst.accent4()))
                                            if isStrict
                                         {
                                            // Java: return false;
                                         }

                                         // Java: if (!isStrict && !this.accent4().equalsLax(dst.accent4()))
                                            if !isStrict
                                         {
                                            // Java: return false;
                                         }
                                      }

                                      // Java: if ((this.hasAccent5() || !dst.hasAccent5()) && (!this.hasAccent5() || dst.hasAccent5()))
                                         if (self.hasAccent5() || !dst.hasAccent5()) && (!self.hasAccent5() || dst.hasAccent5())
                                      {
                                         // Java: if (this.hasAccent5() && dst.hasAccent5())
                                            if self.hasAccent5() && dst.hasAccent5()
                                         {
                                            // Java: if (isStrict && !this.accent5().equals(dst.accent5()))
                                               if isStrict
                                            {
                                               // Java: return false;
                                            }

                                            // Java: if (!isStrict && !this.accent5().equalsLax(dst.accent5()))
                                               if !isStrict
                                            {
                                               // Java: return false;
                                            }
                                         }

                                         // Java: if ((this.hasAccent6() || !dst.hasAccent6()) && (!this.hasAccent6() || dst.hasAccent6()))
                                            if (self.hasAccent6() || !dst.hasAccent6()) && (!self.hasAccent6() || dst.hasAccent6())
                                         {
                                            // Java: if (this.hasAccent6() && dst.hasAccent6())
                                               if self.hasAccent6() && dst.hasAccent6()
                                            {
                                               // Java: if (isStrict && !this.accent6().equals(dst.accent6()))
                                                  if isStrict
                                               {
                                                  // Java: return false;
                                               }

                                               // Java: if (!isStrict && !this.accent6().equalsLax(dst.accent6()))
                                                  if !isStrict
                                               {
                                                  // Java: return false;
                                               }
                                            }

                                            // Java: if ((this.hasHlink() || !dst.hasHlink()) && (!this.hasHlink() || dst.hasHlink()))
                                               if (self.hasHlink() || !dst.hasHlink()) && (!self.hasHlink() || dst.hasHlink())
                                            {
                                               // Java: if (this.hasHlink() && dst.hasHlink())
                                                  if self.hasHlink() && dst.hasHlink()
                                               {
                                                  // Java: if (isStrict && !this.hlink().equals(dst.hlink()))
                                                     if isStrict
                                                  {
                                                     // Java: return false;
                                                  }

                                                  // Java: if (!isStrict && !this.hlink().equalsLax(dst.hlink()))
                                                     if !isStrict
                                                  {
                                                     // Java: return false;
                                                  }
                                               }

                                               // Java: if ((this.hasFolHlink() || !dst.hasFolHlink()) && (!this.hasFolHlink() || dst.hasFolHlink()))
                                                  if (self.hasFolHlink() || !dst.hasFolHlink()) && (!self.hasFolHlink() || dst.hasFolHlink())
                                               {
                                                  // Java: if (this.hasFolHlink() && dst.hasFolHlink())
                                                     if self.hasFolHlink() && dst.hasFolHlink()
                                                  {
                                                     // Java: if (isStrict && !this.folHlink().equals(dst.folHlink()))
                                                        if isStrict
                                                     {
                                                        // Java: return false;
                                                     }

                                                     // Java: if (!isStrict && !this.folHlink().equalsLax(dst.folHlink()))
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
      // Java: if (this.hasName())
        if self.hasName()
      {
         // Java: ret += String.valueOf(this.name()).hashCode();
         todo!()
      }
      // Java: if (this.hasDk1())
        if self.hasDk1()
      {
         // Java: ret += this.dk1().hashCode();
         todo!()
      }
      // Java: if (this.hasLt1())
        if self.hasLt1()
      {
         // Java: ret += this.lt1().hashCode();
         todo!()
      }
      // Java: if (this.hasDk2())
        if self.hasDk2()
      {
         // Java: ret += this.dk2().hashCode();
         todo!()
      }
      // Java: if (this.hasLt2())
        if self.hasLt2()
      {
         // Java: ret += this.lt2().hashCode();
         todo!()
      }
      // Java: if (this.hasAccent1())
        if self.hasAccent1()
      {
         // Java: ret += this.accent1().hashCode();
         todo!()
      }
      // Java: if (this.hasAccent2())
        if self.hasAccent2()
      {
         // Java: ret += this.accent2().hashCode();
         todo!()
      }
      // Java: if (this.hasAccent3())
        if self.hasAccent3()
      {
         // Java: ret += this.accent3().hashCode();
         todo!()
      }
      // Java: if (this.hasAccent4())
        if self.hasAccent4()
      {
         // Java: ret += this.accent4().hashCode();
         todo!()
      }
      // Java: if (this.hasAccent5())
        if self.hasAccent5()
      {
         // Java: ret += this.accent5().hashCode();
         todo!()
      }
      // Java: if (this.hasAccent6())
        if self.hasAccent6()
      {
         // Java: ret += this.accent6().hashCode();
         todo!()
      }
      // Java: if (this.hasHlink())
        if self.hasHlink()
      {
         // Java: ret += this.hlink().hashCode();
         todo!()
      }
      // Java: if (this.hasFolHlink())
        if self.hasFolHlink()
      {
         // Java: ret += this.folHlink().hashCode();
         todo!()
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.dk1 != null && this.dk1.dirty())
        if self.dk1.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.lt1 != null && this.lt1.dirty())
        else if self.lt1.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.dk2 != null && this.dk2.dirty())
        else if self.dk2.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.lt2 != null && this.lt2.dirty())
        else if self.lt2.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.accent1 != null && this.accent1.dirty())
        else if self.accent1.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.accent2 != null && this.accent2.dirty())
        else if self.accent2.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.accent3 != null && this.accent3.dirty())
        else if self.accent3.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.accent4 != null && this.accent4.dirty())
        else if self.accent4.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.accent5 != null && this.accent5.dirty())
        else if self.accent5.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.accent6 != null && this.accent6.dirty())
        else if self.accent6.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.hlink != null && this.hlink.dirty())
        else if self.hlink.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
        else
      {
         // Java: return this.folHlink != null && this.folHlink.dirty() ? true : this.map.dirty();
         return self.folHlink.as_ref().map_or(false, |v| v.dirty()) || self.map.dirty();
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
      // Java: this.dk1 = null;
      self.dk1 = None;
      // Java: this.lt1 = null;
      self.lt1 = None;
      // Java: this.dk2 = null;
      self.dk2 = None;
      // Java: this.lt2 = null;
      self.lt2 = None;
      // Java: this.accent1 = null;
      self.accent1 = None;
      // Java: this.accent2 = null;
      self.accent2 = None;
      // Java: this.accent3 = null;
      self.accent3 = None;
      // Java: this.accent4 = null;
      self.accent4 = None;
      // Java: this.accent5 = null;
      self.accent5 = None;
      // Java: this.accent6 = null;
      self.accent6 = None;
      // Java: this.hlink = null;
      self.hlink = None;
      // Java: this.folHlink = null;
      self.folHlink = None;
    }

    // Java: void follow(ColorScheme parent)
    pub fn follow(&mut self, parent: Option<&mut ColorScheme>) {
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

    // Java: ColorScheme parent()
    pub fn parent(&self) -> Option<&ColorScheme> {
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

    // Java: String name()
    pub fn name(&self) -> String {
      // Java: if (!this.map.has(1))
        if !self.map.has(1)
      {
         // Java: return this.parent == null ? "" : this.parent.name();
         return self.parent.as_ref().map_or(String::new(), |p| p.name());
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

    // Java: boolean hasName()
    pub fn hasName(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasName();
      self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasName())
    }

    // Java: void deleteName()
    pub fn deleteName(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
    }

    // Java: void putName(String value)
    pub fn putName(&mut self, value: &str) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(1, Encode::from_string(value, self.zone));
    }

    // Java: Color dk1()
    pub fn dk1(&mut self) -> &Color {
      // Java: if (this.dk1 == null)
        if self.dk1.is_none()
      {
         // Java: if (this.map.has(2))
            if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            // Java: this.dk1 = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let bin = self.map.byKey(2);
            self.dk1 = Some(Box::new(Color::Color_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasDk1())
            else if self.parent.as_ref().map_or(false, |p| p.hasDk1())
         {
            // Java: this.dk1 = this.parent.dk1();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.dk1 = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let null_bin = Null::empty();
            self.dk1 = Some(Box::new(Color::Color_from(null_bin.as_ref())));
         }
      }
      // Java: return this.dk1;
      self.dk1.as_deref().unwrap()
    }

    // Java: boolean hasDk1()
    pub fn hasDk1(&self) -> bool {
      // Java: return this.map.has(2) || this.dk1 != null && this.dk1.dirty() || this.parent != null && this.parent.hasDk1();
      self.map.has(2) || self.dk1.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasDk1())
    }

    // Java: void deleteDk1()
    pub fn deleteDk1(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.dk1 != null)
        if self.dk1.is_some()
      {
         // Java: this.dk1 = null;
         self.dk1 = None;
      }
    }

    // Java: void writeDk1(Color value)
    pub fn writeDk1(&mut self, value: Option<&Color>) {
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
            // Java: if (this.dk1 != null)
               if self.dk1.is_some()
            {
               // Java: this.dk1 = null;
               self.dk1 = None;
            }
         }
      }
    }

    // Java: void forceWriteDk1(Color value)
    pub fn forceWriteDk1(&mut self, value: Option<&Color>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.dk1 != null)
            if self.dk1.is_some()
         {
            // Java: this.dk1 = null;
            self.dk1 = None;
         }
      }
    }

    // Java: Color lt1()
    pub fn lt1(&mut self) -> &Color {
      // Java: if (this.lt1 == null)
        if self.lt1.is_none()
      {
         // Java: if (this.map.has(3))
            if self.map.has(3)
         {
            // Java: KoBin bin = this.map.byKey(3);
            // Java: this.lt1 = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let bin = self.map.byKey(3);
            self.lt1 = Some(Box::new(Color::Color_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasLt1())
            else if self.parent.as_ref().map_or(false, |p| p.hasLt1())
         {
            // Java: this.lt1 = this.parent.lt1();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.lt1 = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let null_bin = Null::empty();
            self.lt1 = Some(Box::new(Color::Color_from(null_bin.as_ref())));
         }
      }
      // Java: return this.lt1;
      self.lt1.as_deref().unwrap()
    }

    // Java: boolean hasLt1()
    pub fn hasLt1(&self) -> bool {
      // Java: return this.map.has(3) || this.lt1 != null && this.lt1.dirty() || this.parent != null && this.parent.hasLt1();
      self.map.has(3) || self.lt1.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasLt1())
    }

    // Java: void deleteLt1()
    pub fn deleteLt1(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
      // Java: if (this.lt1 != null)
        if self.lt1.is_some()
      {
         // Java: this.lt1 = null;
         self.lt1 = None;
      }
    }

    // Java: void writeLt1(Color value)
    pub fn writeLt1(&mut self, value: Option<&Color>) {
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
            // Java: if (this.lt1 != null)
               if self.lt1.is_some()
            {
               // Java: this.lt1 = null;
               self.lt1 = None;
            }
         }
      }
    }

    // Java: void forceWriteLt1(Color value)
    pub fn forceWriteLt1(&mut self, value: Option<&Color>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(3, value.lastBin(this.zone));
         self.map.put(3, v.lastBin(self.zone));
         // Java: if (this.lt1 != null)
            if self.lt1.is_some()
         {
            // Java: this.lt1 = null;
            self.lt1 = None;
         }
      }
    }

    // Java: Color dk2()
    pub fn dk2(&mut self) -> &Color {
      // Java: if (this.dk2 == null)
        if self.dk2.is_none()
      {
         // Java: if (this.map.has(4))
            if self.map.has(4)
         {
            // Java: KoBin bin = this.map.byKey(4);
            // Java: this.dk2 = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let bin = self.map.byKey(4);
            self.dk2 = Some(Box::new(Color::Color_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasDk2())
            else if self.parent.as_ref().map_or(false, |p| p.hasDk2())
         {
            // Java: this.dk2 = this.parent.dk2();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.dk2 = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let null_bin = Null::empty();
            self.dk2 = Some(Box::new(Color::Color_from(null_bin.as_ref())));
         }
      }
      // Java: return this.dk2;
      self.dk2.as_deref().unwrap()
    }

    // Java: boolean hasDk2()
    pub fn hasDk2(&self) -> bool {
      // Java: return this.map.has(4) || this.dk2 != null && this.dk2.dirty() || this.parent != null && this.parent.hasDk2();
      self.map.has(4) || self.dk2.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasDk2())
    }

    // Java: void deleteDk2()
    pub fn deleteDk2(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
      // Java: if (this.dk2 != null)
        if self.dk2.is_some()
      {
         // Java: this.dk2 = null;
         self.dk2 = None;
      }
    }

    // Java: void writeDk2(Color value)
    pub fn writeDk2(&mut self, value: Option<&Color>) {
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
            // Java: if (this.dk2 != null)
               if self.dk2.is_some()
            {
               // Java: this.dk2 = null;
               self.dk2 = None;
            }
         }
      }
    }

    // Java: void forceWriteDk2(Color value)
    pub fn forceWriteDk2(&mut self, value: Option<&Color>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(4, value.lastBin(this.zone));
         self.map.put(4, v.lastBin(self.zone));
         // Java: if (this.dk2 != null)
            if self.dk2.is_some()
         {
            // Java: this.dk2 = null;
            self.dk2 = None;
         }
      }
    }

    // Java: Color lt2()
    pub fn lt2(&mut self) -> &Color {
      // Java: if (this.lt2 == null)
        if self.lt2.is_none()
      {
         // Java: if (this.map.has(5))
            if self.map.has(5)
         {
            // Java: KoBin bin = this.map.byKey(5);
            // Java: this.lt2 = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let bin = self.map.byKey(5);
            self.lt2 = Some(Box::new(Color::Color_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasLt2())
            else if self.parent.as_ref().map_or(false, |p| p.hasLt2())
         {
            // Java: this.lt2 = this.parent.lt2();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.lt2 = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let null_bin = Null::empty();
            self.lt2 = Some(Box::new(Color::Color_from(null_bin.as_ref())));
         }
      }
      // Java: return this.lt2;
      self.lt2.as_deref().unwrap()
    }

    // Java: boolean hasLt2()
    pub fn hasLt2(&self) -> bool {
      // Java: return this.map.has(5) || this.lt2 != null && this.lt2.dirty() || this.parent != null && this.parent.hasLt2();
      self.map.has(5) || self.lt2.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasLt2())
    }

    // Java: void deleteLt2()
    pub fn deleteLt2(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(5);
      // Java: if (this.lt2 != null)
        if self.lt2.is_some()
      {
         // Java: this.lt2 = null;
         self.lt2 = None;
      }
    }

    // Java: void writeLt2(Color value)
    pub fn writeLt2(&mut self, value: Option<&Color>) {
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
            // Java: if (this.lt2 != null)
               if self.lt2.is_some()
            {
               // Java: this.lt2 = null;
               self.lt2 = None;
            }
         }
      }
    }

    // Java: void forceWriteLt2(Color value)
    pub fn forceWriteLt2(&mut self, value: Option<&Color>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(5, value.lastBin(this.zone));
         self.map.put(5, v.lastBin(self.zone));
         // Java: if (this.lt2 != null)
            if self.lt2.is_some()
         {
            // Java: this.lt2 = null;
            self.lt2 = None;
         }
      }
    }

    // Java: Color accent1()
    pub fn accent1(&mut self) -> &Color {
      // Java: if (this.accent1 == null)
        if self.accent1.is_none()
      {
         // Java: if (this.map.has(6))
            if self.map.has(6)
         {
            // Java: KoBin bin = this.map.byKey(6);
            // Java: this.accent1 = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let bin = self.map.byKey(6);
            self.accent1 = Some(Box::new(Color::Color_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasAccent1())
            else if self.parent.as_ref().map_or(false, |p| p.hasAccent1())
         {
            // Java: this.accent1 = this.parent.accent1();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.accent1 = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let null_bin = Null::empty();
            self.accent1 = Some(Box::new(Color::Color_from(null_bin.as_ref())));
         }
      }
      // Java: return this.accent1;
      self.accent1.as_deref().unwrap()
    }

    // Java: boolean hasAccent1()
    pub fn hasAccent1(&self) -> bool {
      // Java: return this.map.has(6) || this.accent1 != null && this.accent1.dirty() || this.parent != null && this.parent.hasAccent1();
      self.map.has(6) || self.accent1.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasAccent1())
    }

    // Java: void deleteAccent1()
    pub fn deleteAccent1(&mut self) {
      // Java: this.map.put(6, (KoBin)null);
      self.map.clear_key(6);
      // Java: if (this.accent1 != null)
        if self.accent1.is_some()
      {
         // Java: this.accent1 = null;
         self.accent1 = None;
      }
    }

    // Java: void writeAccent1(Color value)
    pub fn writeAccent1(&mut self, value: Option<&Color>) {
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
            // Java: if (this.accent1 != null)
               if self.accent1.is_some()
            {
               // Java: this.accent1 = null;
               self.accent1 = None;
            }
         }
      }
    }

    // Java: void forceWriteAccent1(Color value)
    pub fn forceWriteAccent1(&mut self, value: Option<&Color>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(6, value.lastBin(this.zone));
         self.map.put(6, v.lastBin(self.zone));
         // Java: if (this.accent1 != null)
            if self.accent1.is_some()
         {
            // Java: this.accent1 = null;
            self.accent1 = None;
         }
      }
    }

    // Java: Color accent2()
    pub fn accent2(&mut self) -> &Color {
      // Java: if (this.accent2 == null)
        if self.accent2.is_none()
      {
         // Java: if (this.map.has(7))
            if self.map.has(7)
         {
            // Java: KoBin bin = this.map.byKey(7);
            // Java: this.accent2 = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let bin = self.map.byKey(7);
            self.accent2 = Some(Box::new(Color::Color_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasAccent2())
            else if self.parent.as_ref().map_or(false, |p| p.hasAccent2())
         {
            // Java: this.accent2 = this.parent.accent2();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.accent2 = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let null_bin = Null::empty();
            self.accent2 = Some(Box::new(Color::Color_from(null_bin.as_ref())));
         }
      }
      // Java: return this.accent2;
      self.accent2.as_deref().unwrap()
    }

    // Java: boolean hasAccent2()
    pub fn hasAccent2(&self) -> bool {
      // Java: return this.map.has(7) || this.accent2 != null && this.accent2.dirty() || this.parent != null && this.parent.hasAccent2();
      self.map.has(7) || self.accent2.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasAccent2())
    }

    // Java: void deleteAccent2()
    pub fn deleteAccent2(&mut self) {
      // Java: this.map.put(7, (KoBin)null);
      self.map.clear_key(7);
      // Java: if (this.accent2 != null)
        if self.accent2.is_some()
      {
         // Java: this.accent2 = null;
         self.accent2 = None;
      }
    }

    // Java: void writeAccent2(Color value)
    pub fn writeAccent2(&mut self, value: Option<&Color>) {
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
            // Java: if (this.accent2 != null)
               if self.accent2.is_some()
            {
               // Java: this.accent2 = null;
               self.accent2 = None;
            }
         }
      }
    }

    // Java: void forceWriteAccent2(Color value)
    pub fn forceWriteAccent2(&mut self, value: Option<&Color>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(7, value.lastBin(this.zone));
         self.map.put(7, v.lastBin(self.zone));
         // Java: if (this.accent2 != null)
            if self.accent2.is_some()
         {
            // Java: this.accent2 = null;
            self.accent2 = None;
         }
      }
    }

    // Java: Color accent3()
    pub fn accent3(&mut self) -> &Color {
      // Java: if (this.accent3 == null)
        if self.accent3.is_none()
      {
         // Java: if (this.map.has(8))
            if self.map.has(8)
         {
            // Java: KoBin bin = this.map.byKey(8);
            // Java: this.accent3 = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let bin = self.map.byKey(8);
            self.accent3 = Some(Box::new(Color::Color_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasAccent3())
            else if self.parent.as_ref().map_or(false, |p| p.hasAccent3())
         {
            // Java: this.accent3 = this.parent.accent3();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.accent3 = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let null_bin = Null::empty();
            self.accent3 = Some(Box::new(Color::Color_from(null_bin.as_ref())));
         }
      }
      // Java: return this.accent3;
      self.accent3.as_deref().unwrap()
    }

    // Java: boolean hasAccent3()
    pub fn hasAccent3(&self) -> bool {
      // Java: return this.map.has(8) || this.accent3 != null && this.accent3.dirty() || this.parent != null && this.parent.hasAccent3();
      self.map.has(8) || self.accent3.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasAccent3())
    }

    // Java: void deleteAccent3()
    pub fn deleteAccent3(&mut self) {
      // Java: this.map.put(8, (KoBin)null);
      self.map.clear_key(8);
      // Java: if (this.accent3 != null)
        if self.accent3.is_some()
      {
         // Java: this.accent3 = null;
         self.accent3 = None;
      }
    }

    // Java: void writeAccent3(Color value)
    pub fn writeAccent3(&mut self, value: Option<&Color>) {
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
            // Java: if (this.accent3 != null)
               if self.accent3.is_some()
            {
               // Java: this.accent3 = null;
               self.accent3 = None;
            }
         }
      }
    }

    // Java: void forceWriteAccent3(Color value)
    pub fn forceWriteAccent3(&mut self, value: Option<&Color>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(8, value.lastBin(this.zone));
         self.map.put(8, v.lastBin(self.zone));
         // Java: if (this.accent3 != null)
            if self.accent3.is_some()
         {
            // Java: this.accent3 = null;
            self.accent3 = None;
         }
      }
    }

    // Java: Color accent4()
    pub fn accent4(&mut self) -> &Color {
      // Java: if (this.accent4 == null)
        if self.accent4.is_none()
      {
         // Java: if (this.map.has(9))
            if self.map.has(9)
         {
            // Java: KoBin bin = this.map.byKey(9);
            // Java: this.accent4 = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let bin = self.map.byKey(9);
            self.accent4 = Some(Box::new(Color::Color_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasAccent4())
            else if self.parent.as_ref().map_or(false, |p| p.hasAccent4())
         {
            // Java: this.accent4 = this.parent.accent4();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.accent4 = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let null_bin = Null::empty();
            self.accent4 = Some(Box::new(Color::Color_from(null_bin.as_ref())));
         }
      }
      // Java: return this.accent4;
      self.accent4.as_deref().unwrap()
    }

    // Java: boolean hasAccent4()
    pub fn hasAccent4(&self) -> bool {
      // Java: return this.map.has(9) || this.accent4 != null && this.accent4.dirty() || this.parent != null && this.parent.hasAccent4();
      self.map.has(9) || self.accent4.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasAccent4())
    }

    // Java: void deleteAccent4()
    pub fn deleteAccent4(&mut self) {
      // Java: this.map.put(9, (KoBin)null);
      self.map.clear_key(9);
      // Java: if (this.accent4 != null)
        if self.accent4.is_some()
      {
         // Java: this.accent4 = null;
         self.accent4 = None;
      }
    }

    // Java: void writeAccent4(Color value)
    pub fn writeAccent4(&mut self, value: Option<&Color>) {
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
            // Java: if (this.accent4 != null)
               if self.accent4.is_some()
            {
               // Java: this.accent4 = null;
               self.accent4 = None;
            }
         }
      }
    }

    // Java: void forceWriteAccent4(Color value)
    pub fn forceWriteAccent4(&mut self, value: Option<&Color>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(9, value.lastBin(this.zone));
         self.map.put(9, v.lastBin(self.zone));
         // Java: if (this.accent4 != null)
            if self.accent4.is_some()
         {
            // Java: this.accent4 = null;
            self.accent4 = None;
         }
      }
    }

    // Java: Color accent5()
    pub fn accent5(&mut self) -> &Color {
      // Java: if (this.accent5 == null)
        if self.accent5.is_none()
      {
         // Java: if (this.map.has(10))
            if self.map.has(10)
         {
            // Java: KoBin bin = this.map.byKey(10);
            // Java: this.accent5 = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let bin = self.map.byKey(10);
            self.accent5 = Some(Box::new(Color::Color_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasAccent5())
            else if self.parent.as_ref().map_or(false, |p| p.hasAccent5())
         {
            // Java: this.accent5 = this.parent.accent5();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.accent5 = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let null_bin = Null::empty();
            self.accent5 = Some(Box::new(Color::Color_from(null_bin.as_ref())));
         }
      }
      // Java: return this.accent5;
      self.accent5.as_deref().unwrap()
    }

    // Java: boolean hasAccent5()
    pub fn hasAccent5(&self) -> bool {
      // Java: return this.map.has(10) || this.accent5 != null && this.accent5.dirty() || this.parent != null && this.parent.hasAccent5();
      self.map.has(10) || self.accent5.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasAccent5())
    }

    // Java: void deleteAccent5()
    pub fn deleteAccent5(&mut self) {
      // Java: this.map.put(10, (KoBin)null);
      self.map.clear_key(10);
      // Java: if (this.accent5 != null)
        if self.accent5.is_some()
      {
         // Java: this.accent5 = null;
         self.accent5 = None;
      }
    }

    // Java: void writeAccent5(Color value)
    pub fn writeAccent5(&mut self, value: Option<&Color>) {
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
            // Java: if (this.accent5 != null)
               if self.accent5.is_some()
            {
               // Java: this.accent5 = null;
               self.accent5 = None;
            }
         }
      }
    }

    // Java: void forceWriteAccent5(Color value)
    pub fn forceWriteAccent5(&mut self, value: Option<&Color>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(10, value.lastBin(this.zone));
         self.map.put(10, v.lastBin(self.zone));
         // Java: if (this.accent5 != null)
            if self.accent5.is_some()
         {
            // Java: this.accent5 = null;
            self.accent5 = None;
         }
      }
    }

    // Java: Color accent6()
    pub fn accent6(&mut self) -> &Color {
      // Java: if (this.accent6 == null)
        if self.accent6.is_none()
      {
         // Java: if (this.map.has(11))
            if self.map.has(11)
         {
            // Java: KoBin bin = this.map.byKey(11);
            // Java: this.accent6 = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let bin = self.map.byKey(11);
            self.accent6 = Some(Box::new(Color::Color_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasAccent6())
            else if self.parent.as_ref().map_or(false, |p| p.hasAccent6())
         {
            // Java: this.accent6 = this.parent.accent6();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.accent6 = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let null_bin = Null::empty();
            self.accent6 = Some(Box::new(Color::Color_from(null_bin.as_ref())));
         }
      }
      // Java: return this.accent6;
      self.accent6.as_deref().unwrap()
    }

    // Java: boolean hasAccent6()
    pub fn hasAccent6(&self) -> bool {
      // Java: return this.map.has(11) || this.accent6 != null && this.accent6.dirty() || this.parent != null && this.parent.hasAccent6();
      self.map.has(11) || self.accent6.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasAccent6())
    }

    // Java: void deleteAccent6()
    pub fn deleteAccent6(&mut self) {
      // Java: this.map.put(11, (KoBin)null);
      self.map.clear_key(11);
      // Java: if (this.accent6 != null)
        if self.accent6.is_some()
      {
         // Java: this.accent6 = null;
         self.accent6 = None;
      }
    }

    // Java: void writeAccent6(Color value)
    pub fn writeAccent6(&mut self, value: Option<&Color>) {
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
            // Java: if (this.accent6 != null)
               if self.accent6.is_some()
            {
               // Java: this.accent6 = null;
               self.accent6 = None;
            }
         }
      }
    }

    // Java: void forceWriteAccent6(Color value)
    pub fn forceWriteAccent6(&mut self, value: Option<&Color>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(11, value.lastBin(this.zone));
         self.map.put(11, v.lastBin(self.zone));
         // Java: if (this.accent6 != null)
            if self.accent6.is_some()
         {
            // Java: this.accent6 = null;
            self.accent6 = None;
         }
      }
    }

    // Java: Color hlink()
    pub fn hlink(&mut self) -> &Color {
      // Java: if (this.hlink == null)
        if self.hlink.is_none()
      {
         // Java: if (this.map.has(12))
            if self.map.has(12)
         {
            // Java: KoBin bin = this.map.byKey(12);
            // Java: this.hlink = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let bin = self.map.byKey(12);
            self.hlink = Some(Box::new(Color::Color_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasHlink())
            else if self.parent.as_ref().map_or(false, |p| p.hasHlink())
         {
            // Java: this.hlink = this.parent.hlink();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.hlink = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let null_bin = Null::empty();
            self.hlink = Some(Box::new(Color::Color_from(null_bin.as_ref())));
         }
      }
      // Java: return this.hlink;
      self.hlink.as_deref().unwrap()
    }

    // Java: boolean hasHlink()
    pub fn hasHlink(&self) -> bool {
      // Java: return this.map.has(12) || this.hlink != null && this.hlink.dirty() || this.parent != null && this.parent.hasHlink();
      self.map.has(12) || self.hlink.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasHlink())
    }

    // Java: void deleteHlink()
    pub fn deleteHlink(&mut self) {
      // Java: this.map.put(12, (KoBin)null);
      self.map.clear_key(12);
      // Java: if (this.hlink != null)
        if self.hlink.is_some()
      {
         // Java: this.hlink = null;
         self.hlink = None;
      }
    }

    // Java: void writeHlink(Color value)
    pub fn writeHlink(&mut self, value: Option<&Color>) {
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
            // Java: if (this.hlink != null)
               if self.hlink.is_some()
            {
               // Java: this.hlink = null;
               self.hlink = None;
            }
         }
      }
    }

    // Java: void forceWriteHlink(Color value)
    pub fn forceWriteHlink(&mut self, value: Option<&Color>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(12, value.lastBin(this.zone));
         self.map.put(12, v.lastBin(self.zone));
         // Java: if (this.hlink != null)
            if self.hlink.is_some()
         {
            // Java: this.hlink = null;
            self.hlink = None;
         }
      }
    }

    // Java: Color folHlink()
    pub fn folHlink(&mut self) -> &Color {
      // Java: if (this.folHlink == null)
        if self.folHlink.is_none()
      {
         // Java: if (this.map.has(13))
            if self.map.has(13)
         {
            // Java: KoBin bin = this.map.byKey(13);
            // Java: this.folHlink = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let bin = self.map.byKey(13);
            self.folHlink = Some(Box::new(Color::Color_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasFolHlink())
            else if self.parent.as_ref().map_or(false, |p| p.hasFolHlink())
         {
            // Java: this.folHlink = this.parent.folHlink();
            todo!()
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.folHlink = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            let null_bin = Null::empty();
            self.folHlink = Some(Box::new(Color::Color_from(null_bin.as_ref())));
         }
      }
      // Java: return this.folHlink;
      self.folHlink.as_deref().unwrap()
    }

    // Java: boolean hasFolHlink()
    pub fn hasFolHlink(&self) -> bool {
      // Java: return this.map.has(13) || this.folHlink != null && this.folHlink.dirty() || this.parent != null && this.parent.hasFolHlink();
      self.map.has(13) || self.folHlink.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasFolHlink())
    }

    // Java: void deleteFolHlink()
    pub fn deleteFolHlink(&mut self) {
      // Java: this.map.put(13, (KoBin)null);
      self.map.clear_key(13);
      // Java: if (this.folHlink != null)
        if self.folHlink.is_some()
      {
         // Java: this.folHlink = null;
         self.folHlink = None;
      }
    }

    // Java: void writeFolHlink(Color value)
    pub fn writeFolHlink(&mut self, value: Option<&Color>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(13, bin);
            self.map.put(13, bin);
            // Java: if (this.folHlink != null)
               if self.folHlink.is_some()
            {
               // Java: this.folHlink = null;
               self.folHlink = None;
            }
         }
      }
    }

    // Java: void forceWriteFolHlink(Color value)
    pub fn forceWriteFolHlink(&mut self, value: Option<&Color>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(13, value.lastBin(this.zone));
         self.map.put(13, v.lastBin(self.zone));
         // Java: if (this.folHlink != null)
            if self.folHlink.is_some()
         {
            // Java: this.folHlink = null;
            self.folHlink = None;
         }
      }
    }

}

impl KoAr for ColorScheme {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for ColorScheme { fn close(&mut self) { self.close(); } }
