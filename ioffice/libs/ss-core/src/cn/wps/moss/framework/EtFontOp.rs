use super::FontInfo::FontInfo;
use crate::cn::wps::moss::app::{xf::KmoFont::KmoFont, KmoSheet::KmoSheet};
// java public final class EtFontOp
pub struct EtFontOp;
pub const INSTANCE: EtFontOp = EtFontOp;
impl EtFontOp {
    // java public final FontInfo getFont(@NotNull KmoSheet kmoSheet, int row, int col) {
    // java Intrinsics.checkNotNullParameter(kmoSheet, "kmoSheet");
    // java KmoXF xf = kmoSheet.getCellXF(row, col);
    // java KmoFont var10000 = xf != null ? xf.getFont() : null;
    // java if (var10000 == null) {
    // java return null;
    // java } else {
    // java KmoFont font = var10000;
    // java int var10002 = font.getColorPaletteIndex();
    // java short var10003 = font.getBoldWeight();
    // java boolean var10004 = font.isItalic();
    // java byte var10005 = font.getUnderline();
    // java int var10006 = font.getFontHeight() / 20;
    // java String var10007 = font.getFontName();
    // java Intrinsics.checkNotNullExpressionValue(var10007, "font.fontName");
    // java return new FontInfo(var10002, var10003, var10004, var10005, var10006, var10007, font.getSuperSubScript(), font.isStrikeout());
    // java }
    // java }
    pub fn getFont(&self, s: &KmoSheet, row: i32, col: i32) -> Option<FontInfo> {
        s.getCellXF__2(row, col)
            .and_then(|xf| xf.getFont())
            .map(|f| Self::fromFont(&f))
    }
    // java public final FontInfo getFont(@NotNull KmoSheet kmoSheet, int fontIndex) {
    // java Intrinsics.checkNotNullParameter(kmoSheet, "kmoSheet");
    // java KmoFont var10000 = kmoSheet.getBook().getXFManager().getFont(fontIndex);
    // java Intrinsics.checkNotNullExpressionValue(var10000, "kmoSheet.book.xfManager.getFont(fontIndex)");
    // java KmoFont font = var10000;
    // java int var10002 = font.getColorPaletteIndex();
    // java short var10003 = font.getBoldWeight();
    // java boolean var10004 = font.isItalic();
    // java byte var10005 = font.getUnderline();
    // java int var10006 = font.getFontHeight() / 20;
    // java String var10007 = font.getFontName();
    // java Intrinsics.checkNotNullExpressionValue(var10007, "font.fontName");
    // java return new FontInfo(var10002, var10003, var10004, var10005, var10006, var10007, font.getSuperSubScript(), font.isStrikeout());
    // java }
    pub fn getFont__index(&self, s: &KmoSheet, index: i32) -> FontInfo {
        Self::fromFont(&s.getBook().getXFManager().borrow().getFont(index))
    }
    // rust: Value-construction adapter shared by the two original overloads.
    fn fromFont(f: &KmoFont) -> FontInfo {
        FontInfo::new(
            f.getColorPaletteIndex(),
            f.getBoldWeight(),
            f.isItalic(),
            f.getUnderline(),
            f.getFontHeight() as i32 / 20,
            f.getFontName(),
            f.getSuperSubScript(),
            f.isStrikeout(),
        )
    }
}
