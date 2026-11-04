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

// Java: public class RecolorEntryBlush implements KoAr, KoClosable
pub struct RecolorEntryBlush {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Color toColor
    pub(crate) toColor: Option<Box<dyn KoAr>>,
    // Java field: Color lbColor
    pub(crate) lbColor: Option<Box<dyn KoAr>>,
    // Java field: Color fgColor
    pub(crate) fgColor: Option<Box<dyn KoAr>>,
    // Java field: Color bgColor
    pub(crate) bgColor: Option<Box<dyn KoAr>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: RecolorEntryBlush parent
    pub(crate) parent: Option<Box<RecolorEntryBlush>>,
}

// Java: public class RecolorEntryBlush implements KoAr, KoClosable
impl RecolorEntryBlush {
    // Java: static protected static <Value1 extends RecolorEntryBlush> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::RecolorEntryBlush_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static RecolorEntryBlush create()
    pub fn create() -> Self {
      // Java: return initialize(RecolorEntryBlush.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static RecolorEntryBlush open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(RecolorEntryBlush.class, bin);
      Self::initialize(bin)
    }

    // Java: RecolorEntryBlush()
    pub fn new() -> Self {
      Self {
      // Java: this.box = new KpBinBox();
      bin_box: KpBinBox::new(),
      // Java: this.map = new KpMap();
      map: KpMap::new(),
      zone: KoZone::defaultZone(), toColor: None, lbColor: None, fgColor: None, bgColor: None, parent: None,
      }
    }

    // Java: RecolorEntryBlush(KoBin bin)
    pub fn RecolorEntryBlush_from(bin: &dyn KoBinTrait) -> Self {
      Self {
      // Java: this.box = new KpBinBox(bin);
      bin_box: KpBinBox::KpBinBox_from(Some(bin)),
      // Java: this.map = new KpMap(bin);
      map: KpMap::KpMap_from(bin),
      zone: KoZone::defaultZone(), toColor: None, lbColor: None, fgColor: None, bgColor: None, parent: None,
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

    // Java: void load(RecolorEntryBlush value)
    pub fn load_value(&mut self, value: Option<&RecolorEntryBlush>) {
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
      // Java: if (this.toColor != null && this.toColor.dirty())
        if self.toColor.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(2, this.toColor.saveBin(zone));
         let saved = self.toColor.as_mut().unwrap().saveBin(zone);
         self.map.put(2, saved);
      }
      // Java: if (this.lbColor != null && this.lbColor.dirty())
        if self.lbColor.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(5, this.lbColor.saveBin(zone));
         let saved = self.lbColor.as_mut().unwrap().saveBin(zone);
         self.map.put(5, saved);
      }
      // Java: if (this.fgColor != null && this.fgColor.dirty())
        if self.fgColor.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(7, this.fgColor.saveBin(zone));
         let saved = self.fgColor.as_mut().unwrap().saveBin(zone);
         self.map.put(7, saved);
      }
      // Java: if (this.bgColor != null && this.bgColor.dirty())
        if self.bgColor.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: this.map.put(8, this.bgColor.saveBin(zone));
         let saved = self.bgColor.as_mut().unwrap().saveBin(zone);
         self.map.put(8, saved);
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

    // Java: boolean equalsLax(RecolorEntryBlush o)
    pub fn equalsLax(&self, o: &RecolorEntryBlush) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &RecolorEntryBlush) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &RecolorEntryBlush, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof RecolorEntryBlush)
        let dst = o;
      {
         // Java: RecolorEntryBlush dst = (RecolorEntryBlush)o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasFDoRecolor() || !dst.hasFDoRecolor()) && (!this.hasFDoRecolor() || dst.hasFDoRecolor()))
            else if (self.hasFDoRecolor() || !dst.hasFDoRecolor()) && (!self.hasFDoRecolor() || dst.hasFDoRecolor())
         {
            // Java: if (this.hasFDoRecolor() && dst.hasFDoRecolor() && this.isFDoRecolor() != dst.isFDoRecolor())
                if self.hasFDoRecolor() && dst.hasFDoRecolor() && self.isFDoRecolor() != dst.isFDoRecolor()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasToColor() || !dst.hasToColor()) && (!this.hasToColor() || dst.hasToColor()))
                else if (self.hasToColor() || !dst.hasToColor()) && (!self.hasToColor() || dst.hasToColor())
            {
               // Java: if (this.hasToColor() && dst.hasToColor())
                    if self.hasToColor() && dst.hasToColor()
               {
                  // Java: if (isStrict && !this.toColor().equals(dst.toColor()))
                        if isStrict && !self.toColor().equals(dst.toColor())
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: if (!isStrict && !this.toColor().equalsLax(dst.toColor()))
                        if !isStrict && !self.toColor().equalsLax(dst.toColor())
                  {
                     // Java: return false;
                     return false;
                  }
               }
               // Java: if ((this.hasToIndex() || !dst.hasToIndex()) && (!this.hasToIndex() || dst.hasToIndex()))
                    if (self.hasToIndex() || !dst.hasToIndex()) && (!self.hasToIndex() || dst.hasToIndex())
               {
                  // Java: if (this.hasToIndex() && dst.hasToIndex() && this.toIndex() != dst.toIndex())
                        if self.hasToIndex() && dst.hasToIndex() && self.toIndex() != dst.toIndex()
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasLbStyle() || !dst.hasLbStyle()) && (!this.hasLbStyle() || dst.hasLbStyle()))
                        else if (self.hasLbStyle() || !dst.hasLbStyle()) && (!self.hasLbStyle() || dst.hasLbStyle())
                  {
                     // Java: if (this.hasLbStyle() && dst.hasLbStyle() && this.lbStyle() != dst.lbStyle())
                            if self.hasLbStyle() && dst.hasLbStyle() && self.lbStyle() != dst.lbStyle()
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: else if ((this.hasLbColor() || !dst.hasLbColor()) && (!this.hasLbColor() || dst.hasLbColor()))
                            else if (self.hasLbColor() || !dst.hasLbColor()) && (!self.hasLbColor() || dst.hasLbColor())
                     {
                        // Java: if (this.hasLbColor() && dst.hasLbColor())
                                if self.hasLbColor() && dst.hasLbColor()
                        {
                           // Java: if (isStrict && !this.lbColor().equals(dst.lbColor()))
                                    if isStrict && !self.lbColor().equals(dst.lbColor())
                           {
                              // Java: return false;
                              return false;
                           }
                           // Java: if (!isStrict && !this.lbColor().equalsLax(dst.lbColor()))
                                    if !isStrict && !self.lbColor().equalsLax(dst.lbColor())
                           {
                              // Java: return false;
                              return false;
                           }
                        }
                        // Java: if ((this.hasLbHatch() || !dst.hasLbHatch()) && (!this.hasLbHatch() || dst.hasLbHatch()))
                                if (self.hasLbHatch() || !dst.hasLbHatch()) && (!self.hasLbHatch() || dst.hasLbHatch())
                        {
                           // Java: if (this.hasLbHatch() && dst.hasLbHatch() && this.lbHatch() != dst.lbHatch())
                                    if self.hasLbHatch() && dst.hasLbHatch() && self.lbHatch() != dst.lbHatch()
                           {
                              // Java: return false;
                              return false;
                           }
                           // Java: else if ((this.hasFgColor() || !dst.hasFgColor()) && (!this.hasFgColor() || dst.hasFgColor()))
                                    else if (self.hasFgColor() || !dst.hasFgColor()) && (!self.hasFgColor() || dst.hasFgColor())
                           {
                              // Java: if (this.hasFgColor() && dst.hasFgColor())
                                        if self.hasFgColor() && dst.hasFgColor()
                              {
                                 // Java: if (isStrict && !this.fgColor().equals(dst.fgColor()))
                                            if isStrict && !self.fgColor().equals(dst.fgColor())
                                 {
                                    // Java: return false;
                                    return false;
                                 }
                                 // Java: if (!isStrict && !this.fgColor().equalsLax(dst.fgColor()))
                                            if !isStrict && !self.fgColor().equalsLax(dst.fgColor())
                                 {
                                    // Java: return false;
                                    return false;
                                 }
                              }
                              // Java: if ((this.hasBgColor() || !dst.hasBgColor()) && (!this.hasBgColor() || dst.hasBgColor()))
                                        if (self.hasBgColor() || !dst.hasBgColor()) && (!self.hasBgColor() || dst.hasBgColor())
                              {
                                 // Java: if (this.hasBgColor() && dst.hasBgColor())
                                            if self.hasBgColor() && dst.hasBgColor()
                                 {
                                    // Java: if (isStrict && !this.bgColor().equals(dst.bgColor()))
                                                if isStrict && !self.bgColor().equals(dst.bgColor())
                                    {
                                       // Java: return false;
                                       return false;
                                    }
                                    // Java: if (!isStrict && !this.bgColor().equalsLax(dst.bgColor()))
                                                if !isStrict && !self.bgColor().equalsLax(dst.bgColor())
                                    {
                                       // Java: return false;
                                       return false;
                                    }
                                 }
                                 // Java: if ((this.hasBitmapType() || !dst.hasBitmapType()) && (!this.hasBitmapType() || dst.hasBitmapType()))
                                            if (self.hasBitmapType() || !dst.hasBitmapType()) && (!self.hasBitmapType() || dst.hasBitmapType())
                                 {
                                    // Java: if (this.hasBitmapType() && dst.hasBitmapType() && this.bitmapType() != dst.bitmapType())
                                                if self.hasBitmapType() && dst.hasBitmapType() && self.bitmapType() != dst.bitmapType()
                                    {
                                       // Java: return false;
                                       return false;
                                    }
                                    // Java: else if ((this.hasPattern() || !dst.hasPattern()) && (!this.hasPattern() || dst.hasPattern()))
                                                else if (self.hasPattern() || !dst.hasPattern()) && (!self.hasPattern() || dst.hasPattern())
                                    {
                                       // Java: if (this.hasPattern() && dst.hasPattern() && this.pattern() != dst.pattern())
                                                    if self.hasPattern() && dst.hasPattern() && self.pattern() != dst.pattern()
                                       {
                                          // Java: return false;
                                          return false;
                                       }
                                       // Java: else if ((this.hasPattern2() || !dst.hasPattern2()) && (!this.hasPattern2() || dst.hasPattern2()))
                                                    else if (self.hasPattern2() || !dst.hasPattern2()) && (!self.hasPattern2() || dst.hasPattern2())
                                       {
                                          // Java: return !this.hasPattern2() || !dst.hasPattern2() || this.pattern2() == dst.pattern2();
                                          return !self.hasPattern2() || !dst.hasPattern2() || self.pattern2() == dst.pattern2();
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
      // Java: if (this.hasFDoRecolor())
        if self.hasFDoRecolor()
      {
         // Java: ret += Boolean.valueOf(this.isFDoRecolor()).hashCode();
         ret = ret.wrapping_add(if self.isFDoRecolor() { 1231 } else { 1237 });
      }
      // Java: if (this.hasToColor())
        if self.hasToColor()
      {
         // Java: ret += this.toColor().hashCode();
         ret = ret.wrapping_add(todo!("Color hashCode"));
      }
      // Java: if (this.hasToIndex())
        if self.hasToIndex()
      {
         // Java: ret += Integer.valueOf(this.toIndex()).hashCode();
         ret = ret.wrapping_add(self.toIndex());
      }
      // Java: if (this.hasLbStyle())
        if self.hasLbStyle()
      {
         // Java: ret += Integer.valueOf(this.lbStyle()).hashCode();
         ret = ret.wrapping_add(self.lbStyle());
      }
      // Java: if (this.hasLbColor())
        if self.hasLbColor()
      {
         // Java: ret += this.lbColor().hashCode();
         ret = ret.wrapping_add(todo!("Color hashCode"));
      }
      // Java: if (this.hasLbHatch())
        if self.hasLbHatch()
      {
         // Java: ret += Integer.valueOf(this.lbHatch()).hashCode();
         ret = ret.wrapping_add(self.lbHatch());
      }
      // Java: if (this.hasFgColor())
        if self.hasFgColor()
      {
         // Java: ret += this.fgColor().hashCode();
         ret = ret.wrapping_add(todo!("Color hashCode"));
      }
      // Java: if (this.hasBgColor())
        if self.hasBgColor()
      {
         // Java: ret += this.bgColor().hashCode();
         ret = ret.wrapping_add(todo!("Color hashCode"));
      }
      // Java: if (this.hasBitmapType())
        if self.hasBitmapType()
      {
         // Java: ret += Integer.valueOf(this.bitmapType()).hashCode();
         ret = ret.wrapping_add(self.bitmapType());
      }
      // Java: if (this.hasPattern())
        if self.hasPattern()
      {
         // Java: ret += Integer.valueOf(this.pattern()).hashCode();
         ret = ret.wrapping_add(self.pattern());
      }
      // Java: if (this.hasPattern2())
        if self.hasPattern2()
      {
         // Java: ret += Integer.valueOf(this.pattern2()).hashCode();
         ret = ret.wrapping_add(self.pattern2());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.toColor != null && this.toColor.dirty())
        if self.toColor.as_ref().map_or(false, |x| x.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.lbColor != null && this.lbColor.dirty())
        else if self.lbColor != null && self.lbColor.dirty()
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.fgColor != null && this.fgColor.dirty())
        else if self.fgColor != null && self.fgColor.dirty()
      {
         // Java: return true;
         return true;
      }
      // Java: else
        else
      {
         // Java: return this.bgColor != null && this.bgColor.dirty() ? true : this.map.dirty();
         return self.bgColor.as_ref().map_or(false, |x| x.dirty()) || self.map.dirty();
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
      // Java: this.toColor = null;
      self.toColor = None;
      // Java: this.lbColor = null;
      self.lbColor = None;
      // Java: this.fgColor = null;
      self.fgColor = None;
      // Java: this.bgColor = null;
      self.bgColor = None;
    }

    // Java: void follow(RecolorEntryBlush parent)
    pub fn follow(&mut self, parent: Option<Box<RecolorEntryBlush>>) {
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

    // Java: RecolorEntryBlush parent()
    pub fn get_parent(&self) -> Option<&RecolorEntryBlush> {
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

    // Java: boolean isFDoRecolor()
    pub fn isFDoRecolor(&self) -> bool {
      // Java: if (!this.map.has(1))
        if !self.map.has(1)
      {
         // Java: return this.parent == null ? false : this.parent.isFDoRecolor();
         return self.parent.as_ref().map_or(false, |p| p.isFDoRecolor());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(1);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasFDoRecolor()
    pub fn hasFDoRecolor(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasFDoRecolor();
      self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasFDoRecolor())
    }

    // Java: void deleteFDoRecolor()
    pub fn deleteFDoRecolor(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
    }

    // Java: void putFDoRecolor(boolean value)
    pub fn putFDoRecolor(&mut self, value: bool) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(1, Encode::from_int(value, self.zone));
    }

    // Java: Color toColor()
    pub fn get_toColor(&self) -> &dyn KoAr {
      // Java: if (this.toColor == null)
        if self.toColor.is_none()
      {
         // Java: if (this.map.has(2))
            if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: this.toColor = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
         // Java: else if (this.parent != null && this.parent.hasToColor())
            else if self.parent != null && self.parent.hasToColor()
         {
            // Java: this.toColor = this.parent.toColor();
            todo!("lazy init requires &mut self")
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.toColor = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
      }
      // Java: return this.toColor;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasToColor()
    pub fn hasToColor(&self) -> bool {
      // Java: return this.map.has(2) || this.toColor != null && this.toColor.dirty() || this.parent != null && this.parent.hasToColor();
      self.map.has(2) || self.toColor.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasToColor())
    }

    // Java: void deleteToColor()
    pub fn deleteToColor(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.toColor != null)
        if self.toColor.is_some()
      {
         // Java: this.toColor = null;
         self.toColor = None;
      }
    }

    // Java: void writeToColor(Color value)
    pub fn writeToColor(&mut self, value: Option<&dyn KoAr>) {
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
            // Java: if (this.toColor != null)
                if self.toColor.is_some()
            {
               // Java: this.toColor = null;
               self.toColor = None;
            }
         }
      }
    }

    // Java: void forceWriteToColor(Color value)
    pub fn forceWriteToColor(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.toColor != null)
            if self.toColor.is_some()
         {
            // Java: this.toColor = null;
            self.toColor = None;
         }
      }
    }

    // Java: int toIndex()
    pub fn toIndex(&self) -> i32 {
      // Java: if (!this.map.has(3))
        if !self.map.has(3)
      {
         // Java: return this.parent == null ? 0 : this.parent.toIndex();
         return self.parent.as_ref().map_or(0, |p| p.toIndex());
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

    // Java: boolean hasToIndex()
    pub fn hasToIndex(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasToIndex();
      self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasToIndex())
    }

    // Java: void deleteToIndex()
    pub fn deleteToIndex(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
    }

    // Java: void putToIndex(int value)
    pub fn putToIndex(&mut self, value: i32) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(3, Encode::from_int(value, self.zone));
    }

    // Java: int lbStyle()
    pub fn lbStyle(&self) -> i32 {
      // Java: if (!this.map.has(4))
        if !self.map.has(4)
      {
         // Java: return this.parent == null ? 0 : this.parent.lbStyle();
         return self.parent.as_ref().map_or(0, |p| p.lbStyle());
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

    // Java: boolean hasLbStyle()
    pub fn hasLbStyle(&self) -> bool {
      // Java: return this.map.has(4) || this.parent != null && this.parent.hasLbStyle();
      self.map.has(4) || self.parent.as_ref().map_or(false, |p| p.hasLbStyle())
    }

    // Java: void deleteLbStyle()
    pub fn deleteLbStyle(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
    }

    // Java: void putLbStyle(int value)
    pub fn putLbStyle(&mut self, value: i32) {
      // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
      self.map.put(4, Encode::from_int(value, self.zone));
    }

    // Java: Color lbColor()
    pub fn get_lbColor(&self) -> &dyn KoAr {
      // Java: if (this.lbColor == null)
        if self.lbColor.is_none()
      {
         // Java: if (this.map.has(5))
            if self.map.has(5)
         {
            // Java: KoBin bin = this.map.byKey(5);
            let bin = self.map.byKey(5);
            // Java: this.lbColor = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
         // Java: else if (this.parent != null && this.parent.hasLbColor())
            else if self.parent != null && self.parent.hasLbColor()
         {
            // Java: this.lbColor = this.parent.lbColor();
            todo!("lazy init requires &mut self")
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.lbColor = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
      }
      // Java: return this.lbColor;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasLbColor()
    pub fn hasLbColor(&self) -> bool {
      // Java: return this.map.has(5) || this.lbColor != null && this.lbColor.dirty() || this.parent != null && this.parent.hasLbColor();
      self.map.has(5) || self.lbColor.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasLbColor())
    }

    // Java: void deleteLbColor()
    pub fn deleteLbColor(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(5);
      // Java: if (this.lbColor != null)
        if self.lbColor.is_some()
      {
         // Java: this.lbColor = null;
         self.lbColor = None;
      }
    }

    // Java: void writeLbColor(Color value)
    pub fn writeLbColor(&mut self, value: Option<&dyn KoAr>) {
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
            // Java: if (this.lbColor != null)
                if self.lbColor.is_some()
            {
               // Java: this.lbColor = null;
               self.lbColor = None;
            }
         }
      }
    }

    // Java: void forceWriteLbColor(Color value)
    pub fn forceWriteLbColor(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(5, value.lastBin(this.zone));
         self.map.put(5, v.lastBin(self.zone));
         // Java: if (this.lbColor != null)
            if self.lbColor.is_some()
         {
            // Java: this.lbColor = null;
            self.lbColor = None;
         }
      }
    }

    // Java: int lbHatch()
    pub fn lbHatch(&self) -> i32 {
      // Java: if (!this.map.has(6))
        if !self.map.has(6)
      {
         // Java: return this.parent == null ? 0 : this.parent.lbHatch();
         return self.parent.as_ref().map_or(0, |p| p.lbHatch());
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

    // Java: boolean hasLbHatch()
    pub fn hasLbHatch(&self) -> bool {
      // Java: return this.map.has(6) || this.parent != null && this.parent.hasLbHatch();
      self.map.has(6) || self.parent.as_ref().map_or(false, |p| p.hasLbHatch())
    }

    // Java: void deleteLbHatch()
    pub fn deleteLbHatch(&mut self) {
      // Java: this.map.put(6, (KoBin)null);
      self.map.clear_key(6);
    }

    // Java: void putLbHatch(int value)
    pub fn putLbHatch(&mut self, value: i32) {
      // Java: this.map.put(6, KoBin.Encode.from(value, this.zone));
      self.map.put(6, Encode::from_int(value, self.zone));
    }

    // Java: Color fgColor()
    pub fn get_fgColor(&self) -> &dyn KoAr {
      // Java: if (this.fgColor == null)
        if self.fgColor.is_none()
      {
         // Java: if (this.map.has(7))
            if self.map.has(7)
         {
            // Java: KoBin bin = this.map.byKey(7);
            let bin = self.map.byKey(7);
            // Java: this.fgColor = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
         // Java: else if (this.parent != null && this.parent.hasFgColor())
            else if self.parent != null && self.parent.hasFgColor()
         {
            // Java: this.fgColor = this.parent.fgColor();
            todo!("lazy init requires &mut self")
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.fgColor = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
      }
      // Java: return this.fgColor;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasFgColor()
    pub fn hasFgColor(&self) -> bool {
      // Java: return this.map.has(7) || this.fgColor != null && this.fgColor.dirty() || this.parent != null && this.parent.hasFgColor();
      self.map.has(7) || self.fgColor.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasFgColor())
    }

    // Java: void deleteFgColor()
    pub fn deleteFgColor(&mut self) {
      // Java: this.map.put(7, (KoBin)null);
      self.map.clear_key(7);
      // Java: if (this.fgColor != null)
        if self.fgColor.is_some()
      {
         // Java: this.fgColor = null;
         self.fgColor = None;
      }
    }

    // Java: void writeFgColor(Color value)
    pub fn writeFgColor(&mut self, value: Option<&dyn KoAr>) {
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
            // Java: if (this.fgColor != null)
                if self.fgColor.is_some()
            {
               // Java: this.fgColor = null;
               self.fgColor = None;
            }
         }
      }
    }

    // Java: void forceWriteFgColor(Color value)
    pub fn forceWriteFgColor(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(7, value.lastBin(this.zone));
         self.map.put(7, v.lastBin(self.zone));
         // Java: if (this.fgColor != null)
            if self.fgColor.is_some()
         {
            // Java: this.fgColor = null;
            self.fgColor = None;
         }
      }
    }

    // Java: Color bgColor()
    pub fn get_bgColor(&self) -> &dyn KoAr {
      // Java: if (this.bgColor == null)
        if self.bgColor.is_none()
      {
         // Java: if (this.map.has(8))
            if self.map.has(8)
         {
            // Java: KoBin bin = this.map.byKey(8);
            let bin = self.map.byKey(8);
            // Java: this.bgColor = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
         // Java: else if (this.parent != null && this.parent.hasBgColor())
            else if self.parent != null && self.parent.hasBgColor()
         {
            // Java: this.bgColor = this.parent.bgColor();
            todo!("lazy init requires &mut self")
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let _bin = Null::value1();
            // Java: this.bgColor = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            todo!("lazy init requires &mut self")
         }
      }
      // Java: return this.bgColor;
      todo!("lazy init requires &mut self")
    }

    // Java: boolean hasBgColor()
    pub fn hasBgColor(&self) -> bool {
      // Java: return this.map.has(8) || this.bgColor != null && this.bgColor.dirty() || this.parent != null && this.parent.hasBgColor();
      self.map.has(8) || self.bgColor.as_ref().map_or(false, |x| x.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasBgColor())
    }

    // Java: void deleteBgColor()
    pub fn deleteBgColor(&mut self) {
      // Java: this.map.put(8, (KoBin)null);
      self.map.clear_key(8);
      // Java: if (this.bgColor != null)
        if self.bgColor.is_some()
      {
         // Java: this.bgColor = null;
         self.bgColor = None;
      }
    }

    // Java: void writeBgColor(Color value)
    pub fn writeBgColor(&mut self, value: Option<&dyn KoAr>) {
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
            // Java: if (this.bgColor != null)
                if self.bgColor.is_some()
            {
               // Java: this.bgColor = null;
               self.bgColor = None;
            }
         }
      }
    }

    // Java: void forceWriteBgColor(Color value)
    pub fn forceWriteBgColor(&mut self, value: Option<&dyn KoAr>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(8, value.lastBin(this.zone));
         self.map.put(8, v.lastBin(self.zone));
         // Java: if (this.bgColor != null)
            if self.bgColor.is_some()
         {
            // Java: this.bgColor = null;
            self.bgColor = None;
         }
      }
    }

    // Java: int bitmapType()
    pub fn bitmapType(&self) -> i32 {
      // Java: if (!this.map.has(9))
        if !self.map.has(9)
      {
         // Java: return this.parent == null ? 0 : this.parent.bitmapType();
         return self.parent.as_ref().map_or(0, |p| p.bitmapType());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(9);
         let bin = self.map.byKey(9);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasBitmapType()
    pub fn hasBitmapType(&self) -> bool {
      // Java: return this.map.has(9) || this.parent != null && this.parent.hasBitmapType();
      self.map.has(9) || self.parent.as_ref().map_or(false, |p| p.hasBitmapType())
    }

    // Java: void deleteBitmapType()
    pub fn deleteBitmapType(&mut self) {
      // Java: this.map.put(9, (KoBin)null);
      self.map.clear_key(9);
    }

    // Java: void putBitmapType(int value)
    pub fn putBitmapType(&mut self, value: i32) {
      // Java: this.map.put(9, KoBin.Encode.from(value, this.zone));
      self.map.put(9, Encode::from_int(value, self.zone));
    }

    // Java: int pattern()
    pub fn pattern(&self) -> i32 {
      // Java: if (!this.map.has(10))
        if !self.map.has(10)
      {
         // Java: return this.parent == null ? 0 : this.parent.pattern();
         return self.parent.as_ref().map_or(0, |p| p.pattern());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(10);
         let bin = self.map.byKey(10);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasPattern()
    pub fn hasPattern(&self) -> bool {
      // Java: return this.map.has(10) || this.parent != null && this.parent.hasPattern();
      self.map.has(10) || self.parent.as_ref().map_or(false, |p| p.hasPattern())
    }

    // Java: void deletePattern()
    pub fn deletePattern(&mut self) {
      // Java: this.map.put(10, (KoBin)null);
      self.map.clear_key(10);
    }

    // Java: void putPattern(int value)
    pub fn putPattern(&mut self, value: i32) {
      // Java: this.map.put(10, KoBin.Encode.from(value, this.zone));
      self.map.put(10, Encode::from_int(value, self.zone));
    }

    // Java: int pattern2()
    pub fn pattern2(&self) -> i32 {
      // Java: if (!this.map.has(11))
        if !self.map.has(11)
      {
         // Java: return this.parent == null ? 0 : this.parent.pattern2();
         return self.parent.as_ref().map_or(0, |p| p.pattern2());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(11);
         let bin = self.map.byKey(11);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasPattern2()
    pub fn hasPattern2(&self) -> bool {
      // Java: return this.map.has(11) || this.parent != null && this.parent.hasPattern2();
      self.map.has(11) || self.parent.as_ref().map_or(false, |p| p.hasPattern2())
    }

    // Java: void deletePattern2()
    pub fn deletePattern2(&mut self) {
      // Java: this.map.put(11, (KoBin)null);
      self.map.clear_key(11);
    }

    // Java: void putPattern2(int value)
    pub fn putPattern2(&mut self, value: i32) {
      // Java: this.map.put(11, KoBin.Encode.from(value, this.zone));
      self.map.put(11, Encode::from_int(value, self.zone));
    }

}

impl KoAr for RecolorEntryBlush {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { RecolorEntryBlush::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { RecolorEntryBlush::lastBin(self, var1) }
}

impl KoClosable for RecolorEntryBlush {
    fn close(&mut self) { RecolorEntryBlush::close(self); }
}
