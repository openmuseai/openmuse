// Java: import cn.wps.drawing.data.effect.EffectProperties
use crate::cn::wps::drawing::data::effect::EffectProperties::EffectProperties;
// Java: import cn.wps.drawing.data.fill.Fill
use crate::cn::wps::drawing::data::fill::Fill::Fill;
// Java: import cn.wps.kso.kproto.KoAr
use crate::cn::wps::kso::kproto::KoAr::KoAr;
// Java: import cn.wps.kso.kproto.KoBin
use crate::cn::wps::kso::kproto::KoBin::KoBin;
// Java: import cn.wps.kso.kproto.KoClosable
use crate::cn::wps::kso::kproto::KoClosable::KoClosable;
// Java: import cn.wps.kso.kproto.KoZone
use crate::cn::wps::kso::kproto::KoZone::KoZone;
// Java: import cn.wps.kso.kproto.internal.KpBinBox
use crate::cn::wps::kso::kproto::internal::KpBinBox::KpBinBox;
// Java: import cn.wps.kso.kproto.internal.KpMap
use crate::cn::wps::kso::kproto::internal::KpMap::KpMap;

use super::TableStyle::TableStyle;
use crate::cn::wps::kso::kproto::KoBin::{KoBinTrait, Null, Factory, Encode, Decode};

// Java: public class Table implements KoAr, KoClosable
pub struct Table {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Fill fillProperties
    pub(crate) fillProperties: Option<Box<Fill>>,
    // Java field: EffectProperties effectProperties
    pub(crate) effectProperties: Option<Box<EffectProperties>>,
    // Java field: TableStyle tableStyle
    pub(crate) tableStyle: Option<Box<TableStyle>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: Table parent
    pub(crate) parent: Option<Box<Table>>,
}

// Java: public class Table implements KoAr, KoClosable
impl Table {
    // Java: static protected static <Value1 extends Table> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      // Java: inst.zone = zone;
      // Java: inst.load(bin);
      let mut inst = Self::new_from(bin);
      inst.zone = zone;
      // Java: return inst;
      inst
    }

    // Java: static Table create()
    pub fn create() -> Self {
      // Java: return initialize(Table.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static Table open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(Table.class, bin);
      Self::initialize(bin)
    }

    // Java: Table()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Self {
        zone: KoZone::defaultZone(),
        fillProperties: None,
        effectProperties: None,
        tableStyle: None,
        bin_box: KpBinBox::new(),
        map: KpMap::new(),
        parent: None,
      }
    }

    // Java: Table(KoBin bin)
    pub fn new_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Self {
        zone: KoZone::defaultZone(),
        fillProperties: None,
        effectProperties: None,
        tableStyle: None,
        bin_box: KpBinBox::KpBinBox_from(Some(bin)),
        map: KpMap::KpMap_from(bin),
        parent: None,
      }
    }
    pub fn Table_from(bin: &dyn KoBinTrait) -> Self { Self::new_from(bin) }

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
      let zone = self.zone; self.loadBin(bin, zone);
    }

    // Java: void load(Table value)
    pub fn load_from(&mut self, value: Option<&Table>) {
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
      // Java: if (this.fillProperties != null && this.fillProperties.dirty())
        if self.fillProperties.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(8, this.fillProperties.saveBin(zone));
         let bin = self.fillProperties.as_mut().unwrap().saveBin(zone);
         self.map.put(8, bin);
      }
      // Java: if (this.effectProperties != null && this.effectProperties.dirty())
        if self.effectProperties.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(9, this.effectProperties.saveBin(zone));
         let bin = self.effectProperties.as_mut().unwrap().saveBin(zone);
         self.map.put(9, bin);
      }
      // Java: if (this.tableStyle != null && this.tableStyle.dirty())
        if self.tableStyle.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(11, this.tableStyle.saveBin(zone));
         let bin = self.tableStyle.as_mut().unwrap().saveBin(zone);
         self.map.put(11, bin);
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(Table o)
    pub fn equalsLax(&self, o: &Table) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Table) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &Table, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof Table)
      {
         // Java: Table dst = (Table)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasBandCol() || !dst.hasBandCol()) && (!this.hasBandCol() || dst.hasBandCol()))
            else if (self.hasBandCol() || !dst.hasBandCol()) && (!self.hasBandCol() || dst.hasBandCol())
         {
            // Java: if (this.hasBandCol() && dst.hasBandCol() && this.isBandCol() != dst.isBandCol())
                if self.hasBandCol() && dst.hasBandCol() && self.isBandCol() != dst.isBandCol()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasBandRow() || !dst.hasBandRow()) && (!this.hasBandRow() || dst.hasBandRow()))
                else if (self.hasBandRow() || !dst.hasBandRow()) && (!self.hasBandRow() || dst.hasBandRow())
            {
               // Java: if (this.hasBandRow() && dst.hasBandRow() && this.isBandRow() != dst.isBandRow())
                    if self.hasBandRow() && dst.hasBandRow() && self.isBandRow() != dst.isBandRow()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasFirstCol() || !dst.hasFirstCol()) && (!this.hasFirstCol() || dst.hasFirstCol()))
                    else if (self.hasFirstCol() || !dst.hasFirstCol()) && (!self.hasFirstCol() || dst.hasFirstCol())
               {
                  // Java: if (this.hasFirstCol() && dst.hasFirstCol() && this.isFirstCol() != dst.isFirstCol())
                        if self.hasFirstCol() && dst.hasFirstCol() && self.isFirstCol() != dst.isFirstCol()
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasFirstRow() || !dst.hasFirstRow()) && (!this.hasFirstRow() || dst.hasFirstRow()))
                        else if (self.hasFirstRow() || !dst.hasFirstRow()) && (!self.hasFirstRow() || dst.hasFirstRow())
                  {
                     // Java: if (this.hasFirstRow() && dst.hasFirstRow() && this.isFirstRow() != dst.isFirstRow())
                            if self.hasFirstRow() && dst.hasFirstRow() && self.isFirstRow() != dst.isFirstRow()
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: else if ((this.hasLastCol() || !dst.hasLastCol()) && (!this.hasLastCol() || dst.hasLastCol()))
                            else if (self.hasLastCol() || !dst.hasLastCol()) && (!self.hasLastCol() || dst.hasLastCol())
                     {
                        // Java: if (this.hasLastCol() && dst.hasLastCol() && this.isLastCol() != dst.isLastCol())
                                if self.hasLastCol() && dst.hasLastCol() && self.isLastCol() != dst.isLastCol()
                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: else if ((this.hasLastRow() || !dst.hasLastRow()) && (!this.hasLastRow() || dst.hasLastRow()))
                                else if (self.hasLastRow() || !dst.hasLastRow()) && (!self.hasLastRow() || dst.hasLastRow())
                        {
                           // Java: if (this.hasLastRow() && dst.hasLastRow() && this.isLastRow() != dst.isLastRow())
                                    if self.hasLastRow() && dst.hasLastRow() && self.isLastRow() != dst.isLastRow()
                           {
                              // Java: return false;
                              return false;
                           }
                           // Java: else if ((this.hasRightToLeft() || !dst.hasRightToLeft()) && (!this.hasRightToLeft() || dst.hasRightToLeft()))
                                    else if (self.hasRightToLeft() || !dst.hasRightToLeft()) && (!self.hasRightToLeft() || dst.hasRightToLeft())
                           {
                              // Java: if (this.hasRightToLeft() && dst.hasRightToLeft() && this.isRightToLeft() != dst.isRightToLeft())
                                        if self.hasRightToLeft() && dst.hasRightToLeft() && self.isRightToLeft() != dst.isRightToLeft()
                              {
                                 // Java: return false;
                                 return false;
                              }
                              // Java: else if ((this.hasFillProperties() || !dst.hasFillProperties()) && (!this.hasFillProperties() || dst.hasFillProperties()))
                                        else if (self.hasFillProperties() || !dst.hasFillProperties()) && (!self.hasFillProperties() || dst.hasFillProperties())
                              {
                                 // Java: if (this.hasFillProperties() && dst.hasFillProperties())
                                            if self.hasFillProperties() && dst.hasFillProperties()
                                 {
                                    // Java: if (isStrict && !this.fillProperties().equals(dst.fillProperties()))
                                                if isStrict
                                    {
                                       // Java: return false;
                                       todo!();
                                    }
                                    // Java: if (!isStrict && !this.fillProperties().equalsLax(dst.fillProperties()))
                                                if !isStrict
                                    {
                                       // Java: return false;
                                       todo!();
                                    }
                                 }
                                 // Java: if ((this.hasEffectProperties() || !dst.hasEffectProperties()) && (!this.hasEffectProperties() || dst.hasEffectProperties()))
                                            if (self.hasEffectProperties() || !dst.hasEffectProperties()) && (!self.hasEffectProperties() || dst.hasEffectProperties())
                                 {
                                    // Java: if (this.hasEffectProperties() && dst.hasEffectProperties())
                                                if self.hasEffectProperties() && dst.hasEffectProperties()
                                    {
                                       // Java: if (isStrict && !this.effectProperties().equals(dst.effectProperties()))
                                                    if isStrict
                                       {
                                          // Java: return false;
                                          todo!();
                                       }
                                       // Java: if (!isStrict && !this.effectProperties().equalsLax(dst.effectProperties()))
                                                    if !isStrict
                                       {
                                          // Java: return false;
                                          todo!();
                                       }
                                    }
                                    // Java: if ((this.hasTableStyleId() || !dst.hasTableStyleId()) && (!this.hasTableStyleId() || dst.hasTableStyleId()))
                                                if (self.hasTableStyleId() || !dst.hasTableStyleId()) && (!self.hasTableStyleId() || dst.hasTableStyleId())
                                    {
                                       // Java: if (this.hasTableStyleId() && dst.hasTableStyleId() && !this.tableStyleId().equals(dst.tableStyleId()))
                                                    if self.hasTableStyleId() && dst.hasTableStyleId() && self.tableStyleId() != dst.tableStyleId()
                                       {
                                          // Java: return false;
                                          return false;
                                       }
                                       // Java: else if ((this.hasTableStyle() || !dst.hasTableStyle()) && (!this.hasTableStyle() || dst.hasTableStyle()))
                                                    else if (self.hasTableStyle() || !dst.hasTableStyle()) && (!self.hasTableStyle() || dst.hasTableStyle())
                                       {
                                          // Java: if (this.hasTableStyle() && dst.hasTableStyle())
                                                        if self.hasTableStyle() && dst.hasTableStyle()
                                          {
                                             // Java: if (isStrict && !this.tableStyle().equals(dst.tableStyle()))
                                                            if isStrict
                                             {
                                                // Java: return false;
                                                todo!();
                                             }
                                             // Java: if (!isStrict && !this.tableStyle().equalsLax(dst.tableStyle()))
                                                            if !isStrict
                                             {
                                                // Java: return false;
                                                todo!();
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
      // Java: return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasBandCol())
        if self.hasBandCol()
      {
         // Java: ret += Boolean.valueOf(this.isBandCol()).hashCode();
         ret += if self.isBandCol() { 1231 } else { 1237 };
      }
      // Java: if (this.hasBandRow())
        if self.hasBandRow()
      {
         // Java: ret += Boolean.valueOf(this.isBandRow()).hashCode();
         ret += if self.isBandRow() { 1231 } else { 1237 };
      }
      // Java: if (this.hasFirstCol())
        if self.hasFirstCol()
      {
         // Java: ret += Boolean.valueOf(this.isFirstCol()).hashCode();
         ret += if self.isFirstCol() { 1231 } else { 1237 };
      }
      // Java: if (this.hasFirstRow())
        if self.hasFirstRow()
      {
         // Java: ret += Boolean.valueOf(this.isFirstRow()).hashCode();
         ret += if self.isFirstRow() { 1231 } else { 1237 };
      }
      // Java: if (this.hasLastCol())
        if self.hasLastCol()
      {
         // Java: ret += Boolean.valueOf(this.isLastCol()).hashCode();
         ret += if self.isLastCol() { 1231 } else { 1237 };
      }
      // Java: if (this.hasLastRow())
        if self.hasLastRow()
      {
         // Java: ret += Boolean.valueOf(this.isLastRow()).hashCode();
         ret += if self.isLastRow() { 1231 } else { 1237 };
      }
      // Java: if (this.hasRightToLeft())
        if self.hasRightToLeft()
      {
         // Java: ret += Boolean.valueOf(this.isRightToLeft()).hashCode();
         ret += if self.isRightToLeft() { 1231 } else { 1237 };
      }
      // Java: if (this.hasFillProperties())
        if self.hasFillProperties()
      {
         // Java: ret += this.fillProperties().hashCode();
         todo!();
      }
      // Java: if (this.hasEffectProperties())
        if self.hasEffectProperties()
      {
         // Java: ret += this.effectProperties().hashCode();
         todo!();
      }
      // Java: if (this.hasTableStyleId())
        if self.hasTableStyleId()
      {
         // Java: ret += String.valueOf(this.tableStyleId()).hashCode();
         todo!();
      }
      // Java: if (this.hasTableStyle())
        if self.hasTableStyle()
      {
         // Java: ret += this.tableStyle().hashCode();
         todo!();
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.fillProperties != null && this.fillProperties.dirty())
        if self.fillProperties.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.effectProperties != null && this.effectProperties.dirty())
        else if self.effectProperties.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
      else
      {
         // Java: return this.tableStyle != null && this.tableStyle.dirty() ? true : this.map.dirty();
         return self.tableStyle.as_ref().map_or(false, |v| v.dirty()) || self.map.dirty();
      }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
      // Java: this.saveBin(this.zone);
      let zone = self.zone; self.saveBin(zone);
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
      let bin = self.bin_box.get(); self.loadBin(bin.as_ref(), zone);
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
      let null_bin = Null::value1(); self.map.load(null_bin.as_ref());
      // Java: this.box.clear();
      self.bin_box.set(None);
      // Java: this.zone.leave();
      // Java: this.zone = KoZone.defaultZone();
      self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    fn cleanCache(&mut self) {
      // Java: this.fillProperties = null;
      self.fillProperties = None;
      // Java: this.effectProperties = null;
      self.effectProperties = None;
      // Java: this.tableStyle = null;
      self.tableStyle = None;
    }

    // Java: void follow(Table parent)
    pub fn follow(&mut self, parent: Option<Box<Table>>) {
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

    // Java: Table parent()
    pub fn parent(&self) -> &Table {
      // Java: return this.parent;
      self.parent.as_deref().unwrap()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
      // Java: this.discard(this.zone);
      let zone = self.zone; self.discard_with_zone(zone);
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
      // Java: this.parent = null;
      self.parent = None;
    }

    // Java: boolean isBandCol()
    pub fn isBandCol(&self) -> bool {
      // Java: if (!this.map.has(1))
        if !self.map.has(1)
      {
         // Java: return this.parent == null ? false : this.parent.isBandCol();
         return self.parent.as_ref().map_or(false, |p| p.isBandCol());
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

    // Java: boolean hasBandCol()
    pub fn hasBandCol(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasBandCol();
      self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasBandCol())
    }

    // Java: void deleteBandCol()
    pub fn deleteBandCol(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
    }

    // Java: void putBandCol(boolean value)
    pub fn putBandCol(&mut self, value: bool) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(1, Encode::from_bool(value, self.zone));
    }

    // Java: boolean isBandRow()
    pub fn isBandRow(&self) -> bool {
      // Java: if (!this.map.has(2))
        if !self.map.has(2)
      {
         // Java: return this.parent == null ? false : this.parent.isBandRow();
         return self.parent.as_ref().map_or(false, |p| p.isBandRow());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(2);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasBandRow()
    pub fn hasBandRow(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasBandRow();
      self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasBandRow())
    }

    // Java: void deleteBandRow()
    pub fn deleteBandRow(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
    }

    // Java: void putBandRow(boolean value)
    pub fn putBandRow(&mut self, value: bool) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(2, Encode::from_bool(value, self.zone));
    }

    // Java: boolean isFirstCol()
    pub fn isFirstCol(&self) -> bool {
      // Java: if (!this.map.has(3))
        if !self.map.has(3)
      {
         // Java: return this.parent == null ? false : this.parent.isFirstCol();
         return self.parent.as_ref().map_or(false, |p| p.isFirstCol());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(3);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasFirstCol()
    pub fn hasFirstCol(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasFirstCol();
      self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasFirstCol())
    }

    // Java: void deleteFirstCol()
    pub fn deleteFirstCol(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
    }

    // Java: void putFirstCol(boolean value)
    pub fn putFirstCol(&mut self, value: bool) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(3, Encode::from_bool(value, self.zone));
    }

    // Java: boolean isFirstRow()
    pub fn isFirstRow(&self) -> bool {
      // Java: if (!this.map.has(4))
        if !self.map.has(4)
      {
         // Java: return this.parent == null ? false : this.parent.isFirstRow();
         return self.parent.as_ref().map_or(false, |p| p.isFirstRow());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(4);
         let bin = self.map.byKey(4);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasFirstRow()
    pub fn hasFirstRow(&self) -> bool {
      // Java: return this.map.has(4) || this.parent != null && this.parent.hasFirstRow();
      self.map.has(4) || self.parent.as_ref().map_or(false, |p| p.hasFirstRow())
    }

    // Java: void deleteFirstRow()
    pub fn deleteFirstRow(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
    }

    // Java: void putFirstRow(boolean value)
    pub fn putFirstRow(&mut self, value: bool) {
      // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
      self.map.put(4, Encode::from_bool(value, self.zone));
    }

    // Java: boolean isLastCol()
    pub fn isLastCol(&self) -> bool {
      // Java: if (!this.map.has(5))
        if !self.map.has(5)
      {
         // Java: return this.parent == null ? false : this.parent.isLastCol();
         return self.parent.as_ref().map_or(false, |p| p.isLastCol());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(5);
         let bin = self.map.byKey(5);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasLastCol()
    pub fn hasLastCol(&self) -> bool {
      // Java: return this.map.has(5) || this.parent != null && this.parent.hasLastCol();
      self.map.has(5) || self.parent.as_ref().map_or(false, |p| p.hasLastCol())
    }

    // Java: void deleteLastCol()
    pub fn deleteLastCol(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(5);
    }

    // Java: void putLastCol(boolean value)
    pub fn putLastCol(&mut self, value: bool) {
      // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
      self.map.put(5, Encode::from_bool(value, self.zone));
    }

    // Java: boolean isLastRow()
    pub fn isLastRow(&self) -> bool {
      // Java: if (!this.map.has(6))
        if !self.map.has(6)
      {
         // Java: return this.parent == null ? false : this.parent.isLastRow();
         return self.parent.as_ref().map_or(false, |p| p.isLastRow());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(6);
         let bin = self.map.byKey(6);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasLastRow()
    pub fn hasLastRow(&self) -> bool {
      // Java: return this.map.has(6) || this.parent != null && this.parent.hasLastRow();
      self.map.has(6) || self.parent.as_ref().map_or(false, |p| p.hasLastRow())
    }

    // Java: void deleteLastRow()
    pub fn deleteLastRow(&mut self) {
      // Java: this.map.put(6, (KoBin)null);
      self.map.clear_key(6);
    }

    // Java: void putLastRow(boolean value)
    pub fn putLastRow(&mut self, value: bool) {
      // Java: this.map.put(6, KoBin.Encode.from(value, this.zone));
      self.map.put(6, Encode::from_bool(value, self.zone));
    }

    // Java: boolean isRightToLeft()
    pub fn isRightToLeft(&self) -> bool {
      // Java: if (!this.map.has(7))
        if !self.map.has(7)
      {
         // Java: return this.parent == null ? false : this.parent.isRightToLeft();
         return self.parent.as_ref().map_or(false, |p| p.isRightToLeft());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(7);
         let bin = self.map.byKey(7);
         // Java: return KoBin.Decode.toBool(bin);
         return Decode::toBool(bin.as_ref());
      }
    }

    // Java: boolean hasRightToLeft()
    pub fn hasRightToLeft(&self) -> bool {
      // Java: return this.map.has(7) || this.parent != null && this.parent.hasRightToLeft();
      self.map.has(7) || self.parent.as_ref().map_or(false, |p| p.hasRightToLeft())
    }

    // Java: void deleteRightToLeft()
    pub fn deleteRightToLeft(&mut self) {
      // Java: this.map.put(7, (KoBin)null);
      self.map.clear_key(7);
    }

    // Java: void putRightToLeft(boolean value)
    pub fn putRightToLeft(&mut self, value: bool) {
      // Java: this.map.put(7, KoBin.Encode.from(value, this.zone));
      self.map.put(7, Encode::from_bool(value, self.zone));
    }

    // Java: Fill fillProperties()
    pub fn fillProperties(&mut self) -> &Fill {
      // Java: if (this.fillProperties == null)
        if self.fillProperties.is_none()
      {
         // Java: if (this.map.has(8))
            if self.map.has(8)
         {
            // Java: KoBin bin = this.map.byKey(8);
            let bin = self.map.byKey(8);
            // Java: this.fillProperties = (Fill)KoBin.Decode.toAny(Fill.class, bin, this.zone);
            self.fillProperties = Some(Box::new(Fill::Fill_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasFillProperties())
            else if self.parent.as_ref().map_or(false, |p| p.hasFillProperties())
         {
            // Java: this.fillProperties = this.parent.fillProperties();
            todo!();
         }
         // Java: else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.fillProperties = (Fill)KoBin.Decode.toAny(Fill.class, bin, this.zone);
            self.fillProperties = Some(Box::new(Fill::Fill_from(bin.as_ref())));
         }
      }
      // Java: return this.fillProperties;
      self.fillProperties.as_deref().unwrap()
    }

    // Java: boolean hasFillProperties()
    pub fn hasFillProperties(&self) -> bool {
      // Java: return this.map.has(8) || this.fillProperties != null && this.fillProperties.dirty() || this.parent != null && this.parent.hasFillProperties();
      self.map.has(8) || self.fillProperties.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasFillProperties())
    }

    // Java: void deleteFillProperties()
    pub fn deleteFillProperties(&mut self) {
      // Java: this.map.put(8, (KoBin)null);
      self.map.clear_key(8);
      // Java: if (this.fillProperties != null)
        if self.fillProperties.is_some()
      {
         // Java: this.fillProperties = null;
         self.fillProperties = None;
      }
    }

    // Java: void writeFillProperties(Fill value)
    pub fn writeFillProperties(&mut self, value: Option<&Fill>) {
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
            // Java: if (this.fillProperties != null)
                if self.fillProperties.is_some()
            {
               // Java: this.fillProperties = null;
               self.fillProperties = None;
            }
         }
      }
    }

    // Java: void forceWriteFillProperties(Fill value)
    pub fn forceWriteFillProperties(&mut self, value: Option<&Fill>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(8, value.lastBin(this.zone));
         self.map.put(8, v.lastBin(self.zone));
         // Java: if (this.fillProperties != null)
            if self.fillProperties.is_some()
         {
            // Java: this.fillProperties = null;
            self.fillProperties = None;
         }
      }
    }

    // Java: EffectProperties effectProperties()
    pub fn effectProperties(&mut self) -> &EffectProperties {
      // Java: if (this.effectProperties == null)
        if self.effectProperties.is_none()
      {
         // Java: if (this.map.has(9))
            if self.map.has(9)
         {
            // Java: KoBin bin = this.map.byKey(9);
            let bin = self.map.byKey(9);
            // Java: this.effectProperties = (EffectProperties)KoBin.Decode.toAny(EffectProperties.class, bin, this.zone);
            self.effectProperties = Some(Box::new(EffectProperties::EffectProperties_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasEffectProperties())
            else if self.parent.as_ref().map_or(false, |p| p.hasEffectProperties())
         {
            // Java: this.effectProperties = this.parent.effectProperties();
            todo!();
         }
         // Java: else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.effectProperties = (EffectProperties)KoBin.Decode.toAny(EffectProperties.class, bin, this.zone);
            self.effectProperties = Some(Box::new(EffectProperties::EffectProperties_from(bin.as_ref())));
         }
      }
      // Java: return this.effectProperties;
      self.effectProperties.as_deref().unwrap()
    }

    // Java: boolean hasEffectProperties()
    pub fn hasEffectProperties(&self) -> bool {
      // Java: return this.map.has(9) || this.effectProperties != null && this.effectProperties.dirty() || this.parent != null && this.parent.hasEffectProperties();
      self.map.has(9) || self.effectProperties.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasEffectProperties())
    }

    // Java: void deleteEffectProperties()
    pub fn deleteEffectProperties(&mut self) {
      // Java: this.map.put(9, (KoBin)null);
      self.map.clear_key(9);
      // Java: if (this.effectProperties != null)
        if self.effectProperties.is_some()
      {
         // Java: this.effectProperties = null;
         self.effectProperties = None;
      }
    }

    // Java: void writeEffectProperties(EffectProperties value)
    pub fn writeEffectProperties(&mut self, value: Option<&EffectProperties>) {
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
            // Java: if (this.effectProperties != null)
                if self.effectProperties.is_some()
            {
               // Java: this.effectProperties = null;
               self.effectProperties = None;
            }
         }
      }
    }

    // Java: void forceWriteEffectProperties(EffectProperties value)
    pub fn forceWriteEffectProperties(&mut self, value: Option<&EffectProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(9, value.lastBin(this.zone));
         self.map.put(9, v.lastBin(self.zone));
         // Java: if (this.effectProperties != null)
            if self.effectProperties.is_some()
         {
            // Java: this.effectProperties = null;
            self.effectProperties = None;
         }
      }
    }

    // Java: String tableStyleId()
    pub fn tableStyleId(&self) -> String {
      // Java: if (!this.map.has(10))
        if !self.map.has(10)
      {
         // Java: return this.parent == null ? "" : this.parent.tableStyleId();
         return self.parent.as_ref().map_or(String::new(), |p| p.tableStyleId());
      }
      // Java: else
      else
      {
         // Java: KoBin bin = this.map.byKey(10);
         let bin = self.map.byKey(10);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasTableStyleId()
    pub fn hasTableStyleId(&self) -> bool {
      // Java: return this.map.has(10) || this.parent != null && this.parent.hasTableStyleId();
      self.map.has(10) || self.parent.as_ref().map_or(false, |p| p.hasTableStyleId())
    }

    // Java: void deleteTableStyleId()
    pub fn deleteTableStyleId(&mut self) {
      // Java: this.map.put(10, (KoBin)null);
      self.map.clear_key(10);
    }

    // Java: void putTableStyleId(String value)
    pub fn putTableStyleId(&mut self, value: &str) {
      // Java: this.map.put(10, KoBin.Encode.from(value, this.zone));
      self.map.put(10, Encode::from_string(value, self.zone));
    }

    // Java: TableStyle tableStyle()
    pub fn tableStyle(&mut self) -> &TableStyle {
      // Java: if (this.tableStyle == null)
        if self.tableStyle.is_none()
      {
         // Java: if (this.map.has(11))
            if self.map.has(11)
         {
            // Java: KoBin bin = this.map.byKey(11);
            let bin = self.map.byKey(11);
            // Java: this.tableStyle = (TableStyle)KoBin.Decode.toAny(TableStyle.class, bin, this.zone);
            self.tableStyle = Some(Box::new(TableStyle::TableStyle_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasTableStyle())
            else if self.parent.as_ref().map_or(false, |p| p.hasTableStyle())
         {
            // Java: this.tableStyle = this.parent.tableStyle();
            todo!();
         }
         // Java: else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.tableStyle = (TableStyle)KoBin.Decode.toAny(TableStyle.class, bin, this.zone);
            self.tableStyle = Some(Box::new(TableStyle::TableStyle_from(bin.as_ref())));
         }
      }
      // Java: return this.tableStyle;
      self.tableStyle.as_deref().unwrap()
    }

    // Java: boolean hasTableStyle()
    pub fn hasTableStyle(&self) -> bool {
      // Java: return this.map.has(11) || this.tableStyle != null && this.tableStyle.dirty() || this.parent != null && this.parent.hasTableStyle();
      self.map.has(11) || self.tableStyle.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasTableStyle())
    }

    // Java: void deleteTableStyle()
    pub fn deleteTableStyle(&mut self) {
      // Java: this.map.put(11, (KoBin)null);
      self.map.clear_key(11);
      // Java: if (this.tableStyle != null)
        if self.tableStyle.is_some()
      {
         // Java: this.tableStyle = null;
         self.tableStyle = None;
      }
    }

    // Java: void writeTableStyle(TableStyle value)
    pub fn writeTableStyle(&mut self, value: Option<&TableStyle>) {
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
            // Java: if (this.tableStyle != null)
                if self.tableStyle.is_some()
            {
               // Java: this.tableStyle = null;
               self.tableStyle = None;
            }
         }
      }
    }

    // Java: void forceWriteTableStyle(TableStyle value)
    pub fn forceWriteTableStyle(&mut self, value: Option<&TableStyle>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(11, value.lastBin(this.zone));
         self.map.put(11, v.lastBin(self.zone));
         // Java: if (this.tableStyle != null)
            if self.tableStyle.is_some()
         {
            // Java: this.tableStyle = null;
            self.tableStyle = None;
         }
      }
    }

}

impl KoAr for Table {
}

impl KoClosable for Table {
}
