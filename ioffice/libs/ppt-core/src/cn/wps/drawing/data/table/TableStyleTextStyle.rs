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

use super::ThemeableFontStyles::ThemeableFontStyles;

// Java: public class TableStyleTextStyle implements KoAr, KoClosable
pub struct TableStyleTextStyle {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: ThemeableFontStyles fontStyles
    pub(crate) fontStyles: Option<Box<ThemeableFontStyles>>,
    // Java field: Color color
    pub(crate) color: Option<Box<Color>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: TableStyleTextStyle parent
    pub(crate) parent: Option<Box<TableStyleTextStyle>>,
}

// Java: public class TableStyleTextStyle implements KoAr, KoClosable
impl TableStyleTextStyle {
    // Java: static protected static <Value1 extends TableStyleTextStyle> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::TableStyleTextStyle_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static TableStyleTextStyle create()
    pub fn create() -> Self {
      // Java: return initialize(TableStyleTextStyle.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static TableStyleTextStyle open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(TableStyleTextStyle.class, bin);
      Self::initialize(bin)
    }

    // Java: TableStyleTextStyle()
    pub fn new() -> Self {
      Self {
      // Java: this.box = new KpBinBox();
      bin_box: KpBinBox::new(),
      // Java: this.map = new KpMap();
      map: KpMap::new(),
      zone: KoZone::defaultZone(), fontStyles: None, color: None, parent: None,
      }
    }

    // Java: TableStyleTextStyle(KoBin bin)
    pub fn TableStyleTextStyle_from(bin: &dyn KoBinTrait) -> Self {
      Self {
      // Java: this.box = new KpBinBox(bin);
      bin_box: KpBinBox::KpBinBox_from(Some(bin)),
      // Java: this.map = new KpMap(bin);
      map: KpMap::KpMap_from(bin),
      zone: KoZone::defaultZone(), fontStyles: None, color: None, parent: None,
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

    // Java: void load(TableStyleTextStyle value)
    pub fn load_value(&mut self, value: Option<&TableStyleTextStyle>) {
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
      // Java: if (this.fontStyles != null && this.fontStyles.dirty())
        if self.fontStyles.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(3, this.fontStyles.saveBin(zone));
         let bin = self.fontStyles.as_mut().unwrap().saveBin(zone);
         self.map.put(3, bin);
      }
      // Java: if (this.color != null && this.color.dirty())
        if self.color.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(4, this.color.saveBin(zone));
         let bin = self.color.as_mut().unwrap().saveBin(zone);
         self.map.put(4, bin);
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

    // Java: boolean equalsLax(TableStyleTextStyle o)
    pub fn equalsLax(&mut self, o: &mut TableStyleTextStyle) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&mut self, o: &mut TableStyleTextStyle) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&mut self, o: &mut TableStyleTextStyle, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof TableStyleTextStyle)
        if true
      {
         // Java: TableStyleTextStyle dst = (TableStyleTextStyle)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self as *const _, dst as *const _)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasBold() || !dst.hasBold()) && (!this.hasBold() || dst.hasBold()))
            else if (self.hasBold() || !dst.hasBold()) && (!self.hasBold() || dst.hasBold())
         {
            // Java: if (this.hasBold() && dst.hasBold() && this.bold() != dst.bold())
                if self.hasBold() && dst.hasBold() && self.bold() != dst.bold()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasItalic() || !dst.hasItalic()) && (!this.hasItalic() || dst.hasItalic()))
                else if (self.hasItalic() || !dst.hasItalic()) && (!self.hasItalic() || dst.hasItalic())
            {
               // Java: if (this.hasItalic() && dst.hasItalic() && this.italic() != dst.italic())
                    if self.hasItalic() && dst.hasItalic() && self.italic() != dst.italic()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasFontStyles() || !dst.hasFontStyles()) && (!this.hasFontStyles() || dst.hasFontStyles()))
                    else if (self.hasFontStyles() || !dst.hasFontStyles()) && (!self.hasFontStyles() || dst.hasFontStyles())
               {
                  // Java: if (this.hasFontStyles() && dst.hasFontStyles())
                        if self.hasFontStyles() && dst.hasFontStyles()
                  {
                     // Java: if (isStrict && !this.fontStyles().equals(dst.fontStyles()))
                            if isStrict && !self.get_fontStyles_mut().equals(dst.get_fontStyles_mut())
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: if (!isStrict && !this.fontStyles().equalsLax(dst.fontStyles()))
                            if !isStrict && !self.get_fontStyles_mut().equalsLax(dst.get_fontStyles_mut())
                     {
                        // Java: return false;
                        return false;
                     }
                  }
                  // Java: if ((this.hasColor() || !dst.hasColor()) && (!this.hasColor() || dst.hasColor()))
                        if (self.hasColor() || !dst.hasColor()) && (!self.hasColor() || dst.hasColor())
                  {
                     // Java: if (this.hasColor() && dst.hasColor())
                            if self.hasColor() && dst.hasColor()
                     {
                        // Java: if (isStrict && !this.color().equals(dst.color()))
                                if isStrict && !self.get_color().equals(dst.get_color())
                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: if (!isStrict && !this.color().equalsLax(dst.color()))
                                if !isStrict && !self.get_color().equalsLax(dst.get_color())
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

    // Java: int hashCode()
    pub fn hashCode(&mut self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasBold())
        if self.hasBold()
      {
         // Java: ret += Integer.valueOf(this.bold()).hashCode();
         ret = ret.wrapping_add(self.bold());
      }
      // Java: if (this.hasItalic())
        if self.hasItalic()
      {
         // Java: ret += Integer.valueOf(this.italic()).hashCode();
         ret = ret.wrapping_add(self.italic());
      }
      // Java: if (this.hasFontStyles())
        if self.hasFontStyles()
      {
         // Java: ret += this.fontStyles().hashCode();
         ret = ret.wrapping_add(self.get_fontStyles_mut().hashCode());
      }
      // Java: if (this.hasColor())
        if self.hasColor()
      {
         // Java: ret += this.color().hashCode();
         ret = ret.wrapping_add(self.get_color().hashCode());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.fontStyles != null && this.fontStyles.dirty())
        if self.fontStyles.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
        else
      {
         // Java: return this.color != null && this.color.dirty() ? true : this.map.dirty();
         if self.color.as_ref().map_or(false, |s| s.dirty()) { true } else { self.map.dirty() }
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
      // Java: this.fontStyles = null;
      self.fontStyles = None;
      // Java: this.color = null;
      self.color = None;
    }

    // Java: void follow(TableStyleTextStyle parent)
    pub fn follow(&mut self, parent: Option<Box<TableStyleTextStyle>>) {
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

    // Java: TableStyleTextStyle parent()
    pub fn get_parent(&self) -> Option<&TableStyleTextStyle> {
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

    // Java: int bold()
    pub fn bold(&self) -> i32 {
      // Java: if (!this.map.has(1))
        if !self.map.has(1)
      {
         // Java: return this.parent == null ? 0 : this.parent.bold();
         return self.parent.as_ref().map_or(0, |p| p.bold());
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

    // Java: boolean hasBold()
    pub fn hasBold(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasBold();
      self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasBold())
    }

    // Java: void deleteBold()
    pub fn deleteBold(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
    }

    // Java: void putBold(int value)
    pub fn putBold(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(1, Encode::from_int(value, self.zone));
    }

    // Java: int italic()
    pub fn italic(&self) -> i32 {
      // Java: if (!this.map.has(2))
        if !self.map.has(2)
      {
         // Java: return this.parent == null ? 0 : this.parent.italic();
         return self.parent.as_ref().map_or(0, |p| p.italic());
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

    // Java: boolean hasItalic()
    pub fn hasItalic(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasItalic();
      self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasItalic())
    }

    // Java: void deleteItalic()
    pub fn deleteItalic(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
    }

    // Java: void putItalic(int value)
    pub fn putItalic(&mut self, value: i32) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(2, Encode::from_int(value, self.zone));
    }

    pub fn get_fontStyles_mut(&mut self) -> &mut ThemeableFontStyles {
      if self.fontStyles.is_none() {
        if self.map.has(3) {
          todo!("lazy init fontStyles from map");
        } else if self.parent.as_ref().map_or(false, |p| p.hasFontStyles()) {
          todo!("copy fontStyles from parent");
        } else {
          todo!("default init fontStyles");
        }
      }
      self.fontStyles.as_deref_mut().unwrap()
    }

    // Java: ThemeableFontStyles fontStyles()
    pub fn get_fontStyles(&self) -> &ThemeableFontStyles {
      // Java: if (this.fontStyles == null)
        if self.fontStyles.is_none()
      {
         // Java: if (this.map.has(3))
            if self.map.has(3)
         {
            // Java: KoBin bin = this.map.byKey(3);
            // Java: this.fontStyles = (ThemeableFontStyles)KoBin.Decode.toAny(ThemeableFontStyles.class, bin, this.zone);
            todo!("lazy init fontStyles from map");
         }
         // Java: else if (this.parent != null && this.parent.hasFontStyles())
            else if self.parent.as_ref().map_or(false, |p| p.hasFontStyles())
         {
            // Java: this.fontStyles = this.parent.fontStyles();
            todo!("copy fontStyles from parent");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.fontStyles = (ThemeableFontStyles)KoBin.Decode.toAny(ThemeableFontStyles.class, bin, this.zone);
            todo!("default init fontStyles");
         }
      }
      // Java: return this.fontStyles;
      self.fontStyles.as_deref().unwrap()
    }

    // Java: boolean hasFontStyles()
    pub fn hasFontStyles(&self) -> bool {
      // Java: return this.map.has(3) || this.fontStyles != null && this.fontStyles.dirty() || this.parent != null && this.parent.hasFontStyles();
      self.map.has(3) || self.fontStyles.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasFontStyles())
    }

    // Java: void deleteFontStyles()
    pub fn deleteFontStyles(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
      // Java: if (this.fontStyles != null)
        if self.fontStyles.is_some()
      {
         // Java: this.fontStyles = null;
         self.fontStyles = None;
      }
    }

    // Java: void writeFontStyles(ThemeableFontStyles value)
    pub fn writeFontStyles(&mut self, value: Option<&ThemeableFontStyles>) {
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
            // Java: if (this.fontStyles != null)
                if self.fontStyles.is_some()
            {
               // Java: this.fontStyles = null;
               self.fontStyles = None;
            }
         }
      }
    }

    // Java: void forceWriteFontStyles(ThemeableFontStyles value)
    pub fn forceWriteFontStyles(&mut self, value: Option<&ThemeableFontStyles>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(3, value.lastBin(this.zone));
         self.map.put(3, v.lastBin(self.zone));
         // Java: if (this.fontStyles != null)
            if self.fontStyles.is_some()
         {
            // Java: this.fontStyles = null;
            self.fontStyles = None;
         }
      }
    }

    // Java: Color color()
    pub fn get_color(&self) -> &Color {
      // Java: if (this.color == null)
        if self.color.is_none()
      {
         // Java: if (this.map.has(4))
            if self.map.has(4)
         {
            // Java: KoBin bin = this.map.byKey(4);
            // Java: this.color = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            todo!("lazy init color from map");
         }
         // Java: else if (this.parent != null && this.parent.hasColor())
            else if self.parent.as_ref().map_or(false, |p| p.hasColor())
         {
            // Java: this.color = this.parent.color();
            todo!("copy color from parent");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.color = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
            todo!("default init color");
         }
      }
      // Java: return this.color;
      self.color.as_deref().unwrap()
    }

    // Java: boolean hasColor()
    pub fn hasColor(&self) -> bool {
      // Java: return this.map.has(4) || this.color != null && this.color.dirty() || this.parent != null && this.parent.hasColor();
      self.map.has(4) || self.color.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasColor())
    }

    // Java: void deleteColor()
    pub fn deleteColor(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
      // Java: if (this.color != null)
        if self.color.is_some()
      {
         // Java: this.color = null;
         self.color = None;
      }
    }

    // Java: void writeColor(Color value)
    pub fn writeColor(&mut self, value: Option<&Color>) {
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
            // Java: if (this.color != null)
                if self.color.is_some()
            {
               // Java: this.color = null;
               self.color = None;
            }
         }
      }
    }

    // Java: void forceWriteColor(Color value)
    pub fn forceWriteColor(&mut self, value: Option<&Color>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(4, value.lastBin(this.zone));
         self.map.put(4, v.lastBin(self.zone));
         // Java: if (this.color != null)
            if self.color.is_some()
         {
            // Java: this.color = null;
            self.color = None;
         }
      }
    }

}

impl KoAr for TableStyleTextStyle {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { TableStyleTextStyle::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { TableStyleTextStyle::lastBin(self, var1) }
}

impl KoClosable for TableStyleTextStyle {
    fn close(&mut self) { TableStyleTextStyle::close(self); }
}
