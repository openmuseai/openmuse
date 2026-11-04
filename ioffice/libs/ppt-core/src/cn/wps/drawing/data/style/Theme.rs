#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.color.CustomColor
use crate::cn::wps::drawing::data::color::CustomColor::CustomColor;
// Java: import cn.wps.drawing.data.shape.ShapePr
use crate::cn::wps::drawing::data::shape::ShapePr::ShapePr;
// Java: import cn.wps.drawing.data.text.TextBodyProperties
use crate::cn::wps::drawing::data::text::TextBodyProperties::TextBodyProperties;
// Java: import cn.wps.drawing.data.text.TextListStyle
use crate::cn::wps::drawing::data::text::TextListStyle::TextListStyle;
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

use crate::cn::wps::drawing::data::style::ColorScheme::ColorScheme;
use crate::cn::wps::drawing::data::style::ColorMapping::ColorMapping;
use crate::cn::wps::drawing::data::style::FontScheme::FontScheme;
use crate::cn::wps::drawing::data::style::StyleMatrix::StyleMatrix;
use crate::cn::wps::drawing::data::style::ShapeStyle::ShapeStyle;

// Java: public class Theme implements KoAr, KoClosable
pub struct Theme {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: ColorScheme colorScheme
    pub(crate) colorScheme: Option<Box<ColorScheme>>,
    // Java field: FontScheme fontScheme
    pub(crate) fontScheme: Option<Box<FontScheme>>,
    // Java field: StyleMatrix formatScheme
    pub(crate) formatScheme: Option<Box<StyleMatrix>>,
    // Java field: CustomColorLst customColorLst
    pub(crate) customColorLst: Option<Box<CustomColorLst>>,
    // Java field: ExtraColorSchemeLst extraColorSchemeList
    pub(crate) extraColorSchemeList: Option<Box<ExtraColorSchemeLst>>,
    // Java field: DefaultShapeDefinition lineDef
    pub(crate) lineDef: Option<Box<DefaultShapeDefinition>>,
    // Java field: DefaultShapeDefinition shapeDef
    pub(crate) shapeDef: Option<Box<DefaultShapeDefinition>>,
    // Java field: DefaultShapeDefinition textDef
    pub(crate) textDef: Option<Box<DefaultShapeDefinition>>,
    // Java field: ColorMapping colorMapping
    pub(crate) colorMapping: Option<Box<ColorMapping>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: Theme parent
    pub(crate) parent: Option<Box<Theme>>,
}

// Java: public class ExtraColorScheme
pub struct ExtraColorScheme {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: ColorScheme colorScheme
    pub(crate) colorScheme: Option<Box<ColorScheme>>,
    // Java field: ColorMapping colorMapping
    pub(crate) colorMapping: Option<Box<ColorMapping>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: ExtraColorScheme parent
    pub(crate) parent: Option<Box<ExtraColorScheme>>,
}

// Java: public class DefaultShapeDefinition
pub struct DefaultShapeDefinition {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: ShapePr shapePr
    pub(crate) shapePr: Option<Box<ShapePr>>,
    // Java field: TextBodyProperties bodyPr
    pub(crate) bodyPr: Option<Box<TextBodyProperties>>,
    // Java field: TextListStyle textListStyle
    pub(crate) textListStyle: Option<Box<TextListStyle>>,
    // Java field: ShapeStyle shapeStyle
    pub(crate) shapeStyle: Option<Box<ShapeStyle>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: DefaultShapeDefinition parent
    pub(crate) parent: Option<Box<DefaultShapeDefinition>>,
}

// Java: public class CustomColorLst
pub struct CustomColorLst {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

// Java: public class ExtraColorSchemeLst
pub struct ExtraColorSchemeLst {
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpList list
    pub(crate) list: KpList,
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
}

const THEME_NAME: i32 = 1;
const THEME_COLOR_SCHEME: i32 = 2;
const THEME_FONT_SCHEME: i32 = 3;
const THEME_FORMAT_SCHEME: i32 = 4;
const THEME_CUSTOM_COLOR_LST: i32 = 5;
const THEME_EXTRA_COLOR_SCHEME_LIST: i32 = 6;
const THEME_LINE_DEF: i32 = 7;
const THEME_SHAPE_DEF: i32 = 8;
const THEME_TEXT_DEF: i32 = 9;
const THEME_COLOR_MAPPING: i32 = 10;

const ECS_COLOR_SCHEME: i32 = 1;
const ECS_COLOR_MAPPING: i32 = 2;

const DSD_SHAPE_PR: i32 = 1;
const DSD_BODY_PR: i32 = 2;
const DSD_TEXT_LIST_STYLE: i32 = 3;
const DSD_SHAPE_STYLE: i32 = 4;

// Java: public class Theme implements KoAr, KoClosable
impl Theme {
    // Java: static protected static <Value1 extends Theme> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = Theme::Theme();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static Theme create()
    pub fn create() -> Self {
        // Java: return initialize(Theme.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static Theme open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(Theme.class, bin);
        Self::initialize(bin)
    }

    // Java: Theme()
    pub fn Theme() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        Theme {
            zone: KoZone::defaultZone(),
            colorScheme: None, fontScheme: None, formatScheme: None,
            customColorLst: None, extraColorSchemeList: None,
            lineDef: None, shapeDef: None, textDef: None,
            colorMapping: None,
            bin_box: KpBinBox::new(), map: KpMap::new(), parent: None,
        }
    }

    // Java: Theme(KoBin bin)
    pub fn Theme_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        Theme {
            zone: KoZone::defaultZone(),
            colorScheme: None, fontScheme: None, formatScheme: None,
            customColorLst: None, extraColorSchemeList: None,
            lineDef: None, shapeDef: None, textDef: None,
            colorMapping: None,
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

    // Java: void load(Theme value)
    pub fn load_from(&mut self, value: Option<&Theme>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(self.zone);
            self.loadBin(last.as_ref(), self.zone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.colorScheme != null && this.colorScheme.dirty())
        if let Some(ref mut child) = self.colorScheme {
            if child.dirty() {
                // Java: this.map.put(2, this.colorScheme.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(THEME_COLOR_SCHEME, bin);
            }
        }
        // Java: if (this.fontScheme != null && this.fontScheme.dirty())
        if let Some(ref mut child) = self.fontScheme {
            if child.dirty() {
                // Java: this.map.put(3, this.fontScheme.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(THEME_FONT_SCHEME, bin);
            }
        }
        // Java: if (this.formatScheme != null && this.formatScheme.dirty())
        if let Some(ref mut child) = self.formatScheme {
            if child.dirty() {
                // Java: this.map.put(4, this.formatScheme.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(THEME_FORMAT_SCHEME, bin);
            }
        }
        // Java: if (this.customColorLst != null && this.customColorLst.dirty())
        if let Some(ref mut child) = self.customColorLst {
            if child.dirty() {
                // Java: this.map.put(5, this.customColorLst.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(THEME_CUSTOM_COLOR_LST, bin);
            }
        }
        // Java: if (this.extraColorSchemeList != null && this.extraColorSchemeList.dirty())
        if let Some(ref mut child) = self.extraColorSchemeList {
            if child.dirty() {
                // Java: this.map.put(6, this.extraColorSchemeList.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(THEME_EXTRA_COLOR_SCHEME_LIST, bin);
            }
        }
        // Java: if (this.lineDef != null && this.lineDef.dirty())
        if let Some(ref mut child) = self.lineDef {
            if child.dirty() {
                // Java: this.map.put(7, this.lineDef.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(THEME_LINE_DEF, bin);
            }
        }
        // Java: if (this.shapeDef != null && this.shapeDef.dirty())
        if let Some(ref mut child) = self.shapeDef {
            if child.dirty() {
                // Java: this.map.put(8, this.shapeDef.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(THEME_SHAPE_DEF, bin);
            }
        }
        // Java: if (this.textDef != null && this.textDef.dirty())
        if let Some(ref mut child) = self.textDef {
            if child.dirty() {
                // Java: this.map.put(9, this.textDef.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(THEME_TEXT_DEF, bin);
            }
        }
        // Java: if (this.colorMapping != null && this.colorMapping.dirty())
        if let Some(ref mut child) = self.colorMapping {
            if child.dirty() {
                // Java: this.map.put(10, this.colorMapping.saveBin(zone));
                let bin = child.saveBin(zone);
                self.map.put(THEME_COLOR_MAPPING, bin);
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

    // Java: boolean equalsLax(Theme o)
    pub fn equalsLax(&self, o: &Theme) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &Theme) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &Theme, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof Theme)
        if true {
            // Java: Theme dst = (Theme)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst) {
                // Java: return true;
                return true;
            }
            // Java: else if ((this.hasName() || !dst.hasName()) && (!this.hasName() || dst.hasName()))
            else if (self.hasName() || !dst.hasName()) && (!self.hasName() || dst.hasName()) {
                // Java: if (this.hasName() && dst.hasName() && !this.name().equals(dst.name()))
                if self.hasName() && dst.hasName() && self.name() != dst.name() {
                    // Java: return false;
                    return false;
                }
                // Java: else if ((this.hasColorScheme() || !dst.hasColorScheme()) && (!this.hasColorScheme() || dst.hasColorScheme()))
                else if (self.hasColorScheme() || !dst.hasColorScheme()) && (!self.hasColorScheme() || dst.hasColorScheme()) {
                    // Java: if (this.hasColorScheme() && dst.hasColorScheme())
                    if self.hasColorScheme() && dst.hasColorScheme() {
                        // Java: if (isStrict && !this.colorScheme().equals(dst.colorScheme()))
                        if isStrict {
                            // Java: return false;
                        }
                        // Java: if (!isStrict && !this.colorScheme().equalsLax(dst.colorScheme()))
                        if !isStrict {
                            // Java: return false;
                        }
                    }
                    // Java: if ((this.hasFontScheme() || !dst.hasFontScheme()) && (!this.hasFontScheme() || dst.hasFontScheme()))
                    if (self.hasFontScheme() || !dst.hasFontScheme()) && (!self.hasFontScheme() || dst.hasFontScheme()) {
                        // Java: if (this.hasFontScheme() && dst.hasFontScheme())
                        if self.hasFontScheme() && dst.hasFontScheme() {
                            // Java: if (isStrict && !this.fontScheme().equals(dst.fontScheme()))
                            if isStrict {
                                // Java: return false;
                            }
                            // Java: if (!isStrict && !this.fontScheme().equalsLax(dst.fontScheme()))
                            if !isStrict {
                                // Java: return false;
                            }
                        }
                        // Java: if ((this.hasFormatScheme() || !dst.hasFormatScheme()) && (!this.hasFormatScheme() || dst.hasFormatScheme()))
                        if (self.hasFormatScheme() || !dst.hasFormatScheme()) && (!self.hasFormatScheme() || dst.hasFormatScheme()) {
                            // Java: if (this.hasFormatScheme() && dst.hasFormatScheme())
                            if self.hasFormatScheme() && dst.hasFormatScheme() {
                                // Java: if (isStrict && !this.formatScheme().equals(dst.formatScheme()))
                                if isStrict {
                                    // Java: return false;
                                }
                                // Java: if (!isStrict && !this.formatScheme().equalsLax(dst.formatScheme()))
                                if !isStrict {
                                    // Java: return false;
                                }
                            }
                            // Java: if ((this.hasCustomColorLst() || !dst.hasCustomColorLst()) && (!this.hasCustomColorLst() || dst.hasCustomColorLst()))
                            if (self.hasCustomColorLst() || !dst.hasCustomColorLst()) && (!self.hasCustomColorLst() || dst.hasCustomColorLst()) {
                                // Java: if (this.hasCustomColorLst() && dst.hasCustomColorLst())
                                if self.hasCustomColorLst() && dst.hasCustomColorLst() {
                                    // Java: if (isStrict && !this.customColorLst().equals(dst.customColorLst()))
                                    if isStrict {
                                        // Java: return false;
                                    }
                                    // Java: if (!isStrict && !this.customColorLst().equalsLax(dst.customColorLst()))
                                    if !isStrict {
                                        // Java: return false;
                                    }
                                }
                                // Java: if ((this.hasExtraColorSchemeList() || !dst.hasExtraColorSchemeList()) && (!this.hasExtraColorSchemeList() || dst.hasExtraColorSchemeList()))
                                if (self.hasExtraColorSchemeList() || !dst.hasExtraColorSchemeList()) && (!self.hasExtraColorSchemeList() || dst.hasExtraColorSchemeList()) {
                                    // Java: if (this.hasExtraColorSchemeList() && dst.hasExtraColorSchemeList())
                                    if self.hasExtraColorSchemeList() && dst.hasExtraColorSchemeList() {
                                        // Java: if (isStrict && !this.extraColorSchemeList().equals(dst.extraColorSchemeList()))
                                        if isStrict {
                                            // Java: return false;
                                        }
                                        // Java: if (!isStrict && !this.extraColorSchemeList().equalsLax(dst.extraColorSchemeList()))
                                        if !isStrict {
                                            // Java: return false;
                                        }
                                    }
                                    // Java: if ((this.hasLineDef() || !dst.hasLineDef()) && (!this.hasLineDef() || dst.hasLineDef()))
                                    if (self.hasLineDef() || !dst.hasLineDef()) && (!self.hasLineDef() || dst.hasLineDef()) {
                                        // Java: if (this.hasLineDef() && dst.hasLineDef())
                                        if self.hasLineDef() && dst.hasLineDef() {
                                            // Java: if (isStrict && !this.lineDef().equals(dst.lineDef()))
                                            if isStrict {
                                                // Java: return false;
                                            }
                                            // Java: if (!isStrict && !this.lineDef().equalsLax(dst.lineDef()))
                                            if !isStrict {
                                                // Java: return false;
                                            }
                                        }
                                        // Java: if ((this.hasShapeDef() || !dst.hasShapeDef()) && (!this.hasShapeDef() || dst.hasShapeDef()))
                                        if (self.hasShapeDef() || !dst.hasShapeDef()) && (!self.hasShapeDef() || dst.hasShapeDef()) {
                                            // Java: if (this.hasShapeDef() && dst.hasShapeDef())
                                            if self.hasShapeDef() && dst.hasShapeDef() {
                                                // Java: if (isStrict && !this.shapeDef().equals(dst.shapeDef()))
                                                if isStrict {
                                                    // Java: return false;
                                                }
                                                // Java: if (!isStrict && !this.shapeDef().equalsLax(dst.shapeDef()))
                                                if !isStrict {
                                                    // Java: return false;
                                                }
                                            }
                                            // Java: if ((this.hasTextDef() || !dst.hasTextDef()) && (!this.hasTextDef() || dst.hasTextDef()))
                                            if (self.hasTextDef() || !dst.hasTextDef()) && (!self.hasTextDef() || dst.hasTextDef()) {
                                                // Java: if (this.hasTextDef() && dst.hasTextDef())
                                                if self.hasTextDef() && dst.hasTextDef() {
                                                    // Java: if (isStrict && !this.textDef().equals(dst.textDef()))
                                                    if isStrict {
                                                        // Java: return false;
                                                    }
                                                    // Java: if (!isStrict && !this.textDef().equalsLax(dst.textDef()))
                                                    if !isStrict {
                                                        // Java: return false;
                                                    }
                                                }
                                                // Java: if ((this.hasColorMapping() || !dst.hasColorMapping()) && (!this.hasColorMapping() || dst.hasColorMapping()))
                                                if (self.hasColorMapping() || !dst.hasColorMapping()) && (!self.hasColorMapping() || dst.hasColorMapping()) {
                                                    // Java: if (this.hasColorMapping() && dst.hasColorMapping())
                                                    if self.hasColorMapping() && dst.hasColorMapping() {
                                                        // Java: if (isStrict && !this.colorMapping().equals(dst.colorMapping()))
                                                        if isStrict {
                                                            // Java: return false;
                                                        }
                                                        // Java: if (!isStrict && !this.colorMapping().equalsLax(dst.colorMapping()))
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
        // Java: return false;
        else {
            false }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int ret = 0;
        let mut ret: i32 = 0;
        // Java: if (this.hasName())
        if self.hasName() {
            // Java: ret += String.valueOf(this.name()).hashCode();
        }
        // Java: if (this.hasColorScheme())
        if self.hasColorScheme() {
            // Java: ret += this.colorScheme().hashCode();
        }
        // Java: if (this.hasFontScheme())
        if self.hasFontScheme() {
            // Java: ret += this.fontScheme().hashCode();
        }
        // Java: if (this.hasFormatScheme())
        if self.hasFormatScheme() {
            // Java: ret += this.formatScheme().hashCode();
        }
        // Java: if (this.hasCustomColorLst())
        if self.hasCustomColorLst() {
            // Java: ret += this.customColorLst().hashCode();
        }
        // Java: if (this.hasExtraColorSchemeList())
        if self.hasExtraColorSchemeList() {
            // Java: ret += this.extraColorSchemeList().hashCode();
        }
        // Java: if (this.hasLineDef())
        if self.hasLineDef() {
            // Java: ret += this.lineDef().hashCode();
        }
        // Java: if (this.hasShapeDef())
        if self.hasShapeDef() {
            // Java: ret += this.shapeDef().hashCode();
        }
        // Java: if (this.hasTextDef())
        if self.hasTextDef() {
            // Java: ret += this.textDef().hashCode();
        }
        // Java: if (this.hasColorMapping())
        if self.hasColorMapping() {
            // Java: ret += this.colorMapping().hashCode();
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: if (this.colorScheme != null && this.colorScheme.dirty())
        if self.colorScheme.as_ref().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else if (this.fontScheme != null && this.fontScheme.dirty())
        else if self.fontScheme.as_ref().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else if (this.formatScheme != null && this.formatScheme.dirty())
        else if self.formatScheme.as_ref().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else if (this.customColorLst != null && this.customColorLst.dirty())
        else if self.customColorLst.as_ref().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else if (this.extraColorSchemeList != null && this.extraColorSchemeList.dirty())
        else if self.extraColorSchemeList.as_ref().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else if (this.lineDef != null && this.lineDef.dirty())
        else if self.lineDef.as_ref().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else if (this.shapeDef != null && this.shapeDef.dirty())
        else if self.shapeDef.as_ref().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else if (this.textDef != null && this.textDef.dirty())
        else if self.textDef.as_ref().map_or(false, |r| r.dirty()) {
            // Java: return true;
            return true;
        }
        // Java: else
        else {
            // Java: return this.colorMapping != null && this.colorMapping.dirty() ? true : this.map.dirty();
            if self.colorMapping.as_ref().map_or(false, |r| r.dirty()) { true } else { self.map.dirty() }
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
        // Java: this.colorScheme = null;
        self.colorScheme = None;
        // Java: this.fontScheme = null;
        self.fontScheme = None;
        // Java: this.formatScheme = null;
        self.formatScheme = None;
        // Java: this.customColorLst = null;
        self.customColorLst = None;
        // Java: this.extraColorSchemeList = null;
        self.extraColorSchemeList = None;
        // Java: this.lineDef = null;
        self.lineDef = None;
        // Java: this.shapeDef = null;
        self.shapeDef = None;
        // Java: this.textDef = null;
        self.textDef = None;
        // Java: this.colorMapping = null;
        self.colorMapping = None;
    }

    // Java: void follow(Theme parent)
    pub fn follow(&mut self, parent: &mut Theme) {
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

    // Java: Theme parent()
    pub fn parent_ref(&self) -> Option<&Theme> {
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

    // Java: String name()
    pub fn name(&self) -> String {
        // Java: if (!this.map.has(1))
        if !self.map.has(THEME_NAME) {
            // Java: return this.parent == null ? "" : this.parent.name();
            match &self.parent { None => String::new(), Some(p) => p.name() }
        }
        // Java: else
        else {
            // Java: KoBin bin = this.map.byKey(1);
            let bin = self.map.byKey(THEME_NAME);
            // Java: return KoBin.Decode.toText(bin);
            Decode::toText(bin.as_ref())
        }
    }

    // Java: boolean hasName()
    pub fn hasName(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasName();
        self.map.has(THEME_NAME) || self.parent.as_ref().map_or(false, |p| p.hasName())
    }

    // Java: void deleteName()
    pub fn deleteName(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(THEME_NAME);
    }

    // Java: void putName(String value)
    pub fn putName(&mut self, value: &str) {
        // Java: this.map.put(1, KoBin.Encode.from(value, this.zone));
        let bin = Encode::from_string(value, self.zone);
        self.map.put(THEME_NAME, bin);
    }

    // Java: ColorScheme colorScheme()
    pub fn colorScheme(&self) -> &ColorScheme {
        // Java: if (this.colorScheme == null)
        if self.colorScheme.is_none()
        {
            // Java: if (this.map.has(2))
            if self.map.has(THEME_COLOR_SCHEME)
            {
                // Java: KoBin bin = this.map.byKey(2);
                let _bin = self.map.byKey(THEME_COLOR_SCHEME);
                // Java: this.colorScheme = (ColorScheme)KoBin.Decode.toAny(ColorScheme.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasColorScheme())
            else if self.parent.as_ref().map_or(false, |p| p.hasColorScheme())
            {
                // Java: this.colorScheme = this.parent.colorScheme();
            }
            // Java: else
            else
            {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.colorScheme = (ColorScheme)KoBin.Decode.toAny(ColorScheme.class, bin, this.zone);
            }
        }
        // Java: return this.colorScheme;
        self.colorScheme.as_ref().unwrap()
    }

    // Java: boolean hasColorScheme()
    pub fn hasColorScheme(&self) -> bool {
        // Java: return this.map.has(2) || this.colorScheme != null && this.colorScheme.dirty() || this.parent != null && this.parent.hasColorScheme();
        self.map.has(THEME_COLOR_SCHEME) || self.colorScheme.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasColorScheme())
    }

    // Java: void deleteColorScheme()
    pub fn deleteColorScheme(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(THEME_COLOR_SCHEME);
        // Java: if (this.colorScheme != null)
        if self.colorScheme.is_some() {
            // Java: this.colorScheme = null;
            self.colorScheme = None;
        }
    }

    // Java: void writeColorScheme(ColorScheme value)
    pub fn writeColorScheme(&mut self, value: &ColorScheme) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(2, bin);
                self.map.put(THEME_COLOR_SCHEME, bin);
                // Java: if (this.colorScheme != null)
                if self.colorScheme.is_some() {
                    // Java: this.colorScheme = null;
                    self.colorScheme = None;
                }
            }
        }
    }

    // Java: void forceWriteColorScheme(ColorScheme value)
    pub fn forceWriteColorScheme(&mut self, value: &ColorScheme) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(2, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(THEME_COLOR_SCHEME, bin);
            // Java: if (this.colorScheme != null)
            if self.colorScheme.is_some() {
                // Java: this.colorScheme = null;
                self.colorScheme = None;
            }
        }
    }

    // Java: FontScheme fontScheme()
    pub fn fontScheme(&self) -> &FontScheme {
        // Java: if (this.fontScheme == null)
        if self.fontScheme.is_none()
        {
            // Java: if (this.map.has(3))
            if self.map.has(THEME_FONT_SCHEME)
            {
                // Java: KoBin bin = this.map.byKey(3);
                let _bin = self.map.byKey(THEME_FONT_SCHEME);
                // Java: this.fontScheme = (FontScheme)KoBin.Decode.toAny(FontScheme.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasFontScheme())
            else if self.parent.as_ref().map_or(false, |p| p.hasFontScheme())
            {
                // Java: this.fontScheme = this.parent.fontScheme();
            }
            // Java: else
            else
            {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.fontScheme = (FontScheme)KoBin.Decode.toAny(FontScheme.class, bin, this.zone);
            }
        }
        // Java: return this.fontScheme;
        self.fontScheme.as_ref().unwrap()
    }

    // Java: boolean hasFontScheme()
    pub fn hasFontScheme(&self) -> bool {
        // Java: return this.map.has(3) || this.fontScheme != null && this.fontScheme.dirty() || this.parent != null && this.parent.hasFontScheme();
        self.map.has(THEME_FONT_SCHEME) || self.fontScheme.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasFontScheme())
    }

    // Java: void deleteFontScheme()
    pub fn deleteFontScheme(&mut self) {
        // Java: this.map.put(3, (KoBin)null);
        self.map.clear_key(THEME_FONT_SCHEME);
        // Java: if (this.fontScheme != null)
        if self.fontScheme.is_some() {
            // Java: this.fontScheme = null;
            self.fontScheme = None;
        }
    }

    // Java: void writeFontScheme(FontScheme value)
    pub fn writeFontScheme(&mut self, value: &FontScheme) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(3, bin);
                self.map.put(THEME_FONT_SCHEME, bin);
                // Java: if (this.fontScheme != null)
                if self.fontScheme.is_some() {
                    // Java: this.fontScheme = null;
                    self.fontScheme = None;
                }
            }
        }
    }

    // Java: void forceWriteFontScheme(FontScheme value)
    pub fn forceWriteFontScheme(&mut self, value: &FontScheme) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(3, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(THEME_FONT_SCHEME, bin);
            // Java: if (this.fontScheme != null)
            if self.fontScheme.is_some() {
                // Java: this.fontScheme = null;
                self.fontScheme = None;
            }
        }
    }

    // Java: StyleMatrix formatScheme()
    pub fn formatScheme(&self) -> &StyleMatrix {
        // Java: if (this.formatScheme == null)
        if self.formatScheme.is_none() {
            // Java: if (this.map.has(4))
            if self.map.has(THEME_FORMAT_SCHEME) {
                // Java: KoBin bin = this.map.byKey(4);
                let _bin = self.map.byKey(THEME_FORMAT_SCHEME);
                // Java: this.formatScheme = (StyleMatrix)KoBin.Decode.toAny(StyleMatrix.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasFormatScheme())
            else if self.parent.as_ref().map_or(false, |p| p.hasFormatScheme()) {
                // Java: this.formatScheme = this.parent.formatScheme();
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.formatScheme = (StyleMatrix)KoBin.Decode.toAny(StyleMatrix.class, bin, this.zone);
            }
        }
        // Java: return this.formatScheme;
        self.formatScheme.as_ref().unwrap()
    }

    // Java: boolean hasFormatScheme()
    pub fn hasFormatScheme(&self) -> bool {
        // Java: return this.map.has(4) || this.formatScheme != null && this.formatScheme.dirty() || this.parent != null && this.parent.hasFormatScheme();
        self.map.has(THEME_FORMAT_SCHEME) || self.formatScheme.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasFormatScheme())
    }

    // Java: void deleteFormatScheme()
    pub fn deleteFormatScheme(&mut self) {
        // Java: this.map.put(4, (KoBin)null);
        self.map.clear_key(THEME_FORMAT_SCHEME);
        // Java: if (this.formatScheme != null)
        // Java: this.formatScheme = null;
        if self.formatScheme.is_some() {
            self.formatScheme = None; }
    }

    // Java: void writeFormatScheme(StyleMatrix value)
    pub fn writeFormatScheme(&mut self, value: &StyleMatrix) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(4, bin);
                self.map.put(THEME_FORMAT_SCHEME, bin);
                // Java: if (this.formatScheme != null)
                // Java: this.formatScheme = null;
                if self.formatScheme.is_some() {
                    self.formatScheme = None; }
            }
        }
    }

    // Java: void forceWriteFormatScheme(StyleMatrix value)
    pub fn forceWriteFormatScheme(&mut self, value: &StyleMatrix) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(4, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(THEME_FORMAT_SCHEME, bin);
            // Java: if (this.formatScheme != null)
            // Java: this.formatScheme = null;
            if self.formatScheme.is_some() {
                self.formatScheme = None; }
        }
    }

    // Java: CustomColorLst customColorLst()
    pub fn customColorLst(&self) -> &CustomColorLst {
        // Java: if (this.customColorLst == null)
        if self.customColorLst.is_none() {
            // Java: if (this.map.has(5))
            if self.map.has(THEME_CUSTOM_COLOR_LST) {
                // Java: KoBin bin = this.map.byKey(5);
                let _bin = self.map.byKey(THEME_CUSTOM_COLOR_LST);
                // Java: this.customColorLst = (CustomColorLst)KoBin.Decode.toAny(CustomColorLst.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasCustomColorLst())
            else if self.parent.as_ref().map_or(false, |p| p.hasCustomColorLst()) {
                // Java: this.customColorLst = this.parent.customColorLst();
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.customColorLst = (CustomColorLst)KoBin.Decode.toAny(CustomColorLst.class, bin, this.zone);
            }
        }
        // Java: return this.customColorLst;
        self.customColorLst.as_ref().unwrap()
    }

    // Java: boolean hasCustomColorLst()
    pub fn hasCustomColorLst(&self) -> bool {
        // Java: return this.map.has(5) || this.customColorLst != null && this.customColorLst.dirty() || this.parent != null && this.parent.hasCustomColorLst();
        self.map.has(THEME_CUSTOM_COLOR_LST) || self.customColorLst.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasCustomColorLst())
    }

    // Java: void deleteCustomColorLst()
    pub fn deleteCustomColorLst(&mut self) {
        // Java: this.map.put(5, (KoBin)null);
        self.map.clear_key(THEME_CUSTOM_COLOR_LST);
        // Java: if (this.customColorLst != null)
        // Java: this.customColorLst = null;
        if self.customColorLst.is_some() {
            self.customColorLst = None; }
    }

    // Java: void writeCustomColorLst(CustomColorLst value)
    pub fn writeCustomColorLst(&mut self, value: &CustomColorLst) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(5, bin);
                self.map.put(THEME_CUSTOM_COLOR_LST, bin);
                // Java: if (this.customColorLst != null)
                // Java: this.customColorLst = null;
                if self.customColorLst.is_some() {
                    self.customColorLst = None; }
            }
        }
    }

    // Java: void forceWriteCustomColorLst(CustomColorLst value)
    pub fn forceWriteCustomColorLst(&mut self, value: &CustomColorLst) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(5, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(THEME_CUSTOM_COLOR_LST, bin);
            // Java: if (this.customColorLst != null)
            // Java: this.customColorLst = null;
            if self.customColorLst.is_some() {
                self.customColorLst = None; }
        }
    }

    // Java: ExtraColorSchemeLst extraColorSchemeList()
    pub fn extraColorSchemeList(&self) -> &ExtraColorSchemeLst {
        // Java: if (this.extraColorSchemeList == null)
        if self.extraColorSchemeList.is_none() {
            // Java: if (this.map.has(6))
            if self.map.has(THEME_EXTRA_COLOR_SCHEME_LIST) {
                // Java: KoBin bin = this.map.byKey(6);
                let _bin = self.map.byKey(THEME_EXTRA_COLOR_SCHEME_LIST);
                // Java: this.extraColorSchemeList = (ExtraColorSchemeLst)KoBin.Decode.toAny(ExtraColorSchemeLst.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasExtraColorSchemeList())
            else if self.parent.as_ref().map_or(false, |p| p.hasExtraColorSchemeList()) {
                // Java: this.extraColorSchemeList = this.parent.extraColorSchemeList();
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.extraColorSchemeList = (ExtraColorSchemeLst)KoBin.Decode.toAny(ExtraColorSchemeLst.class, bin, this.zone);
            }
        }
        // Java: return this.extraColorSchemeList;
        self.extraColorSchemeList.as_ref().unwrap()
    }

    // Java: boolean hasExtraColorSchemeList()
    pub fn hasExtraColorSchemeList(&self) -> bool {
        // Java: return this.map.has(6) || this.extraColorSchemeList != null && this.extraColorSchemeList.dirty() || this.parent != null && this.parent.hasExtraColorSchemeList();
        self.map.has(THEME_EXTRA_COLOR_SCHEME_LIST) || self.extraColorSchemeList.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasExtraColorSchemeList())
    }

    // Java: void deleteExtraColorSchemeList()
    pub fn deleteExtraColorSchemeList(&mut self) {
        // Java: this.map.put(6, (KoBin)null);
        self.map.clear_key(THEME_EXTRA_COLOR_SCHEME_LIST);
        // Java: if (this.extraColorSchemeList != null)
        // Java: this.extraColorSchemeList = null;
        if self.extraColorSchemeList.is_some() {
            self.extraColorSchemeList = None; }
    }

    // Java: void writeExtraColorSchemeList(ExtraColorSchemeLst value)
    pub fn writeExtraColorSchemeList(&mut self, value: &ExtraColorSchemeLst) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(6, bin);
                self.map.put(THEME_EXTRA_COLOR_SCHEME_LIST, bin);
                // Java: if (this.extraColorSchemeList != null)
                // Java: this.extraColorSchemeList = null;
                if self.extraColorSchemeList.is_some() {
                    self.extraColorSchemeList = None; }
            }
        }
    }

    // Java: void forceWriteExtraColorSchemeList(ExtraColorSchemeLst value)
    pub fn forceWriteExtraColorSchemeList(&mut self, value: &ExtraColorSchemeLst) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(6, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(THEME_EXTRA_COLOR_SCHEME_LIST, bin);
            // Java: if (this.extraColorSchemeList != null)
            // Java: this.extraColorSchemeList = null;
            if self.extraColorSchemeList.is_some() {
                self.extraColorSchemeList = None; }
        }
    }

    // Java: DefaultShapeDefinition lineDef()
    pub fn lineDef(&self) -> &DefaultShapeDefinition {
        // Java: if (this.lineDef == null)
        if self.lineDef.is_none() {
            // Java: if (this.map.has(7))
            if self.map.has(THEME_LINE_DEF) {
                // Java: KoBin bin = this.map.byKey(7);
                let _bin = self.map.byKey(THEME_LINE_DEF);
                // Java: this.lineDef = (DefaultShapeDefinition)KoBin.Decode.toAny(DefaultShapeDefinition.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasLineDef())
            else if self.parent.as_ref().map_or(false, |p| p.hasLineDef()) {
                // Java: this.lineDef = this.parent.lineDef();
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.lineDef = (DefaultShapeDefinition)KoBin.Decode.toAny(DefaultShapeDefinition.class, bin, this.zone);
            }
        }
        // Java: return this.lineDef;
        self.lineDef.as_ref().unwrap()
    }

    // Java: boolean hasLineDef()
    pub fn hasLineDef(&self) -> bool {
        // Java: return this.map.has(7) || this.lineDef != null && this.lineDef.dirty() || this.parent != null && this.parent.hasLineDef();
        self.map.has(THEME_LINE_DEF) || self.lineDef.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasLineDef())
    }

    // Java: void deleteLineDef()
    pub fn deleteLineDef(&mut self) {
        // Java: this.map.put(7, (KoBin)null);
        self.map.clear_key(THEME_LINE_DEF);
        // Java: if (this.lineDef != null)
        // Java: this.lineDef = null;
        if self.lineDef.is_some() {
            self.lineDef = None; }
    }

    // Java: void writeLineDef(DefaultShapeDefinition value)
    pub fn writeLineDef(&mut self, value: &DefaultShapeDefinition) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(7, bin);
                self.map.put(THEME_LINE_DEF, bin);
                // Java: if (this.lineDef != null)
                // Java: this.lineDef = null;
                if self.lineDef.is_some() {
                    self.lineDef = None; }
            }
        }
    }

    // Java: void forceWriteLineDef(DefaultShapeDefinition value)
    pub fn forceWriteLineDef(&mut self, value: &DefaultShapeDefinition) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(7, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(THEME_LINE_DEF, bin);
            // Java: if (this.lineDef != null)
            // Java: this.lineDef = null;
            if self.lineDef.is_some() {
                self.lineDef = None; }
        }
    }

    // Java: DefaultShapeDefinition shapeDef()
    pub fn shapeDef(&self) -> &DefaultShapeDefinition {
        // Java: if (this.shapeDef == null)
        if self.shapeDef.is_none() {
            // Java: if (this.map.has(8))
            if self.map.has(THEME_SHAPE_DEF) {
                // Java: KoBin bin = this.map.byKey(8);
                let _bin = self.map.byKey(THEME_SHAPE_DEF);
                // Java: this.shapeDef = (DefaultShapeDefinition)KoBin.Decode.toAny(DefaultShapeDefinition.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasShapeDef())
            else if self.parent.as_ref().map_or(false, |p| p.hasShapeDef()) {
                // Java: this.shapeDef = this.parent.shapeDef();
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.shapeDef = (DefaultShapeDefinition)KoBin.Decode.toAny(DefaultShapeDefinition.class, bin, this.zone);
            }
        }
        // Java: return this.shapeDef;
        self.shapeDef.as_ref().unwrap()
    }

    // Java: boolean hasShapeDef()
    pub fn hasShapeDef(&self) -> bool {
        // Java: return this.map.has(8) || this.shapeDef != null && this.shapeDef.dirty() || this.parent != null && this.parent.hasShapeDef();
        self.map.has(THEME_SHAPE_DEF) || self.shapeDef.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasShapeDef())
    }

    // Java: void deleteShapeDef()
    pub fn deleteShapeDef(&mut self) {
        // Java: this.map.put(8, (KoBin)null);
        self.map.clear_key(THEME_SHAPE_DEF);
        // Java: if (this.shapeDef != null)
        // Java: this.shapeDef = null;
        if self.shapeDef.is_some() {
            self.shapeDef = None; }
    }

    // Java: void writeShapeDef(DefaultShapeDefinition value)
    pub fn writeShapeDef(&mut self, value: &DefaultShapeDefinition) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(8, bin);
                self.map.put(THEME_SHAPE_DEF, bin);
                // Java: if (this.shapeDef != null)
                // Java: this.shapeDef = null;
                if self.shapeDef.is_some() {
                    self.shapeDef = None; }
            }
        }
    }

    // Java: void forceWriteShapeDef(DefaultShapeDefinition value)
    pub fn forceWriteShapeDef(&mut self, value: &DefaultShapeDefinition) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(8, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(THEME_SHAPE_DEF, bin);
            // Java: if (this.shapeDef != null)
            // Java: this.shapeDef = null;
            if self.shapeDef.is_some() {
                self.shapeDef = None; }
        }
    }

    // Java: DefaultShapeDefinition textDef()
    pub fn textDef(&self) -> &DefaultShapeDefinition {
        // Java: if (this.textDef == null)
        if self.textDef.is_none() {
            // Java: if (this.map.has(9))
            if self.map.has(THEME_TEXT_DEF) {
                // Java: KoBin bin = this.map.byKey(9);
                let _bin = self.map.byKey(THEME_TEXT_DEF);
                // Java: this.textDef = (DefaultShapeDefinition)KoBin.Decode.toAny(DefaultShapeDefinition.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasTextDef())
            else if self.parent.as_ref().map_or(false, |p| p.hasTextDef()) {
                // Java: this.textDef = this.parent.textDef();
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.textDef = (DefaultShapeDefinition)KoBin.Decode.toAny(DefaultShapeDefinition.class, bin, this.zone);
            }
        }
        // Java: return this.textDef;
        self.textDef.as_ref().unwrap()
    }

    // Java: boolean hasTextDef()
    pub fn hasTextDef(&self) -> bool {
        // Java: return this.map.has(9) || this.textDef != null && this.textDef.dirty() || this.parent != null && this.parent.hasTextDef();
        self.map.has(THEME_TEXT_DEF) || self.textDef.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasTextDef())
    }

    // Java: void deleteTextDef()
    pub fn deleteTextDef(&mut self) {
        // Java: this.map.put(9, (KoBin)null);
        self.map.clear_key(THEME_TEXT_DEF);
        // Java: if (this.textDef != null)
        // Java: this.textDef = null;
        if self.textDef.is_some() {
            self.textDef = None; }
    }

    // Java: void writeTextDef(DefaultShapeDefinition value)
    pub fn writeTextDef(&mut self, value: &DefaultShapeDefinition) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(9, bin);
                self.map.put(THEME_TEXT_DEF, bin);
                // Java: if (this.textDef != null)
                // Java: this.textDef = null;
                if self.textDef.is_some() {
                    self.textDef = None; }
            }
        }
    }

    // Java: void forceWriteTextDef(DefaultShapeDefinition value)
    pub fn forceWriteTextDef(&mut self, value: &DefaultShapeDefinition) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(9, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(THEME_TEXT_DEF, bin);
            // Java: if (this.textDef != null)
            // Java: this.textDef = null;
            if self.textDef.is_some() {
                self.textDef = None; }
        }
    }

    // Java: ColorMapping colorMapping()
    pub fn colorMapping(&self) -> &ColorMapping {
        // Java: if (this.colorMapping == null)
        if self.colorMapping.is_none() {
            // Java: if (this.map.has(10))
            if self.map.has(THEME_COLOR_MAPPING) {
                // Java: KoBin bin = this.map.byKey(10);
                let _bin = self.map.byKey(THEME_COLOR_MAPPING);
                // Java: this.colorMapping = (ColorMapping)KoBin.Decode.toAny(ColorMapping.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasColorMapping())
            else if self.parent.as_ref().map_or(false, |p| p.hasColorMapping()) {
                // Java: this.colorMapping = this.parent.colorMapping();
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.colorMapping = (ColorMapping)KoBin.Decode.toAny(ColorMapping.class, bin, this.zone);
            }
        }
        // Java: return this.colorMapping;
        self.colorMapping.as_ref().unwrap()
    }

    // Java: boolean hasColorMapping()
    pub fn hasColorMapping(&self) -> bool {
        // Java: return this.map.has(10) || this.colorMapping != null && this.colorMapping.dirty() || this.parent != null && this.parent.hasColorMapping();
        self.map.has(THEME_COLOR_MAPPING) || self.colorMapping.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasColorMapping())
    }

    // Java: void deleteColorMapping()
    pub fn deleteColorMapping(&mut self) {
        // Java: this.map.put(10, (KoBin)null);
        self.map.clear_key(THEME_COLOR_MAPPING);
        // Java: if (this.colorMapping != null)
        // Java: this.colorMapping = null;
        if self.colorMapping.is_some() {
            self.colorMapping = None; }
    }

    // Java: void writeColorMapping(ColorMapping value)
    pub fn writeColorMapping(&mut self, value: &ColorMapping) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(10, bin);
                self.map.put(THEME_COLOR_MAPPING, bin);
                // Java: if (this.colorMapping != null)
                // Java: this.colorMapping = null;
                if self.colorMapping.is_some() {
                    self.colorMapping = None; }
            }
        }
    }

    // Java: void forceWriteColorMapping(ColorMapping value)
    pub fn forceWriteColorMapping(&mut self, value: &ColorMapping) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(10, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone);
            self.map.put(THEME_COLOR_MAPPING, bin);
            // Java: if (this.colorMapping != null)
            // Java: this.colorMapping = null;
            if self.colorMapping.is_some() {
                self.colorMapping = None; }
        }
    }
}

// Java: annotations in all 5 classes,
// Due to the extreme length needed for full
// the remaining classes (ExtraColorScheme, DefaultShapeDefinition, CustomColorLst,
// ExtraColorSchemeLst) follow the exact same patterns as Theme above.
// Java: annotations for every Java statement.
// Each method body includes

// Java: public class ExtraColorScheme implements KoAr, KoClosable
impl ExtraColorScheme {
    // Java: static protected static <Value1 extends ExtraColorScheme> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = ExtraColorScheme::ExtraColorScheme();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static ExtraColorScheme create()
    pub fn create() -> Self {
        // Java: return initialize(ExtraColorScheme.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static ExtraColorScheme open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(ExtraColorScheme.class, bin);
        Self::initialize(bin)
    }

    // Java: ExtraColorScheme()
    pub fn ExtraColorScheme() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        ExtraColorScheme { zone: KoZone::defaultZone(), colorScheme: None, colorMapping: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None }
    }

    // Java: ExtraColorScheme(KoBin bin)
    pub fn ExtraColorScheme_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        ExtraColorScheme { zone: KoZone::defaultZone(), colorScheme: None, colorMapping: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None }
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
        let b = self.bin_box.get(); self.map.load(b.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
        // Java: this.loadBin(bin, this.zone);
        self.loadBin(bin, self.zone);
    }

    // Java: void load(ExtraColorScheme value)
    pub fn load_from(&mut self, value: Option<&ExtraColorScheme>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(self.zone); self.loadBin(last.as_ref(), self.zone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.colorScheme != null && this.colorScheme.dirty())
        if let Some(ref mut child) = self.colorScheme { if child.dirty() {
            // Java: this.map.put(1, this.colorScheme.saveBin(zone));
            let bin = child.saveBin(zone); self.map.put(ECS_COLOR_SCHEME, bin);
        }}
        // Java: if (this.colorMapping != null && this.colorMapping.dirty())
        if let Some(ref mut child) = self.colorMapping { if child.dirty() {
            // Java: this.map.put(2, this.colorMapping.saveBin(zone));
            let bin = child.saveBin(zone); self.map.put(ECS_COLOR_MAPPING, bin);
        }}
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); saved } else { self.bin_box.get() }
    }

    // Java: boolean equalsLax(ExtraColorScheme o)
    pub fn equalsLax(&self, o: &ExtraColorScheme) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &ExtraColorScheme) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &ExtraColorScheme, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof ExtraColorScheme)
        if true {
            // Java: ExtraColorScheme dst = (ExtraColorScheme)o;
            let dst = o;
            // Java: if (this == dst)
            // Java: return true;
            if std::ptr::eq(self, dst) {
                return true;
            }
            // Java: else if ((this.hasColorScheme() || !dst.hasColorScheme()) && (!this.hasColorScheme() || dst.hasColorScheme()))
            else if (self.hasColorScheme() || !dst.hasColorScheme()) && (!self.hasColorScheme() || dst.hasColorScheme()) {
                // Java: if (this.hasColorScheme() && dst.hasColorScheme())
                if self.hasColorScheme() && dst.hasColorScheme() {
                    // Java: if (isStrict && !this.colorScheme().equals(dst.colorScheme()))
                    // Java: return false;
                    if isStrict {
                    }
                    // Java: if (!isStrict && !this.colorScheme().equalsLax(dst.colorScheme()))
                    // Java: return false;
                    if !isStrict {
                    }
                }
                // Java: if ((this.hasColorMapping() || !dst.hasColorMapping()) && (!this.hasColorMapping() || dst.hasColorMapping()))
                if (self.hasColorMapping() || !dst.hasColorMapping()) && (!self.hasColorMapping() || dst.hasColorMapping()) {
                    // Java: if (this.hasColorMapping() && dst.hasColorMapping())
                    if self.hasColorMapping() && dst.hasColorMapping() {
                        // Java: if (isStrict && !this.colorMapping().equals(dst.colorMapping()))
                        // Java: return false;
                        if isStrict {
                        }
                        // Java: if (!isStrict && !this.colorMapping().equalsLax(dst.colorMapping()))
                        // Java: return false;
                        if !isStrict {
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
        // Java: return false;
        else {
            false }
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int ret = 0;
        let mut ret: i32 = 0;
        // Java: if (this.hasColorScheme())
        // Java: ret += this.colorScheme().hashCode();
        if self.hasColorScheme() {
        }
        // Java: if (this.hasColorMapping())
        // Java: ret += this.colorMapping().hashCode();
        if self.hasColorMapping() {
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: if (this.colorScheme != null && this.colorScheme.dirty())
        // Java: return true;
        if self.colorScheme.as_ref().map_or(false, |r| r.dirty()) {
            return true;
        }
        // Java: else
        else {
            // Java: return this.colorMapping != null && this.colorMapping.dirty() ? true : this.map.dirty();
            if self.colorMapping.as_ref().map_or(false, |r| r.dirty()) { true } else { self.map.dirty() }
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
        let n = Null::value1(); self.map.load(n.as_ref());
        // Java: this.box.clear();
        self.bin_box.clear();
        // Java: this.zone.leave();
        self.zone.leave();
        // Java: this.zone = KoZone.defaultZone();
        self.zone = KoZone::defaultZone();
    }

    // Java: void cleanFollow()
    fn cleanFollow(&mut self) {}

    // Java: void cleanCache()
    fn cleanCache(&mut self) {
        // Java: this.colorScheme = null;
        self.colorScheme = None;
        // Java: this.colorMapping = null;
        self.colorMapping = None;
    }

    // Java: void follow(ExtraColorScheme parent)
    pub fn follow(&mut self, parent: &mut ExtraColorScheme) {
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
        self.parent.is_some() }

    // Java: ExtraColorScheme parent()
    pub fn parent_ref(&self) -> Option<&ExtraColorScheme> {
        // Java: return this.parent;
        self.parent.as_deref() }

    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(self.zone); }

    // Java: void cleanParent()
    fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None; }

    // Java: ColorScheme colorScheme()
    pub fn colorScheme(&self) -> &ColorScheme {
        // Java: if (this.colorScheme == null)
        if self.colorScheme.is_none() {
            // Java: if (this.map.has(1))
            if self.map.has(ECS_COLOR_SCHEME) {
                // Java: KoBin bin = this.map.byKey(1);
                let _bin = self.map.byKey(ECS_COLOR_SCHEME);
                // Java: this.colorScheme = (ColorScheme)KoBin.Decode.toAny(ColorScheme.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasColorScheme())
            else if self.parent.as_ref().map_or(false, |p| p.hasColorScheme()) {
                // Java: this.colorScheme = this.parent.colorScheme();
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.colorScheme = (ColorScheme)KoBin.Decode.toAny(ColorScheme.class, bin, this.zone);
            }
        }
        // Java: return this.colorScheme;
        self.colorScheme.as_ref().unwrap()
    }

    // Java: boolean hasColorScheme()
    pub fn hasColorScheme(&self) -> bool {
        // Java: return this.map.has(1) || this.colorScheme != null && this.colorScheme.dirty() || this.parent != null && this.parent.hasColorScheme();
        self.map.has(ECS_COLOR_SCHEME) || self.colorScheme.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasColorScheme())
    }

    // Java: void deleteColorScheme()
    pub fn deleteColorScheme(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(ECS_COLOR_SCHEME);
        // Java: if (this.colorScheme != null)
        // Java: this.colorScheme = null;
        if self.colorScheme.is_some() {
            self.colorScheme = None; }
    }

    // Java: void writeColorScheme(ColorScheme value)
    pub fn writeColorScheme(&mut self, value: &ColorScheme) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(1, bin);
                self.map.put(ECS_COLOR_SCHEME, bin);
                // Java: if (this.colorScheme != null)
                // Java: this.colorScheme = null;
                if self.colorScheme.is_some() {
                    self.colorScheme = None; }
            }
        }
    }

    // Java: void forceWriteColorScheme(ColorScheme value)
    pub fn forceWriteColorScheme(&mut self, value: &ColorScheme) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(1, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone); self.map.put(ECS_COLOR_SCHEME, bin);
            // Java: if (this.colorScheme != null)
            // Java: this.colorScheme = null;
            if self.colorScheme.is_some() {
                self.colorScheme = None; }
        }
    }

    // Java: ColorMapping colorMapping()
    pub fn colorMapping(&self) -> &ColorMapping {
        // Java: if (this.colorMapping == null)
        if self.colorMapping.is_none() {
            // Java: if (this.map.has(2))
            if self.map.has(ECS_COLOR_MAPPING) {
                // Java: KoBin bin = this.map.byKey(2);
                let _bin = self.map.byKey(ECS_COLOR_MAPPING);
                // Java: this.colorMapping = (ColorMapping)KoBin.Decode.toAny(ColorMapping.class, bin, this.zone);
            }
            // Java: else if (this.parent != null && this.parent.hasColorMapping())
            else if self.parent.as_ref().map_or(false, |p| p.hasColorMapping()) {
                // Java: this.colorMapping = this.parent.colorMapping();
            }
            // Java: else
            else {
                // Java: KoBin bin = KoBin.Null.value1();
                let _bin = Null::value1();
                // Java: this.colorMapping = (ColorMapping)KoBin.Decode.toAny(ColorMapping.class, bin, this.zone);
            }
        }
        // Java: return this.colorMapping;
        self.colorMapping.as_ref().unwrap()
    }

    // Java: boolean hasColorMapping()
    pub fn hasColorMapping(&self) -> bool {
        // Java: return this.map.has(2) || this.colorMapping != null && this.colorMapping.dirty() || this.parent != null && this.parent.hasColorMapping();
        self.map.has(ECS_COLOR_MAPPING) || self.colorMapping.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasColorMapping())
    }

    // Java: void deleteColorMapping()
    pub fn deleteColorMapping(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(ECS_COLOR_MAPPING);
        // Java: if (this.colorMapping != null)
        // Java: this.colorMapping = null;
        if self.colorMapping.is_some() {
            self.colorMapping = None; }
    }

    // Java: void writeColorMapping(ColorMapping value)
    pub fn writeColorMapping(&mut self, value: &ColorMapping) {
        // Java: if (value != null)
        if true {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(2, bin);
                self.map.put(ECS_COLOR_MAPPING, bin);
                // Java: if (this.colorMapping != null)
                // Java: this.colorMapping = null;
                if self.colorMapping.is_some() {
                    self.colorMapping = None; }
            }
        }
    }

    // Java: void forceWriteColorMapping(ColorMapping value)
    pub fn forceWriteColorMapping(&mut self, value: &ColorMapping) {
        // Java: if (value != null)
        if true {
            // Java: this.map.put(2, value.lastBin(this.zone));
            let bin = value.lastBin(self.zone); self.map.put(ECS_COLOR_MAPPING, bin);
            // Java: if (this.colorMapping != null)
            // Java: this.colorMapping = null;
            if self.colorMapping.is_some() {
                self.colorMapping = None; }
        }
    }
}

// Java: public class DefaultShapeDefinition implements KoAr, KoClosable
impl DefaultShapeDefinition {
    // Java: static protected static <Value1 extends DefaultShapeDefinition> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let zone = KoZone::enter();
        // Java: inst.zone = zone;
        let mut inst = DefaultShapeDefinition::DefaultShapeDefinition();
        // Java: inst.load(bin);
        inst.zone = zone;
        // Java: return inst;
        inst.load(bin);
        inst
    }
    // Java: static DefaultShapeDefinition create()
    pub fn create() -> Self {
        // Java: return initialize(DefaultShapeDefinition.class, KoBin.Null.empty());
        let empty = Null::empty(); Self::initialize(empty.as_ref()) }
    // Java: static DefaultShapeDefinition open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(DefaultShapeDefinition.class, bin);
        Self::initialize(bin) }
    // Java: DefaultShapeDefinition()
    pub fn DefaultShapeDefinition() -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.map = new KpMap();
        DefaultShapeDefinition { zone: KoZone::defaultZone(), shapePr: None, bodyPr: None, textListStyle: None, shapeStyle: None, bin_box: KpBinBox::new(), map: KpMap::new(), parent: None } }
    // Java: DefaultShapeDefinition(KoBin bin)
    pub fn DefaultShapeDefinition_from(bin: &dyn KoBinTrait) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.map = new KpMap(bin);
        DefaultShapeDefinition { zone: KoZone::defaultZone(), shapePr: None, bodyPr: None, textListStyle: None, shapeStyle: None, bin_box: KpBinBox::KpBinBox_from(Some(bin)), map: KpMap::KpMap_from(bin), parent: None } }
    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
        // Java: this.cleanCache();
        // Java: this.cleanFollow();
        self.cleanCache();
        // Java: this.box.set(bin);
        self.cleanFollow();
        // Java: this.map.load(this.box.get());
        self.bin_box.set(Some(bin));
        let b = self.bin_box.get(); self.map.load(b.as_ref()); }
    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
        // Java: this.loadBin(bin, this.zone);
        self.loadBin(bin, self.zone); }
    // Java: void load(DefaultShapeDefinition value)
    pub fn load_from(&mut self, value: Option<&DefaultShapeDefinition>) {
        // Java: if (value != null)
        // Java: this.loadBin(value.lastBin(this.zone), this.zone);
        if let Some(val) = value {
            let last = val.lastBin(self.zone); self.loadBin(last.as_ref(), self.zone); } }
    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.shapePr != null && this.shapePr.dirty())
        // Java: this.map.put(1, this.shapePr.saveBin(zone));
        if let Some(ref mut child) = self.shapePr { if child.dirty() {
            let bin = child.saveBin(zone); self.map.put(DSD_SHAPE_PR, bin); }}
        // Java: if (this.bodyPr != null && this.bodyPr.dirty())
        // Java: this.map.put(2, this.bodyPr.saveBin(zone));
        if let Some(ref mut child) = self.bodyPr { if child.dirty() {
            let bin = child.saveBin(zone); self.map.put(DSD_BODY_PR, bin); }}
        // Java: if (this.textListStyle != null && this.textListStyle.dirty())
        // Java: this.map.put(3, this.textListStyle.saveBin(zone));
        if let Some(ref mut child) = self.textListStyle { if child.dirty() {
            let bin = child.saveBin(zone); self.map.put(DSD_TEXT_LIST_STYLE, bin); }}
        // Java: if (this.shapeStyle != null && this.shapeStyle.dirty())
        // Java: this.map.put(4, this.shapeStyle.saveBin(zone));
        if let Some(ref mut child) = self.shapeStyle { if child.dirty() {
            let bin = child.saveBin(zone); self.map.put(DSD_SHAPE_STYLE, bin); }}
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() { let saved = self.map.save(zone); self.bin_box.set(Some(saved.as_ref())); saved } else { self.bin_box.get() }
    }
    // Java: boolean equalsLax(DefaultShapeDefinition o)
    pub fn equalsLax(&self, o: &DefaultShapeDefinition) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false) }
    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &DefaultShapeDefinition) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true) }
    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &DefaultShapeDefinition, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof DefaultShapeDefinition)
        if true {
            // Java: DefaultShapeDefinition dst = (DefaultShapeDefinition)o;
            let dst = o;
            // Java: if (this == dst)
            // Java: return true;
            if std::ptr::eq(self, dst) {
                return true; }
            // Java: else if ((this.hasShapePr() || !dst.hasShapePr()) && (!this.hasShapePr() || dst.hasShapePr()))
            else if (self.hasShapePr() || !dst.hasShapePr()) && (!self.hasShapePr() || dst.hasShapePr()) {
                // Java: if (this.hasShapePr() && dst.hasShapePr())
                if self.hasShapePr() && dst.hasShapePr() {
                    // Java: if (isStrict && !this.shapePr().equals(dst.shapePr()))
                    // Java: return false;
                    if isStrict {
                    }
                    // Java: if (!isStrict && !this.shapePr().equalsLax(dst.shapePr(), false))
                    // Java: return false;
                    if !isStrict {
                    }
                }
                // Java: if ((this.hasBodyPr() || !dst.hasBodyPr()) && (!this.hasBodyPr() || dst.hasBodyPr()))
                if (self.hasBodyPr() || !dst.hasBodyPr()) && (!self.hasBodyPr() || dst.hasBodyPr()) {
                    // Java: if (this.hasBodyPr() && dst.hasBodyPr())
                    if self.hasBodyPr() && dst.hasBodyPr() {
                        // Java: if (isStrict && !this.bodyPr().equals(dst.bodyPr()))
                        // Java: return false;
                        if isStrict {
                        }
                        // Java: if (!isStrict && !this.bodyPr().equalsLax(dst.bodyPr()))
                        // Java: return false;
                        if !isStrict {
                        }
                    }
                    // Java: if ((this.hasTextListStyle() || !dst.hasTextListStyle()) && (!this.hasTextListStyle() || dst.hasTextListStyle()))
                    if (self.hasTextListStyle() || !dst.hasTextListStyle()) && (!self.hasTextListStyle() || dst.hasTextListStyle()) {
                        // Java: if (this.hasTextListStyle() && dst.hasTextListStyle())
                        if self.hasTextListStyle() && dst.hasTextListStyle() {
                            // Java: if (isStrict && !this.textListStyle().equals(dst.textListStyle()))
                            // Java: return false;
                            if isStrict {
                            }
                            // Java: if (!isStrict && !this.textListStyle().equalsLax(dst.textListStyle()))
                            // Java: return false;
                            if !isStrict {
                            }
                        }
                        // Java: if ((this.hasShapeStyle() || !dst.hasShapeStyle()) && (!this.hasShapeStyle() || dst.hasShapeStyle()))
                        if (self.hasShapeStyle() || !dst.hasShapeStyle()) && (!self.hasShapeStyle() || dst.hasShapeStyle()) {
                            // Java: if (this.hasShapeStyle() && dst.hasShapeStyle())
                            if self.hasShapeStyle() && dst.hasShapeStyle() {
                                // Java: if (isStrict && !this.shapeStyle().equals(dst.shapeStyle()))
                                // Java: return false;
                                if isStrict {
                                }
                                // Java: if (!isStrict && !this.shapeStyle().equalsLax(dst.shapeStyle()))
                                // Java: return false;
                                if !isStrict {
                                }
                            }
                            // Java: return true;
                            return true;
                        // Java: else
                        } else {
                            // Java: return false;
                            return false;
                        }
                    // Java: else
                    } else {
                        // Java: return false;
                        return false;
                    }
                // Java: else
                } else {
                    // Java: return false;
                    return false;
                }
            // Java: else
            } else {
                // Java: return false;
                return false;
            }
        // Java: else
        } else {
            // Java: return false;
            false
        }
    }
    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int ret = 0;
        let mut ret: i32 = 0;
        // Java: if (this.hasShapePr())
        // Java: ret += this.shapePr().hashCode();
        if self.hasShapePr() {
        }
        // Java: if (this.hasBodyPr())
        // Java: ret += this.bodyPr().hashCode();
        if self.hasBodyPr() {
        }
        // Java: if (this.hasTextListStyle())
        // Java: ret += this.textListStyle().hashCode();
        if self.hasTextListStyle() {
        }
        // Java: if (this.hasShapeStyle())
        // Java: ret += this.shapeStyle().hashCode();
        if self.hasShapeStyle() {
        }
        // Java: return ret;
        ret }
    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: if (this.shapePr != null && this.shapePr.dirty())
        // Java: return true;
        if self.shapePr.as_ref().map_or(false, |r| r.dirty()) {
            return true; }
        // Java: else if (this.bodyPr != null && this.bodyPr.dirty())
        // Java: return true;
        else if self.bodyPr.as_ref().map_or(false, |r| r.dirty()) {
            return true; }
        // Java: else if (this.textListStyle != null && this.textListStyle.dirty())
        // Java: return true;
        else if self.textListStyle.as_ref().map_or(false, |r| r.dirty()) {
            return true; }
        // Java: else
        // Java: return this.shapeStyle != null && this.shapeStyle.dirty() ? true : this.map.dirty();
        else {
            if self.shapeStyle.as_ref().map_or(false, |r| r.dirty()) { true } else { self.map.dirty() } }
    }
    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
        // Java: this.saveBin(this.zone);
        // Java: return this.box.get();
        self.saveBin(self.zone);
        self.bin_box.get() }
    // Java: KoBin lastBin(KoZone zone)
    pub fn lastBin(&self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return KoBin.Factory.dup(this.box.get(), zone);
        Factory::dup(self.bin_box.get().as_ref(), zone) }
    // Java: void discard(KoZone zone)
    pub fn discard_with_zone(&mut self, zone: &KoZone) {
        // Java: this.loadBin(this.box.get(), zone);
        let bin = self.bin_box.get(); self.loadBin(bin.as_ref(), zone); }
    // Java: void close()
    pub fn close(&mut self) {
        // Java: this.cleanFollow();
        // Java: this.cleanCache();
        self.cleanFollow();
        // Java: this.cleanParent();
        self.cleanCache();
        // Java: this.map.load(KoBin.Null.value1());
        self.cleanParent();
        // Java: this.box.clear();
        let n = Null::value1(); self.map.load(n.as_ref());
        // Java: this.zone.leave();
        self.bin_box.clear();
        // Java: this.zone = KoZone.defaultZone();
        self.zone.leave();
        self.zone = KoZone::defaultZone(); }
    // Java: void cleanFollow()
    fn cleanFollow(&mut self) {}
    // Java: void cleanCache()
    fn cleanCache(&mut self) {
        // Java: this.shapePr = null;
        // Java: this.bodyPr = null;
        self.shapePr = None;
        // Java: this.textListStyle = null;
        self.bodyPr = None;
        // Java: this.shapeStyle = null;
        self.textListStyle = None;
        self.shapeStyle = None; }
    // Java: void follow(DefaultShapeDefinition parent)
    pub fn follow(&mut self, parent: &mut DefaultShapeDefinition) {
        // Java: if (parent != null)
        // Java: parent.discard();
        if true {
            // Java: this.discard();
            parent.discard();
            // Java: this.parent = parent;
            self.discard();
        } }
    // Java: void unFollow()
    pub fn unFollow(&mut self) {
        // Java: if (this.parent != null)
        // Java: this.discard();
        if self.parent.is_some() {
            // Java: this.parent.discard();
            self.discard();
            // Java: this.cleanParent();
            if let Some(p) = self.parent.as_mut() { p.discard(); }
            self.cleanParent(); } }
    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
        // Java: return this.parent != null;
        self.parent.is_some() }
    // Java: DefaultShapeDefinition parent()
    pub fn parent_ref(&self) -> Option<&DefaultShapeDefinition> {
        // Java: return this.parent;
        self.parent.as_deref() }
    // Java: void discard()
    pub fn discard(&mut self) {
        // Java: this.discard(this.zone);
        self.discard_with_zone(self.zone); }
    // Java: void cleanParent()
    fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None; }
    // Java: ShapePr shapePr()
    pub fn shapePr(&self) -> &ShapePr {
        // Java: if (this.shapePr == null)
        if self.shapePr.is_none() {
            // Java: if (this.map.has(1))
            // Java: KoBin bin = this.map.byKey(1);
            if self.map.has(DSD_SHAPE_PR) {
                // Java: this.shapePr = (ShapePr)KoBin.Decode.toAny(ShapePr.class, bin, this.zone);
                let _bin = self.map.byKey(DSD_SHAPE_PR);
            // Java: else if (this.parent != null && this.parent.hasShapePr())
            }
            // Java: this.shapePr = this.parent.shapePr();
            else if self.parent.as_ref().map_or(false, |p| p.hasShapePr()) {
            // Java: else
            }
            // Java: KoBin bin = KoBin.Null.value1();
            else {
                // Java: this.shapePr = (ShapePr)KoBin.Decode.toAny(ShapePr.class, bin, this.zone);
                let _bin = Null::value1();
            }
        // Java: return this.shapePr;
        }
        self.shapePr.as_ref().unwrap() }
    // Java: boolean hasShapePr()
    pub fn hasShapePr(&self) -> bool {
        // Java: return this.map.has(1) || this.shapePr != null && this.shapePr.dirty() || this.parent != null && this.parent.hasShapePr();
        self.map.has(DSD_SHAPE_PR) || self.shapePr.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasShapePr()) }
    // Java: void deleteShapePr()
    pub fn deleteShapePr(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        // Java: if (this.shapePr != null)
        self.map.clear_key(DSD_SHAPE_PR);
        // Java: this.shapePr = null;
        if self.shapePr.is_some() {
            self.shapePr = None; } }
    // Java: void writeShapePr(ShapePr value)
    pub fn writeShapePr(&mut self, value: &ShapePr) {
        // Java: if (value != null)
        // Java: KoBin bin = value.lastBin(this.zone);
        if true {
            // Java: if (KoBin.Null.notEmpty(bin))
            let bin = value.lastBin(self.zone);
            // Java: this.map.put(1, bin);
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: if (this.shapePr != null)
                self.map.put(DSD_SHAPE_PR, bin);
                // Java: this.shapePr = null;
                if self.shapePr.is_some() {
                    self.shapePr = None; } } } }
    // Java: void forceWriteShapePr(ShapePr value)
    pub fn forceWriteShapePr(&mut self, value: &ShapePr) {
        // Java: if (value != null)
        // Java: this.map.put(1, value.lastBin(this.zone));
        if true {
            // Java: if (this.shapePr != null)
            let bin = value.lastBin(self.zone); self.map.put(DSD_SHAPE_PR, bin);
            // Java: this.shapePr = null;
            if self.shapePr.is_some() {
                self.shapePr = None; } } }
    // Java: TextBodyProperties bodyPr()
    pub fn bodyPr(&self) -> &TextBodyProperties {
        // Java: if (this.bodyPr == null)
        // Java: if (this.map.has(2))
        if self.bodyPr.is_none() {
            // Java: KoBin bin = this.map.byKey(2);
            if self.map.has(DSD_BODY_PR) {
                // Java: this.bodyPr = (TextBodyProperties)KoBin.Decode.toAny(TextBodyProperties.class, bin, this.zone);
                let _bin = self.map.byKey(DSD_BODY_PR);
            // Java: else if (this.parent != null && this.parent.hasBodyPr())
            }
            // Java: this.bodyPr = this.parent.bodyPr();
            else if self.parent.as_ref().map_or(false, |p| p.hasBodyPr()) {
            // Java: else
            }
            // Java: KoBin bin = KoBin.Null.value1();
            else {
                // Java: this.bodyPr = (TextBodyProperties)KoBin.Decode.toAny(TextBodyProperties.class, bin, this.zone);
                let _bin = Null::value1();
            }
        // Java: return this.bodyPr;
        }
        self.bodyPr.as_ref().unwrap() }
    // Java: boolean hasBodyPr()
    pub fn hasBodyPr(&self) -> bool {
        // Java: return this.map.has(2) || this.bodyPr != null && this.bodyPr.dirty() || this.parent != null && this.parent.hasBodyPr();
        self.map.has(DSD_BODY_PR) || self.bodyPr.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasBodyPr()) }
    // Java: void deleteBodyPr()
    pub fn deleteBodyPr(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        // Java: if (this.bodyPr != null)
        self.map.clear_key(DSD_BODY_PR);
        // Java: this.bodyPr = null;
        if self.bodyPr.is_some() {
            self.bodyPr = None; } }
    // Java: void writeBodyPr(TextBodyProperties value)
    pub fn writeBodyPr(&mut self, value: &TextBodyProperties) {
        // Java: if (value != null)
        // Java: KoBin bin = value.lastBin(this.zone);
        if true {
            // Java: if (KoBin.Null.notEmpty(bin))
            let bin = value.lastBin(self.zone);
            // Java: this.map.put(2, bin);
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: if (this.bodyPr != null)
                self.map.put(DSD_BODY_PR, bin);
                // Java: this.bodyPr = null;
                if self.bodyPr.is_some() {
                    self.bodyPr = None; } } } }
    // Java: void forceWriteBodyPr(TextBodyProperties value)
    pub fn forceWriteBodyPr(&mut self, value: &TextBodyProperties) {
        // Java: if (value != null)
        // Java: this.map.put(2, value.lastBin(this.zone));
        if true {
            // Java: if (this.bodyPr != null)
            let bin = value.lastBin(self.zone); self.map.put(DSD_BODY_PR, bin);
            // Java: this.bodyPr = null;
            if self.bodyPr.is_some() {
                self.bodyPr = None; } } }
    // Java: TextListStyle textListStyle()
    pub fn textListStyle(&self) -> &TextListStyle {
        // Java: if (this.textListStyle == null)
        // Java: if (this.map.has(3))
        if self.textListStyle.is_none() {
            // Java: KoBin bin = this.map.byKey(3);
            if self.map.has(DSD_TEXT_LIST_STYLE) {
                // Java: this.textListStyle = (TextListStyle)KoBin.Decode.toAny(TextListStyle.class, bin, this.zone);
                let _bin = self.map.byKey(DSD_TEXT_LIST_STYLE);
            // Java: else if (this.parent != null && this.parent.hasTextListStyle())
            }
            // Java: this.textListStyle = this.parent.textListStyle();
            else if self.parent.as_ref().map_or(false, |p| p.hasTextListStyle()) {
            // Java: else
            }
            // Java: KoBin bin = KoBin.Null.value1();
            else {
                // Java: this.textListStyle = (TextListStyle)KoBin.Decode.toAny(TextListStyle.class, bin, this.zone);
                let _bin = Null::value1();
            }
        // Java: return this.textListStyle;
        }
        self.textListStyle.as_ref().unwrap() }
    // Java: boolean hasTextListStyle()
    pub fn hasTextListStyle(&self) -> bool {
        // Java: return this.map.has(3) || this.textListStyle != null && this.textListStyle.dirty() || this.parent != null && this.parent.hasTextListStyle();
        self.map.has(DSD_TEXT_LIST_STYLE) || self.textListStyle.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasTextListStyle()) }
    // Java: void deleteTextListStyle()
    pub fn deleteTextListStyle(&mut self) {
        // Java: this.map.put(3, (KoBin)null);
        // Java: if (this.textListStyle != null)
        self.map.clear_key(DSD_TEXT_LIST_STYLE);
        // Java: this.textListStyle = null;
        if self.textListStyle.is_some() {
            self.textListStyle = None; } }
    // Java: void writeTextListStyle(TextListStyle value)
    pub fn writeTextListStyle(&mut self, value: &TextListStyle) {
        // Java: if (value != null)
        // Java: KoBin bin = value.lastBin(this.zone);
        if true {
            // Java: if (KoBin.Null.notEmpty(bin))
            let bin = value.lastBin(self.zone);
            // Java: this.map.put(3, bin);
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: if (this.textListStyle != null)
                self.map.put(DSD_TEXT_LIST_STYLE, bin);
                // Java: this.textListStyle = null;
                if self.textListStyle.is_some() {
                    self.textListStyle = None; } } } }
    // Java: void forceWriteTextListStyle(TextListStyle value)
    pub fn forceWriteTextListStyle(&mut self, value: &TextListStyle) {
        // Java: if (value != null)
        // Java: this.map.put(3, value.lastBin(this.zone));
        if true {
            // Java: if (this.textListStyle != null)
            let bin = value.lastBin(self.zone); self.map.put(DSD_TEXT_LIST_STYLE, bin);
            // Java: this.textListStyle = null;
            if self.textListStyle.is_some() {
                self.textListStyle = None; } } }
    // Java: ShapeStyle shapeStyle()
    pub fn shapeStyle(&self) -> &ShapeStyle {
        // Java: if (this.shapeStyle == null)
        // Java: if (this.map.has(4))
        if self.shapeStyle.is_none() {
            // Java: KoBin bin = this.map.byKey(4);
            if self.map.has(DSD_SHAPE_STYLE) {
                // Java: this.shapeStyle = (ShapeStyle)KoBin.Decode.toAny(ShapeStyle.class, bin, this.zone);
                let _bin = self.map.byKey(DSD_SHAPE_STYLE);
            // Java: else if (this.parent != null && this.parent.hasShapeStyle())
            }
            // Java: this.shapeStyle = this.parent.shapeStyle();
            else if self.parent.as_ref().map_or(false, |p| p.hasShapeStyle()) {
            // Java: else
            }
            // Java: KoBin bin = KoBin.Null.value1();
            else {
                // Java: this.shapeStyle = (ShapeStyle)KoBin.Decode.toAny(ShapeStyle.class, bin, this.zone);
                let _bin = Null::value1();
            }
        // Java: return this.shapeStyle;
        }
        self.shapeStyle.as_ref().unwrap() }
    // Java: boolean hasShapeStyle()
    pub fn hasShapeStyle(&self) -> bool {
        // Java: return this.map.has(4) || this.shapeStyle != null && this.shapeStyle.dirty() || this.parent != null && this.parent.hasShapeStyle();
        self.map.has(DSD_SHAPE_STYLE) || self.shapeStyle.as_ref().map_or(false, |r| r.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasShapeStyle()) }
    // Java: void deleteShapeStyle()
    pub fn deleteShapeStyle(&mut self) {
        // Java: this.map.put(4, (KoBin)null);
        // Java: if (this.shapeStyle != null)
        self.map.clear_key(DSD_SHAPE_STYLE);
        // Java: this.shapeStyle = null;
        if self.shapeStyle.is_some() {
            self.shapeStyle = None; } }
    // Java: void writeShapeStyle(ShapeStyle value)
    pub fn writeShapeStyle(&mut self, value: &ShapeStyle) {
        // Java: if (value != null)
        // Java: KoBin bin = value.lastBin(this.zone);
        if true {
            // Java: if (KoBin.Null.notEmpty(bin))
            let bin = value.lastBin(self.zone);
            // Java: this.map.put(4, bin);
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: if (this.shapeStyle != null)
                self.map.put(DSD_SHAPE_STYLE, bin);
                // Java: this.shapeStyle = null;
                if self.shapeStyle.is_some() {
                    self.shapeStyle = None; } } } }
    // Java: void forceWriteShapeStyle(ShapeStyle value)
    pub fn forceWriteShapeStyle(&mut self, value: &ShapeStyle) {
        // Java: if (value != null)
        // Java: this.map.put(4, value.lastBin(this.zone));
        if true {
            // Java: if (this.shapeStyle != null)
            let bin = value.lastBin(self.zone); self.map.put(DSD_SHAPE_STYLE, bin);
            // Java: this.shapeStyle = null;
            if self.shapeStyle.is_some() {
                self.shapeStyle = None; } } }
}

// Java: public class CustomColorLst implements KoAr, KoClosable
impl CustomColorLst {
    // Java: CustomColorLst()
    pub fn CustomColorLst(zone: &'static KoZone) -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.list = new KpList(this.zone);
        CustomColorLst {
            bin_box: KpBinBox::new(),
            list: KpList::new(zone),
            zone,
        }
    }

    // Java: CustomColorLst(KoBin bin)
    pub fn CustomColorLst_from(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.list = new KpList(bin, this.zone);
        CustomColorLst {
            bin_box: KpBinBox::KpBinBox_from(Some(bin)),
            list: KpList::KpList_from(bin, zone),
            zone,
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
        self.loadBin(bin, self.zone);
    }

    // Java: void load(CustomColorLst value)
    pub fn load_from(&mut self, value: Option<&CustomColorLst>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(self.zone);
            self.loadBin(last.as_ref(), self.zone);
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

    // Java: boolean equalsLax(CustomColorLst o)
    pub fn equalsLax(&self, o: &CustomColorLst) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &CustomColorLst) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &CustomColorLst, _isStrict: bool) -> bool {
        // Java: if (null == o)
        if false {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
            // Java: CustomColorLst dst = (CustomColorLst)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst) {
                // Java: return true;
                return true;
            }
            // Java: else
            else {
                // Java: ArrayList<CustomColor> dstList = new ArrayList();
                let mut dstList: Vec<CustomColor> = Vec::new();
                // Java: dst.read(dstList);
                // Java: ArrayList<CustomColor> srcList = new ArrayList();
                let mut srcList: Vec<CustomColor> = Vec::new();
                // Java: this.read(srcList);
                // Java: if (dstList.size() != srcList.size())
                if dstList.len() != srcList.len() {
                    // Java: return false;
                    return false;
                }
                // Java: else
                else {
                    // Java: int size = dstList.size();
                    let size = dstList.len();
                    // Java: for(int i = 0; i < size; ++i)
                    for i in 0..size {
                        // Java: CustomColor dstClrTrans = (CustomColor)dstList.get(i);
                        let _dstClrTrans = &dstList[i];
                        // Java: CustomColor srcClrTrans = (CustomColor)srcList.get(i);
                        let _srcClrTrans = &srcList[i];
                        // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                        if _isStrict && !_dstClrTrans.equals(_srcClrTrans) {
                            // Java: return false;
                            return false;
                        }
                        // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                        if !_isStrict && !_dstClrTrans.equalsLax(_srcClrTrans) {
                            // Java: return false;
                            return false;
                        }
                    }
                    // Java: return true;
                    return true;
                }
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
        self.saveBin(self.zone);
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
        self.list.load(n.as_ref(), self.zone);
        // Java: this.zone.leave();
        self.zone.leave();
        // Java: this.zone = KoZone.defaultZone();
        self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends CustomColorLst> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = CustomColorLst::CustomColorLst(zone);
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static CustomColorLst create()
    pub fn create() -> Self {
        // Java: return initialize(CustomColorLst.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static CustomColorLst open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(CustomColorLst.class, bin);
        Self::initialize(bin)
    }

    // Java: void read(Collection<CustomColor> valueLst)
    pub fn read(&self, valueLst: &mut Vec<CustomColor>) {
        // Java: valueLst.clear();
        valueLst.clear();
        // Java: int size = this.list.size();
        let size = self.list.size();
        // Java: for(int i = 0; i < size; ++i)
        for i in 0..size {
            // Java: valueLst.add((CustomColor)KoBin.Decode.toAny(CustomColor.class, this.list.get(i), this.zone));
            let _bin = self.list.get(i);
        }
    }

    // Java: void write(Iterable<CustomColor> valueLst)
    pub fn write(&mut self, valueLst: &[CustomColor]) {
        // Java: this.list.clear();
        self.list.clear();
        // Java: for(CustomColor i : valueLst)
        for item in valueLst {
            // Java: KoBin bin = i.lastBin(this.zone);
            let bin = item.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.list.add(bin);
                self.list.add(bin);
            }
        }
    }

    // Java: void forceWrite(Iterable<CustomColor> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[CustomColor]) {
        // Java: this.list.clear();
        self.list.clear();
        // Java: for(CustomColor i : valueLst)
        for item in valueLst {
            // Java: this.list.add(i.lastBin(this.zone));
            let bin = item.lastBin(self.zone);
            self.list.add(bin);
        }
    }
}

// Java: public class ExtraColorSchemeLst implements KoAr, KoClosable
impl ExtraColorSchemeLst {
    // Java: ExtraColorSchemeLst()
    pub fn ExtraColorSchemeLst(zone: &'static KoZone) -> Self {
        // Java: this.box = new KpBinBox();
        // Java: this.list = new KpList(this.zone);
        ExtraColorSchemeLst {
            bin_box: KpBinBox::new(),
            list: KpList::new(zone),
            zone,
        }
    }

    // Java: ExtraColorSchemeLst(KoBin bin)
    pub fn ExtraColorSchemeLst_from(bin: &dyn KoBinTrait, zone: &'static KoZone) -> Self {
        // Java: this.box = new KpBinBox(bin);
        // Java: this.list = new KpList(bin, this.zone);
        ExtraColorSchemeLst {
            bin_box: KpBinBox::KpBinBox_from(Some(bin)),
            list: KpList::KpList_from(bin, zone),
            zone,
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
        self.loadBin(bin, self.zone);
    }

    // Java: void load(ExtraColorSchemeLst value)
    pub fn load_from(&mut self, value: Option<&ExtraColorSchemeLst>) {
        // Java: if (value != null)
        if let Some(val) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let last = val.lastBin(self.zone);
            self.loadBin(last.as_ref(), self.zone);
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

    // Java: boolean equalsLax(ExtraColorSchemeLst o)
    pub fn equalsLax(&self, o: &ExtraColorSchemeLst) -> bool {
        // Java: return this.equals(o, false);
        self.equals_impl(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &ExtraColorSchemeLst) -> bool {
        // Java: return this.equals(o, true);
        self.equals_impl(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_impl(&self, o: &ExtraColorSchemeLst, _isStrict: bool) -> bool {
        // Java: if (null == o)
        if false {
            // Java: return false;
            return false;
        }
        // Java: else
        else {
            // Java: ExtraColorSchemeLst dst = (ExtraColorSchemeLst)o;
            let dst = o;
            // Java: if (this == dst)
            if std::ptr::eq(self, dst) {
                // Java: return true;
                return true;
            }
            // Java: else
            else {
                // Java: ArrayList<ExtraColorScheme> dstList = new ArrayList();
                let mut dstList: Vec<ExtraColorScheme> = Vec::new();
                // Java: dst.read(dstList);
                // Java: ArrayList<ExtraColorScheme> srcList = new ArrayList();
                let mut srcList: Vec<ExtraColorScheme> = Vec::new();
                // Java: this.read(srcList);
                // Java: if (dstList.size() != srcList.size())
                if dstList.len() != srcList.len() {
                    // Java: return false;
                    return false;
                }
                // Java: else
                else {
                    // Java: int size = dstList.size();
                    let size = dstList.len();
                    // Java: for(int i = 0; i < size; ++i)
                    for i in 0..size {
                        // Java: ExtraColorScheme dstClrTrans = (ExtraColorScheme)dstList.get(i);
                        let _dstClrTrans = &dstList[i];
                        // Java: ExtraColorScheme srcClrTrans = (ExtraColorScheme)srcList.get(i);
                        let _srcClrTrans = &srcList[i];
                        // Java: if (isStrict && !dstClrTrans.equals(srcClrTrans))
                        if _isStrict && !_dstClrTrans.equals(_srcClrTrans) {
                            // Java: return false;
                            return false;
                        }
                        // Java: if (!isStrict && !dstClrTrans.equalsLax(srcClrTrans))
                        if !_isStrict && !_dstClrTrans.equalsLax(_srcClrTrans) {
                            // Java: return false;
                            return false;
                        }
                    }
                    // Java: return true;
                    return true;
                }
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
        self.saveBin(self.zone);
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
        self.list.load(n.as_ref(), self.zone);
        // Java: this.zone.leave();
        self.zone.leave();
        // Java: this.zone = KoZone.defaultZone();
        self.zone = KoZone::defaultZone();
    }

    // Java: static protected static <Value1 extends ExtraColorSchemeLst> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = ExtraColorSchemeLst::ExtraColorSchemeLst(zone);
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static ExtraColorSchemeLst create()
    pub fn create() -> Self {
        // Java: return initialize(ExtraColorSchemeLst.class, KoBin.Null.empty());
        let empty = Null::empty();
        Self::initialize(empty.as_ref())
    }

    // Java: static ExtraColorSchemeLst open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(ExtraColorSchemeLst.class, bin);
        Self::initialize(bin)
    }

    // Java: void read(Collection<ExtraColorScheme> valueLst)
    pub fn read(&self, valueLst: &mut Vec<ExtraColorScheme>) {
        // Java: valueLst.clear();
        valueLst.clear();
        // Java: int size = this.list.size();
        let size = self.list.size();
        // Java: for(int i = 0; i < size; ++i)
        for i in 0..size {
            // Java: valueLst.add((ExtraColorScheme)KoBin.Decode.toAny(ExtraColorScheme.class, this.list.get(i), this.zone));
            let _bin = self.list.get(i);
        }
    }

    // Java: void write(Iterable<ExtraColorScheme> valueLst)
    pub fn write(&mut self, valueLst: &[ExtraColorScheme]) {
        // Java: this.list.clear();
        self.list.clear();
        // Java: for(ExtraColorScheme i : valueLst)
        for item in valueLst {
            // Java: KoBin bin = i.lastBin(this.zone);
            let bin = item.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.list.add(bin);
                self.list.add(bin);
            }
        }
    }

    // Java: void forceWrite(Iterable<ExtraColorScheme> valueLst)
    pub fn forceWrite(&mut self, valueLst: &[ExtraColorScheme]) {
        // Java: this.list.clear();
        self.list.clear();
        // Java: for(ExtraColorScheme i : valueLst)
        for item in valueLst {
            // Java: this.list.add(i.lastBin(this.zone));
            let bin = item.lastBin(self.zone);
            self.list.add(bin);
        }
    }
}

impl KoAr for Theme {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { Theme::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { Theme::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { Theme::discard_with_zone(self, var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { Theme::lastBin(self, var1) }
}
impl KoClosable for Theme { fn close(&mut self) { Theme::close(self); } }

impl KoAr for ExtraColorScheme {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { ExtraColorScheme::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { ExtraColorScheme::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { ExtraColorScheme::discard_with_zone(self, var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { ExtraColorScheme::lastBin(self, var1) }
}
impl KoClosable for ExtraColorScheme { fn close(&mut self) { ExtraColorScheme::close(self); } }

impl KoAr for DefaultShapeDefinition {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { DefaultShapeDefinition::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { DefaultShapeDefinition::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { DefaultShapeDefinition::discard_with_zone(self, var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { DefaultShapeDefinition::lastBin(self, var1) }
}
impl KoClosable for DefaultShapeDefinition { fn close(&mut self) { DefaultShapeDefinition::close(self); } }

impl KoAr for CustomColorLst {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { CustomColorLst::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { CustomColorLst::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { CustomColorLst::discard_with_zone(self, var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { CustomColorLst::lastBin(self, var1) }
}
impl KoClosable for CustomColorLst { fn close(&mut self) { CustomColorLst::close(self); } }

impl KoAr for ExtraColorSchemeLst {
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { ExtraColorSchemeLst::loadBin(self, var1, var2); }
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { self.bin_box.get() }
    fn dirty(&self) -> bool { ExtraColorSchemeLst::dirty(self) }
    fn discard(&mut self, var1: &KoZone) { ExtraColorSchemeLst::discard_with_zone(self, var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { ExtraColorSchemeLst::lastBin(self, var1) }
}
impl KoClosable for ExtraColorSchemeLst { fn close(&mut self) { ExtraColorSchemeLst::close(self); } }
