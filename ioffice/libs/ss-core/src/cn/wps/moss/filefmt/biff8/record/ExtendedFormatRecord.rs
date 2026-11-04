use crate::cn::wps::moss::filefmt::biff8::record::common::XFGradient::{GradientRef, XFGradient};
use util::org::apache::poi::util::BitField::BitField;
// java public final class ExtendedFormatRecord
#[derive(Clone, Debug)]
pub struct ExtendedFormatRecord {
    // java private short field_1_font_index;
    pub(crate) field_1_font_index: i16,
    // java private short field_2_format_index;
    pub(crate) field_2_format_index: i16,
    // java private short field_3_cell_options;
    pub(crate) field_3_cell_options: i16,
    // java private short field_4_alignment_options;
    pub(crate) field_4_alignment_options: i16,
    // java private short field_5_indention_options;
    pub(crate) field_5_indention_options: i16,
    // java private short field_6_border_options;
    pub(crate) field_6_border_options: i16,
    // java private int border_left_color;
    pub(crate) border_left_color: i32,
    // java private int border_right_color;
    pub(crate) border_right_color: i32,
    // java private byte border_diag_flag;
    pub(crate) border_diag_flag: i8,
    // java private int field_8_adtl_palette_options;
    pub(crate) field_8_adtl_palette_options: i32,
    // java private int border_top_color;
    pub(crate) border_top_color: i32,
    // java private int border_bottom_color;
    pub(crate) border_bottom_color: i32,
    // java private int border_diag_color;
    pub(crate) border_diag_color: i32,
    // java private int fill_fore_color;
    pub(crate) fill_fore_color: i32,
    // java private int fill_back_color;
    pub(crate) fill_back_color: i32,
    // java private XFGradient xfGradient;
    pub(crate) xfGradient: Option<GradientRef>,
}
impl Default for ExtendedFormatRecord {
    fn default() -> Self {
        Self {
            field_1_font_index: 0,
            field_2_format_index: 0,
            field_3_cell_options: 0,
            field_4_alignment_options: 0,
            field_5_indention_options: 0,
            field_6_border_options: 0,
            border_left_color: 0,
            border_right_color: 0,
            border_diag_flag: 0,
            field_8_adtl_palette_options: 0,
            border_top_color: 0,
            border_bottom_color: 0,
            border_diag_color: 0,
            fill_fore_color: 0,
            fill_back_color: 0,
            xfGradient: None,
        }
    }
}
impl ExtendedFormatRecord {
    // java public ExtendedFormatRecord() { }

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:122
    // java public ExtendedFormatRecord() {
    // java }
    pub fn ExtendedFormatRecord() -> Self {
        Self::default()
    }
    // java public void setFontIndex(short index) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:133
    // java public void setFontIndex(short index) {
    // java this.field_1_font_index = index;
    // java }
    pub fn setFontIndex(&mut self, index: i16) {
        // java this.field_1_font_index = index;
        self.field_1_font_index = index;
    }
    // java public void setFormatIndex(short index) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:137
    // java public void setFormatIndex(short index) {
    // java this.field_2_format_index = index;
    // java }
    pub fn setFormatIndex(&mut self, index: i16) {
        // java this.field_2_format_index = index;
        self.field_2_format_index = index;
    }
    // java public void setCellOptions(short options) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:141
    // java public void setCellOptions(short options) {
    // java this.field_3_cell_options = options;
    // java }
    pub fn setCellOptions(&mut self, options: i16) {
        // java this.field_3_cell_options = options;
        self.field_3_cell_options = options;
    }
    // java public void setLocked(boolean locked) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:145
    // java public void setLocked(boolean locked) {
    // java this.field_3_cell_options = _locked.setShortBoolean(this.field_3_cell_options, locked);
    // java }
    pub fn setLocked(&mut self, locked: bool) {
        // java this.field_3_cell_options = _locked.setShortBoolean(this.field_3_cell_options, locked);
        self.field_3_cell_options =
            (((self.field_3_cell_options as i32) & !1) | if locked { 1 } else { 0 }) as i16;
    }
    // java public void setHidden(boolean hidden) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:149
    // java public void setHidden(boolean hidden) {
    // java this.field_3_cell_options = _hidden.setShortBoolean(this.field_3_cell_options, hidden);
    // java }
    pub fn setHidden(&mut self, hidden: bool) {
        // java this.field_3_cell_options = _hidden.setShortBoolean(this.field_3_cell_options, hidden);
        self.field_3_cell_options =
            (((self.field_3_cell_options as i32) & !2) | if hidden { 2 } else { 0 }) as i16;
    }
    // java public void setXFType(short type) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:153
    // java public void setXFType(short type) {
    // java this.field_3_cell_options = _xf_type.setShortValue(this.field_3_cell_options, type);
    // java }
    pub fn setXFType(&mut self, r#type: i16) {
        // java this.field_3_cell_options = _xf_type.setShortValue(this.field_3_cell_options, type);
        self.field_3_cell_options = (((self.field_3_cell_options as i32) & !4)
            | (((r#type) as i32).wrapping_shl(2) & 4)) as i16;
    }
    // java public void set123Prefix(boolean prefix) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:157
    // java public void set123Prefix(boolean prefix) {
    // java this.field_3_cell_options = _123_prefix.setShortBoolean(this.field_3_cell_options, prefix);
    // java }
    pub fn set123Prefix(&mut self, prefix: bool) {
        // java this.field_3_cell_options = _123_prefix.setShortBoolean(this.field_3_cell_options, prefix);
        self.field_3_cell_options =
            (((self.field_3_cell_options as i32) & !8) | if prefix { 8 } else { 0 }) as i16;
    }
    // java public void setParentIndex(short parent) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:161
    // java public void setParentIndex(short parent) {
    // java this.field_3_cell_options = _parent_index.setShortValue(this.field_3_cell_options, parent);
    // java }
    pub fn setParentIndex(&mut self, parent: i16) {
        // java this.field_3_cell_options = _parent_index.setShortValue(this.field_3_cell_options, parent);
        self.field_3_cell_options = (((self.field_3_cell_options as i32) & !65520)
            | (((parent) as i32).wrapping_shl(4) & 65520))
            as i16;
    }
    // java public void setAlignmentOptions(short options) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:165
    // java public void setAlignmentOptions(short options) {
    // java this.field_4_alignment_options = options;
    // java }
    pub fn setAlignmentOptions(&mut self, options: i16) {
        // java this.field_4_alignment_options = options;
        self.field_4_alignment_options = options;
    }
    // java public void setAlignment(short align) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:169
    // java public void setAlignment(short align) {
    // java this.field_4_alignment_options = _alignment.setShortValue(this.field_4_alignment_options, align);
    // java }
    pub fn setAlignment(&mut self, align: i16) {
        // java this.field_4_alignment_options = _alignment.setShortValue(this.field_4_alignment_options, align);
        self.field_4_alignment_options = (((self.field_4_alignment_options as i32) & !7)
            | (((align) as i32).wrapping_shl(0) & 7))
            as i16;
    }
    // java public void setWrapText(boolean wrapped) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:173
    // java public void setWrapText(boolean wrapped) {
    // java this.field_4_alignment_options = _wrap_text.setShortBoolean(this.field_4_alignment_options, wrapped);
    // java }
    pub fn setWrapText(&mut self, wrapped: bool) {
        // java this.field_4_alignment_options = _wrap_text.setShortBoolean(this.field_4_alignment_options, wrapped);
        self.field_4_alignment_options =
            (((self.field_4_alignment_options as i32) & !8) | if wrapped { 8 } else { 0 }) as i16;
    }
    // java public void setVerticalAlignment(short align) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:177
    // java public void setVerticalAlignment(short align) {
    // java this.field_4_alignment_options = _vertical_alignment.setShortValue(this.field_4_alignment_options, align);
    // java }
    pub fn setVerticalAlignment(&mut self, align: i16) {
        // java this.field_4_alignment_options = _vertical_alignment.setShortValue(this.field_4_alignment_options, align);
        self.field_4_alignment_options = (((self.field_4_alignment_options as i32) & !112)
            | (((align) as i32).wrapping_shl(4) & 112))
            as i16;
    }
    // java public void setJustifyLast(short justify) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:181
    // java public void setJustifyLast(short justify) {
    // java this.field_4_alignment_options = _justify_last.setShortValue(this.field_4_alignment_options, justify);
    // java }
    pub fn setJustifyLast(&mut self, justify: i16) {
        // java this.field_4_alignment_options = _justify_last.setShortValue(this.field_4_alignment_options, justify);
        self.field_4_alignment_options = (((self.field_4_alignment_options as i32) & !128)
            | (((justify) as i32).wrapping_shl(7) & 128))
            as i16;
    }
    // java public void setRotation(short rotation) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:185
    // java public void setRotation(short rotation) {
    // java this.field_4_alignment_options = _rotation.setShortValue(this.field_4_alignment_options, rotation);
    // java }
    pub fn setRotation(&mut self, rotation: i16) {
        // java this.field_4_alignment_options = _rotation.setShortValue(this.field_4_alignment_options, rotation);
        self.field_4_alignment_options = (((self.field_4_alignment_options as i32) & !65280)
            | (((rotation) as i32).wrapping_shl(8) & 65280))
            as i16;
    }
    // java public void setIndentionOptions(short options) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:189
    // java public void setIndentionOptions(short options) {
    // java this.field_5_indention_options = options;
    // java }
    pub fn setIndentionOptions(&mut self, options: i16) {
        // java this.field_5_indention_options = options;
        self.field_5_indention_options = options;
    }
    // java public void setIndent(short indent) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:193
    // java public void setIndent(short indent) {
    // java this.field_5_indention_options = _indent.setShortValue(this.field_5_indention_options, indent);
    // java }
    pub fn setIndent(&mut self, indent: i16) {
        // java this.field_5_indention_options = _indent.setShortValue(this.field_5_indention_options, indent);
        self.field_5_indention_options = (((self.field_5_indention_options as i32) & !15)
            | (((indent) as i32).wrapping_shl(0) & 15))
            as i16;
    }
    // java public void setShrinkToFit(boolean shrink) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:197
    // java public void setShrinkToFit(boolean shrink) {
    // java this.field_5_indention_options = _shrink_to_fit.setShortBoolean(this.field_5_indention_options, shrink);
    // java }
    pub fn setShrinkToFit(&mut self, shrink: bool) {
        // java this.field_5_indention_options = _shrink_to_fit.setShortBoolean(this.field_5_indention_options, shrink);
        self.field_5_indention_options =
            (((self.field_5_indention_options as i32) & !16) | if shrink { 16 } else { 0 }) as i16;
    }
    // java public void setMergeCells(boolean merge) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:201
    // java public void setMergeCells(boolean merge) {
    // java this.field_5_indention_options = _merge_cells.setShortBoolean(this.field_5_indention_options, merge);
    // java }
    pub fn setMergeCells(&mut self, merge: bool) {
        // java this.field_5_indention_options = _merge_cells.setShortBoolean(this.field_5_indention_options, merge);
        self.field_5_indention_options =
            (((self.field_5_indention_options as i32) & !32) | if merge { 32 } else { 0 }) as i16;
    }
    // java public void setReadingOrder(short order) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:205
    // java public void setReadingOrder(short order) {
    // java this.field_5_indention_options = _reading_order.setShortValue(this.field_5_indention_options, order);
    // java }
    pub fn setReadingOrder(&mut self, order: i16) {
        // java this.field_5_indention_options = _reading_order.setShortValue(this.field_5_indention_options, order);
        self.field_5_indention_options = (((self.field_5_indention_options as i32) & !192)
            | (((order) as i32).wrapping_shl(6) & 192))
            as i16;
    }
    // java public void setAttrFormat(boolean attr) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:209
    // java public void setAttrFormat(boolean attr) {
    // java this.field_5_indention_options = _attr_format.setShortBoolean(this.field_5_indention_options, attr);
    // java }
    pub fn setAttrFormat(&mut self, attr: bool) {
        // java this.field_5_indention_options = _attr_format.setShortBoolean(this.field_5_indention_options, attr);
        self.field_5_indention_options = (((self.field_5_indention_options as i32) & !1024)
            | if attr { 1024 } else { 0 }) as i16;
    }
    // java public void setValidFormat(boolean valid) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:213
    // java public void setValidFormat(boolean valid) {
    // java boolean isStyle = this.getXFType() == 1;
    // java this.field_5_indention_options = _attr_format.setShortBoolean(this.field_5_indention_options, isStyle ? !valid : valid);
    // java }
    pub fn setValidFormat(&mut self, valid: bool) {
        // java boolean isStyle = this.getXFType() == 1;
        let isStyle = self.getXFType() == 1;
        // java this.field_5_indention_options = _attr_format.setShortBoolean(this.field_5_indention_options, isStyle ? !valid : valid);
        self.field_5_indention_options = (((self.field_5_indention_options as i32) & !1024)
            | if if isStyle { !valid } else { valid } {
                1024
            } else {
                0
            }) as i16;
    }
    // java public void setAttrFont(boolean attr) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:218
    // java public void setAttrFont(boolean attr) {
    // java this.field_5_indention_options = _attr_font.setShortBoolean(this.field_5_indention_options, attr);
    // java }
    pub fn setAttrFont(&mut self, attr: bool) {
        // java this.field_5_indention_options = _attr_font.setShortBoolean(this.field_5_indention_options, attr);
        self.field_5_indention_options = (((self.field_5_indention_options as i32) & !2048)
            | if attr { 2048 } else { 0 }) as i16;
    }
    // java public void setValidFont(boolean valid) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:222
    // java public void setValidFont(boolean valid) {
    // java boolean isStyle = this.getXFType() == 1;
    // java this.field_5_indention_options = _attr_font.setShortBoolean(this.field_5_indention_options, isStyle ? !valid : valid);
    // java }
    pub fn setValidFont(&mut self, valid: bool) {
        // java boolean isStyle = this.getXFType() == 1;
        let isStyle = self.getXFType() == 1;
        // java this.field_5_indention_options = _attr_font.setShortBoolean(this.field_5_indention_options, isStyle ? !valid : valid);
        self.field_5_indention_options = (((self.field_5_indention_options as i32) & !2048)
            | if if isStyle { !valid } else { valid } {
                2048
            } else {
                0
            }) as i16;
    }
    // java public void setAttrAlignment(boolean attr) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:227
    // java public void setAttrAlignment(boolean attr) {
    // java this.field_5_indention_options = _attr_alignment.setShortBoolean(this.field_5_indention_options, attr);
    // java }
    pub fn setAttrAlignment(&mut self, attr: bool) {
        // java this.field_5_indention_options = _attr_alignment.setShortBoolean(this.field_5_indention_options, attr);
        self.field_5_indention_options = (((self.field_5_indention_options as i32) & !4096)
            | if attr { 4096 } else { 0 }) as i16;
    }
    // java public void setValidAlignment(boolean valid) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:231
    // java public void setValidAlignment(boolean valid) {
    // java boolean isStyle = this.getXFType() == 1;
    // java this.field_5_indention_options = _attr_alignment.setShortBoolean(this.field_5_indention_options, isStyle ? !valid : valid);
    // java }
    pub fn setValidAlignment(&mut self, valid: bool) {
        // java boolean isStyle = this.getXFType() == 1;
        let isStyle = self.getXFType() == 1;
        // java this.field_5_indention_options = _attr_alignment.setShortBoolean(this.field_5_indention_options, isStyle ? !valid : valid);
        self.field_5_indention_options = (((self.field_5_indention_options as i32) & !4096)
            | if if isStyle { !valid } else { valid } {
                4096
            } else {
                0
            }) as i16;
    }
    // java public void setAttrBorder(boolean attr) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:236
    // java public void setAttrBorder(boolean attr) {
    // java this.field_5_indention_options = _attr_border.setShortBoolean(this.field_5_indention_options, attr);
    // java }
    pub fn setAttrBorder(&mut self, attr: bool) {
        // java this.field_5_indention_options = _attr_border.setShortBoolean(this.field_5_indention_options, attr);
        self.field_5_indention_options = (((self.field_5_indention_options as i32) & !8192)
            | if attr { 8192 } else { 0 }) as i16;
    }
    // java public void setValidBorder(boolean valid) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:240
    // java public void setValidBorder(boolean valid) {
    // java boolean isStyle = this.getXFType() == 1;
    // java this.field_5_indention_options = _attr_border.setShortBoolean(this.field_5_indention_options, isStyle ? !valid : valid);
    // java }
    pub fn setValidBorder(&mut self, valid: bool) {
        // java boolean isStyle = this.getXFType() == 1;
        let isStyle = self.getXFType() == 1;
        // java this.field_5_indention_options = _attr_border.setShortBoolean(this.field_5_indention_options, isStyle ? !valid : valid);
        self.field_5_indention_options = (((self.field_5_indention_options as i32) & !8192)
            | if if isStyle { !valid } else { valid } {
                8192
            } else {
                0
            }) as i16;
    }
    // java public void setAttrPattern(boolean attr) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:245
    // java public void setAttrPattern(boolean attr) {
    // java this.field_5_indention_options = _attr_pattern.setShortBoolean(this.field_5_indention_options, attr);
    // java }
    pub fn setAttrPattern(&mut self, attr: bool) {
        // java this.field_5_indention_options = _attr_pattern.setShortBoolean(this.field_5_indention_options, attr);
        self.field_5_indention_options = (((self.field_5_indention_options as i32) & !16384)
            | if attr { 16384 } else { 0 }) as i16;
    }
    // java public void setValidPattern(boolean valid) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:249
    // java public void setValidPattern(boolean valid) {
    // java boolean isStyle = this.getXFType() == 1;
    // java this.field_5_indention_options = _attr_pattern.setShortBoolean(this.field_5_indention_options, isStyle ? !valid : valid);
    // java }
    pub fn setValidPattern(&mut self, valid: bool) {
        // java boolean isStyle = this.getXFType() == 1;
        let isStyle = self.getXFType() == 1;
        // java this.field_5_indention_options = _attr_pattern.setShortBoolean(this.field_5_indention_options, isStyle ? !valid : valid);
        self.field_5_indention_options = (((self.field_5_indention_options as i32) & !16384)
            | if if isStyle { !valid } else { valid } {
                16384
            } else {
                0
            }) as i16;
    }
    // java public void setAttrCellOptions(boolean attr) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:254
    // java public void setAttrCellOptions(boolean attr) {
    // java this.field_5_indention_options = _attr_cell_options.setShortBoolean(this.field_5_indention_options, attr);
    // java }
    pub fn setAttrCellOptions(&mut self, attr: bool) {
        // java this.field_5_indention_options = _attr_cell_options.setShortBoolean(this.field_5_indention_options, attr);
        self.field_5_indention_options = (((self.field_5_indention_options as i32) & !32768)
            | if attr { 32768 } else { 0 }) as i16;
    }
    // java public void setValidCellOptions(boolean valid) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:258
    // java public void setValidCellOptions(boolean valid) {
    // java boolean isStyle = this.getXFType() == 1;
    // java this.field_5_indention_options = _attr_cell_options.setShortBoolean(this.field_5_indention_options, isStyle ? !valid : valid);
    // java }
    pub fn setValidCellOptions(&mut self, valid: bool) {
        // java boolean isStyle = this.getXFType() == 1;
        let isStyle = self.getXFType() == 1;
        // java this.field_5_indention_options = _attr_cell_options.setShortBoolean(this.field_5_indention_options, isStyle ? !valid : valid);
        self.field_5_indention_options = (((self.field_5_indention_options as i32) & !32768)
            | if if isStyle { !valid } else { valid } {
                32768
            } else {
                0
            }) as i16;
    }
    // java public void setBorderOptions(short options) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:263
    // java public void setBorderOptions(short options) {
    // java this.field_6_border_options = options;
    // java }
    pub fn setBorderOptions(&mut self, options: i16) {
        // java this.field_6_border_options = options;
        self.field_6_border_options = options;
    }
    // java public void setBorderLeft(short border) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:267
    // java public void setBorderLeft(short border) {
    // java this.field_6_border_options = _border_left.setShortValue(this.field_6_border_options, border);
    // java }
    pub fn setBorderLeft(&mut self, border: i16) {
        // java this.field_6_border_options = _border_left.setShortValue(this.field_6_border_options, border);
        self.field_6_border_options = (((self.field_6_border_options as i32) & !15)
            | (((border) as i32).wrapping_shl(0) & 15))
            as i16;
    }
    // java public void setBorderRight(short border) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:271
    // java public void setBorderRight(short border) {
    // java this.field_6_border_options = _border_right.setShortValue(this.field_6_border_options, border);
    // java }
    pub fn setBorderRight(&mut self, border: i16) {
        // java this.field_6_border_options = _border_right.setShortValue(this.field_6_border_options, border);
        self.field_6_border_options = (((self.field_6_border_options as i32) & !240)
            | (((border) as i32).wrapping_shl(4) & 240))
            as i16;
    }
    // java public void setBorderTop(short border) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:275
    // java public void setBorderTop(short border) {
    // java this.field_6_border_options = _border_top.setShortValue(this.field_6_border_options, border);
    // java }
    pub fn setBorderTop(&mut self, border: i16) {
        // java this.field_6_border_options = _border_top.setShortValue(this.field_6_border_options, border);
        self.field_6_border_options = (((self.field_6_border_options as i32) & !3840)
            | (((border) as i32).wrapping_shl(8) & 3840))
            as i16;
    }
    // java public void setBorderBottom(short border) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:279
    // java public void setBorderBottom(short border) {
    // java this.field_6_border_options = _border_bottom.setShortValue(this.field_6_border_options, border);
    // java }
    pub fn setBorderBottom(&mut self, border: i16) {
        // java this.field_6_border_options = _border_bottom.setShortValue(this.field_6_border_options, border);
        self.field_6_border_options = (((self.field_6_border_options as i32) & !61440)
            | (((border) as i32).wrapping_shl(12) & 61440))
            as i16;
    }
    // java public void setPaletteOptions(short options) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:283
    // java public void setPaletteOptions(short options) {
    // java this.border_left_color = _left_border_palette_idx.getShortValue(options);
    // java this.border_right_color = _right_border_palette_idx.getShortValue(options);
    // java this.border_diag_flag = (byte)_diag.getShortValue(options);
    // java }
    pub fn setPaletteOptions(&mut self, options: i16) {
        // java this.border_left_color = _left_border_palette_idx.getShortValue(options);
        self.border_left_color = (((options as i32) as u32 & (127i32 as u32)) >> 0) as i16 as i32;
        // java this.border_right_color = _right_border_palette_idx.getShortValue(options);
        self.border_right_color =
            (((options as i32) as u32 & (16256i32 as u32)) >> 7) as i16 as i32;
        // java this.border_diag_flag = (byte)_diag.getShortValue(options);
        self.border_diag_flag =
            ((((options as i32) as u32 & (49152i32 as u32)) >> 14) as i16) as i8;
    }
    // java public void setLeftBorderPaletteIdx(int border) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:289
    // java public void setLeftBorderPaletteIdx(int border) {
    // java this.border_left_color = border;
    // java }
    pub fn setLeftBorderPaletteIdx(&mut self, border: i32) {
        // java this.border_left_color = border;
        self.border_left_color = border;
    }
    // java public void setRightBorderPaletteIdx(int border) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:293
    // java public void setRightBorderPaletteIdx(int border) {
    // java this.border_right_color = border;
    // java }
    pub fn setRightBorderPaletteIdx(&mut self, border: i32) {
        // java this.border_right_color = border;
        self.border_right_color = border;
    }
    // java public void setDiag(byte diag) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:297
    // java public void setDiag(byte diag) {
    // java this.border_diag_flag = diag;
    // java }
    pub fn setDiag(&mut self, diag: i8) {
        // java this.border_diag_flag = diag;
        self.border_diag_flag = diag;
    }
    // java public void setAdtlPaletteOptions(int options) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:301
    // java public void setAdtlPaletteOptions(int options) {
    // java this.field_8_adtl_palette_options = options;
    // java this.border_top_color = _top_border_palette_idx.getValue(this.field_8_adtl_palette_options);
    // java this.border_bottom_color = _bottom_border_palette_idx.getValue(this.field_8_adtl_palette_options);
    // java this.border_diag_color = _adtl_diag.getValue(this.field_8_adtl_palette_options);
    // java }
    pub fn setAdtlPaletteOptions(&mut self, options: i32) {
        // java this.field_8_adtl_palette_options = options;
        self.field_8_adtl_palette_options = options;
        // java this.border_top_color = _top_border_palette_idx.getValue(this.field_8_adtl_palette_options);
        self.border_top_color =
            (((self.field_8_adtl_palette_options as i32) as u32 & (127i32 as u32)) >> 0) as i32;
        // java this.border_bottom_color = _bottom_border_palette_idx.getValue(this.field_8_adtl_palette_options);
        self.border_bottom_color =
            (((self.field_8_adtl_palette_options as i32) as u32 & (16256i32 as u32)) >> 7) as i32;
        // java this.border_diag_color = _adtl_diag.getValue(this.field_8_adtl_palette_options);
        self.border_diag_color = (((self.field_8_adtl_palette_options as i32) as u32
            & (2080768i32 as u32))
            >> 14) as i32;
    }
    // java public void setTopBorderPaletteIdx(int border) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:308
    // java public void setTopBorderPaletteIdx(int border) {
    // java this.border_top_color = border;
    // java }
    pub fn setTopBorderPaletteIdx(&mut self, border: i32) {
        // java this.border_top_color = border;
        self.border_top_color = border;
    }
    // java public void setBottomBorderPaletteIdx(int border) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:312
    // java public void setBottomBorderPaletteIdx(int border) {
    // java this.border_bottom_color = border;
    // java }
    pub fn setBottomBorderPaletteIdx(&mut self, border: i32) {
        // java this.border_bottom_color = border;
        self.border_bottom_color = border;
    }
    // java public void setAdtlDiag(int diag) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:316
    // java public void setAdtlDiag(int diag) {
    // java this.border_diag_color = diag;
    // java }
    pub fn setAdtlDiag(&mut self, diag: i32) {
        // java this.border_diag_color = diag;
        self.border_diag_color = diag;
    }
    // java public void setAdtlDiagLineStyle(short diag) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:320
    // java public void setAdtlDiagLineStyle(short diag) {
    // java this.field_8_adtl_palette_options = _adtl_diag_line_style.setValue(this.field_8_adtl_palette_options, diag);
    // java }
    pub fn setAdtlDiagLineStyle(&mut self, diag: i16) {
        // java this.field_8_adtl_palette_options = _adtl_diag_line_style.setValue(this.field_8_adtl_palette_options, diag);
        self.field_8_adtl_palette_options =
            (((self.field_8_adtl_palette_options as i32) & !31457280)
                | (((diag) as i32).wrapping_shl(21) & 31457280)) as i32;
    }
    // java public void setAdtlFillPattern(short fill) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:324
    // java public void setAdtlFillPattern(short fill) {
    // java this.field_8_adtl_palette_options = _adtl_fill_pattern.setValue(this.field_8_adtl_palette_options, fill);
    // java }
    pub fn setAdtlFillPattern(&mut self, fill: i16) {
        // java this.field_8_adtl_palette_options = _adtl_fill_pattern.setValue(this.field_8_adtl_palette_options, fill);
        self.field_8_adtl_palette_options =
            (((self.field_8_adtl_palette_options as i32) & !-67108864)
                | (((fill) as i32).wrapping_shl(26) & -67108864)) as i32;
    }
    // java public void setFillPaletteOptions(short options) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:328
    // java public void setFillPaletteOptions(short options) {
    // java this.fill_fore_color = _fill_foreground.getShortValue(options);
    // java this.fill_back_color = _fill_background.getShortValue(options);
    // java }
    pub fn setFillPaletteOptions(&mut self, options: i16) {
        // java this.fill_fore_color = _fill_foreground.getShortValue(options);
        self.fill_fore_color = (((options as i32) as u32 & (127i32 as u32)) >> 0) as i16 as i32;
        // java this.fill_back_color = _fill_background.getShortValue(options);
        self.fill_back_color = (((options as i32) as u32 & (16256i32 as u32)) >> 7) as i16 as i32;
    }
    // java public void setFillForeground(int color) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:333
    // java public void setFillForeground(int color) {
    // java this.fill_fore_color = color;
    // java }
    pub fn setFillForeground(&mut self, color: i32) {
        // java this.fill_fore_color = color;
        self.fill_fore_color = color;
    }
    // java public void setFillBackground(int color) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:337
    // java public void setFillBackground(int color) {
    // java this.fill_back_color = color;
    // java }
    pub fn setFillBackground(&mut self, color: i32) {
        // java this.fill_back_color = color;
        self.fill_back_color = color;
    }
    // java public short getFontIndex() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:341
    // java public short getFontIndex() {
    // java return this.field_1_font_index;
    // java }
    pub fn getFontIndex(&self) -> i16 {
        // java return this.field_1_font_index;
        (self.field_1_font_index) as i16
    }
    // java public short getFormatIndex() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:345
    // java public short getFormatIndex() {
    // java return this.field_2_format_index;
    // java }
    pub fn getFormatIndex(&self) -> i16 {
        // java return this.field_2_format_index;
        (self.field_2_format_index) as i16
    }
    // java public short getCellOptions() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:349
    // java public short getCellOptions() {
    // java return this.field_3_cell_options;
    // java }
    pub fn getCellOptions(&self) -> i16 {
        // java return this.field_3_cell_options;
        (self.field_3_cell_options) as i16
    }
    // java public boolean isLocked() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:353
    // java public boolean isLocked() {
    // java return _locked.isSet(this.field_3_cell_options);
    // java }
    pub fn isLocked(&self) -> bool {
        // java return _locked.isSet(this.field_3_cell_options);
        ((self.field_3_cell_options as i32) & 1) != 0
    }
    // java public boolean isHidden() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:357
    // java public boolean isHidden() {
    // java return _hidden.isSet(this.field_3_cell_options);
    // java }
    pub fn isHidden(&self) -> bool {
        // java return _hidden.isSet(this.field_3_cell_options);
        ((self.field_3_cell_options as i32) & 2) != 0
    }
    // java public short getXFType() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:361
    // java public short getXFType() {
    // java return _xf_type.getShortValue(this.field_3_cell_options);
    // java }
    pub fn getXFType(&self) -> i16 {
        // java return _xf_type.getShortValue(this.field_3_cell_options);
        ((((self.field_3_cell_options as i32) as u32 & (4i32 as u32)) >> 2) as i16) as i16
    }
    // java public boolean get123Prefix() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:365
    // java public boolean get123Prefix() {
    // java return _123_prefix.isSet(this.field_3_cell_options);
    // java }
    pub fn get123Prefix(&self) -> bool {
        // java return _123_prefix.isSet(this.field_3_cell_options);
        ((self.field_3_cell_options as i32) & 8) != 0
    }
    // java public short getParentIndex() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:369
    // java public short getParentIndex() {
    // java return _parent_index.getShortValue(this.field_3_cell_options);
    // java }
    pub fn getParentIndex(&self) -> i16 {
        // java return _parent_index.getShortValue(this.field_3_cell_options);
        ((((self.field_3_cell_options as i32) as u32 & (65520i32 as u32)) >> 4) as i16) as i16
    }
    // java public short getAlignmentOptions() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:373
    // java public short getAlignmentOptions() {
    // java return this.field_4_alignment_options;
    // java }
    pub fn getAlignmentOptions(&self) -> i16 {
        // java return this.field_4_alignment_options;
        (self.field_4_alignment_options) as i16
    }
    // java public short getAlignment() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:377
    // java public short getAlignment() {
    // java return _alignment.getShortValue(this.field_4_alignment_options);
    // java }
    pub fn getAlignment(&self) -> i16 {
        // java return _alignment.getShortValue(this.field_4_alignment_options);
        ((((self.field_4_alignment_options as i32) as u32 & (7i32 as u32)) >> 0) as i16) as i16
    }
    // java public boolean getWrapText() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:381
    // java public boolean getWrapText() {
    // java return _wrap_text.isSet(this.field_4_alignment_options);
    // java }
    pub fn getWrapText(&self) -> bool {
        // java return _wrap_text.isSet(this.field_4_alignment_options);
        ((self.field_4_alignment_options as i32) & 8) != 0
    }
    // java public short getVerticalAlignment() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:385
    // java public short getVerticalAlignment() {
    // java return _vertical_alignment.getShortValue(this.field_4_alignment_options);
    // java }
    pub fn getVerticalAlignment(&self) -> i16 {
        // java return _vertical_alignment.getShortValue(this.field_4_alignment_options);
        ((((self.field_4_alignment_options as i32) as u32 & (112i32 as u32)) >> 4) as i16) as i16
    }
    // java public short getJustifyLast() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:389
    // java public short getJustifyLast() {
    // java return _justify_last.getShortValue(this.field_4_alignment_options);
    // java }
    pub fn getJustifyLast(&self) -> i16 {
        // java return _justify_last.getShortValue(this.field_4_alignment_options);
        ((((self.field_4_alignment_options as i32) as u32 & (128i32 as u32)) >> 7) as i16) as i16
    }
    // java public short getRotation() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:393
    // java public short getRotation() {
    // java return _rotation.getShortValue(this.field_4_alignment_options);
    // java }
    pub fn getRotation(&self) -> i16 {
        // java return _rotation.getShortValue(this.field_4_alignment_options);
        ((((self.field_4_alignment_options as i32) as u32 & (65280i32 as u32)) >> 8) as i16) as i16
    }
    // java public short getIndentionOptions() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:397
    // java public short getIndentionOptions() {
    // java return this.field_5_indention_options;
    // java }
    pub fn getIndentionOptions(&self) -> i16 {
        // java return this.field_5_indention_options;
        (self.field_5_indention_options) as i16
    }
    // java public short getIndent() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:401
    // java public short getIndent() {
    // java return _indent.getShortValue(this.field_5_indention_options);
    // java }
    pub fn getIndent(&self) -> i16 {
        // java return _indent.getShortValue(this.field_5_indention_options);
        ((((self.field_5_indention_options as i32) as u32 & (15i32 as u32)) >> 0) as i16) as i16
    }
    // java public boolean getShrinkToFit() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:405
    // java public boolean getShrinkToFit() {
    // java return _shrink_to_fit.isSet(this.field_5_indention_options);
    // java }
    pub fn getShrinkToFit(&self) -> bool {
        // java return _shrink_to_fit.isSet(this.field_5_indention_options);
        ((self.field_5_indention_options as i32) & 16) != 0
    }
    // java public boolean getMergeCells() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:409
    // java public boolean getMergeCells() {
    // java return _merge_cells.isSet(this.field_5_indention_options);
    // java }
    pub fn getMergeCells(&self) -> bool {
        // java return _merge_cells.isSet(this.field_5_indention_options);
        ((self.field_5_indention_options as i32) & 32) != 0
    }
    // java public short getReadingOrder() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:413
    // java public short getReadingOrder() {
    // java return _reading_order.getShortValue(this.field_5_indention_options);
    // java }
    pub fn getReadingOrder(&self) -> i16 {
        // java return _reading_order.getShortValue(this.field_5_indention_options);
        ((((self.field_5_indention_options as i32) as u32 & (192i32 as u32)) >> 6) as i16) as i16
    }
    // java public boolean getAttrFormat() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:417
    // java public boolean getAttrFormat() {
    // java return _attr_format.isSet(this.field_5_indention_options);
    // java }
    pub fn getAttrFormat(&self) -> bool {
        // java return _attr_format.isSet(this.field_5_indention_options);
        ((self.field_5_indention_options as i32) & 1024) != 0
    }
    // java public boolean getAttrFont() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:427
    // java public boolean getAttrFont() {
    // java return _attr_font.isSet(this.field_5_indention_options);
    // java }
    pub fn getAttrFont(&self) -> bool {
        // java return _attr_font.isSet(this.field_5_indention_options);
        ((self.field_5_indention_options as i32) & 2048) != 0
    }
    // java public boolean getAttrAllignment() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:437
    // java public boolean getAttrAllignment() {
    // java return _attr_alignment.isSet(this.field_5_indention_options);
    // java }
    pub fn getAttrAllignment(&self) -> bool {
        // java return _attr_alignment.isSet(this.field_5_indention_options);
        ((self.field_5_indention_options as i32) & 4096) != 0
    }
    // java public boolean getAttrBorder() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:447
    // java public boolean getAttrBorder() {
    // java return _attr_border.isSet(this.field_5_indention_options);
    // java }
    pub fn getAttrBorder(&self) -> bool {
        // java return _attr_border.isSet(this.field_5_indention_options);
        ((self.field_5_indention_options as i32) & 8192) != 0
    }
    // java public boolean getAttrPattern() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:457
    // java public boolean getAttrPattern() {
    // java return _attr_pattern.isSet(this.field_5_indention_options);
    // java }
    pub fn getAttrPattern(&self) -> bool {
        // java return _attr_pattern.isSet(this.field_5_indention_options);
        ((self.field_5_indention_options as i32) & 16384) != 0
    }
    // java public boolean getAttrCellOptions() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:467
    // java public boolean getAttrCellOptions() {
    // java return _attr_cell_options.isSet(this.field_5_indention_options);
    // java }
    pub fn getAttrCellOptions(&self) -> bool {
        // java return _attr_cell_options.isSet(this.field_5_indention_options);
        ((self.field_5_indention_options as i32) & 32768) != 0
    }
    // java public short getBorderOptions() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:477
    // java public short getBorderOptions() {
    // java return this.field_6_border_options;
    // java }
    pub fn getBorderOptions(&self) -> i16 {
        // java return this.field_6_border_options;
        (self.field_6_border_options) as i16
    }
    // java public short getBorderLeft() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:481
    // java public short getBorderLeft() {
    // java return _border_left.getShortValue(this.field_6_border_options);
    // java }
    pub fn getBorderLeft(&self) -> i16 {
        // java return _border_left.getShortValue(this.field_6_border_options);
        ((((self.field_6_border_options as i32) as u32 & (15i32 as u32)) >> 0) as i16) as i16
    }
    // java public short getBorderRight() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:485
    // java public short getBorderRight() {
    // java return _border_right.getShortValue(this.field_6_border_options);
    // java }
    pub fn getBorderRight(&self) -> i16 {
        // java return _border_right.getShortValue(this.field_6_border_options);
        ((((self.field_6_border_options as i32) as u32 & (240i32 as u32)) >> 4) as i16) as i16
    }
    // java public short getBorderTop() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:489
    // java public short getBorderTop() {
    // java return _border_top.getShortValue(this.field_6_border_options);
    // java }
    pub fn getBorderTop(&self) -> i16 {
        // java return _border_top.getShortValue(this.field_6_border_options);
        ((((self.field_6_border_options as i32) as u32 & (3840i32 as u32)) >> 8) as i16) as i16
    }
    // java public short getBorderBottom() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:493
    // java public short getBorderBottom() {
    // java return _border_bottom.getShortValue(this.field_6_border_options);
    // java }
    pub fn getBorderBottom(&self) -> i16 {
        // java return _border_bottom.getShortValue(this.field_6_border_options);
        ((((self.field_6_border_options as i32) as u32 & (61440i32 as u32)) >> 12) as i16) as i16
    }
    // java public int getLeftBorderPaletteIdx() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:497
    // java public int getLeftBorderPaletteIdx() {
    // java return this.border_left_color;
    // java }
    pub fn getLeftBorderPaletteIdx(&self) -> i32 {
        // java return this.border_left_color;
        (self.border_left_color) as i32
    }
    // java public int getRightBorderPaletteIdx() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:501
    // java public int getRightBorderPaletteIdx() {
    // java return this.border_right_color;
    // java }
    pub fn getRightBorderPaletteIdx(&self) -> i32 {
        // java return this.border_right_color;
        (self.border_right_color) as i32
    }
    // java public byte getDiag() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:505
    // java public byte getDiag() {
    // java return this.border_diag_flag;
    // java }
    pub fn getDiag(&self) -> i8 {
        // java return this.border_diag_flag;
        (self.border_diag_flag) as i8
    }
    // java public int getAdtlPaletteOptions() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:509
    // java public int getAdtlPaletteOptions() {
    // java return this.field_8_adtl_palette_options;
    // java }
    pub fn getAdtlPaletteOptions(&self) -> i32 {
        // java return this.field_8_adtl_palette_options;
        (self.field_8_adtl_palette_options) as i32
    }
    // java public int getTopBorderPaletteIdx() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:513
    // java public int getTopBorderPaletteIdx() {
    // java return this.border_top_color;
    // java }
    pub fn getTopBorderPaletteIdx(&self) -> i32 {
        // java return this.border_top_color;
        (self.border_top_color) as i32
    }
    // java public int getBottomBorderPaletteIdx() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:517
    // java public int getBottomBorderPaletteIdx() {
    // java return this.border_bottom_color;
    // java }
    pub fn getBottomBorderPaletteIdx(&self) -> i32 {
        // java return this.border_bottom_color;
        (self.border_bottom_color) as i32
    }
    // java public int getAdtlDiag() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:521
    // java public int getAdtlDiag() {
    // java return this.border_diag_color;
    // java }
    pub fn getAdtlDiag(&self) -> i32 {
        // java return this.border_diag_color;
        (self.border_diag_color) as i32
    }
    // java public short getAdtlDiagLineStyle() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:525
    // java public short getAdtlDiagLineStyle() {
    // java return (short)_adtl_diag_line_style.getValue(this.field_8_adtl_palette_options);
    // java }
    pub fn getAdtlDiagLineStyle(&self) -> i16 {
        // java return (short)_adtl_diag_line_style.getValue(this.field_8_adtl_palette_options);
        (((((self.field_8_adtl_palette_options as i32) as u32 & (31457280i32 as u32)) >> 21) as i32)
            as i16) as i16
    }
    // java public short getAdtlFillPattern() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:529
    // java public short getAdtlFillPattern() {
    // java return (short)_adtl_fill_pattern.getValue(this.field_8_adtl_palette_options);
    // java }
    pub fn getAdtlFillPattern(&self) -> i16 {
        // java return (short)_adtl_fill_pattern.getValue(this.field_8_adtl_palette_options);
        (((((self.field_8_adtl_palette_options as i32) as u32 & (-67108864i32 as u32)) >> 26)
            as i32) as i16) as i16
    }
    // java public int getFillForeground() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:533
    // java public int getFillForeground() {
    // java return this.fill_fore_color;
    // java }
    pub fn getFillForeground(&self) -> i32 {
        // java return this.fill_fore_color;
        (self.fill_fore_color) as i32
    }
    // java public int getFillBackground() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:537
    // java public int getFillBackground() {
    // java return this.fill_back_color;
    // java }
    pub fn getFillBackground(&self) -> i32 {
        // java return this.fill_back_color;
        (self.fill_back_color) as i32
    }
    // java public XFGradient getXFGradient() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:541
    // java public XFGradient getXFGradient() {
    // java return this.xfGradient;
    // java }
    pub fn getXFGradient(&self) -> Option<GradientRef> {
        // java return this.xfGradient;
        self.xfGradient.clone()
    }
    // java public void setXFGradient(XFGradient gradient) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:545
    // java public void setXFGradient(XFGradient gradient) {
    // java this.xfGradient = gradient;
    // java }
    pub fn setXFGradient(&mut self, gradient: Option<GradientRef>) {
        // java this.xfGradient = gradient;
        self.xfGradient = gradient.clone();
    }
    // java public boolean getXFExtFlag() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:964
    // java public boolean getXFExtFlag() {
    // java return _has_xf_ext.isSet(this.field_8_adtl_palette_options);
    // java }
    pub fn getXFExtFlag(&self) -> bool {
        // java return _has_xf_ext.isSet(this.field_8_adtl_palette_options);
        ((self.field_8_adtl_palette_options as i32) & 33554432) != 0
    }
    // java public void cloneStyleFrom(ExtendedFormatRecord source) {
    // java this.field_1_font_index = source.field_1_font_index;
    // java this.field_2_format_index = source.field_2_format_index;
    // java this.field_3_cell_options = source.field_3_cell_options;
    // java this.field_4_alignment_options = source.field_4_alignment_options;
    // java this.field_5_indention_options = source.field_5_indention_options;
    // java this.field_6_border_options = source.field_6_border_options;
    // java this.border_left_color = source.border_left_color;
    // java this.border_right_color = source.border_right_color;
    // java this.border_diag_flag = source.border_diag_flag;
    // java this.field_8_adtl_palette_options = source.field_8_adtl_palette_options;
    // java this.border_top_color = source.border_top_color;
    // java this.border_bottom_color = source.border_bottom_color;
    // java this.border_diag_color = source.border_diag_color;
    // java this.fill_fore_color = source.fill_fore_color;
    // java this.fill_back_color = source.fill_back_color;
    // java }
    pub fn cloneStyleFrom(&mut self, source: &Self) {
        self.field_1_font_index = source.field_1_font_index.clone();
        self.field_2_format_index = source.field_2_format_index.clone();
        self.field_3_cell_options = source.field_3_cell_options.clone();
        self.field_4_alignment_options = source.field_4_alignment_options.clone();
        self.field_5_indention_options = source.field_5_indention_options.clone();
        self.field_6_border_options = source.field_6_border_options.clone();
        self.border_left_color = source.border_left_color.clone();
        self.border_right_color = source.border_right_color.clone();
        self.border_diag_flag = source.border_diag_flag.clone();
        self.field_8_adtl_palette_options = source.field_8_adtl_palette_options.clone();
        self.border_top_color = source.border_top_color.clone();
        self.border_bottom_color = source.border_bottom_color.clone();
        self.border_diag_color = source.border_diag_color.clone();
        self.fill_fore_color = source.fill_fore_color.clone();
        self.fill_back_color = source.fill_back_color.clone();
    }
}

impl ExtendedFormatRecord {
    // java boolean attr = _attr_format.isSet(this.field_5_indention_options); boolean isStyle = this.getXFType() == 1; return isStyle ? !attr : attr;

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:421
    // java public boolean isValidFormat() {
    // java boolean attr = _attr_format.isSet(this.field_5_indention_options);
    // java boolean isStyle = this.getXFType() == 1;
    // java return isStyle ? !attr : attr;
    // java }
    pub fn isValidFormat(&self) -> bool {
        let attr = self.getAttrFormat();
        if self.getXFType() == 1 {
            !attr
        } else {
            attr
        }
    }
    // java boolean attr = _attr_font.isSet(this.field_5_indention_options); boolean isStyle = this.getXFType() == 1; return isStyle ? !attr : attr;

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:431
    // java public boolean isValidFont() {
    // java boolean attr = _attr_font.isSet(this.field_5_indention_options);
    // java boolean isStyle = this.getXFType() == 1;
    // java return isStyle ? !attr : attr;
    // java }
    pub fn isValidFont(&self) -> bool {
        let attr = self.getAttrFont();
        if self.getXFType() == 1 {
            !attr
        } else {
            attr
        }
    }
    // java boolean attr = _attr_alignment.isSet(this.field_5_indention_options); boolean isStyle = this.getXFType() == 1; return isStyle ? !attr : attr;

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:441
    // java public boolean isValidAlignment() {
    // java boolean attr = _attr_alignment.isSet(this.field_5_indention_options);
    // java boolean isStyle = this.getXFType() == 1;
    // java return isStyle ? !attr : attr;
    // java }
    pub fn isValidAlignment(&self) -> bool {
        let attr = self.getAttrAllignment();
        if self.getXFType() == 1 {
            !attr
        } else {
            attr
        }
    }
    // java boolean attr = _attr_border.isSet(this.field_5_indention_options); boolean isStyle = this.getXFType() == 1; return isStyle ? !attr : attr;

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:451
    // java public boolean isValidBorder() {
    // java boolean attr = _attr_border.isSet(this.field_5_indention_options);
    // java boolean isStyle = this.getXFType() == 1;
    // java return isStyle ? !attr : attr;
    // java }
    pub fn isValidBorder(&self) -> bool {
        let attr = self.getAttrBorder();
        if self.getXFType() == 1 {
            !attr
        } else {
            attr
        }
    }
    // java boolean attr = _attr_pattern.isSet(this.field_5_indention_options); boolean isStyle = this.getXFType() == 1; return isStyle ? !attr : attr;

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:461
    // java public boolean isValidPattern() {
    // java boolean attr = _attr_pattern.isSet(this.field_5_indention_options);
    // java boolean isStyle = this.getXFType() == 1;
    // java return isStyle ? !attr : attr;
    // java }
    pub fn isValidPattern(&self) -> bool {
        let attr = self.getAttrPattern();
        if self.getXFType() == 1 {
            !attr
        } else {
            attr
        }
    }
    // rust: boolean attr = _attr_celloptions.isSet(this.field_5_indention_options); boolean isStyle = this.getXFType() == 1; return isStyle ? !attr : attr;

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/ExtendedFormatRecord.java:471
    // java public boolean isValidCellOptions() {
    // java boolean attr = _attr_cell_options.isSet(this.field_5_indention_options);
    // java boolean isStyle = this.getXFType() == 1;
    // java return isStyle ? !attr : attr;
    // java }
    pub fn isValidCellOptions(&self) -> bool {
        let attr = self.getAttrCellOptions();
        if self.getXFType() == 1 {
            !attr
        } else {
            attr
        }
    }
}
