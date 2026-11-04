use util::org::apache::poi::util::BitField::BitField;
// java public final class BorderFormatting
#[derive(Clone, Debug, PartialEq)]
pub struct BorderFormatting {
    // java private int field_13_border_styles1;
    pub(crate) field_13_border_styles1: i32,
    // java private int border_left_color;
    pub(crate) border_left_color: i32,
    // java private int border_right_color;
    pub(crate) border_right_color: i32,
    // java private int border_top_color;
    pub(crate) border_top_color: i32,
    // java private int border_bottom_color;
    pub(crate) border_bottom_color: i32,
    // java private int border_diag_color;
    pub(crate) border_diag_color: i32,
    // java private byte border_diag_type;
    pub(crate) border_diag_type: i8,
    // java private int border_vertical_color;
    pub(crate) border_vertical_color: i32,
    // java private int border_horizontal_color;
    pub(crate) border_horizontal_color: i32,
    // java private int internalLineStyle;
    pub(crate) internalLineStyle: i32,
    // java private byte dxfOptions;
    pub(crate) dxfOptions: i8,
}
impl Default for BorderFormatting {
    fn default() -> Self {
        Self {
            field_13_border_styles1: 0,
            border_left_color: 0,
            border_right_color: 0,
            border_top_color: 0,
            border_bottom_color: 0,
            border_diag_color: 0,
            border_diag_type: 0,
            border_vertical_color: 0,
            border_horizontal_color: 0,
            internalLineStyle: 0,
            dxfOptions: 0,
        }
    }
}
impl BorderFormatting {
    // rust: public BorderFormatting() { }

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:58
    // java public BorderFormatting() {
    // java this.field_13_border_styles1 = 0;
    // java this.border_left_color = 0;
    // java this.border_right_color = 0;
    // java this.border_top_color = 0;
    // java this.border_bottom_color = 0;
    // java this.border_diag_color = 0;
    // java this.border_diag_type = 0;
    // java this.internalLineStyle = 0;
    // java }
    pub fn BorderFormatting() -> Self {
        Self::default()
    }
    // java public void setBorderLeft(int border) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:80
    // java public void setBorderLeft(int border) {
    // java this.setLeftOption();
    // java this.field_13_border_styles1 = bordLeftLineStyle.setValue(this.field_13_border_styles1, border);
    // java }
    pub fn setBorderLeft(&mut self, border: i32) {
        // java this.setLeftOption();
        self.setLeftOption();
        // java this.field_13_border_styles1 = bordLeftLineStyle.setValue(this.field_13_border_styles1, border);
        self.field_13_border_styles1 = (((self.field_13_border_styles1 as i32) & !15)
            | (((border) as i32).wrapping_shl(0) & 15))
            as i32;
    }
    // java public int getBorderLeft() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:85
    // java public int getBorderLeft() {
    // java return bordLeftLineStyle.getValue(this.field_13_border_styles1);
    // java }
    pub fn getBorderLeft(&self) -> i32 {
        // java return bordLeftLineStyle.getValue(this.field_13_border_styles1);
        ((((self.field_13_border_styles1 as i32) as u32 & (15i32 as u32)) >> 0) as i32) as i32
    }
    // java public void setBorderRight(int border) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:89
    // java public void setBorderRight(int border) {
    // java this.setRightOption();
    // java this.field_13_border_styles1 = bordRightLineStyle.setValue(this.field_13_border_styles1, border);
    // java }
    pub fn setBorderRight(&mut self, border: i32) {
        // java this.setRightOption();
        self.setRightOption();
        // java this.field_13_border_styles1 = bordRightLineStyle.setValue(this.field_13_border_styles1, border);
        self.field_13_border_styles1 = (((self.field_13_border_styles1 as i32) & !240)
            | (((border) as i32).wrapping_shl(4) & 240))
            as i32;
    }
    // java public int getBorderRight() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:94
    // java public int getBorderRight() {
    // java return bordRightLineStyle.getValue(this.field_13_border_styles1);
    // java }
    pub fn getBorderRight(&self) -> i32 {
        // java return bordRightLineStyle.getValue(this.field_13_border_styles1);
        ((((self.field_13_border_styles1 as i32) as u32 & (240i32 as u32)) >> 4) as i32) as i32
    }
    // java public void setBorderTop(int border) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:98
    // java public void setBorderTop(int border) {
    // java this.setTopOption();
    // java this.field_13_border_styles1 = bordTopLineStyle.setValue(this.field_13_border_styles1, border);
    // java }
    pub fn setBorderTop(&mut self, border: i32) {
        // java this.setTopOption();
        self.setTopOption();
        // java this.field_13_border_styles1 = bordTopLineStyle.setValue(this.field_13_border_styles1, border);
        self.field_13_border_styles1 = (((self.field_13_border_styles1 as i32) & !3840)
            | (((border) as i32).wrapping_shl(8) & 3840))
            as i32;
    }
    // java public int getBorderTop() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:103
    // java public int getBorderTop() {
    // java return bordTopLineStyle.getValue(this.field_13_border_styles1);
    // java }
    pub fn getBorderTop(&self) -> i32 {
        // java return bordTopLineStyle.getValue(this.field_13_border_styles1);
        ((((self.field_13_border_styles1 as i32) as u32 & (3840i32 as u32)) >> 8) as i32) as i32
    }
    // java public void setBorderBottom(int border) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:107
    // java public void setBorderBottom(int border) {
    // java this.setBottomOption();
    // java this.field_13_border_styles1 = bordBottomLineStyle.setValue(this.field_13_border_styles1, border);
    // java }
    pub fn setBorderBottom(&mut self, border: i32) {
        // java this.setBottomOption();
        self.setBottomOption();
        // java this.field_13_border_styles1 = bordBottomLineStyle.setValue(this.field_13_border_styles1, border);
        self.field_13_border_styles1 = (((self.field_13_border_styles1 as i32) & !61440)
            | (((border) as i32).wrapping_shl(12) & 61440))
            as i32;
    }
    // java public int getBorderBottom() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:112
    // java public int getBorderBottom() {
    // java return bordBottomLineStyle.getValue(this.field_13_border_styles1);
    // java }
    pub fn getBorderBottom(&self) -> i32 {
        // java return bordBottomLineStyle.getValue(this.field_13_border_styles1);
        ((((self.field_13_border_styles1 as i32) as u32 & (61440i32 as u32)) >> 12) as i32) as i32
    }
    // java public void setBorderDiagonal(int border) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:116
    // java public void setBorderDiagonal(int border) {
    // java this.border_diag_type = (byte)(border & 15);
    // java }
    pub fn setBorderDiagonal(&mut self, border: i32) {
        // java this.border_diag_type = (byte)(border & 15);
        self.border_diag_type = (border & 15) as i8;
    }
    // java public int getBorderDiagonal() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:120
    // java public int getBorderDiagonal() {
    // java return this.border_diag_type;
    // java }
    pub fn getBorderDiagonal(&self) -> i32 {
        // java return this.border_diag_type;
        (self.border_diag_type) as i32
    }
    // java public void setLeftBorderColor(int color) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:124
    // java public void setLeftBorderColor(int color) {
    // java this.border_left_color = color;
    // java }
    pub fn setLeftBorderColor(&mut self, color: i32) {
        // java this.border_left_color = color;
        self.border_left_color = color;
    }
    // java public int getLeftBorderColor() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:128
    // java public int getLeftBorderColor() {
    // java return this.border_left_color;
    // java }
    pub fn getLeftBorderColor(&self) -> i32 {
        // java return this.border_left_color;
        (self.border_left_color) as i32
    }
    // java public void setRightBorderColor(int color) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:132
    // java public void setRightBorderColor(int color) {
    // java this.border_right_color = color;
    // java }
    pub fn setRightBorderColor(&mut self, color: i32) {
        // java this.border_right_color = color;
        self.border_right_color = color;
    }
    // java public int getRightBorderColor() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:136
    // java public int getRightBorderColor() {
    // java return this.border_right_color;
    // java }
    pub fn getRightBorderColor(&self) -> i32 {
        // java return this.border_right_color;
        (self.border_right_color) as i32
    }
    // java public void setTopBorderColor(int color) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:140
    // java public void setTopBorderColor(int color) {
    // java this.border_top_color = color;
    // java }
    pub fn setTopBorderColor(&mut self, color: i32) {
        // java this.border_top_color = color;
        self.border_top_color = color;
    }
    // java public int getTopBorderColor() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:144
    // java public int getTopBorderColor() {
    // java return this.border_top_color;
    // java }
    pub fn getTopBorderColor(&self) -> i32 {
        // java return this.border_top_color;
        (self.border_top_color) as i32
    }
    // java public void setBottomBorderColor(int color) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:148
    // java public void setBottomBorderColor(int color) {
    // java this.border_bottom_color = color;
    // java }
    pub fn setBottomBorderColor(&mut self, color: i32) {
        // java this.border_bottom_color = color;
        self.border_bottom_color = color;
    }
    // java public int getBottomBorderColor() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:152
    // java public int getBottomBorderColor() {
    // java return this.border_bottom_color;
    // java }
    pub fn getBottomBorderColor(&self) -> i32 {
        // java return this.border_bottom_color;
        (self.border_bottom_color) as i32
    }
    // java public void setDiagonalBorderColor(int color) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:156
    // java public void setDiagonalBorderColor(int color) {
    // java this.border_diag_color = color;
    // java }
    pub fn setDiagonalBorderColor(&mut self, color: i32) {
        // java this.border_diag_color = color;
        self.border_diag_color = color;
    }
    // java public int getDiagonalBorderColor() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:160
    // java public int getDiagonalBorderColor() {
    // java return this.border_diag_color;
    // java }
    pub fn getDiagonalBorderColor(&self) -> i32 {
        // java return this.border_diag_color;
        (self.border_diag_color) as i32
    }
    // java public void setBackwardDiagonalOn(boolean on) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:168
    // java public void setBackwardDiagonalOn(boolean on) {
    // java this.field_13_border_styles1 = bordTlBrLineOnOff.setBoolean(this.field_13_border_styles1, on);
    // java }
    pub fn setBackwardDiagonalOn(&mut self, on: bool) {
        // java this.field_13_border_styles1 = bordTlBrLineOnOff.setBoolean(this.field_13_border_styles1, on);
        self.field_13_border_styles1 = (((self.field_13_border_styles1 as i32) & !1073741824)
            | if on { 1073741824 } else { 0 }) as i32;
    }
    // java public boolean isBackwardDiagonalOn() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:176
    // java public boolean isBackwardDiagonalOn() {
    // java return bordTlBrLineOnOff.isSet(this.field_13_border_styles1);
    // java }
    pub fn isBackwardDiagonalOn(&self) -> bool {
        // java return bordTlBrLineOnOff.isSet(this.field_13_border_styles1);
        ((self.field_13_border_styles1 as i32) & 1073741824) != 0
    }
    // java public int getBorderVertical() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:180
    // java public int getBorderVertical() {
    // java return bordVerticalLineStyle.getValue(this.internalLineStyle);
    // java }
    pub fn getBorderVertical(&self) -> i32 {
        // java return bordVerticalLineStyle.getValue(this.internalLineStyle);
        ((((self.internalLineStyle as i32) as u32 & (15i32 as u32)) >> 0) as i32) as i32
    }
    // java public void setBorderVertical(int border) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:184
    // java public void setBorderVertical(int border) {
    // java this.setVerticalOption();
    // java this.internalLineStyle = bordVerticalLineStyle.setValue(this.internalLineStyle, border);
    // java }
    pub fn setBorderVertical(&mut self, border: i32) {
        // java this.setVerticalOption();
        self.setVerticalOption();
        // java this.internalLineStyle = bordVerticalLineStyle.setValue(this.internalLineStyle, border);
        self.internalLineStyle = (((self.internalLineStyle as i32) & !15)
            | (((border) as i32).wrapping_shl(0) & 15)) as i32;
    }
    // java public int getVerticalBorderColor() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:189
    // java public int getVerticalBorderColor() {
    // java return this.border_vertical_color;
    // java }
    pub fn getVerticalBorderColor(&self) -> i32 {
        // java return this.border_vertical_color;
        (self.border_vertical_color) as i32
    }
    // java public void setVerticalBorderColor(int color) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:193
    // java public void setVerticalBorderColor(int color) {
    // java this.border_vertical_color = color;
    // java }
    pub fn setVerticalBorderColor(&mut self, color: i32) {
        // java this.border_vertical_color = color;
        self.border_vertical_color = color;
    }
    // java public int getBorderHorizontal() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:197
    // java public int getBorderHorizontal() {
    // java return bordHorizontalLineStyle.getValue(this.internalLineStyle);
    // java }
    pub fn getBorderHorizontal(&self) -> i32 {
        // java return bordHorizontalLineStyle.getValue(this.internalLineStyle);
        ((((self.internalLineStyle as i32) as u32 & (240i32 as u32)) >> 4) as i32) as i32
    }
    // java public void setBorderHorizontal(int border) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:201
    // java public void setBorderHorizontal(int border) {
    // java this.setHorizontalOption();
    // java this.internalLineStyle = bordHorizontalLineStyle.setValue(this.internalLineStyle, border);
    // java }
    pub fn setBorderHorizontal(&mut self, border: i32) {
        // java this.setHorizontalOption();
        self.setHorizontalOption();
        // java this.internalLineStyle = bordHorizontalLineStyle.setValue(this.internalLineStyle, border);
        self.internalLineStyle = (((self.internalLineStyle as i32) & !240)
            | (((border) as i32).wrapping_shl(4) & 240)) as i32;
    }
    // java public int getHorizontalBorderColor() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:206
    // java public int getHorizontalBorderColor() {
    // java return this.border_horizontal_color;
    // java }
    pub fn getHorizontalBorderColor(&self) -> i32 {
        // java return this.border_horizontal_color;
        (self.border_horizontal_color) as i32
    }
    // java public void setHorizontalBorderColor(int color) {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:210
    // java public void setHorizontalBorderColor(int color) {
    // java this.border_horizontal_color = color;
    // java }
    pub fn setHorizontalBorderColor(&mut self, color: i32) {
        // java this.border_horizontal_color = color;
        self.border_horizontal_color = color;
    }
    // java public void setLeftOption() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:355
    // java public void setLeftOption() {
    // java this.dxfOptions = left_option.setByteBoolean(this.dxfOptions, true);
    // java }
    pub fn setLeftOption(&mut self) {
        // java this.dxfOptions = left_option.setByteBoolean(this.dxfOptions, true);
        self.dxfOptions = (((self.dxfOptions as i32) & !1) | if true { 1 } else { 0 }) as i8;
    }
    // java public boolean hasLeft() { return left_option.isSet(this.dxfOptions); }
    pub fn hasLeft(&self)->bool{self.dxfOptions&1!=0}
    // java public void setRightOption() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:363
    // java public void setRightOption() {
    // java this.dxfOptions = right_option.setByteBoolean(this.dxfOptions, true);
    // java }
    pub fn setRightOption(&mut self) {
        // java this.dxfOptions = right_option.setByteBoolean(this.dxfOptions, true);
        self.dxfOptions = (((self.dxfOptions as i32) & !2) | if true { 2 } else { 0 }) as i8;
    }
    // java public boolean hasRight() { return right_option.isSet(this.dxfOptions); }
    pub fn hasRight(&self)->bool{self.dxfOptions&2!=0}
    // java public void setTopOption() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:371
    // java public void setTopOption() {
    // java this.dxfOptions = top_option.setByteBoolean(this.dxfOptions, true);
    // java }
    pub fn setTopOption(&mut self) {
        // java this.dxfOptions = top_option.setByteBoolean(this.dxfOptions, true);
        self.dxfOptions = (((self.dxfOptions as i32) & !4) | if true { 4 } else { 0 }) as i8;
    }
    // java public boolean hasTop() { return top_option.isSet(this.dxfOptions); }
    pub fn hasTop(&self)->bool{self.dxfOptions&4!=0}
    // java public void setBottomOption() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:379
    // java public void setBottomOption() {
    // java this.dxfOptions = bottom_option.setByteBoolean(this.dxfOptions, true);
    // java }
    pub fn setBottomOption(&mut self) {
        // java this.dxfOptions = bottom_option.setByteBoolean(this.dxfOptions, true);
        self.dxfOptions = (((self.dxfOptions as i32) & !8) | if true { 8 } else { 0 }) as i8;
    }
    // java public boolean hasBottom() { return bottom_option.isSet(this.dxfOptions); }
    pub fn hasBottom(&self)->bool{self.dxfOptions&8!=0}
    // java public void setVerticalOption() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:387
    // java public void setVerticalOption() {
    // java this.dxfOptions = vertical_option.setByteBoolean(this.dxfOptions, true);
    // java }
    pub fn setVerticalOption(&mut self) {
        // java this.dxfOptions = vertical_option.setByteBoolean(this.dxfOptions, true);
        self.dxfOptions = (((self.dxfOptions as i32) & !16) | if true { 16 } else { 0 }) as i8;
    }
    // java public boolean hasVertical() { return vertical_option.isSet(this.dxfOptions); }
    pub fn hasVertical(&self)->bool{self.dxfOptions&16!=0}
    // java public void setHorizontalOption() {

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:395
    // java public void setHorizontalOption() {
    // java this.dxfOptions = horizontal_option.setByteBoolean(this.dxfOptions, true);
    // java }
    pub fn setHorizontalOption(&mut self) {
        // java this.dxfOptions = horizontal_option.setByteBoolean(this.dxfOptions, true);
        self.dxfOptions = (((self.dxfOptions as i32) & !32) | if true { 32 } else { 0 }) as i8;
    }
    // java public boolean hasHorizontal() { return horizontal_option.isSet(this.dxfOptions); }
    pub fn hasHorizontal(&self)->bool{self.dxfOptions&32!=0}
}

impl BorderFormatting {
    // java this.field_13_border_styles1 = bordBlTrtLineOnOff.setBoolean(this.field_13_border_styles1, on);

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:164
    // java public void setForwardDiagonalOn(boolean on) {
    // java this.field_13_border_styles1 = bordBlTrtLineOnOff.setBoolean(this.field_13_border_styles1, on);
    // java }
    pub fn setForwardDiagonalOn(&mut self, on: bool) {
        self.field_13_border_styles1 =
            (self.field_13_border_styles1 & i32::MAX) | if on { i32::MIN } else { 0 };
    }
    // java return bordBlTrtLineOnOff.isSet(this.field_13_border_styles1);

    // rust: source koffice/et/core/cn/wps/moss/filefmt/biff8/record/cf/BorderFormatting.java:172
    // java public boolean isForwardDiagonalOn() {
    // java return bordBlTrtLineOnOff.isSet(this.field_13_border_styles1);
    // java }
    pub fn isForwardDiagonalOn(&self) -> bool {
        self.field_13_border_styles1 & i32::MIN != 0
    }
}

impl BorderFormatting{// java public int hashCode() {
// java int prime = 31;
// java int result = 1;
// java result = 31 * result + this.border_bottom_color;
// java result = 31 * result + this.border_diag_color;
// java result = 31 * result + this.border_diag_type;
// java result = 31 * result + this.border_left_color;
// java result = 31 * result + this.border_right_color;
// java result = 31 * result + this.border_top_color;
// java result = 31 * result + this.field_13_border_styles1;
// java result = 31 * result + this.border_vertical_color;
// java result = 31 * result + this.border_horizontal_color;
// java result = 31 * result + this.internalLineStyle;
// java return result;
// java }
pub fn hashCode(&self)->i32 {
crate::cn::wps::moss::app::condfmt::java_values::fold([self.border_bottom_color as i32,self.border_diag_color as i32,self.border_diag_type as i32,self.border_left_color as i32,self.border_right_color as i32,self.border_top_color as i32,self.field_13_border_styles1 as i32,self.border_vertical_color as i32,self.border_horizontal_color as i32,self.internalLineStyle as i32])
}
// java public boolean equals(Object obj) {
// java if (this == obj) {
// java return true;
// java } else if (obj == null) {
// java return false;
// java } else if (this.getClass() != obj.getClass()) {
// java return false;
// java } else {
// java BorderFormatting other = (BorderFormatting)obj;
// java if (this.border_bottom_color != other.border_bottom_color) {
// java return false;
// java } else if (this.border_diag_color != other.border_diag_color) {
// java return false;
// java } else if (this.border_diag_type != other.border_diag_type) {
// java return false;
// java } else if (this.border_left_color != other.border_left_color) {
// java return false;
// java } else if (this.border_right_color != other.border_right_color) {
// java return false;
// java } else if (this.border_top_color != other.border_top_color) {
// java return false;
// java } else if (this.field_13_border_styles1 != other.field_13_border_styles1) {
// java return false;
// java } else if (this.border_horizontal_color != other.border_horizontal_color) {
// java return false;
// java } else if (this.border_vertical_color != other.border_vertical_color) {
// java return false;
// java } else {
// java return this.internalLineStyle == other.internalLineStyle;
// java }
// java }
// java }
pub fn equals(&self,o:&Self)->bool {
std::ptr::eq(self,o)||(self.border_bottom_color==o.border_bottom_color&&self.border_diag_color==o.border_diag_color&&self.border_diag_type==o.border_diag_type&&self.border_left_color==o.border_left_color&&self.border_right_color==o.border_right_color&&self.border_top_color==o.border_top_color&&self.field_13_border_styles1==o.field_13_border_styles1&&self.border_vertical_color==o.border_vertical_color&&self.border_horizontal_color==o.border_horizontal_color&&self.internalLineStyle==o.internalLineStyle)
}
}
