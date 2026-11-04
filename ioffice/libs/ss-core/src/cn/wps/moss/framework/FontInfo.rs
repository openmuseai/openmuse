// java public final class FontInfo
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FontInfo {
    color: i32,
    boldWeight: i16,
    isItalic: bool,
    underline: i8,
    fontSize: i32,
    fontName: String,
    superSubScript: i16,
    isStrikeout: bool,
}
impl FontInfo {
    // java public FontInfo(int color, short boldWeight, boolean isItalic, byte underline, int fontSize, @NotNull String fontName, short superSubScript, boolean isStrikeout) {
    // java Intrinsics.checkNotNullParameter(fontName, "fontName");
    // java this.color = color;
    // java this.boldWeight = boldWeight;
    // java this.isItalic = isItalic;
    // java this.underline = underline;
    // java this.fontSize = fontSize;
    // java this.fontName = fontName;
    // java this.superSubScript = superSubScript;
    // java this.isStrikeout = isStrikeout;
    // java }
    pub fn new(
        color: i32,
        boldWeight: i16,
        isItalic: bool,
        underline: i8,
        fontSize: i32,
        fontName: String,
        superSubScript: i16,
        isStrikeout: bool,
    ) -> Self {
        Self {
            color,
            boldWeight,
            isItalic,
            underline,
            fontSize,
            fontName,
            superSubScript,
            isStrikeout,
        }
    }
    // java public final int getColor() {
    // java return this.color;
    // java }
    pub fn getColor(&self) -> i32 {
        self.color
    }
    // java public final short getBoldWeight() {
    // java return this.boldWeight;
    // java }
    pub fn getBoldWeight(&self) -> i16 {
        self.boldWeight
    }
    // java public final boolean isItalic() {
    // java return this.isItalic;
    // java }
    pub fn isItalic(&self) -> bool {
        self.isItalic
    }
    // java public final byte getUnderline() {
    // java return this.underline;
    // java }
    pub fn getUnderline(&self) -> i8 {
        self.underline
    }
    // java public final int getFontSize() {
    // java return this.fontSize;
    // java }
    pub fn getFontSize(&self) -> i32 {
        self.fontSize
    }
    // java public final short getSuperSubScript() {
    // java return this.superSubScript;
    // java }
    pub fn getSuperSubScript(&self) -> i16 {
        self.superSubScript
    }
    // java public final boolean isStrikeout() {
    // java return this.isStrikeout;
    // java }
    pub fn isStrikeout(&self) -> bool {
        self.isStrikeout
    }
    // java public final String getFontName() {
    // java return this.fontName;
    // java }
    pub fn getFontName(&self) -> &str {
        &self.fontName
    }
    // java public int hashCode() {
    // java int result = Integer.hashCode(this.color);
    // java result = result * 31 + Short.hashCode(this.boldWeight);
    // java result = result * 31 + (this.isItalic ? 1 : 0);
    // java result = result * 31 + Byte.hashCode(this.underline);
    // java result = result * 31 + Integer.hashCode(this.fontSize);
    // java result = result * 31 + this.fontName.hashCode();
    // java result = result * 31 + Short.hashCode(this.superSubScript);
    // java result = result * 31 + (this.isStrikeout ? 1 : 0);
    // java return result;
    // java }
    pub fn hashCode(&self) -> i32 {
        let mut result = self.color;
        for v in [
            self.boldWeight as i32,
            self.isItalic as i32,
            self.underline as i32,
            self.fontSize,
            self.fontName
                .encode_utf16()
                .fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32)),
            self.superSubScript as i32,
            self.isStrikeout as i32,
        ] {
            result = result.wrapping_mul(31).wrapping_add(v);
        }
        result
    }
}
