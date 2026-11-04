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

use crate::cn::wps::drawing::data::table::TableCellProperties::TableCellProperties;

fn java_string_hash(s: &str) -> i32 {
    let mut h: i32 = 0;
    for ch in s.chars() {
        h = h.wrapping_mul(31).wrapping_add(ch as i32);
    }
    h
}

// Java: public class TableCell implements KoAr, KoClosable
pub struct TableCell {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: TableCellProperties cellProperties
    pub(crate) cellProperties: Option<Box<TableCellProperties>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: TableCell parent
    pub(crate) parent: Option<Box<TableCell>>,
}

// Java: public class TableCell implements KoAr, KoClosable
impl TableCell {
    // Java: static protected static <Value1 extends TableCell> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::TableCell_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static TableCell create()
    pub fn create() -> Self {
      // Java: return initialize(TableCell.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static TableCell open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(TableCell.class, bin);
      Self::initialize(bin)
    }

    // Java: TableCell()
    pub fn new() -> Self {
      Self {
      // Java: this.box = new KpBinBox();
      bin_box: KpBinBox::new(),
      // Java: this.map = new KpMap();
      map: KpMap::new(),
      zone: KoZone::defaultZone(), cellProperties: None, parent: None,
      }
    }

    // Java: TableCell(KoBin bin)
    pub fn TableCell_from(bin: &dyn KoBinTrait) -> Self {
      Self {
      // Java: this.box = new KpBinBox(bin);
      bin_box: KpBinBox::KpBinBox_from(Some(bin)),
      // Java: this.map = new KpMap(bin);
      map: KpMap::KpMap_from(bin),
      zone: KoZone::defaultZone(), cellProperties: None, parent: None,
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

    // Java: void load(TableCell value)
    pub fn load_value(&mut self, value: Option<&TableCell>) {
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
      // Java: if (this.cellProperties != null && this.cellProperties.dirty())
        if self.cellProperties.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(6, this.cellProperties.saveBin(zone));
         let bin = self.cellProperties.as_mut().unwrap().saveBin(zone);
         self.map.put(6, bin);
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

    // Java: boolean equalsLax(TableCell o)
    pub fn equalsLax(&self, o: &TableCell) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TableCell) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &TableCell, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof TableCell)
        if true
      {
         // Java: TableCell dst = (TableCell)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasGridSpan() || !dst.hasGridSpan()) && (!this.hasGridSpan() || dst.hasGridSpan()))
            else if (self.hasGridSpan() || !dst.hasGridSpan()) && (!self.hasGridSpan() || dst.hasGridSpan())
         {
            // Java: if (this.hasGridSpan() && dst.hasGridSpan() && this.gridSpan() != dst.gridSpan())
                if self.hasGridSpan() && dst.hasGridSpan() && self.gridSpan() != dst.gridSpan()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasHorizontalMerge() || !dst.hasHorizontalMerge()) && (!this.hasHorizontalMerge() || dst.hasHorizontalMerge()))
                else if (self.hasHorizontalMerge() || !dst.hasHorizontalMerge()) && (!self.hasHorizontalMerge() || dst.hasHorizontalMerge())
            {
               // Java: if (this.hasHorizontalMerge() && dst.hasHorizontalMerge() && this.isHorizontalMerge() != dst.isHorizontalMerge())
                    if self.hasHorizontalMerge() && dst.hasHorizontalMerge() && self.isHorizontalMerge() != dst.isHorizontalMerge()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasTableCellIdentifier() || !dst.hasTableCellIdentifier()) && (!this.hasTableCellIdentifier() || dst.hasTableCellIdentifier()))
                    else if (self.hasTableCellIdentifier() || !dst.hasTableCellIdentifier()) && (!self.hasTableCellIdentifier() || dst.hasTableCellIdentifier())
               {
                  // Java: if (this.hasTableCellIdentifier() && dst.hasTableCellIdentifier() && !this.tableCellIdentifier().equals(dst.tableCellIdentifier()))
                        if self.hasTableCellIdentifier() && dst.hasTableCellIdentifier() && self.tableCellIdentifier() != dst.tableCellIdentifier()
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasRowSpan() || !dst.hasRowSpan()) && (!this.hasRowSpan() || dst.hasRowSpan()))
                        else if (self.hasRowSpan() || !dst.hasRowSpan()) && (!self.hasRowSpan() || dst.hasRowSpan())
                  {
                     // Java: if (this.hasRowSpan() && dst.hasRowSpan() && this.rowSpan() != dst.rowSpan())
                            if self.hasRowSpan() && dst.hasRowSpan() && self.rowSpan() != dst.rowSpan()
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: else if ((this.hasVerticalMerge() || !dst.hasVerticalMerge()) && (!this.hasVerticalMerge() || dst.hasVerticalMerge()))
                            else if (self.hasVerticalMerge() || !dst.hasVerticalMerge()) && (!self.hasVerticalMerge() || dst.hasVerticalMerge())
                     {
                        // Java: if (this.hasVerticalMerge() && dst.hasVerticalMerge() && this.isVerticalMerge() != dst.isVerticalMerge())
                                if self.hasVerticalMerge() && dst.hasVerticalMerge() && self.isVerticalMerge() != dst.isVerticalMerge()
                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: else if ((this.hasCellProperties() || !dst.hasCellProperties()) && (!this.hasCellProperties() || dst.hasCellProperties()))
                                else if (self.hasCellProperties() || !dst.hasCellProperties()) && (!self.hasCellProperties() || dst.hasCellProperties())
                        {
                           // Java: if (this.hasCellProperties() && dst.hasCellProperties())
                                    if self.hasCellProperties() && dst.hasCellProperties()
                           {
                              // Java: if (isStrict && !this.cellProperties().equals(dst.cellProperties()))
                                        if isStrict && !self.get_cellProperties().equals(dst.get_cellProperties())
                              {
                                 // Java: return false;
                                 return false;
                              }
                              // Java: if (!isStrict && !this.cellProperties().equalsLax(dst.cellProperties()))
                                        if !isStrict && !self.get_cellProperties().equalsLax(dst.get_cellProperties())
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

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasGridSpan())
        if self.hasGridSpan()
      {
         // Java: ret += Integer.valueOf(this.gridSpan()).hashCode();
         ret = ret.wrapping_add(self.gridSpan());
      }
      // Java: if (this.hasHorizontalMerge())
        if self.hasHorizontalMerge()
      {
         // Java: ret += Boolean.valueOf(this.isHorizontalMerge()).hashCode();
         ret = ret.wrapping_add(if self.isHorizontalMerge() { 1231 } else { 1237 });
      }
      // Java: if (this.hasTableCellIdentifier())
        if self.hasTableCellIdentifier()
      {
         // Java: ret += String.valueOf(this.tableCellIdentifier()).hashCode();
         ret = ret.wrapping_add(java_string_hash(&self.tableCellIdentifier()));
      }
      // Java: if (this.hasRowSpan())
        if self.hasRowSpan()
      {
         // Java: ret += Integer.valueOf(this.rowSpan()).hashCode();
         ret = ret.wrapping_add(self.rowSpan());
      }
      // Java: if (this.hasVerticalMerge())
        if self.hasVerticalMerge()
      {
         // Java: ret += Boolean.valueOf(this.isVerticalMerge()).hashCode();
         ret = ret.wrapping_add(if self.isVerticalMerge() { 1231 } else { 1237 });
      }
      // Java: if (this.hasCellProperties())
        if self.hasCellProperties()
      {
         // Java: ret += this.cellProperties().hashCode();
         ret = ret.wrapping_add(self.get_cellProperties().hashCode());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: return this.cellProperties != null && this.cellProperties.dirty() ? true : this.map.dirty();
      if self.cellProperties.as_ref().map_or(false, |s| s.dirty()) { true } else { self.map.dirty() }
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
      // Java: this.cellProperties = null;
      self.cellProperties = None;
    }

    // Java: void follow(TableCell parent)
    pub fn follow(&mut self, parent: Option<Box<TableCell>>) {
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

    // Java: TableCell parent()
    pub fn get_parent(&self) -> Option<&TableCell> {
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

    // Java: int gridSpan()
    pub fn gridSpan(&self) -> i32 {
      // Java: if (!this.map.has(1))
        if !self.map.has(1)
      {
         // Java: return this.parent == null ? 1 : this.parent.gridSpan();
         return self.parent.as_ref().map_or(1, |p| p.gridSpan());
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

    // Java: boolean hasGridSpan()
    pub fn hasGridSpan(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasGridSpan();
      self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasGridSpan())
    }

    // Java: void deleteGridSpan()
    pub fn deleteGridSpan(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
    }

    // Java: void putGridSpan(int value)
    pub fn putGridSpan(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(1, Encode::from_int(value, self.zone));
    }

    // Java: boolean isHorizontalMerge()
    pub fn isHorizontalMerge(&self) -> bool {
      // Java: if (!this.map.has(2))
        if !self.map.has(2)
      {
         // Java: return this.parent == null ? false : this.parent.isHorizontalMerge();
         return self.parent.as_ref().map_or(false, |p| p.isHorizontalMerge());
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

    // Java: boolean hasHorizontalMerge()
    pub fn hasHorizontalMerge(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasHorizontalMerge();
      self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasHorizontalMerge())
    }

    // Java: void deleteHorizontalMerge()
    pub fn deleteHorizontalMerge(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
    }

    // Java: void putHorizontalMerge(boolean value)
    pub fn putHorizontalMerge(&mut self, value: bool) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(2, Encode::from_bool(value, self.zone));
    }

    // Java: String tableCellIdentifier()
    pub fn tableCellIdentifier(&self) -> String {
      // Java: if (!this.map.has(3))
        if !self.map.has(3)
      {
         // Java: return this.parent == null ? "" : this.parent.tableCellIdentifier();
         return self.parent.as_ref().map_or(String::new(), |p| p.tableCellIdentifier());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(3);
         let bin = self.map.byKey(3);
         // Java: return KoBin.Decode.toText(bin);
         return Decode::toText(bin.as_ref());
      }
    }

    // Java: boolean hasTableCellIdentifier()
    pub fn hasTableCellIdentifier(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasTableCellIdentifier();
      self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasTableCellIdentifier())
    }

    // Java: void deleteTableCellIdentifier()
    pub fn deleteTableCellIdentifier(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
    }

    // Java: void putTableCellIdentifier(String value)
    pub fn putTableCellIdentifier(&mut self, value: &str) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(3, Encode::from_string(value, self.zone));
    }

    // Java: int rowSpan()
    pub fn rowSpan(&self) -> i32 {
      // Java: if (!this.map.has(4))
        if !self.map.has(4)
      {
         // Java: return this.parent == null ? 1 : this.parent.rowSpan();
         return self.parent.as_ref().map_or(1, |p| p.rowSpan());
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

    // Java: boolean hasRowSpan()
    pub fn hasRowSpan(&self) -> bool {
      // Java: return this.map.has(4) || this.parent != null && this.parent.hasRowSpan();
      self.map.has(4) || self.parent.as_ref().map_or(false, |p| p.hasRowSpan())
    }

    // Java: void deleteRowSpan()
    pub fn deleteRowSpan(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
    }

    // Java: void putRowSpan(int value)
    pub fn putRowSpan(&mut self, value: i32) {
      // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
      self.map.put(4, Encode::from_int(value, self.zone));
    }

    // Java: boolean isVerticalMerge()
    pub fn isVerticalMerge(&self) -> bool {
      // Java: if (!this.map.has(5))
        if !self.map.has(5)
      {
         // Java: return this.parent == null ? false : this.parent.isVerticalMerge();
         return self.parent.as_ref().map_or(false, |p| p.isVerticalMerge());
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

    // Java: boolean hasVerticalMerge()
    pub fn hasVerticalMerge(&self) -> bool {
      // Java: return this.map.has(5) || this.parent != null && this.parent.hasVerticalMerge();
      self.map.has(5) || self.parent.as_ref().map_or(false, |p| p.hasVerticalMerge())
    }

    // Java: void deleteVerticalMerge()
    pub fn deleteVerticalMerge(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(5);
    }

    // Java: void putVerticalMerge(boolean value)
    pub fn putVerticalMerge(&mut self, value: bool) {
      // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
      self.map.put(5, Encode::from_bool(value, self.zone));
    }

    // Java: TableCellProperties cellProperties()
    pub fn get_cellProperties(&self) -> &TableCellProperties {
      // Java: if (this.cellProperties == null)
        if self.cellProperties.is_none()
      {
         // Java: if (this.map.has(6))
            if self.map.has(6)
         {
            // Java: KoBin bin = this.map.byKey(6);
            // Java: this.cellProperties = (TableCellProperties)KoBin.Decode.toAny(TableCellProperties.class, bin, this.zone);
            todo!("lazy init cellProperties from map");
         }
         // Java: else if (this.parent != null && this.parent.hasCellProperties())
            else if self.parent.as_ref().map_or(false, |p| p.hasCellProperties())
         {
            // Java: this.cellProperties = this.parent.cellProperties();
            todo!("copy cellProperties from parent");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.cellProperties = (TableCellProperties)KoBin.Decode.toAny(TableCellProperties.class, bin, this.zone);
            todo!("default init cellProperties");
         }
      }
      // Java: return this.cellProperties;
      self.cellProperties.as_deref().unwrap()
    }

    // Java: boolean hasCellProperties()
    pub fn hasCellProperties(&self) -> bool {
      // Java: return this.map.has(6) || this.cellProperties != null && this.cellProperties.dirty() || this.parent != null && this.parent.hasCellProperties();
      self.map.has(6) || self.cellProperties.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasCellProperties())
    }

    // Java: void deleteCellProperties()
    pub fn deleteCellProperties(&mut self) {
      // Java: this.map.put(6, (KoBin)null);
      self.map.clear_key(6);
      // Java: if (this.cellProperties != null)
        if self.cellProperties.is_some()
      {
         // Java: this.cellProperties = null;
         self.cellProperties = None;
      }
    }

    // Java: void writeCellProperties(TableCellProperties value)
    pub fn writeCellProperties(&mut self, value: Option<&TableCellProperties>) {
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
            // Java: if (this.cellProperties != null)
                if self.cellProperties.is_some()
            {
               // Java: this.cellProperties = null;
               self.cellProperties = None;
            }
         }
      }
    }

    // Java: void forceWriteCellProperties(TableCellProperties value)
    pub fn forceWriteCellProperties(&mut self, value: Option<&TableCellProperties>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(6, value.lastBin(this.zone));
         self.map.put(6, v.lastBin(self.zone));
         // Java: if (this.cellProperties != null)
            if self.cellProperties.is_some()
         {
            // Java: this.cellProperties = null;
            self.cellProperties = None;
         }
      }
    }

}

impl KoAr for TableCell {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { TableCell::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { TableCell::lastBin(self, var1) }
}

impl KoClosable for TableCell {
    fn close(&mut self) { TableCell::close(self); }
}
