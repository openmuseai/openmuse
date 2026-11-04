use util::org::apache::poi::util::BitField::BitField;
// java public final class FontRecord
#[derive(Clone, Debug, PartialEq)]
pub struct FontRecord {
    // rust: private short field_1_font_height;
    pub(crate) field_1_font_height: i16,
    // java private short field_2_attributes;
    pub(crate) field_2_attributes: i16,
    // rust: private int field_3_color_palette_index;
    pub(crate) field_3_color_palette_index: i32,
    // java private short field_4_bold_weight;
    pub(crate) field_4_bold_weight: i16,
    // java private short field_5_super_sub_script;
    pub(crate) field_5_super_sub_script: i16,
    // java private byte field_6_underline;
    pub(crate) field_6_underline: i8,
    // java private byte field_7_family;
    pub(crate) field_7_family: i8,
    // java private byte field_8_charset;
    pub(crate) field_8_charset: i8,
    // rust: private byte field_9_zero;
    pub(crate) field_9_zero: i8,
    // rust: private String field_11_font_name;
    pub(crate) field_11_font_name: String,
    // java private byte dxfOptions;
    pub(crate) dxfOptions: i8,
}
impl Default for FontRecord {
    fn default() -> Self {
        Self {
            field_1_font_height: -1,
            field_2_attributes: 0,
            field_3_color_palette_index: 32767,
            field_4_bold_weight: 0,
            field_5_super_sub_script: 0,
            field_6_underline: 0,
            field_7_family: 0,
            field_8_charset: 0,
            field_9_zero: 0,
            field_11_font_name: "".to_string(),
            dxfOptions: 0,
        }
    }
}
impl FontRecord {
    // java public FontRecord() { }

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:44
    // java public FontRecord() {
    // java }
    pub fn FontRecord() -> Self {
        Self::default()
    }
    // java public void setFontHeight(short height) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:55
    // java public void setFontHeight(short height) {
    // java this.field_1_font_height = height;
    // java }
    pub fn setFontHeight(&mut self, height: i16) {
        // java this.field_1_font_height = height;
        self.field_1_font_height = height;
    }
    // java public void setAttributes(short attributes) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:59
    // java public void setAttributes(short attributes) {
    // java this.field_2_attributes = attributes;
    // java }
    pub fn setAttributes(&mut self, attributes: i16) {
        // java this.field_2_attributes = attributes;
        self.field_2_attributes = attributes;
    }
    // java public void setItalic(boolean italics) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:63
    // java public void setItalic(boolean italics) {
    // java this.field_2_attributes = italic.setShortBoolean(this.field_2_attributes, italics);
    // java }
    pub fn setItalic(&mut self, italics: bool) {
        // java this.field_2_attributes = italic.setShortBoolean(this.field_2_attributes, italics);
        self.field_2_attributes =
            (((self.field_2_attributes as i32) & !2) | if italics { 2 } else { 0 }) as i16;
    }
    // java public void setStrikeout(boolean strike) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:67
    // java public void setStrikeout(boolean strike) {
    // java this.field_2_attributes = strikeout.setShortBoolean(this.field_2_attributes, strike);
    // java }
    pub fn setStrikeout(&mut self, strike: bool) {
        // java this.field_2_attributes = strikeout.setShortBoolean(this.field_2_attributes, strike);
        self.field_2_attributes =
            (((self.field_2_attributes as i32) & !8) | if strike { 8 } else { 0 }) as i16;
    }
    // java public void setMacoutline(boolean mac) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:71
    // java public void setMacoutline(boolean mac) {
    // java this.field_2_attributes = macoutline.setShortBoolean(this.field_2_attributes, mac);
    // java }
    pub fn setMacoutline(&mut self, mac: bool) {
        // java this.field_2_attributes = macoutline.setShortBoolean(this.field_2_attributes, mac);
        self.field_2_attributes =
            (((self.field_2_attributes as i32) & !16) | if mac { 16 } else { 0 }) as i16;
    }
    // java public void setMacshadow(boolean mac) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:75
    // java public void setMacshadow(boolean mac) {
    // java this.field_2_attributes = macshadow.setShortBoolean(this.field_2_attributes, mac);
    // java }
    pub fn setMacshadow(&mut self, mac: bool) {
        // java this.field_2_attributes = macshadow.setShortBoolean(this.field_2_attributes, mac);
        self.field_2_attributes =
            (((self.field_2_attributes as i32) & !32) | if mac { 32 } else { 0 }) as i16;
    }
    // java public void setColorPaletteIndex(int cpi) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:79
    // java public void setColorPaletteIndex(int cpi) {
    // java this.field_3_color_palette_index = cpi;
    // java }
    pub fn setColorPaletteIndex(&mut self, cpi: i32) {
        // java this.field_3_color_palette_index = cpi;
        self.field_3_color_palette_index = cpi;
    }
    // java public void setBoldWeight(short bw) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:83
    // java public void setBoldWeight(short bw) {
    // java this.field_4_bold_weight = bw;
    // java }
    pub fn setBoldWeight(&mut self, bw: i16) {
        // java this.field_4_bold_weight = bw;
        self.field_4_bold_weight = bw;
    }
    // java public void setSuperSubScript(short sss) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:87
    // java public void setSuperSubScript(short sss) {
    // java this.field_5_super_sub_script = sss;
    // java }
    pub fn setSuperSubScript(&mut self, sss: i16) {
        // java this.field_5_super_sub_script = sss;
        self.field_5_super_sub_script = sss;
    }
    // java public void setUnderline(byte u) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:91
    // java public void setUnderline(byte u) {
    // java this.field_6_underline = u;
    // java }
    pub fn setUnderline(&mut self, u: i8) {
        // java this.field_6_underline = u;
        self.field_6_underline = u;
    }
    // java public void setFamily(byte f) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:95
    // java public void setFamily(byte f) {
    // java this.field_7_family = f;
    // java }
    pub fn setFamily(&mut self, f: i8) {
        // java this.field_7_family = f;
        self.field_7_family = f;
    }
    // java public void setCharset(byte charset) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:99
    // java public void setCharset(byte charset) {
    // java this.field_8_charset = charset;
    // java }
    pub fn setCharset(&mut self, charset: i8) {
        // java this.field_8_charset = charset;
        self.field_8_charset = charset;
    }
    // java public void setFontName(String fn) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:103
    // java public void setFontName(String fn) {
    // java this.field_11_font_name = fn;
    // java }
    pub fn setFontName(&mut self, r#fn: String) {
        // java this.field_11_font_name = fn;
        self.field_11_font_name = r#fn.clone();
    }
    // java public short getFontHeight() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:107
    // java public short getFontHeight() {
    // java return this.field_1_font_height;
    // java }
    pub fn getFontHeight(&self) -> i16 {
        // java return this.field_1_font_height;
        (self.field_1_font_height) as i16
    }
    // java public short getAttributes() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:111
    // java public short getAttributes() {
    // java return this.field_2_attributes;
    // java }
    pub fn getAttributes(&self) -> i16 {
        // java return this.field_2_attributes;
        (self.field_2_attributes) as i16
    }
    // java public boolean isItalic() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:115
    // java public boolean isItalic() {
    // java return italic.isSet(this.field_2_attributes);
    // java }
    pub fn isItalic(&self) -> bool {
        // java return italic.isSet(this.field_2_attributes);
        ((self.field_2_attributes as i32) & 2) != 0
    }
    // java public boolean isStruckout() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:119
    // java public boolean isStruckout() {
    // java return strikeout.isSet(this.field_2_attributes);
    // java }
    pub fn isStruckout(&self) -> bool {
        // java return strikeout.isSet(this.field_2_attributes);
        ((self.field_2_attributes as i32) & 8) != 0
    }
    // java public boolean isMacoutlined() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:123
    // java public boolean isMacoutlined() {
    // java return macoutline.isSet(this.field_2_attributes);
    // java }
    pub fn isMacoutlined(&self) -> bool {
        // java return macoutline.isSet(this.field_2_attributes);
        ((self.field_2_attributes as i32) & 16) != 0
    }
    // java public boolean isMacshadowed() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:127
    // java public boolean isMacshadowed() {
    // java return macshadow.isSet(this.field_2_attributes);
    // java }
    pub fn isMacshadowed(&self) -> bool {
        // java return macshadow.isSet(this.field_2_attributes);
        ((self.field_2_attributes as i32) & 32) != 0
    }
    // java public int getColorPaletteIndex() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:131
    // java public int getColorPaletteIndex() {
    // java return this.field_3_color_palette_index;
    // java }
    pub fn getColorPaletteIndex(&self) -> i32 {
        // java return this.field_3_color_palette_index;
        (self.field_3_color_palette_index) as i32
    }
    // java public short getBoldWeight() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:135
    // java public short getBoldWeight() {
    // java return this.field_4_bold_weight;
    // java }
    pub fn getBoldWeight(&self) -> i16 {
        // java return this.field_4_bold_weight;
        (self.field_4_bold_weight) as i16
    }
    // java public short getSuperSubScript() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:139
    // java public short getSuperSubScript() {
    // java return this.field_5_super_sub_script;
    // java }
    pub fn getSuperSubScript(&self) -> i16 {
        // java return this.field_5_super_sub_script;
        (self.field_5_super_sub_script) as i16
    }
    // java public byte getUnderline() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:143
    // java public byte getUnderline() {
    // java return this.field_6_underline;
    // java }
    pub fn getUnderline(&self) -> i8 {
        // java return this.field_6_underline;
        (self.field_6_underline) as i8
    }
    // java public byte getFamily() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:147
    // java public byte getFamily() {
    // java return this.field_7_family;
    // java }
    pub fn getFamily(&self) -> i8 {
        // java return this.field_7_family;
        (self.field_7_family) as i8
    }
    // java public byte getCharset() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:151
    // java public byte getCharset() {
    // java return this.field_8_charset;
    // java }
    pub fn getCharset(&self) -> i8 {
        // java return this.field_8_charset;
        (self.field_8_charset) as i8
    }
    // java public String getFontName() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:155
    // java public String getFontName() {
    // java return this.field_11_font_name;
    // java }
    pub fn getFontName(&self) -> String {
        // java return this.field_11_font_name;
        self.field_11_font_name.clone()
    }
    // java public void setItalicOption() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:363
    // java public void setItalicOption() {
    // java this.dxfOptions = italic_option.setByteBoolean(this.dxfOptions, true);
    // java }
    pub fn setItalicOption(&mut self) {
        // java this.dxfOptions = italic_option.setByteBoolean(this.dxfOptions, true);
        self.dxfOptions = (((self.dxfOptions as i32) & !2) | if true { 2 } else { 0 }) as i8;
    }
    // java public void setBoldOption() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:371
    // java public void setBoldOption() {
    // java this.dxfOptions = bold_option.setByteBoolean(this.dxfOptions, true);
    // java }
    pub fn setBoldOption(&mut self) {
        // java this.dxfOptions = bold_option.setByteBoolean(this.dxfOptions, true);
        self.dxfOptions = (((self.dxfOptions as i32) & !1) | if true { 1 } else { 0 }) as i8;
    }
    // java public void setUnderLineOption() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:379
    // java public void setUnderLineOption() {
    // java this.dxfOptions = underline_option.setByteBoolean(this.dxfOptions, true);
    // java }
    pub fn setUnderLineOption(&mut self) {
        // java this.dxfOptions = underline_option.setByteBoolean(this.dxfOptions, true);
        self.dxfOptions = (((self.dxfOptions as i32) & !4) | if true { 4 } else { 0 }) as i8;
    }
    // java public void setColorOption() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:387
    // java public void setColorOption() {
    // java this.dxfOptions = color_option.setByteBoolean(this.dxfOptions, true);
    // java }
    pub fn setColorOption(&mut self) {
        // java this.dxfOptions = color_option.setByteBoolean(this.dxfOptions, true);
        self.dxfOptions = (((self.dxfOptions as i32) & !8) | if true { 8 } else { 0 }) as i8;
    }
    // java public void setStrikeOption() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/FontRecord.java:395
    // java public void setStrikeOption() {
    // java this.dxfOptions = strike_option.setByteBoolean(this.dxfOptions, true);
    // java }
    pub fn setStrikeOption(&mut self) {
        // java this.dxfOptions = strike_option.setByteBoolean(this.dxfOptions, true);
        self.dxfOptions = (((self.dxfOptions as i32) & !16) | if true { 16 } else { 0 }) as i8;
    }
    // java public void cloneStyleFrom(FontRecord source) {
    // java this.field_1_font_height = source.field_1_font_height;
    // java this.field_2_attributes = source.field_2_attributes;
    // java this.field_3_color_palette_index = source.field_3_color_palette_index;
    // java this.field_4_bold_weight = source.field_4_bold_weight;
    // java this.field_5_super_sub_script = source.field_5_super_sub_script;
    // java this.field_6_underline = source.field_6_underline;
    // java this.field_7_family = source.field_7_family;
    // java this.field_8_charset = source.field_8_charset;
    // java this.field_9_zero = source.field_9_zero;
    // java this.field_11_font_name = source.field_11_font_name;
    // java }
    pub fn cloneStyleFrom(&mut self, source: &Self) {
        self.field_1_font_height = source.field_1_font_height.clone();
        self.field_2_attributes = source.field_2_attributes.clone();
        self.field_3_color_palette_index = source.field_3_color_palette_index.clone();
        self.field_4_bold_weight = source.field_4_bold_weight.clone();
        self.field_5_super_sub_script = source.field_5_super_sub_script.clone();
        self.field_6_underline = source.field_6_underline.clone();
        self.field_7_family = source.field_7_family.clone();
        self.field_8_charset = source.field_8_charset.clone();
        self.field_9_zero = source.field_9_zero.clone();
        self.field_11_font_name = source.field_11_font_name.clone();
    }
// java public boolean hasItalic() {
// java return italic_option.isSet(this.dxfOptions);
// java }
pub fn hasItalic(&self)->bool {
self.dxfOptions&2!=0
}
// java public boolean hasBold() {
// java return bold_option.isSet(this.dxfOptions);
// java }
pub fn hasBold(&self)->bool {
self.dxfOptions&1!=0
}
// java public boolean hasUnderline() {
// java return underline_option.isSet(this.dxfOptions);
// java }
pub fn hasUnderline(&self)->bool {
self.dxfOptions&4!=0
}
// java public boolean hasColor() {
// java return color_option.isSet(this.dxfOptions);
// java }
pub fn hasColor(&self)->bool {
self.dxfOptions&8!=0
}
// java public boolean hasStrike() {
// java return strike_option.isSet(this.dxfOptions);
// java }
pub fn hasStrike(&self)->bool {
self.dxfOptions&16!=0
}
}
