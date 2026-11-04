use super::KmoFont::KmoFont;
use crate::cn::wps::moss::filefmt::biff8::record::{
    common::XFGradient::XFGradient, ExtendedFormatRecord::ExtendedFormatRecord,
    FontRecord::FontRecord,
};
use crate::java_runtime::{Rc, RefCell};
use kfc::cn::wps::memor4j::{
    AtomSnapshot::SharedAtom,
    KmoAtom::KmoAtom,
    KmoAtomData::{KmoAtomData, KmoAtomDataSub},
};
// java class KmoFontData extends KmoAtomData
#[derive(Clone)]
pub struct KmoFontData {
    pub base: KmoAtomData,
    // java protected short field_1_font_height;
    pub(crate) field_1_font_height: i16,
    // java protected short field_2_attributes;
    pub(crate) field_2_attributes: i16,
    // java protected boolean isItalic;
    pub(crate) isItalic: bool,
    // java protected boolean isStrikeout;
    pub(crate) isStrikeout: bool,
    // java protected boolean isMacoutline;
    pub(crate) isMacoutline: bool,
    // java protected boolean isMacshadow;
    pub(crate) isMacshadow: bool,
    // java protected boolean isFcondense;
    pub(crate) isFcondense: bool,
    // java protected boolean isFextend;
    pub(crate) isFextend: bool,
    // rust: protected boolean hasInit;
    pub(crate) hasInit: bool,
    // java protected int field_3_color_palette_index;
    pub(crate) field_3_color_palette_index: i32,
    // java protected short field_4_bold_weight;
    pub(crate) field_4_bold_weight: i16,
    // java protected short field_5_super_sub_script;
    pub(crate) field_5_super_sub_script: i16,
    // java protected byte field_6_underline;
    pub(crate) field_6_underline: i8,
    // java protected byte field_7_family;
    pub(crate) field_7_family: i8,
    // java protected byte field_8_charset;
    pub(crate) field_8_charset: i8,
    // rust: protected String field_11_font_name;
    pub(crate) field_11_font_name: String,
}
impl KmoFontData {
    pub fn new() -> Self {
        Self {
            base: KmoAtomData::new(),
            field_1_font_height: 0,
            field_2_attributes: 0,
            isItalic: false,
            isStrikeout: false,
            isMacoutline: false,
            isMacshadow: false,
            isFcondense: false,
            isFextend: false,
            hasInit: false,
            field_3_color_palette_index: 0,
            field_4_bold_weight: 0,
            field_5_super_sub_script: 0,
            field_6_underline: 0,
            field_7_family: 0,
            field_8_charset: 0,
            field_11_font_name: String::new(),
        }
    }
    // java protected void completeCopy(KmoFontData font) {
    // java this.field_1_font_height = font.field_1_font_height;
    // java this.field_2_attributes = font.field_2_attributes;
    // java this.field_3_color_palette_index = font.field_3_color_palette_index;
    // java this.field_4_bold_weight = font.field_4_bold_weight;
    // java this.field_5_super_sub_script = font.field_5_super_sub_script;
    // java this.field_6_underline = font.field_6_underline;
    // java this.field_7_family = font.field_7_family;
    // java this.field_8_charset = font.field_8_charset;
    // java this.field_11_font_name = font.field_11_font_name;
    // java this.hasInit = font.hasInit;
    // java this.isItalic = font.isItalic;
    // java this.isStrikeout = font.isStrikeout;
    // java this.isMacoutline = font.isMacoutline;
    // java this.isMacshadow = font.isMacshadow;
    // java this.isFcondense = font.isFcondense;
    // java this.isFextend = font.isFextend;
    // java }
    pub fn completeCopy(&mut self, source: &Self) {
        let base = self.base.clone();
        *self = source.clone();
        self.base = base;
    }
    pub fn fromRecord(rec: &FontRecord) -> Self {
        let mut data = Self::new();
        // rust: ((KmoFontData)this.rawData()).field_1_font_height = fontRec.getFontHeight();
        data.field_1_font_height = rec.getFontHeight();
        // rust: ((KmoFontData)this.rawData()).field_2_attributes = fontRec.getAttributes();
        data.field_2_attributes = rec.getAttributes();
        // rust: ((KmoFontData)this.rawData()).field_3_color_palette_index = fontRec.getColorPaletteIndex();
        data.field_3_color_palette_index = rec.getColorPaletteIndex();
        // rust: ((KmoFontData)this.rawData()).field_4_bold_weight = fontRec.getBoldWeight();
        data.field_4_bold_weight = rec.getBoldWeight();
        // rust: ((KmoFontData)this.rawData()).field_5_super_sub_script = fontRec.getSuperSubScript();
        data.field_5_super_sub_script = rec.getSuperSubScript();
        // rust: ((KmoFontData)this.rawData()).field_6_underline = fontRec.getUnderline();
        data.field_6_underline = rec.getUnderline();
        // rust: ((KmoFontData)this.rawData()).field_7_family = fontRec.getFamily();
        data.field_7_family = rec.getFamily();
        // rust: ((KmoFontData)this.rawData()).field_8_charset = fontRec.getCharset();
        data.field_8_charset = rec.getCharset();
        // rust: ((KmoFontData)this.rawData()).field_11_font_name = fontRec.getFontName();
        data.field_11_font_name = rec.getFontName();
        data
    }
    // java protected void copyToRecord(FontRecord fontRec) {
    // java fontRec.setFontHeight(this.field_1_font_height);
    // java fontRec.setAttributes(this.field_2_attributes);
    // java fontRec.setColorPaletteIndex(this.field_3_color_palette_index);
    // java fontRec.setBoldWeight(this.field_4_bold_weight);
    // java fontRec.setSuperSubScript(this.field_5_super_sub_script);
    // java fontRec.setUnderline(this.field_6_underline);
    // java fontRec.setFamily(this.field_7_family);
    // java fontRec.setCharset(this.field_8_charset);
    // java fontRec.setFontName(this.field_11_font_name);
    // java }
    pub fn copyToRecord(&self, rec: &mut FontRecord) {
        rec.setFontHeight(self.field_1_font_height.clone());
        rec.setAttributes(self.field_2_attributes.clone());
        rec.setColorPaletteIndex(self.field_3_color_palette_index.clone());
        rec.setBoldWeight(self.field_4_bold_weight.clone());
        rec.setSuperSubScript(self.field_5_super_sub_script.clone());
        rec.setUnderline(self.field_6_underline.clone());
        rec.setFamily(self.field_7_family.clone());
        rec.setCharset(self.field_8_charset.clone());
        rec.setFontName(self.field_11_font_name.clone());
    }
}
impl KmoAtomDataSub for KmoFontData {
    fn as_base(&self) -> &KmoAtomData {
        &self.base
    }
    fn as_base_mut(&mut self) -> &mut KmoAtomData {
        &mut self.base
    }
    fn copy(&self) -> Self {
        let mut data = Self::new();
        data.completeCopy(self);
        data
    }
}
