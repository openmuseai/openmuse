use crate::cn::wps::font::{FontHost::FontHost, NativeTypeface::NativeTypeface};
use crate::cn::wps::moffice::spreadsheet::control::grid::render::native::{FontMetricsInt, Paint};
// java public class PaintFontHelper
pub struct PaintFontHelper;
thread_local! {static TEXT_PAINT:std::cell::RefCell<Paint>=std::cell::RefCell::new(Paint::default());}
impl PaintFontHelper {
    // java public static Typeface getTypeface(String fontName, boolean isBold, boolean isItalic) {
    // java ITypeface typeface = getITypeface(fontName, isBold, isItalic);
    // java return (Typeface)typeface.getNativeTypeface();
    // java }
    pub fn getTypeface(name: &str, bold: bool, italic: bool) -> NativeTypeface {
        let style = (if bold { 1 } else { 0 }) + (if italic { 2 } else { 0 });
        FontHost::getFontTable()
            .borrow()
            .get(name, true, false)
            .expect("font family")
            .getTypeface(style)
            .expect("native typeface")
    }
    // java public static void getFontMetricsInt(Paint.FontMetricsInt fontMetrics, String fontName, float fontSize, boolean bold, boolean italic) {
    // java Typeface face = getTypeface(fontName, bold, italic);
    // java Paint paint = (Paint)sTextPaint.get();
    // java paint.setTextSize(fontSize);
    // java paint.setTypeface(face);
    // java paint.getFontMetricsInt(fontMetrics);
    // java }
    pub fn getFontMetricsInt(name: &str, size: f32, bold: bool, italic: bool) -> FontMetricsInt {
        let face = Self::getTypeface(name, bold, italic);
        TEXT_PAINT.with(|cell| {
            let mut p = cell.borrow_mut();
            p.textSize = size;
            p.setTypeface(&face);
            p.getFontMetricsInt()
        })
    }
}
