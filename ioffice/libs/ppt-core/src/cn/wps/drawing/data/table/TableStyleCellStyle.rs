#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

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

use crate::cn::wps::drawing::data::table::ThemeableFillStyle::ThemeableFillStyle;
use crate::cn::wps::drawing::data::table::ThemeableLineStyle::ThemeableLineStyle;

// Java: public class TableStyleCellStyle implements KoAr, KoClosable
pub struct TableStyleCellStyle {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Cell3D cell3D
    pub(crate) cell3D: Option<Box<Cell3D>>,
    // Java field: TableCellBorderStyle tableCellBorders
    pub(crate) tableCellBorders: Option<Box<TableCellBorderStyle>>,
    // Java field: ThemeableFillStyle fillStyle
    pub(crate) fillStyle: Option<Box<ThemeableFillStyle>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: TableStyleCellStyle parent
    pub(crate) parent: Option<Box<TableStyleCellStyle>>,
}

// Java: public class TableCellBorderStyle
pub struct TableCellBorderStyle {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: ThemeableLineStyle bottom
    pub(crate) bottom: Option<Box<ThemeableLineStyle>>,
    // Java field: ThemeableLineStyle insideHorizontal
    pub(crate) insideHorizontal: Option<Box<ThemeableLineStyle>>,
    // Java field: ThemeableLineStyle insideVertical
    pub(crate) insideVertical: Option<Box<ThemeableLineStyle>>,
    // Java field: ThemeableLineStyle left
    pub(crate) left: Option<Box<ThemeableLineStyle>>,
    // Java field: ThemeableLineStyle right
    pub(crate) right: Option<Box<ThemeableLineStyle>>,
    // Java field: ThemeableLineStyle topLeft2BottomRight
    pub(crate) topLeft2BottomRight: Option<Box<ThemeableLineStyle>>,
    // Java field: ThemeableLineStyle top
    pub(crate) top: Option<Box<ThemeableLineStyle>>,
    // Java field: ThemeableLineStyle topRight2BottomLeft
    pub(crate) topRight2BottomLeft: Option<Box<ThemeableLineStyle>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: TableCellBorderStyle parent
    pub(crate) parent: Option<Box<TableCellBorderStyle>>,
}

const TSCS_CELL3D: i32 = 1;
const TSCS_TABLE_CELL_BORDERS: i32 = 2;
const TSCS_FILL_STYLE: i32 = 3;

const TCBS_BOTTOM: i32 = 1;
const TCBS_INSIDE_HORIZONTAL: i32 = 2;
const TCBS_INSIDE_VERTICAL: i32 = 3;
const TCBS_LEFT: i32 = 4;
const TCBS_RIGHT: i32 = 5;
const TCBS_TOP_LEFT2_BOTTOM_RIGHT: i32 = 6;
const TCBS_TOP: i32 = 7;
const TCBS_TOP_RIGHT2_BOTTOM_LEFT: i32 = 8;

// Java: public class TableStyleCellStyle implements KoAr, KoClosable
impl TableStyleCellStyle {
    // Java: static protected static <Value1 extends TableStyleCellStyle> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = TableStyleCellStyle::TableStyleCellStyle();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static TableStyleCellStyle create()
    pub fn create() -> Self {
        // Java: return initialize(TableStyleCellStyle.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static TableStyleCellStyle open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(TableStyleCellStyle.class, bin);
        Self::initialize(bin)
    }

    // Java: TableStyleCellStyle()
    pub fn TableStyleCellStyle() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        TableStyleCellStyle {
            zone: KoZone::defaultZone(),
            cell3D: None,
            tableCellBorders: None,
            fillStyle: None,
            bin_box: KpBinBox::new(),
            map: KpMap::new(),
            parent: None,
        }
    }

    // Java: TableStyleCellStyle(KoBin bin)
    pub fn TableStyleCellStyle_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        TableStyleCellStyle {
            zone: KoZone::defaultZone(),
            cell3D: None,
            tableCellBorders: None,
            fillStyle: None,
            bin_box: KpBinBox::KpBinBox_from(Some(bin)),
            map: KpMap::KpMap_from(bin),
            parent: None,
        }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
        // Java: this.cleanCache();
        self.cleanCache();
        // Java: this.cleanFollow();
        self.cleanFollow();
        // Java: this.box.set(bin);
        self.bin_box.set(Some(bin));
        // Java: this.map.load(this.box.get());
        let b = self.bin_box.get();
        self.map.load(b.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
        // Java: this.loadBin(bin, this.zone);
        self.loadBin(bin, self.zone);
    }

    // Java: void load(TableStyleCellStyle value)
    pub fn load_from(&mut self, value: Option<&TableStyleCellStyle>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(self.zone);
            self.loadBin(last.as_ref(), self.zone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.cell3D != null && this.cell3D.dirty())
        if let Some(ref mut child) = self.cell3D {
            if child.dirty() {
                // Java: this.map.put(1, this.cell3D.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(TSCS_CELL3D, bin);
            }
        }
        // Java: if (this.tableCellBorders != null && this.tableCellBorders.dirty())
        if let Some(ref mut child) = self.tableCellBorders {
            if child.dirty() {
                // Java: this.map.put(2, this.tableCellBorders.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(TSCS_TABLE_CELL_BORDERS, bin);
            }
        }
        // Java: if (this.fillStyle != null && this.fillStyle.dirty())
        if let Some(ref mut child) = self.fillStyle {
            if child.dirty() {
                // Java: this.map.put(3, this.fillStyle.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(TSCS_FILL_STYLE, bin);
            }
        }
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            saved
        } else {
            self.bin_box.get()
        }
    }

    // Java: boolean equalsLax(TableStyleCellStyle o)
    pub fn equalsLax(&self, o: &TableStyleCellStyle) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TableStyleCellStyle) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &TableStyleCellStyle, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof TableStyleCellStyle)
        if true {
            // Java: TableStyleCellStyle dst = (TableStyleCellStyle)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst) {
                // Java: return true;
                return true;
            }
            // Java: else if ((this.hasCell3D() || !dst.hasCell3D()) && (!this.hasCell3D() || dst.hasCell3D()))
            else if (self.hasCell3D() || !dst.hasCell3D()) && (!self.hasCell3D() || dst.hasCell3D()) {
                // Java: if (this.hasCell3D() && dst.hasCell3D())
                if self.hasCell3D() && dst.hasCell3D() {
                    // Java: if (isStrict && !this.cell3D().equals(dst.cell3D()))
                    if isStrict {
                        // Java: return false;
                    }
                    // Java: if (!isStrict && !this.cell3D().equalsLax(dst.cell3D()))
                    if !isStrict {
                        // Java: return false;
                    }
                }
                // Java: if ((this.hasTableCellBorders() || !dst.hasTableCellBorders()) && (!this.hasTableCellBorders() || dst.hasTableCellBorders()))
                if (self.hasTableCellBorders() || !dst.hasTableCellBorders()) && (!self.hasTableCellBorders() || dst.hasTableCellBorders()) {
                    // Java: if (this.hasTableCellBorders() && dst.hasTableCellBorders())
                    if self.hasTableCellBorders() && dst.hasTableCellBorders() {
                        // Java: if (isStrict && !this.tableCellBorders().equals(dst.tableCellBorders()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.tableCellBorders().equalsLax(dst.tableCellBorders()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                    // Java: if ((this.hasFillStyle() || !dst.hasFillStyle()) && (!this.hasFillStyle() || dst.hasFillStyle()))
                    if (self.hasFillStyle() || !dst.hasFillStyle()) && (!self.hasFillStyle() || dst.hasFillStyle()) {
                        // Java: if (this.hasFillStyle() && dst.hasFillStyle())
                        if self.hasFillStyle() && dst.hasFillStyle() {
                            // Java: if (isStrict && !this.fillStyle().equals(dst.fillStyle()))
                            if isStrict {
                                // Java: return false;
                            }
                            // Java: if (!isStrict && !this.fillStyle().equalsLax(dst.fillStyle()))
                            if !isStrict {
                                // Java: return false;
                            }
                        }
                        // Java: return true;
                        return true;
                    }
                    // Java: else
                    else {
                        // Java: return false;
                        return false;
                    }
                }
                // Java: else
                else {
                    // Java: return false;
                    return false;
                }
            }
            // Java: else
            else {
                // Java: return false;
                return false;
            }
        }
        // Java: else
        else {
            // Java: return false;
            false
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int ret = 0;
        let mut ret: i32 = 0;
        // Java: if (this.hasCell3D())
        if self.hasCell3D() {
            // Java: ret += this.cell3D().hashCode();
        }
        // Java: if (this.hasTableCellBorders())
        if self.hasTableCellBorders() {
            // Java: ret += this.tableCellBorders().hashCode();
        }
        // Java: if (this.hasFillStyle())
        if self.hasFillStyle() {
            // Java: ret += this.fillStyle().hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: if (this.cell3D != null && this.cell3D.dirty())
        if self.cell3D.as_ref().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else if (this.tableCellBorders != null && this.tableCellBorders.dirty())
        else if self.tableCellBorders.as_ref().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: return this.fillStyle != null && this.fillStyle.dirty() ? true : this.map.dirty();
            if self.fillStyle.as_ref().map_or(false, |r| r.dirty()) { true } else { self.map.dirty() }
        }
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
        let n = Null::value1();
        self.map.load(n.as_ref());
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
        // Java: this.cell3D = null;
        self.cell3D = None;
        // Java: this.tableCellBorders = null;
        self.tableCellBorders = None;
        // Java: this.fillStyle = null;
        self.fillStyle = None;
    }

    // Java: void follow(TableStyleCellStyle parent)
    pub fn follow(&mut self, parent: &mut TableStyleCellStyle) {
        // Java: if (parent != null)
        if true {
            // Java: parent.discard();
            parent.discard();
            // Java: this.discard();
            self.discard();
            // Java: this.parent = parent;
        }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
        // Java: if (this.parent != null)
        if self.parent.is_some() {
            // Java: this.discard();
            self.discard();
            // Java: this.parent.discard();
            if let Some(p) = self.parent.as_mut() { p.discard(); }
            // Java: this.cleanParent();
            self.cleanParent();
        }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
        // Java: return this.parent != null;
        self.parent.is_some()
    }

    // Java: TableStyleCellStyle parent()
    pub fn parent_ref(&self) -> Option<&TableStyleCellStyle> {
        // Java: return this.parent;
        self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(self.zone);
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None;
    }

    // Java: Cell3D cell3D()
    pub fn cell3D(&self) -> &Cell3D {
        // Java: if (this.cell3D == null)
        if self.cell3D.is_none() {
            // Java: if (this.map.has(1))
            if self.map.has(TSCS_CELL3D) {
                // Java: KoBin bin = this.map.byKey(1);
                let _bin = self.map.byKey(TSCS_CELL3D);
                // Java: this.cell3D = (Cell3D)KoBin.Decode.toAny(Cell3D.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasCell3D())
            else if self.parent.as_ref().map_or(false, |p| p.hasCell3D()) {
                // Java: this.cell3D = this.parent.cell3D();
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.cell3D = (Cell3D)KoBin.Decode.toAny(Cell3D.class, bin, this.zone);
            }
        }
        // Java: return this.cell3D;
        self.cell3D.as_ref().unwrap()
    }

    // Java: boolean hasCell3D()
    pub fn hasCell3D(&self) -> bool {
        // Java: return this.map.has(1) || this.cell3D != null && this.cell3D.dirty() || this.parent != null && this.parent.hasCell3D();
        self.map.has(TSCS_CELL3D) || self.cell3D.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasCell3D())
    }

    // Java: void deleteCell3D()
    pub fn deleteCell3D(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(TSCS_CELL3D);
        // Java: if (this.cell3D != null)
        if self.cell3D.is_some() {
            // Java: this.cell3D = null;
            self.cell3D = None;
        }
    }

    // Java: void writeCell3D(Cell3D value)
    pub fn writeCell3D(&mut self, value: &Cell3D) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(1, bin);
                self.map.put(TSCS_CELL3D, bin);
                // Java: if (this.cell3D != null)
                if self.cell3D.is_some() {
                    // Java: this.cell3D = null;
                    self.cell3D = None;
                }
            }
        }
    }

    // Java: void forceWriteCell3D(Cell3D value)
    pub fn forceWriteCell3D(&mut self, value: &Cell3D) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(1, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(TSCS_CELL3D, bin);
            // Java: if (this.cell3D != null)
            if self.cell3D.is_some() {
                // Java: this.cell3D = null;
                self.cell3D = None;
            }
        }
    }

    // Java: TableCellBorderStyle tableCellBorders()
    pub fn tableCellBorders(&self) -> &TableCellBorderStyle {
        // Java: if (this.tableCellBorders == null)
        if self.tableCellBorders.is_none() {
            // Java: if (this.map.has(2))
            if self.map.has(TSCS_TABLE_CELL_BORDERS) {
                // Java: KoBin bin = this.map.byKey(2);
                let _bin = self.map.byKey(TSCS_TABLE_CELL_BORDERS);
                // Java: this.tableCellBorders = (TableCellBorderStyle)KoBin.Decode.toAny(TableCellBorderStyle.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasTableCellBorders())
            else if self.parent.as_ref().map_or(false, |p| p.hasTableCellBorders()) {
                // Java: this.tableCellBorders = this.parent.tableCellBorders();
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.tableCellBorders = (TableCellBorderStyle)KoBin.Decode.toAny(TableCellBorderStyle.class, bin, this.zone);
            }
        }
        // Java: return this.tableCellBorders;
        self.tableCellBorders.as_ref().unwrap()
    }

    // Java: boolean hasTableCellBorders()
    pub fn hasTableCellBorders(&self) -> bool {
        // Java: return this.map.has(2) || this.tableCellBorders != null && this.tableCellBorders.dirty() || this.parent != null && this.parent.hasTableCellBorders();
        self.map.has(TSCS_TABLE_CELL_BORDERS) || self.tableCellBorders.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasTableCellBorders())
    }

    // Java: void deleteTableCellBorders()
    pub fn deleteTableCellBorders(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(TSCS_TABLE_CELL_BORDERS);
        // Java: if (this.tableCellBorders != null)
        if self.tableCellBorders.is_some() {
            // Java: this.tableCellBorders = null;
            self.tableCellBorders = None;
        }
    }

    // Java: void writeTableCellBorders(TableCellBorderStyle value)
    pub fn writeTableCellBorders(&mut self, value: &TableCellBorderStyle) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(2, bin);
                self.map.put(TSCS_TABLE_CELL_BORDERS, bin);
                // Java: if (this.tableCellBorders != null)
                if self.tableCellBorders.is_some() {
                    // Java: this.tableCellBorders = null;
                    self.tableCellBorders = None;
                }
            }
        }
    }

    // Java: void forceWriteTableCellBorders(TableCellBorderStyle value)
    pub fn forceWriteTableCellBorders(&mut self, value: &TableCellBorderStyle) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(2, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(TSCS_TABLE_CELL_BORDERS, bin);
            // Java: if (this.tableCellBorders != null)
            if self.tableCellBorders.is_some() {
                // Java: this.tableCellBorders = null;
                self.tableCellBorders = None;
            }
        }
    }

    // Java: ThemeableFillStyle fillStyle()
    pub fn fillStyle(&self) -> &ThemeableFillStyle {
        // Java: if (this.fillStyle == null)
        if self.fillStyle.is_none() {
            // Java: if (this.map.has(3))
            if self.map.has(TSCS_FILL_STYLE) {
                // Java: KoBin bin = this.map.byKey(3);
                let _bin = self.map.byKey(TSCS_FILL_STYLE);
                // Java: this.fillStyle = (ThemeableFillStyle)KoBin.Decode.toAny(ThemeableFillStyle.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasFillStyle())
            else if self.parent.as_ref().map_or(false, |p| p.hasFillStyle()) {
                // Java: this.fillStyle = this.parent.fillStyle();
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.fillStyle = (ThemeableFillStyle)KoBin.Decode.toAny(ThemeableFillStyle.class, bin, this.zone);
            }
        }
        // Java: return this.fillStyle;
        self.fillStyle.as_ref().unwrap()
    }

    // Java: boolean hasFillStyle()
    pub fn hasFillStyle(&self) -> bool {
        // Java: return this.map.has(3) || this.fillStyle != null && this.fillStyle.dirty() || this.parent != null && this.parent.hasFillStyle();
        self.map.has(TSCS_FILL_STYLE) || self.fillStyle.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasFillStyle())
    }

    // Java: void deleteFillStyle()
    pub fn deleteFillStyle(&mut self) {
        // Java: this.map.put(3, (KoBin)null);
        self.map.clear_key(TSCS_FILL_STYLE);
        // Java: if (this.fillStyle != null)
        if self.fillStyle.is_some() {
            // Java: this.fillStyle = null;
            self.fillStyle = None;
        }
    }

    // Java: void writeFillStyle(ThemeableFillStyle value)
    pub fn writeFillStyle(&mut self, value: &ThemeableFillStyle) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(3, bin);
                self.map.put(TSCS_FILL_STYLE, bin);
                // Java: if (this.fillStyle != null)
                if self.fillStyle.is_some() {
                    // Java: this.fillStyle = null;
                    self.fillStyle = None;
                }
            }
        }
    }

    // Java: void forceWriteFillStyle(ThemeableFillStyle value)
    pub fn forceWriteFillStyle(&mut self, value: &ThemeableFillStyle) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(3, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(TSCS_FILL_STYLE, bin);
            // Java: if (this.fillStyle != null)
            if self.fillStyle.is_some() {
                // Java: this.fillStyle = null;
                self.fillStyle = None;
            }
        }
    }
}

// Java: public class TableCellBorderStyle
impl TableCellBorderStyle {
    // Java: static protected static <Value1 extends TableCellBorderStyle> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = TableCellBorderStyle::TableCellBorderStyle();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static TableCellBorderStyle create()
    pub fn create() -> Self {
        // Java: return initialize(TableCellBorderStyle.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static TableCellBorderStyle open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(TableCellBorderStyle.class, bin);
        Self::initialize(bin)
    }

    // Java: TableCellBorderStyle()
    pub fn TableCellBorderStyle() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        TableCellBorderStyle {
            zone: KoZone::defaultZone(),
            bottom: None, insideHorizontal: None, insideVertical: None,
            left: None, right: None, topLeft2BottomRight: None,
            top: None, topRight2BottomLeft: None,
            bin_box: KpBinBox::new(), map: KpMap::new(), parent: None,
        }
    }

    // Java: TableCellBorderStyle(KoBin bin)
    pub fn TableCellBorderStyle_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        TableCellBorderStyle {
            zone: KoZone::defaultZone(),
            bottom: None, insideHorizontal: None, insideVertical: None,
            left: None, right: None, topLeft2BottomRight: None,
            top: None, topRight2BottomLeft: None,
            bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None,
        }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
        // Java: this.cleanCache();
        self.cleanCache();
        // Java: this.cleanFollow();
        self.cleanFollow();
        // Java: this.box.set(bin);
        self.bin_box.set(Some(bin));
        // Java: this.map.load(this.box.get());
        let b = self.bin_box.get();
        self.map.load(b.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
        // Java: this.loadBin(bin, this.zone);
        self.loadBin(bin, self.zone);
    }

    // Java: void load(TableCellBorderStyle value)
    pub fn load_from(&mut self, value: Option<&TableCellBorderStyle>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(self.zone);
            self.loadBin(last.as_ref(), self.zone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.bottom != null && this.bottom.dirty())
        if let Some(ref mut child) = self.bottom {
            if child.dirty() {
                // Java: this.map.put(1, this.bottom.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(TCBS_BOTTOM, bin);
            }
        }
        // Java: if (this.insideHorizontal != null && this.insideHorizontal.dirty())
        if let Some(ref mut child) = self.insideHorizontal {
            if child.dirty() {
                // Java: this.map.put(2, this.insideHorizontal.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(TCBS_INSIDE_HORIZONTAL, bin);
            }
        }
        // Java: if (this.insideVertical != null && this.insideVertical.dirty())
        if let Some(ref mut child) = self.insideVertical {
            if child.dirty() {
                // Java: this.map.put(3, this.insideVertical.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(TCBS_INSIDE_VERTICAL, bin);
            }
        }
        // Java: if (this.left != null && this.left.dirty())
        if let Some(ref mut child) = self.left {
            if child.dirty() {
                // Java: this.map.put(4, this.left.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(TCBS_LEFT, bin);
            }
        }
        // Java: if (this.right != null && this.right.dirty())
        if let Some(ref mut child) = self.right {
            if child.dirty() {
                // Java: this.map.put(5, this.right.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(TCBS_RIGHT, bin);
            }
        }
        // Java: if (this.topLeft2BottomRight != null && this.topLeft2BottomRight.dirty())
        if let Some(ref mut child) = self.topLeft2BottomRight {
            if child.dirty() {
                // Java: this.map.put(6, this.topLeft2BottomRight.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(TCBS_TOP_LEFT2_BOTTOM_RIGHT, bin);
            }
        }
        // Java: if (this.top != null && this.top.dirty())
        if let Some(ref mut child) = self.top {
            if child.dirty() {
                // Java: this.map.put(7, this.top.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(TCBS_TOP, bin);
            }
        }
        // Java: if (this.topRight2BottomLeft != null && this.topRight2BottomLeft.dirty())
        if let Some(ref mut child) = self.topRight2BottomLeft {
            if child.dirty() {
                // Java: this.map.put(8, this.topRight2BottomLeft.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(TCBS_TOP_RIGHT2_BOTTOM_LEFT, bin);
            }
        }
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            saved
        } else {
            self.bin_box.get()
        }
    }

    // Java: boolean equalsLax(TableCellBorderStyle o)
    pub fn equalsLax(&self, o: &TableCellBorderStyle) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TableCellBorderStyle) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &TableCellBorderStyle, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof TableCellBorderStyle)
        if true {
            // Java: TableCellBorderStyle dst = (TableCellBorderStyle)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst) {
                // Java: return true;
                return true;
            }
            // Java: else if ((this.hasBottom() || !dst.hasBottom()) && (!this.hasBottom() || dst.hasBottom()))
            else if (self.hasBottom() || !dst.hasBottom()) && (!self.hasBottom() || dst.hasBottom()) {
                // Java: if (this.hasBottom() && dst.hasBottom())
                if self.hasBottom() && dst.hasBottom() {
                    // Java: if (isStrict && !this.bottom().equals(dst.bottom()))
                    if isStrict {
                        // Java: return false;
                    }
                    // Java: if (!isStrict && !this.bottom().equalsLax(dst.bottom()))
                    if !isStrict {
                        // Java: return false;
                    }
                }
                // Java: if ((this.hasInsideHorizontal() || !dst.hasInsideHorizontal()) && (!this.hasInsideHorizontal() || dst.hasInsideHorizontal()))
                if (self.hasInsideHorizontal() || !dst.hasInsideHorizontal()) && (!self.hasInsideHorizontal() || dst.hasInsideHorizontal()) {
                    // Java: if (this.hasInsideHorizontal() && dst.hasInsideHorizontal())
                    if self.hasInsideHorizontal() && dst.hasInsideHorizontal() {
                        // Java: if (isStrict && !this.insideHorizontal().equals(dst.insideHorizontal()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.insideHorizontal().equalsLax(dst.insideHorizontal()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                    // Java: if ((this.hasInsideVertical() || !dst.hasInsideVertical()) && (!this.hasInsideVertical() || dst.hasInsideVertical()))
                    if (self.hasInsideVertical() || !dst.hasInsideVertical()) && (!self.hasInsideVertical() || dst.hasInsideVertical()) {
                        // Java: if (this.hasInsideVertical() && dst.hasInsideVertical())
                        if self.hasInsideVertical() && dst.hasInsideVertical() {
                            // Java: if (isStrict && !this.insideVertical().equals(dst.insideVertical()))
                            if isStrict {
                                // Java: return false;
                            }
                            // Java: if (!isStrict && !this.insideVertical().equalsLax(dst.insideVertical()))
                            if !isStrict {
                                // Java: return false;
                            }
                        }
                        // Java: if ((this.hasLeft() || !dst.hasLeft()) && (!this.hasLeft() || dst.hasLeft()))
                        if (self.hasLeft() || !dst.hasLeft()) && (!self.hasLeft() || dst.hasLeft()) {
                            // Java: if (this.hasLeft() && dst.hasLeft())
                            if self.hasLeft() && dst.hasLeft() {
                                // Java: if (isStrict && !this.left().equals(dst.left()))
                                if isStrict {
                                    // Java: return false;
                                }
                                // Java: if (!isStrict && !this.left().equalsLax(dst.left()))
                                if !isStrict {
                                    // Java: return false;
                                }
                            }
                            // Java: if ((this.hasRight() || !dst.hasRight()) && (!this.hasRight() || dst.hasRight()))
                            if (self.hasRight() || !dst.hasRight()) && (!self.hasRight() || dst.hasRight()) {
                                // Java: if (this.hasRight() && dst.hasRight())
                                if self.hasRight() && dst.hasRight() {
                                    // Java: if (isStrict && !this.right().equals(dst.right()))
                                    if isStrict {
                                        // Java: return false;
                                    }
                                    // Java: if (!isStrict && !this.right().equalsLax(dst.right()))
                                    if !isStrict {
                                        // Java: return false;
                                    }
                                }
                                // Java: if ((this.hasTopLeft2BottomRight() || !dst.hasTopLeft2BottomRight()) && (!this.hasTopLeft2BottomRight() || dst.hasTopLeft2BottomRight()))
                                if (self.hasTopLeft2BottomRight() || !dst.hasTopLeft2BottomRight()) && (!self.hasTopLeft2BottomRight() || dst.hasTopLeft2BottomRight()) {
                                    // Java: if (this.hasTopLeft2BottomRight() && dst.hasTopLeft2BottomRight())
                                    if self.hasTopLeft2BottomRight() && dst.hasTopLeft2BottomRight() {
                                        // Java: if (isStrict && !this.topLeft2BottomRight().equals(dst.topLeft2BottomRight()))
                                        if isStrict {
                                            // Java: return false;
                                        }
                                        // Java: if (!isStrict && !this.topLeft2BottomRight().equalsLax(dst.topLeft2BottomRight()))
                                        if !isStrict {
                                            // Java: return false;
                                        }
                                    }
                                    // Java: if ((this.hasTop() || !dst.hasTop()) && (!this.hasTop() || dst.hasTop()))
                                    if (self.hasTop() || !dst.hasTop()) && (!self.hasTop() || dst.hasTop()) {
                                        // Java: if (this.hasTop() && dst.hasTop())
                                        if self.hasTop() && dst.hasTop() {
                                            // Java: if (isStrict && !this.top().equals(dst.top()))
                                            if isStrict {
                                                // Java: return false;
                                            }
                                            // Java: if (!isStrict && !this.top().equalsLax(dst.top()))
                                            if !isStrict {
                                                // Java: return false;
                                            }
                                        }
                                        // Java: if ((this.hasTopRight2BottomLeft() || !dst.hasTopRight2BottomLeft()) && (!this.hasTopRight2BottomLeft() || dst.hasTopRight2BottomLeft()))
                                        if (self.hasTopRight2BottomLeft() || !dst.hasTopRight2BottomLeft()) && (!self.hasTopRight2BottomLeft() || dst.hasTopRight2BottomLeft()) {
                                            // Java: if (this.hasTopRight2BottomLeft() && dst.hasTopRight2BottomLeft())
                                            if self.hasTopRight2BottomLeft() && dst.hasTopRight2BottomLeft() {
                                                // Java: if (isStrict && !this.topRight2BottomLeft().equals(dst.topRight2BottomLeft()))
                                                if isStrict {
                                                    // Java: return false;
                                                }
                                                // Java: if (!isStrict && !this.topRight2BottomLeft().equalsLax(dst.topRight2BottomLeft()))
                                                if !isStrict {
                                                    // Java: return false;
                                                }
                                            }
                                            // Java: return true;
                                            return true;
                                        }
                                        // Java: else
                                        else {
                                            // Java: return false;
                                            return false;
                                        }
                                    }
                                    // Java: else
                                    else {
                                        // Java: return false;
                                        return false;
                                    }
                                }
                                // Java: else
                                else {
                                    // Java: return false;
                                    return false;
                                }
                            }
                            // Java: else
                            else {
                                // Java: return false;
                                return false;
                            }
                        }
                        // Java: else
                        else {
                            // Java: return false;
                            return false;
                        }
                    }
                    // Java: else
                    else {
                        // Java: return false;
                        return false;
                    }
                }
                // Java: else
                else {
                    // Java: return false;
                    return false;
                }
            }
            // Java: else
            else {
                // Java: return false;
                return false;
            }
        }
        // Java: else
        else {
            // Java: return false;
            false
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int ret = 0;
        let mut ret: i32 = 0;
        // Java: if (this.hasBottom())
        if self.hasBottom() {
            // Java: ret += this.bottom().hashCode();
        }
        // Java: if (this.hasInsideHorizontal())
        if self.hasInsideHorizontal() {
            // Java: ret += this.insideHorizontal().hashCode();
        }
        // Java: if (this.hasInsideVertical())
        if self.hasInsideVertical() {
            // Java: ret += this.insideVertical().hashCode();
        }
        // Java: if (this.hasLeft())
        if self.hasLeft() {
            // Java: ret += this.left().hashCode();
        }
        // Java: if (this.hasRight())
        if self.hasRight() {
            // Java: ret += this.right().hashCode();
        }
        // Java: if (this.hasTopLeft2BottomRight())
        if self.hasTopLeft2BottomRight() {
            // Java: ret += this.topLeft2BottomRight().hashCode();
        }
        // Java: if (this.hasTop())
        if self.hasTop() {
            // Java: ret += this.top().hashCode();
        }
        // Java: if (this.hasTopRight2BottomLeft())
        if self.hasTopRight2BottomLeft() {
            // Java: ret += this.topRight2BottomLeft().hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: if (this.bottom != null && this.bottom.dirty())
        if self.bottom.as_ref().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else if (this.insideHorizontal != null && this.insideHorizontal.dirty())
        else if self.insideHorizontal.as_ref().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else if (this.insideVertical != null && this.insideVertical.dirty())
        else if self.insideVertical.as_ref().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else if (this.left != null && this.left.dirty())
        else if self.left.as_ref().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else if (this.right != null && this.right.dirty())
        else if self.right.as_ref().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else if (this.topLeft2BottomRight != null && this.topLeft2BottomRight.dirty())
        else if self.topLeft2BottomRight.as_ref().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else if (this.top != null && this.top.dirty())
        else if self.top.as_ref().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: return this.topRight2BottomLeft != null && this.topRight2BottomLeft.dirty() ? true : this.map.dirty();
            if self.topRight2BottomLeft.as_ref().map_or(false, |r| r.dirty()) { true } else { self.map.dirty() }
        }
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
        let n = Null::value1();
        self.map.load(n.as_ref());
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
        // Java: this.bottom = null;
        self.bottom = None;
        // Java: this.insideHorizontal = null;
        self.insideHorizontal = None;
        // Java: this.insideVertical = null;
        self.insideVertical = None;
        // Java: this.left = null;
        self.left = None;
        // Java: this.right = null;
        self.right = None;
        // Java: this.topLeft2BottomRight = null;
        self.topLeft2BottomRight = None;
        // Java: this.top = null;
        self.top = None;
        // Java: this.topRight2BottomLeft = null;
        self.topRight2BottomLeft = None;
    }

    // Java: void follow(TableCellBorderStyle parent)
    pub fn follow(&mut self, parent: &mut TableCellBorderStyle) {
        // Java: if (parent != null)
        if true {
            // Java: parent.discard();
            parent.discard();
            // Java: this.discard();
            self.discard();
            // Java: this.parent = parent;
        }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
        // Java: if (this.parent != null)
        if self.parent.is_some() {
            // Java: this.discard();
            self.discard();
            // Java: this.parent.discard();
            if let Some(p) = self.parent.as_mut() { p.discard(); }
            // Java: this.cleanParent();
            self.cleanParent();
        }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
        // Java: return this.parent != null;
        self.parent.is_some()
    }

    // Java: TableCellBorderStyle parent()
    pub fn parent_ref(&self) -> Option<&TableCellBorderStyle> {
        // Java: return this.parent;
        self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(self.zone);
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None;
    }

    // Java: ThemeableLineStyle bottom()
    pub fn bottom(&self) -> &ThemeableLineStyle {
        // Java: if (this.bottom == null)
        if self.bottom.is_none() {
            // Java: if (this.map.has(1))
            if self.map.has(TCBS_BOTTOM) {
                // Java: KoBin bin = this.map.byKey(1);
                let _bin = self.map.byKey(TCBS_BOTTOM);
                // Java: this.bottom = (ThemeableLineStyle)KoBin.Decode.toAny(ThemeableLineStyle.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasBottom())
            else if self.parent.as_ref().map_or(false, |p| p.hasBottom()) {
                // Java: this.bottom = this.parent.bottom();
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.bottom = (ThemeableLineStyle)KoBin.Decode.toAny(ThemeableLineStyle.class, bin, this.zone);
            }
        }
        // Java: return this.bottom;
        self.bottom.as_ref().unwrap()
    }

    // Java: boolean hasBottom()
    pub fn hasBottom(&self) -> bool {
        // Java: return this.map.has(1) || this.bottom != null && this.bottom.dirty() || this.parent != null && this.parent.hasBottom();
        self.map.has(TCBS_BOTTOM) || self.bottom.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasBottom())
    }

    // Java: void deleteBottom()
    pub fn deleteBottom(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(TCBS_BOTTOM);
        // Java: if (this.bottom != null)
        if self.bottom.is_some() {
            // Java: this.bottom = null;
            self.bottom = None;
        }
    }

    // Java: void writeBottom(ThemeableLineStyle value)
    pub fn writeBottom(&mut self, value: &ThemeableLineStyle) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(1, bin);
                self.map.put(TCBS_BOTTOM, bin);
                // Java: if (this.bottom != null)
                if self.bottom.is_some() {
                    // Java: this.bottom = null;
                    self.bottom = None;
                }
            }
        }
    }

    // Java: void forceWriteBottom(ThemeableLineStyle value)
    pub fn forceWriteBottom(&mut self, value: &ThemeableLineStyle) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(1, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(TCBS_BOTTOM, bin);
            // Java: if (this.bottom != null)
            if self.bottom.is_some() {
                // Java: this.bottom = null;
                self.bottom = None;
            }
        }
    }

    // Java: ThemeableLineStyle insideHorizontal()
    pub fn insideHorizontal(&self) -> &ThemeableLineStyle {
        // Java: if (this.insideHorizontal == null)
        if self.insideHorizontal.is_none() {
            // Java: if (this.map.has(2))
            if self.map.has(TCBS_INSIDE_HORIZONTAL) {
                // Java: KoBin bin = this.map.byKey(2);
                let _bin = self.map.byKey(TCBS_INSIDE_HORIZONTAL);
                // Java: this.insideHorizontal = (ThemeableLineStyle)KoBin.Decode.toAny(ThemeableLineStyle.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasInsideHorizontal())
            else if self.parent.as_ref().map_or(false, |p| p.hasInsideHorizontal()) {
                // Java: this.insideHorizontal = this.parent.insideHorizontal();
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.insideHorizontal = (ThemeableLineStyle)KoBin.Decode.toAny(ThemeableLineStyle.class, bin, this.zone);
            }
        }
        // Java: return this.insideHorizontal;
        self.insideHorizontal.as_ref().unwrap()
    }

    // Java: boolean hasInsideHorizontal()
    pub fn hasInsideHorizontal(&self) -> bool {
        // Java: return this.map.has(2) || this.insideHorizontal != null && this.insideHorizontal.dirty() || this.parent != null && this.parent.hasInsideHorizontal();
        self.map.has(TCBS_INSIDE_HORIZONTAL) || self.insideHorizontal.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasInsideHorizontal())
    }

    // Java: void deleteInsideHorizontal()
    pub fn deleteInsideHorizontal(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(TCBS_INSIDE_HORIZONTAL);
        // Java: if (this.insideHorizontal != null)
        if self.insideHorizontal.is_some() {
            // Java: this.insideHorizontal = null;
            self.insideHorizontal = None;
        }
    }

    // Java: void writeInsideHorizontal(ThemeableLineStyle value)
    pub fn writeInsideHorizontal(&mut self, value: &ThemeableLineStyle) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(2, bin);
                self.map.put(TCBS_INSIDE_HORIZONTAL, bin);
                // Java: if (this.insideHorizontal != null)
                if self.insideHorizontal.is_some() {
                    // Java: this.insideHorizontal = null;
                    self.insideHorizontal = None;
                }
            }
        }
    }

    // Java: void forceWriteInsideHorizontal(ThemeableLineStyle value)
    pub fn forceWriteInsideHorizontal(&mut self, value: &ThemeableLineStyle) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(2, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(TCBS_INSIDE_HORIZONTAL, bin);
            // Java: if (this.insideHorizontal != null)
            if self.insideHorizontal.is_some() {
                // Java: this.insideHorizontal = null;
                self.insideHorizontal = None;
            }
        }
    }

    // Java: ThemeableLineStyle insideVertical()
    pub fn insideVertical(&self) -> &ThemeableLineStyle {
        // Java: if (this.insideVertical == null)
        if self.insideVertical.is_none() {
            // Java: if (this.map.has(3))
            if self.map.has(TCBS_INSIDE_VERTICAL) {
                // Java: KoBin bin = this.map.byKey(3);
                let _bin = self.map.byKey(TCBS_INSIDE_VERTICAL);
                // Java: this.insideVertical = (ThemeableLineStyle)KoBin.Decode.toAny(ThemeableLineStyle.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasInsideVertical())
            else if self.parent.as_ref().map_or(false, |p| p.hasInsideVertical()) {
                // Java: this.insideVertical = this.parent.insideVertical();
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.insideVertical = (ThemeableLineStyle)KoBin.Decode.toAny(ThemeableLineStyle.class, bin, this.zone);
            }
        }
        // Java: return this.insideVertical;
        self.insideVertical.as_ref().unwrap()
    }

    // Java: boolean hasInsideVertical()
    pub fn hasInsideVertical(&self) -> bool {
        // Java: return this.map.has(3) || this.insideVertical != null && this.insideVertical.dirty() || this.parent != null && this.parent.hasInsideVertical();
        self.map.has(TCBS_INSIDE_VERTICAL) || self.insideVertical.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasInsideVertical())
    }

    // Java: void deleteInsideVertical()
    pub fn deleteInsideVertical(&mut self) {
        // Java: this.map.put(3, (KoBin)null);
        self.map.clear_key(TCBS_INSIDE_VERTICAL);
        // Java: if (this.insideVertical != null)
        if self.insideVertical.is_some() {
            // Java: this.insideVertical = null;
            self.insideVertical = None;
        }
    }

    // Java: void writeInsideVertical(ThemeableLineStyle value)
    pub fn writeInsideVertical(&mut self, value: &ThemeableLineStyle) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(3, bin);
                self.map.put(TCBS_INSIDE_VERTICAL, bin);
                // Java: if (this.insideVertical != null)
                if self.insideVertical.is_some() {
                    // Java: this.insideVertical = null;
                    self.insideVertical = None;
                }
            }
        }
    }

    // Java: void forceWriteInsideVertical(ThemeableLineStyle value)
    pub fn forceWriteInsideVertical(&mut self, value: &ThemeableLineStyle) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(3, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(TCBS_INSIDE_VERTICAL, bin);
            // Java: if (this.insideVertical != null)
            if self.insideVertical.is_some() {
                // Java: this.insideVertical = null;
                self.insideVertical = None;
            }
        }
    }

    // Java: ThemeableLineStyle left()
    pub fn left(&self) -> &ThemeableLineStyle {
        // Java: if (this.left == null)
        if self.left.is_none() {
            // Java: if (this.map.has(4))
            if self.map.has(TCBS_LEFT) {
                // Java: KoBin bin = this.map.byKey(4);
                let _bin = self.map.byKey(TCBS_LEFT);
                // Java: this.left = (ThemeableLineStyle)KoBin.Decode.toAny(ThemeableLineStyle.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasLeft())
            else if self.parent.as_ref().map_or(false, |p| p.hasLeft()) {
                // Java: this.left = this.parent.left();
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.left = (ThemeableLineStyle)KoBin.Decode.toAny(ThemeableLineStyle.class, bin, this.zone);
            }
        }
        // Java: return this.left;
        self.left.as_ref().unwrap()
    }

    // Java: boolean hasLeft()
    pub fn hasLeft(&self) -> bool {
        // Java: return this.map.has(4) || this.left != null && this.left.dirty() || this.parent != null && this.parent.hasLeft();
        self.map.has(TCBS_LEFT) || self.left.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasLeft())
    }

    // Java: void deleteLeft()
    pub fn deleteLeft(&mut self) {
        // Java: this.map.put(4, (KoBin)null);
        self.map.clear_key(TCBS_LEFT);
        // Java: if (this.left != null)
        if self.left.is_some() {
            // Java: this.left = null;
            self.left = None;
        }
    }

    // Java: void writeLeft(ThemeableLineStyle value)
    pub fn writeLeft(&mut self, value: &ThemeableLineStyle) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(4, bin);
                self.map.put(TCBS_LEFT, bin);
                // Java: if (this.left != null)
                if self.left.is_some() {
                    // Java: this.left = null;
                    self.left = None;
                }
            }
        }
    }

    // Java: void forceWriteLeft(ThemeableLineStyle value)
    pub fn forceWriteLeft(&mut self, value: &ThemeableLineStyle) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(4, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(TCBS_LEFT, bin);
            // Java: if (this.left != null)
            if self.left.is_some() {
                // Java: this.left = null;
                self.left = None;
            }
        }
    }

    // Java: ThemeableLineStyle right()
    pub fn right(&self) -> &ThemeableLineStyle {
        // Java: if (this.right == null)
        if self.right.is_none() {
            // Java: if (this.map.has(5))
            if self.map.has(TCBS_RIGHT) {
                // Java: KoBin bin = this.map.byKey(5);
                let _bin = self.map.byKey(TCBS_RIGHT);
                // Java: this.right = (ThemeableLineStyle)KoBin.Decode.toAny(ThemeableLineStyle.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasRight())
            else if self.parent.as_ref().map_or(false, |p| p.hasRight()) {
                // Java: this.right = this.parent.right();
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.right = (ThemeableLineStyle)KoBin.Decode.toAny(ThemeableLineStyle.class, bin, this.zone);
            }
        }
        // Java: return this.right;
        self.right.as_ref().unwrap()
    }

    // Java: boolean hasRight()
    pub fn hasRight(&self) -> bool {
        // Java: return this.map.has(5) || this.right != null && this.right.dirty() || this.parent != null && this.parent.hasRight();
        self.map.has(TCBS_RIGHT) || self.right.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasRight())
    }

    // Java: void deleteRight()
    pub fn deleteRight(&mut self) {
        // Java: this.map.put(5, (KoBin)null);
        self.map.clear_key(TCBS_RIGHT);
        // Java: if (this.right != null)
        if self.right.is_some() {
            // Java: this.right = null;
            self.right = None;
        }
    }

    // Java: void writeRight(ThemeableLineStyle value)
    pub fn writeRight(&mut self, value: &ThemeableLineStyle) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(5, bin);
                self.map.put(TCBS_RIGHT, bin);
                // Java: if (this.right != null)
                if self.right.is_some() {
                    // Java: this.right = null;
                    self.right = None;
                }
            }
        }
    }

    // Java: void forceWriteRight(ThemeableLineStyle value)
    pub fn forceWriteRight(&mut self, value: &ThemeableLineStyle) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(5, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(TCBS_RIGHT, bin);
            // Java: if (this.right != null)
            if self.right.is_some() {
                // Java: this.right = null;
                self.right = None;
            }
        }
    }

    // Java: ThemeableLineStyle topLeft2BottomRight()
    pub fn topLeft2BottomRight(&self) -> &ThemeableLineStyle {
        // Java: if (this.topLeft2BottomRight == null)
        if self.topLeft2BottomRight.is_none() {
            // Java: if (this.map.has(6))
            if self.map.has(TCBS_TOP_LEFT2_BOTTOM_RIGHT) {
                // Java: KoBin bin = this.map.byKey(6);
                let _bin = self.map.byKey(TCBS_TOP_LEFT2_BOTTOM_RIGHT);
                // Java: this.topLeft2BottomRight = (ThemeableLineStyle)KoBin.Decode.toAny(ThemeableLineStyle.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasTopLeft2BottomRight())
            else if self.parent.as_ref().map_or(false, |p| p.hasTopLeft2BottomRight()) {
                // Java: this.topLeft2BottomRight = this.parent.topLeft2BottomRight();
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.topLeft2BottomRight = (ThemeableLineStyle)KoBin.Decode.toAny(ThemeableLineStyle.class, bin, this.zone);
            }
        }
        // Java: return this.topLeft2BottomRight;
        self.topLeft2BottomRight.as_ref().unwrap()
    }

    // Java: boolean hasTopLeft2BottomRight()
    pub fn hasTopLeft2BottomRight(&self) -> bool {
        // Java: return this.map.has(6) || this.topLeft2BottomRight != null && this.topLeft2BottomRight.dirty() || this.parent != null && this.parent.hasTopLeft2BottomRight();
        self.map.has(TCBS_TOP_LEFT2_BOTTOM_RIGHT) || self.topLeft2BottomRight.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasTopLeft2BottomRight())
    }

    // Java: void deleteTopLeft2BottomRight()
    pub fn deleteTopLeft2BottomRight(&mut self) {
        // Java: this.map.put(6, (KoBin)null);
        self.map.clear_key(TCBS_TOP_LEFT2_BOTTOM_RIGHT);
        // Java: if (this.topLeft2BottomRight != null)
        if self.topLeft2BottomRight.is_some() {
            // Java: this.topLeft2BottomRight = null;
            self.topLeft2BottomRight = None;
        }
    }

    // Java: void writeTopLeft2BottomRight(ThemeableLineStyle value)
    pub fn writeTopLeft2BottomRight(&mut self, value: &ThemeableLineStyle) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(6, bin);
                self.map.put(TCBS_TOP_LEFT2_BOTTOM_RIGHT, bin);
                // Java: if (this.topLeft2BottomRight != null)
                if self.topLeft2BottomRight.is_some() {
                    // Java: this.topLeft2BottomRight = null;
                    self.topLeft2BottomRight = None;
                }
            }
        }
    }

    // Java: void forceWriteTopLeft2BottomRight(ThemeableLineStyle value)
    pub fn forceWriteTopLeft2BottomRight(&mut self, value: &ThemeableLineStyle) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(6, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(TCBS_TOP_LEFT2_BOTTOM_RIGHT, bin);
            // Java: if (this.topLeft2BottomRight != null)
            if self.topLeft2BottomRight.is_some() {
                // Java: this.topLeft2BottomRight = null;
                self.topLeft2BottomRight = None;
            }
        }
    }

    // Java: ThemeableLineStyle top()
    pub fn top(&self) -> &ThemeableLineStyle {
        // Java: if (this.top == null)
        if self.top.is_none() {
            // Java: if (this.map.has(7))
            if self.map.has(TCBS_TOP) {
                // Java: KoBin bin = this.map.byKey(7);
                let _bin = self.map.byKey(TCBS_TOP);
                // Java: this.top = (ThemeableLineStyle)KoBin.Decode.toAny(ThemeableLineStyle.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasTop())
            else if self.parent.as_ref().map_or(false, |p| p.hasTop()) {
                // Java: this.top = this.parent.top();
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.top = (ThemeableLineStyle)KoBin.Decode.toAny(ThemeableLineStyle.class, bin, this.zone);
            }
        }
        // Java: return this.top;
        self.top.as_ref().unwrap()
    }

    // Java: boolean hasTop()
    pub fn hasTop(&self) -> bool {
        // Java: return this.map.has(7) || this.top != null && this.top.dirty() || this.parent != null && this.parent.hasTop();
        self.map.has(TCBS_TOP) || self.top.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasTop())
    }

    // Java: void deleteTop()
    pub fn deleteTop(&mut self) {
        // Java: this.map.put(7, (KoBin)null);
        self.map.clear_key(TCBS_TOP);
        // Java: if (this.top != null)
        if self.top.is_some() {
            // Java: this.top = null;
            self.top = None;
        }
    }

    // Java: void writeTop(ThemeableLineStyle value)
    pub fn writeTop(&mut self, value: &ThemeableLineStyle) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(7, bin);
                self.map.put(TCBS_TOP, bin);
                // Java: if (this.top != null)
                if self.top.is_some() {
                    // Java: this.top = null;
                    self.top = None;
                }
            }
        }
    }

    // Java: void forceWriteTop(ThemeableLineStyle value)
    pub fn forceWriteTop(&mut self, value: &ThemeableLineStyle) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(7, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(TCBS_TOP, bin);
            // Java: if (this.top != null)
            if self.top.is_some() {
                // Java: this.top = null;
                self.top = None;
            }
        }
    }

    // Java: ThemeableLineStyle topRight2BottomLeft()
    pub fn topRight2BottomLeft(&self) -> &ThemeableLineStyle {
        // Java: if (this.topRight2BottomLeft == null)
        if self.topRight2BottomLeft.is_none() {
            // Java: if (this.map.has(8))
            if self.map.has(TCBS_TOP_RIGHT2_BOTTOM_LEFT) {
                // Java: KoBin bin = this.map.byKey(8);
                let _bin = self.map.byKey(TCBS_TOP_RIGHT2_BOTTOM_LEFT);
                // Java: this.topRight2BottomLeft = (ThemeableLineStyle)KoBin.Decode.toAny(ThemeableLineStyle.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasTopRight2BottomLeft())
            else if self.parent.as_ref().map_or(false, |p| p.hasTopRight2BottomLeft()) {
                // Java: this.topRight2BottomLeft = this.parent.topRight2BottomLeft();
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.topRight2BottomLeft = (ThemeableLineStyle)KoBin.Decode.toAny(ThemeableLineStyle.class, bin, this.zone);
            }
        }
        // Java: return this.topRight2BottomLeft;
        self.topRight2BottomLeft.as_ref().unwrap()
    }

    // Java: boolean hasTopRight2BottomLeft()
    pub fn hasTopRight2BottomLeft(&self) -> bool {
        // Java: return this.map.has(8) || this.topRight2BottomLeft != null && this.topRight2BottomLeft.dirty() || this.parent != null && this.parent.hasTopRight2BottomLeft();
        self.map.has(TCBS_TOP_RIGHT2_BOTTOM_LEFT) || self.topRight2BottomLeft.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasTopRight2BottomLeft())
    }

    // Java: void deleteTopRight2BottomLeft()
    pub fn deleteTopRight2BottomLeft(&mut self) {
        // Java: this.map.put(8, (KoBin)null);
        self.map.clear_key(TCBS_TOP_RIGHT2_BOTTOM_LEFT);
        // Java: if (this.topRight2BottomLeft != null)
        if self.topRight2BottomLeft.is_some() {
            // Java: this.topRight2BottomLeft = null;
            self.topRight2BottomLeft = None;
        }
    }

    // Java: void writeTopRight2BottomLeft(ThemeableLineStyle value)
    pub fn writeTopRight2BottomLeft(&mut self, value: &ThemeableLineStyle) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(8, bin);
                self.map.put(TCBS_TOP_RIGHT2_BOTTOM_LEFT, bin);
                // Java: if (this.topRight2BottomLeft != null)
                if self.topRight2BottomLeft.is_some() {
                    // Java: this.topRight2BottomLeft = null;
                    self.topRight2BottomLeft = None;
                }
            }
        }
    }

    // Java: void forceWriteTopRight2BottomLeft(ThemeableLineStyle value)
    pub fn forceWriteTopRight2BottomLeft(&mut self, value: &ThemeableLineStyle) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(8, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(TCBS_TOP_RIGHT2_BOTTOM_LEFT, bin);
            // Java: if (this.topRight2BottomLeft != null)
            if self.topRight2BottomLeft.is_some() {
                // Java: this.topRight2BottomLeft = null;
                self.topRight2BottomLeft = None;
            }
        }
    }
}

impl KoAr for TableStyleCellStyle {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { TableStyleCellStyle::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { TableStyleCellStyle::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { TableStyleCellStyle::discard_with_zone(self, var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { TableStyleCellStyle::lastBin(self, var1) }
}

impl KoClosable for TableStyleCellStyle {
    fn close(&mut self) { TableStyleCellStyle::close(self); }
}
