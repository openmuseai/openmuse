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
// Java: import cn.wps.kso.kproto.internal.KpList
use crate::cn::wps::kso::kproto::internal::KpList::KpList;
// Java: import cn.wps.kso.kproto.internal.KpMap
use crate::cn::wps::kso::kproto::internal::KpMap::KpMap;
// Java: import java.util.ArrayList
// Java: import java.util.Collection

// Java: public class RecolorColorList
pub struct RecolorColorList {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
    // Java field: KoZone zone
    pub(crate) zone: KoZone,
}

// Java: public class RecolorFillList
pub struct RecolorFillList {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
    // Java field: KoZone zone
    pub(crate) zone: KoZone,
}

// Java: public class RecolorInfo implements KoAr, KoClosable
pub struct RecolorInfo {
    // Java field: KoZone zone
    pub(crate) zone: KoZone,
    // Java field: Color monoColor
    pub(crate) monoColor: Option<Box<Color>>,
    // Java field: RecolorColorList colors
    pub(crate) colors: Option<Box<RecolorColorList>>,
    // Java field: RecolorFillList fills
    pub(crate) fills: Option<Box<RecolorFillList>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: RecolorInfo parent
    pub(crate) parent: Option<Box<RecolorInfo>>,
}

// Java: public class RecolorInfo implements KoAr, KoClosable
impl RecolorInfo {
    // Java: static protected static <Value1 extends RecolorInfo> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = RecolorInfo::RecolorInfo();
        // Java: inst.zone = zone;
        inst.zone = *zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static RecolorInfo create()
    pub fn create() -> Self {
        // Java: return initialize(RecolorInfo.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static RecolorInfo open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(RecolorInfo.class, bin);
        Self::initialize(bin)
    }

    // Java: RecolorInfo()
    pub fn RecolorInfo() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        RecolorInfo {
            zone: KoZone,
            monoColor: None,
            colors: None,
            fills: None,
            bin_box: KpBinBox::new(),
            map: KpMap::new(),
            parent: None,
        }
    }

    // Java: RecolorInfo(KoBin bin)
    pub fn RecolorInfo_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        RecolorInfo {
            zone: KoZone,
            monoColor: None,
            colors: None,
            fills: None,
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
        self.loadBin(bin, &KoZone);
    }

    // Java: void load(RecolorInfo value)
    pub fn load_from(&mut self, value: Option<&RecolorInfo>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(&KoZone);
            self.loadBin(last.as_ref(), &KoZone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.monoColor != null && this.monoColor.dirty())
        if let Some(ref mut mc) = self.monoColor {
            if mc.dirty() {
                // Java: this.map.put(1, this.monoColor.saveBin(zone));
                let bin = mc.saveBin(zone);
                self.map.put(1, bin);
            }
        }
        // Java: if (this.colors != null && this.colors.dirty())
        if let Some(ref mut c) = self.colors {
            if c.dirty() {
                // Java: this.map.put(2, this.colors.saveBin(zone));
                let bin = c.saveBin(zone);
                self.map.put(2, bin);
            }
        }
        // Java: if (this.fills != null && this.fills.dirty())
        if let Some(ref mut f) = self.fills {
            if f.dirty() {
                // Java: this.map.put(3, this.fills.saveBin(zone));
                let bin = f.saveBin(zone);
                self.map.put(3, bin);
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

    // Java: boolean equalsLax(RecolorInfo o)
    pub fn equalsLax(&self, o: &RecolorInfo) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &RecolorInfo) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &RecolorInfo, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof RecolorInfo)
        if true {
        // Java: RecolorInfo dst = (RecolorInfo)o;
        let dst = o;
        // Java: if (this == dst)
        if std::ptr::eq(self, dst) {
            // Java: return true;
            return true;
        }
        // Java: else if ((this.hasMonoColor() || !dst.hasMonoColor()) && (!this.hasMonoColor() || dst.hasMonoColor()))
        else if (self.hasMonoColor() || !dst.hasMonoColor()) && (!self.hasMonoColor() || dst.hasMonoColor()) {
            // Java: if (this.hasMonoColor() && dst.hasMonoColor())
            if self.hasMonoColor() && dst.hasMonoColor() {
                // Java: if (isStrict && !this.monoColor().equals(dst.monoColor()))
                if isStrict {
                    // Java: return false;
                }
                // Java: if (!isStrict && !this.monoColor().equalsLax(dst.monoColor()))
                if !isStrict {
                    // Java: return false;
                }
            }
            // Java: if ((this.hasColors() || !dst.hasColors()) && (!this.hasColors() || dst.hasColors()))
            if (self.hasColors() || !dst.hasColors()) && (!self.hasColors() || dst.hasColors()) {
                // Java: if (this.hasColors() && dst.hasColors())
                if self.hasColors() && dst.hasColors() {
                    // Java: if (isStrict && !this.colors().equals(dst.colors()))
                    if isStrict {
                        // Java: return false;
                    }
                    // Java: if (!isStrict && !this.colors().equalsLax(dst.colors()))
                    if !isStrict {
                        // Java: return false;
                    }
                }
                // Java: if ((this.hasFills() || !dst.hasFills()) && (!this.hasFills() || dst.hasFills()))
                if (self.hasFills() || !dst.hasFills()) && (!self.hasFills() || dst.hasFills()) {
                    // Java: if (this.hasFills() && dst.hasFills())
                    if self.hasFills() && dst.hasFills() {
                        // Java: if (isStrict && !this.fills().equals(dst.fills()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.fills().equalsLax(dst.fills()))
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
        // Java: if (this.hasMonoColor())
        if self.hasMonoColor() {
            // Java: ret += this.monoColor().hashCode();
            ret = ret.wrapping_add(0);
        }
        // Java: if (this.hasColors())
        if self.hasColors() {
            // Java: ret += this.colors().hashCode();
            ret = ret.wrapping_add(0);
        }
        // Java: if (this.hasFills())
        if self.hasFills() {
            // Java: ret += this.fills().hashCode();
            ret = ret.wrapping_add(0);
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: if (this.monoColor != null && this.monoColor.dirty())
        if let Some(ref mc) = self.monoColor {
            if mc.dirty() {
                // Java: return true;
                return true;
            }
        }
        // Java: else if (this.colors != null && this.colors.dirty())
        if let Some(ref c) = self.colors {
            if c.dirty() {
                // Java: return true;
                return true;
            }
        }
        // Java: else
        // Java: return this.fills != null && this.fills.dirty() ? true : this.map.dirty();
        if let Some(ref f) = self.fills {
            if f.dirty() {
                return true;
            }
        }
        self.map.dirty()
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
        // Java: this.saveBin(this.zone);
        self.saveBin(&KoZone);
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
        // Java: this.monoColor = null;
        self.monoColor = None;
        // Java: this.colors = null;
        self.colors = None;
        // Java: this.fills = null;
        self.fills = None;
    }

    // Java: void follow(RecolorInfo parent)
    pub fn follow(&mut self, parent: Option<Box<RecolorInfo>>) {
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

    // Java: RecolorInfo parent()
    pub fn parent_ref(&self) -> Option<&RecolorInfo> {
        // Java: return this.parent;
        self.parent.as_deref()
    }

    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(&KoZone);
    }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None;
    }

    // Java: Color monoColor()
    pub fn monoColor(&mut self) -> &mut Color {
        // Java: if (this.monoColor == null)
        if self.monoColor.is_none() {
            // Java: if (this.map.has(1))
            if self.map.has(1) {
                // Java: KoBin bin = this.map.byKey(1);
                let bin = self.map.byKey(1);
                // Java: this.monoColor = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
                self.monoColor = Some(Box::new(Color::open(bin.as_ref())));
            }
            // Java: else if (this.parent != null && this.parent.hasMonoColor())
            else if self.parent.as_ref().map_or(false, |p| p.hasMonoColor()) {
                // Java: this.monoColor = this.parent.monoColor();
                let bin = self.parent.as_ref().unwrap().lastBin(&KoZone);
                self.monoColor = Some(Box::new(Color::open(bin.as_ref())));
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.monoColor = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
                self.monoColor = Some(Box::new(Color::open(bin.as_ref())));
            }
        }
        // Java: return this.monoColor;
        self.monoColor.as_mut().unwrap()
    }

    // Java: boolean hasMonoColor()
    pub fn hasMonoColor(&self) -> bool {
        // Java: return this.map.has(1) || this.monoColor != null && this.monoColor.dirty() || this.parent != null && this.parent.hasMonoColor();
        self.map.has(1)
            || self.monoColor.as_ref().map_or(false, |mc| mc.dirty())
            || self.parent.as_ref().map_or(false, |p| p.hasMonoColor())
    }

    // Java: void deleteMonoColor()
    pub fn deleteMonoColor(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(1);
        // Java: if (this.monoColor != null)
        if self.monoColor.is_some() {
            // Java: this.monoColor = null;
            self.monoColor = None;
        }
    }

    // Java: void writeMonoColor(Color value)
    pub fn writeMonoColor(&mut self, value: Option<&Color>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = val.lastBin(&KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(1, bin);
                self.map.put(1, bin);
                // Java: if (this.monoColor != null)
                if self.monoColor.is_some() {
                    // Java: this.monoColor = null;
                    self.monoColor = None;
                }
            }
        }
    }

    // Java: void forceWriteMonoColor(Color value)
    pub fn forceWriteMonoColor(&mut self, value: Option<&Color>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(1, value.lastBin(this.zone));
            let bin = val.lastBin(&KoZone);
            self.map.put(1, bin);
            // Java: if (this.monoColor != null)
            if self.monoColor.is_some() {
                // Java: this.monoColor = null;
                self.monoColor = None;
            }
        }
    }

    // Java: RecolorColorList colors()
    pub fn colors(&mut self) -> &mut RecolorColorList {
        // Java: if (this.colors == null)
        if self.colors.is_none() {
            // Java: if (this.map.has(2))
            if self.map.has(2) {
                // Java: KoBin bin = this.map.byKey(2);
                let bin = self.map.byKey(2);
                // Java: this.colors = (RecolorColorList)KoBin.Decode.toAny(RecolorColorList.class, bin, this.zone);
                self.colors = Some(Box::new(RecolorColorList::open(bin.as_ref())));
            }
            // Java: else if (this.parent != null && this.parent.hasColors())
            else if self.parent.as_ref().map_or(false, |p| p.hasColors()) {
                // Java: this.colors = this.parent.colors();
                let bin = Null::value1();
                self.colors = Some(Box::new(RecolorColorList::open(bin.as_ref())));
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.colors = (RecolorColorList)KoBin.Decode.toAny(RecolorColorList.class, bin, this.zone);
                self.colors = Some(Box::new(RecolorColorList::open(bin.as_ref())));
            }
        }
        // Java: return this.colors;
        self.colors.as_mut().unwrap()
    }

    // Java: boolean hasColors()
    pub fn hasColors(&self) -> bool {
        // Java: return this.map.has(2) || this.colors != null && this.colors.dirty() || this.parent != null && this.parent.hasColors();
        self.map.has(2)
            || self.colors.as_ref().map_or(false, |c| c.dirty())
            || self.parent.as_ref().map_or(false, |p| p.hasColors())
    }

    // Java: void deleteColors()
    pub fn deleteColors(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(2);
        // Java: if (this.colors != null)
        if self.colors.is_some() {
            // Java: this.colors = null;
            self.colors = None;
        }
    }

    // Java: void writeColors(RecolorColorList value)
    pub fn writeColors(&mut self, value: Option<&RecolorColorList>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = val.lastBin(&KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(2, bin);
                self.map.put(2, bin);
                // Java: if (this.colors != null)
                if self.colors.is_some() {
                    // Java: this.colors = null;
                    self.colors = None;
                }
            }
        }
    }

    // Java: void forceWriteColors(RecolorColorList value)
    pub fn forceWriteColors(&mut self, value: Option<&RecolorColorList>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(2, value.lastBin(this.zone));
            let bin = val.lastBin(&KoZone);
            self.map.put(2, bin);
            // Java: if (this.colors != null)
            if self.colors.is_some() {
                // Java: this.colors = null;
                self.colors = None;
            }
        }
    }

    // Java: RecolorFillList fills()
    pub fn fills(&mut self) -> &mut RecolorFillList {
        // Java: if (this.fills == null)
        if self.fills.is_none() {
            // Java: if (this.map.has(3))
            if self.map.has(3) {
                // Java: KoBin bin = this.map.byKey(3);
                let bin = self.map.byKey(3);
                // Java: this.fills = (RecolorFillList)KoBin.Decode.toAny(RecolorFillList.class, bin, this.zone);
                self.fills = Some(Box::new(RecolorFillList::open(bin.as_ref())));
            }
            // Java: else if (this.parent != null && this.parent.hasFills())
            else if self.parent.as_ref().map_or(false, |p| p.hasFills()) {
                // Java: this.fills = this.parent.fills();
                let bin = Null::value1();
                self.fills = Some(Box::new(RecolorFillList::open(bin.as_ref())));
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.fills = (RecolorFillList)KoBin.Decode.toAny(RecolorFillList.class, bin, this.zone);
                self.fills = Some(Box::new(RecolorFillList::open(bin.as_ref())));
            }
        }
        // Java: return this.fills;
        self.fills.as_mut().unwrap()
    }

    // Java: boolean hasFills()
    pub fn hasFills(&self) -> bool {
        // Java: return this.map.has(3) || this.fills != null && this.fills.dirty() || this.parent != null && this.parent.hasFills();
        self.map.has(3)
            || self.fills.as_ref().map_or(false, |f| f.dirty())
            || self.parent.as_ref().map_or(false, |p| p.hasFills())
    }

    // Java: void deleteFills()
    pub fn deleteFills(&mut self) {
        // Java: this.map.put(3, (KoBin)null);
        self.map.clear_key(3);
        // Java: if (this.fills != null)
        if self.fills.is_some() {
            // Java: this.fills = null;
            self.fills = None;
        }
    }

    // Java: void writeFills(RecolorFillList value)
    pub fn writeFills(&mut self, value: Option<&RecolorFillList>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = val.lastBin(&KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(3, bin);
                self.map.put(3, bin);
                // Java: if (this.fills != null)
                if self.fills.is_some() {
                    // Java: this.fills = null;
                    self.fills = None;
                }
            }
        }
    }

    // Java: void forceWriteFills(RecolorFillList value)
    pub fn forceWriteFills(&mut self, value: Option<&RecolorFillList>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(3, value.lastBin(this.zone));
            let bin = val.lastBin(&KoZone);
            self.map.put(3, bin);
            // Java: if (this.fills != null)
            if self.fills.is_some() {
                // Java: this.fills = null;
                self.fills = None;
            }
        }
    }
}

// Java: public class RecolorColorList
impl RecolorColorList {
    // Java: RecolorColorList()
    pub fn new() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.list = new KpList(this.zone);
        RecolorColorList {
            bin_box: KpBinBox::new(),
            list: KpList::new(&KoZone),
            zone: KoZone,
        }
    }

    // Java: RecolorColorList(KoBin bin)
    pub fn RecolorColorList_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.list = new KpList(bin, this.zone);
        RecolorColorList {
            bin_box: KpBinBox::KpBinBox_from(Some(bin)),
            list: KpList::KpList_from(bin, &KoZone),
            zone: KoZone,
        }
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
        self.loadBin(bin, &KoZone);
    }

    // Java: void load(RecolorColorList value)
    pub fn load_from(&mut self, value: Option<&RecolorColorList>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(&KoZone);
            self.loadBin(last.as_ref(), &KoZone);
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

    // Java: boolean equalsLax(RecolorColorList o)
    pub fn equalsLax(&self, o: &RecolorColorList) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &RecolorColorList) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &RecolorColorList, _isStrict: bool) -> bool {
        // Java: if (null == o)
        // Java: return false;
        // Java: else
        // Java: RecolorColorList dst = (RecolorColorList)o;
        let dst = o;
        // Java: if (this == dst)
        if std::ptr::eq(self, dst) {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: ArrayList<RecolorEntryColor> dstList = new ArrayList();
            // Java: dst.read(dstList);
            // Java: ArrayList<RecolorEntryColor> srcList = new ArrayList();
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
                // Java: RecolorEntryColor dstClrTrans = (RecolorEntryColor)dstList.get(i);
                // Java: RecolorEntryColor srcClrTrans = (RecolorEntryColor)srcList.get(i);
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
        self.saveBin(&KoZone);
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
        self.list.load(n.as_ref(), &KoZone);
        // Java: this.zone.leave();
        self.zone.leave();
        // Java: this.zone = KoZone.defaultZone();
        self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends RecolorColorList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = RecolorColorList::new();
        // Java: inst.zone = zone;
        inst.zone = *zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static RecolorColorList create()
    pub fn create() -> Self {
        // Java: return initialize(RecolorColorList.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static RecolorColorList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(RecolorColorList.class, bin);
        Self::initialize(bin)
    }

    // Java: void read(Collection<RecolorEntryColor> valueLst)
    pub fn read(&self, valueLst: &mut Vec<Box<dyn KoBinTrait>>) {
        // Java: valueLst.clear();
        valueLst.clear();
        // Java: int size = this.list.size();
        let size = self.list.size();
        // Java: for(int i = 0; i < size; ++i)
        for i in 0..size {
            // Java: valueLst.add((RecolorEntryColor)KoBin.Decode.toAny(RecolorEntryColor.class, this.list.get(i), this.zone));
            let item = Factory::dup(self.list.get(i), &KoZone);
            valueLst.push(item);
        }
    }

    // Java: void write(Iterable<RecolorEntryColor> valueLst)
    pub fn write(&mut self, valueLst: &[Box<dyn KoBinTrait>]) {
        // Java: this.list.clear();
        self.list.clear();
        // Java: for(RecolorEntryColor i : valueLst)
        for item in valueLst.iter() {
            // Java: KoBin bin = i.lastBin(this.zone);
            let bin = Factory::dup(item.as_ref(), &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.list.add(bin);
                self.list.add(bin);
            }
        }
    }

    // Java: void forceWrite(Iterable<RecolorEntryColor> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[Box<dyn KoBinTrait>]) {
        // Java: this.list.clear();
        self.list.clear();
        // Java: for(RecolorEntryColor i : valueLst)
        for item in valueLst.iter() {
            // Java: this.list.add(i.lastBin(this.zone));
            let bin = Factory::dup(item.as_ref(), &KoZone);
            self.list.add(bin);
        }
    }
}

// Java: public class RecolorFillList
impl RecolorFillList {
    // Java: RecolorFillList()
    pub fn new() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.list = new KpList(this.zone);
        RecolorFillList {
            bin_box: KpBinBox::new(),
            list: KpList::new(&KoZone),
            zone: KoZone,
        }
    }

    // Java: RecolorFillList(KoBin bin)
    pub fn RecolorFillList_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.list = new KpList(bin, this.zone);
        RecolorFillList {
            bin_box: KpBinBox::KpBinBox_from(Some(bin)),
            list: KpList::KpList_from(bin, &KoZone),
            zone: KoZone,
        }
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
        self.loadBin(bin, &KoZone);
    }

    // Java: void load(RecolorFillList value)
    pub fn load_from(&mut self, value: Option<&RecolorFillList>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(&KoZone);
            self.loadBin(last.as_ref(), &KoZone);
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

    // Java: boolean equalsLax(RecolorFillList o)
    pub fn equalsLax(&self, o: &RecolorFillList) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &RecolorFillList) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &RecolorFillList, _isStrict: bool) -> bool {
        // Java: if (null == o)
        // Java: return false;
        // Java: else
        // Java: RecolorFillList dst = (RecolorFillList)o;
        let dst = o;
        // Java: if (this == dst)
        if std::ptr::eq(self, dst) {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: ArrayList<RecolorEntryBlush> dstList = new ArrayList();
            // Java: dst.read(dstList);
            // Java: ArrayList<RecolorEntryBlush> srcList = new ArrayList();
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
                // Java: RecolorEntryBlush dstClrTrans = (RecolorEntryBlush)dstList.get(i);
                // Java: RecolorEntryBlush srcClrTrans = (RecolorEntryBlush)srcList.get(i);
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
        self.saveBin(&KoZone);
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
        self.list.load(n.as_ref(), &KoZone);
        // Java: this.zone.leave();
        self.zone.leave();
        // Java: this.zone = KoZone.defaultZone();
        self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends RecolorFillList> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = RecolorFillList::new();
        // Java: inst.zone = zone;
        inst.zone = *zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static RecolorFillList create()
    pub fn create() -> Self {
        // Java: return initialize(RecolorFillList.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static RecolorFillList open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(RecolorFillList.class, bin);
        Self::initialize(bin)
    }

    // Java: void read(Collection<RecolorEntryBlush> valueLst)
    pub fn read(&self, valueLst: &mut Vec<Box<dyn KoBinTrait>>) {
        // Java: valueLst.clear();
        valueLst.clear();
        // Java: int size = this.list.size();
        let size = self.list.size();
        // Java: for(int i = 0; i < size; ++i)
        for i in 0..size {
            // Java: valueLst.add((RecolorEntryBlush)KoBin.Decode.toAny(RecolorEntryBlush.class, this.list.get(i), this.zone));
            let item = Factory::dup(self.list.get(i), &KoZone);
            valueLst.push(item);
        }
    }

    // Java: void write(Iterable<RecolorEntryBlush> valueLst)
    pub fn write(&mut self, valueLst: &[Box<dyn KoBinTrait>]) {
        // Java: this.list.clear();
        self.list.clear();
        // Java: for(RecolorEntryBlush i : valueLst)
        for item in valueLst.iter() {
            // Java: KoBin bin = i.lastBin(this.zone);
            let bin = Factory::dup(item.as_ref(), &KoZone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.list.add(bin);
                self.list.add(bin);
            }
        }
    }

    // Java: void forceWrite(Iterable<RecolorEntryBlush> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[Box<dyn KoBinTrait>]) {
        // Java: this.list.clear();
        self.list.clear();
        // Java: for(RecolorEntryBlush i : valueLst)
        for item in valueLst.iter() {
            // Java: this.list.add(i.lastBin(this.zone));
            let bin = Factory::dup(item.as_ref(), &KoZone);
            self.list.add(bin);
        }
    }
}

impl KoAr for RecolorInfo {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { RecolorInfo::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { RecolorInfo::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { RecolorInfo::lastBin(self, var1) }
}

impl KoClosable for RecolorInfo {
    fn close(&mut self) { RecolorInfo::close(self); }
}

impl KoAr for RecolorColorList {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { RecolorColorList::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { RecolorColorList::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { RecolorColorList::lastBin(self, var1) }
}

impl KoClosable for RecolorColorList {
    fn close(&mut self) { RecolorColorList::close(self); }
}

impl KoAr for RecolorFillList {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { RecolorFillList::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { RecolorFillList::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { RecolorFillList::lastBin(self, var1) }
}

impl KoClosable for RecolorFillList {
    fn close(&mut self) { RecolorFillList::close(self); }
}
