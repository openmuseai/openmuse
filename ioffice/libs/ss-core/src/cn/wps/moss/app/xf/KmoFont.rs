use crate::java_runtime::{Rc, RefCell};

use super::KmoFontData::KmoFontData;
use crate::cn::wps::moss::filefmt::biff8::record::{
    common::XFGradient::XFGradient, ExtendedFormatRecord::ExtendedFormatRecord,
    FontRecord::FontRecord,
};
use kfc::cn::wps::memor4j::{
    AtomSnapshot::SharedAtom,
    KmoAtom::KmoAtom,
    KmoAtomData::{KmoAtomData, KmoAtomDataSub},
};
// java public final class KmoFont extends KmoAtom<KmoFontData>
pub struct KmoFont {
    atom: Rc<RefCell<KmoAtom<KmoFontData>>>,
    _owner: SharedAtom,
}
impl KmoFont {
    pub(crate) fn fromData(data: KmoFontData) -> Self {
        let (atom, owner) = super::snapshot::bind(data);
        Self {
            atom,
            _owner: owner,
        }
    }
    // java KmoFont() { super(new KmoFontData()); }

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoFont.java:36
    // java public KmoFont() {
    // java super(new KmoFontData());
    // java }
    pub fn KmoFont() -> Self {
        Self::fromData(KmoFontData::new())
    }
    // java KmoFont(FontRecord fontRec)

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoFont.java:59
    // java KmoFont(FontRecord fontRec) {
    // java super(new KmoFontData());
    // java ((KmoFontData)this.rawData()).field_1_font_height = fontRec.getFontHeight();
    // java ((KmoFontData)this.rawData()).field_2_attributes = fontRec.getAttributes();
    // java ((KmoFontData)this.rawData()).field_3_color_palette_index = fontRec.getColorPaletteIndex();
    // java ((KmoFontData)this.rawData()).field_4_bold_weight = fontRec.getBoldWeight();
    // java ((KmoFontData)this.rawData()).field_5_super_sub_script = fontRec.getSuperSubScript();
    // java ((KmoFontData)this.rawData()).field_6_underline = fontRec.getUnderline();
    // java ((KmoFontData)this.rawData()).field_7_family = fontRec.getFamily();
    // java ((KmoFontData)this.rawData()).field_8_charset = fontRec.getCharset();
    // java ((KmoFontData)this.rawData()).field_11_font_name = fontRec.getFontName();
    // java }
    pub fn fromRecord(rec: &FontRecord) -> Self {
        Self::fromData(KmoFontData::fromRecord(rec))
    }
    // java public boolean isCondense() {
    // java KmoFontData data = (KmoFontData)this.rawData();
    // java return data.hasInit ? data.isFcondense : fcondense.isSet(data.field_2_attributes);
    // java }
    pub fn isCondense(&self) -> bool {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.isFcondense
        } else {
            ((data.field_2_attributes) as i32 & 64) != 0
        }
    }
    // java public boolean isExtend() {
    // java KmoFontData data = (KmoFontData)this.rawData();
    // java return data.hasInit ? data.isFextend : fextend.isSet(((KmoFontData)this.rawData()).field_2_attributes);
    // java }
    pub fn isExtend(&self) -> bool {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.isFextend
        } else {
            ((data.field_2_attributes) as i32 & 128) != 0
        }
    }
    // java public void setFontHeight(short height) {
    // java this.beforeWrite();
    // java ((KmoFontData)this.rawData()).field_1_font_height = height;
    // java }
    pub fn setFontHeight(&self, height: i16) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.field_1_font_height = height;
    }
    // java public void setItalic(boolean italics) {
    // java this.beforeWrite();
    // java ((KmoFontData)this.rawData()).isItalic = italics;
    // java ((KmoFontData)this.rawData()).field_2_attributes = italic.setShortBoolean(((KmoFontData)this.rawData()).field_2_attributes, italics);
    // java }
    pub fn setItalic(&self, italics: bool) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.isItalic = italics;
        data.field_2_attributes =
            (((data.field_2_attributes) as i32 & !2) | if italics { 2 } else { 0 }) as i16;
    }
    // java public void setStrikeout(boolean strike) {
    // java this.beforeWrite();
    // java ((KmoFontData)this.rawData()).isStrikeout = strike;
    // java ((KmoFontData)this.rawData()).field_2_attributes = strikeout.setShortBoolean(((KmoFontData)this.rawData()).field_2_attributes, strike);
    // java }
    pub fn setStrikeout(&self, strike: bool) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.isStrikeout = strike;
        data.field_2_attributes =
            (((data.field_2_attributes) as i32 & !8) | if strike { 8 } else { 0 }) as i16;
    }
    // java public void setMacoutline(boolean mac) {
    // java this.beforeWrite();
    // java ((KmoFontData)this.rawData()).isMacoutline = mac;
    // java ((KmoFontData)this.rawData()).field_2_attributes = macoutline.setShortBoolean(((KmoFontData)this.rawData()).field_2_attributes, mac);
    // java }
    pub fn setMacoutline(&self, mac: bool) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.isMacoutline = mac;
        data.field_2_attributes =
            (((data.field_2_attributes) as i32 & !16) | if mac { 16 } else { 0 }) as i16;
    }
    // java public void setMacshadow(boolean mac) {
    // java this.beforeWrite();
    // java ((KmoFontData)this.rawData()).isMacshadow = mac;
    // java ((KmoFontData)this.rawData()).field_2_attributes = macshadow.setShortBoolean(((KmoFontData)this.rawData()).field_2_attributes, mac);
    // java }
    pub fn setMacshadow(&self, mac: bool) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.isMacshadow = mac;
        data.field_2_attributes =
            (((data.field_2_attributes) as i32 & !32) | if mac { 32 } else { 0 }) as i16;
    }
    // java public void setColorPaletteIndex(int cpi) {
    // java this.beforeWrite();
    // java ((KmoFontData)this.rawData()).field_3_color_palette_index = cpi;
    // java }
    pub fn setColorPaletteIndex(&self, cpi: i32) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.field_3_color_palette_index = cpi;
    }
    // java public void setBoldWeight(short bw) {
    // java this.beforeWrite();
    // java ((KmoFontData)this.rawData()).field_4_bold_weight = bw;
    // java }
    pub fn setBoldWeight(&self, bw: i16) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.field_4_bold_weight = bw;
    }
    // java public void setSuperSubScript(short sss) {
    // java this.beforeWrite();
    // java ((KmoFontData)this.rawData()).field_5_super_sub_script = sss;
    // java }
    pub fn setSuperSubScript(&self, sss: i16) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.field_5_super_sub_script = sss;
    }
    // java public void setUnderline(byte u) {
    // java this.beforeWrite();
    // java ((KmoFontData)this.rawData()).field_6_underline = u;
    // java }
    pub fn setUnderline(&self, u: i8) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.field_6_underline = u;
    }
    // java public void setFamily(byte f) {
    // java this.beforeWrite();
    // java ((KmoFontData)this.rawData()).field_7_family = f;
    // java }
    pub fn setFamily(&self, f: i8) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.field_7_family = f;
    }
    // java public void setCharset(byte charset) {
    // java this.beforeWrite();
    // java ((KmoFontData)this.rawData()).field_8_charset = charset;
    // java }
    pub fn setCharset(&self, charset: i8) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.field_8_charset = charset;
    }
    // java public void setFontName(String fn) {
    // java this.beforeWrite();
    // java ((KmoFontData)this.rawData()).field_11_font_name = fn;
    // java }
    pub fn setFontName(&self, r#fn: String) {
        let mut atom = self.atom.borrow_mut();
        atom.beforeWrite();
        let mut data = atom.rawData_mut();
        data.field_11_font_name = r#fn;
    }
    // java public short getFontHeight() {
    // java return ((KmoFontData)this.rawData()).field_1_font_height;
    // java }
    pub fn getFontHeight(&self) -> i16 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        data.field_1_font_height
    }
    // java public boolean isItalic() {
    // java KmoFontData data = (KmoFontData)this.rawData();
    // java return data.hasInit ? data.isItalic : italic.isSet(data.field_2_attributes);
    // java }
    pub fn isItalic(&self) -> bool {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.isItalic
        } else {
            ((data.field_2_attributes) as i32 & 2) != 0
        }
    }
    // java public boolean isStrikeout() {
    // java KmoFontData data = (KmoFontData)this.rawData();
    // java return data.hasInit ? data.isStrikeout : strikeout.isSet(data.field_2_attributes);
    // java }
    pub fn isStrikeout(&self) -> bool {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.isStrikeout
        } else {
            ((data.field_2_attributes) as i32 & 8) != 0
        }
    }
    // java public boolean isMacoutlined() {
    // java KmoFontData data = (KmoFontData)this.rawData();
    // java return data.hasInit ? data.isMacoutline : macoutline.isSet(data.field_2_attributes);
    // java }
    pub fn isMacoutlined(&self) -> bool {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.isMacoutline
        } else {
            ((data.field_2_attributes) as i32 & 16) != 0
        }
    }
    // java public boolean isMacshadowed() {
    // java KmoFontData data = (KmoFontData)this.rawData();
    // java return data.hasInit ? data.isMacshadow : macshadow.isSet(data.field_2_attributes);
    // java }
    pub fn isMacshadowed(&self) -> bool {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        if data.hasInit {
            data.isMacshadow
        } else {
            ((data.field_2_attributes) as i32 & 32) != 0
        }
    }
    // java public int getColorPaletteIndex() {
    // java return ((KmoFontData)this.rawData()).field_3_color_palette_index;
    // java }
    pub fn getColorPaletteIndex(&self) -> i32 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        data.field_3_color_palette_index
    }
    // java public short getBoldWeight() {
    // java return ((KmoFontData)this.rawData()).field_4_bold_weight;
    // java }
    pub fn getBoldWeight(&self) -> i16 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        data.field_4_bold_weight
    }
    // java public short getSuperSubScript() {
    // java return ((KmoFontData)this.rawData()).field_5_super_sub_script;
    // java }
    pub fn getSuperSubScript(&self) -> i16 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        data.field_5_super_sub_script
    }
    // java public byte getUnderline() {
    // java return ((KmoFontData)this.rawData()).field_6_underline;
    // java }
    pub fn getUnderline(&self) -> i8 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        data.field_6_underline
    }
    // java public byte getFamily() {
    // java return ((KmoFontData)this.rawData()).field_7_family;
    // java }
    pub fn getFamily(&self) -> i8 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        data.field_7_family
    }
    // java public byte getCharset() {
    // java return ((KmoFontData)this.rawData()).field_8_charset;
    // java }
    pub fn getCharset(&self) -> i8 {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        data.field_8_charset
    }
    // java public String getFontName() {
    // java return ((KmoFontData)this.rawData()).field_11_font_name;
    // java }
    pub fn getFontName(&self) -> String {
        let atom = self.atom.borrow();
        let data = atom.rawData();
        data.field_11_font_name.clone()
    }

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoFont.java:218
    // java public void copyToRecord(FontRecord fontRec) {
    // java ((KmoFontData)this.rawData()).copyToRecord(fontRec);
    // java }
    pub fn copyToRecord(&self, rec: &mut FontRecord) {
        self.atom.borrow().rawData().copyToRecord(rec);
    }
    // rust: public void completeCopy(KmoFont source)

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoFont.java:237
    // java public void completeCopy(KmoFont font) {
    // java ((KmoFontData)this.rawData()).completeCopy((KmoFontData)font.rawData());
    // java }
    pub fn completeCopy(&self, source: &Self) {
        let source = source.atom.borrow().rawData().clone();
        self.atom.borrow_mut().rawData_mut().completeCopy(&source);
    }
    // java protected void initOption() {
    // java KmoFontData data = (KmoFontData)this.rawData();
    // java short field_2_attributes = data.field_2_attributes;
    // java data.hasInit = true;
    // java data.isItalic = italic.isSet(field_2_attributes);
    // java data.isStrikeout = strikeout.isSet(field_2_attributes);
    // java data.isMacoutline = macoutline.isSet(field_2_attributes);
    // java data.isMacshadow = macshadow.isSet(field_2_attributes);
    // java data.isFcondense = fcondense.isSet(field_2_attributes);
    // java data.isFextend = fextend.isSet(field_2_attributes);
    // java }
    pub fn initOption(&self) {
        let mut atom = self.atom.borrow_mut();
        let mut data = atom.rawData_mut();
        let bits = data.field_2_attributes;
        data.hasInit = true;
        data.isItalic = bits & 2 != 0;
        data.isStrikeout = bits & 8 != 0;
        data.isMacoutline = bits & 16 != 0;
        data.isMacshadow = bits & 32 != 0;
        data.isFcondense = bits & 64 != 0;
        data.isFextend = bits & 128 != 0;
    }
    // java public static KmoFont createFont(KmoFont font) { return new KmoFont(font); }

    // rust: source koffice/et/core/cn/wps/moss/app/xf/KmoFont.java:32
    // java public static KmoFont createFont(KmoFont font) {
    // java return new KmoFont(font);
    // java }
    pub fn createFont(font: &Self) -> Self {
        let data = font.atom.borrow();
        let mut result = KmoFontData::new();
        result.completeCopy(&data.rawData());
        result.isFcondense = false;
        result.isFextend = false;
        Self::fromData(result)
    }
}
impl PartialEq for KmoFont {
    fn eq(&self, rhs: &Self) -> bool {
        let a = self.atom.borrow();
        let b = rhs.atom.borrow();
        let a = a.rawData();
        let b = b.rawData();
        a.field_1_font_height == b.field_1_font_height
            && a.field_2_attributes == b.field_2_attributes
            && a.field_3_color_palette_index == b.field_3_color_palette_index
            && a.field_4_bold_weight == b.field_4_bold_weight
            && a.field_5_super_sub_script == b.field_5_super_sub_script
            && a.field_6_underline == b.field_6_underline
            && a.field_7_family == b.field_7_family
            && a.field_8_charset == b.field_8_charset
            && a.field_11_font_name == b.field_11_font_name
    }
}
