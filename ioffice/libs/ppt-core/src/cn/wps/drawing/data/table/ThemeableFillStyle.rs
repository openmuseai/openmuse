#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.fill.Fill
use crate::cn::wps::drawing::data::fill::Fill::Fill;
// Java: import cn.wps.drawing.data.style.StyleMatrixReference
use crate::cn::wps::drawing::data::style::StyleMatrixReference::StyleMatrixReference;
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

// Java: public class ThemeableFillStyle implements KoAr, KoClosable
pub struct ThemeableFillStyle {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Fill fill
    pub(crate) fill: Option<Box<Fill>>,
    // Java field: StyleMatrixReference styleMatrixRef
    pub(crate) styleMatrixRef: Option<Box<StyleMatrixReference>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: ThemeableFillStyle parent
    pub(crate) parent: Option<Box<ThemeableFillStyle>>,
}

const TFS_FILL: i32 = 1;
const TFS_STYLE_MATRIX_REF: i32 = 2;

// Java: public class ThemeableFillStyle implements KoAr, KoClosable
impl ThemeableFillStyle {
    // Java: static protected static <Value1 extends ThemeableFillStyle> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = ThemeableFillStyle::ThemeableFillStyle();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static ThemeableFillStyle create()
    pub fn create() -> Self {
        // Java: return initialize(ThemeableFillStyle.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static ThemeableFillStyle open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(ThemeableFillStyle.class, bin);
        Self::initialize(bin)
    }

    // Java: ThemeableFillStyle()
    pub fn ThemeableFillStyle() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        ThemeableFillStyle {
            zone: KoZone::defaultZone(),
            fill: None,
            styleMatrixRef: None,
            bin_box: KpBinBox::new(),
            map: KpMap::new(),
            parent: None,
        }
    }

    // Java: ThemeableFillStyle(KoBin bin)
    pub fn ThemeableFillStyle_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        ThemeableFillStyle {
            zone: KoZone::defaultZone(),
            fill: None,
            styleMatrixRef: None,
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

    // Java: void load(ThemeableFillStyle value)
    pub fn load_from(&mut self, value: Option<&ThemeableFillStyle>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(self.zone);
            self.loadBin(last.as_ref(), self.zone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.fill != null && this.fill.dirty())
        if let Some(ref mut child) = self.fill {
            if child.dirty() {
                // Java: this.map.put(1, this.fill.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(TFS_FILL, bin);
            }
        }
        // Java: if (this.styleMatrixRef != null && this.styleMatrixRef.dirty())
        if let Some(ref mut child) = self.styleMatrixRef {
            if child.dirty() {
                // Java: this.map.put(2, this.styleMatrixRef.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(TFS_STYLE_MATRIX_REF, bin);
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

    // Java: boolean equalsLax(ThemeableFillStyle o)
    pub fn equalsLax(&self, o: &ThemeableFillStyle) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &ThemeableFillStyle) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &ThemeableFillStyle, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof ThemeableFillStyle)
        if true {
            // Java: ThemeableFillStyle dst = (ThemeableFillStyle)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst) {
                // Java: return true;
                return true;
            }
            // Java: else if ((this.hasFill() || !dst.hasFill()) && (!this.hasFill() || dst.hasFill()))
            else if (self.hasFill() || !dst.hasFill()) && (!self.hasFill() || dst.hasFill()) {
                // Java: if (this.hasFill() && dst.hasFill())
                if self.hasFill() && dst.hasFill() {
                    // Java: if (isStrict && !this.fill().equals(dst.fill()))
                    if isStrict {
                        // Java: return false;
                    }
                    // Java: if (!isStrict && !this.fill().equalsLax(dst.fill()))
                    if !isStrict {
                        // Java: return false;
                    }
                }
                // Java: if ((this.hasStyleMatrixRef() || !dst.hasStyleMatrixRef()) && (!this.hasStyleMatrixRef() || dst.hasStyleMatrixRef()))
                if (self.hasStyleMatrixRef() || !dst.hasStyleMatrixRef()) && (!self.hasStyleMatrixRef() || dst.hasStyleMatrixRef()) {
                    // Java: if (this.hasStyleMatrixRef() && dst.hasStyleMatrixRef())
                    if self.hasStyleMatrixRef() && dst.hasStyleMatrixRef() {
                        // Java: if (isStrict && !this.styleMatrixRef().equals(dst.styleMatrixRef()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.styleMatrixRef().equalsLax(dst.styleMatrixRef()))
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
        // Java: if (this.hasFill())
        if self.hasFill() {
            // Java: ret += this.fill().hashCode();
        }
        // Java: if (this.hasStyleMatrixRef())
        if self.hasStyleMatrixRef() {
            // Java: ret += this.styleMatrixRef().hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: if (this.fill != null && this.fill.dirty())
        if self.fill.as_ref().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: return this.styleMatrixRef != null && this.styleMatrixRef.dirty() ? true : this.map.dirty();
            if self.styleMatrixRef.as_ref().map_or(false, |r| r.dirty()) { true } else { self.map.dirty() }
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
        // Java: this.fill = null;
        self.fill = None;
        // Java: this.styleMatrixRef = null;
        self.styleMatrixRef = None;
    }

    // Java: void follow(ThemeableFillStyle parent)
    pub fn follow(&mut self, parent: &mut ThemeableFillStyle) {
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

    // Java: ThemeableFillStyle parent()
    pub fn parent_ref(&self) -> Option<&ThemeableFillStyle> {
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

    // Java: Fill fill()
    pub fn fill(&self) -> &Fill {
        // Java: if (this.fill == null)
        if self.fill.is_none() {
            // Java: if (this.map.has(1))
            if self.map.has(TFS_FILL) {
                // Java: KoBin bin = this.map.byKey(1);
                let _bin = self.map.byKey(TFS_FILL);
                // Java: this.fill = (Fill)KoBin.Decode.toAny(Fill.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasFill())
            else if self.parent.as_ref().map_or(false, |p| p.hasFill()) {
                // Java: this.fill = this.parent.fill();
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.fill = (Fill)KoBin.Decode.toAny(Fill.class, bin, this.zone);
            }
        }
        // Java: return this.fill;
        self.fill.as_ref().unwrap()
    }

    // Java: boolean hasFill()
    pub fn hasFill(&self) -> bool {
        // Java: return this.map.has(1) || this.fill != null && this.fill.dirty() || this.parent != null && this.parent.hasFill();
        self.map.has(TFS_FILL) || self.fill.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasFill())
    }

    // Java: void deleteFill()
    pub fn deleteFill(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(TFS_FILL);
        // Java: if (this.fill != null)
        if self.fill.is_some() {
            // Java: this.fill = null;
            self.fill = None;
        }
    }

    // Java: void writeFill(Fill value)
    pub fn writeFill(&mut self, value: &Fill) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(1, bin);
                self.map.put(TFS_FILL, bin);
                // Java: if (this.fill != null)
                if self.fill.is_some() {
                    // Java: this.fill = null;
                    self.fill = None;
                }
            }
        }
    }

    // Java: void forceWriteFill(Fill value)
    pub fn forceWriteFill(&mut self, value: &Fill) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(1, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(TFS_FILL, bin);
            // Java: if (this.fill != null)
            if self.fill.is_some() {
                // Java: this.fill = null;
                self.fill = None;
            }
        }
    }

    // Java: StyleMatrixReference styleMatrixRef()
    pub fn styleMatrixRef(&self) -> &StyleMatrixReference {
        // Java: if (this.styleMatrixRef == null)
        if self.styleMatrixRef.is_none() {
            // Java: if (this.map.has(2))
            if self.map.has(TFS_STYLE_MATRIX_REF) {
                // Java: KoBin bin = this.map.byKey(2);
                let _bin = self.map.byKey(TFS_STYLE_MATRIX_REF);
                // Java: this.styleMatrixRef = (StyleMatrixReference)KoBin.Decode.toAny(StyleMatrixReference.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasStyleMatrixRef())
            else if self.parent.as_ref().map_or(false, |p| p.hasStyleMatrixRef()) {
                // Java: this.styleMatrixRef = this.parent.styleMatrixRef();
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.styleMatrixRef = (StyleMatrixReference)KoBin.Decode.toAny(StyleMatrixReference.class, bin, this.zone);
            }
        }
        // Java: return this.styleMatrixRef;
        self.styleMatrixRef.as_ref().unwrap()
    }

    // Java: boolean hasStyleMatrixRef()
    pub fn hasStyleMatrixRef(&self) -> bool {
        // Java: return this.map.has(2) || this.styleMatrixRef != null && this.styleMatrixRef.dirty() || this.parent != null && this.parent.hasStyleMatrixRef();
        self.map.has(TFS_STYLE_MATRIX_REF) || self.styleMatrixRef.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasStyleMatrixRef())
    }

    // Java: void deleteStyleMatrixRef()
    pub fn deleteStyleMatrixRef(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(TFS_STYLE_MATRIX_REF);
        // Java: if (this.styleMatrixRef != null)
        if self.styleMatrixRef.is_some() {
            // Java: this.styleMatrixRef = null;
            self.styleMatrixRef = None;
        }
    }

    // Java: void writeStyleMatrixRef(StyleMatrixReference value)
    pub fn writeStyleMatrixRef(&mut self, value: &StyleMatrixReference) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(2, bin);
                self.map.put(TFS_STYLE_MATRIX_REF, bin);
                // Java: if (this.styleMatrixRef != null)
                if self.styleMatrixRef.is_some() {
                    // Java: this.styleMatrixRef = null;
                    self.styleMatrixRef = None;
                }
            }
        }
    }

    // Java: void forceWriteStyleMatrixRef(StyleMatrixReference value)
    pub fn forceWriteStyleMatrixRef(&mut self, value: &StyleMatrixReference) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(2, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(TFS_STYLE_MATRIX_REF, bin);
            // Java: if (this.styleMatrixRef != null)
            if self.styleMatrixRef.is_some() {
                // Java: this.styleMatrixRef = null;
                self.styleMatrixRef = None;
            }
        }
    }
}

impl KoAr for ThemeableFillStyle {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { ThemeableFillStyle::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { ThemeableFillStyle::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { ThemeableFillStyle::discard_with_zone(self, var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { ThemeableFillStyle::lastBin(self, var1) }
}

impl KoClosable for ThemeableFillStyle {
    fn close(&mut self) { ThemeableFillStyle::close(self); }
}
