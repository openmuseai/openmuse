#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.basic.RelativeRect
// Java: import cn.wps.drawing.data.blip.Blip
// Java: import cn.wps.drawing.data.color.Color
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

// Java: public class BlipFill implements KoAr, KoClosable
pub struct BlipFill {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Blip blip
    pub(crate) blip: Option<Box<dyn KoBinTrait>>,
    // Java field: RelativeRect srcRect
    pub(crate) srcRect: Option<Box<dyn KoBinTrait>>,
    // Java field: FillMode fillMode
    pub(crate) fillMode: Option<Box<dyn KoBinTrait>>,
    // Java field: RecolorInfo recolor
    pub(crate) recolor: Option<Box<dyn KoBinTrait>>,
    // Java field: Color mainClr
    pub(crate) mainClr: Option<Box<dyn KoBinTrait>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: BlipFill parent
    pub(crate) parent: Option<Box<BlipFill>>,
}

// Java: public class BlipFill implements KoAr, KoClosable
impl BlipFill {
    // Java: static protected static <Value1 extends BlipFill> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = BlipFill::BlipFill();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static BlipFill create()
    pub fn create() -> Self {
        // Java: return initialize(BlipFill.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static BlipFill open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(BlipFill.class, bin);
        Self::initialize(bin)
    }

    // Java: BlipFill()
    pub fn BlipFill() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        BlipFill { zone: KoZone::enter(), blip: None, srcRect: None, fillMode: None, recolor: None, mainClr: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: BlipFill(KoBin bin)
    pub fn BlipFill_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        BlipFill { zone: KoZone::enter(), blip: None, srcRect: None, fillMode: None, recolor: None, mainClr: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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

    // Java: void load(BlipFill value)
    pub fn load_from(&mut self, value: Option<&BlipFill>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(KoZone::enter());
            self.loadBin(last.as_ref(), KoZone::enter());
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.blip != null && this.blip.dirty())
        // Java: this.map.put(3, this.blip.saveBin(zone));
        // Java: if (this.srcRect != null && this.srcRect.dirty())
        // Java: this.map.put(4, this.srcRect.saveBin(zone));
        // Java: if (this.fillMode != null && this.fillMode.dirty())
        // Java: this.map.put(5, this.fillMode.saveBin(zone));
        // Java: if (this.recolor != null && this.recolor.dirty())
        // Java: this.map.put(6, this.recolor.saveBin(zone));
        // Java: if (this.mainClr != null && this.mainClr.dirty())
        // Java: this.map.put(7, this.mainClr.saveBin(zone));
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            saved
        } else {
            self.bin_box.get()
        }
    }

    // Java: boolean equalsLax(BlipFill o)
    pub fn equalsLax(&self, o: &BlipFill) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &BlipFill) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &BlipFill, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof BlipFill)
        if true {
        // Java: BlipFill dst = (BlipFill)o;
        let dst = o;
        // Java: if (this == dst)
        if std::ptr::eq(self, dst) {
            // Java: return true;
            return true;
        }
        // Java: else if ((this.hasDpi() || !dst.hasDpi()) && (!this.hasDpi() || dst.hasDpi()))
        else if (self.hasDpi() || !dst.hasDpi()) && (!self.hasDpi() || dst.hasDpi()) {
            // Java: if (this.hasDpi() && dst.hasDpi() && this.dpi() != dst.dpi())
            if self.hasDpi() && dst.hasDpi() && self.dpi() != dst.dpi() {
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
                // Java: else if ((this.hasBlip() || !dst.hasBlip()) && (!this.hasBlip() || dst.hasBlip()))
                else if (self.hasBlip() || !dst.hasBlip()) && (!self.hasBlip() || dst.hasBlip()) {
                    // Java: if (this.hasBlip() && dst.hasBlip())
                    if self.hasBlip() && dst.hasBlip() {
                        // Java: if (isStrict && !this.blip().equals(dst.blip()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.blip().equalsLax(dst.blip()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                    // Java: if ((this.hasSrcRect() || !dst.hasSrcRect()) && (!this.hasSrcRect() || dst.hasSrcRect()))
                    if (self.hasSrcRect() || !dst.hasSrcRect()) && (!self.hasSrcRect() || dst.hasSrcRect()) {
                        // Java: if (this.hasSrcRect() && dst.hasSrcRect())
                        if self.hasSrcRect() && dst.hasSrcRect() {
                            // Java: if (isStrict && !this.srcRect().equals(dst.srcRect()))
                            if isStrict {
                                // Java: return false;
                            }
                            // Java: if (!isStrict && !this.srcRect().equalsLax(dst.srcRect()))
                            if !isStrict {
                                // Java: return false;
                            }
                        }
                        // Java: if ((this.hasFillMode() || !dst.hasFillMode()) && (!this.hasFillMode() || dst.hasFillMode()))
                        if (self.hasFillMode() || !dst.hasFillMode()) && (!self.hasFillMode() || dst.hasFillMode()) {
                            // Java: if (this.hasFillMode() && dst.hasFillMode())
                            if self.hasFillMode() && dst.hasFillMode() {
                                // Java: if (isStrict && !this.fillMode().equals(dst.fillMode()))
                                if isStrict {
                                    // Java: return false;
                                }
                                // Java: if (!isStrict && !this.fillMode().equalsLax(dst.fillMode()))
                                if !isStrict {
                                    // Java: return false;
                                }
                            }
                            // Java: if ((this.hasRecolor() || !dst.hasRecolor()) && (!this.hasRecolor() || dst.hasRecolor()))
                            if (self.hasRecolor() || !dst.hasRecolor()) && (!self.hasRecolor() || dst.hasRecolor()) {
                                // Java: if (this.hasRecolor() && dst.hasRecolor())
                                if self.hasRecolor() && dst.hasRecolor() {
                                    // Java: if (isStrict && !this.recolor().equals(dst.recolor()))
                                    if isStrict {
                                        // Java: return false;
                                    }
                                    // Java: if (!isStrict && !this.recolor().equalsLax(dst.recolor()))
                                    if !isStrict {
                                        // Java: return false;
                                    }
                                }
                                // Java: if ((this.hasMainClr() || !dst.hasMainClr()) && (!this.hasMainClr() || dst.hasMainClr()))
                                if (self.hasMainClr() || !dst.hasMainClr()) && (!self.hasMainClr() || dst.hasMainClr()) {
                                    // Java: if (this.hasMainClr() && dst.hasMainClr())
                                    if self.hasMainClr() && dst.hasMainClr() {
                                        // Java: if (isStrict && !this.mainClr().equals(dst.mainClr()))
                                        if isStrict {
                                            // Java: return false;
                                        }
                                        // Java: if (!isStrict && !this.mainClr().equalsLax(dst.mainClr()))
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
            false
        }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int ret = 0;
        let mut ret: i32 = 0;
        // Java: if (this.hasDpi())
        if self.hasDpi() {
            // Java: ret += Integer.valueOf(this.dpi()).hashCode();
            ret = ret.wrapping_add(self.dpi());
        }
        // Java: if (this.hasRotWithShape())
        if self.hasRotWithShape() {
            // Java: ret += Boolean.valueOf(this.isRotWithShape()).hashCode();
            ret = ret.wrapping_add(if self.isRotWithShape() { 1231 } else { 1237 });
        }
        // Java: if (this.hasBlip())
        if self.hasBlip() {
            // Java: ret += this.blip().hashCode();
        }
        // Java: if (this.hasSrcRect())
        if self.hasSrcRect() {
            // Java: ret += this.srcRect().hashCode();
        }
        // Java: if (this.hasFillMode())
        if self.hasFillMode() {
            // Java: ret += this.fillMode().hashCode();
        }
        // Java: if (this.hasRecolor())
        if self.hasRecolor() {
            // Java: ret += this.recolor().hashCode();
        }
        // Java: if (this.hasMainClr())
        if self.hasMainClr() {
            // Java: ret += this.mainClr().hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: if (this.blip != null && this.blip.dirty())
        if self.blip.is_some() {
            // Java: return true;
            return true;
        }
        // Java: else if (this.srcRect != null && this.srcRect.dirty())
        else if self.srcRect.is_some() {
            // Java: return true;
            return true;
        }
        // Java: else if (this.fillMode != null && this.fillMode.dirty())
        else if self.fillMode.is_some() {
            // Java: return true;
            return true;
        }
        // Java: else if (this.recolor != null && this.recolor.dirty())
        else if self.recolor.is_some() {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: return this.mainClr != null && this.mainClr.dirty() ? true : this.map.dirty();
            if self.mainClr.is_some() { return true; }
            self.map.dirty()
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
        // Java: this.blip = null;
        self.blip = None;
        // Java: this.srcRect = null;
        self.srcRect = None;
        // Java: this.fillMode = null;
        self.fillMode = None;
        // Java: this.recolor = null;
        self.recolor = None;
        // Java: this.mainClr = null;
        self.mainClr = None;
    }

    // Java: void follow(BlipFill parent)
    pub fn follow(&mut self, parent: Option<Box<BlipFill>>) {
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

    // Java: BlipFill parent()
    pub fn parent_ref(&self) -> Option<&BlipFill> {
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

    // Java: int dpi()
    pub fn dpi(&self) -> i32 {
        // Java: if (!this.map.has(1))
        if !self.map.has(1) {
            // Java: return this.parent == null ? 0 : this.parent.dpi();
            match &self.parent { None => 0, Some(p) => p.dpi() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(1);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasDpi()
    pub fn hasDpi(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasDpi();
        self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasDpi())
    }

    // Java: void deleteDpi()
    pub fn deleteDpi(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(1);
    }

    // Java: void putDpi(int value)
    pub fn putDpi(&mut self, value: i32) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_int(value, KoZone::enter());
        self.map.put(1, bin);
    }

    // Java: boolean isRotWithShape()
    pub fn isRotWithShape(&self) -> bool {
        // Java: if (!this.map.has(2))
        if !self.map.has(2) {
            // Java: return this.parent == null ? false : this.parent.isRotWithShape();
            match &self.parent { None => false, Some(p) => p.isRotWithShape() }
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

    // Java: Blip blip()
    pub fn blip(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.blip == null)
        if self.blip.is_none() {
            // Java: if (this.map.has(3))
            if self.map.has(3) {
                // Java: KoBin bin = this.map.byKey(3);
                let bin = self.map.byKey(3);
                // Java: this.blip = (Blip)KoBin.Decode.toAny(Blip.class, bin, this.zone);
                self.blip = Some(Factory::dup(bin.as_ref(), KoZone::enter()));
            }
            // Java: else if (this.parent != null && this.parent.hasBlip())
            else if self.parent.as_ref().map_or(false, |p| p.hasBlip()) {
                // Java: this.blip = this.parent.blip();
                let bin = Null::value1();
                self.blip = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.blip = (Blip)KoBin.Decode.toAny(Blip.class, bin, this.zone);
                self.blip = Some(bin);
            }
        }
        // Java: return this.blip;
        self.blip.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasBlip()
    pub fn hasBlip(&self) -> bool {
        // Java: return this.map.has(3) || this.blip != null && this.blip.dirty() || this.parent != null && this.parent.hasBlip();
        self.map.has(3) || self.blip.is_some() || self.parent.as_ref().map_or(false, |p| p.hasBlip())
    }

    // Java: void deleteBlip()
    pub fn deleteBlip(&mut self) {
        // Java: this.map.put(3, (KoBin)null);
        self.map.clear_key(3);
        // Java: if (this.blip != null)
        if self.blip.is_some() {
            // Java: this.blip = null;
            self.blip = None;
        }
    }

    // Java: void writeBlip(Blip value)
    pub fn writeBlip(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, KoZone::enter());
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(3, bin);
                self.map.put(3, bin);
                // Java: if (this.blip != null)
                if self.blip.is_some() {
                    // Java: this.blip = null;
                    self.blip = None;
                }
            }
        }
    }

    // Java: void forceWriteBlip(Blip value)
    pub fn forceWriteBlip(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(3, value.lastBin(this.zone));
            let bin = Factory::dup(val, KoZone::enter());
            self.map.put(3, bin);
            // Java: if (this.blip != null)
            if self.blip.is_some() {
                // Java: this.blip = null;
                self.blip = None;
            }
        }
    }

    // Java: RelativeRect srcRect()
    pub fn srcRect(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.srcRect == null)
        if self.srcRect.is_none() {
            // Java: if (this.map.has(4))
            if self.map.has(4) {
                // Java: KoBin bin = this.map.byKey(4);
                let bin = self.map.byKey(4);
                // Java: this.srcRect = (RelativeRect)KoBin.Decode.toAny(RelativeRect.class, bin, this.zone);
                self.srcRect = Some(Factory::dup(bin.as_ref(), KoZone::enter()));
            }
            // Java: else if (this.parent != null && this.parent.hasSrcRect())
            else if self.parent.as_ref().map_or(false, |p| p.hasSrcRect()) {
                // Java: this.srcRect = this.parent.srcRect();
                let bin = Null::value1();
                self.srcRect = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.srcRect = (RelativeRect)KoBin.Decode.toAny(RelativeRect.class, bin, this.zone);
                self.srcRect = Some(bin);
            }
        }
        // Java: return this.srcRect;
        self.srcRect.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasSrcRect()
    pub fn hasSrcRect(&self) -> bool {
        // Java: return this.map.has(4) || this.srcRect != null && this.srcRect.dirty() || this.parent != null && this.parent.hasSrcRect();
        self.map.has(4) || self.srcRect.is_some() || self.parent.as_ref().map_or(false, |p| p.hasSrcRect())
    }

    // Java: void deleteSrcRect()
    pub fn deleteSrcRect(&mut self) {
        // Java: this.map.put(4, (KoBin)null);
        self.map.clear_key(4);
        // Java: if (this.srcRect != null)
        if self.srcRect.is_some() {
            // Java: this.srcRect = null;
            self.srcRect = None;
        }
    }

    // Java: void writeSrcRect(RelativeRect value)
    pub fn writeSrcRect(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, KoZone::enter());
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(4, bin);
                self.map.put(4, bin);
                // Java: if (this.srcRect != null)
                if self.srcRect.is_some() {
                    // Java: this.srcRect = null;
                    self.srcRect = None;
                }
            }
        }
    }

    // Java: void forceWriteSrcRect(RelativeRect value)
    pub fn forceWriteSrcRect(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(4, value.lastBin(this.zone));
            let bin = Factory::dup(val, KoZone::enter());
            self.map.put(4, bin);
            // Java: if (this.srcRect != null)
            if self.srcRect.is_some() {
                // Java: this.srcRect = null;
                self.srcRect = None;
            }
        }
    }

    // Java: FillMode fillMode()
    pub fn fillMode(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.fillMode == null)
        if self.fillMode.is_none() {
            // Java: if (this.map.has(5))
            if self.map.has(5) {
                // Java: KoBin bin = this.map.byKey(5);
                let bin = self.map.byKey(5);
                // Java: this.fillMode = (FillMode)KoBin.Decode.toAny(FillMode.class, bin, this.zone);
                self.fillMode = Some(Factory::dup(bin.as_ref(), KoZone::enter()));
            }
            // Java: else if (this.parent != null && this.parent.hasFillMode())
            else if self.parent.as_ref().map_or(false, |p| p.hasFillMode()) {
                // Java: this.fillMode = this.parent.fillMode();
                let bin = Null::value1();
                self.fillMode = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.fillMode = (FillMode)KoBin.Decode.toAny(FillMode.class, bin, this.zone);
                self.fillMode = Some(bin);
            }
        }
        // Java: return this.fillMode;
        self.fillMode.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasFillMode()
    pub fn hasFillMode(&self) -> bool {
        // Java: return this.map.has(5) || this.fillMode != null && this.fillMode.dirty() || this.parent != null && this.parent.hasFillMode();
        self.map.has(5) || self.fillMode.is_some() || self.parent.as_ref().map_or(false, |p| p.hasFillMode())
    }

    // Java: void deleteFillMode()
    pub fn deleteFillMode(&mut self) {
        // Java: this.map.put(5, (KoBin)null);
        self.map.clear_key(5);
        // Java: if (this.fillMode != null)
        if self.fillMode.is_some() {
            // Java: this.fillMode = null;
            self.fillMode = None;
        }
    }

    // Java: void writeFillMode(FillMode value)
    pub fn writeFillMode(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, KoZone::enter());
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(5, bin);
                self.map.put(5, bin);
                // Java: if (this.fillMode != null)
                if self.fillMode.is_some() {
                    // Java: this.fillMode = null;
                    self.fillMode = None;
                }
            }
        }
    }

    // Java: void forceWriteFillMode(FillMode value)
    pub fn forceWriteFillMode(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(5, value.lastBin(this.zone));
            let bin = Factory::dup(val, KoZone::enter());
            self.map.put(5, bin);
            // Java: if (this.fillMode != null)
            if self.fillMode.is_some() {
                // Java: this.fillMode = null;
                self.fillMode = None;
            }
        }
    }

    // Java: RecolorInfo recolor()
    pub fn recolor(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.recolor == null)
        if self.recolor.is_none() {
            // Java: if (this.map.has(6))
            if self.map.has(6) {
                // Java: KoBin bin = this.map.byKey(6);
                let bin = self.map.byKey(6);
                // Java: this.recolor = (RecolorInfo)KoBin.Decode.toAny(RecolorInfo.class, bin, this.zone);
                self.recolor = Some(Factory::dup(bin.as_ref(), KoZone::enter()));
            }
            // Java: else if (this.parent != null && this.parent.hasRecolor())
            else if self.parent.as_ref().map_or(false, |p| p.hasRecolor()) {
                // Java: this.recolor = this.parent.recolor();
                let bin = Null::value1();
                self.recolor = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.recolor = (RecolorInfo)KoBin.Decode.toAny(RecolorInfo.class, bin, this.zone);
                self.recolor = Some(bin);
            }
        }
        // Java: return this.recolor;
        self.recolor.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasRecolor()
    pub fn hasRecolor(&self) -> bool {
        // Java: return this.map.has(6) || this.recolor != null && this.recolor.dirty() || this.parent != null && this.parent.hasRecolor();
        self.map.has(6) || self.recolor.is_some() || self.parent.as_ref().map_or(false, |p| p.hasRecolor())
    }

    // Java: void deleteRecolor()
    pub fn deleteRecolor(&mut self) {
        // Java: this.map.put(6, (KoBin)null);
        self.map.clear_key(6);
        // Java: if (this.recolor != null)
        if self.recolor.is_some() {
            // Java: this.recolor = null;
            self.recolor = None;
        }
    }

    // Java: void writeRecolor(RecolorInfo value)
    pub fn writeRecolor(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, KoZone::enter());
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(6, bin);
                self.map.put(6, bin);
                // Java: if (this.recolor != null)
                if self.recolor.is_some() {
                    // Java: this.recolor = null;
                    self.recolor = None;
                }
            }
        }
    }

    // Java: void forceWriteRecolor(RecolorInfo value)
    pub fn forceWriteRecolor(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(6, value.lastBin(this.zone));
            let bin = Factory::dup(val, KoZone::enter());
            self.map.put(6, bin);
            // Java: if (this.recolor != null)
            if self.recolor.is_some() {
                // Java: this.recolor = null;
                self.recolor = None;
            }
        }
    }

    // Java: Color mainClr()
    pub fn mainClr(&mut self) -> &mut dyn KoBinTrait {
        // Java: if (this.mainClr == null)
        if self.mainClr.is_none() {
            // Java: if (this.map.has(7))
            if self.map.has(7) {
                // Java: KoBin bin = this.map.byKey(7);
                let bin = self.map.byKey(7);
                // Java: this.mainClr = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
                self.mainClr = Some(Factory::dup(bin.as_ref(), KoZone::enter()));
            }
            // Java: else if (this.parent != null && this.parent.hasMainClr())
            else if self.parent.as_ref().map_or(false, |p| p.hasMainClr()) {
                // Java: this.mainClr = this.parent.mainClr();
                let bin = Null::value1();
                self.mainClr = Some(bin);
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let bin = Null::value1();
                // Java: this.mainClr = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
                self.mainClr = Some(bin);
            }
        }
        // Java: return this.mainClr;
        self.mainClr.as_mut().unwrap().as_mut()
    }

    // Java: boolean hasMainClr()
    pub fn hasMainClr(&self) -> bool {
        // Java: return this.map.has(7) || this.mainClr != null && this.mainClr.dirty() || this.parent != null && this.parent.hasMainClr();
        self.map.has(7) || self.mainClr.is_some() || self.parent.as_ref().map_or(false, |p| p.hasMainClr())
    }

    // Java: void deleteMainClr()
    pub fn deleteMainClr(&mut self) {
        // Java: this.map.put(7, (KoBin)null);
        self.map.clear_key(7);
        // Java: if (this.mainClr != null)
        if self.mainClr.is_some() {
            // Java: this.mainClr = null;
            self.mainClr = None;
        }
    }

    // Java: void writeMainClr(Color value)
    pub fn writeMainClr(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = Factory::dup(val, KoZone::enter());
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(7, bin);
                self.map.put(7, bin);
                // Java: if (this.mainClr != null)
                if self.mainClr.is_some() {
                    // Java: this.mainClr = null;
                    self.mainClr = None;
                }
            }
        }
    }

    // Java: void forceWriteMainClr(Color value)
    pub fn forceWriteMainClr(&mut self, value: Option<&dyn KoBinTrait>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.map.put(7, value.lastBin(this.zone));
            let bin = Factory::dup(val, KoZone::enter());
            self.map.put(7, bin);
            // Java: if (this.mainClr != null)
            if self.mainClr.is_some() {
                // Java: this.mainClr = null;
                self.mainClr = None;
            }
        }
    }
}

impl KoAr for BlipFill {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { BlipFill::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { BlipFill::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { BlipFill::lastBin(self, var1) }
}
impl KoClosable for BlipFill { fn close(&mut self) { BlipFill::close(self); } }
