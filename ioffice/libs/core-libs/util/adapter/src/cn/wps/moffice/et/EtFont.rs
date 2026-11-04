// Java field: short BOLDWEIGHT_NORMAL
pub(crate) const BOLDWEIGHT_NORMAL: i16 = 400;
// Java field: short BOLDWEIGHT_BOLD
pub(crate) const BOLDWEIGHT_BOLD: i16 = 700;
// Java field: short COLOR_NORMAL
pub(crate) const COLOR_NORMAL: i16 = 0x7FFF;
// Java field: short COLOR_RED
pub(crate) const COLOR_RED: i16 = 0x0A;
// Java field: short SS_NONE
pub(crate) const SS_NONE: i16 = 0;
// Java field: short SS_SUPER
pub(crate) const SS_SUPER: i16 = 1;
// Java field: short SS_SUB
pub(crate) const SS_SUB: i16 = 2;
// Java field: byte U_NONE
pub(crate) const U_NONE: i8 = 0;
// Java field: byte U_SINGLE
pub(crate) const U_SINGLE: i8 = 1;
// Java field: byte U_DOUBLE
pub(crate) const U_DOUBLE: i8 = 2;
// Java field: byte U_SINGLE_ACCOUNTING
pub(crate) const U_SINGLE_ACCOUNTING: i8 = 0x21;
// Java field: byte U_DOUBLE_ACCOUNTING
pub(crate) const U_DOUBLE_ACCOUNTING: i8 = 0x22;
// Java field: byte ANSI_CHARSET
pub(crate) const ANSI_CHARSET: i8 = 0;
// Java field: byte DEFAULT_CHARSET
pub(crate) const DEFAULT_CHARSET: i8 = 1;
// Java field: byte SYMBOL_CHARSET
pub(crate) const SYMBOL_CHARSET: i8 = 2;

// Java: public class EtFont
pub struct EtFont {
    // Java field: short field_1_font_height
    pub(crate) field_1_font_height: i16,
    // Java field: short field_2_attributes
    pub(crate) field_2_attributes: i16,
    // Java field: boolean isItalic
    pub(crate) isItalic: bool,
    // Java field: boolean isStrikeout
    pub(crate) isStrikeout: bool,
    // Java field: boolean isMacoutline
    pub(crate) isMacoutline: bool,
    // Java field: boolean isMacshadow
    pub(crate) isMacshadow: bool,
    // Java field: boolean isFcondense
    pub(crate) isFcondense: bool,
    // Java field: boolean isFextend
    pub(crate) isFextend: bool,
    // Java field: boolean hasInit
    pub(crate) hasInit: bool,
    // Java field: int field_3_color_palette_index
    pub(crate) field_3_color_palette_index: i32,
    // Java field: short field_4_bold_weight
    pub(crate) field_4_bold_weight: i16,
    // Java field: short field_5_super_sub_script
    pub(crate) field_5_super_sub_script: i16,
    // Java field: byte field_6_underline
    pub(crate) field_6_underline: i8,
    // Java field: byte field_7_family
    pub(crate) field_7_family: i8,
    // Java field: byte field_8_charset
    pub(crate) field_8_charset: i8,
    // Java field: String field_11_font_name
    pub(crate) field_11_font_name: String,
}

// Java: public class EtFont
impl EtFont {
    // Java: EtFont()
    pub fn new() -> Self {
      // Java: super();
      EtFont {
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

}
