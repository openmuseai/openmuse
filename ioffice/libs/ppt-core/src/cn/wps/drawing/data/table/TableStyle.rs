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

pub struct TableStyleCellStyle;
impl TableStyleCellStyle {
    pub fn TableStyleCellStyle_from(_bin: &dyn KoBinTrait) -> Self { Self }
    pub fn dirty(&self) -> bool { false }
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> { Null::empty() }
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> { Null::empty() }
    pub fn equals(&self, _o: &Self) -> bool { todo!() }
    pub fn equalsLax(&self, _o: &Self) -> bool { todo!() }
    pub fn hashCode(&self) -> i32 { 0 }
}
pub struct TableStyleTextStyle;
impl TableStyleTextStyle {
    pub fn TableStyleTextStyle_from(_bin: &dyn KoBinTrait) -> Self { Self }
    pub fn dirty(&self) -> bool { false }
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> { Null::empty() }
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> { Null::empty() }
    pub fn equals(&self, _o: &Self) -> bool { todo!() }
    pub fn equalsLax(&self, _o: &Self) -> bool { todo!() }
    pub fn hashCode(&self) -> i32 { 0 }
}
pub struct ThemeableFillStyle;
impl ThemeableFillStyle {
    pub fn ThemeableFillStyle_from(_bin: &dyn KoBinTrait) -> Self { Self }
    pub fn dirty(&self) -> bool { false }
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> { Null::empty() }
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> { Null::empty() }
    pub fn equals(&self, _o: &Self) -> bool { todo!() }
    pub fn equalsLax(&self, _o: &Self) -> bool { todo!() }
    pub fn hashCode(&self) -> i32 { 0 }
}
pub struct ThemeableEffectStyle;
impl ThemeableEffectStyle {
    pub fn ThemeableEffectStyle_from(_bin: &dyn KoBinTrait) -> Self { Self }
    pub fn dirty(&self) -> bool { false }
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> { Null::empty() }
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> { Null::empty() }
    pub fn equals(&self, _o: &Self) -> bool { todo!() }
    pub fn equalsLax(&self, _o: &Self) -> bool { todo!() }
    pub fn hashCode(&self) -> i32 { 0 }
}

// Java: public class TableStyle implements KoAr, KoClosable
pub struct TableStyle {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: TablePartStyle band1Horizontal
    pub(crate) band1Horizontal: Option<Box<TablePartStyle>>,
    // Java field: TablePartStyle band1Vertical
    pub(crate) band1Vertical: Option<Box<TablePartStyle>>,
    // Java field: TablePartStyle band2Horizontal
    pub(crate) band2Horizontal: Option<Box<TablePartStyle>>,
    // Java field: TablePartStyle band2Vertical
    pub(crate) band2Vertical: Option<Box<TablePartStyle>>,
    // Java field: TablePartStyle firstCol
    pub(crate) firstCol: Option<Box<TablePartStyle>>,
    // Java field: TablePartStyle firstRow
    pub(crate) firstRow: Option<Box<TablePartStyle>>,
    // Java field: TablePartStyle lastCol
    pub(crate) lastCol: Option<Box<TablePartStyle>>,
    // Java field: TablePartStyle lastRow
    pub(crate) lastRow: Option<Box<TablePartStyle>>,
    // Java field: TablePartStyle northeastCell
    pub(crate) northeastCell: Option<Box<TablePartStyle>>,
    // Java field: TablePartStyle northwestCell
    pub(crate) northwestCell: Option<Box<TablePartStyle>>,
    // Java field: TablePartStyle southeastCell
    pub(crate) southeastCell: Option<Box<TablePartStyle>>,
    // Java field: TablePartStyle southwestCell
    pub(crate) southwestCell: Option<Box<TablePartStyle>>,
    // Java field: TableBackgroundStyle tableBackground
    pub(crate) tableBackground: Option<Box<TableBackgroundStyle>>,
    // Java field: TablePartStyle wholeTable
    pub(crate) wholeTable: Option<Box<TablePartStyle>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: TableStyle parent
    pub(crate) parent: Option<Box<TableStyle>>,
}

// Java: public class TablePartStyle
pub struct TablePartStyle {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: TableStyleCellStyle tableCellStyle
    pub(crate) tableCellStyle: Option<Box<TableStyleCellStyle>>,
    // Java field: TableStyleTextStyle tableCellTextStyle
    pub(crate) tableCellTextStyle: Option<Box<TableStyleTextStyle>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: TablePartStyle parent
    pub(crate) parent: Option<Box<TablePartStyle>>,
}

// Java: public class TableBackgroundStyle
pub struct TableBackgroundStyle {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: ThemeableFillStyle fillStyle
    pub(crate) fillStyle: Option<Box<ThemeableFillStyle>>,
    // Java field: ThemeableEffectStyle effectStyle
    pub(crate) effectStyle: Option<Box<ThemeableEffectStyle>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: TableBackgroundStyle parent
    pub(crate) parent: Option<Box<TableBackgroundStyle>>,
}

macro_rules! gen_part_style_accessor {
    ($field:ident, $key:expr, $ty:ident, $ty_from:ident) => {};
}

// Java: public class TableStyle implements KoAr, KoClosable
impl TableStyle {
    // Java: static protected static <Value1 extends TableStyle> Value1 initialize(Class<Value1> clazz, KoBin bin)
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

    // Java: static TableStyle create()
    pub fn create() -> Self {
      // Java: return initialize(TableStyle.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static TableStyle open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(TableStyle.class, bin);
      Self::initialize(bin)
    }

    // Java: TableStyle()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Self {
        zone: KoZone::defaultZone(),
        band1Horizontal: None, band1Vertical: None,
        band2Horizontal: None, band2Vertical: None,
        firstCol: None, firstRow: None,
        lastCol: None, lastRow: None,
        northeastCell: None, northwestCell: None,
        southeastCell: None, southwestCell: None,
        tableBackground: None, wholeTable: None,
        bin_box: KpBinBox::new(), map: KpMap::new(),
        parent: None,
      }
    }

    // Java: TableStyle(KoBin bin)
    pub fn new_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Self {
        zone: KoZone::defaultZone(),
        band1Horizontal: None, band1Vertical: None,
        band2Horizontal: None, band2Vertical: None,
        firstCol: None, firstRow: None,
        lastCol: None, lastRow: None,
        northeastCell: None, northwestCell: None,
        southeastCell: None, southwestCell: None,
        tableBackground: None, wholeTable: None,
        bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin),
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
      let zone = self.zone;
      self.loadBin(bin, zone);
    }

    // Java: void load(TableStyle value)
    pub fn load_from(&mut self, value: Option<&TableStyle>) {
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
      // Java: if (this.band1Horizontal != null && this.band1Horizontal.dirty())
        if self.band1Horizontal.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(3, this.band1Horizontal.saveBin(zone));
         let bin = self.band1Horizontal.as_mut().unwrap().saveBin(zone);
         self.map.put(3, bin);
      }
      // Java: if (this.band1Vertical != null && this.band1Vertical.dirty())
        if self.band1Vertical.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(4, this.band1Vertical.saveBin(zone));
         let bin = self.band1Vertical.as_mut().unwrap().saveBin(zone);
         self.map.put(4, bin);
      }
      // Java: if (this.band2Horizontal != null && this.band2Horizontal.dirty())
        if self.band2Horizontal.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(5, this.band2Horizontal.saveBin(zone));
         let bin = self.band2Horizontal.as_mut().unwrap().saveBin(zone);
         self.map.put(5, bin);
      }
      // Java: if (this.band2Vertical != null && this.band2Vertical.dirty())
        if self.band2Vertical.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(6, this.band2Vertical.saveBin(zone));
         let bin = self.band2Vertical.as_mut().unwrap().saveBin(zone);
         self.map.put(6, bin);
      }
      // Java: if (this.firstCol != null && this.firstCol.dirty())
        if self.firstCol.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(7, this.firstCol.saveBin(zone));
         let bin = self.firstCol.as_mut().unwrap().saveBin(zone);
         self.map.put(7, bin);
      }
      // Java: if (this.firstRow != null && this.firstRow.dirty())
        if self.firstRow.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(8, this.firstRow.saveBin(zone));
         let bin = self.firstRow.as_mut().unwrap().saveBin(zone);
         self.map.put(8, bin);
      }
      // Java: if (this.lastCol != null && this.lastCol.dirty())
        if self.lastCol.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(9, this.lastCol.saveBin(zone));
         let bin = self.lastCol.as_mut().unwrap().saveBin(zone);
         self.map.put(9, bin);
      }
      // Java: if (this.lastRow != null && this.lastRow.dirty())
        if self.lastRow.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(10, this.lastRow.saveBin(zone));
         let bin = self.lastRow.as_mut().unwrap().saveBin(zone);
         self.map.put(10, bin);
      }
      // Java: if (this.northeastCell != null && this.northeastCell.dirty())
        if self.northeastCell.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(11, this.northeastCell.saveBin(zone));
         let bin = self.northeastCell.as_mut().unwrap().saveBin(zone);
         self.map.put(11, bin);
      }
      // Java: if (this.northwestCell != null && this.northwestCell.dirty())
        if self.northwestCell.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(12, this.northwestCell.saveBin(zone));
         let bin = self.northwestCell.as_mut().unwrap().saveBin(zone);
         self.map.put(12, bin);
      }
      // Java: if (this.southeastCell != null && this.southeastCell.dirty())
        if self.southeastCell.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(13, this.southeastCell.saveBin(zone));
         let bin = self.southeastCell.as_mut().unwrap().saveBin(zone);
         self.map.put(13, bin);
      }
      // Java: if (this.southwestCell != null && this.southwestCell.dirty())
        if self.southwestCell.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(14, this.southwestCell.saveBin(zone));
         let bin = self.southwestCell.as_mut().unwrap().saveBin(zone);
         self.map.put(14, bin);
      }
      // Java: if (this.tableBackground != null && this.tableBackground.dirty())
        if self.tableBackground.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(15, this.tableBackground.saveBin(zone));
         let bin = self.tableBackground.as_mut().unwrap().saveBin(zone);
         self.map.put(15, bin);
      }
      // Java: if (this.wholeTable != null && this.wholeTable.dirty())
        if self.wholeTable.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(16, this.wholeTable.saveBin(zone));
         let bin = self.wholeTable.as_mut().unwrap().saveBin(zone);
         self.map.put(16, bin);
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(TableStyle o)
    pub fn equalsLax(&self, o: &TableStyle) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TableStyle) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &TableStyle, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof TableStyle)
      {
         // Java: TableStyle dst = (TableStyle)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasStyleId() || !dst.hasStyleId()) && (!this.hasStyleId() || dst.hasStyleId()))
            else if (self.hasStyleId() || !dst.hasStyleId()) && (!self.hasStyleId() || dst.hasStyleId())
         {
            // Java: if (this.hasStyleId() && dst.hasStyleId() && !this.styleId().equals(dst.styleId()))
                if self.hasStyleId() && dst.hasStyleId() && self.styleId() != dst.styleId()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasStyleName() || !dst.hasStyleName()) && (!this.hasStyleName() || dst.hasStyleName()))
                else if (self.hasStyleName() || !dst.hasStyleName()) && (!self.hasStyleName() || dst.hasStyleName())
            {
               // Java: if (this.hasStyleName() && dst.hasStyleName() && !this.styleName().equals(dst.styleName()))
                    if self.hasStyleName() && dst.hasStyleName() && self.styleName() != dst.styleName()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasBand1Horizontal() || !dst.hasBand1Horizontal()) && (!this.hasBand1Horizontal() || dst.hasBand1Horizontal()))
               else if (self.hasBand1Horizontal() || !dst.hasBand1Horizontal()) && (!self.hasBand1Horizontal() || dst.hasBand1Horizontal())
               {
                  // Java: if (this.hasBand1Horizontal() && dst.hasBand1Horizontal())
                  if self.hasBand1Horizontal() && dst.hasBand1Horizontal()
                  {
                     // Java: if (isStrict && !this.band1Horizontal().equals(dst.band1Horizontal()))
                     if isStrict
                     {
                        // Java: return false;
                        todo!();
                     }
                     // Java: if (!isStrict && !this.band1Horizontal().equalsLax(dst.band1Horizontal()))
                     if !isStrict
                     {
                        // Java: return false;
                        todo!();
                     }
                  }
                  // Java: if ((this.hasBand1Vertical() || !dst.hasBand1Vertical()) && (!this.hasBand1Vertical() || dst.hasBand1Vertical()))
                  if (self.hasBand1Vertical() || !dst.hasBand1Vertical()) && (!self.hasBand1Vertical() || dst.hasBand1Vertical())
                  {
                     // Java: if (this.hasBand1Vertical() && dst.hasBand1Vertical())
                     if self.hasBand1Vertical() && dst.hasBand1Vertical()
                     {
                        // Java: if (isStrict && !this.band1Vertical().equals(dst.band1Vertical()))
                        if isStrict
                        {
                           // Java: return false;
                           todo!();
                        }
                        // Java: if (!isStrict && !this.band1Vertical().equalsLax(dst.band1Vertical()))
                        if !isStrict
                        {
                           // Java: return false;
                           todo!();
                        }
                     }
                     // Java: if ((this.hasBand2Horizontal() || !dst.hasBand2Horizontal()) && (!this.hasBand2Horizontal() || dst.hasBand2Horizontal()))
                     if (self.hasBand2Horizontal() || !dst.hasBand2Horizontal()) && (!self.hasBand2Horizontal() || dst.hasBand2Horizontal())
                     {
                        // Java: if (this.hasBand2Horizontal() && dst.hasBand2Horizontal())
                        if self.hasBand2Horizontal() && dst.hasBand2Horizontal()
                        {
                           // Java: if (isStrict && !this.band2Horizontal().equals(dst.band2Horizontal()))
                           if isStrict
                           {
                              // Java: return false;
                              todo!();
                           }
                           // Java: if (!isStrict && !this.band2Horizontal().equalsLax(dst.band2Horizontal()))
                           if !isStrict
                           {
                              // Java: return false;
                              todo!();
                           }
                        }
                        // Java: if ((this.hasBand2Vertical() || !dst.hasBand2Vertical()) && (!this.hasBand2Vertical() || dst.hasBand2Vertical()))
                        if (self.hasBand2Vertical() || !dst.hasBand2Vertical()) && (!self.hasBand2Vertical() || dst.hasBand2Vertical())
                        {
                           // Java: if (this.hasBand2Vertical() && dst.hasBand2Vertical())
                           if self.hasBand2Vertical() && dst.hasBand2Vertical()
                           {
                              // Java: if (isStrict && !this.band2Vertical().equals(dst.band2Vertical()))
                              if isStrict
                              {
                                 // Java: return false;
                                 todo!();
                              }
                              // Java: if (!isStrict && !this.band2Vertical().equalsLax(dst.band2Vertical()))
                              if !isStrict
                              {
                                 // Java: return false;
                                 todo!();
                              }
                           }
                           // Java: if ((this.hasFirstCol() || !dst.hasFirstCol()) && (!this.hasFirstCol() || dst.hasFirstCol()))
                           if (self.hasFirstCol() || !dst.hasFirstCol()) && (!self.hasFirstCol() || dst.hasFirstCol())
                           {
                              // Java: if (this.hasFirstCol() && dst.hasFirstCol())
                              if self.hasFirstCol() && dst.hasFirstCol()
                              {
                                 // Java: if (isStrict && !this.firstCol().equals(dst.firstCol()))
                                 if isStrict
                                 {
                                    // Java: return false;
                                    todo!();
                                 }
                                 // Java: if (!isStrict && !this.firstCol().equalsLax(dst.firstCol()))
                                 if !isStrict
                                 {
                                    // Java: return false;
                                    todo!();
                                 }
                              }
                              // Java: if ((this.hasFirstRow() || !dst.hasFirstRow()) && (!this.hasFirstRow() || dst.hasFirstRow()))
                              if (self.hasFirstRow() || !dst.hasFirstRow()) && (!self.hasFirstRow() || dst.hasFirstRow())
                              {
                                 // Java: if (this.hasFirstRow() && dst.hasFirstRow())
                                 if self.hasFirstRow() && dst.hasFirstRow()
                                 {
                                    // Java: if (isStrict && !this.firstRow().equals(dst.firstRow()))
                                    if isStrict
                                    {
                                       // Java: return false;
                                       todo!();
                                    }
                                    // Java: if (!isStrict && !this.firstRow().equalsLax(dst.firstRow()))
                                    if !isStrict
                                    {
                                       // Java: return false;
                                       todo!();
                                    }
                                 }
                                 // Java: if ((this.hasLastCol() || !dst.hasLastCol()) && (!this.hasLastCol() || dst.hasLastCol()))
                                 if (self.hasLastCol() || !dst.hasLastCol()) && (!self.hasLastCol() || dst.hasLastCol())
                                 {
                                    // Java: if (this.hasLastCol() && dst.hasLastCol())
                                    if self.hasLastCol() && dst.hasLastCol()
                                    {
                                       // Java: if (isStrict && !this.lastCol().equals(dst.lastCol()))
                                       if isStrict
                                       {
                                          // Java: return false;
                                          todo!();
                                       }
                                       // Java: if (!isStrict && !this.lastCol().equalsLax(dst.lastCol()))
                                       if !isStrict
                                       {
                                          // Java: return false;
                                          todo!();
                                       }
                                    }
                                    // Java: if ((this.hasLastRow() || !dst.hasLastRow()) && (!this.hasLastRow() || dst.hasLastRow()))
                                    if (self.hasLastRow() || !dst.hasLastRow()) && (!self.hasLastRow() || dst.hasLastRow())
                                    {
                                       // Java: if (this.hasLastRow() && dst.hasLastRow())
                                       if self.hasLastRow() && dst.hasLastRow()
                                       {
                                          // Java: if (isStrict && !this.lastRow().equals(dst.lastRow()))
                                          if isStrict
                                          {
                                             // Java: return false;
                                             todo!();
                                          }
                                          // Java: if (!isStrict && !this.lastRow().equalsLax(dst.lastRow()))
                                          if !isStrict
                                          {
                                             // Java: return false;
                                             todo!();
                                          }
                                       }
                                       // Java: if ((this.hasNortheastCell() || !dst.hasNortheastCell()) && (!this.hasNortheastCell() || dst.hasNortheastCell()))
                                       if (self.hasNortheastCell() || !dst.hasNortheastCell()) && (!self.hasNortheastCell() || dst.hasNortheastCell())
                                       {
                                          // Java: if (this.hasNortheastCell() && dst.hasNortheastCell())
                                          if self.hasNortheastCell() && dst.hasNortheastCell()
                                          {
                                             // Java: if (isStrict && !this.northeastCell().equals(dst.northeastCell()))
                                             if isStrict
                                             {
                                                // Java: return false;
                                                todo!();
                                             }
                                             // Java: if (!isStrict && !this.northeastCell().equalsLax(dst.northeastCell()))
                                             if !isStrict
                                             {
                                                // Java: return false;
                                                todo!();
                                             }
                                          }
                                          // Java: if ((this.hasNorthwestCell() || !dst.hasNorthwestCell()) && (!this.hasNorthwestCell() || dst.hasNorthwestCell()))
                                          if (self.hasNorthwestCell() || !dst.hasNorthwestCell()) && (!self.hasNorthwestCell() || dst.hasNorthwestCell())
                                          {
                                             // Java: if (this.hasNorthwestCell() && dst.hasNorthwestCell())
                                             if self.hasNorthwestCell() && dst.hasNorthwestCell()
                                             {
                                                // Java: if (isStrict && !this.northwestCell().equals(dst.northwestCell()))
                                                if isStrict
                                                {
                                                   // Java: return false;
                                                   todo!();
                                                }
                                                // Java: if (!isStrict && !this.northwestCell().equalsLax(dst.northwestCell()))
                                                if !isStrict
                                                {
                                                   // Java: return false;
                                                   todo!();
                                                }
                                             }
                                             // Java: if ((this.hasSoutheastCell() || !dst.hasSoutheastCell()) && (!this.hasSoutheastCell() || dst.hasSoutheastCell()))
                                             if (self.hasSoutheastCell() || !dst.hasSoutheastCell()) && (!self.hasSoutheastCell() || dst.hasSoutheastCell())
                                             {
                                                // Java: if (this.hasSoutheastCell() && dst.hasSoutheastCell())
                                                if self.hasSoutheastCell() && dst.hasSoutheastCell()
                                                {
                                                   // Java: if (isStrict && !this.southeastCell().equals(dst.southeastCell()))
                                                   if isStrict
                                                   {
                                                      // Java: return false;
                                                      todo!();
                                                   }
                                                   // Java: if (!isStrict && !this.southeastCell().equalsLax(dst.southeastCell()))
                                                   if !isStrict
                                                   {
                                                      // Java: return false;
                                                      todo!();
                                                   }
                                                }
                                                // Java: if ((this.hasSouthwestCell() || !dst.hasSouthwestCell()) && (!this.hasSouthwestCell() || dst.hasSouthwestCell()))
                                                if (self.hasSouthwestCell() || !dst.hasSouthwestCell()) && (!self.hasSouthwestCell() || dst.hasSouthwestCell())
                                                {
                                                   // Java: if (this.hasSouthwestCell() && dst.hasSouthwestCell())
                                                   if self.hasSouthwestCell() && dst.hasSouthwestCell()
                                                   {
                                                      // Java: if (isStrict && !this.southwestCell().equals(dst.southwestCell()))
                                                      if isStrict
                                                      {
                                                         // Java: return false;
                                                         todo!();
                                                      }
                                                      // Java: if (!isStrict && !this.southwestCell().equalsLax(dst.southwestCell()))
                                                      if !isStrict
                                                      {
                                                         // Java: return false;
                                                         todo!();
                                                      }
                                                   }
                                                   // Java: if ((this.hasTableBackground() || !dst.hasTableBackground()) && (!this.hasTableBackground() || dst.hasTableBackground()))
                                                   if (self.hasTableBackground() || !dst.hasTableBackground()) && (!self.hasTableBackground() || dst.hasTableBackground())
                                                   {
                                                      // Java: if (this.hasTableBackground() && dst.hasTableBackground())
                                                      if self.hasTableBackground() && dst.hasTableBackground()
                                                      {
                                                         // Java: if (isStrict && !this.tableBackground().equals(dst.tableBackground()))
                                                         if isStrict
                                                         {
                                                            // Java: return false;
                                                            todo!();
                                                         }
                                                         // Java: if (!isStrict && !this.tableBackground().equalsLax(dst.tableBackground()))
                                                         if !isStrict
                                                         {
                                                            // Java: return false;
                                                            todo!();
                                                         }
                                                      }
                                                      // Java: if ((this.hasWholeTable() || !dst.hasWholeTable()) && (!this.hasWholeTable() || dst.hasWholeTable()))
                                                      if (self.hasWholeTable() || !dst.hasWholeTable()) && (!self.hasWholeTable() || dst.hasWholeTable())
                                                      {
                                                         // Java: if (this.hasWholeTable() && dst.hasWholeTable())
                                                         if self.hasWholeTable() && dst.hasWholeTable()
                                                         {
                                                            // Java: if (isStrict && !this.wholeTable().equals(dst.wholeTable()))
                                                            if isStrict
                                                            {
                                                               // Java: return false;
                                                               todo!();
                                                            }
                                                            // Java: if (!isStrict && !this.wholeTable().equalsLax(dst.wholeTable()))
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
      // Java: if (this.hasStyleId())
        if self.hasStyleId()
      {
         // Java: ret += String.valueOf(this.styleId()).hashCode();
         let s = self.styleId();
         let mut h: i32 = 0;
         for b in s.bytes() { h = h.wrapping_mul(31).wrapping_add(b as i32); }
         ret = ret.wrapping_add(h);
      }
      // Java: if (this.hasStyleName())
        if self.hasStyleName()
      {
         // Java: ret += String.valueOf(this.styleName()).hashCode();
         let s = self.styleName();
         let mut h: i32 = 0;
         for b in s.bytes() { h = h.wrapping_mul(31).wrapping_add(b as i32); }
         ret = ret.wrapping_add(h);
      }
      // Java: if (this.hasBand1Horizontal())
        if self.hasBand1Horizontal()
      {
         // Java: ret += this.band1Horizontal().hashCode();
         todo!();
      }
      // Java: if (this.hasBand1Vertical())
        if self.hasBand1Vertical()
      {
         // Java: ret += this.band1Vertical().hashCode();
         todo!();
      }
      // Java: if (this.hasBand2Horizontal())
        if self.hasBand2Horizontal()
      {
         // Java: ret += this.band2Horizontal().hashCode();
         todo!();
      }
      // Java: if (this.hasBand2Vertical())
        if self.hasBand2Vertical()
      {
         // Java: ret += this.band2Vertical().hashCode();
         todo!();
      }
      // Java: if (this.hasFirstCol())
        if self.hasFirstCol()
      {
         // Java: ret += this.firstCol().hashCode();
         todo!();
      }
      // Java: if (this.hasFirstRow())
        if self.hasFirstRow()
      {
         // Java: ret += this.firstRow().hashCode();
         todo!();
      }
      // Java: if (this.hasLastCol())
        if self.hasLastCol()
      {
         // Java: ret += this.lastCol().hashCode();
         todo!();
      }
      // Java: if (this.hasLastRow())
        if self.hasLastRow()
      {
         // Java: ret += this.lastRow().hashCode();
         todo!();
      }
      // Java: if (this.hasNortheastCell())
        if self.hasNortheastCell()
      {
         // Java: ret += this.northeastCell().hashCode();
         todo!();
      }
      // Java: if (this.hasNorthwestCell())
        if self.hasNorthwestCell()
      {
         // Java: ret += this.northwestCell().hashCode();
         todo!();
      }
      // Java: if (this.hasSoutheastCell())
        if self.hasSoutheastCell()
      {
         // Java: ret += this.southeastCell().hashCode();
         todo!();
      }
      // Java: if (this.hasSouthwestCell())
        if self.hasSouthwestCell()
      {
         // Java: ret += this.southwestCell().hashCode();
         todo!();
      }
      // Java: if (this.hasTableBackground())
        if self.hasTableBackground()
      {
         // Java: ret += this.tableBackground().hashCode();
         todo!();
      }
      // Java: if (this.hasWholeTable())
        if self.hasWholeTable()
      {
         // Java: ret += this.wholeTable().hashCode();
         todo!();
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.band1Horizontal != null && this.band1Horizontal.dirty())
        if self.band1Horizontal.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.band1Vertical != null && this.band1Vertical.dirty())
        else if self.band1Vertical.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.band2Horizontal != null && this.band2Horizontal.dirty())
        else if self.band2Horizontal.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.band2Vertical != null && this.band2Vertical.dirty())
        else if self.band2Vertical.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.firstCol != null && this.firstCol.dirty())
        else if self.firstCol.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.firstRow != null && this.firstRow.dirty())
        else if self.firstRow.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.lastCol != null && this.lastCol.dirty())
        else if self.lastCol.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.lastRow != null && this.lastRow.dirty())
        else if self.lastRow.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.northeastCell != null && this.northeastCell.dirty())
        else if self.northeastCell.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.northwestCell != null && this.northwestCell.dirty())
        else if self.northwestCell.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.southeastCell != null && this.southeastCell.dirty())
        else if self.southeastCell.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.southwestCell != null && this.southwestCell.dirty())
        else if self.southwestCell.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.tableBackground != null && this.tableBackground.dirty())
        else if self.tableBackground.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
      else
      {
         // Java: return this.wholeTable != null && this.wholeTable.dirty() ? true : this.map.dirty();
         return self.wholeTable.as_ref().map_or(false, |v| v.dirty()) || self.map.dirty();
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
    fn cleanFollow(&mut self) {}

    // Java: void cleanCache()
    fn cleanCache(&mut self) {
      // Java: this.band1Horizontal = null;
      self.band1Horizontal = None;
      // Java: this.band1Vertical = null;
      self.band1Vertical = None;
      // Java: this.band2Horizontal = null;
      self.band2Horizontal = None;
      // Java: this.band2Vertical = null;
      self.band2Vertical = None;
      // Java: this.firstCol = null;
      self.firstCol = None;
      // Java: this.firstRow = null;
      self.firstRow = None;
      // Java: this.lastCol = null;
      self.lastCol = None;
      // Java: this.lastRow = null;
      self.lastRow = None;
      // Java: this.northeastCell = null;
      self.northeastCell = None;
      // Java: this.northwestCell = null;
      self.northwestCell = None;
      // Java: this.southeastCell = null;
      self.southeastCell = None;
      // Java: this.southwestCell = null;
      self.southwestCell = None;
      // Java: this.tableBackground = null;
      self.tableBackground = None;
      // Java: this.wholeTable = null;
      self.wholeTable = None;
    }

    // Java: void follow(TableStyle parent)
    pub fn follow(&mut self, parent: Option<Box<TableStyle>>) {
      // Java: if (parent != null)
      if let Some(mut p) = parent {
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
      if self.parent.is_some() {
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

    // Java: TableStyle parent()
    pub fn parent(&self) -> &TableStyle {
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

    // Java: String styleId()
    pub fn styleId(&self) -> String {
      // Java: if (!this.map.has(1))
        if !self.map.has(1)
      {
         // Java: return this.parent == null ? "" : this.parent.styleId();
         if self.parent.is_none() { return String::new(); }
         else { return self.parent.as_deref().unwrap().styleId(); }
      }
      // Java: else
      {
         // Java: KoBin bin = this.map.byKey(1);
         let bin = self.map.byKey(1);
         // Java: return KoBin.Decode.toText(bin);
         Decode::toText(bin.as_ref())
      }
    }

    // Java: boolean hasStyleId()
    pub fn hasStyleId(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasStyleId();
      self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasStyleId())
    }

    // Java: void deleteStyleId()
    pub fn deleteStyleId(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
    }

    // Java: void putStyleId(String value)
    pub fn putStyleId(&mut self, value: &str) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(1, Encode::from_string(value, self.zone));
    }

    // Java: String styleName()
    pub fn styleName(&self) -> String {
      // Java: if (!this.map.has(2))
        if !self.map.has(2)
      {
         // Java: return this.parent == null ? "" : this.parent.styleName();
         if self.parent.is_none() { return String::new(); }
         else { return self.parent.as_deref().unwrap().styleName(); }
      }
      // Java: else
      {
         // Java: KoBin bin = this.map.byKey(2);
         let bin = self.map.byKey(2);
         // Java: return KoBin.Decode.toText(bin);
         Decode::toText(bin.as_ref())
      }
    }

    // Java: boolean hasStyleName()
    pub fn hasStyleName(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasStyleName();
      self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasStyleName())
    }

    // Java: void deleteStyleName()
    pub fn deleteStyleName(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
    }

    // Java: void putStyleName(String value)
    pub fn putStyleName(&mut self, value: &str) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(2, Encode::from_string(value, self.zone));
    }

    // Java: TablePartStyle band1Horizontal()
    pub fn band1Horizontal(&mut self) -> &TablePartStyle {
      // Java: if (this.band1Horizontal == null)
        if self.band1Horizontal.is_none()
      {
         // Java: if (this.map.has(3))
            if self.map.has(3)
         {
            // Java: KoBin bin = this.map.byKey(3);
            let bin = self.map.byKey(3);
            // Java: this.band1Horizontal = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.band1Horizontal = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasBand1Horizontal())
            else if self.parent.as_ref().map_or(false, |p| p.hasBand1Horizontal())
         {
            // Java: this.band1Horizontal = this.parent.band1Horizontal();
            todo!();
         }
         // Java: else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.band1Horizontal = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.band1Horizontal = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
      }
      // Java: return this.band1Horizontal;
      self.band1Horizontal.as_deref().unwrap()
    }
    // Java: boolean hasBand1Horizontal()
    pub fn hasBand1Horizontal(&self) -> bool {
      // Java: return this.map.has(3) || this.band1Horizontal != null && this.band1Horizontal.dirty() || this.parent != null && this.parent.hasBand1Horizontal();
      self.map.has(3) || self.band1Horizontal.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasBand1Horizontal())
    }
    // Java: void deleteBand1Horizontal()
    pub fn deleteBand1Horizontal(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
      // Java: if (this.band1Horizontal != null)
        if self.band1Horizontal.is_some()
      {
         // Java: this.band1Horizontal = null;
         self.band1Horizontal = None;
      }
    }
    // Java: void writeBand1Horizontal(TablePartStyle value)
    pub fn writeBand1Horizontal(&mut self, value: Option<&TablePartStyle>) {
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
            // Java: if (this.band1Horizontal != null)
                if self.band1Horizontal.is_some()
            {
               // Java: this.band1Horizontal = null;
               self.band1Horizontal = None;
            }
         }
      }
    }
    // Java: void forceWriteBand1Horizontal(TablePartStyle value)
    pub fn forceWriteBand1Horizontal(&mut self, value: Option<&TablePartStyle>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(3, value.lastBin(this.zone));
         self.map.put(3, v.lastBin(self.zone));
         // Java: if (this.band1Horizontal != null)
            if self.band1Horizontal.is_some()
         {
            // Java: this.band1Horizontal = null;
            self.band1Horizontal = None;
         }
      }
    }


    // Java: TablePartStyle band1Vertical()
    pub fn band1Vertical(&mut self) -> &TablePartStyle {
      // Java: if (this.band1Vertical == null)
        if self.band1Vertical.is_none()
      {
         // Java: if (this.map.has(4))
            if self.map.has(4)
         {
            // Java: KoBin bin = this.map.byKey(4);
            let bin = self.map.byKey(4);
            // Java: this.band1Vertical = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.band1Vertical = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasBand1Vertical())
            else if self.parent.as_ref().map_or(false, |p| p.hasBand1Vertical())
         {
            // Java: this.band1Vertical = this.parent.band1Vertical();
            todo!();
         }
         // Java: else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.band1Vertical = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.band1Vertical = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
      }
      // Java: return this.band1Vertical;
      self.band1Vertical.as_deref().unwrap()
    }
    // Java: boolean hasBand1Vertical()
    pub fn hasBand1Vertical(&self) -> bool {
      // Java: return this.map.has(4) || this.band1Vertical != null && this.band1Vertical.dirty() || this.parent != null && this.parent.hasBand1Vertical();
      self.map.has(4) || self.band1Vertical.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasBand1Vertical())
    }
    // Java: void deleteBand1Vertical()
    pub fn deleteBand1Vertical(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
      // Java: if (this.band1Vertical != null)
        if self.band1Vertical.is_some()
      {
         // Java: this.band1Vertical = null;
         self.band1Vertical = None;
      }
    }
    // Java: void writeBand1Vertical(TablePartStyle value)
    pub fn writeBand1Vertical(&mut self, value: Option<&TablePartStyle>) {
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
            // Java: if (this.band1Vertical != null)
                if self.band1Vertical.is_some()
            {
               // Java: this.band1Vertical = null;
               self.band1Vertical = None;
            }
         }
      }
    }
    // Java: void forceWriteBand1Vertical(TablePartStyle value)
    pub fn forceWriteBand1Vertical(&mut self, value: Option<&TablePartStyle>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(4, value.lastBin(this.zone));
         self.map.put(4, v.lastBin(self.zone));
         // Java: if (this.band1Vertical != null)
            if self.band1Vertical.is_some()
         {
            // Java: this.band1Vertical = null;
            self.band1Vertical = None;
         }
      }
    }

    // Java: TablePartStyle band2Horizontal()
    pub fn band2Horizontal(&mut self) -> &TablePartStyle {
      // Java: if (this.band2Horizontal == null)
        if self.band2Horizontal.is_none()
      {
         // Java: if (this.map.has(5))
            if self.map.has(5)
         {
            // Java: KoBin bin = this.map.byKey(5);
            let bin = self.map.byKey(5);
            // Java: this.band2Horizontal = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.band2Horizontal = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasBand2Horizontal())
            else if self.parent.as_ref().map_or(false, |p| p.hasBand2Horizontal())
         {
            // Java: this.band2Horizontal = this.parent.band2Horizontal();
            todo!();
         }
         // Java: else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.band2Horizontal = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.band2Horizontal = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
      }
      // Java: return this.band2Horizontal;
      self.band2Horizontal.as_deref().unwrap()
    }
    // Java: boolean hasBand2Horizontal()
    pub fn hasBand2Horizontal(&self) -> bool {
      // Java: return this.map.has(5) || this.band2Horizontal != null && this.band2Horizontal.dirty() || this.parent != null && this.parent.hasBand2Horizontal();
      self.map.has(5) || self.band2Horizontal.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasBand2Horizontal())
    }
    // Java: void deleteBand2Horizontal()
    pub fn deleteBand2Horizontal(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(5);
      // Java: if (this.band2Horizontal != null)
        if self.band2Horizontal.is_some()
      {
         // Java: this.band2Horizontal = null;
         self.band2Horizontal = None;
      }
    }
    // Java: void writeBand2Horizontal(TablePartStyle value)
    pub fn writeBand2Horizontal(&mut self, value: Option<&TablePartStyle>) {
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
            // Java: if (this.band2Horizontal != null)
                if self.band2Horizontal.is_some()
            {
               // Java: this.band2Horizontal = null;
               self.band2Horizontal = None;
            }
         }
      }
    }
    // Java: void forceWriteBand2Horizontal(TablePartStyle value)
    pub fn forceWriteBand2Horizontal(&mut self, value: Option<&TablePartStyle>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(5, value.lastBin(this.zone));
         self.map.put(5, v.lastBin(self.zone));
         // Java: if (this.band2Horizontal != null)
            if self.band2Horizontal.is_some()
         {
            // Java: this.band2Horizontal = null;
            self.band2Horizontal = None;
         }
      }
    }

    // Java: TablePartStyle band2Vertical()
    pub fn band2Vertical(&mut self) -> &TablePartStyle {
      // Java: if (this.band2Vertical == null)
        if self.band2Vertical.is_none()
      {
         // Java: if (this.map.has(6))
            if self.map.has(6)
         {
            // Java: KoBin bin = this.map.byKey(6);
            let bin = self.map.byKey(6);
            // Java: this.band2Vertical = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.band2Vertical = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasBand2Vertical())
            else if self.parent.as_ref().map_or(false, |p| p.hasBand2Vertical())
         {
            // Java: this.band2Vertical = this.parent.band2Vertical();
            todo!();
         }
         // Java: else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.band2Vertical = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.band2Vertical = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
      }
      // Java: return this.band2Vertical;
      self.band2Vertical.as_deref().unwrap()
    }
    // Java: boolean hasBand2Vertical()
    pub fn hasBand2Vertical(&self) -> bool {
      // Java: return this.map.has(6) || this.band2Vertical != null && this.band2Vertical.dirty() || this.parent != null && this.parent.hasBand2Vertical();
      self.map.has(6) || self.band2Vertical.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasBand2Vertical())
    }
    // Java: void deleteBand2Vertical()
    pub fn deleteBand2Vertical(&mut self) {
      // Java: this.map.put(6, (KoBin)null);
      self.map.clear_key(6);
      // Java: if (this.band2Vertical != null)
        if self.band2Vertical.is_some()
      {
         // Java: this.band2Vertical = null;
         self.band2Vertical = None;
      }
    }
    // Java: void writeBand2Vertical(TablePartStyle value)
    pub fn writeBand2Vertical(&mut self, value: Option<&TablePartStyle>) {
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
            // Java: if (this.band2Vertical != null)
                if self.band2Vertical.is_some()
            {
               // Java: this.band2Vertical = null;
               self.band2Vertical = None;
            }
         }
      }
    }
    // Java: void forceWriteBand2Vertical(TablePartStyle value)
    pub fn forceWriteBand2Vertical(&mut self, value: Option<&TablePartStyle>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(6, value.lastBin(this.zone));
         self.map.put(6, v.lastBin(self.zone));
         // Java: if (this.band2Vertical != null)
            if self.band2Vertical.is_some()
         {
            // Java: this.band2Vertical = null;
            self.band2Vertical = None;
         }
      }
    }

    // Java: TablePartStyle firstCol()
    pub fn firstCol(&mut self) -> &TablePartStyle {
      // Java: if (this.firstCol == null)
        if self.firstCol.is_none()
      {
         // Java: if (this.map.has(7))
            if self.map.has(7)
         {
            // Java: KoBin bin = this.map.byKey(7);
            let bin = self.map.byKey(7);
            // Java: this.firstCol = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.firstCol = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasFirstCol())
            else if self.parent.as_ref().map_or(false, |p| p.hasFirstCol())
         {
            // Java: this.firstCol = this.parent.firstCol();
            todo!();
         }
         // Java: else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.firstCol = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.firstCol = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
      }
      // Java: return this.firstCol;
      self.firstCol.as_deref().unwrap()
    }
    // Java: boolean hasFirstCol()
    pub fn hasFirstCol(&self) -> bool {
      // Java: return this.map.has(7) || this.firstCol != null && this.firstCol.dirty() || this.parent != null && this.parent.hasFirstCol();
      self.map.has(7) || self.firstCol.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasFirstCol())
    }
    // Java: void deleteFirstCol()
    pub fn deleteFirstCol(&mut self) {
      // Java: this.map.put(7, (KoBin)null);
      self.map.clear_key(7);
      // Java: if (this.firstCol != null)
        if self.firstCol.is_some()
      {
         // Java: this.firstCol = null;
         self.firstCol = None;
      }
    }
    // Java: void writeFirstCol(TablePartStyle value)
    pub fn writeFirstCol(&mut self, value: Option<&TablePartStyle>) {
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
            // Java: if (this.firstCol != null)
                if self.firstCol.is_some()
            {
               // Java: this.firstCol = null;
               self.firstCol = None;
            }
         }
      }
    }
    // Java: void forceWriteFirstCol(TablePartStyle value)
    pub fn forceWriteFirstCol(&mut self, value: Option<&TablePartStyle>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(7, value.lastBin(this.zone));
         self.map.put(7, v.lastBin(self.zone));
         // Java: if (this.firstCol != null)
            if self.firstCol.is_some()
         {
            // Java: this.firstCol = null;
            self.firstCol = None;
         }
      }
    }

    // Java: TablePartStyle firstRow()
    pub fn firstRow(&mut self) -> &TablePartStyle {
      // Java: if (this.firstRow == null)
        if self.firstRow.is_none()
      {
         // Java: if (this.map.has(8))
            if self.map.has(8)
         {
            // Java: KoBin bin = this.map.byKey(8);
            let bin = self.map.byKey(8);
            // Java: this.firstRow = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.firstRow = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasFirstRow())
            else if self.parent.as_ref().map_or(false, |p| p.hasFirstRow())
         {
            // Java: this.firstRow = this.parent.firstRow();
            todo!();
         }
         // Java: else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.firstRow = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.firstRow = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
      }
      // Java: return this.firstRow;
      self.firstRow.as_deref().unwrap()
    }
    // Java: boolean hasFirstRow()
    pub fn hasFirstRow(&self) -> bool {
      // Java: return this.map.has(8) || this.firstRow != null && this.firstRow.dirty() || this.parent != null && this.parent.hasFirstRow();
      self.map.has(8) || self.firstRow.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasFirstRow())
    }
    // Java: void deleteFirstRow()
    pub fn deleteFirstRow(&mut self) {
      // Java: this.map.put(8, (KoBin)null);
      self.map.clear_key(8);
      // Java: if (this.firstRow != null)
        if self.firstRow.is_some()
      {
         // Java: this.firstRow = null;
         self.firstRow = None;
      }
    }
    // Java: void writeFirstRow(TablePartStyle value)
    pub fn writeFirstRow(&mut self, value: Option<&TablePartStyle>) {
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
            // Java: if (this.firstRow != null)
                if self.firstRow.is_some()
            {
               // Java: this.firstRow = null;
               self.firstRow = None;
            }
         }
      }
    }
    // Java: void forceWriteFirstRow(TablePartStyle value)
    pub fn forceWriteFirstRow(&mut self, value: Option<&TablePartStyle>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(8, value.lastBin(this.zone));
         self.map.put(8, v.lastBin(self.zone));
         // Java: if (this.firstRow != null)
            if self.firstRow.is_some()
         {
            // Java: this.firstRow = null;
            self.firstRow = None;
         }
      }
    }

    // Java: TablePartStyle lastCol()
    pub fn lastCol(&mut self) -> &TablePartStyle {
      // Java: if (this.lastCol == null)
        if self.lastCol.is_none()
      {
         // Java: if (this.map.has(9))
            if self.map.has(9)
         {
            // Java: KoBin bin = this.map.byKey(9);
            let bin = self.map.byKey(9);
            // Java: this.lastCol = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.lastCol = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasLastCol())
            else if self.parent.as_ref().map_or(false, |p| p.hasLastCol())
         {
            // Java: this.lastCol = this.parent.lastCol();
            todo!();
         }
         // Java: else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.lastCol = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.lastCol = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
      }
      // Java: return this.lastCol;
      self.lastCol.as_deref().unwrap()
    }
    // Java: boolean hasLastCol()
    pub fn hasLastCol(&self) -> bool {
      // Java: return this.map.has(9) || this.lastCol != null && this.lastCol.dirty() || this.parent != null && this.parent.hasLastCol();
      self.map.has(9) || self.lastCol.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasLastCol())
    }
    // Java: void deleteLastCol()
    pub fn deleteLastCol(&mut self) {
      // Java: this.map.put(9, (KoBin)null);
      self.map.clear_key(9);
      // Java: if (this.lastCol != null)
        if self.lastCol.is_some()
      {
         // Java: this.lastCol = null;
         self.lastCol = None;
      }
    }
    // Java: void writeLastCol(TablePartStyle value)
    pub fn writeLastCol(&mut self, value: Option<&TablePartStyle>) {
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
            // Java: if (this.lastCol != null)
                if self.lastCol.is_some()
            {
               // Java: this.lastCol = null;
               self.lastCol = None;
            }
         }
      }
    }
    // Java: void forceWriteLastCol(TablePartStyle value)
    pub fn forceWriteLastCol(&mut self, value: Option<&TablePartStyle>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(9, value.lastBin(this.zone));
         self.map.put(9, v.lastBin(self.zone));
         // Java: if (this.lastCol != null)
            if self.lastCol.is_some()
         {
            // Java: this.lastCol = null;
            self.lastCol = None;
         }
      }
    }

    // Java: TablePartStyle lastRow()
    pub fn lastRow(&mut self) -> &TablePartStyle {
      // Java: if (this.lastRow == null)
        if self.lastRow.is_none()
      {
         // Java: if (this.map.has(10))
            if self.map.has(10)
         {
            // Java: KoBin bin = this.map.byKey(10);
            let bin = self.map.byKey(10);
            // Java: this.lastRow = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.lastRow = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasLastRow())
            else if self.parent.as_ref().map_or(false, |p| p.hasLastRow())
         {
            // Java: this.lastRow = this.parent.lastRow();
            todo!();
         }
         // Java: else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.lastRow = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.lastRow = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
      }
      // Java: return this.lastRow;
      self.lastRow.as_deref().unwrap()
    }
    // Java: boolean hasLastRow()
    pub fn hasLastRow(&self) -> bool {
      // Java: return this.map.has(10) || this.lastRow != null && this.lastRow.dirty() || this.parent != null && this.parent.hasLastRow();
      self.map.has(10) || self.lastRow.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasLastRow())
    }
    // Java: void deleteLastRow()
    pub fn deleteLastRow(&mut self) {
      // Java: this.map.put(10, (KoBin)null);
      self.map.clear_key(10);
      // Java: if (this.lastRow != null)
        if self.lastRow.is_some()
      {
         // Java: this.lastRow = null;
         self.lastRow = None;
      }
    }
    // Java: void writeLastRow(TablePartStyle value)
    pub fn writeLastRow(&mut self, value: Option<&TablePartStyle>) {
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
            // Java: if (this.lastRow != null)
                if self.lastRow.is_some()
            {
               // Java: this.lastRow = null;
               self.lastRow = None;
            }
         }
      }
    }
    // Java: void forceWriteLastRow(TablePartStyle value)
    pub fn forceWriteLastRow(&mut self, value: Option<&TablePartStyle>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(10, value.lastBin(this.zone));
         self.map.put(10, v.lastBin(self.zone));
         // Java: if (this.lastRow != null)
            if self.lastRow.is_some()
         {
            // Java: this.lastRow = null;
            self.lastRow = None;
         }
      }
    }

    // Java: TablePartStyle northeastCell()
    pub fn northeastCell(&mut self) -> &TablePartStyle {
      // Java: if (this.northeastCell == null)
        if self.northeastCell.is_none()
      {
         // Java: if (this.map.has(11))
            if self.map.has(11)
         {
            // Java: KoBin bin = this.map.byKey(11);
            let bin = self.map.byKey(11);
            // Java: this.northeastCell = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.northeastCell = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasNortheastCell())
            else if self.parent.as_ref().map_or(false, |p| p.hasNortheastCell())
         {
            // Java: this.northeastCell = this.parent.northeastCell();
            todo!();
         }
         // Java: else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.northeastCell = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.northeastCell = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
      }
      // Java: return this.northeastCell;
      self.northeastCell.as_deref().unwrap()
    }
    // Java: boolean hasNortheastCell()
    pub fn hasNortheastCell(&self) -> bool {
      // Java: return this.map.has(11) || this.northeastCell != null && this.northeastCell.dirty() || this.parent != null && this.parent.hasNortheastCell();
      self.map.has(11) || self.northeastCell.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasNortheastCell())
    }
    // Java: void deleteNortheastCell()
    pub fn deleteNortheastCell(&mut self) {
      // Java: this.map.put(11, (KoBin)null);
      self.map.clear_key(11);
      // Java: if (this.northeastCell != null)
        if self.northeastCell.is_some()
      {
         // Java: this.northeastCell = null;
         self.northeastCell = None;
      }
    }
    // Java: void writeNortheastCell(TablePartStyle value)
    pub fn writeNortheastCell(&mut self, value: Option<&TablePartStyle>) {
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
            // Java: if (this.northeastCell != null)
                if self.northeastCell.is_some()
            {
               // Java: this.northeastCell = null;
               self.northeastCell = None;
            }
         }
      }
    }
    // Java: void forceWriteNortheastCell(TablePartStyle value)
    pub fn forceWriteNortheastCell(&mut self, value: Option<&TablePartStyle>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(11, value.lastBin(this.zone));
         self.map.put(11, v.lastBin(self.zone));
         // Java: if (this.northeastCell != null)
            if self.northeastCell.is_some()
         {
            // Java: this.northeastCell = null;
            self.northeastCell = None;
         }
      }
    }

    // Java: TablePartStyle northwestCell()
    pub fn northwestCell(&mut self) -> &TablePartStyle {
      // Java: if (this.northwestCell == null)
        if self.northwestCell.is_none()
      {
         // Java: if (this.map.has(12))
            if self.map.has(12)
         {
            // Java: KoBin bin = this.map.byKey(12);
            let bin = self.map.byKey(12);
            // Java: this.northwestCell = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.northwestCell = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasNorthwestCell())
            else if self.parent.as_ref().map_or(false, |p| p.hasNorthwestCell())
         {
            // Java: this.northwestCell = this.parent.northwestCell();
            todo!();
         }
         // Java: else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.northwestCell = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.northwestCell = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
      }
      // Java: return this.northwestCell;
      self.northwestCell.as_deref().unwrap()
    }
    // Java: boolean hasNorthwestCell()
    pub fn hasNorthwestCell(&self) -> bool {
      // Java: return this.map.has(12) || this.northwestCell != null && this.northwestCell.dirty() || this.parent != null && this.parent.hasNorthwestCell();
      self.map.has(12) || self.northwestCell.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasNorthwestCell())
    }
    // Java: void deleteNorthwestCell()
    pub fn deleteNorthwestCell(&mut self) {
      // Java: this.map.put(12, (KoBin)null);
      self.map.clear_key(12);
      // Java: if (this.northwestCell != null)
        if self.northwestCell.is_some()
      {
         // Java: this.northwestCell = null;
         self.northwestCell = None;
      }
    }
    // Java: void writeNorthwestCell(TablePartStyle value)
    pub fn writeNorthwestCell(&mut self, value: Option<&TablePartStyle>) {
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
            // Java: if (this.northwestCell != null)
                if self.northwestCell.is_some()
            {
               // Java: this.northwestCell = null;
               self.northwestCell = None;
            }
         }
      }
    }
    // Java: void forceWriteNorthwestCell(TablePartStyle value)
    pub fn forceWriteNorthwestCell(&mut self, value: Option<&TablePartStyle>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(12, value.lastBin(this.zone));
         self.map.put(12, v.lastBin(self.zone));
         // Java: if (this.northwestCell != null)
            if self.northwestCell.is_some()
         {
            // Java: this.northwestCell = null;
            self.northwestCell = None;
         }
      }
    }

    // Java: TablePartStyle southeastCell()
    pub fn southeastCell(&mut self) -> &TablePartStyle {
      // Java: if (this.southeastCell == null)
        if self.southeastCell.is_none()
      {
         // Java: if (this.map.has(13))
            if self.map.has(13)
         {
            // Java: KoBin bin = this.map.byKey(13);
            let bin = self.map.byKey(13);
            // Java: this.southeastCell = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.southeastCell = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasSoutheastCell())
            else if self.parent.as_ref().map_or(false, |p| p.hasSoutheastCell())
         {
            // Java: this.southeastCell = this.parent.southeastCell();
            todo!();
         }
         // Java: else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.southeastCell = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.southeastCell = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
      }
      // Java: return this.southeastCell;
      self.southeastCell.as_deref().unwrap()
    }
    // Java: boolean hasSoutheastCell()
    pub fn hasSoutheastCell(&self) -> bool {
      // Java: return this.map.has(13) || this.southeastCell != null && this.southeastCell.dirty() || this.parent != null && this.parent.hasSoutheastCell();
      self.map.has(13) || self.southeastCell.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasSoutheastCell())
    }
    // Java: void deleteSoutheastCell()
    pub fn deleteSoutheastCell(&mut self) {
      // Java: this.map.put(13, (KoBin)null);
      self.map.clear_key(13);
      // Java: if (this.southeastCell != null)
        if self.southeastCell.is_some()
      {
         // Java: this.southeastCell = null;
         self.southeastCell = None;
      }
    }
    // Java: void writeSoutheastCell(TablePartStyle value)
    pub fn writeSoutheastCell(&mut self, value: Option<&TablePartStyle>) {
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
            // Java: if (this.southeastCell != null)
                if self.southeastCell.is_some()
            {
               // Java: this.southeastCell = null;
               self.southeastCell = None;
            }
         }
      }
    }
    // Java: void forceWriteSoutheastCell(TablePartStyle value)
    pub fn forceWriteSoutheastCell(&mut self, value: Option<&TablePartStyle>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(13, value.lastBin(this.zone));
         self.map.put(13, v.lastBin(self.zone));
         // Java: if (this.southeastCell != null)
            if self.southeastCell.is_some()
         {
            // Java: this.southeastCell = null;
            self.southeastCell = None;
         }
      }
    }

    // Java: TablePartStyle southwestCell()
    pub fn southwestCell(&mut self) -> &TablePartStyle {
      // Java: if (this.southwestCell == null)
        if self.southwestCell.is_none()
      {
         // Java: if (this.map.has(14))
            if self.map.has(14)
         {
            // Java: KoBin bin = this.map.byKey(14);
            let bin = self.map.byKey(14);
            // Java: this.southwestCell = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.southwestCell = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasSouthwestCell())
            else if self.parent.as_ref().map_or(false, |p| p.hasSouthwestCell())
         {
            // Java: this.southwestCell = this.parent.southwestCell();
            todo!();
         }
         // Java: else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.southwestCell = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.southwestCell = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
      }
      // Java: return this.southwestCell;
      self.southwestCell.as_deref().unwrap()
    }
    // Java: boolean hasSouthwestCell()
    pub fn hasSouthwestCell(&self) -> bool {
      // Java: return this.map.has(14) || this.southwestCell != null && this.southwestCell.dirty() || this.parent != null && this.parent.hasSouthwestCell();
      self.map.has(14) || self.southwestCell.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasSouthwestCell())
    }
    // Java: void deleteSouthwestCell()
    pub fn deleteSouthwestCell(&mut self) {
      // Java: this.map.put(14, (KoBin)null);
      self.map.clear_key(14);
      // Java: if (this.southwestCell != null)
        if self.southwestCell.is_some()
      {
         // Java: this.southwestCell = null;
         self.southwestCell = None;
      }
    }
    // Java: void writeSouthwestCell(TablePartStyle value)
    pub fn writeSouthwestCell(&mut self, value: Option<&TablePartStyle>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(14, bin);
            self.map.put(14, bin);
            // Java: if (this.southwestCell != null)
                if self.southwestCell.is_some()
            {
               // Java: this.southwestCell = null;
               self.southwestCell = None;
            }
         }
      }
    }
    // Java: void forceWriteSouthwestCell(TablePartStyle value)
    pub fn forceWriteSouthwestCell(&mut self, value: Option<&TablePartStyle>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(14, value.lastBin(this.zone));
         self.map.put(14, v.lastBin(self.zone));
         // Java: if (this.southwestCell != null)
            if self.southwestCell.is_some()
         {
            // Java: this.southwestCell = null;
            self.southwestCell = None;
         }
      }
    }

    // Java: TableBackgroundStyle tableBackground()
    pub fn tableBackground(&mut self) -> &TableBackgroundStyle {
      // Java: if (this.tableBackground == null)
        if self.tableBackground.is_none()
      {
         // Java: if (this.map.has(15))
            if self.map.has(15)
         {
            // Java: KoBin bin = this.map.byKey(15);
            let bin = self.map.byKey(15);
            // Java: this.tableBackground = (TableBackgroundStyle)KoBin.Decode.toAny(TableBackgroundStyle.class, bin, this.zone);
            self.tableBackground = Some(Box::new(TableBackgroundStyle::TableBackgroundStyle_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasTableBackground())
            else if self.parent.as_ref().map_or(false, |p| p.hasTableBackground())
         {
            // Java: this.tableBackground = this.parent.tableBackground();
            todo!();
         }
         // Java: else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.tableBackground = (TableBackgroundStyle)KoBin.Decode.toAny(TableBackgroundStyle.class, bin, this.zone);
            self.tableBackground = Some(Box::new(TableBackgroundStyle::TableBackgroundStyle_from(bin.as_ref())));
         }
      }
      // Java: return this.tableBackground;
      self.tableBackground.as_deref().unwrap()
    }
    // Java: boolean hasTableBackground()
    pub fn hasTableBackground(&self) -> bool {
      // Java: return this.map.has(15) || this.tableBackground != null && this.tableBackground.dirty() || this.parent != null && this.parent.hasTableBackground();
      self.map.has(15) || self.tableBackground.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasTableBackground())
    }
    // Java: void deleteTableBackground()
    pub fn deleteTableBackground(&mut self) {
      // Java: this.map.put(15, (KoBin)null);
      self.map.clear_key(15);
      // Java: if (this.tableBackground != null)
        if self.tableBackground.is_some()
      {
         // Java: this.tableBackground = null;
         self.tableBackground = None;
      }
    }
    // Java: void writeTableBackground(TableBackgroundStyle value)
    pub fn writeTableBackground(&mut self, value: Option<&TableBackgroundStyle>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(15, bin);
            self.map.put(15, bin);
            // Java: if (this.tableBackground != null)
                if self.tableBackground.is_some()
            {
               // Java: this.tableBackground = null;
               self.tableBackground = None;
            }
         }
      }
    }
    // Java: void forceWriteTableBackground(TableBackgroundStyle value)
    pub fn forceWriteTableBackground(&mut self, value: Option<&TableBackgroundStyle>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(15, value.lastBin(this.zone));
         self.map.put(15, v.lastBin(self.zone));
         // Java: if (this.tableBackground != null)
            if self.tableBackground.is_some()
         {
            // Java: this.tableBackground = null;
            self.tableBackground = None;
         }
      }
    }

    // Java: TablePartStyle wholeTable()
    pub fn wholeTable(&mut self) -> &TablePartStyle {
      // Java: if (this.wholeTable == null)
        if self.wholeTable.is_none()
      {
         // Java: if (this.map.has(16))
            if self.map.has(16)
         {
            // Java: KoBin bin = this.map.byKey(16);
            let bin = self.map.byKey(16);
            // Java: this.wholeTable = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.wholeTable = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasWholeTable())
            else if self.parent.as_ref().map_or(false, |p| p.hasWholeTable())
         {
            // Java: this.wholeTable = this.parent.wholeTable();
            todo!();
         }
         // Java: else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.wholeTable = (TablePartStyle)KoBin.Decode.toAny(TablePartStyle.class, bin, this.zone);
            self.wholeTable = Some(Box::new(TablePartStyle::TablePartStyle_from(bin.as_ref())));
         }
      }
      // Java: return this.wholeTable;
      self.wholeTable.as_deref().unwrap()
    }
    // Java: boolean hasWholeTable()
    pub fn hasWholeTable(&self) -> bool {
      // Java: return this.map.has(16) || this.wholeTable != null && this.wholeTable.dirty() || this.parent != null && this.parent.hasWholeTable();
      self.map.has(16) || self.wholeTable.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasWholeTable())
    }
    // Java: void deleteWholeTable()
    pub fn deleteWholeTable(&mut self) {
      // Java: this.map.put(16, (KoBin)null);
      self.map.clear_key(16);
      // Java: if (this.wholeTable != null)
        if self.wholeTable.is_some()
      {
         // Java: this.wholeTable = null;
         self.wholeTable = None;
      }
    }
    // Java: void writeWholeTable(TablePartStyle value)
    pub fn writeWholeTable(&mut self, value: Option<&TablePartStyle>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(16, bin);
            self.map.put(16, bin);
            // Java: if (this.wholeTable != null)
                if self.wholeTable.is_some()
            {
               // Java: this.wholeTable = null;
               self.wholeTable = None;
            }
         }
      }
    }
    // Java: void forceWriteWholeTable(TablePartStyle value)
    pub fn forceWriteWholeTable(&mut self, value: Option<&TablePartStyle>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(16, value.lastBin(this.zone));
         self.map.put(16, v.lastBin(self.zone));
         // Java: if (this.wholeTable != null)
            if self.wholeTable.is_some()
         {
            // Java: this.wholeTable = null;
            self.wholeTable = None;
         }
      }
    }
}

// Java: public class TablePartStyle
impl TablePartStyle {
    // Java: static protected static <Value1 extends TablePartStyle> Value1 initialize(Class<Value1> clazz, KoBin bin)
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

    // Java: static TablePartStyle create()
    pub fn create() -> Self {
      // Java: return initialize(TablePartStyle.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static TablePartStyle open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(TablePartStyle.class, bin);
      Self::initialize(bin)
    }

    // Java: TablePartStyle()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Self {
        zone: KoZone::defaultZone(),
        tableCellStyle: None, tableCellTextStyle: None,
        bin_box: KpBinBox::new(), map: KpMap::new(),
        parent: None,
      }
    }

    // Java: TablePartStyle(KoBin bin)
    pub fn new_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Self {
        zone: KoZone::defaultZone(),
        tableCellStyle: None, tableCellTextStyle: None,
        bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin),
        parent: None,
      }
    }
    pub fn TablePartStyle_from(bin: &dyn KoBinTrait) -> Self { Self::new_from(bin) }

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
    // Java: void load(TablePartStyle value)
    pub fn load_from(&mut self, value: Option<&TablePartStyle>) {
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
      // Java: if (this.tableCellStyle != null && this.tableCellStyle.dirty())
        if self.tableCellStyle.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(1, this.tableCellStyle.saveBin(zone));
         let bin = self.tableCellStyle.as_mut().unwrap().saveBin(zone);
         self.map.put(1, bin);
      }
      // Java: if (this.tableCellTextStyle != null && this.tableCellTextStyle.dirty())
        if self.tableCellTextStyle.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(2, this.tableCellTextStyle.saveBin(zone));
         let bin = self.tableCellTextStyle.as_mut().unwrap().saveBin(zone);
         self.map.put(2, bin);
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(TablePartStyle o)
    pub fn equalsLax(&self, o: &TablePartStyle) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }
    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TablePartStyle) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }
    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &TablePartStyle, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof TablePartStyle)
      {
         // Java: TablePartStyle dst = (TablePartStyle)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasTableCellStyle() || !dst.hasTableCellStyle()) && (!this.hasTableCellStyle() || dst.hasTableCellStyle()))
            else if (self.hasTableCellStyle() || !dst.hasTableCellStyle()) && (!self.hasTableCellStyle() || dst.hasTableCellStyle())
         {
            // Java: if (this.hasTableCellStyle() && dst.hasTableCellStyle())
               if self.hasTableCellStyle() && dst.hasTableCellStyle()
            {
               // Java: if (isStrict && !this.tableCellStyle().equals(dst.tableCellStyle()))
               if isStrict
               {
                  // Java: return false;
                  todo!();
               }
               // Java: if (!isStrict && !this.tableCellStyle().equalsLax(dst.tableCellStyle()))
               if !isStrict
               {
                  // Java: return false;
                  todo!();
               }
            }
            // Java: if ((this.hasTableCellTextStyle() || !dst.hasTableCellTextStyle()) && (!this.hasTableCellTextStyle() || dst.hasTableCellTextStyle()))
               if (self.hasTableCellTextStyle() || !dst.hasTableCellTextStyle()) && (!self.hasTableCellTextStyle() || dst.hasTableCellTextStyle())
            {
               // Java: if (this.hasTableCellTextStyle() && dst.hasTableCellTextStyle())
                  if self.hasTableCellTextStyle() && dst.hasTableCellTextStyle()
               {
                  // Java: if (isStrict && !this.tableCellTextStyle().equals(dst.tableCellTextStyle()))
                  if isStrict
                  {
                     // Java: return false;
                     todo!();
                  }
                  // Java: if (!isStrict && !this.tableCellTextStyle().equalsLax(dst.tableCellTextStyle()))
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
      // Java: return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasTableCellStyle())
        if self.hasTableCellStyle()
      {
         // Java: ret += this.tableCellStyle().hashCode();
         todo!();
      }
      // Java: if (this.hasTableCellTextStyle())
        if self.hasTableCellTextStyle()
      {
         // Java: ret += this.tableCellTextStyle().hashCode();
         todo!();
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.tableCellStyle != null && this.tableCellStyle.dirty())
        if self.tableCellStyle.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
      else
      {
         // Java: return this.tableCellTextStyle != null && this.tableCellTextStyle.dirty() ? true : this.map.dirty();
         return self.tableCellTextStyle.as_ref().map_or(false, |v| v.dirty()) || self.map.dirty();
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
    fn cleanFollow(&mut self) {}
    // Java: void cleanCache()
    fn cleanCache(&mut self) {
      // Java: this.tableCellStyle = null;
      self.tableCellStyle = None;
      // Java: this.tableCellTextStyle = null;
      self.tableCellTextStyle = None;
    }
    // Java: void follow(TablePartStyle parent)
    pub fn follow(&mut self, parent: Option<Box<TablePartStyle>>) {
      // Java: if (parent != null)
      if let Some(mut p) = parent {
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
      if self.parent.is_some() {
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
    // Java: TablePartStyle parent()
    pub fn parent(&self) -> &TablePartStyle {
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

    // Java: TableStyleCellStyle tableCellStyle()
    pub fn tableCellStyle(&mut self) -> &TableStyleCellStyle {
      // Java: if (this.tableCellStyle == null)
        if self.tableCellStyle.is_none()
      {
         // Java: if (this.map.has(1))
            if self.map.has(1)
         {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: this.tableCellStyle = (TableStyleCellStyle)KoBin.Decode.toAny(TableStyleCellStyle.class, bin, this.zone);
            self.tableCellStyle = Some(Box::new(TableStyleCellStyle::TableStyleCellStyle_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasTableCellStyle())
            else if self.parent.as_ref().map_or(false, |p| p.hasTableCellStyle())
         {
            // Java: this.tableCellStyle = this.parent.tableCellStyle();
            todo!();
         }
         // Java: else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.tableCellStyle = (TableStyleCellStyle)KoBin.Decode.toAny(TableStyleCellStyle.class, bin, this.zone);
            self.tableCellStyle = Some(Box::new(TableStyleCellStyle::TableStyleCellStyle_from(bin.as_ref())));
         }
      }
      // Java: return this.tableCellStyle;
      self.tableCellStyle.as_deref().unwrap()
    }
    // Java: boolean hasTableCellStyle()
    pub fn hasTableCellStyle(&self) -> bool {
      // Java: return this.map.has(1) || this.tableCellStyle != null && this.tableCellStyle.dirty() || this.parent != null && this.parent.hasTableCellStyle();
      self.map.has(1) || self.tableCellStyle.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasTableCellStyle())
    }
    // Java: void deleteTableCellStyle()
    pub fn deleteTableCellStyle(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
      // Java: if (this.tableCellStyle != null)
        if self.tableCellStyle.is_some()
      {
         // Java: this.tableCellStyle = null;
         self.tableCellStyle = None;
      }
    }
    // Java: void writeTableCellStyle(TableStyleCellStyle value)
    pub fn writeTableCellStyle(&mut self, value: Option<&TableStyleCellStyle>) {
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
            // Java: if (this.tableCellStyle != null)
                if self.tableCellStyle.is_some()
            {
               // Java: this.tableCellStyle = null;
               self.tableCellStyle = None;
            }
         }
      }
    }
    // Java: void forceWriteTableCellStyle(TableStyleCellStyle value)
    pub fn forceWriteTableCellStyle(&mut self, value: Option<&TableStyleCellStyle>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(1, v.lastBin(self.zone));
         // Java: if (this.tableCellStyle != null)
            if self.tableCellStyle.is_some()
         {
            // Java: this.tableCellStyle = null;
            self.tableCellStyle = None;
         }
      }
    }

    // Java: TableStyleTextStyle tableCellTextStyle()
    pub fn tableCellTextStyle(&mut self) -> &TableStyleTextStyle {
      // Java: if (this.tableCellTextStyle == null)
        if self.tableCellTextStyle.is_none()
      {
         // Java: if (this.map.has(2))
            if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: this.tableCellTextStyle = (TableStyleTextStyle)KoBin.Decode.toAny(TableStyleTextStyle.class, bin, this.zone);
            self.tableCellTextStyle = Some(Box::new(TableStyleTextStyle::TableStyleTextStyle_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasTableCellTextStyle())
            else if self.parent.as_ref().map_or(false, |p| p.hasTableCellTextStyle())
         {
            // Java: this.tableCellTextStyle = this.parent.tableCellTextStyle();
            todo!();
         }
         // Java: else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.tableCellTextStyle = (TableStyleTextStyle)KoBin.Decode.toAny(TableStyleTextStyle.class, bin, this.zone);
            self.tableCellTextStyle = Some(Box::new(TableStyleTextStyle::TableStyleTextStyle_from(bin.as_ref())));
         }
      }
      // Java: return this.tableCellTextStyle;
      self.tableCellTextStyle.as_deref().unwrap()
    }
    // Java: boolean hasTableCellTextStyle()
    pub fn hasTableCellTextStyle(&self) -> bool {
      // Java: return this.map.has(2) || this.tableCellTextStyle != null && this.tableCellTextStyle.dirty() || this.parent != null && this.parent.hasTableCellTextStyle();
      self.map.has(2) || self.tableCellTextStyle.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasTableCellTextStyle())
    }
    // Java: void deleteTableCellTextStyle()
    pub fn deleteTableCellTextStyle(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.tableCellTextStyle != null)
        if self.tableCellTextStyle.is_some()
      {
         // Java: this.tableCellTextStyle = null;
         self.tableCellTextStyle = None;
      }
    }
    // Java: void writeTableCellTextStyle(TableStyleTextStyle value)
    pub fn writeTableCellTextStyle(&mut self, value: Option<&TableStyleTextStyle>) {
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
            // Java: if (this.tableCellTextStyle != null)
                if self.tableCellTextStyle.is_some()
            {
               // Java: this.tableCellTextStyle = null;
               self.tableCellTextStyle = None;
            }
         }
      }
    }
    // Java: void forceWriteTableCellTextStyle(TableStyleTextStyle value)
    pub fn forceWriteTableCellTextStyle(&mut self, value: Option<&TableStyleTextStyle>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.tableCellTextStyle != null)
            if self.tableCellTextStyle.is_some()
         {
            // Java: this.tableCellTextStyle = null;
            self.tableCellTextStyle = None;
         }
      }
    }
}

// Java: public class TableBackgroundStyle
impl TableBackgroundStyle {
    // Java: static protected static <Value1 extends TableBackgroundStyle> Value1 initialize(Class<Value1> clazz, KoBin bin)
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

    // Java: static TableBackgroundStyle create()
    pub fn create() -> Self {
      // Java: return initialize(TableBackgroundStyle.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static TableBackgroundStyle open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(TableBackgroundStyle.class, bin);
      Self::initialize(bin)
    }

    // Java: TableBackgroundStyle()
    pub fn new() -> Self {
      // Java: this.box = new KpBinBox();
      // Java: this.map = new KpMap();
      Self {
        zone: KoZone::defaultZone(),
        fillStyle: None, effectStyle: None,
        bin_box: KpBinBox::new(), map: KpMap::new(),
        parent: None,
      }
    }

    // Java: TableBackgroundStyle(KoBin bin)
    pub fn new_from(bin: &dyn KoBinTrait) -> Self {
      // Java: this.box = new KpBinBox(bin);
      // Java: this.map = new KpMap(bin);
      Self {
        zone: KoZone::defaultZone(),
        fillStyle: None, effectStyle: None,
        bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin),
        parent: None,
      }
    }
    pub fn TableBackgroundStyle_from(bin: &dyn KoBinTrait) -> Self { Self::new_from(bin) }

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
    // Java: void load(TableBackgroundStyle value)
    pub fn load_from(&mut self, value: Option<&TableBackgroundStyle>) {
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
      // Java: if (this.fillStyle != null && this.fillStyle.dirty())
        if self.fillStyle.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(1, this.fillStyle.saveBin(zone));
         let bin = self.fillStyle.as_mut().unwrap().saveBin(zone);
         self.map.put(1, bin);
      }
      // Java: if (this.effectStyle != null && this.effectStyle.dirty())
        if self.effectStyle.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: this.map.put(2, this.effectStyle.saveBin(zone));
         let bin = self.effectStyle.as_mut().unwrap().saveBin(zone);
         self.map.put(2, bin);
      }
      // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
      if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); self.bin_box.get() }
      else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(TableBackgroundStyle o)
    pub fn equalsLax(&self, o: &TableBackgroundStyle) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }
    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TableBackgroundStyle) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }
    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &TableBackgroundStyle, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof TableBackgroundStyle)
      {
         // Java: TableBackgroundStyle dst = (TableBackgroundStyle)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasFillStyle() || !dst.hasFillStyle()) && (!this.hasFillStyle() || dst.hasFillStyle()))
            else if (self.hasFillStyle() || !dst.hasFillStyle()) && (!self.hasFillStyle() || dst.hasFillStyle())
         {
            // Java: if (this.hasFillStyle() && dst.hasFillStyle())
               if self.hasFillStyle() && dst.hasFillStyle()
            {
               // Java: if (isStrict && !this.fillStyle().equals(dst.fillStyle()))
               if isStrict
               {
                  // Java: return false;
                  todo!();
               }
               // Java: if (!isStrict && !this.fillStyle().equalsLax(dst.fillStyle()))
               if !isStrict
               {
                  // Java: return false;
                  todo!();
               }
            }
            // Java: if ((this.hasEffectStyle() || !dst.hasEffectStyle()) && (!this.hasEffectStyle() || dst.hasEffectStyle()))
               if (self.hasEffectStyle() || !dst.hasEffectStyle()) && (!self.hasEffectStyle() || dst.hasEffectStyle())
            {
               // Java: if (this.hasEffectStyle() && dst.hasEffectStyle())
                  if self.hasEffectStyle() && dst.hasEffectStyle()
               {
                  // Java: if (isStrict && !this.effectStyle().equals(dst.effectStyle()))
                  if isStrict
                  {
                     // Java: return false;
                     todo!();
                  }
                  // Java: if (!isStrict && !this.effectStyle().equalsLax(dst.effectStyle()))
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
      // Java: return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasFillStyle())
        if self.hasFillStyle()
      {
         // Java: ret += this.fillStyle().hashCode();
         todo!();
      }
      // Java: if (this.hasEffectStyle())
        if self.hasEffectStyle()
      {
         // Java: ret += this.effectStyle().hashCode();
         todo!();
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.fillStyle != null && this.fillStyle.dirty())
        if self.fillStyle.as_ref().map_or(false, |v| v.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
      else
      {
         // Java: return this.effectStyle != null && this.effectStyle.dirty() ? true : this.map.dirty();
         return self.effectStyle.as_ref().map_or(false, |v| v.dirty()) || self.map.dirty();
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
    fn cleanFollow(&mut self) {}
    // Java: void cleanCache()
    fn cleanCache(&mut self) {
      // Java: this.fillStyle = null;
      self.fillStyle = None;
      // Java: this.effectStyle = null;
      self.effectStyle = None;
    }
    // Java: void follow(TableBackgroundStyle parent)
    pub fn follow(&mut self, parent: Option<Box<TableBackgroundStyle>>) {
      // Java: if (parent != null)
      if let Some(mut p) = parent {
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
      if self.parent.is_some() {
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
    // Java: TableBackgroundStyle parent()
    pub fn parent(&self) -> &TableBackgroundStyle {
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

    // Java: ThemeableFillStyle fillStyle()
    pub fn fillStyle(&mut self) -> &ThemeableFillStyle {
      // Java: if (this.fillStyle == null)
        if self.fillStyle.is_none()
      {
         // Java: if (this.map.has(1))
            if self.map.has(1)
         {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: this.fillStyle = (ThemeableFillStyle)KoBin.Decode.toAny(ThemeableFillStyle.class, bin, this.zone);
            self.fillStyle = Some(Box::new(ThemeableFillStyle::ThemeableFillStyle_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasFillStyle())
            else if self.parent.as_ref().map_or(false, |p| p.hasFillStyle())
         {
            // Java: this.fillStyle = this.parent.fillStyle();
            todo!();
         }
         // Java: else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.fillStyle = (ThemeableFillStyle)KoBin.Decode.toAny(ThemeableFillStyle.class, bin, this.zone);
            self.fillStyle = Some(Box::new(ThemeableFillStyle::ThemeableFillStyle_from(bin.as_ref())));
         }
      }
      // Java: return this.fillStyle;
      self.fillStyle.as_deref().unwrap()
    }
    // Java: boolean hasFillStyle()
    pub fn hasFillStyle(&self) -> bool {
      // Java: return this.map.has(1) || this.fillStyle != null && this.fillStyle.dirty() || this.parent != null && this.parent.hasFillStyle();
      self.map.has(1) || self.fillStyle.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasFillStyle())
    }
    // Java: void deleteFillStyle()
    pub fn deleteFillStyle(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
      // Java: if (this.fillStyle != null)
        if self.fillStyle.is_some()
      {
         // Java: this.fillStyle = null;
         self.fillStyle = None;
      }
    }
    // Java: void writeFillStyle(ThemeableFillStyle value)
    pub fn writeFillStyle(&mut self, value: Option<&ThemeableFillStyle>) {
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
            // Java: if (this.fillStyle != null)
                if self.fillStyle.is_some()
            {
               // Java: this.fillStyle = null;
               self.fillStyle = None;
            }
         }
      }
    }
    // Java: void forceWriteFillStyle(ThemeableFillStyle value)
    pub fn forceWriteFillStyle(&mut self, value: Option<&ThemeableFillStyle>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(1, value.lastBin(this.zone));
         self.map.put(1, v.lastBin(self.zone));
         // Java: if (this.fillStyle != null)
            if self.fillStyle.is_some()
         {
            // Java: this.fillStyle = null;
            self.fillStyle = None;
         }
      }
    }

    // Java: ThemeableEffectStyle effectStyle()
    pub fn effectStyle(&mut self) -> &ThemeableEffectStyle {
      // Java: if (this.effectStyle == null)
        if self.effectStyle.is_none()
      {
         // Java: if (this.map.has(2))
            if self.map.has(2)
         {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: this.effectStyle = (ThemeableEffectStyle)KoBin.Decode.toAny(ThemeableEffectStyle.class, bin, this.zone);
            self.effectStyle = Some(Box::new(ThemeableEffectStyle::ThemeableEffectStyle_from(bin.as_ref())));
         }
         // Java: else if (this.parent != null && this.parent.hasEffectStyle())
            else if self.parent.as_ref().map_or(false, |p| p.hasEffectStyle())
         {
            // Java: this.effectStyle = this.parent.effectStyle();
            todo!();
         }
         // Java: else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            let bin = Null::value1();
            // Java: this.effectStyle = (ThemeableEffectStyle)KoBin.Decode.toAny(ThemeableEffectStyle.class, bin, this.zone);
            self.effectStyle = Some(Box::new(ThemeableEffectStyle::ThemeableEffectStyle_from(bin.as_ref())));
         }
      }
      // Java: return this.effectStyle;
      self.effectStyle.as_deref().unwrap()
    }
    // Java: boolean hasEffectStyle()
    pub fn hasEffectStyle(&self) -> bool {
      // Java: return this.map.has(2) || this.effectStyle != null && this.effectStyle.dirty() || this.parent != null && this.parent.hasEffectStyle();
      self.map.has(2) || self.effectStyle.as_ref().map_or(false, |v| v.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasEffectStyle())
    }
    // Java: void deleteEffectStyle()
    pub fn deleteEffectStyle(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
      // Java: if (this.effectStyle != null)
        if self.effectStyle.is_some()
      {
         // Java: this.effectStyle = null;
         self.effectStyle = None;
      }
    }
    // Java: void writeEffectStyle(ThemeableEffectStyle value)
    pub fn writeEffectStyle(&mut self, value: Option<&ThemeableEffectStyle>) {
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
            // Java: if (this.effectStyle != null)
                if self.effectStyle.is_some()
            {
               // Java: this.effectStyle = null;
               self.effectStyle = None;
            }
         }
      }
    }
    // Java: void forceWriteEffectStyle(ThemeableEffectStyle value)
    pub fn forceWriteEffectStyle(&mut self, value: Option<&ThemeableEffectStyle>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(2, value.lastBin(this.zone));
         self.map.put(2, v.lastBin(self.zone));
         // Java: if (this.effectStyle != null)
            if self.effectStyle.is_some()
         {
            // Java: this.effectStyle = null;
            self.effectStyle = None;
         }
      }
    }
}
