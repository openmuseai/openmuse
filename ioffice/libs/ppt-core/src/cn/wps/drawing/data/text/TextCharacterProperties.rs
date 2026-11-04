#![allow(dead_code, non_snake_case, unused_variables, unused_imports, unreachable_code, unused_mut)]

// Java: import cn.wps.drawing.data.color.Color
use crate::cn::wps::drawing::data::color::Color::Color;
// Java: import cn.wps.drawing.data.effect.EffectProperties
use crate::cn::wps::drawing::data::effect::EffectProperties::EffectProperties;
// Java: import cn.wps.drawing.data.fill.Fill
// use crate::cn::wps::drawing::data::fill::Fill; // not yet converted
// Java: import cn.wps.drawing.data.font.TextFont
use crate::cn::wps::drawing::data::font::TextFont::TextFont;
// Java: import cn.wps.drawing.data.hyperlink.Hyperlink
use crate::cn::wps::drawing::data::hyperlink::Hyperlink::Hyperlink;
// Java: import cn.wps.drawing.data.outline.Outline
use crate::cn::wps::drawing::data::outline::Outline::Outline;
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
// Java: import cn.wps.kso.kproto.internal.KpBool
use crate::cn::wps::kso::kproto::internal::KpBool::KpBool;
// Java: import cn.wps.kso.kproto.internal.KpMap
use crate::cn::wps::kso::kproto::internal::KpMap::KpMap;

// Java: public class TextCharacterProperties implements KoAr, KoClosable
pub struct TextCharacterProperties {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: Outline outline
    pub(crate) outline: Option<Box<Outline>>,
    // Java field: Fill fill
    pub(crate) fill: Option<Box<dyn KoAr>>,
    // Java field: EffectProperties effect
    pub(crate) effect: Option<Box<EffectProperties>>,
    // Java field: Color highlight
    pub(crate) highlight: Option<Box<Color>>,
    // Java field: Outline underlineStroke
    pub(crate) underlineStroke: Option<Box<Outline>>,
    // Java field: Fill underlineFill
    pub(crate) underlineFill: Option<Box<dyn KoAr>>,
    // Java field: TextFont latinFont
    pub(crate) latinFont: Option<Box<TextFont>>,
    // Java field: TextFont eastAsianFont
    pub(crate) eastAsianFont: Option<Box<TextFont>>,
    // Java field: TextFont complexScriptFont
    pub(crate) complexScriptFont: Option<Box<TextFont>>,
    // Java field: TextFont symbolFont
    pub(crate) symbolFont: Option<Box<TextFont>>,
    // Java field: Hyperlink hlinkClick
    pub(crate) hlinkClick: Option<Box<Hyperlink>>,
    // Java field: Hyperlink hlinkMouseOver
    pub(crate) hlinkMouseOver: Option<Box<Hyperlink>>,
    // Java field: TextCharOptions options
    pub(crate) options: Option<Box<TextCharOptions>>,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpMap map
    pub(crate) map: KpMap,
    // Java field: TextCharacterProperties parent
    pub(crate) parent: Option<Box<TextCharacterProperties>>,
}

// Java: private static final int $outline = 1
const OUTLINE: i32 = 1;
// Java: private static final int $fill = 2
const FILL: i32 = 2;
// Java: private static final int $effect = 3
const EFFECT: i32 = 3;
// Java: private static final int $highlight = 4
const HIGHLIGHT: i32 = 4;
// Java: private static final int $underlineFollowText = 5
const UNDERLINE_FOLLOW_TEXT: i32 = 5;
// Java: private static final int $underlineStroke = 6
const UNDERLINE_STROKE: i32 = 6;
// Java: private static final int $underlineFillFollowText = 7
const UNDERLINE_FILL_FOLLOW_TEXT: i32 = 7;
// Java: private static final int $underlineFill = 8
const UNDERLINE_FILL: i32 = 8;
// Java: private static final int $latinFont = 9
const LATIN_FONT: i32 = 9;
// Java: private static final int $eastAsianFont = 10
const EAST_ASIAN_FONT: i32 = 10;
// Java: private static final int $complexScriptFont = 11
const COMPLEX_SCRIPT_FONT: i32 = 11;
// Java: private static final int $symbolFont = 12
const SYMBOL_FONT: i32 = 12;
// Java: private static final int $hlinkClick = 13
const HLINK_CLICK: i32 = 13;
// Java: private static final int $hlinkMouseOver = 14
const HLINK_MOUSE_OVER: i32 = 14;
// Java: private static final int $lang = 15
const LANG: i32 = 15;
// Java: private static final int $altLang = 16
const ALT_LANG: i32 = 16;
// Java: private static final int $fontSize = 17
const FONT_SIZE: i32 = 17;
// Java: private static final int $underline = 18
const UNDERLINE: i32 = 18;
// Java: private static final int $strike = 19
const STRIKE: i32 = 19;
// Java: private static final int $kern = 20
const KERN: i32 = 20;
// Java: private static final int $cap = 21
const CAP: i32 = 21;
// Java: private static final int $spacing = 22
const SPACING: i32 = 22;
// Java: private static final int $smtId = 23
const SMT_ID: i32 = 23;
// Java: private static final int $baseline = 24
const BASELINE: i32 = 24;
// Java: private static final int $bookmarkLinkTarget = 25
const BOOKMARK_LINK_TARGET: i32 = 25;
// Java: private static final int $options = 26
const OPTIONS: i32 = 26;

// Java: public class TextCharOptions
pub struct TextCharOptions {
    // Java field: KoZone zone
    pub(crate) zone: &'static KoZone,
    // Java field: KpBinBox box
    pub(crate) bin_box: KpBinBox,
    // Java field: KpBool map
    pub(crate) map: KpBool,
    // Java field: TextCharOptions parent
    pub(crate) parent: Option<Box<TextCharOptions>>,
}

// Java: private static final int $bold = 1
const BOLD: i32 = 1;
// Java: private static final int $dirty = 2
const DIRTY: i32 = 2;
// Java: private static final int $spellingErr = 3
const SPELLING_ERR: i32 = 3;
// Java: private static final int $italics = 4
const ITALICS: i32 = 4;
// Java: private static final int $kumimoji = 5
const KUMIMOJI: i32 = 5;
// Java: private static final int $noProof = 6
const NO_PROOF: i32 = 6;
// Java: private static final int $normalizeH = 7
const NORMALIZE_H: i32 = 7;
// Java: private static final int $smtClean = 8
const SMT_CLEAN: i32 = 8;
// Java: private static final int $rightToLeft = 9
const RIGHT_TO_LEFT: i32 = 9;

// Java: public class TextCharacterProperties implements KoAr, KoClosable
impl TextCharacterProperties {
    // Java: static protected static <Value1 extends TextCharacterProperties> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = Self::new();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static TextCharacterProperties create()
    pub fn create() -> Self {
        // Java: return initialize(TextCharacterProperties.class, KoBin.Null.empty());
        Self::initialize(Null::empty().as_ref())
    }

    // Java: static TextCharacterProperties open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(TextCharacterProperties.class, bin);
        Self::initialize(bin)
    }

    // Java: TextCharacterProperties()
    pub fn new() -> Self {
        Self {
            zone: KoZone::defaultZone(),
            outline: None,
            fill: None,
            effect: None,
            highlight: None,
            underlineStroke: None,
            underlineFill: None,
            latinFont: None,
            eastAsianFont: None,
            complexScriptFont: None,
            symbolFont: None,
            hlinkClick: None,
            hlinkMouseOver: None,
            options: None,
            // Java: this.box = new KpBinBox();
            bin_box: KpBinBox::new(),
            // Java: this.map = new KpMap();
            map: KpMap::new(),
            parent: None,
        }
    }

    // Java: TextCharacterProperties(KoBin bin)
    pub fn new_from(bin: &dyn KoBinTrait) -> Self {
        Self {
            zone: KoZone::defaultZone(),
            outline: None,
            fill: None,
            effect: None,
            highlight: None,
            underlineStroke: None,
            underlineFill: None,
            latinFont: None,
            eastAsianFont: None,
            complexScriptFont: None,
            symbolFont: None,
            hlinkClick: None,
            hlinkMouseOver: None,
            options: None,
            // Java: this.box = new KpBinBox(bin);
            bin_box: KpBinBox::KpBinBox_from(Some(bin)),
            // Java: this.map = new KpMap(bin);
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
        let bin_ref = self.bin_box.get();
        self.map.load(bin_ref.as_ref());
    }

    // Java: void load(KoBin bin)
    pub fn load(&mut self, bin: &dyn KoBinTrait) {
        // Java: this.loadBin(bin, this.zone);
        let zone = self.zone;
        self.loadBin(bin, zone);
    }

    // Java: void load(TextCharacterProperties value)
    pub fn load_from(&mut self, value: Option<&TextCharacterProperties>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let zone = self.zone;
            let bin = value.lastBin(zone);
            self.loadBin(bin.as_ref(), zone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: if (this.outline != null && this.outline.dirty())
        if self.outline.as_ref().map_or(false, |s| s.dirty())
        {
            // Java: this.map.put(1, this.outline.saveBin(zone));
            let bin = self.outline.as_mut().unwrap().saveBin(zone);
            self.map.put(1, bin);
        }
        // Java: if (this.fill != null && this.fill.dirty())
        if self.fill.as_ref().map_or(false, |s| s.dirty())
        {
            // Java: this.map.put(2, this.fill.saveBin(zone));
            let bin = self.fill.as_mut().unwrap().saveBin(zone);
            self.map.put(2, bin);
        }
        // Java: if (this.effect != null && this.effect.dirty())
        if self.effect.as_ref().map_or(false, |s| s.dirty())
        {
            // Java: this.map.put(3, this.effect.saveBin(zone));
            let bin = self.effect.as_mut().unwrap().saveBin(zone);
            self.map.put(3, bin);
        }
        // Java: if (this.highlight != null && this.highlight.dirty())
        if self.highlight.as_ref().map_or(false, |s| s.dirty())
        {
            // Java: this.map.put(4, this.highlight.saveBin(zone));
            let bin = self.highlight.as_mut().unwrap().saveBin(zone);
            self.map.put(4, bin);
        }
        // Java: if (this.underlineStroke != null && this.underlineStroke.dirty())
        if self.underlineStroke.as_ref().map_or(false, |s| s.dirty())
        {
            // Java: this.map.put(6, this.underlineStroke.saveBin(zone));
            let bin = self.underlineStroke.as_mut().unwrap().saveBin(zone);
            self.map.put(6, bin);
        }
        // Java: if (this.underlineFill != null && this.underlineFill.dirty())
        if self.underlineFill.as_ref().map_or(false, |s| s.dirty())
        {
            // Java: this.map.put(8, this.underlineFill.saveBin(zone));
            let bin = self.underlineFill.as_mut().unwrap().saveBin(zone);
            self.map.put(8, bin);
        }
        // Java: if (this.latinFont != null && this.latinFont.dirty())
        if self.latinFont.as_ref().map_or(false, |s| s.dirty())
        {
            // Java: this.map.put(9, this.latinFont.saveBin(zone));
            let bin = self.latinFont.as_mut().unwrap().saveBin(zone);
            self.map.put(9, bin);
        }
        // Java: if (this.eastAsianFont != null && this.eastAsianFont.dirty())
        if self.eastAsianFont.as_ref().map_or(false, |s| s.dirty())
        {
            // Java: this.map.put(10, this.eastAsianFont.saveBin(zone));
            let bin = self.eastAsianFont.as_mut().unwrap().saveBin(zone);
            self.map.put(10, bin);
        }
        // Java: if (this.complexScriptFont != null && this.complexScriptFont.dirty())
        if self.complexScriptFont.as_ref().map_or(false, |s| s.dirty())
        {
            // Java: this.map.put(11, this.complexScriptFont.saveBin(zone));
            let bin = self.complexScriptFont.as_mut().unwrap().saveBin(zone);
            self.map.put(11, bin);
        }
        // Java: if (this.symbolFont != null && this.symbolFont.dirty())
        if self.symbolFont.as_ref().map_or(false, |s| s.dirty())
        {
            // Java: this.map.put(12, this.symbolFont.saveBin(zone));
            let bin = self.symbolFont.as_mut().unwrap().saveBin(zone);
            self.map.put(12, bin);
        }
        // Java: if (this.hlinkClick != null && this.hlinkClick.dirty())
        if self.hlinkClick.as_ref().map_or(false, |s| s.dirty())
        {
            // Java: this.map.put(13, this.hlinkClick.saveBin(zone));
            let bin = self.hlinkClick.as_mut().unwrap().saveBin(zone);
            self.map.put(13, bin);
        }
        // Java: if (this.hlinkMouseOver != null && this.hlinkMouseOver.dirty())
        if self.hlinkMouseOver.as_ref().map_or(false, |s| s.dirty())
        {
            // Java: this.map.put(14, this.hlinkMouseOver.saveBin(zone));
            let bin = self.hlinkMouseOver.as_mut().unwrap().saveBin(zone);
            self.map.put(14, bin);
        }
        // Java: if (this.options != null && this.options.dirty())
        if self.options.as_ref().map_or(false, |s| s.dirty())
        {
            // Java: this.map.put(26, this.options.saveBin(zone));
            let bin = self.options.as_mut().unwrap().saveBin(zone);
            self.map.put(26, bin);
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

    // Java: boolean equalsLax(TextCharacterProperties o)
    pub fn equalsLax(&self, o: &TextCharacterProperties) -> bool {
        // Java: return this.equals(o, false);
        self.equals_inner(o, false)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TextCharacterProperties) -> bool {
        // Java: return this.equals(o, true);
        self.equals_inner(o, true)
    }

    // Java: boolean equals(Object o, boolean isStrict)
    fn equals_inner(&self, o: &TextCharacterProperties, isStrict: bool) -> bool {
        // Java: if (null != o && o instanceof TextCharacterProperties)
        // Java: TextCharacterProperties dst = (TextCharacterProperties)o;
        let dst = o;
        // Java: if (this == dst)
        if std::ptr::eq(self, dst) {
            // Java: return true;
            return true;
        // Java: else if ((this.hasOutline() || !dst.hasOutline()) && (!this.hasOutline() || dst.hasOutline()))
        } else if (self.hasOutline() || !dst.hasOutline()) && (!self.hasOutline() || dst.hasOutline()) {
            // Java: if (this.hasOutline() && dst.hasOutline())
            if self.hasOutline() && dst.hasOutline() {
                // Java: if (isStrict && !this.outline().equals(dst.outline()))
                if isStrict && !self.outline().equals(&dst.outline()) {
                    // Java: return false;
                    return false;
                }
                // Java: if (!isStrict && !this.outline().equalsLax(dst.outline()))
                if !isStrict && !self.outline().equalsLax(&dst.outline()) {
                    // Java: return false;
                    return false;
                }
            }
            // Java: if ((this.hasFill() || !dst.hasFill()) && (!this.hasFill() || dst.hasFill()))
            if (self.hasFill() || !dst.hasFill()) && (!self.hasFill() || dst.hasFill()) {
                // Java: if (this.hasFill() && dst.hasFill())
                if self.hasFill() && dst.hasFill() {
                    // Java: if (isStrict && !this.fill().equals(dst.fill()))
                    if isStrict {
                        // Java: return false;
                        return false;
                    }
                    // Java: if (!isStrict && !this.fill().equalsLax(dst.fill()))
                    if !isStrict {
                        // Java: return false;
                        return false;
                    }
                }
                // Java: if ((this.hasEffect() || !dst.hasEffect()) && (!this.hasEffect() || dst.hasEffect()))
                if (self.hasEffect() || !dst.hasEffect()) && (!self.hasEffect() || dst.hasEffect()) {
                    // Java: if (this.hasEffect() && dst.hasEffect())
                    if self.hasEffect() && dst.hasEffect() {
                        // Java: if (isStrict && !this.effect().equals(dst.effect()))
                        if isStrict && !self.effect().equals(&dst.effect()) {
                            // Java: return false;
                            return false;
                        }
                        // Java: if (!isStrict && !this.effect().equalsLax(dst.effect()))
                        if !isStrict && !self.effect().equalsLax(&dst.effect()) {
                            // Java: return false;
                            return false;
                        }
                    }
                    // Java: if ((this.hasHighlight() || !dst.hasHighlight()) && (!this.hasHighlight() || dst.hasHighlight()))
                    if (self.hasHighlight() || !dst.hasHighlight()) && (!self.hasHighlight() || dst.hasHighlight()) {
                        // Java: if (this.hasHighlight() && dst.hasHighlight())
                        if self.hasHighlight() && dst.hasHighlight() {
                            // Java: if (isStrict && !this.highlight().equals(dst.highlight()))
                            if isStrict && !self.highlight().equals(&dst.highlight()) {
                                // Java: return false;
                                return false;
                            }
                            // Java: if (!isStrict && !this.highlight().equalsLax(dst.highlight()))
                            if !isStrict && !self.highlight().equalsLax(&dst.highlight()) {
                                // Java: return false;
                                return false;
                            }
                        }
                        // Java: if ((this.hasUnderlineFollowText() || !dst.hasUnderlineFollowText()) && (!this.hasUnderlineFollowText() || dst.hasUnderlineFollowText()))
                        if (self.hasUnderlineFollowText() || !dst.hasUnderlineFollowText()) && (!self.hasUnderlineFollowText() || dst.hasUnderlineFollowText()) {
                            // Java: if (this.hasUnderlineFollowText() && dst.hasUnderlineFollowText() && this.isUnderlineFollowText() != dst.isUnderlineFollowText())
                            if self.hasUnderlineFollowText() && dst.hasUnderlineFollowText() && self.isUnderlineFollowText() != dst.isUnderlineFollowText() {
                                // Java: return false;
                                return false;
                            // Java: else if ((this.hasUnderlineStroke() || !dst.hasUnderlineStroke()) && (!this.hasUnderlineStroke() || dst.hasUnderlineStroke()))
                            } else if (self.hasUnderlineStroke() || !dst.hasUnderlineStroke()) && (!self.hasUnderlineStroke() || dst.hasUnderlineStroke()) {
                                // Java: if (this.hasUnderlineStroke() && dst.hasUnderlineStroke())
                                if self.hasUnderlineStroke() && dst.hasUnderlineStroke() {
                                    // Java: if (isStrict && !this.underlineStroke().equals(dst.underlineStroke()))
                                    if isStrict && !self.underlineStroke().equals(&dst.underlineStroke()) {
                                        // Java: return false;
                                        return false;
                                    }
                                    // Java: if (!isStrict && !this.underlineStroke().equalsLax(dst.underlineStroke()))
                                    if !isStrict && !self.underlineStroke().equalsLax(&dst.underlineStroke()) {
                                        // Java: return false;
                                        return false;
                                    }
                                }
                                // Java: if ((this.hasUnderlineFillFollowText() || !dst.hasUnderlineFillFollowText()) && (!this.hasUnderlineFillFollowText() || dst.hasUnderlineFillFollowText()))
                                if (self.hasUnderlineFillFollowText() || !dst.hasUnderlineFillFollowText()) && (!self.hasUnderlineFillFollowText() || dst.hasUnderlineFillFollowText()) {
                                    // Java: if (this.hasUnderlineFillFollowText() && dst.hasUnderlineFillFollowText() && this.isUnderlineFillFollowText() != dst.isUnderlineFillFollowText())
                                    if self.hasUnderlineFillFollowText() && dst.hasUnderlineFillFollowText() && self.isUnderlineFillFollowText() != dst.isUnderlineFillFollowText() {
                                        // Java: return false;
                                        return false;
                                    // Java: else if ((this.hasUnderlineFill() || !dst.hasUnderlineFill()) && (!this.hasUnderlineFill() || dst.hasUnderlineFill()))
                                    } else if (self.hasUnderlineFill() || !dst.hasUnderlineFill()) && (!self.hasUnderlineFill() || dst.hasUnderlineFill()) {
                                        // Java: if (this.hasUnderlineFill() && dst.hasUnderlineFill())
                                        if self.hasUnderlineFill() && dst.hasUnderlineFill() {
                                            // Java: if (isStrict && !this.underlineFill().equals(dst.underlineFill()))
                                            if isStrict {
                                                // Java: return false;
                                                return false;
                                            }
                                            // Java: if (!isStrict && !this.underlineFill().equalsLax(dst.underlineFill()))
                                            if !isStrict {
                                                // Java: return false;
                                                return false;
                                            }
                                        }
                                        // Java: if ((this.hasLatinFont() || !dst.hasLatinFont()) && (!this.hasLatinFont() || dst.hasLatinFont()))
                                        if (self.hasLatinFont() || !dst.hasLatinFont()) && (!self.hasLatinFont() || dst.hasLatinFont()) {
                                            // Java: if (this.hasLatinFont() && dst.hasLatinFont())
                                            if self.hasLatinFont() && dst.hasLatinFont() {
                                                // Java: if (isStrict && !this.latinFont().equals(dst.latinFont()))
                                                if isStrict && !self.latinFont().equals(&dst.latinFont()) {
                                                    // Java: return false;
                                                    return false;
                                                }
                                                // Java: if (!isStrict && !this.latinFont().equalsLax(dst.latinFont()))
                                                if !isStrict && !self.latinFont().equalsLax(&dst.latinFont()) {
                                                    // Java: return false;
                                                    return false;
                                                }
                                            }
                                            // Java: if ((this.hasEastAsianFont() || !dst.hasEastAsianFont()) && (!this.hasEastAsianFont() || dst.hasEastAsianFont()))
                                            if (self.hasEastAsianFont() || !dst.hasEastAsianFont()) && (!self.hasEastAsianFont() || dst.hasEastAsianFont()) {
                                                // Java: if (this.hasEastAsianFont() && dst.hasEastAsianFont())
                                                if self.hasEastAsianFont() && dst.hasEastAsianFont() {
                                                    // Java: if (isStrict && !this.eastAsianFont().equals(dst.eastAsianFont()))
                                                    if isStrict && !self.eastAsianFont().equals(&dst.eastAsianFont()) {
                                                        // Java: return false;
                                                        return false;
                                                    }
                                                    // Java: if (!isStrict && !this.eastAsianFont().equalsLax(dst.eastAsianFont()))
                                                    if !isStrict && !self.eastAsianFont().equalsLax(&dst.eastAsianFont()) {
                                                        // Java: return false;
                                                        return false;
                                                    }
                                                }
                                                // Java: if ((this.hasComplexScriptFont() || !dst.hasComplexScriptFont()) && (!this.hasComplexScriptFont() || dst.hasComplexScriptFont()))
                                                if (self.hasComplexScriptFont() || !dst.hasComplexScriptFont()) && (!self.hasComplexScriptFont() || dst.hasComplexScriptFont()) {
                                                    // Java: if (this.hasComplexScriptFont() && dst.hasComplexScriptFont())
                                                    if self.hasComplexScriptFont() && dst.hasComplexScriptFont() {
                                                        // Java: if (isStrict && !this.complexScriptFont().equals(dst.complexScriptFont()))
                                                        if isStrict && !self.complexScriptFont().equals(&dst.complexScriptFont()) {
                                                            // Java: return false;
                                                            return false;
                                                        }
                                                        // Java: if (!isStrict && !this.complexScriptFont().equalsLax(dst.complexScriptFont()))
                                                        if !isStrict && !self.complexScriptFont().equalsLax(&dst.complexScriptFont()) {
                                                            // Java: return false;
                                                            return false;
                                                        }
                                                    }
                                                    // Java: if ((this.hasSymbolFont() || !dst.hasSymbolFont()) && (!this.hasSymbolFont() || dst.hasSymbolFont()))
                                                    if (self.hasSymbolFont() || !dst.hasSymbolFont()) && (!self.hasSymbolFont() || dst.hasSymbolFont()) {
                                                        // Java: if (this.hasSymbolFont() && dst.hasSymbolFont())
                                                        if self.hasSymbolFont() && dst.hasSymbolFont() {
                                                            // Java: if (isStrict && !this.symbolFont().equals(dst.symbolFont()))
                                                            if isStrict && !self.symbolFont().equals(&dst.symbolFont()) {
                                                                // Java: return false;
                                                                return false;
                                                            }
                                                            // Java: if (!isStrict && !this.symbolFont().equalsLax(dst.symbolFont()))
                                                            if !isStrict && !self.symbolFont().equalsLax(&dst.symbolFont()) {
                                                                // Java: return false;
                                                                return false;
                                                            }
                                                        }
                                                        // Java: if ((this.hasHlinkClick() || !dst.hasHlinkClick()) && (!this.hasHlinkClick() || dst.hasHlinkClick()))
                                                        if (self.hasHlinkClick() || !dst.hasHlinkClick()) && (!self.hasHlinkClick() || dst.hasHlinkClick()) {
                                                            // Java: if (this.hasHlinkClick() && dst.hasHlinkClick())
                                                            if self.hasHlinkClick() && dst.hasHlinkClick() {
                                                                // Java: if (isStrict && !this.hlinkClick().equals(dst.hlinkClick()))
                                                                if isStrict && !self.hlinkClick().equals(&dst.hlinkClick()) {
                                                                    // Java: return false;
                                                                    return false;
                                                                }
                                                                // Java: if (!isStrict && !this.hlinkClick().equalsLax(dst.hlinkClick()))
                                                                if !isStrict && !self.hlinkClick().equalsLax(&dst.hlinkClick()) {
                                                                    // Java: return false;
                                                                    return false;
                                                                }
                                                            }
                                                            // Java: if ((this.hasHlinkMouseOver() || !dst.hasHlinkMouseOver()) && (!this.hasHlinkMouseOver() || dst.hasHlinkMouseOver()))
                                                            if (self.hasHlinkMouseOver() || !dst.hasHlinkMouseOver()) && (!self.hasHlinkMouseOver() || dst.hasHlinkMouseOver()) {
                                                                // Java: if (this.hasHlinkMouseOver() && dst.hasHlinkMouseOver())
                                                                if self.hasHlinkMouseOver() && dst.hasHlinkMouseOver() {
                                                                    // Java: if (isStrict && !this.hlinkMouseOver().equals(dst.hlinkMouseOver()))
                                                                    if isStrict && !self.hlinkMouseOver().equals(&dst.hlinkMouseOver()) {
                                                                        // Java: return false;
                                                                        return false;
                                                                    }
                                                                    // Java: if (!isStrict && !this.hlinkMouseOver().equalsLax(dst.hlinkMouseOver()))
                                                                    if !isStrict && !self.hlinkMouseOver().equalsLax(&dst.hlinkMouseOver()) {
                                                                        // Java: return false;
                                                                        return false;
                                                                    }
                                                                }
                                                                // Java: if ((this.hasLang() || !dst.hasLang()) && (!this.hasLang() || dst.hasLang()))
                                                                if (self.hasLang() || !dst.hasLang()) && (!self.hasLang() || dst.hasLang()) {
                                                                    // Java: if (this.hasLang() && dst.hasLang() && !this.lang().equals(dst.lang()))
                                                                    if self.hasLang() && dst.hasLang() && self.lang() != dst.lang() {
                                                                        // Java: return false;
                                                                        return false;
                                                                    // Java: else if ((this.hasAltLang() || !dst.hasAltLang()) && (!this.hasAltLang() || dst.hasAltLang()))
                                                                    } else if (self.hasAltLang() || !dst.hasAltLang()) && (!self.hasAltLang() || dst.hasAltLang()) {
                                                                        // Java: if (this.hasAltLang() && dst.hasAltLang() && !this.altLang().equals(dst.altLang()))
                                                                        if self.hasAltLang() && dst.hasAltLang() && self.altLang() != dst.altLang() {
                                                                            // Java: return false;
                                                                            return false;
                                                                        // Java: else if ((this.hasFontSize() || !dst.hasFontSize()) && (!this.hasFontSize() || dst.hasFontSize()))
                                                                        } else if (self.hasFontSize() || !dst.hasFontSize()) && (!self.hasFontSize() || dst.hasFontSize()) {
                                                                            // Java: if (this.hasFontSize() && dst.hasFontSize() && this.fontSize() != dst.fontSize())
                                                                            if self.hasFontSize() && dst.hasFontSize() && self.fontSize() != dst.fontSize() {
                                                                                // Java: return false;
                                                                                return false;
                                                                            // Java: else if ((this.hasUnderline() || !dst.hasUnderline()) && (!this.hasUnderline() || dst.hasUnderline()))
                                                                            } else if (self.hasUnderline() || !dst.hasUnderline()) && (!self.hasUnderline() || dst.hasUnderline()) {
                                                                                // Java: if (this.hasUnderline() && dst.hasUnderline() && this.underline() != dst.underline())
                                                                                if self.hasUnderline() && dst.hasUnderline() && self.underline() != dst.underline() {
                                                                                    // Java: return false;
                                                                                    return false;
                                                                                // Java: else if ((this.hasStrike() || !dst.hasStrike()) && (!this.hasStrike() || dst.hasStrike()))
                                                                                } else if (self.hasStrike() || !dst.hasStrike()) && (!self.hasStrike() || dst.hasStrike()) {
                                                                                    // Java: if (this.hasStrike() && dst.hasStrike() && this.strike() != dst.strike())
                                                                                    if self.hasStrike() && dst.hasStrike() && self.strike() != dst.strike() {
                                                                                        // Java: return false;
                                                                                        return false;
                                                                                    // Java: else if ((this.hasKern() || !dst.hasKern()) && (!this.hasKern() || dst.hasKern()))
                                                                                    } else if (self.hasKern() || !dst.hasKern()) && (!self.hasKern() || dst.hasKern()) {
                                                                                        // Java: if (this.hasKern() && dst.hasKern() && this.kern() != dst.kern())
                                                                                        if self.hasKern() && dst.hasKern() && self.kern() != dst.kern() {
                                                                                            // Java: return false;
                                                                                            return false;
                                                                                        // Java: else if ((this.hasCap() || !dst.hasCap()) && (!this.hasCap() || dst.hasCap()))
                                                                                        } else if (self.hasCap() || !dst.hasCap()) && (!self.hasCap() || dst.hasCap()) {
                                                                                            // Java: if (this.hasCap() && dst.hasCap() && this.cap() != dst.cap())
                                                                                            if self.hasCap() && dst.hasCap() && self.cap() != dst.cap() {
                                                                                                // Java: return false;
                                                                                                return false;
                                                                                            // Java: else if ((this.hasSpacing() || !dst.hasSpacing()) && (!this.hasSpacing() || dst.hasSpacing()))
                                                                                            } else if (self.hasSpacing() || !dst.hasSpacing()) && (!self.hasSpacing() || dst.hasSpacing()) {
                                                                                                // Java: if (this.hasSpacing() && dst.hasSpacing() && this.spacing() != dst.spacing())
                                                                                                if self.hasSpacing() && dst.hasSpacing() && self.spacing() != dst.spacing() {
                                                                                                    // Java: return false;
                                                                                                    return false;
                                                                                                // Java: else if ((this.hasSmtId() || !dst.hasSmtId()) && (!this.hasSmtId() || dst.hasSmtId()))
                                                                                                } else if (self.hasSmtId() || !dst.hasSmtId()) && (!self.hasSmtId() || dst.hasSmtId()) {
                                                                                                    // Java: if (this.hasSmtId() && dst.hasSmtId() && this.smtId() != dst.smtId())
                                                                                                    if self.hasSmtId() && dst.hasSmtId() && self.smtId() != dst.smtId() {
                                                                                                        // Java: return false;
                                                                                                        return false;
                                                                                                    // Java: else if ((this.hasBaseline() || !dst.hasBaseline()) && (!this.hasBaseline() || dst.hasBaseline()))
                                                                                                    } else if (self.hasBaseline() || !dst.hasBaseline()) && (!self.hasBaseline() || dst.hasBaseline()) {
                                                                                                        // Java: if (this.hasBaseline() && dst.hasBaseline() && this.baseline() != dst.baseline())
                                                                                                        if self.hasBaseline() && dst.hasBaseline() && self.baseline() != dst.baseline() {
                                                                                                            // Java: return false;
                                                                                                            return false;
                                                                                                        // Java: else if ((this.hasBookmarkLinkTarget() || !dst.hasBookmarkLinkTarget()) && (!this.hasBookmarkLinkTarget() || dst.hasBookmarkLinkTarget()))
                                                                                                        } else if (self.hasBookmarkLinkTarget() || !dst.hasBookmarkLinkTarget()) && (!self.hasBookmarkLinkTarget() || dst.hasBookmarkLinkTarget()) {
                                                                                                            // Java: if (this.hasBookmarkLinkTarget() && dst.hasBookmarkLinkTarget() && !this.bookmarkLinkTarget().equals(dst.bookmarkLinkTarget()))
                                                                                                            if self.hasBookmarkLinkTarget() && dst.hasBookmarkLinkTarget() && self.bookmarkLinkTarget() != dst.bookmarkLinkTarget() {
                                                                                                                // Java: return false;
                                                                                                                return false;
                                                                                                            // Java: else if ((this.hasOptions() || !dst.hasOptions()) && (!this.hasOptions() || dst.hasOptions()))
                                                                                                            } else if (self.hasOptions() || !dst.hasOptions()) && (!self.hasOptions() || dst.hasOptions()) {
                                                                                                                // Java: if (this.hasOptions() && dst.hasOptions())
                                                                                                                if self.hasOptions() && dst.hasOptions() {
                                                                                                                    // Java: if (isStrict && !this.options().equals(dst.options()))
                                                                                                                    if isStrict && !self.options().equals(&dst.options()) {
                                                                                                                        // Java: return false;
                                                                                                                        return false;
                                                                                                                    }
                                                                                                                    // Java: if (!isStrict && !this.options().equalsLax(dst.options()))
                                                                                                                    if !isStrict && !self.options().equalsLax(&dst.options()) {
                                                                                                                        // Java: return false;
                                                                                                                        return false;
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
                return false;
            }
        // Java: else
        } else {
            // Java: return false;
            return false;
        }
        // Java: else
        // Java: return false;
        return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: int ret = 0;
        let mut ret: i32 = 0;
        // Java: if (this.hasOutline())
        if self.hasOutline() {
            // Java: ret += this.outline().hashCode();
            ret = ret.wrapping_add(self.outline().hashCode());
        }
        // Java: if (this.hasFill())
        if self.hasFill() {
            // Java: ret += this.fill().hashCode();
            todo!()
        }
        // Java: if (this.hasEffect())
        if self.hasEffect() {
            // Java: ret += this.effect().hashCode();
            ret = ret.wrapping_add(self.effect().hashCode());
        }
        // Java: if (this.hasHighlight())
        if self.hasHighlight() {
            // Java: ret += this.highlight().hashCode();
            ret = ret.wrapping_add(self.highlight().hashCode());
        }
        // Java: if (this.hasUnderlineFollowText())
        if self.hasUnderlineFollowText() {
            // Java: ret += Boolean.valueOf(this.isUnderlineFollowText()).hashCode();
            ret = ret.wrapping_add(if self.isUnderlineFollowText() { 1231 } else { 1237 });
        }
        // Java: if (this.hasUnderlineStroke())
        if self.hasUnderlineStroke() {
            // Java: ret += this.underlineStroke().hashCode();
            ret = ret.wrapping_add(self.underlineStroke().hashCode());
        }
        // Java: if (this.hasUnderlineFillFollowText())
        if self.hasUnderlineFillFollowText() {
            // Java: ret += Boolean.valueOf(this.isUnderlineFillFollowText()).hashCode();
            ret = ret.wrapping_add(if self.isUnderlineFillFollowText() { 1231 } else { 1237 });
        }
        // Java: if (this.hasUnderlineFill())
        if self.hasUnderlineFill() {
            // Java: ret += this.underlineFill().hashCode();
            todo!()
        }
        // Java: if (this.hasLatinFont())
        if self.hasLatinFont() {
            // Java: ret += this.latinFont().hashCode();
            ret = ret.wrapping_add(self.latinFont().hashCode());
        }
        // Java: if (this.hasEastAsianFont())
        if self.hasEastAsianFont() {
            // Java: ret += this.eastAsianFont().hashCode();
            ret = ret.wrapping_add(self.eastAsianFont().hashCode());
        }
        // Java: if (this.hasComplexScriptFont())
        if self.hasComplexScriptFont() {
            // Java: ret += this.complexScriptFont().hashCode();
            ret = ret.wrapping_add(self.complexScriptFont().hashCode());
        }
        // Java: if (this.hasSymbolFont())
        if self.hasSymbolFont() {
            // Java: ret += this.symbolFont().hashCode();
            ret = ret.wrapping_add(self.symbolFont().hashCode());
        }
        // Java: if (this.hasHlinkClick())
        if self.hasHlinkClick() {
            // Java: ret += this.hlinkClick().hashCode();
            ret = ret.wrapping_add(self.hlinkClick().hashCode());
        }
        // Java: if (this.hasHlinkMouseOver())
        if self.hasHlinkMouseOver() {
            // Java: ret += this.hlinkMouseOver().hashCode();
            ret = ret.wrapping_add(self.hlinkMouseOver().hashCode());
        }
        // Java: if (this.hasLang())
        if self.hasLang() {
            // Java: ret += String.valueOf(this.lang()).hashCode();
            ret = ret.wrapping_add(self.lang().len() as i32);
        }
        // Java: if (this.hasAltLang())
        if self.hasAltLang() {
            // Java: ret += String.valueOf(this.altLang()).hashCode();
            ret = ret.wrapping_add(self.altLang().len() as i32);
        }
        // Java: if (this.hasFontSize())
        if self.hasFontSize() {
            // Java: ret += Integer.valueOf(this.fontSize()).hashCode();
            ret = ret.wrapping_add(self.fontSize());
        }
        // Java: if (this.hasUnderline())
        if self.hasUnderline() {
            // Java: ret += Integer.valueOf(this.underline()).hashCode();
            ret = ret.wrapping_add(self.underline());
        }
        // Java: if (this.hasStrike())
        if self.hasStrike() {
            // Java: ret += Integer.valueOf(this.strike()).hashCode();
            ret = ret.wrapping_add(self.strike());
        }
        // Java: if (this.hasKern())
        if self.hasKern() {
            // Java: ret += Integer.valueOf(this.kern()).hashCode();
            ret = ret.wrapping_add(self.kern());
        }
        // Java: if (this.hasCap())
        if self.hasCap() {
            // Java: ret += Integer.valueOf(this.cap()).hashCode();
            ret = ret.wrapping_add(self.cap());
        }
        // Java: if (this.hasSpacing())
        if self.hasSpacing() {
            // Java: ret += Integer.valueOf(this.spacing()).hashCode();
            ret = ret.wrapping_add(self.spacing());
        }
        // Java: if (this.hasSmtId())
        if self.hasSmtId() {
            // Java: ret += Integer.valueOf(this.smtId()).hashCode();
            ret = ret.wrapping_add(self.smtId());
        }
        // Java: if (this.hasBaseline())
        if self.hasBaseline() {
            // Java: ret += Double.valueOf(this.baseline()).hashCode();
            ret = ret.wrapping_add(self.baseline().to_bits() as i32);
        }
        // Java: if (this.hasBookmarkLinkTarget())
        if self.hasBookmarkLinkTarget() {
            // Java: ret += String.valueOf(this.bookmarkLinkTarget()).hashCode();
            ret = ret.wrapping_add(self.bookmarkLinkTarget().len() as i32);
        }
        // Java: if (this.hasOptions())
        if self.hasOptions() {
            // Java: ret += this.options().hashCode();
            ret = ret.wrapping_add(self.options().hashCode());
        }
        // Java: return ret;
        ret
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: if (this.outline != null && this.outline.dirty())
        if self.outline.as_ref().map_or(false, |o| o.dirty()) {
            // Java: return true;
            return true;
        // Java: else if (this.fill != null && this.fill.dirty())
        } else if self.fill.as_ref().map_or(false, |f| f.dirty()) {
            // Java: return true;
            return true;
        // Java: else if (this.effect != null && this.effect.dirty())
        } else if self.effect.as_ref().map_or(false, |e| e.dirty()) {
            // Java: return true;
            return true;
        // Java: else if (this.highlight != null && this.highlight.dirty())
        } else if self.highlight.as_ref().map_or(false, |h| h.dirty()) {
            // Java: return true;
            return true;
        // Java: else if (this.underlineStroke != null && this.underlineStroke.dirty())
        } else if self.underlineStroke.as_ref().map_or(false, |u| u.dirty()) {
            // Java: return true;
            return true;
        // Java: else if (this.underlineFill != null && this.underlineFill.dirty())
        } else if self.underlineFill.as_ref().map_or(false, |u| u.dirty()) {
            // Java: return true;
            return true;
        // Java: else if (this.latinFont != null && this.latinFont.dirty())
        } else if self.latinFont.as_ref().map_or(false, |l| l.dirty()) {
            // Java: return true;
            return true;
        // Java: else if (this.eastAsianFont != null && this.eastAsianFont.dirty())
        } else if self.eastAsianFont.as_ref().map_or(false, |e| e.dirty()) {
            // Java: return true;
            return true;
        // Java: else if (this.complexScriptFont != null && this.complexScriptFont.dirty())
        } else if self.complexScriptFont.as_ref().map_or(false, |c| c.dirty()) {
            // Java: return true;
            return true;
        // Java: else if (this.symbolFont != null && this.symbolFont.dirty())
        } else if self.symbolFont.as_ref().map_or(false, |s| s.dirty()) {
            // Java: return true;
            return true;
        // Java: else if (this.hlinkClick != null && this.hlinkClick.dirty())
        } else if self.hlinkClick.as_ref().map_or(false, |h| h.dirty()) {
            // Java: return true;
            return true;
        // Java: else if (this.hlinkMouseOver != null && this.hlinkMouseOver.dirty())
        } else if self.hlinkMouseOver.as_ref().map_or(false, |h| h.dirty()) {
            // Java: return true;
            return true;
        // Java: else
        } else {
            // Java: return this.options != null && this.options.dirty() ? true : this.map.dirty();
            if let Some(ref options) = self.options {
                if options.dirty() {
                    return true;
                }
            }
            self.map.dirty()
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
    fn cleanFollow(&mut self) {
    }

    // Java: void cleanCache()
    fn cleanCache(&mut self) {
        // Java: this.outline = null;
        self.outline = None;
        // Java: this.fill = null;
        self.fill = None;
        // Java: this.effect = null;
        self.effect = None;
        // Java: this.highlight = null;
        self.highlight = None;
        // Java: this.underlineStroke = null;
        self.underlineStroke = None;
        // Java: this.underlineFill = null;
        self.underlineFill = None;
        // Java: this.latinFont = null;
        self.latinFont = None;
        // Java: this.eastAsianFont = null;
        self.eastAsianFont = None;
        // Java: this.complexScriptFont = null;
        self.complexScriptFont = None;
        // Java: this.symbolFont = null;
        self.symbolFont = None;
        // Java: this.hlinkClick = null;
        self.hlinkClick = None;
        // Java: this.hlinkMouseOver = null;
        self.hlinkMouseOver = None;
        // Java: this.options = null;
        self.options = None;
    }

    // Java: void follow(TextCharacterProperties parent)
    pub fn follow(&mut self, parent: Option<Box<TextCharacterProperties>>) {
        // Java: if (parent != null)
        if let Some(mut parent) = parent {
            // Java: parent.discard();
            parent.discard();
            // Java: this.discard();
            self.discard();
            // Java: this.parent = parent;
            self.parent = Some(parent);
        }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
        // Java: if (this.parent != null)
        if self.parent.is_some() {
            // Java: this.discard();
            self.discard();
            // Java: this.parent.discard();
            if let Some(ref mut parent) = self.parent {
                parent.discard();
            }
            // Java: this.cleanParent();
            self.cleanParent();
        }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
        // Java: return this.parent != null;
        self.parent.is_some()
    }

    // Java: TextCharacterProperties parent()
    pub fn parent(&self) -> Option<&TextCharacterProperties> {
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
    fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None;
    }

    // Java: Outline outline()
    pub fn outline(&self) -> &Outline {
        // Java: if (this.outline == null)
        // Java: if (this.map.has(1))
        // Java: KoBin bin = this.map.byKey(1);
        // Java: this.outline = (Outline)KoBin.Decode.toAny(Outline.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasOutline())
        // Java: this.outline = this.parent.outline();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.outline = (Outline)KoBin.Decode.toAny(Outline.class, bin, this.zone);
        // Java: return this.outline;
        todo!()
    }

    // Java: boolean hasOutline()
    pub fn hasOutline(&self) -> bool {
        // Java: return this.map.has(1) || this.outline != null && this.outline.dirty() || this.parent != null && this.parent.hasOutline();
        self.map.has(1) || self.outline.as_ref().map_or(false, |o| o.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasOutline())
    }

    // Java: void deleteOutline()
    pub fn deleteOutline(&mut self) {
        // Java: this.map.put(1, (KoBin)null);
        self.map.clear_key(1);
        // Java: if (this.outline != null)
        if self.outline.is_some() {
            // Java: this.outline = null;
            self.outline = None;
        }
    }

    // Java: void writeOutline(Outline value)
    pub fn writeOutline(&mut self, value: Option<&Outline>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(1, bin);
                self.map.put(1, bin);
                // Java: if (this.outline != null)
                if self.outline.is_some() {
                    // Java: this.outline = null;
                    self.outline = None;
                }
            }
        }
    }

    // Java: void forceWriteOutline(Outline value)
    pub fn forceWriteOutline(&mut self, value: Option<&Outline>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: this.map.put(1, value.lastBin(this.zone));
            self.map.put(1, value.lastBin(self.zone));
            // Java: if (this.outline != null)
            if self.outline.is_some() {
                // Java: this.outline = null;
                self.outline = None;
            }
        }
    }

    // Java: Fill fill()
    pub fn fill(&self) -> &dyn KoAr {
        // Java: if (this.fill == null)
        // Java: if (this.map.has(2))
        // Java: KoBin bin = this.map.byKey(2);
        // Java: this.fill = (Fill)KoBin.Decode.toAny(Fill.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasFill())
        // Java: this.fill = this.parent.fill();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.fill = (Fill)KoBin.Decode.toAny(Fill.class, bin, this.zone);
        // Java: return this.fill;
        todo!()
    }

    // Java: boolean hasFill()
    pub fn hasFill(&self) -> bool {
        // Java: return this.map.has(2) || this.fill != null && this.fill.dirty() || this.parent != null && this.parent.hasFill();
        self.map.has(2) || self.fill.as_ref().map_or(false, |f| f.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasFill())
    }

    // Java: void deleteFill()
    pub fn deleteFill(&mut self) {
        // Java: this.map.put(2, (KoBin)null);
        self.map.clear_key(2);
        // Java: if (this.fill != null)
        if self.fill.is_some() {
            // Java: this.fill = null;
            self.fill = None;
        }
    }

    // Java: void writeFill(Fill value)
    pub fn writeFill(&mut self, value: Option<&dyn KoAr>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(2, bin);
                self.map.put(2, bin);
                // Java: if (this.fill != null)
                if self.fill.is_some() {
                    // Java: this.fill = null;
                    self.fill = None;
                }
            }
        }
    }

    // Java: void forceWriteFill(Fill value)
    pub fn forceWriteFill(&mut self, value: Option<&dyn KoAr>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: this.map.put(2, value.lastBin(this.zone));
            self.map.put(2, value.lastBin(self.zone));
            // Java: if (this.fill != null)
            if self.fill.is_some() {
                // Java: this.fill = null;
                self.fill = None;
            }
        }
    }

    // Java: EffectProperties effect()
    pub fn effect(&self) -> &EffectProperties {
        // Java: if (this.effect == null)
        // Java: if (this.map.has(3))
        // Java: KoBin bin = this.map.byKey(3);
        // Java: this.effect = (EffectProperties)KoBin.Decode.toAny(EffectProperties.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasEffect())
        // Java: this.effect = this.parent.effect();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.effect = (EffectProperties)KoBin.Decode.toAny(EffectProperties.class, bin, this.zone);
        // Java: return this.effect;
        todo!()
    }

    // Java: boolean hasEffect()
    pub fn hasEffect(&self) -> bool {
        // Java: return this.map.has(3) || this.effect != null && this.effect.dirty() || this.parent != null && this.parent.hasEffect();
        self.map.has(3) || self.effect.as_ref().map_or(false, |e| e.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasEffect())
    }

    // Java: void deleteEffect()
    pub fn deleteEffect(&mut self) {
        // Java: this.map.put(3, (KoBin)null);
        self.map.clear_key(3);
        // Java: if (this.effect != null)
        if self.effect.is_some() {
            // Java: this.effect = null;
            self.effect = None;
        }
    }

    // Java: void writeEffect(EffectProperties value)
    pub fn writeEffect(&mut self, value: Option<&EffectProperties>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(3, bin);
                self.map.put(3, bin);
                // Java: if (this.effect != null)
                if self.effect.is_some() {
                    // Java: this.effect = null;
                    self.effect = None;
                }
            }
        }
    }

    // Java: void forceWriteEffect(EffectProperties value)
    pub fn forceWriteEffect(&mut self, value: Option<&EffectProperties>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: this.map.put(3, value.lastBin(this.zone));
            self.map.put(3, value.lastBin(self.zone));
            // Java: if (this.effect != null)
            if self.effect.is_some() {
                // Java: this.effect = null;
                self.effect = None;
            }
        }
    }

    // Java: Color highlight()
    pub fn highlight(&self) -> &Color {
        // Java: if (this.highlight == null)
        // Java: if (this.map.has(4))
        // Java: KoBin bin = this.map.byKey(4);
        // Java: this.highlight = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasHighlight())
        // Java: this.highlight = this.parent.highlight();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.highlight = (Color)KoBin.Decode.toAny(Color.class, bin, this.zone);
        // Java: return this.highlight;
        todo!()
    }

    // Java: boolean hasHighlight()
    pub fn hasHighlight(&self) -> bool {
        // Java: return this.map.has(4) || this.highlight != null && this.highlight.dirty() || this.parent != null && this.parent.hasHighlight();
        self.map.has(4) || self.highlight.as_ref().map_or(false, |h| h.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasHighlight())
    }

    // Java: void deleteHighlight()
    pub fn deleteHighlight(&mut self) {
        // Java: this.map.put(4, (KoBin)null);
        self.map.clear_key(4);
        // Java: if (this.highlight != null)
        if self.highlight.is_some() {
            // Java: this.highlight = null;
            self.highlight = None;
        }
    }

    // Java: void writeHighlight(Color value)
    pub fn writeHighlight(&mut self, value: Option<&Color>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(4, bin);
                self.map.put(4, bin);
                // Java: if (this.highlight != null)
                if self.highlight.is_some() {
                    // Java: this.highlight = null;
                    self.highlight = None;
                }
            }
        }
    }

    // Java: void forceWriteHighlight(Color value)
    pub fn forceWriteHighlight(&mut self, value: Option<&Color>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: this.map.put(4, value.lastBin(this.zone));
            self.map.put(4, value.lastBin(self.zone));
            // Java: if (this.highlight != null)
            if self.highlight.is_some() {
                // Java: this.highlight = null;
                self.highlight = None;
            }
        }
    }

    // Java: boolean isUnderlineFollowText()
    pub fn isUnderlineFollowText(&self) -> bool {
        // Java: if (!this.map.has(5))
        if !self.map.has(5) {
            // Java: return this.parent == null ? false : this.parent.isUnderlineFollowText();
            return self.parent.as_ref().map_or(false, |p| p.isUnderlineFollowText());
        // Java: else
        } else {
            // Java: KoBin bin = this.map.byKey(5);
            let bin = self.map.byKey(5);
            // Java: return KoBin.Decode.toBool(bin);
            Decode::toBool(bin.as_ref())
        }
    }

    // Java: boolean hasUnderlineFollowText()
    pub fn hasUnderlineFollowText(&self) -> bool {
        // Java: return this.map.has(5) || this.parent != null && this.parent.hasUnderlineFollowText();
        self.map.has(5) || self.parent.as_ref().map_or(false, |p| p.hasUnderlineFollowText())
    }

    // Java: void deleteUnderlineFollowText()
    pub fn deleteUnderlineFollowText(&mut self) {
        // Java: this.map.put(5, (KoBin)null);
        self.map.clear_key(5);
    }

    // Java: void putUnderlineFollowText(boolean value)
    pub fn putUnderlineFollowText(&mut self, value: bool) {
        // Java: this.map.put(5, KoBin.Encode.from(value, this.zone));
        self.map.put(5, Encode::from_bool(value, self.zone));
    }

    // Java: Outline underlineStroke()
    pub fn underlineStroke(&self) -> &Outline {
        // Java: if (this.underlineStroke == null)
        // Java: if (this.map.has(6))
        // Java: KoBin bin = this.map.byKey(6);
        // Java: this.underlineStroke = (Outline)KoBin.Decode.toAny(Outline.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasUnderlineStroke())
        // Java: this.underlineStroke = this.parent.underlineStroke();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.underlineStroke = (Outline)KoBin.Decode.toAny(Outline.class, bin, this.zone);
        // Java: return this.underlineStroke;
        todo!()
    }

    // Java: boolean hasUnderlineStroke()
    pub fn hasUnderlineStroke(&self) -> bool {
        // Java: return this.map.has(6) || this.underlineStroke != null && this.underlineStroke.dirty() || this.parent != null && this.parent.hasUnderlineStroke();
        self.map.has(6) || self.underlineStroke.as_ref().map_or(false, |u| u.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasUnderlineStroke())
    }

    // Java: void deleteUnderlineStroke()
    pub fn deleteUnderlineStroke(&mut self) {
        // Java: this.map.put(6, (KoBin)null);
        self.map.clear_key(6);
        // Java: if (this.underlineStroke != null)
        if self.underlineStroke.is_some() {
            // Java: this.underlineStroke = null;
            self.underlineStroke = None;
        }
    }

    // Java: void writeUnderlineStroke(Outline value)
    pub fn writeUnderlineStroke(&mut self, value: Option<&Outline>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(6, bin);
                self.map.put(6, bin);
                // Java: if (this.underlineStroke != null)
                if self.underlineStroke.is_some() {
                    // Java: this.underlineStroke = null;
                    self.underlineStroke = None;
                }
            }
        }
    }

    // Java: void forceWriteUnderlineStroke(Outline value)
    pub fn forceWriteUnderlineStroke(&mut self, value: Option<&Outline>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: this.map.put(6, value.lastBin(this.zone));
            self.map.put(6, value.lastBin(self.zone));
            // Java: if (this.underlineStroke != null)
            if self.underlineStroke.is_some() {
                // Java: this.underlineStroke = null;
                self.underlineStroke = None;
            }
        }
    }

    // Java: boolean isUnderlineFillFollowText()
    pub fn isUnderlineFillFollowText(&self) -> bool {
        // Java: if (!this.map.has(7))
        if !self.map.has(7) {
            // Java: return this.parent == null ? false : this.parent.isUnderlineFillFollowText();
            return self.parent.as_ref().map_or(false, |p| p.isUnderlineFillFollowText());
        // Java: else
        } else {
            // Java: KoBin bin = this.map.byKey(7);
            let bin = self.map.byKey(7);
            // Java: return KoBin.Decode.toBool(bin);
            Decode::toBool(bin.as_ref())
        }
    }

    // Java: boolean hasUnderlineFillFollowText()
    pub fn hasUnderlineFillFollowText(&self) -> bool {
        // Java: return this.map.has(7) || this.parent != null && this.parent.hasUnderlineFillFollowText();
        self.map.has(7) || self.parent.as_ref().map_or(false, |p| p.hasUnderlineFillFollowText())
    }

    // Java: void deleteUnderlineFillFollowText()
    pub fn deleteUnderlineFillFollowText(&mut self) {
        // Java: this.map.put(7, (KoBin)null);
        self.map.clear_key(7);
    }

    // Java: void putUnderlineFillFollowText(boolean value)
    pub fn putUnderlineFillFollowText(&mut self, value: bool) {
        // Java: this.map.put(7, KoBin.Encode.from(value, this.zone));
        self.map.put(7, Encode::from_bool(value, self.zone));
    }

    // Java: Fill underlineFill()
    pub fn underlineFill(&self) -> &dyn KoAr {
        // Java: if (this.underlineFill == null)
        // Java: if (this.map.has(8))
        // Java: KoBin bin = this.map.byKey(8);
        // Java: this.underlineFill = (Fill)KoBin.Decode.toAny(Fill.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasUnderlineFill())
        // Java: this.underlineFill = this.parent.underlineFill();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.underlineFill = (Fill)KoBin.Decode.toAny(Fill.class, bin, this.zone);
        // Java: return this.underlineFill;
        todo!()
    }

    // Java: boolean hasUnderlineFill()
    pub fn hasUnderlineFill(&self) -> bool {
        // Java: return this.map.has(8) || this.underlineFill != null && this.underlineFill.dirty() || this.parent != null && this.parent.hasUnderlineFill();
        self.map.has(8) || self.underlineFill.as_ref().map_or(false, |f| f.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasUnderlineFill())
    }

    // Java: void deleteUnderlineFill()
    pub fn deleteUnderlineFill(&mut self) {
        // Java: this.map.put(8, (KoBin)null);
        self.map.clear_key(8);
        // Java: if (this.underlineFill != null)
        if self.underlineFill.is_some() {
            // Java: this.underlineFill = null;
            self.underlineFill = None;
        }
    }

    // Java: void writeUnderlineFill(Fill value)
    pub fn writeUnderlineFill(&mut self, value: Option<&dyn KoAr>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(8, bin);
                self.map.put(8, bin);
                // Java: if (this.underlineFill != null)
                if self.underlineFill.is_some() {
                    // Java: this.underlineFill = null;
                    self.underlineFill = None;
                }
            }
        }
    }

    // Java: void forceWriteUnderlineFill(Fill value)
    pub fn forceWriteUnderlineFill(&mut self, value: Option<&dyn KoAr>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: this.map.put(8, value.lastBin(this.zone));
            self.map.put(8, value.lastBin(self.zone));
            // Java: if (this.underlineFill != null)
            if self.underlineFill.is_some() {
                // Java: this.underlineFill = null;
                self.underlineFill = None;
            }
        }
    }

    // Java: TextFont latinFont()
    pub fn latinFont(&self) -> &TextFont {
        // Java: if (this.latinFont == null)
        // Java: if (this.map.has(9))
        // Java: KoBin bin = this.map.byKey(9);
        // Java: this.latinFont = (TextFont)KoBin.Decode.toAny(TextFont.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasLatinFont())
        // Java: this.latinFont = this.parent.latinFont();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.latinFont = (TextFont)KoBin.Decode.toAny(TextFont.class, bin, this.zone);
        // Java: return this.latinFont;
        todo!()
    }

    // Java: boolean hasLatinFont()
    pub fn hasLatinFont(&self) -> bool {
        // Java: return this.map.has(9) || this.latinFont != null && this.latinFont.dirty() || this.parent != null && this.parent.hasLatinFont();
        self.map.has(9) || self.latinFont.as_ref().map_or(false, |f| f.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasLatinFont())
    }

    // Java: void deleteLatinFont()
    pub fn deleteLatinFont(&mut self) {
        // Java: this.map.put(9, (KoBin)null);
        self.map.clear_key(9);
        // Java: if (this.latinFont != null)
        if self.latinFont.is_some() {
            // Java: this.latinFont = null;
            self.latinFont = None;
        }
    }

    // Java: void writeLatinFont(TextFont value)
    pub fn writeLatinFont(&mut self, value: Option<&TextFont>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(9, bin);
                self.map.put(9, bin);
                // Java: if (this.latinFont != null)
                if self.latinFont.is_some() {
                    // Java: this.latinFont = null;
                    self.latinFont = None;
                }
            }
        }
    }

    // Java: void forceWriteLatinFont(TextFont value)
    pub fn forceWriteLatinFont(&mut self, value: Option<&TextFont>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: this.map.put(9, value.lastBin(this.zone));
            self.map.put(9, value.lastBin(self.zone));
            // Java: if (this.latinFont != null)
            if self.latinFont.is_some() {
                // Java: this.latinFont = null;
                self.latinFont = None;
            }
        }
    }

    // Java: TextFont eastAsianFont()
    pub fn eastAsianFont(&self) -> &TextFont {
        // Java: if (this.eastAsianFont == null)
        // Java: if (this.map.has(10))
        // Java: KoBin bin = this.map.byKey(10);
        // Java: this.eastAsianFont = (TextFont)KoBin.Decode.toAny(TextFont.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasEastAsianFont())
        // Java: this.eastAsianFont = this.parent.eastAsianFont();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.eastAsianFont = (TextFont)KoBin.Decode.toAny(TextFont.class, bin, this.zone);
        // Java: return this.eastAsianFont;
        todo!()
    }

    // Java: boolean hasEastAsianFont()
    pub fn hasEastAsianFont(&self) -> bool {
        // Java: return this.map.has(10) || this.eastAsianFont != null && this.eastAsianFont.dirty() || this.parent != null && this.parent.hasEastAsianFont();
        self.map.has(10) || self.eastAsianFont.as_ref().map_or(false, |f| f.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasEastAsianFont())
    }

    // Java: void deleteEastAsianFont()
    pub fn deleteEastAsianFont(&mut self) {
        // Java: this.map.put(10, (KoBin)null);
        self.map.clear_key(10);
        // Java: if (this.eastAsianFont != null)
        if self.eastAsianFont.is_some() {
            // Java: this.eastAsianFont = null;
            self.eastAsianFont = None;
        }
    }

    // Java: void writeEastAsianFont(TextFont value)
    pub fn writeEastAsianFont(&mut self, value: Option<&TextFont>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(10, bin);
                self.map.put(10, bin);
                // Java: if (this.eastAsianFont != null)
                if self.eastAsianFont.is_some() {
                    // Java: this.eastAsianFont = null;
                    self.eastAsianFont = None;
                }
            }
        }
    }

    // Java: void forceWriteEastAsianFont(TextFont value)
    pub fn forceWriteEastAsianFont(&mut self, value: Option<&TextFont>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: this.map.put(10, value.lastBin(this.zone));
            self.map.put(10, value.lastBin(self.zone));
            // Java: if (this.eastAsianFont != null)
            if self.eastAsianFont.is_some() {
                // Java: this.eastAsianFont = null;
                self.eastAsianFont = None;
            }
        }
    }

    // Java: TextFont complexScriptFont()
    pub fn complexScriptFont(&self) -> &TextFont {
        // Java: if (this.complexScriptFont == null)
        // Java: if (this.map.has(11))
        // Java: KoBin bin = this.map.byKey(11);
        // Java: this.complexScriptFont = (TextFont)KoBin.Decode.toAny(TextFont.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasComplexScriptFont())
        // Java: this.complexScriptFont = this.parent.complexScriptFont();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.complexScriptFont = (TextFont)KoBin.Decode.toAny(TextFont.class, bin, this.zone);
        // Java: return this.complexScriptFont;
        todo!()
    }

    // Java: boolean hasComplexScriptFont()
    pub fn hasComplexScriptFont(&self) -> bool {
        // Java: return this.map.has(11) || this.complexScriptFont != null && this.complexScriptFont.dirty() || this.parent != null && this.parent.hasComplexScriptFont();
        self.map.has(11) || self.complexScriptFont.as_ref().map_or(false, |f| f.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasComplexScriptFont())
    }

    // Java: void deleteComplexScriptFont()
    pub fn deleteComplexScriptFont(&mut self) {
        // Java: this.map.put(11, (KoBin)null);
        self.map.clear_key(11);
        // Java: if (this.complexScriptFont != null)
        if self.complexScriptFont.is_some() {
            // Java: this.complexScriptFont = null;
            self.complexScriptFont = None;
        }
    }

    // Java: void writeComplexScriptFont(TextFont value)
    pub fn writeComplexScriptFont(&mut self, value: Option<&TextFont>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(11, bin);
                self.map.put(11, bin);
                // Java: if (this.complexScriptFont != null)
                if self.complexScriptFont.is_some() {
                    // Java: this.complexScriptFont = null;
                    self.complexScriptFont = None;
                }
            }
        }
    }

    // Java: void forceWriteComplexScriptFont(TextFont value)
    pub fn forceWriteComplexScriptFont(&mut self, value: Option<&TextFont>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: this.map.put(11, value.lastBin(this.zone));
            self.map.put(11, value.lastBin(self.zone));
            // Java: if (this.complexScriptFont != null)
            if self.complexScriptFont.is_some() {
                // Java: this.complexScriptFont = null;
                self.complexScriptFont = None;
            }
        }
    }

    // Java: TextFont symbolFont()
    pub fn symbolFont(&self) -> &TextFont {
        // Java: if (this.symbolFont == null)
        // Java: if (this.map.has(12))
        // Java: KoBin bin = this.map.byKey(12);
        // Java: this.symbolFont = (TextFont)KoBin.Decode.toAny(TextFont.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasSymbolFont())
        // Java: this.symbolFont = this.parent.symbolFont();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.symbolFont = (TextFont)KoBin.Decode.toAny(TextFont.class, bin, this.zone);
        // Java: return this.symbolFont;
        todo!()
    }

    // Java: boolean hasSymbolFont()
    pub fn hasSymbolFont(&self) -> bool {
        // Java: return this.map.has(12) || this.symbolFont != null && this.symbolFont.dirty() || this.parent != null && this.parent.hasSymbolFont();
        self.map.has(12) || self.symbolFont.as_ref().map_or(false, |f| f.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasSymbolFont())
    }

    // Java: void deleteSymbolFont()
    pub fn deleteSymbolFont(&mut self) {
        // Java: this.map.put(12, (KoBin)null);
        self.map.clear_key(12);
        // Java: if (this.symbolFont != null)
        if self.symbolFont.is_some() {
            // Java: this.symbolFont = null;
            self.symbolFont = None;
        }
    }

    // Java: void writeSymbolFont(TextFont value)
    pub fn writeSymbolFont(&mut self, value: Option<&TextFont>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(12, bin);
                self.map.put(12, bin);
                // Java: if (this.symbolFont != null)
                if self.symbolFont.is_some() {
                    // Java: this.symbolFont = null;
                    self.symbolFont = None;
                }
            }
        }
    }

    // Java: void forceWriteSymbolFont(TextFont value)
    pub fn forceWriteSymbolFont(&mut self, value: Option<&TextFont>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: this.map.put(12, value.lastBin(this.zone));
            self.map.put(12, value.lastBin(self.zone));
            // Java: if (this.symbolFont != null)
            if self.symbolFont.is_some() {
                // Java: this.symbolFont = null;
                self.symbolFont = None;
            }
        }
    }

    // Java: Hyperlink hlinkClick()
    pub fn hlinkClick(&self) -> &Hyperlink {
        // Java: if (this.hlinkClick == null)
        // Java: if (this.map.has(13))
        // Java: KoBin bin = this.map.byKey(13);
        // Java: this.hlinkClick = (Hyperlink)KoBin.Decode.toAny(Hyperlink.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasHlinkClick())
        // Java: this.hlinkClick = this.parent.hlinkClick();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.hlinkClick = (Hyperlink)KoBin.Decode.toAny(Hyperlink.class, bin, this.zone);
        // Java: return this.hlinkClick;
        todo!()
    }

    // Java: boolean hasHlinkClick()
    pub fn hasHlinkClick(&self) -> bool {
        // Java: return this.map.has(13) || this.hlinkClick != null && this.hlinkClick.dirty() || this.parent != null && this.parent.hasHlinkClick();
        self.map.has(13) || self.hlinkClick.as_ref().map_or(false, |h| h.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasHlinkClick())
    }

    // Java: void deleteHlinkClick()
    pub fn deleteHlinkClick(&mut self) {
        // Java: this.map.put(13, (KoBin)null);
        self.map.clear_key(13);
        // Java: if (this.hlinkClick != null)
        if self.hlinkClick.is_some() {
            // Java: this.hlinkClick = null;
            self.hlinkClick = None;
        }
    }

    // Java: void writeHlinkClick(Hyperlink value)
    pub fn writeHlinkClick(&mut self, value: Option<&Hyperlink>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(13, bin);
                self.map.put(13, bin);
                // Java: if (this.hlinkClick != null)
                if self.hlinkClick.is_some() {
                    // Java: this.hlinkClick = null;
                    self.hlinkClick = None;
                }
            }
        }
    }

    // Java: void forceWriteHlinkClick(Hyperlink value)
    pub fn forceWriteHlinkClick(&mut self, value: Option<&Hyperlink>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: this.map.put(13, value.lastBin(this.zone));
            self.map.put(13, value.lastBin(self.zone));
            // Java: if (this.hlinkClick != null)
            if self.hlinkClick.is_some() {
                // Java: this.hlinkClick = null;
                self.hlinkClick = None;
            }
        }
    }

    // Java: Hyperlink hlinkMouseOver()
    pub fn hlinkMouseOver(&self) -> &Hyperlink {
        // Java: if (this.hlinkMouseOver == null)
        // Java: if (this.map.has(14))
        // Java: KoBin bin = this.map.byKey(14);
        // Java: this.hlinkMouseOver = (Hyperlink)KoBin.Decode.toAny(Hyperlink.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasHlinkMouseOver())
        // Java: this.hlinkMouseOver = this.parent.hlinkMouseOver();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.hlinkMouseOver = (Hyperlink)KoBin.Decode.toAny(Hyperlink.class, bin, this.zone);
        // Java: return this.hlinkMouseOver;
        todo!()
    }

    // Java: boolean hasHlinkMouseOver()
    pub fn hasHlinkMouseOver(&self) -> bool {
        // Java: return this.map.has(14) || this.hlinkMouseOver != null && this.hlinkMouseOver.dirty() || this.parent != null && this.parent.hasHlinkMouseOver();
        self.map.has(14) || self.hlinkMouseOver.as_ref().map_or(false, |h| h.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasHlinkMouseOver())
    }

    // Java: void deleteHlinkMouseOver()
    pub fn deleteHlinkMouseOver(&mut self) {
        // Java: this.map.put(14, (KoBin)null);
        self.map.clear_key(14);
        // Java: if (this.hlinkMouseOver != null)
        if self.hlinkMouseOver.is_some() {
            // Java: this.hlinkMouseOver = null;
            self.hlinkMouseOver = None;
        }
    }

    // Java: void writeHlinkMouseOver(Hyperlink value)
    pub fn writeHlinkMouseOver(&mut self, value: Option<&Hyperlink>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(14, bin);
                self.map.put(14, bin);
                // Java: if (this.hlinkMouseOver != null)
                if self.hlinkMouseOver.is_some() {
                    // Java: this.hlinkMouseOver = null;
                    self.hlinkMouseOver = None;
                }
            }
        }
    }

    // Java: void forceWriteHlinkMouseOver(Hyperlink value)
    pub fn forceWriteHlinkMouseOver(&mut self, value: Option<&Hyperlink>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: this.map.put(14, value.lastBin(this.zone));
            self.map.put(14, value.lastBin(self.zone));
            // Java: if (this.hlinkMouseOver != null)
            if self.hlinkMouseOver.is_some() {
                // Java: this.hlinkMouseOver = null;
                self.hlinkMouseOver = None;
            }
        }
    }

    // Java: String lang()
    pub fn lang(&self) -> String {
        // Java: if (!this.map.has(15))
        if !self.map.has(15) {
            // Java: return this.parent == null ? "" : this.parent.lang();
            return self.parent.as_ref().map_or(String::new(), |p| p.lang());
        // Java: else
        } else {
            // Java: KoBin bin = this.map.byKey(15);
            let bin = self.map.byKey(15);
            // Java: return KoBin.Decode.toText(bin);
            Decode::toText(bin.as_ref())
        }
    }

    // Java: boolean hasLang()
    pub fn hasLang(&self) -> bool {
        // Java: return this.map.has(15) || this.parent != null && this.parent.hasLang();
        self.map.has(15) || self.parent.as_ref().map_or(false, |p| p.hasLang())
    }

    // Java: void deleteLang()
    pub fn deleteLang(&mut self) {
        // Java: this.map.put(15, (KoBin)null);
        self.map.clear_key(15);
    }

    // Java: void putLang(String value)
    pub fn putLang(&mut self, value: &str) {
        // Java: this.map.put(15, KoBin.Encode.from(value, this.zone));
        self.map.put(15, Encode::from_string(value, self.zone));
    }

    // Java: String altLang()
    pub fn altLang(&self) -> String {
        // Java: if (!this.map.has(16))
        if !self.map.has(16) {
            // Java: return this.parent == null ? "" : this.parent.altLang();
            return self.parent.as_ref().map_or(String::new(), |p| p.altLang());
        // Java: else
        } else {
            // Java: KoBin bin = this.map.byKey(16);
            let bin = self.map.byKey(16);
            // Java: return KoBin.Decode.toText(bin);
            Decode::toText(bin.as_ref())
        }
    }

    // Java: boolean hasAltLang()
    pub fn hasAltLang(&self) -> bool {
        // Java: return this.map.has(16) || this.parent != null && this.parent.hasAltLang();
        self.map.has(16) || self.parent.as_ref().map_or(false, |p| p.hasAltLang())
    }

    // Java: void deleteAltLang()
    pub fn deleteAltLang(&mut self) {
        // Java: this.map.put(16, (KoBin)null);
        self.map.clear_key(16);
    }

    // Java: void putAltLang(String value)
    pub fn putAltLang(&mut self, value: &str) {
        // Java: this.map.put(16, KoBin.Encode.from(value, this.zone));
        self.map.put(16, Encode::from_string(value, self.zone));
    }

    // Java: int fontSize()
    pub fn fontSize(&self) -> i32 {
        // Java: if (!this.map.has(17))
        if !self.map.has(17) {
            // Java: return this.parent == null ? 0 : this.parent.fontSize();
            return self.parent.as_ref().map_or(0, |p| p.fontSize());
        // Java: else
        } else {
            // Java: KoBin bin = this.map.byKey(17);
            let bin = self.map.byKey(17);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasFontSize()
    pub fn hasFontSize(&self) -> bool {
        // Java: return this.map.has(17) || this.parent != null && this.parent.hasFontSize();
        self.map.has(17) || self.parent.as_ref().map_or(false, |p| p.hasFontSize())
    }

    // Java: void deleteFontSize()
    pub fn deleteFontSize(&mut self) {
        // Java: this.map.put(17, (KoBin)null);
        self.map.clear_key(17);
    }

    // Java: void putFontSize(int value)
    pub fn putFontSize(&mut self, value: i32) {
        // Java: this.map.put(17, KoBin.Encode.from(value, this.zone));
        self.map.put(17, Encode::from_int(value, self.zone));
    }

    // Java: int underline()
    pub fn underline(&self) -> i32 {
        // Java: if (!this.map.has(18))
        if !self.map.has(18) {
            // Java: return this.parent == null ? 12 : this.parent.underline();
            return self.parent.as_ref().map_or(12, |p| p.underline());
        // Java: else
        } else {
            // Java: KoBin bin = this.map.byKey(18);
            let bin = self.map.byKey(18);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasUnderline()
    pub fn hasUnderline(&self) -> bool {
        // Java: return this.map.has(18) || this.parent != null && this.parent.hasUnderline();
        self.map.has(18) || self.parent.as_ref().map_or(false, |p| p.hasUnderline())
    }

    // Java: void deleteUnderline()
    pub fn deleteUnderline(&mut self) {
        // Java: this.map.put(18, (KoBin)null);
        self.map.clear_key(18);
    }

    // Java: void putUnderline(int value)
    pub fn putUnderline(&mut self, value: i32) {
        // Java: this.map.put(18, KoBin.Encode.from(value, this.zone));
        self.map.put(18, Encode::from_int(value, self.zone));
    }

    // Java: int strike()
    pub fn strike(&self) -> i32 {
        // Java: if (!this.map.has(19))
        if !self.map.has(19) {
            // Java: return this.parent == null ? 0 : this.parent.strike();
            return self.parent.as_ref().map_or(0, |p| p.strike());
        // Java: else
        } else {
            // Java: KoBin bin = this.map.byKey(19);
            let bin = self.map.byKey(19);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasStrike()
    pub fn hasStrike(&self) -> bool {
        // Java: return this.map.has(19) || this.parent != null && this.parent.hasStrike();
        self.map.has(19) || self.parent.as_ref().map_or(false, |p| p.hasStrike())
    }

    // Java: void deleteStrike()
    pub fn deleteStrike(&mut self) {
        // Java: this.map.put(19, (KoBin)null);
        self.map.clear_key(19);
    }

    // Java: void putStrike(int value)
    pub fn putStrike(&mut self, value: i32) {
        // Java: this.map.put(19, KoBin.Encode.from(value, this.zone));
        self.map.put(19, Encode::from_int(value, self.zone));
    }

    // Java: int kern()
    pub fn kern(&self) -> i32 {
        // Java: if (!this.map.has(20))
        if !self.map.has(20) {
            // Java: return this.parent == null ? 0 : this.parent.kern();
            return self.parent.as_ref().map_or(0, |p| p.kern());
        // Java: else
        } else {
            // Java: KoBin bin = this.map.byKey(20);
            let bin = self.map.byKey(20);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasKern()
    pub fn hasKern(&self) -> bool {
        // Java: return this.map.has(20) || this.parent != null && this.parent.hasKern();
        self.map.has(20) || self.parent.as_ref().map_or(false, |p| p.hasKern())
    }

    // Java: void deleteKern()
    pub fn deleteKern(&mut self) {
        // Java: this.map.put(20, (KoBin)null);
        self.map.clear_key(20);
    }

    // Java: void putKern(int value)
    pub fn putKern(&mut self, value: i32) {
        // Java: this.map.put(20, KoBin.Encode.from(value, this.zone));
        self.map.put(20, Encode::from_int(value, self.zone));
    }

    // Java: int cap()
    pub fn cap(&self) -> i32 {
        // Java: if (!this.map.has(21))
        if !self.map.has(21) {
            // Java: return this.parent == null ? 1 : this.parent.cap();
            return self.parent.as_ref().map_or(1, |p| p.cap());
        // Java: else
        } else {
            // Java: KoBin bin = this.map.byKey(21);
            let bin = self.map.byKey(21);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasCap()
    pub fn hasCap(&self) -> bool {
        // Java: return this.map.has(21) || this.parent != null && this.parent.hasCap();
        self.map.has(21) || self.parent.as_ref().map_or(false, |p| p.hasCap())
    }

    // Java: void deleteCap()
    pub fn deleteCap(&mut self) {
        // Java: this.map.put(21, (KoBin)null);
        self.map.clear_key(21);
    }

    // Java: void putCap(int value)
    pub fn putCap(&mut self, value: i32) {
        // Java: this.map.put(21, KoBin.Encode.from(value, this.zone));
        self.map.put(21, Encode::from_int(value, self.zone));
    }

    // Java: int spacing()
    pub fn spacing(&self) -> i32 {
        // Java: if (!this.map.has(22))
        if !self.map.has(22) {
            // Java: return this.parent == null ? 0 : this.parent.spacing();
            return self.parent.as_ref().map_or(0, |p| p.spacing());
        // Java: else
        } else {
            // Java: KoBin bin = this.map.byKey(22);
            let bin = self.map.byKey(22);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasSpacing()
    pub fn hasSpacing(&self) -> bool {
        // Java: return this.map.has(22) || this.parent != null && this.parent.hasSpacing();
        self.map.has(22) || self.parent.as_ref().map_or(false, |p| p.hasSpacing())
    }

    // Java: void deleteSpacing()
    pub fn deleteSpacing(&mut self) {
        // Java: this.map.put(22, (KoBin)null);
        self.map.clear_key(22);
    }

    // Java: void putSpacing(int value)
    pub fn putSpacing(&mut self, value: i32) {
        // Java: this.map.put(22, KoBin.Encode.from(value, this.zone));
        self.map.put(22, Encode::from_int(value, self.zone));
    }

    // Java: int smtId()
    pub fn smtId(&self) -> i32 {
        // Java: if (!this.map.has(23))
        if !self.map.has(23) {
            // Java: return this.parent == null ? 0 : this.parent.smtId();
            return self.parent.as_ref().map_or(0, |p| p.smtId());
        // Java: else
        } else {
            // Java: KoBin bin = this.map.byKey(23);
            let bin = self.map.byKey(23);
            // Java: return KoBin.Decode.toInt(bin);
            Decode::toInt(bin.as_ref())
        }
    }

    // Java: boolean hasSmtId()
    pub fn hasSmtId(&self) -> bool {
        // Java: return this.map.has(23) || this.parent != null && this.parent.hasSmtId();
        self.map.has(23) || self.parent.as_ref().map_or(false, |p| p.hasSmtId())
    }

    // Java: void deleteSmtId()
    pub fn deleteSmtId(&mut self) {
        // Java: this.map.put(23, (KoBin)null);
        self.map.clear_key(23);
    }

    // Java: void putSmtId(int value)
    pub fn putSmtId(&mut self, value: i32) {
        // Java: this.map.put(23, KoBin.Encode.from(value, this.zone));
        self.map.put(23, Encode::from_int(value, self.zone));
    }

    // Java: double baseline()
    pub fn baseline(&self) -> f64 {
        // Java: if (!this.map.has(24))
        if !self.map.has(24) {
            // Java: return this.parent == null ? (double)0.0F : this.parent.baseline();
            return self.parent.as_ref().map_or(0.0f64, |p| p.baseline());
        // Java: else
        } else {
            // Java: KoBin bin = this.map.byKey(24);
            let bin = self.map.byKey(24);
            // Java: return KoBin.Decode.toDouble(bin);
            Decode::toDouble(bin.as_ref())
        }
    }

    // Java: boolean hasBaseline()
    pub fn hasBaseline(&self) -> bool {
        // Java: return this.map.has(24) || this.parent != null && this.parent.hasBaseline();
        self.map.has(24) || self.parent.as_ref().map_or(false, |p| p.hasBaseline())
    }

    // Java: void deleteBaseline()
    pub fn deleteBaseline(&mut self) {
        // Java: this.map.put(24, (KoBin)null);
        self.map.clear_key(24);
    }

    // Java: void putBaseline(double value)
    pub fn putBaseline(&mut self, value: f64) {
        // Java: this.map.put(24, KoBin.Encode.from(value, this.zone));
        self.map.put(24, Encode::from_double(value, self.zone));
    }

    // Java: String bookmarkLinkTarget()
    pub fn bookmarkLinkTarget(&self) -> String {
        // Java: if (!this.map.has(25))
        if !self.map.has(25) {
            // Java: return this.parent == null ? "" : this.parent.bookmarkLinkTarget();
            return self.parent.as_ref().map_or(String::new(), |p| p.bookmarkLinkTarget());
        // Java: else
        } else {
            // Java: KoBin bin = this.map.byKey(25);
            let bin = self.map.byKey(25);
            // Java: return KoBin.Decode.toText(bin);
            Decode::toText(bin.as_ref())
        }
    }

    // Java: boolean hasBookmarkLinkTarget()
    pub fn hasBookmarkLinkTarget(&self) -> bool {
        // Java: return this.map.has(25) || this.parent != null && this.parent.hasBookmarkLinkTarget();
        self.map.has(25) || self.parent.as_ref().map_or(false, |p| p.hasBookmarkLinkTarget())
    }

    // Java: void deleteBookmarkLinkTarget()
    pub fn deleteBookmarkLinkTarget(&mut self) {
        // Java: this.map.put(25, (KoBin)null);
        self.map.clear_key(25);
    }

    // Java: void putBookmarkLinkTarget(String value)
    pub fn putBookmarkLinkTarget(&mut self, value: &str) {
        // Java: this.map.put(25, KoBin.Encode.from(value, this.zone));
        self.map.put(25, Encode::from_string(value, self.zone));
    }

    // Java: TextCharOptions options()
    pub fn options(&self) -> &TextCharOptions {
        // Java: if (this.options == null)
        // Java: if (this.map.has(26))
        // Java: KoBin bin = this.map.byKey(26);
        // Java: this.options = (TextCharOptions)KoBin.Decode.toAny(TextCharOptions.class, bin, this.zone);
        // Java: else if (this.parent != null && this.parent.hasOptions())
        // Java: this.options = this.parent.options();
        // Java: else
        // Java: KoBin bin = KoBin.Null.value1();
        // Java: this.options = (TextCharOptions)KoBin.Decode.toAny(TextCharOptions.class, bin, this.zone);
        // Java: return this.options;
        todo!()
    }

    // LLM: mutable variant for write access
    pub fn options_mut(&mut self) -> &mut TextCharOptions {
        todo!()
    }

    // Java: boolean hasOptions()
    pub fn hasOptions(&self) -> bool {
        // Java: return this.map.has(26) || this.options != null && this.options.dirty() || this.parent != null && this.parent.hasOptions();
        self.map.has(26) || self.options.as_ref().map_or(false, |o| o.dirty()) || self.parent.as_ref().map_or(false, |p| p.hasOptions())
    }

    // Java: void deleteOptions()
    pub fn deleteOptions(&mut self) {
        // Java: this.map.put(26, (KoBin)null);
        self.map.clear_key(26);
        // Java: if (this.options != null)
        if self.options.is_some() {
            // Java: this.options = null;
            self.options = None;
        }
    }

    // Java: void writeOptions(TextCharOptions value)
    pub fn writeOptions(&mut self, value: Option<&TextCharOptions>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: KoBin bin = value.lastBin(this.zone);
            let bin = value.lastBin(self.zone);
            // Java: if (KoBin.Null.notEmpty(bin))
            if Null::notEmpty(Some(bin.as_ref())) {
                // Java: this.map.put(26, bin);
                self.map.put(26, bin);
                // Java: if (this.options != null)
                if self.options.is_some() {
                    // Java: this.options = null;
                    self.options = None;
                }
            }
        }
    }

    // Java: void forceWriteOptions(TextCharOptions value)
    pub fn forceWriteOptions(&mut self, value: Option<&TextCharOptions>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: this.map.put(26, value.lastBin(this.zone));
            self.map.put(26, value.lastBin(self.zone));
            // Java: if (this.options != null)
            if self.options.is_some() {
                // Java: this.options = null;
                self.options = None;
            }
        }
    }
}

impl KoAr for TextCharacterProperties {
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for TextCharacterProperties {
    fn close(&mut self) {
        self.close();
    }
}

// Java: public class TextCharOptions implements KoAr, KoClosable
impl TextCharOptions {
    // Java: static protected static <Value1 extends TextCharOptions> Value1 initialize(Class<Value1> clazz, KoBin bin)
    pub fn initialize(bin: &dyn KoBinTrait) -> Self {
        // Java: KoZone zone = KoZone.enter();
        let zone = KoZone::enter();
        // Java: Value1 inst = (Value1)(zone.allocate(clazz));
        let mut inst = Self::new();
        // Java: inst.zone = zone;
        inst.zone = zone;
        // Java: inst.load(bin);
        inst.load(bin);
        // Java: return inst;
        inst
    }

    // Java: static TextCharOptions create()
    pub fn create() -> Self {
        // Java: return initialize(TextCharOptions.class, KoBin.Null.empty());
        Self::initialize(Null::empty().as_ref())
    }

    // Java: static TextCharOptions open(KoBin bin)
    pub fn open(bin: &dyn KoBinTrait) -> Self {
        // Java: return initialize(TextCharOptions.class, bin);
        Self::initialize(bin)
    }

    // Java: TextCharOptions()
    pub fn new() -> Self {
        Self {
            zone: KoZone::defaultZone(),
            // Java: this.box = new KpBinBox();
            bin_box: KpBinBox::new(),
            // Java: this.map = new KpBool();
            map: KpBool::new(),
            parent: None,
        }
    }

    // Java: TextCharOptions(KoBin bin)
    pub fn new_from(bin: &dyn KoBinTrait) -> Self {
        Self {
            zone: KoZone::defaultZone(),
            // Java: this.box = new KpBinBox(bin);
            bin_box: KpBinBox::KpBinBox_from(Some(bin)),
            // Java: this.map = new KpBool(bin);
            map: KpBool::KpBool_from(bin),
            parent: None,
        }
    }

    // Java: void loadBin(KoBin bin, KoZone zone)
    pub fn loadBin(&mut self, bin: &dyn KoBinTrait, _zone: &KoZone) {
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

    // Java: void load(TextCharOptions value)
    pub fn load_from(&mut self, value: Option<&TextCharOptions>) {
        // Java: if (value != null)
        if let Some(value) = value {
            // Java: this.loadBin(value.lastBin(this.zone), this.zone);
            let zone = self.zone;
            let bin = value.lastBin(zone);
            self.loadBin(bin.as_ref(), zone);
        }
    }

    // Java: KoBin saveBin(KoZone zone)
    pub fn saveBin(&mut self, zone: &KoZone) -> Box<dyn KoBinTrait> {
        // Java: return this.map.dirty() ? this.box.set(this.map.save(zone)) : this.box.get();
        if self.map.dirty() {
            let saved = self.map.save(zone);
            self.bin_box.set(Some(saved.as_ref()));
            self.bin_box.get()
        } else {
            self.bin_box.get()
        }
    }

    // Java: boolean equalsLax(TextCharOptions o)
    pub fn equalsLax(&self, o: &TextCharOptions) -> bool {
        // Java: return this.equals(o);
        self.equals(o)
    }

    // Java: boolean equals(Object o)
    pub fn equals(&self, o: &TextCharOptions) -> bool {
        // Java: if (null != o && o instanceof TextCharOptions)
        // Java: TextCharOptions dst = (TextCharOptions)o;
        let dst = o;
        // Java: return this == dst ? true : this.map.equals(dst.map);
        if std::ptr::eq(self, dst) {
            return true;
        }
        self.map.equals(&dst.map)
        // Java: else
        // Java: return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: return this.map.hashCode();
        self.map.hashCode()
    }

    // Java: boolean dirty()
    pub fn dirty(&self) -> bool {
        // Java: return this.map.dirty();
        self.map.dirty()
    }

    // Java: KoBin save()
    pub fn save(&mut self) -> Box<dyn KoBinTrait> {
        // Java: this.saveBin(this.zone);
        let zone = self.zone;
        self.saveBin(zone);
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

    // Java: void follow(TextCharOptions parent)
    pub fn follow(&mut self, parent: Option<Box<TextCharOptions>>) {
        // Java: if (parent != null)
        if let Some(mut parent) = parent {
            // Java: parent.discard();
            parent.discard();
            // Java: this.discard();
            self.discard();
            // Java: this.parent = parent;
            self.parent = Some(parent);
        }
    }

    // Java: void unFollow()
    pub fn unFollow(&mut self) {
        // Java: if (this.parent != null)
        if self.parent.is_some() {
            // Java: this.discard();
            self.discard();
            // Java: this.parent.discard();
            if let Some(ref mut parent) = self.parent {
                parent.discard();
            }
            // Java: this.cleanParent();
            self.cleanParent();
        }
    }

    // Java: boolean hasParent()
    pub fn hasParent(&self) -> bool {
        // Java: return this.parent != null;
        self.parent.is_some()
    }

    // Java: TextCharOptions parent()
    pub fn parent(&self) -> Option<&TextCharOptions> {
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
    fn cleanParent(&mut self) {
        // Java: this.parent = null;
        self.parent = None;
    }

    // Java: boolean isBold()
    pub fn isBold(&self) -> bool {
        // Java: if (!this.map.has(1))
        if !self.map.has(1) {
            // Java: return this.parent == null ? false : this.parent.isBold();
            return self.parent.as_ref().map_or(false, |p| p.isBold());
        // Java: else
        } else {
            // Java: return this.map.get(1);
            self.map.get(1)
        }
    }

    // Java: boolean hasBold()
    pub fn hasBold(&self) -> bool {
        // Java: return this.map.has(1) || this.parent != null && this.parent.hasBold();
        self.map.has(1) || self.parent.as_ref().map_or(false, |p| p.hasBold())
    }

    // Java: void deleteBold()
    pub fn deleteBold(&mut self) {
        // Java: this.map.clear(1);
        self.map.clear(1);
    }

    // Java: void putBold(boolean value)
    pub fn putBold(&mut self, value: bool) {
        // Java: this.map.set(1, value);
        self.map.set(1, value);
    }

    // Java: boolean isDirty()
    pub fn isDirty(&self) -> bool {
        // Java: if (!this.map.has(2))
        if !self.map.has(2) {
            // Java: return this.parent == null ? true : this.parent.isDirty();
            return self.parent.as_ref().map_or(true, |p| p.isDirty());
        // Java: else
        } else {
            // Java: return this.map.get(2);
            self.map.get(2)
        }
    }

    // Java: boolean hasDirty()
    pub fn hasDirty(&self) -> bool {
        // Java: return this.map.has(2) || this.parent != null && this.parent.hasDirty();
        self.map.has(2) || self.parent.as_ref().map_or(false, |p| p.hasDirty())
    }

    // Java: void deleteDirty()
    pub fn deleteDirty(&mut self) {
        // Java: this.map.clear(2);
        self.map.clear(2);
    }

    // Java: void putDirty(boolean value)
    pub fn putDirty(&mut self, value: bool) {
        // Java: this.map.set(2, value);
        self.map.set(2, value);
    }

    // Java: boolean isSpellingErr()
    pub fn isSpellingErr(&self) -> bool {
        // Java: if (!this.map.has(3))
        if !self.map.has(3) {
            // Java: return this.parent == null ? false : this.parent.isSpellingErr();
            return self.parent.as_ref().map_or(false, |p| p.isSpellingErr());
        // Java: else
        } else {
            // Java: return this.map.get(3);
            self.map.get(3)
        }
    }

    // Java: boolean hasSpellingErr()
    pub fn hasSpellingErr(&self) -> bool {
        // Java: return this.map.has(3) || this.parent != null && this.parent.hasSpellingErr();
        self.map.has(3) || self.parent.as_ref().map_or(false, |p| p.hasSpellingErr())
    }

    // Java: void deleteSpellingErr()
    pub fn deleteSpellingErr(&mut self) {
        // Java: this.map.clear(3);
        self.map.clear(3);
    }

    // Java: void putSpellingErr(boolean value)
    pub fn putSpellingErr(&mut self, value: bool) {
        // Java: this.map.set(3, value);
        self.map.set(3, value);
    }

    // Java: boolean isItalics()
    pub fn isItalics(&self) -> bool {
        // Java: if (!this.map.has(4))
        if !self.map.has(4) {
            // Java: return this.parent == null ? false : this.parent.isItalics();
            return self.parent.as_ref().map_or(false, |p| p.isItalics());
        // Java: else
        } else {
            // Java: return this.map.get(4);
            self.map.get(4)
        }
    }

    // Java: boolean hasItalics()
    pub fn hasItalics(&self) -> bool {
        // Java: return this.map.has(4) || this.parent != null && this.parent.hasItalics();
        self.map.has(4) || self.parent.as_ref().map_or(false, |p| p.hasItalics())
    }

    // Java: void deleteItalics()
    pub fn deleteItalics(&mut self) {
        // Java: this.map.clear(4);
        self.map.clear(4);
    }

    // Java: void putItalics(boolean value)
    pub fn putItalics(&mut self, value: bool) {
        // Java: this.map.set(4, value);
        self.map.set(4, value);
    }

    // Java: boolean isKumimoji()
    pub fn isKumimoji(&self) -> bool {
        // Java: if (!this.map.has(5))
        if !self.map.has(5) {
            // Java: return this.parent == null ? false : this.parent.isKumimoji();
            return self.parent.as_ref().map_or(false, |p| p.isKumimoji());
        // Java: else
        } else {
            // Java: return this.map.get(5);
            self.map.get(5)
        }
    }

    // Java: boolean hasKumimoji()
    pub fn hasKumimoji(&self) -> bool {
        // Java: return this.map.has(5) || this.parent != null && this.parent.hasKumimoji();
        self.map.has(5) || self.parent.as_ref().map_or(false, |p| p.hasKumimoji())
    }

    // Java: void deleteKumimoji()
    pub fn deleteKumimoji(&mut self) {
        // Java: this.map.clear(5);
        self.map.clear(5);
    }

    // Java: void putKumimoji(boolean value)
    pub fn putKumimoji(&mut self, value: bool) {
        // Java: this.map.set(5, value);
        self.map.set(5, value);
    }

    // Java: boolean isNoProof()
    pub fn isNoProof(&self) -> bool {
        // Java: if (!this.map.has(6))
        if !self.map.has(6) {
            // Java: return this.parent == null ? false : this.parent.isNoProof();
            return self.parent.as_ref().map_or(false, |p| p.isNoProof());
        // Java: else
        } else {
            // Java: return this.map.get(6);
            self.map.get(6)
        }
    }

    // Java: boolean hasNoProof()
    pub fn hasNoProof(&self) -> bool {
        // Java: return this.map.has(6) || this.parent != null && this.parent.hasNoProof();
        self.map.has(6) || self.parent.as_ref().map_or(false, |p| p.hasNoProof())
    }

    // Java: void deleteNoProof()
    pub fn deleteNoProof(&mut self) {
        // Java: this.map.clear(6);
        self.map.clear(6);
    }

    // Java: void putNoProof(boolean value)
    pub fn putNoProof(&mut self, value: bool) {
        // Java: this.map.set(6, value);
        self.map.set(6, value);
    }

    // Java: boolean isNormalizeH()
    pub fn isNormalizeH(&self) -> bool {
        // Java: if (!this.map.has(7))
        if !self.map.has(7) {
            // Java: return this.parent == null ? false : this.parent.isNormalizeH();
            return self.parent.as_ref().map_or(false, |p| p.isNormalizeH());
        // Java: else
        } else {
            // Java: return this.map.get(7);
            self.map.get(7)
        }
    }

    // Java: boolean hasNormalizeH()
    pub fn hasNormalizeH(&self) -> bool {
        // Java: return this.map.has(7) || this.parent != null && this.parent.hasNormalizeH();
        self.map.has(7) || self.parent.as_ref().map_or(false, |p| p.hasNormalizeH())
    }

    // Java: void deleteNormalizeH()
    pub fn deleteNormalizeH(&mut self) {
        // Java: this.map.clear(7);
        self.map.clear(7);
    }

    // Java: void putNormalizeH(boolean value)
    pub fn putNormalizeH(&mut self, value: bool) {
        // Java: this.map.set(7, value);
        self.map.set(7, value);
    }

    // Java: boolean isSmtClean()
    pub fn isSmtClean(&self) -> bool {
        // Java: if (!this.map.has(8))
        if !self.map.has(8) {
            // Java: return this.parent == null ? true : this.parent.isSmtClean();
            return self.parent.as_ref().map_or(true, |p| p.isSmtClean());
        // Java: else
        } else {
            // Java: return this.map.get(8);
            self.map.get(8)
        }
    }

    // Java: boolean hasSmtClean()
    pub fn hasSmtClean(&self) -> bool {
        // Java: return this.map.has(8) || this.parent != null && this.parent.hasSmtClean();
        self.map.has(8) || self.parent.as_ref().map_or(false, |p| p.hasSmtClean())
    }

    // Java: void deleteSmtClean()
    pub fn deleteSmtClean(&mut self) {
        // Java: this.map.clear(8);
        self.map.clear(8);
    }

    // Java: void putSmtClean(boolean value)
    pub fn putSmtClean(&mut self, value: bool) {
        // Java: this.map.set(8, value);
        self.map.set(8, value);
    }

    // Java: boolean isRightToLeft()
    pub fn isRightToLeft(&self) -> bool {
        // Java: if (!this.map.has(9))
        if !self.map.has(9) {
            // Java: return this.parent == null ? false : this.parent.isRightToLeft();
            return self.parent.as_ref().map_or(false, |p| p.isRightToLeft());
        // Java: else
        } else {
            // Java: return this.map.get(9);
            self.map.get(9)
        }
    }

    // Java: boolean hasRightToLeft()
    pub fn hasRightToLeft(&self) -> bool {
        // Java: return this.map.has(9) || this.parent != null && this.parent.hasRightToLeft();
        self.map.has(9) || self.parent.as_ref().map_or(false, |p| p.hasRightToLeft())
    }

    // Java: void deleteRightToLeft()
    pub fn deleteRightToLeft(&mut self) {
        // Java: this.map.clear(9);
        self.map.clear(9);
    }

    // Java: void putRightToLeft(boolean value)
    pub fn putRightToLeft(&mut self, value: bool) {
        // Java: this.map.set(9, value);
        self.map.set(9, value);
    }
}

impl KoAr for TextCharOptions {
    fn saveBin(&self, _var1: &KoZone) -> Box<dyn KoBinTrait> { todo!("saveBin requires &mut self") }
    fn loadBin(&mut self, var1: &dyn KoBinTrait, var2: &KoZone) { self.loadBin(var1, var2); }
    fn dirty(&self) -> bool { self.dirty() }
    fn discard(&mut self, var1: &KoZone) { self.discard_with_zone(var1); }
    fn lastBin(&self, var1: &KoZone) -> Box<dyn KoBinTrait> { self.lastBin(var1) }
}

impl KoClosable for TextCharOptions {
    fn close(&mut self) {
        self.close();
    }
}
