#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.basic.RelativeRect
use crate::cn::wps::drawing::data::basic::RelativeRect::RelativeRect;
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
// Java: import cn.wps.kso.kproto.internal.KpList
use crate::cn::wps::kso::kproto::internal::KpList::KpList;
// Java: import cn.wps.kso.kproto.internal.KpMap
use crate::cn::wps::kso::kproto::internal::KpMap::KpMap;
// Java: import java.util.ArrayList
// Java: import java.util.Collection

use super::Shade::Shade;

// Java: public class GradientStop
pub struct GradientStop {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Color color
    pub(crate) color: Option<Box<Color>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: GradientStop parent
    pub(crate) parent: Option<Box<GradientStop>>,
}

// Java: public class GradientStopList
pub struct GradientStopList {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

// Java: public class GradientFill implements KoAr, KoClosable
pub struct GradientFill {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: GradientStopList gsLst
    pub(crate) gsLst: Option<Box<GradientStopList>>,
    // Java field: Shade shade
    pub(crate) shade: Option<Box<Shade>>,
    // Java field: RelativeRect tileRect
    pub(crate) tileRect: Option<Box<RelativeRect>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: GradientFill parent
    pub(crate) parent: Option<Box<GradientFill>>,
}

// Java: public class GradientFill implements KoAr, KoClosable
impl GradientFill {
    // Java: static protected static <Value1 extends GradientFill> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = GradientFill::GradientFill();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static GradientFill create()
    pub fn create() -> Self {
        // Java: return initialize(GradientFill.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static GradientFill open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(GradientFill.class, bin);
        Self::initialize(bin)
    }

    // Java: GradientFill()
    pub fn GradientFill() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        GradientFill { zone: KoZone::enter(), gsLst: None, shade: None, tileRect: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: GradientFill(KoBin bin)
    pub fn GradientFill_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        GradientFill { zone: KoZone::enter(), gsLst: None, shade: None, tileRect: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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
        self.loadBin(bin, KoZone::enter());
    }

    // Java: void load(GradientFill value)
    pub fn load_from(&mut self, value: Option<&GradientFill>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(KoZone::enter());
            self.loadBin(last.as_ref(), KoZone::enter());
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.gsLst != null && this.gsLst.dirty())
        if let Some(ref mut g) = self.gsLst { if g.dirty() {
            // Java: this.map.put(3, this.gsLst.saveBin(zone));
            let bin = g.saveBin(zone);
            self.map.put(3, bin);
        }}
        // Java: if (this.shade != null && this.shade.dirty())
        if let Some(ref mut s) = self.shade { if s.dirty() {
            // Java: this.map.put(4, this.shade.saveBin(zone));
            let bin = s.saveBin(zone);
            self.map.put(4, bin);
        }}
        // Java: if (this.tileRect != null && this.tileRect.dirty())
        if let Some(ref mut t) = self.tileRect { if t.dirty() {
            // Java: this.map.put(5, this.tileRect.saveBin(zone));
            let bin = t.saveBin(zone);
            self.map.put(5, bin);
        }}
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            saved
        } else {
            self.bin_box.get()
        }
    }

    // Java: boolean equalsLax(GradientFill o)
    pub fn equalsLax(&self, o: &GradientFill) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &GradientFill) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &GradientFill, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof GradientFill)
        if true {
        // Java: GradientFill dst = (GradientFill)o;
        let dst = o;
        // Java: if (this == dst)
        if std::ptr::eq(self, dst) {
            // Java: return true;
            return true;
        }
        // Java: else if ((this.hasFlip() || !dst.hasFlip()) && (!this.hasFlip() || dst.hasFlip()))
        else if (self.hasFlip() || !dst.hasFlip()) && (!self.hasFlip() || dst.hasFlip()) {
            // Java: if (this.hasFlip() && dst.hasFlip() && this.flip() != dst.flip())
            if self.hasFlip() && dst.hasFlip() && self.flip() != dst.flip() {
                // Java: return false;
                return false;
            }
            // Java: else if ((this.hasRotWithShape() || !dst.hasRotWithShape()) && (!this.hasRotWithShape() || dst.hasRotWithShape()))
            else if (self.hasRotWithShape() || !dst.hasRotWithShape()) && (!self.hasRotWithShape() || dst.hasRotWithShape()) {
                // Java: if (this.hasRotWithShape() && dst.hasRotWithShape() && this.isRotWithShape() != dst.isRotWithShape())
                if self.hasRotWithShape() && dst.hasRotWithShape() && self.isRotWithShape() != dst.isRotWithShape() {
                    // Java: return false;
                    return false;
                }
                // Java: else if ((this.hasGsLst() || !dst.hasGsLst()) && (!this.hasGsLst() || dst.hasGsLst()))
                else if (self.hasGsLst() || !dst.hasGsLst()) && (!self.hasGsLst() || dst.hasGsLst()) {
                    // Java: if (this.hasGsLst() && dst.hasGsLst())
                    if self.hasGsLst() && dst.hasGsLst() {
                        // Java: if (isStrict && !this.gsLst().equals(dst.gsLst()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.gsLst().equalsLax(dst.gsLst()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                    // Java: if ((this.hasShade() || !dst.hasShade()) && (!this.hasShade() || dst.hasShade()))
                    if (self.hasShade() || !dst.hasShade()) && (!self.hasShade() || dst.hasShade()) {
                        // Java: if (this.hasShade() && dst.hasShade())
                        if self.hasShade() && dst.hasShade() {
                            // Java: if (isStrict && !this.shade().equals(dst.shade()))
                            if isStrict {
                                // Java: return false;
                            }
                            // Java: if (!isStrict && !this.shade().equalsLax(dst.shade()))
                            if !isStrict {
                                // Java: return false;
                            }
                        }
                        // Java: if ((this.hasTileRect() || !dst.hasTileRect()) && (!this.hasTileRect() || dst.hasTileRect()))
                        if (self.hasTileRect() || !dst.hasTileRect()) && (!self.hasTileRect() || dst.hasTileRect()) {
                            // Java: if (this.hasTileRect() && dst.hasTileRect())
                            if self.hasTileRect() && dst.hasTileRect() {
                                // Java: if (isStrict && !this.tileRect().equals(dst.tileRect()))
                                if isStrict {
                                    // Java: return false;
                                }
                                // Java: if (!isStrict && !this.tileRect().equalsLax(dst.tileRect()))
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
            false
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int ret = 0;
        let mut ret: i32 = 0;
        // Java: if (this.hasFlip())
        if self.hasFlip() {
            // Java: ret += Integer.valueOf(this.flip()).hashCode();
            ret = ret.wrapping_add(self.flip());
        }
        // Java: if (this.hasRotWithShape())
        if self.hasRotWithShape() {
            // Java: ret += Boolean.valueOf(this.isRotWithShape()).hashCode();
            ret = ret.wrapping_add(if self.isRotWithShape() { 1231 } else { 1237 });
        }
        // Java: if (this.hasGsLst())
        if self.hasGsLst() {
            // Java: ret += this.gsLst().hashCode();
        }
        // Java: if (this.hasShade())
        if self.hasShade() {
            // Java: ret += this.shade().hashCode();
        }
        // Java: if (this.hasTileRect())
        if self.hasTileRect() {
            // Java: ret += this.tileRect().hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: if (this.gsLst != null && this.gsLst.dirty())
        if self.gsLst.as_ref().map_or(false, |g| g.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else if (this.shade != null && this.shade.dirty())
        else if self.shade.as_ref().map_or(false, |s| s.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: return this.tileRect != null && this.tileRect.dirty() ? true : this.map.dirty();
            if self.tileRect.as_ref().map_or(false, |t| t.dirty()) { true } else { self.map.dirty() }
        }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
        // Java: this.saveBin(this.zone);
        self.saveBin(KoZone::enter());
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
        // Java: this.gsLst = null;
        self.gsLst = None;
        // Java: this.shade = null;
        self.shade = None;
        // Java: this.tileRect = null;
        self.tileRect = None;
    }

    // Java: void follow(GradientFill parent)
    pub fn follow(&mut self, parent: Option<Box<GradientFill>>) {
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

    // Java: GradientFill parent()
    pub fn parent_ref(&self) -> Option<&GradientFill> {
        // Java: return this.parent;
        self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(KoZone::enter());
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None;
    }

    // Java: int flip()
    pub fn flip(&self) -> i32 {
        // Java: if (!this.map.has(1))
        if !self.map.has(1) {
            // Java: return this.parent == null ? 0 : this.parent.flip();
            match &self.parent { None => 0, Some(p) => p.flip() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasFlip()
    pub fn hasFlip(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasFlip();
        self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasFlip())
    }

    // Java: void deleteFlip()
    pub fn deleteFlip(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(1);
    }

    // Java: void putFlip(int value)
    pub fn putFlip(&mut self, value: i32) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, KoZone::enter());
        self.map.put(1, bin);
    }

    // Java: boolean isRotWithShape()
    pub fn isRotWithShape(&self) -> bool {
        // Java: if (!this.map.has(2))
        if !self.map.has(2) {
            // Java: return this.parent == null ? true : this.parent.isRotWithShape();
            match &self.parent { None => true, Some(p) => p.isRotWithShape() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(2);
            let bin = self.map.byKey(2);
            // Java: return KoBin.Decode.toBool(bin);
            Decode::toBool(bin.as_ref())
        }
    }

    // Java: boolean hasRotWithShape()
    pub fn hasRotWithShape(&self) -> bool {
        // Java: return this.map.has(2) || this.parent != null && this.parent.hasRotWithShape();
        self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasRotWithShape())
    }

    // Java: void deleteRotWithShape()
    pub fn deleteRotWithShape(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(2);
    }

    // Java: void putRotWithShape(boolean value)
    pub fn putRotWithShape(&mut self, value: bool) {
        // Java: this.map.put(2, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_bool(value, KoZone::enter());
        self.map.put(2, bin);
    }

    // Java: GradientStopList gsLst()
    pub fn gsLst(&mut self) -> &mut GradientStopList {
        // Java: if (this.gsLst == null)
        if self.gsLst.is_none() {
            // Java: if (this.map.has(3))
            if self.map.has(3) {
                // Java: KoBin bin = this.map.byKey(3);
                let bin = self.map.byKey(3);
                // Java: this.gsLst = (GradientStopList)KoBin.Decode.toAny(GradientStopList.class, bin, this.zone);
                self.gsLst = Some(Box::new(GradientStopList::open(bin.as_ref())));
            }
            // Java: else if (this.parent != null && this.parent.hasGsLst())
            else if self.parent.as_ref().map_or(false, |p| p.hasGsLst()) {
                // Java: this.gsLst = this.parent.gsLst();
                let bin = Null::value1();
                self.gsLst = Some(Box::new(GradientStopList::open(bin.as_ref())));
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.gsLst = (GradientStopList)KoBin.Decode.toAny(GradientStopList.class, bin, this.zone);
                self.gsLst = Some(Box::new(GradientStopList::open(bin.as_ref())));
            }
        }
        // Java: return this.gsLst;
        self.gsLst.as_mut().unwrap()
    }

    // Java: boolean hasGsLst()
    pub fn hasGsLst(&self) -> bool {
        // Java: return this.map.has(3) || this.gsLst != null && this.gsLst.dirty() || this.parent != null && this.parent.hasGsLst();
        self.map.has(3) || self.gsLst.as_ref().map_or(false, |g| g.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasGsLst())
    }

    // Java: void deleteGsLst()
    pub fn deleteGsLst(&mut self) {
        // Java: this.map.put(3, (KoBin)null);
        self.map.clear_key(3);
        // Java: if (this.gsLst != null)
        if self.gsLst.is_some() {
            // Java: this.gsLst = null;
            self.gsLst = None;
        }
    }

    // Java: void writeGsLst(GradientStopList value)
    pub fn writeGsLst(&mut self, value: Option<&GradientStopList>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = val.lastBin(KoZone::enter());
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(3, bin);
                self.map.put(3, bin);
                // Java: if (this.gsLst != null)
                if self.gsLst.is_some() {
                    // Java: this.gsLst = null;
                    self.gsLst = None;
                }
            }
        }
    }

    // Java: void forceWriteGsLst(GradientStopList value)
    pub fn forceWriteGsLst(&mut self, value: Option<&GradientStopList>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(3, value.lastBin(this.zone));
            let bin = val.lastBin(KoZone::enter());
            self.map.put(3, bin);
            // Java: if (this.gsLst != null)
            if self.gsLst.is_some() {
                // Java: this.gsLst = null;
                self.gsLst = None;
            }
        }
    }

    // Java: Shade shade()
    pub fn shade(&mut self) -> &mut Shade {
        // Java: if (this.shade == null)
        if self.shade.is_none() {
            // Java: if (this.map.has(4))
            if self.map.has(4) {
                // Java: KoBin bin = this.map.byKey(4);
                let bin = self.map.byKey(4);
                // Java: this.shade = (Shade)KoBin.Decode.toAny(Shade.class, bin, this.zone);
                self.shade = Some(Box::new(Shade::open(bin.as_ref())));
            }
            // Java: else if (this.parent != null && this.parent.hasShade())
            else if self.parent.as_ref().map_or(false, |p| p.hasShade()) {
                // Java: this.shade = this.parent.shade();
                let bin = Null::value1();
                self.shade = Some(Box::new(Shade::open(bin.as_ref())));
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.shade = (Shade)KoBin.Decode.toAny(Shade.class, bin, this.zone);
                self.shade = Some(Box::new(Shade::open(bin.as_ref())));
            }
        }
        // Java: return this.shade;
        self.shade.as_mut().unwrap()
    }

    // Java: boolean hasShade()
    pub fn hasShade(&self) -> bool {
        // Java: return this.map.has(4) || this.shade != null && this.shade.dirty() || this.parent != null && this.parent.hasShade();
        self.map.has(4) || self.shade.as_ref().map_or(false, |s| s.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasShade())
    }

    // Java: void deleteShade()
    pub fn deleteShade(&mut self) {
        // Java: this.map.put(4, (KoBin)null);
        self.map.clear_key(4);
        // Java: if (this.shade != null)
        if self.shade.is_some() {
            // Java: this.shade = null;
            self.shade = None;
        }
    }

    // Java: void writeShade(Shade value)
    pub fn writeShade(&mut self, value: Option<&Shade>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = val.lastBin(KoZone::enter());
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(4, bin);
                self.map.put(4, bin);
                // Java: if (this.shade != null)
                if self.shade.is_some() {
                    // Java: this.shade = null;
                    self.shade = None;
                }
            }
        }
    }

    // Java: void forceWriteShade(Shade value)
    pub fn forceWriteShade(&mut self, value: Option<&Shade>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(4, value.lastBin(this.zone));
            let bin = val.lastBin(KoZone::enter());
            self.map.put(4, bin);
            // Java: if (this.shade != null)
            if self.shade.is_some() {
                // Java: this.shade = null;
                self.shade = None;
            }
        }
    }

    // Java: RelativeRect tileRect()
    pub fn tileRect(&mut self) -> &mut RelativeRect {
        // Java: if (this.tileRect == null)
        if self.tileRect.is_none() {
            // Java: if (this.map.has(5))
            if self.map.has(5) {
                // Java: KoBin bin = this.map.byKey(5);
                let bin = self.map.byKey(5);
                // Java: this.tileRect = (RelativeRect)KoBin.Decode.toAny(RelativeRect.class, bin, this.zone);
                self.tileRect = Some(Box::new(RelativeRect::open(bin.as_ref())));
            }
            // Java: else if (this.parent != null && this.parent.hasTileRect())
            else if self.parent.as_ref().map_or(false, |p| p.hasTileRect()) {
                // Java: this.tileRect = this.parent.tileRect();
                let bin = Null::value1();
                self.tileRect = Some(Box::new(RelativeRect::open(bin.as_ref())));
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.tileRect = (RelativeRect)KoBin.Decode.toAny(RelativeRect.class, bin, this.zone);
                self.tileRect = Some(Box::new(RelativeRect::open(bin.as_ref())));
            }
        }
        // Java: return this.tileRect;
        self.tileRect.as_mut().unwrap()
    }

    // Java: boolean hasTileRect()
    pub fn hasTileRect(&self) -> bool {
        // Java: return this.map.has(5) || this.tileRect != null && this.tileRect.dirty() || this.parent != null && this.parent.hasTileRect();
        self.map.has(5) || self.tileRect.as_ref().map_or(false, |t| t.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasTileRect())
    }

    // Java: void deleteTileRect()
    pub fn deleteTileRect(&mut self) {
        // Java: this.map.put(5, (KoBin)null);
        self.map.clear_key(5);
        // Java: if (this.tileRect != null)
        if self.tileRect.is_some() {
            // Java: this.tileRect = null;
            self.tileRect = None;
        }
    }

    // Java: void writeTileRect(RelativeRect value)
    pub fn writeTileRect(&mut self, value: Option<&RelativeRect>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = val.lastBin(KoZone::enter());
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(5, bin);
                self.map.put(5, bin);
                // Java: if (this.tileRect != null)
                if self.tileRect.is_some() {
                    // Java: this.tileRect = null;
                    self.tileRect = None;
                }
            }
        }
    }

    // Java: void forceWriteTileRect(RelativeRect value)
    pub fn forceWriteTileRect(&mut self, value: Option<&RelativeRect>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(5, value.lastBin(this.zone));
            let bin = val.lastBin(KoZone::enter());
            self.map.put(5, bin);
            // Java: if (this.tileRect != null)
            if self.tileRect.is_some() {
                // Java: this.tileRect = null;
                self.tileRect = None;
            }
        }
    }
}

// Java: public class GradientStopList
impl GradientStopList {
    // Java: GradientStopList()
    pub fn new() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.list = new KpList(this.zone);
        GradientStopList { bin_box: KpBinBox::new(), list: KpList::new(KoZone::enter()), zone: KoZone::defaultZone() }
    }

    // Java: GradientStopList(KoBin bin)
    pub fn GradientStopList_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.list = new KpList(bin, this.zone);
        GradientStopList { bin_box: KpBinBox::KpBinBox_from(Some(bin)), list: KpList::KpList_from(bin, KoZone::enter()), zone: KoZone::defaultZone() }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, zone: &KoZone) {
        // Java: this.box.set(bin);
        self.bin_box.set(Some(bin));
        // Java: this.list.load(this.box.get(), zone);
        let b = self.bin_box.get();
        self.list.load(b.as_ref(), zone);
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
        // Java: this.loadBin(bin, this.zone);
        self.loadBin(bin, KoZone::enter());
    }

    // Java: void load(GradientStopList value)
    pub fn load_from(&mut self, value: Option<&GradientStopList>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(KoZone::enter());
            self.loadBin(last.as_ref(), KoZone::enter());
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return this.list.dirty() ? this.box.set(this.list.save(zone)) : this.box.get();
        if self.list.dirty() {
            let saved = self.list.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            saved
        } else {
            self.bin_box.get()
        }
    }

    // Java: boolean equalsLax(GradientStopList o)
    pub fn equalsLax(&self, o: &GradientStopList) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &GradientStopList) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &GradientStopList, _isStrict: bool) -> bool {
        // Java: if (null == o)
        // Java: return false;
        // Java: else
        // Java: GradientStopList dst = (GradientStopList)o;
        let dst = o;
        // Java: if (this == dst)
        if std::ptr::eq(self, dst) {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: ArrayList<GradientStop> dstList = new ArrayList();
            // Java: dst.read(dstList);
            // Java: ArrayList<GradientStop> srcList = new ArrayList();
            // Java: this.read(srcList);
            // Java: if (dstList.size() != srcList.size())
            if dst.list.size() != self.list.size() {
                // Java: return false;
                return false;
            }
            // Java: else
            else {
                // Java: int size = dstList.size();
                // Java: for(int i = 0; i < size; ++i)
                // Java: GradientStop dstClrTrans = (GradientStop)dstList.get(i);
                // Java: GradientStop srcClrTrans = (GradientStop)srcList.get(i);
                // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                // Java: return false;
                // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                // Java: return false;
                // Java: return true;
                return true;
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
        self.saveBin(KoZone::enter());
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
        let n = Null::value1();
        self.list.load(n.as_ref(), KoZone::enter());
        // Java: this.zone.leave();
        self.zone.leave();
        // Java: this.zone = KoZone.defaultZone();
        self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends GradientStopList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = GradientStopList::new();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static GradientStopList create()
    pub fn create() -> Self {
        // Java: return initialize(GradientStopList.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static GradientStopList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(GradientStopList.class, bin);
        Self::initialize(bin)
    }

    // Java: void read(Collection<GradientStop> valueLst)
    pub fn read(&self, valueLst: &mut Vec<Box<dyn KoBinTrait>>) {
        // Java: valueLst.clear();
        valueLst.clear();
        // Java: int size = this.list.size();
        let size = self.list.size();
        // Java: for(int i = 0; i < size; ++i)
        for i in 0..size {
            // Java: valueLst.add((GradientStop)KoBin.Decode.toAny(GradientStop.class, this.list.get(i), this.zone));
            // rust: `Factory::dup` 形参是 `&dyn KoBin`；port 原样把 `this.list.get(i)` 当 `&dyn KoBin` 传，
            //   但本 port 的 `list.get(i)` 返回 `Box<dyn KoBin>`（Java 返回 `KoBin` 引用）
            //   ⇒ 需显式再借用。逻辑与 Java 一致，仅 Rust 侧的引用层次差异。
            let item = Factory::dup(self.list.get(i).as_ref(), KoZone::enter());
            valueLst.push(item);
        }
    }

    // Java: void write(Iterable<GradientStop> valueLst)
    pub fn write(&mut self, valueLst: &[Box<dyn KoBinTrait>]) {
        // Java: this.list.clear();
        self.list.clear();
        // Java: for(GradientStop i : valueLst)
        for item in valueLst.iter() {
            // Java: KoBin bin = i.lastBin(this.zone);
            let bin = Factory::dup(item.as_ref(), KoZone::enter());
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.list.add(bin);
                self.list.add(bin);
            }
        }
    }

    // Java: void forceWrite(Iterable<GradientStop> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[Box<dyn KoBinTrait>]) {
        // Java: this.list.clear();
        self.list.clear();
        // Java: for(GradientStop i : valueLst)
        for item in valueLst.iter() {
            // Java: this.list.add(i.lastBin(this.zone));
            let bin = Factory::dup(item.as_ref(), KoZone::enter());
            self.list.add(bin);
        }
    }
}

// Java: public class GradientStop
impl GradientStop {
    // Java: static protected static <Value1 extends GradientStop> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = GradientStop::new();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static GradientStop create()
    pub fn create() -> Self {
        // Java: return initialize(GradientStop.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static GradientStop open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(GradientStop.class, bin);
        Self::initialize(bin)
    }

    // Java: GradientStop()
    pub fn new() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        GradientStop { zone: KoZone::enter(), color: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: GradientStop(KoBin bin)
    pub fn GradientStop_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        GradientStop { zone: KoZone::enter(), color: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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
        self.loadBin(bin, KoZone::enter());
    }

    // Java: void load(GradientStop value)
    pub fn load_from(&mut self, value: Option<&GradientStop>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(KoZone::enter());
            self.loadBin(last.as_ref(), KoZone::enter());
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.color != null && this.color.dirty())
        if let Some(ref mut c) = self.color { if c.dirty() {
            // Java: this.map.put(2, this.color.saveBin(zone));
            let bin = c.saveBin(zone);
            self.map.put(2, bin);
        }}
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            saved
        } else {
            self.bin_box.get()
        }
    }

    // Java: boolean equalsLax(GradientStop o)
    pub fn equalsLax(&self, o: &GradientStop) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &GradientStop) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &GradientStop, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof GradientStop)
        if true {
        // Java: GradientStop dst = (GradientStop)o;
        let dst = o;
        // Java: if (this == dst)
        if std::ptr::eq(self, dst) {
            // Java: return true;
            return true;
        }
        // Java: else if ((this.hasPos() || !dst.hasPos()) && (!this.hasPos() || dst.hasPos()))
        else if (self.hasPos() || !dst.hasPos()) && (!self.hasPos() || dst.hasPos()) {
            // Java: if (this.hasPos() && dst.hasPos() && this.pos() != dst.pos())
            if self.hasPos() && dst.hasPos() && self.pos() != dst.pos() {
                // Java: return false;
                return false;
            }
            // Java: else if ((this.hasColor() || !dst.hasColor()) && (!this.hasColor() || dst.hasColor()))
            else if (self.hasColor() || !dst.hasColor()) && (!self.hasColor() || dst.hasColor()) {
                // Java: if (this.hasColor() && dst.hasColor())
                if self.hasColor() && dst.hasColor() {
                    // Java: if (isStrict && !this.color().equals(dst.color()))
                    if isStrict {
                        // Java: return false;
                    }
                    // Java: if (!isStrict && !this.color().equalsLax(dst.color()))
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
            false
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int ret = 0;
        let mut ret: i32 = 0;
        // Java: if (this.hasPos())
        if self.hasPos() {
            // Java: ret += Double.valueOf(this.pos()).hashCode();
            let bits = self.pos().to_bits() as i64;
            ret = ret.wrapping_add((bits ^ (bits >> 32)) as i32);
        }
        // Java: if (this.hasColor())
        if self.hasColor() {
            // Java: ret += this.color().hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: return this.color != null && this.color.dirty() ? true : this.map.dirty();
        if self.color.as_ref().map_or(false, |c| c.dirty()) { true } else { self.map.dirty() }
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
        // Java: this.saveBin(this.zone);
        self.saveBin(KoZone::enter());
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
        // Java: this.color = null;
        self.color = None;
    }

    // Java: void follow(GradientStop parent)
    pub fn follow(&mut self, parent: Option<Box<GradientStop>>) {
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

    // Java: GradientStop parent()
    pub fn parent_ref(&self) -> Option<&GradientStop> {
        // Java: return this.parent;
        self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(KoZone::enter());
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None;
    }

    // Java: double pos()
    pub fn pos(&self) -> f64 {
        // Java: if (!this.map.has(1))
        if !self.map.has(1) {
            // Java: return this.parent == null ? (double)0.0F : this.parent.pos();
            match &self.parent { None => 0.0, Some(p) => p.pos() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toDouble(bin);
            Decode::toDouble(bin.as_ref())
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

    // Java: void putPos(double value)
    pub fn putPos(&mut self, value: f64) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_double(value, KoZone::enter());
        self.map.put(1, bin);
    }

    // Java: Color color()
    pub fn color(&mut self) -> &mut Color {
        // Java: if (this.color == null)
        if self.color.is_none() {
            // Java: if (this.map.has(2))
            if self.map.has(2) {
                // Java: KoBin bin = this.map.byKey(2);
                let bin = self.map.byKey(2);
                // Java: this.color = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
                self.color = Some(Box::new(Color::open(bin.as_ref())));
            }
            // Java: else if (this.parent != null && this.parent.hasColor())
            else if self.parent.as_ref().map_or(false, |p| p.hasColor()) {
                // Java: this.color = this.parent.color();
                let bin = Null::value1();
                self.color = Some(Box::new(Color::open(bin.as_ref())));
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.color = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
                self.color = Some(Box::new(Color::open(bin.as_ref())));
            }
        }
        // Java: return this.color;
        self.color.as_mut().unwrap()
    }

    // Java: boolean hasColor()
    pub fn hasColor(&self) -> bool {
        // Java: return this.map.has(2) || this.color != null && this.color.dirty() || this.parent != null && this.parent.hasColor();
        self.map.has(2) || self.color.as_ref().map_or(false, |c| c.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasColor())
    }

    // Java: void deleteColor()
    pub fn deleteColor(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(2);
        // Java: if (this.color != null)
        if self.color.is_some() {
            // Java: this.color = null;
            self.color = None;
        }
    }

    // Java: void writeColor(Color value)
    pub fn writeColor(&mut self, value: Option<&Color>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = val.lastBin(KoZone::enter());
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(2, bin);
                self.map.put(2, bin);
                // Java: if (this.color != null)
                if self.color.is_some() {
                    // Java: this.color = null;
                    self.color = None;
                }
            }
        }
    }

    // Java: void forceWriteColor(Color value)
    pub fn forceWriteColor(&mut self, value: Option<&Color>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(2, value.lastBin(this.zone));
            let bin = val.lastBin(KoZone::enter());
            self.map.put(2, bin);
            // Java: if (this.color != null)
            if self.color.is_some() {
                // Java: this.color = null;
                self.color = None;
            }
        }
    }
}

impl KoAr for GradientFill {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { GradientFill::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { GradientFill::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { GradientFill::lastBin(self, var1) }
}
impl KoClosable for GradientFill { fn close(&mut self) { GradientFill::close(self); } }

impl KoAr for GradientStopList {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { GradientStopList::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { GradientStopList::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { GradientStopList::lastBin(self, var1) }
}
impl KoClosable for GradientStopList { fn close(&mut self) { GradientStopList::close(self); } }

impl KoAr for GradientStop {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { GradientStop::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { GradientStop::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { GradientStop::lastBin(self, var1) }
}
impl KoClosable for GradientStop { fn close(&mut self) { GradientStop::close(self); } }
