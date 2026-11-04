#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.fill.Fill
use crate::cn::wps::drawing::data::fill::Fill::Fill;
// Java: import cn.wps.drawing.data.outline.Outline
use crate::cn::wps::drawing::data::outline::Outline::Outline;
// Java: import cn.wps.drawing.data.threeD.Cell3D
use crate::cn::wps::drawing::data::threeD::Cell3D::Cell3D;
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

use crate::cn::wps::drawing::data::table::Headers::Headers;

// Java: public class TableCellProperties implements KoAr, KoClosable
pub struct TableCellProperties {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Fill fill
    pub(crate) fill: Option<Box<Fill>>,
    // Java field: Cell3D cell3D
    pub(crate) cell3D: Option<Box<Cell3D>>,
    // Java field: Headers headers
    pub(crate) headers: Option<Box<Headers>>,
    // Java field: Outline bottomLine
    pub(crate) bottomLine: Option<Box<Outline>>,
    // Java field: Outline bottomLeft2TopRightLine
    pub(crate) bottomLeft2TopRightLine: Option<Box<Outline>>,
    // Java field: Outline leftLine
    pub(crate) leftLine: Option<Box<Outline>>,
    // Java field: Outline rightLine
    pub(crate) rightLine: Option<Box<Outline>>,
    // Java field: Outline topLine
    pub(crate) topLine: Option<Box<Outline>>,
    // Java field: Outline topLeft2BottomRightLine
    pub(crate) topLeft2BottomRightLine: Option<Box<Outline>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: TableCellProperties parent
    pub(crate) parent: Option<Box<TableCellProperties>>,
}

// Java: public class TableCellProperties implements KoAr, KoClosable
impl TableCellProperties {
    // Java: static protected static <Value1 extends TableCellProperties> Value1 initialize(Class<Value1> clazz, KoBin bin)
    fn initialize(bin: &dyn KoBinTrait) -> Self {
      // Java: KoZone zone = KoZone.enter();
      let zone = KoZone::enter();
      // Java: Value1 inst = (Value1)(zone.allocate(clazz));
      let mut inst = Self::TableCellProperties_from(bin);
      // Java: inst.zone = zone;
      inst.zone = zone;
      // Java: inst.load(bin);
      inst.load_bin(bin);
      // Java: return inst;
      inst
    }

    // Java: static TableCellProperties create()
    pub fn create() -> Self {
      // Java: return initialize(TableCellProperties.class, KoBin.Null.empty());
      let empty = Null::empty();
      Self::initialize(empty.as_ref())
    }

    // Java: static TableCellProperties open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
      // Java: return initialize(TableCellProperties.class, bin);
      Self::initialize(bin)
    }

    // Java: TableCellProperties()
    pub fn new() -> Self {
      Self {
      // Java: this.box = new KpBinBox();
      bin_box: KpBinBox::new(),
      // Java: this.map = new KpMap();
      map: KpMap::new(),
      zone: KoZone::defaultZone(), fill: None, cell3D: None, headers: None,
      bottomLine: None, bottomLeft2TopRightLine: None, leftLine: None,
      rightLine: None, topLine: None, topLeft2BottomRightLine: None, parent: None,
      }
    }

    // Java: TableCellProperties(KoBin bin)
    pub fn TableCellProperties_from(bin: &dyn KoBinTrait) -> Self {
      Self {
      // Java: this.box = new KpBinBox(bin);
      bin_box: KpBinBox::KpBinBox_from(Some(bin)),
      // Java: this.map = new KpMap(bin);
      map: KpMap::KpMap_from(bin),
      zone: KoZone::defaultZone(), fill: None, cell3D: None, headers: None,
      bottomLine: None, bottomLeft2TopRightLine: None, leftLine: None,
      rightLine: None, topLine: None, topLeft2BottomRightLine: None, parent: None,
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

    // Java: void load(TableCellProperties value)
    pub fn load_value(&mut self, value: Option<&TableCellProperties>) {
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
      // Java: if (this.fill != null && this.fill.dirty())
        if self.fill.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(9, this.fill.saveBin(zone));
         let bin = self.fill.as_mut().unwrap().saveBin(zone);
         self.map.put(9, bin);
      }
      // Java: if (this.cell3D != null && this.cell3D.dirty())
        if self.cell3D.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(10, this.cell3D.saveBin(zone));
         let bin = self.cell3D.as_mut().unwrap().saveBin(zone);
         self.map.put(10, bin);
      }
      // Java: if (this.headers != null && this.headers.dirty())
        if self.headers.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(11, this.headers.saveBin(zone));
         let bin = self.headers.as_mut().unwrap().saveBin(zone);
         self.map.put(11, bin);
      }
      // Java: if (this.bottomLine != null && this.bottomLine.dirty())
        if self.bottomLine.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(12, this.bottomLine.saveBin(zone));
         let bin = self.bottomLine.as_mut().unwrap().saveBin(zone);
         self.map.put(12, bin);
      }
      // Java: if (this.bottomLeft2TopRightLine != null && this.bottomLeft2TopRightLine.dirty())
        if self.bottomLeft2TopRightLine.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(13, this.bottomLeft2TopRightLine.saveBin(zone));
         let bin = self.bottomLeft2TopRightLine.as_mut().unwrap().saveBin(zone);
         self.map.put(13, bin);
      }
      // Java: if (this.leftLine != null && this.leftLine.dirty())
        if self.leftLine.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(14, this.leftLine.saveBin(zone));
         let bin = self.leftLine.as_mut().unwrap().saveBin(zone);
         self.map.put(14, bin);
      }
      // Java: if (this.rightLine != null && this.rightLine.dirty())
        if self.rightLine.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(15, this.rightLine.saveBin(zone));
         let bin = self.rightLine.as_mut().unwrap().saveBin(zone);
         self.map.put(15, bin);
      }
      // Java: if (this.topLine != null && this.topLine.dirty())
        if self.topLine.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(16, this.topLine.saveBin(zone));
         let bin = self.topLine.as_mut().unwrap().saveBin(zone);
         self.map.put(16, bin);
      }
      // Java: if (this.topLeft2BottomRightLine != null && this.topLeft2BottomRightLine.dirty())
        if self.topLeft2BottomRightLine.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: this.map.put(17, this.topLeft2BottomRightLine.saveBin(zone));
         let bin = self.topLeft2BottomRightLine.as_mut().unwrap().saveBin(zone);
         self.map.put(17, bin);
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

    // Java: boolean equalsLax(TableCellProperties o)
    pub fn equalsLax(&self, o: &TableCellProperties) -> bool {
      // Java: return this.equals(o, false);
      self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TableCellProperties) -> bool {
      // Java: return this.equals(o, true);
      self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &TableCellProperties, isStrict: bool) -> bool {
      // Java: if (null != o && o instanceof TableCellProperties)
        if true
      {
         // Java: TableCellProperties dst = (TableCellProperties)o;
         let dst = o;
         // Java: if (this == dst)
            if std::ptr::eq(self, dst)
         {
            // Java: return true;
            return true;
         }
         // Java: else if ((this.hasAnchor() || !dst.hasAnchor()) && (!this.hasAnchor() || dst.hasAnchor()))
            else if (self.hasAnchor() || !dst.hasAnchor()) && (!self.hasAnchor() || dst.hasAnchor())
         {
            // Java: if (this.hasAnchor() && dst.hasAnchor() && this.anchor() != dst.anchor())
                if self.hasAnchor() && dst.hasAnchor() && self.anchor() != dst.anchor()
            {
               // Java: return false;
               return false;
            }
            // Java: else if ((this.hasAnchorCenter() || !dst.hasAnchorCenter()) && (!this.hasAnchorCenter() || dst.hasAnchorCenter()))
                else if (self.hasAnchorCenter() || !dst.hasAnchorCenter()) && (!self.hasAnchorCenter() || dst.hasAnchorCenter())
            {
               // Java: if (this.hasAnchorCenter() && dst.hasAnchorCenter() && this.isAnchorCenter() != dst.isAnchorCenter())
                    if self.hasAnchorCenter() && dst.hasAnchorCenter() && self.isAnchorCenter() != dst.isAnchorCenter()
               {
                  // Java: return false;
                  return false;
               }
               // Java: else if ((this.hasHorizontalOverflow() || !dst.hasHorizontalOverflow()) && (!this.hasHorizontalOverflow() || dst.hasHorizontalOverflow()))
                    else if (self.hasHorizontalOverflow() || !dst.hasHorizontalOverflow()) && (!self.hasHorizontalOverflow() || dst.hasHorizontalOverflow())
               {
                  // Java: if (this.hasHorizontalOverflow() && dst.hasHorizontalOverflow() && this.horizontalOverflow() != dst.horizontalOverflow())
                        if self.hasHorizontalOverflow() && dst.hasHorizontalOverflow() && self.horizontalOverflow() != dst.horizontalOverflow()
                  {
                     // Java: return false;
                     return false;
                  }
                  // Java: else if ((this.hasTextDirection() || !dst.hasTextDirection()) && (!this.hasTextDirection() || dst.hasTextDirection()))
                        else if (self.hasTextDirection() || !dst.hasTextDirection()) && (!self.hasTextDirection() || dst.hasTextDirection())
                  {
                     // Java: if (this.hasTextDirection() && dst.hasTextDirection() && this.textDirection() != dst.textDirection())
                            if self.hasTextDirection() && dst.hasTextDirection() && self.textDirection() != dst.textDirection()
                     {
                        // Java: return false;
                        return false;
                     }
                     // Java: else if ((this.hasBottomMargin() || !dst.hasBottomMargin()) && (!this.hasBottomMargin() || dst.hasBottomMargin()))
                            else if (self.hasBottomMargin() || !dst.hasBottomMargin()) && (!self.hasBottomMargin() || dst.hasBottomMargin())
                     {
                        // Java: if (this.hasBottomMargin() && dst.hasBottomMargin() && this.bottomMargin() != dst.bottomMargin())
                                if self.hasBottomMargin() && dst.hasBottomMargin() && self.bottomMargin() != dst.bottomMargin()
                        {
                           // Java: return false;
                           return false;
                        }
                        // Java: else if ((this.hasLeftMargin() || !dst.hasLeftMargin()) && (!this.hasLeftMargin() || dst.hasLeftMargin()))
                                else if (self.hasLeftMargin() || !dst.hasLeftMargin()) && (!self.hasLeftMargin() || dst.hasLeftMargin())
                        {
                           // Java: if (this.hasLeftMargin() && dst.hasLeftMargin() && this.leftMargin() != dst.leftMargin())
                                    if self.hasLeftMargin() && dst.hasLeftMargin() && self.leftMargin() != dst.leftMargin()
                           {
                              // Java: return false;
                              return false;
                           }
                           // Java: else if ((this.hasRightMargin() || !dst.hasRightMargin()) && (!this.hasRightMargin() || dst.hasRightMargin()))
                                    else if (self.hasRightMargin() || !dst.hasRightMargin()) && (!self.hasRightMargin() || dst.hasRightMargin())
                           {
                              // Java: if (this.hasRightMargin() && dst.hasRightMargin() && this.rightMargin() != dst.rightMargin())
                                        if self.hasRightMargin() && dst.hasRightMargin() && self.rightMargin() != dst.rightMargin()
                              {
                                 // Java: return false;
                                 return false;
                              }
                              // Java: else if ((this.hasTopMargin() || !dst.hasTopMargin()) && (!this.hasTopMargin() || dst.hasTopMargin()))
                                        else if (self.hasTopMargin() || !dst.hasTopMargin()) && (!self.hasTopMargin() || dst.hasTopMargin())
                              {
                                 // Java: if (this.hasTopMargin() && dst.hasTopMargin() && this.topMargin() != dst.topMargin())
                                            if self.hasTopMargin() && dst.hasTopMargin() && self.topMargin() != dst.topMargin()
                                 {
                                    // Java: return false;
                                    return false;
                                 }
                                 // Java: else if ((this.hasFill() || !dst.hasFill()) && (!this.hasFill() || dst.hasFill()))
                                            else if (self.hasFill() || !dst.hasFill()) && (!self.hasFill() || dst.hasFill())
                                 {
                                    // Java: if (this.hasFill() && dst.hasFill())
                                                if self.hasFill() && dst.hasFill()
                                    {
                                       // Java: if (isStrict && !this.fill().equals(dst.fill()))
                                                    if isStrict && !self.get_fill().equals(dst.get_fill())
                                       {
                                          // Java: return false;
                                          return false;
                                       }
                                       // Java: if (!isStrict && !this.fill().equalsLax(dst.fill()))
                                                    if !isStrict && !self.get_fill().equalsLax(dst.get_fill())
                                       {
                                          // Java: return false;
                                          return false;
                                       }
                                    }
                                    // Java: if ((this.hasCell3D() || !dst.hasCell3D()) && (!this.hasCell3D() || dst.hasCell3D()))
                                                if (self.hasCell3D() || !dst.hasCell3D()) && (!self.hasCell3D() || dst.hasCell3D())
                                    {
                                       // Java: if (this.hasCell3D() && dst.hasCell3D())
                                                    if self.hasCell3D() && dst.hasCell3D()
                                       {
                                          // Java: if (isStrict && !this.cell3D().equals(dst.cell3D()))
                                                        if isStrict && !self.get_cell3D().equals(dst.get_cell3D())
                                          {
                                             // Java: return false;
                                             return false;
                                          }
                                          // Java: if (!isStrict && !this.cell3D().equalsLax(dst.cell3D()))
                                                        if !isStrict && !self.get_cell3D().equalsLax(dst.get_cell3D())
                                          {
                                             // Java: return false;
                                             return false;
                                          }
                                       }
                                       // Java: if ((this.hasHeaders() || !dst.hasHeaders()) && (!this.hasHeaders() || dst.hasHeaders()))
                                                    if (self.hasHeaders() || !dst.hasHeaders()) && (!self.hasHeaders() || dst.hasHeaders())
                                       {
                                          // Java: if (this.hasHeaders() && dst.hasHeaders())
                                                        if self.hasHeaders() && dst.hasHeaders()
                                          {
                                             // Java: if (isStrict && !this.headers().equals(dst.headers()))
                                                            if isStrict && !self.get_headers().equals(dst.get_headers())
                                             {
                                                // Java: return false;
                                                return false;
                                             }
                                             // Java: if (!isStrict && !this.headers().equalsLax(dst.headers()))
                                                            if !isStrict && !self.get_headers().equalsLax(dst.get_headers())
                                             {
                                                // Java: return false;
                                                return false;
                                             }
                                          }
                                          // Java: if ((this.hasBottomLine() || !dst.hasBottomLine()) && (!this.hasBottomLine() || dst.hasBottomLine()))
                                                        if (self.hasBottomLine() || !dst.hasBottomLine()) && (!self.hasBottomLine() || dst.hasBottomLine())
                                          {
                                             // Java: if (this.hasBottomLine() && dst.hasBottomLine())
                                                            if self.hasBottomLine() && dst.hasBottomLine()
                                             {
                                                // Java: if (isStrict && !this.bottomLine().equals(dst.bottomLine()))
                                                                if isStrict && !self.get_bottomLine().equals(dst.get_bottomLine())
                                                {
                                                   // Java: return false;
                                                   return false;
                                                }
                                                // Java: if (!isStrict && !this.bottomLine().equalsLax(dst.bottomLine()))
                                                                if !isStrict && !self.get_bottomLine().equalsLax(dst.get_bottomLine())
                                                {
                                                   // Java: return false;
                                                   return false;
                                                }
                                             }
                                             // Java: if ((this.hasBottomLeft2TopRightLine() || !dst.hasBottomLeft2TopRightLine()) && (!this.hasBottomLeft2TopRightLine() || dst.hasBottomLeft2TopRightLine()))
                                                            if (self.hasBottomLeft2TopRightLine() || !dst.hasBottomLeft2TopRightLine()) && (!self.hasBottomLeft2TopRightLine() || dst.hasBottomLeft2TopRightLine())
                                             {
                                                // Java: if (this.hasBottomLeft2TopRightLine() && dst.hasBottomLeft2TopRightLine())
                                                                if self.hasBottomLeft2TopRightLine() && dst.hasBottomLeft2TopRightLine()
                                                {
                                                   // Java: if (isStrict && !this.bottomLeft2TopRightLine().equals(dst.bottomLeft2TopRightLine()))
                                                                    if isStrict && !self.get_bottomLeft2TopRightLine().equals(dst.get_bottomLeft2TopRightLine())
                                                   {
                                                      // Java: return false;
                                                      return false;
                                                   }
                                                   // Java: if (!isStrict && !this.bottomLeft2TopRightLine().equalsLax(dst.bottomLeft2TopRightLine()))
                                                                    if !isStrict && !self.get_bottomLeft2TopRightLine().equalsLax(dst.get_bottomLeft2TopRightLine())
                                                   {
                                                      // Java: return false;
                                                      return false;
                                                   }
                                                }
                                                // Java: if ((this.hasLeftLine() || !dst.hasLeftLine()) && (!this.hasLeftLine() || dst.hasLeftLine()))
                                                                if (self.hasLeftLine() || !dst.hasLeftLine()) && (!self.hasLeftLine() || dst.hasLeftLine())
                                                {
                                                   // Java: if (this.hasLeftLine() && dst.hasLeftLine())
                                                                    if self.hasLeftLine() && dst.hasLeftLine()
                                                   {
                                                      // Java: if (isStrict && !this.leftLine().equals(dst.leftLine()))
                                                                        if isStrict && !self.get_leftLine().equals(dst.get_leftLine())
                                                      {
                                                         // Java: return false;
                                                         return false;
                                                      }
                                                      // Java: if (!isStrict && !this.leftLine().equalsLax(dst.leftLine()))
                                                                        if !isStrict && !self.get_leftLine().equalsLax(dst.get_leftLine())
                                                      {
                                                         // Java: return false;
                                                         return false;
                                                      }
                                                   }
                                                   // Java: if ((this.hasRightLine() || !dst.hasRightLine()) && (!this.hasRightLine() || dst.hasRightLine()))
                                                                    if (self.hasRightLine() || !dst.hasRightLine()) && (!self.hasRightLine() || dst.hasRightLine())
                                                   {
                                                      // Java: if (this.hasRightLine() && dst.hasRightLine())
                                                                        if self.hasRightLine() && dst.hasRightLine()
                                                      {
                                                         // Java: if (isStrict && !this.rightLine().equals(dst.rightLine()))
                                                                            if isStrict && !self.get_rightLine().equals(dst.get_rightLine())
                                                         {
                                                            // Java: return false;
                                                            return false;
                                                         }
                                                         // Java: if (!isStrict && !this.rightLine().equalsLax(dst.rightLine()))
                                                                            if !isStrict && !self.get_rightLine().equalsLax(dst.get_rightLine())
                                                         {
                                                            // Java: return false;
                                                            return false;
                                                         }
                                                      }
                                                      // Java: if ((this.hasTopLine() || !dst.hasTopLine()) && (!this.hasTopLine() || dst.hasTopLine()))
                                                                        if (self.hasTopLine() || !dst.hasTopLine()) && (!self.hasTopLine() || dst.hasTopLine())
                                                      {
                                                         // Java: if (this.hasTopLine() && dst.hasTopLine())
                                                                            if self.hasTopLine() && dst.hasTopLine()
                                                         {
                                                            // Java: if (isStrict && !this.topLine().equals(dst.topLine()))
                                                                                if isStrict && !self.get_topLine().equals(dst.get_topLine())
                                                            {
                                                               // Java: return false;
                                                               return false;
                                                            }
                                                            // Java: if (!isStrict && !this.topLine().equalsLax(dst.topLine()))
                                                                                if !isStrict && !self.get_topLine().equalsLax(dst.get_topLine())
                                                            {
                                                               // Java: return false;
                                                               return false;
                                                            }
                                                         }
                                                         // Java: if ((this.hasTopLeft2BottomRightLine() || !dst.hasTopLeft2BottomRightLine()) && (!this.hasTopLeft2BottomRightLine() || dst.hasTopLeft2BottomRightLine()))
                                                                            if (self.hasTopLeft2BottomRightLine() || !dst.hasTopLeft2BottomRightLine()) && (!self.hasTopLeft2BottomRightLine() || dst.hasTopLeft2BottomRightLine())
                                                         {
                                                            // Java: if (this.hasTopLeft2BottomRightLine() && dst.hasTopLeft2BottomRightLine())
                                                                                if self.hasTopLeft2BottomRightLine() && dst.hasTopLeft2BottomRightLine()
                                                            {
                                                               // Java: if (isStrict && !this.topLeft2BottomRightLine().equals(dst.topLeft2BottomRightLine()))
                                                                                    if isStrict && !self.get_topLeft2BottomRightLine().equals(dst.get_topLeft2BottomRightLine())
                                                               {
                                                                  // Java: return false;
                                                                  return false;
                                                               }
                                                               // Java: if (!isStrict && !this.topLeft2BottomRightLine().equalsLax(dst.topLeft2BottomRightLine()))
                                                                                    if !isStrict && !self.get_topLeft2BottomRightLine().equalsLax(dst.get_topLeft2BottomRightLine())
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

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
      // Java: int ret = 0;
      let mut ret: i32 = 0;
      // Java: if (this.hasAnchor())
        if self.hasAnchor()
      {
         // Java: ret += Integer.valueOf(this.anchor()).hashCode();
         ret = ret.wrapping_add(self.anchor());
      }
      // Java: if (this.hasAnchorCenter())
        if self.hasAnchorCenter()
      {
         // Java: ret += Boolean.valueOf(this.isAnchorCenter()).hashCode();
         ret = ret.wrapping_add(if self.isAnchorCenter() { 1231 } else { 1237 });
      }
      // Java: if (this.hasHorizontalOverflow())
        if self.hasHorizontalOverflow()
      {
         // Java: ret += Integer.valueOf(this.horizontalOverflow()).hashCode();
         ret = ret.wrapping_add(self.horizontalOverflow());
      }
      // Java: if (this.hasTextDirection())
        if self.hasTextDirection()
      {
         // Java: ret += Integer.valueOf(this.textDirection()).hashCode();
         ret = ret.wrapping_add(self.textDirection());
      }
      // Java: if (this.hasBottomMargin())
        if self.hasBottomMargin()
      {
         // Java: ret += Integer.valueOf(this.bottomMargin()).hashCode();
         ret = ret.wrapping_add(self.bottomMargin());
      }
      // Java: if (this.hasLeftMargin())
        if self.hasLeftMargin()
      {
         // Java: ret += Integer.valueOf(this.leftMargin()).hashCode();
         ret = ret.wrapping_add(self.leftMargin());
      }
      // Java: if (this.hasRightMargin())
        if self.hasRightMargin()
      {
         // Java: ret += Integer.valueOf(this.rightMargin()).hashCode();
         ret = ret.wrapping_add(self.rightMargin());
      }
      // Java: if (this.hasTopMargin())
        if self.hasTopMargin()
      {
         // Java: ret += Integer.valueOf(this.topMargin()).hashCode();
         ret = ret.wrapping_add(self.topMargin());
      }
      // Java: if (this.hasFill())
        if self.hasFill()
      {
         // Java: ret += this.fill().hashCode();
         ret = ret.wrapping_add(self.get_fill().hashCode());
      }
      // Java: if (this.hasCell3D())
        if self.hasCell3D()
      {
         // Java: ret += this.cell3D().hashCode();
         ret = ret.wrapping_add(self.get_cell3D().hashCode());
      }
      // Java: if (this.hasHeaders())
        if self.hasHeaders()
      {
         // Java: ret += this.headers().hashCode();
         ret = ret.wrapping_add(self.get_headers().hashCode());
      }
      // Java: if (this.hasBottomLine())
        if self.hasBottomLine()
      {
         // Java: ret += this.bottomLine().hashCode();
         ret = ret.wrapping_add(self.get_bottomLine().hashCode());
      }
      // Java: if (this.hasBottomLeft2TopRightLine())
        if self.hasBottomLeft2TopRightLine()
      {
         // Java: ret += this.bottomLeft2TopRightLine().hashCode();
         ret = ret.wrapping_add(self.get_bottomLeft2TopRightLine().hashCode());
      }
      // Java: if (this.hasLeftLine())
        if self.hasLeftLine()
      {
         // Java: ret += this.leftLine().hashCode();
         ret = ret.wrapping_add(self.get_leftLine().hashCode());
      }
      // Java: if (this.hasRightLine())
        if self.hasRightLine()
      {
         // Java: ret += this.rightLine().hashCode();
         ret = ret.wrapping_add(self.get_rightLine().hashCode());
      }
      // Java: if (this.hasTopLine())
        if self.hasTopLine()
      {
         // Java: ret += this.topLine().hashCode();
         ret = ret.wrapping_add(self.get_topLine().hashCode());
      }
      // Java: if (this.hasTopLeft2BottomRightLine())
        if self.hasTopLeft2BottomRightLine()
      {
         // Java: ret += this.topLeft2BottomRightLine().hashCode();
         ret = ret.wrapping_add(self.get_topLeft2BottomRightLine().hashCode());
      }
      // Java: return ret;
      ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
      // Java: if (this.fill != null && this.fill.dirty())
        if self.fill.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.cell3D != null && this.cell3D.dirty())
        else if self.cell3D.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.headers != null && this.headers.dirty())
        else if self.headers.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.bottomLine != null && this.bottomLine.dirty())
        else if self.bottomLine.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.bottomLeft2TopRightLine != null && this.bottomLeft2TopRightLine.dirty())
        else if self.bottomLeft2TopRightLine.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.leftLine != null && this.leftLine.dirty())
        else if self.leftLine.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.rightLine != null && this.rightLine.dirty())
        else if self.rightLine.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else if (this.topLine != null && this.topLine.dirty())
        else if self.topLine.as_ref().map_or(false, |s| s.dirty())
      {
         // Java: return true;
         return true;
      }
      // Java: else
        else
      {
         // Java: return this.topLeft2BottomRightLine != null && this.topLeft2BottomRightLine.dirty() ? true : this.map.dirty();
         if self.topLeft2BottomRightLine.as_ref().map_or(false, |s| s.dirty()) { true } else { self.map.dirty() }
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
      // Java: this.fill = null;
      self.fill = None;
      // Java: this.cell3D = null;
      self.cell3D = None;
      // Java: this.headers = null;
      self.headers = None;
      // Java: this.bottomLine = null;
      self.bottomLine = None;
      // Java: this.bottomLeft2TopRightLine = null;
      self.bottomLeft2TopRightLine = None;
      // Java: this.leftLine = null;
      self.leftLine = None;
      // Java: this.rightLine = null;
      self.rightLine = None;
      // Java: this.topLine = null;
      self.topLine = None;
      // Java: this.topLeft2BottomRightLine = null;
      self.topLeft2BottomRightLine = None;
    }

    // Java: void follow(TableCellProperties parent)
    pub fn follow(&mut self, parent: Option<Box<TableCellProperties>>) {
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

    // Java: TableCellProperties parent()
    pub fn get_parent(&self) -> Option<&TableCellProperties> {
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

    // Java: int anchor()
    pub fn anchor(&self) -> i32 {
      // Java: if (!this.map.has(1))
        if !self.map.has(1)
      {
         // Java: return this.parent == null ? 0 : this.parent.anchor();
         return self.parent.as_ref().map_or(0, |p| p.anchor());
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

    // Java: boolean hasAnchor()
    pub fn hasAnchor(&self) -> bool {
      // Java: return this.map.has(1) || this.parent != null && this.parent.hasAnchor();
      self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasAnchor())
    }

    // Java: void deleteAnchor()
    pub fn deleteAnchor(&mut self) {
      // Java: this.map.put(1, (KoBin)null);
      self.map.clear_key(1);
    }

    // Java: void putAnchor(int value)
    pub fn putAnchor(&mut self, value: i32) {
      // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
      self.map.put(1, Encode::from_int(value, self.zone));
    }

    // Java: boolean isAnchorCenter()
    pub fn isAnchorCenter(&self) -> bool {
      // Java: if (!this.map.has(2))
        if !self.map.has(2)
      {
         // Java: return this.parent == null ? false : this.parent.isAnchorCenter();
         return self.parent.as_ref().map_or(false, |p| p.isAnchorCenter());
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

    // Java: boolean hasAnchorCenter()
    pub fn hasAnchorCenter(&self) -> bool {
      // Java: return this.map.has(2) || this.parent != null && this.parent.hasAnchorCenter();
      self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasAnchorCenter())
    }

    // Java: void deleteAnchorCenter()
    pub fn deleteAnchorCenter(&mut self) {
      // Java: this.map.put(2, (KoBin)null);
      self.map.clear_key(2);
    }

    // Java: void putAnchorCenter(boolean value)
    pub fn putAnchorCenter(&mut self, value: bool) {
      // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
      self.map.put(2, Encode::from_bool(value, self.zone));
    }

    // Java: int horizontalOverflow()
    pub fn horizontalOverflow(&self) -> i32 {
      // Java: if (!this.map.has(3))
        if !self.map.has(3)
      {
         // Java: return this.parent == null ? 1 : this.parent.horizontalOverflow();
         return self.parent.as_ref().map_or(1, |p| p.horizontalOverflow());
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

    // Java: boolean hasHorizontalOverflow()
    pub fn hasHorizontalOverflow(&self) -> bool {
      // Java: return this.map.has(3) || this.parent != null && this.parent.hasHorizontalOverflow();
      self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasHorizontalOverflow())
    }

    // Java: void deleteHorizontalOverflow()
    pub fn deleteHorizontalOverflow(&mut self) {
      // Java: this.map.put(3, (KoBin)null);
      self.map.clear_key(3);
    }

    // Java: void putHorizontalOverflow(int value)
    pub fn putHorizontalOverflow(&mut self, value: i32) {
      // Java: this.map.put(3, KoBin.Encode.from(value, this.zone));
      self.map.put(3, Encode::from_int(value, self.zone));
    }

    // Java: int textDirection()
    pub fn textDirection(&self) -> i32 {
      // Java: if (!this.map.has(4))
        if !self.map.has(4)
      {
         // Java: return this.parent == null ? 0 : this.parent.textDirection();
         return self.parent.as_ref().map_or(0, |p| p.textDirection());
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

    // Java: boolean hasTextDirection()
    pub fn hasTextDirection(&self) -> bool {
      // Java: return this.map.has(4) || this.parent != null && this.parent.hasTextDirection();
      self.map.has(4) || self.parent.as_ref().map_or(false, |p| p.hasTextDirection())
    }

    // Java: void deleteTextDirection()
    pub fn deleteTextDirection(&mut self) {
      // Java: this.map.put(4, (KoBin)null);
      self.map.clear_key(4);
    }

    // Java: void putTextDirection(int value)
    pub fn putTextDirection(&mut self, value: i32) {
      // Java: this.map.put(4, KoBin.Encode.from(value, this.zone));
      self.map.put(4, Encode::from_int(value, self.zone));
    }

    // Java: int bottomMargin()
    pub fn bottomMargin(&self) -> i32 {
      // Java: if (!this.map.has(5))
        if !self.map.has(5)
      {
         // Java: return this.parent == null ? '늘' : this.parent.bottomMargin();
         return self.parent.as_ref().map_or(45720, |p| p.bottomMargin());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(5);
         let bin = self.map.byKey(5);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasBottomMargin()
    pub fn hasBottomMargin(&self) -> bool {
      // Java: return this.map.has(5) || this.parent != null && this.parent.hasBottomMargin();
      self.map.has(5) || self.parent.as_ref().map_or(false, |p| p.hasBottomMargin())
    }

    // Java: void deleteBottomMargin()
    pub fn deleteBottomMargin(&mut self) {
      // Java: this.map.put(5, (KoBin)null);
      self.map.clear_key(5);
    }

    // Java: void putBottomMargin(int value)
    pub fn putBottomMargin(&mut self, value: i32) {
      // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
      self.map.put(5, Encode::from_int(value, self.zone));
    }

    // Java: int leftMargin()
    pub fn leftMargin(&self) -> i32 {
      // Java: if (!this.map.has(6))
        if !self.map.has(6)
      {
         // Java: return this.parent == null ? 91440 : this.parent.leftMargin();
         return self.parent.as_ref().map_or(91440, |p| p.leftMargin());
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

    // Java: boolean hasLeftMargin()
    pub fn hasLeftMargin(&self) -> bool {
      // Java: return this.map.has(6) || this.parent != null && this.parent.hasLeftMargin();
      self.map.has(6) || self.parent.as_ref().map_or(false, |p| p.hasLeftMargin())
    }

    // Java: void deleteLeftMargin()
    pub fn deleteLeftMargin(&mut self) {
      // Java: this.map.put(6, (KoBin)null);
      self.map.clear_key(6);
    }

    // Java: void putLeftMargin(int value)
    pub fn putLeftMargin(&mut self, value: i32) {
      // Java: this.map.put(6, KoBin.Encode.from(value, this.zone));
      self.map.put(6, Encode::from_int(value, self.zone));
    }

    // Java: int rightMargin()
    pub fn rightMargin(&self) -> i32 {
      // Java: if (!this.map.has(7))
        if !self.map.has(7)
      {
         // Java: return this.parent == null ? 91440 : this.parent.rightMargin();
         return self.parent.as_ref().map_or(91440, |p| p.rightMargin());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(7);
         let bin = self.map.byKey(7);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasRightMargin()
    pub fn hasRightMargin(&self) -> bool {
      // Java: return this.map.has(7) || this.parent != null && this.parent.hasRightMargin();
      self.map.has(7) || self.parent.as_ref().map_or(false, |p| p.hasRightMargin())
    }

    // Java: void deleteRightMargin()
    pub fn deleteRightMargin(&mut self) {
      // Java: this.map.put(7, (KoBin)null);
      self.map.clear_key(7);
    }

    // Java: void putRightMargin(int value)
    pub fn putRightMargin(&mut self, value: i32) {
      // Java: this.map.put(7, KoBin.Encode.from(value, this.zone));
      self.map.put(7, Encode::from_int(value, self.zone));
    }

    // Java: int topMargin()
    pub fn topMargin(&self) -> i32 {
      // Java: if (!this.map.has(8))
        if !self.map.has(8)
      {
         // Java: return this.parent == null ? '늘' : this.parent.topMargin();
         return self.parent.as_ref().map_or(45720, |p| p.topMargin());
      }
      // Java: else
        else
      {
         // Java: KoBin bin = this.map.byKey(8);
         let bin = self.map.byKey(8);
         // Java: return KoBin.Decode.toInt(bin);
         return Decode::toInt(bin.as_ref());
      }
    }

    // Java: boolean hasTopMargin()
    pub fn hasTopMargin(&self) -> bool {
      // Java: return this.map.has(8) || this.parent != null && this.parent.hasTopMargin();
      self.map.has(8) || self.parent.as_ref().map_or(false, |p| p.hasTopMargin())
    }

    // Java: void deleteTopMargin()
    pub fn deleteTopMargin(&mut self) {
      // Java: this.map.put(8, (KoBin)null);
      self.map.clear_key(8);
    }

    // Java: void putTopMargin(int value)
    pub fn putTopMargin(&mut self, value: i32) {
      // Java: this.map.put(8, KoBin.Encode.from(value, this.zone));
      self.map.put(8, Encode::from_int(value, self.zone));
    }

    // Java: Fill fill()
    pub fn get_fill(&self) -> &Fill {
      // Java: if (this.fill == null)
        if self.fill.is_none()
      {
         // Java: if (this.map.has(9))
            if self.map.has(9)
         {
            // Java: KoBin bin = this.map.byKey(9);
            // Java: this.fill = (Fill)KoBin.Decode.toAny(Fill.class, bin, this.zone);
            todo!("lazy init fill from map");
         }
         // Java: else if (this.parent != null && this.parent.hasFill())
            else if self.parent.as_ref().map_or(false, |p| p.hasFill())
         {
            // Java: this.fill = this.parent.fill();
            todo!("copy fill from parent");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.fill = (Fill)KoBin.Decode.toAny(Fill.class, bin, this.zone);
            todo!("default init fill");
         }
      }
      // Java: return this.fill;
      self.fill.as_deref().unwrap()
    }

    // Java: boolean hasFill()
    pub fn hasFill(&self) -> bool {
      // Java: return this.map.has(9) || this.fill != null && this.fill.dirty() || this.parent != null && this.parent.hasFill();
      self.map.has(9) || self.fill.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasFill())
    }

    // Java: void deleteFill()
    pub fn deleteFill(&mut self) {
      // Java: this.map.put(9, (KoBin)null);
      self.map.clear_key(9);
      // Java: if (this.fill != null)
        if self.fill.is_some()
      {
         // Java: this.fill = null;
         self.fill = None;
      }
    }

    // Java: void writeFill(Fill value)
    pub fn writeFill(&mut self, value: Option<&Fill>) {
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
            // Java: if (this.fill != null)
                if self.fill.is_some()
            {
               // Java: this.fill = null;
               self.fill = None;
            }
         }
      }
    }

    // Java: void forceWriteFill(Fill value)
    pub fn forceWriteFill(&mut self, value: Option<&Fill>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(9, value.lastBin(this.zone));
         self.map.put(9, v.lastBin(self.zone));
         // Java: if (this.fill != null)
            if self.fill.is_some()
         {
            // Java: this.fill = null;
            self.fill = None;
         }
      }
    }

    // Java: Cell3D cell3D()
    pub fn get_cell3D(&self) -> &Cell3D {
      // Java: if (this.cell3D == null)
        if self.cell3D.is_none()
      {
         // Java: if (this.map.has(10))
            if self.map.has(10)
         {
            // Java: KoBin bin = this.map.byKey(10);
            // Java: this.cell3D = (Cell3D)KoBin.Decode.toAny(Cell3D.class, bin, this.zone);
            todo!("lazy init cell3D from map");
         }
         // Java: else if (this.parent != null && this.parent.hasCell3D())
            else if self.parent.as_ref().map_or(false, |p| p.hasCell3D())
         {
            // Java: this.cell3D = this.parent.cell3D();
            todo!("copy cell3D from parent");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.cell3D = (Cell3D)KoBin.Decode.toAny(Cell3D.class, bin, this.zone);
            todo!("default init cell3D");
         }
      }
      // Java: return this.cell3D;
      self.cell3D.as_deref().unwrap()
    }

    // Java: boolean hasCell3D()
    pub fn hasCell3D(&self) -> bool {
      // Java: return this.map.has(10) || this.cell3D != null && this.cell3D.dirty() || this.parent != null && this.parent.hasCell3D();
      self.map.has(10) || self.cell3D.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasCell3D())
    }

    // Java: void deleteCell3D()
    pub fn deleteCell3D(&mut self) {
      // Java: this.map.put(10, (KoBin)null);
      self.map.clear_key(10);
      // Java: if (this.cell3D != null)
        if self.cell3D.is_some()
      {
         // Java: this.cell3D = null;
         self.cell3D = None;
      }
    }

    // Java: void writeCell3D(Cell3D value)
    pub fn writeCell3D(&mut self, value: Option<&Cell3D>) {
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
            // Java: if (this.cell3D != null)
                if self.cell3D.is_some()
            {
               // Java: this.cell3D = null;
               self.cell3D = None;
            }
         }
      }
    }

    // Java: void forceWriteCell3D(Cell3D value)
    pub fn forceWriteCell3D(&mut self, value: Option<&Cell3D>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(10, value.lastBin(this.zone));
         self.map.put(10, v.lastBin(self.zone));
         // Java: if (this.cell3D != null)
            if self.cell3D.is_some()
         {
            // Java: this.cell3D = null;
            self.cell3D = None;
         }
      }
    }

    // Java: Headers headers()
    pub fn get_headers(&self) -> &Headers {
      // Java: if (this.headers == null)
        if self.headers.is_none()
      {
         // Java: if (this.map.has(11))
            if self.map.has(11)
         {
            // Java: KoBin bin = this.map.byKey(11);
            // Java: this.headers = (Headers)KoBin.Decode.toAny(Headers.class, bin, this.zone);
            todo!("lazy init headers from map");
         }
         // Java: else if (this.parent != null && this.parent.hasHeaders())
            else if self.parent.as_ref().map_or(false, |p| p.hasHeaders())
         {
            // Java: this.headers = this.parent.headers();
            todo!("copy headers from parent");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.headers = (Headers)KoBin.Decode.toAny(Headers.class, bin, this.zone);
            todo!("default init headers");
         }
      }
      // Java: return this.headers;
      self.headers.as_deref().unwrap()
    }

    // Java: boolean hasHeaders()
    pub fn hasHeaders(&self) -> bool {
      // Java: return this.map.has(11) || this.headers != null && this.headers.dirty() || this.parent != null && this.parent.hasHeaders();
      self.map.has(11) || self.headers.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasHeaders())
    }

    // Java: void deleteHeaders()
    pub fn deleteHeaders(&mut self) {
      // Java: this.map.put(11, (KoBin)null);
      self.map.clear_key(11);
      // Java: if (this.headers != null)
        if self.headers.is_some()
      {
         // Java: this.headers = null;
         self.headers = None;
      }
    }

    // Java: void writeHeaders(Headers value)
    pub fn writeHeaders(&mut self, value: Option<&Headers>) {
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
            // Java: if (this.headers != null)
                if self.headers.is_some()
            {
               // Java: this.headers = null;
               self.headers = None;
            }
         }
      }
    }

    // Java: void forceWriteHeaders(Headers value)
    pub fn forceWriteHeaders(&mut self, value: Option<&Headers>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(11, value.lastBin(this.zone));
         self.map.put(11, v.lastBin(self.zone));
         // Java: if (this.headers != null)
            if self.headers.is_some()
         {
            // Java: this.headers = null;
            self.headers = None;
         }
      }
    }

    // Java: Outline bottomLine()
    pub fn get_bottomLine(&self) -> &Outline {
      // Java: if (this.bottomLine == null)
        if self.bottomLine.is_none()
      {
         // Java: if (this.map.has(12))
            if self.map.has(12)
         {
            // Java: KoBin bin = this.map.byKey(12);
            // Java: this.bottomLine = (Outline)KoBin.Decode.toAny(Outline.class, bin, this.zone);
            todo!("lazy init bottomLine from map");
         }
         // Java: else if (this.parent != null && this.parent.hasBottomLine())
            else if self.parent.as_ref().map_or(false, |p| p.hasBottomLine())
         {
            // Java: this.bottomLine = this.parent.bottomLine();
            todo!("copy bottomLine from parent");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.bottomLine = (Outline)KoBin.Decode.toAny(Outline.class, bin, this.zone);
            todo!("default init bottomLine");
         }
      }
      // Java: return this.bottomLine;
      self.bottomLine.as_deref().unwrap()
    }

    // Java: boolean hasBottomLine()
    pub fn hasBottomLine(&self) -> bool {
      // Java: return this.map.has(12) || this.bottomLine != null && this.bottomLine.dirty() || this.parent != null && this.parent.hasBottomLine();
      self.map.has(12) || self.bottomLine.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasBottomLine())
    }

    // Java: void deleteBottomLine()
    pub fn deleteBottomLine(&mut self) {
      // Java: this.map.put(12, (KoBin)null);
      self.map.clear_key(12);
      // Java: if (this.bottomLine != null)
        if self.bottomLine.is_some()
      {
         // Java: this.bottomLine = null;
         self.bottomLine = None;
      }
    }

    // Java: void writeBottomLine(Outline value)
    pub fn writeBottomLine(&mut self, value: Option<&Outline>) {
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
            // Java: if (this.bottomLine != null)
                if self.bottomLine.is_some()
            {
               // Java: this.bottomLine = null;
               self.bottomLine = None;
            }
         }
      }
    }

    // Java: void forceWriteBottomLine(Outline value)
    pub fn forceWriteBottomLine(&mut self, value: Option<&Outline>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(12, value.lastBin(this.zone));
         self.map.put(12, v.lastBin(self.zone));
         // Java: if (this.bottomLine != null)
            if self.bottomLine.is_some()
         {
            // Java: this.bottomLine = null;
            self.bottomLine = None;
         }
      }
    }

    // Java: Outline bottomLeft2TopRightLine()
    pub fn get_bottomLeft2TopRightLine(&self) -> &Outline {
      // Java: if (this.bottomLeft2TopRightLine == null)
        if self.bottomLeft2TopRightLine.is_none()
      {
         // Java: if (this.map.has(13))
            if self.map.has(13)
         {
            // Java: KoBin bin = this.map.byKey(13);
            // Java: this.bottomLeft2TopRightLine = (Outline)KoBin.Decode.toAny(Outline.class, bin, this.zone);
            todo!("lazy init bottomLeft2TopRightLine from map");
         }
         // Java: else if (this.parent != null && this.parent.hasBottomLeft2TopRightLine())
            else if self.parent.as_ref().map_or(false, |p| p.hasBottomLeft2TopRightLine())
         {
            // Java: this.bottomLeft2TopRightLine = this.parent.bottomLeft2TopRightLine();
            todo!("copy bottomLeft2TopRightLine from parent");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.bottomLeft2TopRightLine = (Outline)KoBin.Decode.toAny(Outline.class, bin, this.zone);
            todo!("default init bottomLeft2TopRightLine");
         }
      }
      // Java: return this.bottomLeft2TopRightLine;
      self.bottomLeft2TopRightLine.as_deref().unwrap()
    }

    // Java: boolean hasBottomLeft2TopRightLine()
    pub fn hasBottomLeft2TopRightLine(&self) -> bool {
      // Java: return this.map.has(13) || this.bottomLeft2TopRightLine != null && this.bottomLeft2TopRightLine.dirty() || this.parent != null && this.parent.hasBottomLeft2TopRightLine();
      self.map.has(13) || self.bottomLeft2TopRightLine.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasBottomLeft2TopRightLine())
    }

    // Java: void deleteBottomLeft2TopRightLine()
    pub fn deleteBottomLeft2TopRightLine(&mut self) {
      // Java: this.map.put(13, (KoBin)null);
      self.map.clear_key(13);
      // Java: if (this.bottomLeft2TopRightLine != null)
        if self.bottomLeft2TopRightLine.is_some()
      {
         // Java: this.bottomLeft2TopRightLine = null;
         self.bottomLeft2TopRightLine = None;
      }
    }

    // Java: void writeBottomLeft2TopRightLine(Outline value)
    pub fn writeBottomLeft2TopRightLine(&mut self, value: Option<&Outline>) {
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
            // Java: if (this.bottomLeft2TopRightLine != null)
                if self.bottomLeft2TopRightLine.is_some()
            {
               // Java: this.bottomLeft2TopRightLine = null;
               self.bottomLeft2TopRightLine = None;
            }
         }
      }
    }

    // Java: void forceWriteBottomLeft2TopRightLine(Outline value)
    pub fn forceWriteBottomLeft2TopRightLine(&mut self, value: Option<&Outline>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(13, value.lastBin(this.zone));
         self.map.put(13, v.lastBin(self.zone));
         // Java: if (this.bottomLeft2TopRightLine != null)
            if self.bottomLeft2TopRightLine.is_some()
         {
            // Java: this.bottomLeft2TopRightLine = null;
            self.bottomLeft2TopRightLine = None;
         }
      }
    }

    // Java: Outline leftLine()
    pub fn get_leftLine(&self) -> &Outline {
      // Java: if (this.leftLine == null)
        if self.leftLine.is_none()
      {
         // Java: if (this.map.has(14))
            if self.map.has(14)
         {
            // Java: KoBin bin = this.map.byKey(14);
            // Java: this.leftLine = (Outline)KoBin.Decode.toAny(Outline.class, bin, this.zone);
            todo!("lazy init leftLine from map");
         }
         // Java: else if (this.parent != null && this.parent.hasLeftLine())
            else if self.parent.as_ref().map_or(false, |p| p.hasLeftLine())
         {
            // Java: this.leftLine = this.parent.leftLine();
            todo!("copy leftLine from parent");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.leftLine = (Outline)KoBin.Decode.toAny(Outline.class, bin, this.zone);
            todo!("default init leftLine");
         }
      }
      // Java: return this.leftLine;
      self.leftLine.as_deref().unwrap()
    }

    // Java: boolean hasLeftLine()
    pub fn hasLeftLine(&self) -> bool {
      // Java: return this.map.has(14) || this.leftLine != null && this.leftLine.dirty() || this.parent != null && this.parent.hasLeftLine();
      self.map.has(14) || self.leftLine.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasLeftLine())
    }

    // Java: void deleteLeftLine()
    pub fn deleteLeftLine(&mut self) {
      // Java: this.map.put(14, (KoBin)null);
      self.map.clear_key(14);
      // Java: if (this.leftLine != null)
        if self.leftLine.is_some()
      {
         // Java: this.leftLine = null;
         self.leftLine = None;
      }
    }

    // Java: void writeLeftLine(Outline value)
    pub fn writeLeftLine(&mut self, value: Option<&Outline>) {
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
            // Java: if (this.leftLine != null)
                if self.leftLine.is_some()
            {
               // Java: this.leftLine = null;
               self.leftLine = None;
            }
         }
      }
    }

    // Java: void forceWriteLeftLine(Outline value)
    pub fn forceWriteLeftLine(&mut self, value: Option<&Outline>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(14, value.lastBin(this.zone));
         self.map.put(14, v.lastBin(self.zone));
         // Java: if (this.leftLine != null)
            if self.leftLine.is_some()
         {
            // Java: this.leftLine = null;
            self.leftLine = None;
         }
      }
    }

    // Java: Outline rightLine()
    pub fn get_rightLine(&self) -> &Outline {
      // Java: if (this.rightLine == null)
        if self.rightLine.is_none()
      {
         // Java: if (this.map.has(15))
            if self.map.has(15)
         {
            // Java: KoBin bin = this.map.byKey(15);
            // Java: this.rightLine = (Outline)KoBin.Decode.toAny(Outline.class, bin, this.zone);
            todo!("lazy init rightLine from map");
         }
         // Java: else if (this.parent != null && this.parent.hasRightLine())
            else if self.parent.as_ref().map_or(false, |p| p.hasRightLine())
         {
            // Java: this.rightLine = this.parent.rightLine();
            todo!("copy rightLine from parent");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.rightLine = (Outline)KoBin.Decode.toAny(Outline.class, bin, this.zone);
            todo!("default init rightLine");
         }
      }
      // Java: return this.rightLine;
      self.rightLine.as_deref().unwrap()
    }

    // Java: boolean hasRightLine()
    pub fn hasRightLine(&self) -> bool {
      // Java: return this.map.has(15) || this.rightLine != null && this.rightLine.dirty() || this.parent != null && this.parent.hasRightLine();
      self.map.has(15) || self.rightLine.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasRightLine())
    }

    // Java: void deleteRightLine()
    pub fn deleteRightLine(&mut self) {
      // Java: this.map.put(15, (KoBin)null);
      self.map.clear_key(15);
      // Java: if (this.rightLine != null)
        if self.rightLine.is_some()
      {
         // Java: this.rightLine = null;
         self.rightLine = None;
      }
    }

    // Java: void writeRightLine(Outline value)
    pub fn writeRightLine(&mut self, value: Option<&Outline>) {
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
            // Java: if (this.rightLine != null)
                if self.rightLine.is_some()
            {
               // Java: this.rightLine = null;
               self.rightLine = None;
            }
         }
      }
    }

    // Java: void forceWriteRightLine(Outline value)
    pub fn forceWriteRightLine(&mut self, value: Option<&Outline>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(15, value.lastBin(this.zone));
         self.map.put(15, v.lastBin(self.zone));
         // Java: if (this.rightLine != null)
            if self.rightLine.is_some()
         {
            // Java: this.rightLine = null;
            self.rightLine = None;
         }
      }
    }

    // Java: Outline topLine()
    pub fn get_topLine(&self) -> &Outline {
      // Java: if (this.topLine == null)
        if self.topLine.is_none()
      {
         // Java: if (this.map.has(16))
            if self.map.has(16)
         {
            // Java: KoBin bin = this.map.byKey(16);
            // Java: this.topLine = (Outline)KoBin.Decode.toAny(Outline.class, bin, this.zone);
            todo!("lazy init topLine from map");
         }
         // Java: else if (this.parent != null && this.parent.hasTopLine())
            else if self.parent.as_ref().map_or(false, |p| p.hasTopLine())
         {
            // Java: this.topLine = this.parent.topLine();
            todo!("copy topLine from parent");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.topLine = (Outline)KoBin.Decode.toAny(Outline.class, bin, this.zone);
            todo!("default init topLine");
         }
      }
      // Java: return this.topLine;
      self.topLine.as_deref().unwrap()
    }

    // Java: boolean hasTopLine()
    pub fn hasTopLine(&self) -> bool {
      // Java: return this.map.has(16) || this.topLine != null && this.topLine.dirty() || this.parent != null && this.parent.hasTopLine();
      self.map.has(16) || self.topLine.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasTopLine())
    }

    // Java: void deleteTopLine()
    pub fn deleteTopLine(&mut self) {
      // Java: this.map.put(16, (KoBin)null);
      self.map.clear_key(16);
      // Java: if (this.topLine != null)
        if self.topLine.is_some()
      {
         // Java: this.topLine = null;
         self.topLine = None;
      }
    }

    // Java: void writeTopLine(Outline value)
    pub fn writeTopLine(&mut self, value: Option<&Outline>) {
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
            // Java: if (this.topLine != null)
                if self.topLine.is_some()
            {
               // Java: this.topLine = null;
               self.topLine = None;
            }
         }
      }
    }

    // Java: void forceWriteTopLine(Outline value)
    pub fn forceWriteTopLine(&mut self, value: Option<&Outline>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(16, value.lastBin(this.zone));
         self.map.put(16, v.lastBin(self.zone));
         // Java: if (this.topLine != null)
            if self.topLine.is_some()
         {
            // Java: this.topLine = null;
            self.topLine = None;
         }
      }
    }

    // Java: Outline topLeft2BottomRightLine()
    pub fn get_topLeft2BottomRightLine(&self) -> &Outline {
      // Java: if (this.topLeft2BottomRightLine == null)
        if self.topLeft2BottomRightLine.is_none()
      {
         // Java: if (this.map.has(17))
            if self.map.has(17)
         {
            // Java: KoBin bin = this.map.byKey(17);
            // Java: this.topLeft2BottomRightLine = (Outline)KoBin.Decode.toAny(Outline.class, bin, this.zone);
            todo!("lazy init topLeft2BottomRightLine from map");
         }
         // Java: else if (this.parent != null && this.parent.hasTopLeft2BottomRightLine())
            else if self.parent.as_ref().map_or(false, |p| p.hasTopLeft2BottomRightLine())
         {
            // Java: this.topLeft2BottomRightLine = this.parent.topLeft2BottomRightLine();
            todo!("copy topLeft2BottomRightLine from parent");
         }
         // Java: else
            else
         {
            // Java: KoBin bin = KoBin.Null.value1();
            // Java: this.topLeft2BottomRightLine = (Outline)KoBin.Decode.toAny(Outline.class, bin, this.zone);
            todo!("default init topLeft2BottomRightLine");
         }
      }
      // Java: return this.topLeft2BottomRightLine;
      self.topLeft2BottomRightLine.as_deref().unwrap()
    }

    // Java: boolean hasTopLeft2BottomRightLine()
    pub fn hasTopLeft2BottomRightLine(&self) -> bool {
      // Java: return this.map.has(17) || this.topLeft2BottomRightLine != null && this.topLeft2BottomRightLine.dirty() || this.parent != null && this.parent.hasTopLeft2BottomRightLine();
      self.map.has(17) || self.topLeft2BottomRightLine.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasTopLeft2BottomRightLine())
    }

    // Java: void deleteTopLeft2BottomRightLine()
    pub fn deleteTopLeft2BottomRightLine(&mut self) {
      // Java: this.map.put(17, (KoBin)null);
      self.map.clear_key(17);
      // Java: if (this.topLeft2BottomRightLine != null)
        if self.topLeft2BottomRightLine.is_some()
      {
         // Java: this.topLeft2BottomRightLine = null;
         self.topLeft2BottomRightLine = None;
      }
    }

    // Java: void writeTopLeft2BottomRightLine(Outline value)
    pub fn writeTopLeft2BottomRightLine(&mut self, value: Option<&Outline>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: KoBin bin = value.lastBin(this.zone);
         let bin = v.lastBin(self.zone);
         // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref()))
         {
            // Java: this.map.put(17, bin);
            self.map.put(17, bin);
            // Java: if (this.topLeft2BottomRightLine != null)
                if self.topLeft2BottomRightLine.is_some()
            {
               // Java: this.topLeft2BottomRightLine = null;
               self.topLeft2BottomRightLine = None;
            }
         }
      }
    }

    // Java: void forceWriteTopLeft2BottomRightLine(Outline value)
    pub fn forceWriteTopLeft2BottomRightLine(&mut self, value: Option<&Outline>) {
      // Java: if (value != null)
        if let Some(v) = value
      {
         // Java: this.map.put(17, value.lastBin(this.zone));
         self.map.put(17, v.lastBin(self.zone));
         // Java: if (this.topLeft2BottomRightLine != null)
            if self.topLeft2BottomRightLine.is_some()
         {
            // Java: this.topLeft2BottomRightLine = null;
            self.topLeft2BottomRightLine = None;
         }
      }
    }

}

impl KoAr for TableCellProperties {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin_with_zone(var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn dirty(&self) -> bool { TableCellProperties::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { TableCellProperties::lastBin(self, var1) }
}

impl KoClosable for TableCellProperties {
    fn close(&mut self) { TableCellProperties::close(self); }
}
